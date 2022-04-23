use warp::{Rejection, Reply};

use crate::{ws, Clients};

pub async fn ws_handler(
    ws: warp::ws::Ws,
    id: String,
    clients: Clients,
) -> Result<impl Reply, Rejection> {
    let client = clients.read().await.get(&id).cloned();
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}
