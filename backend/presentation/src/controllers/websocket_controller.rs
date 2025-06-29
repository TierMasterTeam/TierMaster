use crate::presenters::TierlistPresenter;
use application::AppState;
use domain::entities::TierlistEntity;
use domain::error::ApiError;
use domain::mappers::EntityMapper;
use log::{error, info};
use socketioxide::extract::{Data, SocketRef, State};
use socketioxide::{BroadcastError, SendError};

pub struct WebsocketController;

impl WebsocketController {
    pub async fn on_connect(socket: SocketRef) {
        info!("socket connected: {}", socket.id);

        socket.on(
            "join",
            async |socket: SocketRef, Data::<String>(room), app_state: State<AppState>| {
                info!("Received join : {:?}", room);
                socket.leave_all();
                socket.join(room.clone());

                let room_result = app_state.services()
                    .websocket()
                    .get(room.as_str())
                    .await;

                if room_result.is_err() {
                    info!("Creating room : {:?}", room);
                    let result = init_room(room.as_str(), app_state.clone()).await;
                    emit_tierlist(socket, result);
                    return
                }

                emit_tierlist(socket, room_result);
            },
        );

        socket.on(
            "update",
            async move |socket: SocketRef, Data::<TierlistPresenter>(tierlist), State::<AppState>(app_state)| {
                let room_id = tierlist.id.clone();
                
                info!("Received update event for room : {:?}", room_id);
                
                let update_result = app_state.services()
                    .websocket()
                    .update(&room_id, tierlist.clone().to_entity())
                    .await;

                broadcast_tierlist(socket, update_result, room_id).await;
            },
        );

    }
}

/// create a new room for a tierlist with id 'room_id'
async fn init_room(room_id: &str, app_state: AppState) -> Result<TierlistEntity, ApiError> {
    let tierlist = app_state.services().tierlist().get_tierlist_by_id(room_id, None)
        .await?;
    
    app_state.services().websocket().create(room_id, tierlist.clone()).await
        .map(|_| tierlist)
}

/// Broadcast the "tierlist" event to all socket within the room (except to the current socket)
async fn broadcast_tierlist(socket: SocketRef, tierlist_result: Result<TierlistEntity, ApiError>, room_id: String) {
    match tierlist_result {
        Err(api_error) => on_error(api_error),
        Ok(tierlist) => {
            let presenter = TierlistPresenter::from(tierlist);
            let broadcast_result = socket
                .to(room_id)
                .emit("tierlist", &presenter)
                .await;
            handle_broadcast_result(broadcast_result);
        }
    }
}

/// Emit the "tierlist" event to the current socket only
fn emit_tierlist(socket: SocketRef, tierlist_result: Result<TierlistEntity, ApiError>) {
    match tierlist_result {
        Err(api_error) => on_error(api_error),
        Ok(tierlist) => {
            let presenter = TierlistPresenter::from(tierlist);
            handle_emit_result(socket.emit("tierlist", &presenter));
        }
    }
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