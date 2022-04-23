use std::{collections::HashMap, convert::Infallible, sync::Arc};

use client::Client;
use tokio::sync::RwLock;
use warp::Filter;

mod client;
mod handler;
mod ws;

type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
