use tauri::async_runtime::spawn;
use tokio::time::sleep as async_sleep;
use std::sync::{Arc, Mutex};
use warp::Filter;
use warp::ws::{WebSocket, Ws};
use std::net::SocketAddr;
use futures_util::StreamExt;
use super::app;

pub async fn run_websocket_server(addr: SocketAddr, app_handle: tauri::AppHandle<tauri::Wry>, conn: Arc<Mutex<rusqlite::Connection>>) {
    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || app_handle.clone()))
        .and(warp::any().map(move || conn.clone()))
        .map(|ws: Ws, app_handle: tauri::AppHandle<tauri::Wry>, conn: Arc<Mutex<rusqlite::Connection>>| {
            ws.on_upgrade(move |websocket| handle_connection(websocket, app_handle, conn))
        });

    warp::serve(websocket_route).run(addr).await;
}

async fn handle_connection(websocket: WebSocket, _app_handle: tauri::AppHandle, conn: Arc<Mutex<rusqlite::Connection>>) {
    let (_tx, mut rx) = websocket.split();  // Correct unused and mutable unnecessary issues

    while let Some(result) = rx.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() {
                    // Handle incoming messages
                }
                if msg.is_close() {
                    break;
                }
            }
            Err(e) => eprintln!("websocket error: {:?}", e),
        }
    }
}
