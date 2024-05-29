use std::sync::{Arc, Mutex};
use warp::Filter;
use warp::ws::{WebSocket, Ws};
use std::net::SocketAddr;
use futures_util::StreamExt;
use rusqlite::{Connection, params, Result as SqlResult};
use serde_json::Value;

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

pub fn update_time_spent(conn: &Connection, window_id: i64, duration: i64) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO time (window_id, duration) VALUES (?, ?)",
        params![window_id, duration],
    )?;

    Ok(())
}

async fn handle_connection(websocket: WebSocket, _app_handle: tauri::AppHandle, conn: Arc<Mutex<rusqlite::Connection>>) {
    let (_tx, mut rx) = websocket.split();  // Correct unused and mutable unnecessary issues

    while let Some(result) = rx.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() {
                    let data = msg.to_str().unwrap();
                    println!("Received message: {}", data);

                    match serde_json::from_str::<Value>(data) {
                        Ok(parsed) => {
                            if let (Some(app_name), Some(duration)) = (
                                parsed["app_name"].as_str(),
                                parsed["duration"].as_i64(),
                            ) {
                                let db_conn = conn.lock().unwrap();
                                match update_time_spent(&db_conn, app_name.parse().unwrap(), duration) {
                                    Ok(_) => println!("Database updated successfully"),
                                    Err(e) => eprintln!("Error updating database: {}", e),
                                }
                            }
                        }
                        Err(e) => eprintln!("Error parsing JSON: {}", e),
                    }
                }
                if msg.is_close() {
                    break;
                }
            }
            Err(e) => eprintln!("websocket error: {:?}", e),
        }
    }
}
