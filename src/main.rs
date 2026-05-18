use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio::sync::broadcast;
use std::sync::Arc;
use tracing::{info, warn, error};
use tracing_subscriber;

use lumen as lib;
use lib::SecurityEngine;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    info!("🛡️ Lumen Security Telemetry Engine v0.2.0");
    info!("Listening on port 9999 for logs...");
    
    let (shutdown_tx, _) = broadcast::channel(1);
    
    // Initialize security engine with rules
    let engine = Arc::new(SecurityEngine::new(vec![
        "SELECT".to_string(),
        "DROP TABLE".to_string(),
        "<script>".to_string(),
        "javascript:".to_string(),
        "cc_number".to_string(),
        "password".to_string(),
    ]));
    
    let (tx, mut rx) = mpsc::channel::<String>(1000);
    
    let engine_clone = engine.clone();
    let mut shutdown_rx = shutdown_tx.subscribe();
    let sanitizer_handle = tokio::spawn(async move {
        let mut clean_count = 0u64;
        let mut quarantine_count = 0u64;
        
        loop {
            tokio::select! {
                Some(log_line) = rx.recv() => {
                    let is_threat = engine_clone.check_threat(&log_line).await;
                    
                    if is_threat {
                        quarantine_count += 1;
                        warn!("[QUARANTINED #{}] {}", quarantine_count, log_line);
                    } else {
                        clean_count += 1;
                        info!("[CLEAN #{}] {}", clean_count, log_line);
                    }
                }
                _ = shutdown_rx.recv() => {
                    info!("Sanitizer shutting down. Stats: {} clean, {} quarantined", 
                          clean_count, quarantine_count);
                    break;
                }
            }
        }
    });
    
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    let tx_clone = tx.clone();
    let mut shutdown_rx_ingestor = shutdown_tx.subscribe();
    
    let ingestor_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                accept_result = listener.accept() => {
                    match accept_result {
                        Ok((socket, addr)) => {
                            info!("New connection from: {}", addr);
                            let tx = tx_clone.clone();
                            let mut reader = BufReader::new(socket);
                            let mut line = String::new();
                            
                            tokio::spawn(async move {
                                loop {
                                    line.clear();
                                    match reader.read_line(&mut line).await {
                                        Ok(0) => break,
                                        Ok(_) => {
                                            let log = line.trim().to_string();
                                            if !log.is_empty() {
                                                if let Err(e) = tx.send(log).await {
                                                    error!("Failed to send log: {}", e);
                                                    break;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            error!("Error reading from socket: {}", e);
                                            break;
                                        }
                                    }
                                }
                            });
                        }
                        Err(e) => error!("Connection error: {}", e),
                    }
                }
                _ = shutdown_rx_ingestor.recv() => {
                    info!("Ingestor shutting down...");
                    break;
                }
            }
        }
    });
    
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");
    let _ = shutdown_tx.send(());
    
    tokio::select! {
        _ = ingestor_handle => {},
        _ = sanitizer_handle => {},
    }
    
    Ok(())
}
