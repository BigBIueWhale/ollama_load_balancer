//! Freeze Server - A minimal TCP server for testing radio silence scenarios
//!
//! This server accepts TCP connections and can be paused via SIGSTOP to simulate
//! a "VM pause" scenario where the server becomes completely unresponsive without
//! sending any TCP packets (no FIN, no RST - complete radio silence).
//!
//! # Platform Support
//! This binary only works on Unix-like systems (Linux, macOS) because it relies
//! on POSIX signals (SIGSTOP/SIGCONT).
//!
//! # Usage
//! ```sh
//! # Start the server
//! freeze_server <port> [--send-partial <bytes>] [--mode <mode>]
//!
//! # Modes:
//! #   accept-only     - Accept connection then wait (default)
//! #   send-partial    - Accept, send N bytes, then wait
//! #   send-headers    - Accept, send HTTP headers, then wait
//! #   ollama-partial  - Accept, send partial Ollama streaming response, then wait
//! ```
//!
//! The server prints "READY" to stdout when it's listening, and "CONNECTED" when
//! a client connects. The test runner can then send SIGSTOP to freeze the process.

#[cfg(not(unix))]
compile_error!(
    "freeze_server only supports Unix-like systems (Linux, macOS). \
     Windows does not support SIGSTOP/SIGCONT signals. \
     To add Windows support, you would need to implement process suspension using: \
     - NtSuspendProcess/NtResumeProcess from ntdll.dll, or \
     - SuspendThread/ResumeThread for each thread, or \
     - DebugActiveProcess for debugging-based suspension"
);

#[cfg(unix)]
mod server {
    use std::env;
    use std::io::{self, Write};
    use std::net::{TcpListener, TcpStream};
    use std::process;
    use std::thread;
    use std::time::Duration;

    #[derive(Debug, Clone)]
    pub enum FreezeMode {
        /// Accept connection then wait forever (no data sent)
        AcceptOnly,
        /// Send N bytes of data then wait
        SendPartial { bytes: usize },
        /// Send HTTP headers then wait
        SendHeaders,
        /// Send partial Ollama streaming response then wait
        OllamaPartial { tokens: usize },
    }

    fn print_usage() {
        eprintln!("Usage: freeze_server <port> [options]");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --mode <mode>          Mode of operation:");
        eprintln!("    accept-only          Accept connection then wait (default)");
        eprintln!("    send-partial         Send partial data then wait");
        eprintln!("    send-headers         Send HTTP headers then wait");
        eprintln!("    ollama-partial       Send partial Ollama response then wait");
        eprintln!("  --bytes <n>            Bytes to send in send-partial mode (default: 100)");
        eprintln!("  --tokens <n>           Tokens to send in ollama-partial mode (default: 3)");
        eprintln!();
        eprintln!("The server prints 'READY' when listening and 'CONNECTED' when a client connects.");
        eprintln!("Send SIGSTOP to freeze the server (simulating VM pause / radio silence).");
        eprintln!("Send SIGCONT to resume.");
    }

