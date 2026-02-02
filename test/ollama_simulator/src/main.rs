//! Ollama Simulator - A realistic mock Ollama server for testing ollama_load_balancer
//!
//! This simulator provides programmatic control over server behavior to enable
//! automated testing of all edge cases documented in the main README.md.
//!
//! # Architecture
//!
//! The simulator consists of:
//! - Multiple simulated Ollama servers, each on their own port
//! - A control API server for programmatic test control
//! - Realistic HTTP response behavior matching actual Ollama
//!
//! # Usage
//!
//! Run with: `cargo run --bin ollama_simulator -- --control-port 11500 --server-ports 11501,11502,11503`

mod simulator;
mod control;
mod types;

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::simulator::SimulatorState;
use crate::control::run_control_server;
use crate::simulator::run_ollama_server;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = std::env::args().collect();

    let mut control_port: u16 = 11500;
    let mut server_ports: Vec<u16> = vec![11501, 11502, 11503];

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--control-port" => {
                i += 1;
                if i >= args.len() {
                    return Err("--control-port requires a value".into());
                }
                control_port = args[i].parse().map_err(|e| {
                    format!("Invalid control port '{}': {}", args[i], e)
                })?;
            }
            "--server-ports" => {
                i += 1;
                if i >= args.len() {
                    return Err("--server-ports requires a value".into());
                }
                server_ports = args[i]
                    .split(',')
                    .map(|s| s.trim().parse::<u16>())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| format!("Invalid server ports '{}': {}", args[i], e))?;
            }
            "--help" | "-h" => {
                println!("Ollama Simulator - Mock Ollama servers for testing");
                println!();
                println!("Usage: ollama_simulator [OPTIONS]");
                println!();
                println!("Options:");
                println!("  --control-port PORT     Control API port (default: 11500)");
                println!("  --server-ports PORTS    Comma-separated list of server ports (default: 11501,11502,11503)");
                println!("  --help, -h              Show this help message");
                return Ok(());
            }
            other => {
                return Err(format!("Unknown argument: {}", other).into());
            }
        }
        i += 1;
    }

    if server_ports.is_empty() {
        return Err("At least one server port is required".into());
    }

    // Check for port conflicts
    if server_ports.contains(&control_port) {
        return Err(format!("Control port {} conflicts with server ports", control_port).into());
    }

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("Failed to create tokio runtime: {}", e))?;

    runtime.block_on(async move {
        run_simulator(control_port, server_ports).await
    })
}

async fn run_simulator(
    control_port: u16,
    server_ports: Vec<u16>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create shared state
    let state = Arc::new(RwLock::new(SimulatorState::new(server_ports.clone())));

    println!("Ollama Simulator starting...");
    println!("  Control API: http://127.0.0.1:{}", control_port);
    for port in &server_ports {
        println!("  Ollama server: http://127.0.0.1:{}", port);
    }
    println!();

    // Spawn all servers
    let mut handles = Vec::new();

    // Control server
    let state_clone = state.clone();
    handles.push(tokio::spawn(async move {
        if let Err(e) = run_control_server(control_port, state_clone).await {
            eprintln!("Control server error: {}", e);
        }
    }));

    // Ollama servers
    for port in server_ports {
        let state_clone = state.clone();
        handles.push(tokio::spawn(async move {
            if let Err(e) = run_ollama_server(port, state_clone).await {
                eprintln!("Ollama server {} error: {}", port, e);
            }
        }));
    }

    // Wait for shutdown signal
    tokio::signal::ctrl_c().await.map_err(|e| {
        format!("Failed to listen for ctrl_c: {}", e)
    })?;

    println!("\nShutting down simulator...");

    // Cancel all tasks
    for handle in handles {
        handle.abort();
    }

    Ok(())
}
