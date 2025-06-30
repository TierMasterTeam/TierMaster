use crate::presenters::{RoomPresenter, RoomUserPresenter, TierlistRoomPresenter};
use crate::states::{AuthSession, OptionalAuthSession};
use application::AppState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::headers::Cookie;
use axum_extra::TypedHeader;
use domain::entities::{RoomEntity, TierlistRoomEntity};
use domain::error::ApiError;
use domain::mappers::EntityMapper;
use log::{debug, error, info, warn};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use socketioxide::extract::{Data, SocketRef, State};
use socketioxide::socket::Sid;
use socketioxide::{BroadcastError, SendError, SocketIo};
use std::hash::{DefaultHasher, Hash, Hasher};

pub struct WebsocketController;

impl WebsocketController {
    pub async fn on_connect(socket: SocketRef, state: State<AppState>) {
        info!("Socket connected: {}", socket.id);

        let auth_session = extract_auth_session(&mut socket.req_parts().clone(), &state).await;
        info!("User is logged in: {}", auth_session.auth_state.is_some());

        let room_user = get_room_user(socket.clone(), auth_session.clone(), &state).await;

        register_events(socket.clone(), room_user.clone());
        
        let _ = socket.emit("ready", &room_user);
        debug!("Emitted ready event to client.");
    }
}

async fn get_room_user(socket: SocketRef, auth_session: OptionalAuthSession, state: &AppState) -> RoomUserPresenter {
    let user_name = match auth_session.auth_state.clone() {
        Some(auth_session) => state.services.user().get_by_id(auth_session.user_id.as_str())
            .await
            .map_err(on_error)
            .map(|user| user.username),
        None => Ok(generate_random_user_name(socket.id.clone()))
    }.unwrap_or_else(|_| generate_random_user_name(socket.id.clone()));

    let user_id = match auth_session.auth_state.clone() {
        Some(auth_session) => auth_session.user_id,
        None => get_anonymous_room_user_id(socket.clone().id)
    };

    RoomUserPresenter {
        id: user_id.clone(),
        name: user_name,
        color: generate_color_from_id(user_id.as_str())
    }
}

fn generate_random_user_name(socket_id: Sid) -> String {
    let mut hasher = DefaultHasher::new();
    socket_id.hash(&mut hasher);
    let seed = hasher.finish();
    let mut rng = StdRng::seed_from_u64(seed);

    let adjectives = [
        "Curious", "Bold", "Silent", "Witty", "Opinionated",
        "Clever", "Sneaky", "Quirky", "Chill", "Passionate",
        "Shy", "Honest", "Casual"
    ];

    let tier_nouns = [
        "Ranker", "Critic", "Judge", "Voter", "Tiermaker",
        "Sorter", "Rater", "Reviewer", "Champion",
        "Contender", "TierHero", "Debater", "TierFan"
    ];

    let adjective = adjectives[rng.random_range(0..adjectives.len())];
    let noun = tier_nouns[rng.random_range(0..tier_nouns.len())];
    let number = rng.random_range(10..100);

    format!("{}{}{}", adjective, noun, number)
}

fn get_anonymous_room_user_id(socket_id: Sid) -> String {
    format!("anonymous_{}", socket_id)
}