    fn parse_args() -> Result<(u16, FreezeMode), String> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Err("Missing port argument".to_string());
        }

        let port: u16 = args[1].parse()
            .map_err(|_| format!("Invalid port: {}", args[1]))?;

        let mut mode = FreezeMode::AcceptOnly;
        let mut bytes = 100;
        let mut tokens = 3;
        let mut i = 2;

        while i < args.len() {
            match args[i].as_str() {
                "--help" | "-h" => {
                    print_usage();
                    process::exit(0);
                }
                "--mode" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("--mode requires an argument".to_string());
                    }
                    mode = match args[i].as_str() {
                        "accept-only" => FreezeMode::AcceptOnly,
                        "send-partial" => FreezeMode::SendPartial { bytes },
                        "send-headers" => FreezeMode::SendHeaders,
                        "ollama-partial" => FreezeMode::OllamaPartial { tokens },
                        other => return Err(format!("Unknown mode: {}", other)),
                    };
                }
                "--bytes" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("--bytes requires an argument".to_string());
                    }
                    bytes = args[i].parse()
                        .map_err(|_| format!("Invalid bytes: {}", args[i]))?;
                    if let FreezeMode::SendPartial { .. } = mode {
                        mode = FreezeMode::SendPartial { bytes };
                    }
                }
                "--tokens" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("--tokens requires an argument".to_string());
                    }
                    tokens = args[i].parse()
                        .map_err(|_| format!("Invalid tokens: {}", args[i]))?;
                    if let FreezeMode::OllamaPartial { .. } = mode {
                        mode = FreezeMode::OllamaPartial { tokens };
                    }
                }
                other => {
                    return Err(format!("Unknown argument: {}", other));
                }
            }
            i += 1;
        }

        // Reapply bytes/tokens in case they were specified before --mode
        match &mode {
            FreezeMode::SendPartial { .. } => mode = FreezeMode::SendPartial { bytes },
            FreezeMode::OllamaPartial { .. } => mode = FreezeMode::OllamaPartial { tokens },
            _ => {}
        }

        Ok((port, mode))
    }

    fn handle_connection(mut stream: TcpStream, mode: &FreezeMode) -> io::Result<()> {
        // Disable Nagle's algorithm for more predictable timing
        stream.set_nodelay(true)?;

        match mode {
            FreezeMode::AcceptOnly => {
                // Don't send anything, just hold the connection open
            }

            FreezeMode::SendPartial { bytes } => {
                // Send partial data
                let data = "X".repeat(*bytes);
                stream.write_all(data.as_bytes())?;
                stream.flush()?;
            }

            FreezeMode::SendHeaders => {
                // Send HTTP response headers but no body
                let headers = "HTTP/1.1 200 OK\r\n\
                              Content-Type: application/x-ndjson\r\n\
                              Transfer-Encoding: chunked\r\n\
                              \r\n";
                stream.write_all(headers.as_bytes())?;
                stream.flush()?;
            }

            FreezeMode::OllamaPartial { tokens } => {
                // Send HTTP headers
                let headers = "HTTP/1.1 200 OK\r\n\
                              Content-Type: application/x-ndjson\r\n\
                              Transfer-Encoding: chunked\r\n\
                              \r\n";
                stream.write_all(headers.as_bytes())?;

                // Send partial Ollama streaming response
                for i in 0..*tokens {
                    let token_response = format!(
                        r#"{{"model":"test-model","created_at":"2024-01-01T00:00:00Z","message":{{"role":"assistant","content":"token{}"}},"done":false}}"#,
                        i
                    );
                    // Send as chunked encoding
                    let chunk = format!("{:x}\r\n{}\n\r\n", token_response.len() + 1, token_response);
                    stream.write_all(chunk.as_bytes())?;
                    stream.flush()?;

                    // Small delay between tokens
                    thread::sleep(Duration::from_millis(20));
                }
                // Don't send the final "done: true" message or closing chunk
            }
        }

        // Now just wait forever - the test runner will SIGSTOP us
        println!("WAITING");
        loop {
            thread::sleep(Duration::from_secs(3600));
        }
    }

    pub fn main() {
        let (port, mode) = match parse_args() {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error: {}", e);
                print_usage();
                process::exit(2);
            }
        };

        // Bind to the port
        let listener = match TcpListener::bind(("127.0.0.1", port)) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to bind to port {}: {}", port, e);
                process::exit(2);
            }
        };

        // Print PID for test runner to use with SIGSTOP
        eprintln!("PID: {}", process::id());
        eprintln!("Mode: {:?}", mode);

        // Signal that we're ready
        println!("READY");
        // Flush stdout to ensure READY is immediately visible
        io::stdout().flush().unwrap();

        // Accept one connection (we only need to test one)
        match listener.accept() {
            Ok((stream, addr)) => {
                eprintln!("Connection from: {}", addr);
                println!("CONNECTED");
                io::stdout().flush().unwrap();

                if let Err(e) = handle_connection(stream, &mode) {
                    eprintln!("Connection error: {}", e);
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Accept error: {}", e);
                process::exit(1);
            }
        }
    }
}

#[cfg(unix)]
fn main() {
    server::main();
}

#[cfg(not(unix))]
fn main() {
    // This will never be reached due to compile_error! above
    unreachable!()
}
