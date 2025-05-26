use std::net::IpAddr;
use std::time::Duration;
use surge_ping::{Client, Config, PingIdentifier, PingSequence, ICMP};
use tauri::{AppHandle, Manager};
use tokio::time;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_pinging])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn start_pinging(app_handle: AppHandle, ips: Vec<String>) {
    // Create a single surge-ping client
    let client = Client::new(&Config::default()).expect("Failed to create ping client");

    for ip in ips {
        let app_handle = app_handle.clone(); // Safe to clone, as AppHandle is Arc-backed
        let client = client.clone(); // Client is Arc-backed and safe to clone
        tokio::spawn(ping_ip(app_handle, client, ip));
    }
}

async fn ping_ip(app_handle: AppHandle, client: Client, ip: String) {
    let ip_addr: IpAddr = match ip.parse() {
        Ok(addr) => addr,
        Err(e) => {
            let result = format!("Invalid IP {}: {}", ip, e);
            if let Err(e) = app_handle.emit("ping-result", &result) {
                eprintln!("Failed to emit ping result for {}: {}", ip, e);
            }
            return;
        }
    };

    // Use a random identifier for this pinger
    let identifier = PingIdentifier(rand::random::<u16>());
    let mut sequence = PingSequence(0);

    loop {
        let result = match client
            .pinger(ip_addr, identifier)
            .ping(sequence, &[0; 8]) // 8-byte payload
            .await
        {
            Ok((_, duration)) => {
                format!("Ping to {} successful, time={}ms", ip, duration.as_millis())
            }
            Err(e) => format!("Ping to {} failed: {:?}", ip, e),
        };

        // Emit result to frontend
        if let Err(e) = app_handle.emit("ping-result", &result) {
            eprintln!("Failed to emit ping result for {}: {}", ip, e);
        }

        sequence = sequence.next();
        time::sleep(Duration::from_secs(2)).await;
    }
}