pub fn generate_color_from_id(id: &str) -> String {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    let seed = hasher.finish();

    let mut rng = StdRng::seed_from_u64(seed);

    let r: u8 = rng.random_range(64..=192);
    let g: u8 = rng.random_range(64..=192);
    let b: u8 = rng.random_range(64..=192);

    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn register_events(socket: SocketRef, room_user: RoomUserPresenter) {
    let mut cloned_room_user = room_user.clone();
    socket.on(
        "join",
        |socket: SocketRef, Data::<String>(room), State(app_state): State<AppState>| async move {
            info!("Received join: {:?}", room);

            socket.leave_all();
            socket.join(room.clone());

            let mut room_result = app_state.services()
                .websocket()
                .join(room.as_str(), cloned_room_user.clone().to_entity())
                .await;

            if room_result.is_err() {
                info!("Creating room: {:?}", room);
                room_result = init_room(room.as_str(), cloned_room_user, app_state.clone()).await;
            }

            emit_event("join", socket.clone(), room_result.clone());
            broadcast_event("join", socket, room_result, room).await;
        },
    );

    socket.on(
        "update-tierlist",
        |socket: SocketRef, Data::<TierlistRoomPresenter>(tierlist), State(app_state): State<AppState>| async move {
            let room_id = tierlist.id.clone();
            info!("Received update-tierlist for room: {:?}", room_id);

            let update_result = app_state.services()
                .websocket()
                .update_tierlist(&room_id, tierlist.clone().to_entity())
                .await;

            broadcast_event("update-tierlist", socket, update_result, room_id).await;
        },
    );

    cloned_room_user = room_user.clone();
    socket.on(
        "leave",
        |socket: SocketRef, Data::<String>(room_id), State(app_state): State<AppState>| async move {
            info!("Received leave for room: {}", room_id.clone());

            let result = app_state.services()
                .websocket()
                .leave(&room_id.clone(), cloned_room_user.to_entity())
                .await;

            broadcast_event("leave", socket, result, room_id).await;
        },
    );

    cloned_room_user = room_user.clone();
    socket.on_disconnect(|socket: SocketRef, State(app_state): State<AppState>| async move {
        info!("Socket disconnected : {}", socket.id);

        let result = app_state.services
            .websocket()
            .disconnected(cloned_room_user.clone().to_entity())
            .await
            .map_err(on_error);

        if result.is_err() {
            return;
        }

        let (tierlist_room, room_id) = result.unwrap();
        broadcast_event("leave", socket, Ok(tierlist_room), room_id).await;
    });
}

/// create a new room for a tierlist with id 'room_id'
async fn init_room(room_id: &str, room_user: RoomUserPresenter, app_state: AppState) -> Result<RoomEntity, ApiError> {
    let tierlist = app_state.services()
        .tierlist()
        .get_tierlist_by_id(room_id, Some(room_user.clone().id))
        .await?;

    let tierlist_room = RoomEntity {
        users: vec![room_user.to_entity()],
        tierlist: TierlistRoomEntity::from_tierlist_entity(tierlist),
    };
    
    app_state.services().websocket().create(room_id, tierlist_room.clone()).await
        .map(|_| tierlist_room)
}

/// Broadcast an event to all socket within the room (except to the current socket)
async fn broadcast_event(event: &str, socket: SocketRef, tierlist_room_result: Result<RoomEntity, ApiError>, room_id: String) {
    match tierlist_room_result {
        Err(api_error) => on_error(api_error),
        Ok(room) => {
            let presenter = RoomPresenter::from(room);
            let broadcast_result = socket
                .to(room_id)
                .emit(event, &presenter)
                .await;
            handle_broadcast_result(broadcast_result);
        }
    }
}

/// Emit the event to the current socket only
fn emit_event(event: &str, socket: SocketRef, tierlist_room_result: Result<RoomEntity, ApiError>) {
    match tierlist_room_result {
        Err(api_error) => on_error(api_error),
        Ok(room) => {
            let presenter = RoomPresenter::from(room);
            handle_emit_result(socket.emit(event, &presenter));
        }
    }
}

async fn extract_auth_session(parts: &mut Parts, app_state: &AppState) -> OptionalAuthSession {
    let result_cookies = TypedHeader::<Cookie>::from_request_parts(parts, app_state)
        .await;

    if result_cookies.is_err() {
        warn!("No cookie header present in websocket request");
        return OptionalAuthSession::none();
    }

    let cookies = result_cookies.unwrap();
    let signed_token = cookies.get("token");

    if signed_token.is_none() {
        debug!("Token not found in cookies of socket !");
        return OptionalAuthSession::none();
    }

    if app_state.services.auth().verify_token(signed_token.unwrap()).is_err() {
        debug!("Invalid token during websocket auth");
        return OptionalAuthSession::none();
    }

    let result = app_state
        .services
        .auth()
        .validate_session(signed_token.unwrap())
        .await;

    if result.is_err() {
        debug!("Token has expired, user needs to log back in !");
        return OptionalAuthSession::none();
    }

    let (token, user_id) = result.unwrap();
    OptionalAuthSession::some(AuthSession {
        token,
        user_id
    })
}

fn on_error(api_error: ApiError) {
    error!("{}", api_error)
}

fn handle_emit_result(result: Result<(), SendError>) {
    let _ = result.map_err(|error| error!("Error emitting event : {}", error.to_string()));
}

fn handle_broadcast_result(result: Result<(), BroadcastError>) {
    let _ = result.map_err(|error| error!("Error broadcasting event : {}", error.to_string()));
}