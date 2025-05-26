use rand::random;
use serde::Serialize;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use surge_ping::{Client, Config, PingIdentifier, PingSequence};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

// State to track active ping tasks
// #[derive(Default)]
struct AppState {
    // client: Arc<Client>,
    tasks: Arc<Mutex<HashMap<String, (tauri::async_runtime::JoinHandle<()>, mpsc::Sender<()>)>>>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PingResult {
    host: String,
    timestamp: u128,
    duration: u128,
    status: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(AppState {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        })
        .invoke_handler(tauri::generate_handler![start_pinging, pause_pinging])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn start_pinging(
    ips: Vec<String>,
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut tasks = state.tasks.lock().await;
    let mut errors = Vec::new();

    // Create a shared ping client with default config
    let client = Arc::new(
        Client::new(&Config::default())
            .map_err(|e| format!("Failed to create ping client: {}", e))?,
    );

    for ip in ips {
        // Parse IP address
        let ip_addr: IpAddr = match ip.parse() {
            Ok(addr) => addr,
            Err(e) => {
                errors.push(format!("Invalid IP {}: {}", ip, e));
                continue;
            }
        };

        if tasks.contains_key(&ip) {
            errors.push(format!("Task for IP {} already running", ip));
            continue;
        }

        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>(1);
        let ip_clone = ip.clone();
        let client_clone = client.clone();
        let app_handle_clone = app_handle.clone();
        let handle = tauri::async_runtime::spawn(async move {
            ping_ip(
                ip_clone,
                ip_addr,
                client_clone,
                shutdown_rx,
                app_handle_clone,
            )
            .await;
        });

        tasks.insert(ip.clone(), (handle, shutdown_tx));
        eprintln!("Started task for IP {}", ip);
    }

    if !errors.is_empty() {
        return Err(errors.join("; "));
    }
    Ok(())
}

#[tauri::command]
async fn pause_pinging(ips: Vec<String>, state: State<'_, AppState>) -> Result<(), String> {
    let mut tasks = state.tasks.lock().await;
    eprintln!("Stopping IPs: {:?}", ips);
    let mut errors = Vec::new();

    for ip in &ips {
        if let Some((handle, shutdown_tx)) = tasks.remove(ip) {
            // Send shutdown signal for graceful exit
            if let Err(e) = shutdown_tx.send(()).await {
                eprintln!("Failed to send shutdown signal for IP {}: {}", ip, e);
                errors.push(format!(
                    "Failed to send shutdown signal for IP {}: {}",
                    ip, e
                ));
            }

            // Await the task to ensure it has stopped
            match tokio::time::timeout(std::time::Duration::from_secs(5), handle).await {
                Ok(Ok(_)) => eprintln!("Task for IP {} stopped successfully", ip),
                Ok(Err(e)) => {
                    eprintln!("Failed to stop task for IP {}: {}", ip, e);
                    errors.push(format!("Failed to stop task for IP {}: {}", ip, e));
                }
                Err(_) => {
                    eprintln!("Task for IP {} timed out", ip);
                    errors.push(format!("Task for IP {} timed out", ip));
                }
            }
        } else {
            eprintln!("No task found for IP {}", ip);
            errors.push(format!("No task found for IP {}", ip));
        }
    }

    if !errors.is_empty() {
        return Err(errors.join("; "));
    }
    Ok(())
}

// Ping logic for a single IP, running until shutdown signal is received
async fn ping_ip(
    ip: String,
    ip_addr: IpAddr,
    client: Arc<Client>,
    mut shutdown_rx: mpsc::Receiver<()>,
    app_handle: AppHandle,
) {
    let identifier = PingIdentifier(random::<u16>());
    let mut sequence = PingSequence(0);
    let payload = [0u8; 8]; // 8-byte payload for ping

    loop {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                eprintln!("Stopping ping task for IP {}", ip);
                break;
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => {
                let result: PingResult = match client.pinger(ip_addr, identifier).await.ping(sequence, &payload).await {
                    Ok((_packet, duration)) => {
                        eprintln!("Ping to {} (seq {}): {}ms", ip, sequence, duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1_000_000_000.0);
                        PingResult {
                            host: ip.clone(),
                            duration: duration.as_millis(),
                            timestamp: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards")
                                .as_millis(),
                            status: "success".to_string(),
                        }
                    }
                    Err(e) => {
                        eprintln!("Ping to {} failed: {}", ip, e);
                        PingResult {
                            host: ip.clone(),
                            duration: 0,
                            timestamp: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards")
                                .as_millis(),
                            status: format!("error: {}", e),
                        }
                    }
                };
                sequence.0 += 1; // Increment sequence number

                        if let Err(e) = app_handle.emit("ping-result", &result) {
            eprintln!("Failed to emit ping result for {}: {}", ip, e);
        }
            }
        }
    }
}

// #[tauri::command]
// async fn start_pinging(
//     app_handle: AppHandle,
//     ips: Vec<String>,
//     state: State<'_, AppState>,
// ) -> Result<(), String> {
//     let client = Client::new(&Config::default()).expect("Failed to create ping client");
//     // Create a single surge-ping client
//     let mut tasks = state.tasks.lock().await;
//     eprintln!("Start IPs: {:?}", ips);
//     for ip in ips {
//         // Check if a task for this IP is already running
//         if tasks.contains_key(&ip) {
//             let result = format!("Task for {} already running", ip);
//             if let Err(e) = app_handle.emit("ping-result", &result) {
//                 eprintln!("Failed to emit ping result for {}: {}", ip, e);
//             }
//             continue;
//         }

//         let app_handle = app_handle.clone();
//         // let client = client;
//         // let client = ;
//         let ip_clone = ip.clone();
//         let task = tokio::spawn(ping_ip(app_handle, client.clone(), ip_clone));
//         tasks.insert(ip, task);
//     }

//     Ok(())
// }

// #[tauri::command]
// async fn pause_pinging(ips: Vec<String>, state: State<'_, AppState>) -> Result<(), String> {
//     let mut tasks = state.tasks.lock().await;
//     eprintln!("Stop IPs: {:?}", ips);
//     for ip in ips {
//         if let Some(handle) = tasks.remove(&ip) {
//             handle.abort();
//         }
//     }

//     Ok(())
// }

// async fn ping_ip(app_handle: AppHandle, client: Client, ip: String) {
//     let ip_addr: IpAddr = match ip.parse() {
//         Ok(addr) => addr,
//         Err(e) => {
//             let result = format!("Invalid IP {}: {}", ip, e);
//             if let Err(e) = app_handle.emit("ping-result", &result) {
//                 eprintln!("Failed to emit ping result for {}: {}", ip, e);
//             }
//             return;
//         }
//     };

//     // Use a random identifier for this pinger
//     let identifier = PingIdentifier(rand::random::<u16>());
//     let mut sequence = PingSequence(0);

//     loop {
//         let result: PingResult = match client
//             .pinger(ip_addr, identifier)
//             .await
//             .ping(sequence, &[0; 8]) // 8-byte payload
//             .await
//         {
//             Ok((_, duration)) => PingResult {
//                 host: ip.clone(),
//                 duration: duration.as_millis(),
//                 timestamp: SystemTime::now()
//                     .duration_since(UNIX_EPOCH)
//                     .expect("Time went backwards")
//                     .as_millis(),
//                 status: "success".to_string(),
//             },
//             Err(e) => PingResult {
//                 host: ip.clone(),
//                 duration: 0,
//                 timestamp: SystemTime::now()
//                     .duration_since(UNIX_EPOCH)
//                     .expect("Time went backwards")
//                     .as_millis(),
//                 status: format!("error: {}", e),
//             },
//         };

//         // Emit result to frontend
//         if let Err(e) = app_handle.emit("ping-result", &result) {
//             eprintln!("Failed to emit ping result for {}: {}", ip, e);
//         }

//         sequence = PingSequence(sequence.0.wrapping_add(1)); // Increment sequence number
//         time::sleep(Duration::from_secs(2)).await;
//     }
// }
