use crate::presenters::TierlistPresenter;
use application::AppState;
use domain::entities::TierlistEntity;
use domain::error::ApiError;
use domain::mappers::EntityMapper;
use log::{error, info};
use socketioxide::extract::{Data, SocketRef, State};

pub struct WebsocketController;

impl WebsocketController {
    pub async fn on_connect(socket: SocketRef) {
        info!("socket connected: {}", socket.id);

        socket.on(
            "join",
            async |socket: SocketRef, Data::<String>(room), app_state: State<AppState>| {
                info!("Received join: {:?}", room);
                socket.leave_all();
                socket.join(room.clone());

                let tierlist_result = app_state.services()
                    .websocket()
                    .get(room.as_str())
                    .await;

                emit_tierlist(socket, tierlist_result);
            },
        );

        socket.on(
            "update",
            async |socket: SocketRef, Data::<TierlistPresenter>(tierlist), State::<AppState>(app_state)| {
                info!("Received message: {:?}", tierlist);

                let tierlist_result = app_state.services()
                    .websocket()
                    .update(tierlist.clone().id.as_str(), tierlist.to_entity())
                    .await;

                emit_tierlist(socket, tierlist_result);
            },
        )
    }
}


fn emit_tierlist(socket: SocketRef, tierlist_result: Result<TierlistEntity, ApiError>) {
    match tierlist_result {
        Err(api_error) => error!("{}", api_error),
        Ok(tierlist) => {
            let _ = socket.emit::<TierlistPresenter>("tierlist", &TierlistPresenter::from(tierlist));
        }
    }
}