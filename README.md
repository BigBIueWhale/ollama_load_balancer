# Ollama Load Balancer
Autonomous Rust utility that load balances multiple https://ollama.com/ servers

![project logo small](./doc/logo/logo_small.png)

Designed to be used with https://github.com/BigBIueWhale/openwebui_config or [continue.dev](https://www.continue.dev/)

## Release Notes

### 1.0.3
https://github.com/BigBIueWhale/ollama_load_balancer/blob/RLS_01_00_03_2025_01_28/release

**Changes:**
- Print activity status list of all servers every time something changes

- Breaking change- human-readable name must be specified in CLI arguments.

### 1.0.2
https://github.com/BigBIueWhale/ollama_load_balancer/blob/RLS_01_00_02_2024_11_17/release/ollama_load_balancer.exe

**Changes:**
- Fix: Support any HTTP request type in addition to `POST`. This includes `GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS`, `PATCH`, `TRACE`.\
The https://openwebui.com/ server was trying to use `GET /api/tags` so this feature is required for openwebui servers to agree to use this load balancer.

### 1.0.1
https://github.com/BigBIueWhale/ollama_load_balancer/blob/RLS_01_00_01_2024_10_22/release/ollama_load_balancer.exe

**Changes:**
- Style: Avoid confusing print that includes green status `🟢 Server {} now available` when just after a failure.
- Logic: Fix premature demoting of `Unreliable` server to `SecondChanceGiven`- that would cause bug where if user cancels generation mid-stream, an `Unreliable` server would be marked as `SecondChanceGiven` despite no failure occurring.
- Logic: Fix bug where server gets marked as `Reliable` before stream ends and is successful- that would cause a server that fails every time mid-stream to ruin the user experience.
- Code: Refactor- Use server "ADDRESS:PORT" as key to data structure holding the state of all servers, instead of holding Arc reference to specific server, this avoids needing multiple locks, improves performance, and fixes logical race condition caused by multiple locks.
- Doc: Optimize documentation for end-users

### 1.0.0
https://github.com/BigBIueWhale/ollama_load_balancer/blob/RLS_01_00_00_2024_10_22/release/ollama_load_balancer.exe

**Features:**
- Standalone command-line executable for Windows 10/11 with app icon, linked with MSVC 64-bit toolchain
- Tested on `Windows 11 Pro 23H2`
- Source code is cross platform- compile works on Ubuntu 22.04
- Load balancing implemented
- Streaming HTTP POST request handled by utility
- Robust error handling- edge cases managed
- Well-documented
- Easy-to-read emoji logs to console
- Configurable timeout via command line argument
- Configurable Ollama servers `IP:PORT` list via command line arguments
- Stateless- no saved state between executable runs, no configuration files- all CLI
- Supports any REST server based on `HTTP POST` requests, not just Ollama.
- Optimized for immediate response to user- avoid user needing to wait
- Ideal server-ranking implementation for performance-identical Ollama servers in chaotic environment where they can be turned on and off on a whim.

## Installation


### Executable

Download the [latest release](#release-notes) executable

### Nix (MacOS, Linux)

```
nix --extra-experimental-features "nix-command flakes" profile install "github:BigBIueWhale/ollama_load_balancer"
```

### Docker

```
docker build -t bigbluewhale/ollama_load_balancer .
docker run -it --rm -p 11434 bigbluewhale/ollama_load_balancer --server="http://192.168.150.134:11434=james" --server="http://192.168.150.135:11434=sara"
```

## Usage

1. Run in Powershell, CMD, or terminal. Make sure to [allow access to both public and private networks](./doc/screenshots/allow_access_to_public_and_private_networks.png) during the first time running the utility.
```sh
C:\Users\user\Downloads>ollama_load_balancer.exe --version
ollama_load_balancer 1.0.3

C:\Users\user\Downloads>ollama_load_balancer.exe --server "http://192.168.150.134:11434=James' server" --server "http://192.168.150.135:11434=Sara's server" --server "http://192.168.150.136:11434=Mark's server"

📒 Ollama servers list:
1. http://192.168.150.134:11434 (James' server)
2. http://192.168.150.135:11434 (Sara's server)
3. http://192.168.150.136:11434 (Mark's server)

⚙️  Timeout setting: Will abandon Ollama server after 30 seconds of silence

👂 Ollama Load Balancer listening on http://0.0.0.0:11434

🤖🦸 Chose reliable server: http://192.168.150.134:11434 (James' server) to serve client 127.0.0.1:54619
🗒  Current server statuses:
1. Address: http://192.168.150.134:11434 (James' server), Busy: Busy, Reliability: Reliable
2. Address: http://192.168.150.135:11434 (Sara's server), Busy: Available, Reliability: Reliable
3. Address: http://192.168.150.136:11434 (Mark's server), Busy: Available, Reliability: Reliable

🤖🦸 Chose reliable server: http://192.168.150.135:11434 (Sara's server) to serve client 127.0.0.1:54621
🗒  Current server statuses:
1. Address: http://192.168.150.134:11434 (James' server), Busy: Busy, Reliability: Reliable
2. Address: http://192.168.150.135:11434 (Sara's server), Busy: Busy, Reliability: Reliable
3. Address: http://192.168.150.136:11434 (Mark's server), Busy: Available, Reliability: Reliable

🤖🦸 Chose reliable server: http://192.168.150.136:11434 (Mark's server) to serve client 127.0.0.1:54623
🗒  Current server statuses:
1. Address: http://192.168.150.134:11434 (James' server), Busy: Busy, Reliability: Reliable
2. Address: http://192.168.150.135:11434 (Sara's server), Busy: Busy, Reliability: Reliable
3. Address: http://192.168.150.136:11434 (Mark's server), Busy: Busy, Reliability: Reliable

🗒  Current server statuses:
1. Address: http://192.168.150.134:11434 (James' server), Busy: Busy, Reliability: Reliable
2. Address: http://192.168.150.135:11434 (Sara's server), Busy: Busy, Reliability: Reliable
3. Address: http://192.168.150.136:11434 (Mark's server), Busy: Busy, Reliability: Reliable

🤷 No available servers to serve client 127.0.0.1:54625
🗒  Current server statuses:
1. Address: http://192.168.150.134:11434 (James' server), Busy: Busy, Reliability: Reliable
2. Address: http://192.168.150.135:11434 (Sara's server), Busy: Busy, Reliability: Reliable
3. Address: http://192.168.150.136:11434 (Mark's server), Busy: Busy, Reliability: Reliable

🟢 Server http://192.168.150.134:11434 (James' server) now available
🗒  Current server statuses:
1. Address: http://192.168.150.134:11434 (James' server), Busy: Available, Reliability: Reliable
2. Address: http://192.168.150.135:11434 (Sara's server), Busy: Busy, Reliability: Reliable
3. Address: http://192.168.150.136:11434 (Mark's server), Busy: Busy, Reliability: Reliable

🟢 Server http://192.168.150.135:11434 (Sara's server) now available
🗒  Current server statuses:
1. Address: http://192.168.150.134:11434 (James' server), Busy: Available, Reliability: Reliable
2. Address: http://192.168.150.135:11434 (Sara's server), Busy: Available, Reliability: Reliable
3. Address: http://192.168.150.136:11434 (Mark's server), Busy: Busy, Reliability: Reliable

🟢 Server http://192.168.150.136:11434 (Mark's server) now available
🗒  Current server statuses:
1. Address: http://192.168.150.134:11434 (James' server), Busy: Available, Reliability: Reliable
2. Address: http://192.168.150.135:11434 (Sara's server), Busy: Available, Reliability: Reliable
3. Address: http://192.168.150.136:11434 (Mark's server), Busy: Available, Reliability: Reliable

☠️  Received CTRL+C, shutting down gracefully...

C:\Users\user\Downloads>
```

In this example, we set up 4 VS Code instances (to simulate users) and turn on 3 Ollama servers. We first quickly request LLM chat completion from all four users- Three manage, but the fourth causes: `🤷 No available servers to serve client`

## Purpose
A single Ollama server can (and should) only serve one request at the same time.

Hardware for an Ollama server is expensive. This load balancer allows to distribute a limited number of Ollama servers optimally to multiple users on a local network.

Let's say you have 60 users of an LLM service and 6 Ollama servers. What's the probability that 10% or more of your users are prompting the LLM at the same time?

## Principal of Operation
All users on the network configure their `continue.dev` (VS Code extension) to point to the IP address of this load balancer instead of manually choosing a specific Ollama server.

Any HTTP POST request for an LLM completion from a user triggers this utility to make an identical HTTP POST request to a real Ollama server on bahalf of the user. All while streaming the response back to the user.

We only choose servers that are currently available, we can know which Ollama servers are available based on the assumption that users only access the Ollama servers via this load balancer.

### Unreliable Servers

We assume that the list of Ollama servers isn't perfect.\
A servers might be temporarily or permanently off, a server might have changed its IP address.\
A server might be faulty- fail every time.

Therefore we introduced a state for each server: `failure_record: FailureRecord`

```rs
enum FailureRecord {
    Reliable,
    Unreliable,
    SecondChanceGiven,
}
```

We want to avoid a bad server from causing the user experience to be unreliable when using this load balancer.

Therefore if a server fails during a request, we mark it as `Unreliable`.\
We only choose an unreliable server to process a request if there's no `Reliable` server available (not `busy`)

If an `Unreliable` server is given a chance to repent, and it succeeds to process a request, then it's marked as `Reliable` again, because that most likely means that somebody turned the PC and the Ollama server on.

Question is: how do we choose from multiple possible `Unreliable` servers? How do we make sure that they all get a timely chance to repent?

That's what `SecondChanceGiven` is for. It's a state that we can flip to ensure that we cycle through all `Unreliable` servers evenly, avoiding the situation where we try a single `Unreliable` server twice to no avail while ignoring the other (possibly good) servers.

## Supported Usages
We support `continue.dev` and also any client that streams responses from an Ollama server such as https://openwebui.com/

We support `/api/chat`, `/api/generate` (CTRL+i in `continue.dev`), `/api/show`, and actually we support any POST request. We support streaming based on `Transfer-Encoding: chunked` and `Content-Type: application/x-ndjson`.

Static HTTP is also supported with `stream: false` in JSON given in POST request to Ollama.

## Streaming

The LLM doesn't have the complete response immediately which is why Ollama streams the completions.

Streaming is implemented using `Newline Delimited JSON format` (ndjson). See `Content-Type: application/x-ndjson`, although this format is not hard-coded in the load balancer.

Each line of the ndjson format is mapped to one object in a JSON array.

## Dependencies
These are the versions I used:

- cargo 1.82.0 (8f40fc59f 2024-08-21) on `Windows 11 Pro 23H2`

- Ollama version 0.3.13 on `Windows 10 Pro 22H2`

- VS Code version 1.90.2 on `Windows 11 Pro 23H2`

- `Continue - Codestral, Claude, and more` VS Code extension by `Continue` version 0.8.46 - 2024-08-11

- `rust-analyzer` v0.3.2146 by `The Rust Programming Language`

## Lab testing
1. Use a Windows host with at least 64 gigabytes of RAM and at least 8 CPU cores so that you can run [three virtual machines at the same time](./doc/screenshots/virtual_machines_running_ollama.png).

2. While the virtual machines are connected to the internet, install Ollama and run `ollama pull deepseek-coder:1.3b-instruct-q4_0`. Then kill Ollama from the Windows tray by right-clicking the tray icon. We choose this specific model because it has acceptable performance in CPU mode, and doesn't use much VRAM.

3. Set each virtual machine to be connected with a [host only network adapter](./doc/screenshots/virtual_machine_settings_host_only_network_adapter.png) so that the host (running the load balancer) has access to three Ollama servers on the local network. Now the VMs don't have world wide web internet access anymore.

4. Instead of running `ollama serve`, use [this batch file](https://github.com/BigBIueWhale/assistant_coder/blob/3cfa95ed35605e1d07fea4f8c479729eb0bfb9c9/run_ollama.bat) in each virtual machine so that Ollama runs on all network interfaces (`0.0.0.0`) instead of localhost.

5. [Find out the IP addresses](./doc/screenshots/virtual_machines_ip_addresses.png) of the virtual machines that VMWare decided to assign.\
**Adjust the server configuration** to point to the correct IP addresses of your Ollama servers.

6. Configure `continue.dev` (VS Code extension) to access the Ollama server at: `http://127.0.0.1:11434/` because in lab testing we're running the load balancer on the host- the same device running VS Code.

    The `continue.dev` VS Code extension config.json:
    ```json
    {
      "models": [
        {
          "title": "DeepSeek Coder",
          "provider": "ollama",
          "apiBase": "http://127.0.0.1:11434/",
          "model": "deepseek-coder:1.3b-instruct-q4_0",
          "contextLength": 4096
        }
      ],
      "tabAutocompleteOptions": {
        "disable": true
      },
      "completionOptions": {
        "maxTokens": 2048
      },
      "allowAnonymousTelemetry": false,
      "docs": []
    }
    ```
7. Open multiple instances of VS Code to prompt the LLM concurrently and test-out the load balancer.

## Edge Cases
### VM running Ollama crashes while generating- timeout 30 seconds (default)
```sh
C:\Users\user\Downloads>ollama_load_balancer.exe --version
ollama_load_balancer 1.0.1

C:\Users\user\Downloads>ollama_load_balancer.exe --server http://192.168.150.134:11434

📒 Ollama servers list:
1. http://192.168.150.134:11434

⚙️  Timeout setting: Will abandon Ollama server after 30 seconds of silence

👂 Ollama Load Balancer listening on http://0.0.0.0:11434

🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64266
⛔😱 Server http://192.168.150.134:11434 failed during streaming, now marked Unreliable. Error: error decoding response body
⚠️  Connection closed with Unreliable Server http://192.168.150.134:11434
☠️  Received CTRL+C, shutting down gracefully...

C:\Users\user\Downloads>
```

### PC running Ollama server was off (hard-coded 1 second initial-connection timeout)
```sh
C:\Users\user\Downloads>ollama_load_balancer.exe --version
ollama_load_balancer 1.0.1

C:\Users\user\Downloads>ollama_load_balancer.exe --server http://192.168.150.134:11434

📒 Ollama servers list:
1. http://192.168.150.134:11434

⚙️  Timeout setting: Will abandon Ollama server after 30 seconds of silence

👂 Ollama Load Balancer listening on http://0.0.0.0:11434

🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64288
⛔😱 Server http://192.168.150.134:11434 didn't respond, now marked Unreliable. Error: error sending request for url (http://192.168.150.134:11434/api/chat)
⚠️  Connection closed with Unreliable Server http://192.168.150.134:11434
☠️  Received CTRL+C, shutting down gracefully...

C:\Users\user\Downloads>
```

### All three servers are off, until 192.168.150.134 is turned back on- but then fails during streaming
```sh
C:\Users\user\Downloads>ollama_load_balancer.exe --version
ollama_load_balancer 1.0.1

C:\Users\user\Downloads>ollama_load_balancer.exe --server http://192.168.150.134:11434 --server http://192.168.150.135:11434 --server http://192.168.150.136:11434

📒 Ollama servers list:
1. http://192.168.150.134:11434
2. http://192.168.150.135:11434
3. http://192.168.150.136:11434

⚙️  Timeout setting: Will abandon Ollama server after 30 seconds of silence

👂 Ollama Load Balancer listening on http://0.0.0.0:11434

🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64308
⛔😱 Server http://192.168.150.134:11434 didn't respond, now marked Unreliable. Error: error sending request for url (http://192.168.150.134:11434/api/chat)
⚠️  Connection closed with Unreliable Server http://192.168.150.134:11434
🤖🦸 Chose reliable server: http://192.168.150.135:11434 to serve client 127.0.0.1:64310
⛔😱 Server http://192.168.150.135:11434 didn't respond, now marked Unreliable. Error: error sending request for url (http://192.168.150.135:11434/api/chat)
⚠️  Connection closed with Unreliable Server http://192.168.150.135:11434
🤖🦸 Chose reliable server: http://192.168.150.136:11434 to serve client 127.0.0.1:64312
⛔😱 Server http://192.168.150.136:11434 didn't respond, now marked Unreliable. Error: error sending request for url (http://192.168.150.136:11434/api/chat)
⚠️  Connection closed with Unreliable Server http://192.168.150.136:11434
🤖😇 Giving server http://192.168.150.134:11434 another chance with client 127.0.0.1:64315
⛔😞 Unreliable server http://192.168.150.134:11434 didn't respond. Error: error sending request for url (http://192.168.150.134:11434/api/chat)
⚠️  Connection closed with Unreliable Server http://192.168.150.134:11434
🤖😇 Giving server http://192.168.150.135:11434 another chance with client 127.0.0.1:64317
⛔😞 Unreliable server http://192.168.150.135:11434 didn't respond. Error: error sending request for url (http://192.168.150.135:11434/api/chat)
⚠️  Connection closed with Unreliable Server http://192.168.150.135:11434
🤖😇 Giving server http://192.168.150.136:11434 another chance with client 127.0.0.1:64319
⛔😞 Unreliable server http://192.168.150.136:11434 didn't respond. Error: error sending request for url (http://192.168.150.136:11434/api/chat)
⚠️  Connection closed with Unreliable Server http://192.168.150.136:11434
🤖😇 Giving server http://192.168.150.134:11434 a 3rd+ chance with client 127.0.0.1:64325
⛔😞 Unreliable server http://192.168.150.134:11434 failed during streaming. Error: error decoding response body
⚠️  Connection closed with Unreliable Server http://192.168.150.134:11434
☠️  Received CTRL+C, shutting down gracefully...

C:\Users\user\Downloads>
```

### A server fails twice, then succeeds twice
```sh
C:\Users\user\Downloads>ollama_load_balancer.exe --version
ollama_load_balancer 1.0.1

C:\Users\user\Downloads>ollama_load_balancer.exe --server http://192.168.150.134:11434

📒 Ollama servers list:
1. http://192.168.150.134:11434

⚙️  Timeout setting: Will abandon Ollama server after 30 seconds of silence

👂 Ollama Load Balancer listening on http://0.0.0.0:11434

🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64355
⛔😱 Server http://192.168.150.134:11434 didn't respond, now marked Unreliable. Error: error sending request for url (http://192.168.150.134:11434/api/chat)
⚠️  Connection closed with Unreliable Server http://192.168.150.134:11434
🤖😇 Giving server http://192.168.150.134:11434 another chance with client 127.0.0.1:64358
⛔😞 Unreliable server http://192.168.150.134:11434 didn't respond. Error: error sending request for url (http://192.168.150.134:11434/api/chat)
⚠️  Connection closed with Unreliable Server http://192.168.150.134:11434
🤖😇 Giving server http://192.168.150.134:11434 a 3rd+ chance with client 127.0.0.1:64365
🙏⚕️  Server http://192.168.150.134:11434 has completed streaming successfully and is now marked Reliable
🟢 Server http://192.168.150.134:11434 now available
🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64378
🟢 Server http://192.168.150.134:11434 now available
☠️  Received CTRL+C, shutting down gracefully...

C:\Users\user\Downloads>
```

### Generation canceled by continue.dev- not an error
```sh
C:\Users\user\Downloads>ollama_load_balancer.exe --version
ollama_load_balancer 1.0.1

C:\Users\user\Downloads>ollama_load_balancer.exe --server http://192.168.150.134:11434

📒 Ollama servers list:
1. http://192.168.150.134:11434

⚙️  Timeout setting: Will abandon Ollama server after 30 seconds of silence

👂 Ollama Load Balancer listening on http://0.0.0.0:11434

🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64417
🟢 Server http://192.168.150.134:11434 now available
☠️  Received CTRL+C, shutting down gracefully...

C:\Users\user\Downloads>
```

### Generation canceled by continue.dev while using unreliable server- streaming did not complete successfully so not enough to mark Reliable, but not to report error either
```sh
C:\Users\user\Downloads>ollama_load_balancer.exe --version
ollama_load_balancer 1.0.1

C:\Users\user\Downloads>ollama_load_balancer.exe --server http://192.168.150.134:11434

📒 Ollama servers list:
1. http://192.168.150.134:11434

⚙️  Timeout setting: Will abandon Ollama server after 30 seconds of silence

👂 Ollama Load Balancer listening on http://0.0.0.0:11434

🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64428
⛔😱 Server http://192.168.150.134:11434 didn't respond, now marked Unreliable. Error: error sending request for url (http://192.168.150.134:11434/api/chat)
⚠️  Connection closed with Unreliable Server http://192.168.150.134:11434
🤖😇 Giving server http://192.168.150.134:11434 another chance with client 127.0.0.1:64431
⚠️  Connection closed with Unreliable Server http://192.168.150.134:11434
☠️  Received CTRL+C, shutting down gracefully...

C:\Users\user\Downloads>
```

### CTRL+C waits for as long as needed until generation is done in all open connections
```sh
C:\Users\user\Downloads>ollama_load_balancer.exe --version
ollama_load_balancer 1.0.1

C:\Users\user\Downloads>ollama_load_balancer.exe --server http://192.168.150.134:11434

📒 Ollama servers list:
1. http://192.168.150.134:11434

⚙️  Timeout setting: Will abandon Ollama server after 30 seconds of silence

👂 Ollama Load Balancer listening on http://0.0.0.0:11434

🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64441
☠️  Received CTRL+C, shutting down gracefully...
🟢 Server http://192.168.150.134:11434 now available

C:\Users\user\Downloads>
```

### First server was off, second server was on. First server marked as unreliable, so for every next generation, the second server is chosen again and again
```sh
C:\Users\user\Downloads>ollama_load_balancer.exe --version
ollama_load_balancer 1.0.1

C:\Users\user\Downloads>ollama_load_balancer.exe --server http://192.168.150.134:11434 --server http://192.168.150.135:11434

📒 Ollama servers list:
1. http://192.168.150.134:11434
2. http://192.168.150.135:11434

⚙️  Timeout setting: Will abandon Ollama server after 30 seconds of silence

👂 Ollama Load Balancer listening on http://0.0.0.0:11434

🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64459
⛔😱 Server http://192.168.150.134:11434 didn't respond, now marked Unreliable. Error: error sending request for url (http://192.168.150.134:11434/api/chat)
⚠️  Connection closed with Unreliable Server http://192.168.150.134:11434
🤖🦸 Chose reliable server: http://192.168.150.135:11434 to serve client 127.0.0.1:64462
🟢 Server http://192.168.150.135:11434 now available
🤖🦸 Chose reliable server: http://192.168.150.135:11434 to serve client 127.0.0.1:64474
🟢 Server http://192.168.150.135:11434 now available
🤖🦸 Chose reliable server: http://192.168.150.135:11434 to serve client 127.0.0.1:64477
🟢 Server http://192.168.150.135:11434 now available
☠️  Received CTRL+C, shutting down gracefully...

C:\Users\user\Downloads>
```

### Nothing ever goes wrong, all requests are single file- so first server is always chosen
```sh
C:\Users\user\Downloads>ollama_load_balancer.exe --version
ollama_load_balancer 1.0.1

C:\Users\user\Downloads>ollama_load_balancer.exe --server http://192.168.150.134:11434 --server http://192.168.150.135:11434

📒 Ollama servers list:
1. http://192.168.150.134:11434
2. http://192.168.150.135:11434

⚙️  Timeout setting: Will abandon Ollama server after 30 seconds of silence

👂 Ollama Load Balancer listening on http://0.0.0.0:11434

🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64510
🟢 Server http://192.168.150.134:11434 now available
🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64513
🟢 Server http://192.168.150.134:11434 now available
🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64516
🟢 Server http://192.168.150.134:11434 now available
🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64518
🟢 Server http://192.168.150.134:11434 now available
☠️  Received CTRL+C, shutting down gracefully...

C:\Users\user\Downloads>
```

### Nonsensical server IP + port doesn't crash utility
```sh
C:\Users\user\Downloads>ollama_load_balancer.exe --version
ollama_load_balancer 1.0.1

C:\Users\user\Downloads>ollama_load_balancer.exe --server aksldjflakje

📒 Ollama servers list:
1. aksldjflakje

⚙️  Timeout setting: Will abandon Ollama server after 30 seconds of silence

👂 Ollama Load Balancer listening on http://0.0.0.0:11434

🤖🦸 Chose reliable server: aksldjflakje to serve client 127.0.0.1:64530
⛔😱 Server aksldjflakje didn't respond, now marked Unreliable. Error: builder error
⚠️  Connection closed with Unreliable Server aksldjflakje
☠️  Received CTRL+C, shutting down gracefully...

C:\Users\user\Downloads>
```

### Set timeout to infinity and pause VM running Ollama server during generation- we will be stuck forever now (or until somebody unpauses the VM)

```sh
C:\Users\user\Downloads>ollama_load_balancer.exe --version
ollama_load_balancer 1.0.1

C:\Users\user\Downloads>ollama_load_balancer.exe --server http://192.168.150.134:11434 --timeout 0

📒 Ollama servers list:
1. http://192.168.150.134:11434

⚙️  Timeout setting: Will abandon Ollama server after 0 seconds of silence

👂 Ollama Load Balancer listening on http://0.0.0.0:11434

🤖🦸 Chose reliable server: http://192.168.150.134:11434 to serve client 127.0.0.1:64565
☠️  Received CTRL+C, shutting down gracefully...

```

## Future Plans

### Model-Aware Load Balancing & KV Cache Optimization

**What we have today:** The load balancer is a **path-agnostic HTTP proxy**. It copies the request path (e.g., `/api/chat`, `/api/tags`, `/api/ps`) and forwards the entire request to whichever server passes the availability check (`Reliable` + not `busy`). It has zero knowledge of what models exist on which servers, what's loaded in memory, or hardware capabilities. When a client queries `GET /api/tags` (list available models), the load balancer forwards it to essentially a random available server—the response only reflects that single server's model inventory. When a client requests inference with model X, the load balancer blindly routes to an available server that may not even have model X installed, resulting in connection failure and retry overhead.

**What we can achieve:** Transform the load balancer into a **model-aware orchestrator** that polls Ollama's APIs and [llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows) control APIs to build comprehensive cluster state, then uses that state for intelligent routing and **direct response generation**:

**Model Discovery & Aggregation (`GET /api/tags` on :11434):**
Poll each server's `/api/tags` endpoint (returns installed models: name, size, digest) to build a cluster-wide model inventory. When a client queries `GET /api/tags`, the load balancer **responds directly** with the union of all models across all servers (or intersection—models available on at least one `Reliable` server), eliminating the random-server limitation. For inference requests (`/api/chat`, `/api/generate`), parse the `model` field from request JSON and route **only to servers that have that model installed**. Fail fast with meaningful errors ("model X not available on any server") instead of trial-and-error across incompatible servers. API present in Ollama v0.13.4+.

**Hot Model Preference (`GET /api/ps` on :11434):**
Poll each server's `/api/ps` endpoint (returns currently loaded models with VRAM usage, keep-alive status) to track which models are hot in memory. Among servers with the requested model, prefer servers where it's already loaded to eliminate cold-start latency (can be 5-60 seconds depending on model size and disk speed). Balance load intelligently when multiple models are in concurrent use—e.g., if 3 servers have both model A and B loaded, but 5 clients are using model A and 1 is using model B, route new model B requests to the least-busy model-B server. **Note:** [llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows) uses `OLLAMA_KEEP_ALIVE=-1`, so models remain loaded permanently after first use (no cold starts after initial warmup). API present in Ollama v0.13.4+.

**Heterogeneous VRAM Tier Support:**
Current limitation: The load balancer assumes homogeneous servers—all have identical models with identical configurations. This breaks down with mixed VRAM capacities. Example: Server A (24GB VRAM) runs `qwen3-32b` with q8_0 KV cache at 11k context. Server B (32GB VRAM) runs the same model with q16 KV cache at full 32k context. Users typically work around heterogeneous hardware by creating distinct model names via Ollama Modelfile (Server A: `qwen3-32b`, Server B: `qwen3-32b-long`), analogous to configuring `"think": "high"` or other parameters. OpenWebUI shows both as separate choices. **This bypass doesn't work with our load balancer today**—because it's path-agnostic and model-unaware, it routes `qwen3-32b-long` requests to any available server including Server A, which fails with OOM. We want to make this user-side bypass work because it's a good solution users already have at their disposal.

**KV Cache Optimization ([llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows)):**
Load balancer CLI will accept `--kv-q8` arguments listing model names that require 8-bit KV cache quantization (e.g., `--kv-q8 qwen3-32b --kv-q8 llama3:8b`). Models not in this list default to q16. Load balancer queries `GET :11435/health` to determine each server's current KV cache type (`{"kv_cache_type":"q8_0"}` or `"q16"`), then routes each model request only to servers with matching KV cache configuration—requests for `qwen3-32b` only go to q8_0 servers, requests for `qwen3-32b-long` only go to q16 servers. This enables the user-side bypass to work: lower-VRAM servers run models with q8_0 (50% VRAM of q16, faster, minimal quality loss), higher-VRAM servers run the same model with q16 (full precision, 2× VRAM, slower, maximum accuracy) under different names. Optionally support dynamic reconfiguration via `POST :11435/set-kv-cache` with `{"type":"q8_0"}` or `{"type":"q16"}` (returns 202, restart takes 5-15s).

**Why runtime failure tracking remains critical:** Health check APIs (e.g., [llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows) `GET :11435/health`) exist but are **not useful** for reliability assessment. A server can be network-reachable, report `{"status":"healthy"}`, and still fail every inference request due to insufficient VRAM (model loaded but KV cache allocation fails), corrupted model files (checksum passes but inference crashes), driver bugs (CUDA/ROCm errors mid-stream), or thermal throttling (starts OK, fails after 30 seconds). **The authoritative test is the result of actual inference.** The existing `Reliable`/`Unreliable`/`SecondChanceGiven` failure tracking logic (src/main.rs:244-254, 382-409) must remain the primary mechanism for server tier ranking. Model-aware routing operates **within the pool of `Reliable` servers**—it decides which `Reliable` server to route to based on model availability and load distribution, but runtime failures still demote servers to `Unreliable` regardless of what APIs report.

**API Availability:** `/api/tags` and `/api/ps` present in Ollama v0.13.4+. OpenAI-compatible `GET /v1/models` added via [PR #5209](https://github.com/ollama/ollama/pull/5209) (merged July 2, 2024) and may also be leveraged for aggregation. [llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows) control APIs (`GET :11435/health`, `POST :11435/set-kv-cache`) are custom to that deployment method. Exact version history for native Ollama APIs requires checking git history (no per-endpoint version tables published).

**Implementation:** Background polling threads per server (30-60s intervals for `/api/tags` and `/api/ps`). Maintain in-memory cluster state: installed models per server, loaded models per server, KV cache type per server. Intercept `/api/tags` and `/v1/models` requests to respond directly with aggregated data instead of proxying. For inference requests (`/api/chat`, `/api/generate`, `/v1/chat/completions`), parse model name from request body and filter candidate servers before applying existing availability+reliability selection logic. Continue using runtime failure tracking as authoritative reliability signal.

## Research

I set up an Ollama server running on my local network.

I then set up Continue.dev to access that Ollama server.

`continue.dev` has a chat like ChatGPT.

I recorded that there is no network traffic between my PC running VS Code and the Ollama server, until I press ENTER in the chat in VS Code- to start streaming a response.

In wireshark I saw the request structure.

First the TCP connection is created: [SYN] to 192.168.150.134:11434, then `[SYN, ACK]` to back to the TCP client at: 192.168.150.1 on a random port (the same port as the origin of the original [SYN]).

Then there's an `[ACK]` back to 192.168.150.134. With that, the TCP connection is established.

The very next thing is an HTTP/1.1 POST request 192.168.150.1 -> 192.168.150.134 at endpoint "/api/chat".

TCP payload:
```txt
POST /api/chat HTTP/1.1
accept: */*
accept-encoding: gzip, deflate, br
authorization: Bearer undefined
content-length: 167
content-type: application/json
user-agent: node-fetch
Host: 192.168.150.134:11434
Connection: close

{"model":"deepseek-coder:1.3b-instruct-q4_0","raw":true,"keep_alive":1800,"options":{"num_predict":2048,"num_ctx":4096},"messages":[{"role":"user","content":"Hello"}]}
```

Essentially, that tells the Ollama server to load the model if needed, and to have the model start working with those settings, and that prompt. In this case "Hello" is indeed the prompt in the chat in the VS Code window.

Then there's a stream of the LLM response, which altogether produces this full text:
```txt
Hi! How can I assist you today? Please provide more details about your question or issue regarding programming languages with the AI assistant if it's related to computer science topics rather than general knowledge issues like hello world programmers etc, so we get a better understanding. (Sorry for any confusion in previous responses) If not specifically asked yet and I am unable to provide an answer as per my current capabilities based on what is provided currently - AI model by Deepseek! Please let me know if there's anything else you need help with over here, whether it be a programming language problem or something completely different.
```

Now let's talk about the resopnse:
It starts with a TCP `[PSH, ACK]` packet 192.168.150.134 -> 192.168.150.1 that contains this 294 bytes TCP payload:
```txt
HTTP/1.1 200 OK
Content-Type: application/x-ndjson
Date: Sat, 19 Oct 2024 19:39:14 GMT
Connection: close
Transfer-Encoding: chunked

95
{"model":"deepseek-coder:1.3b-instruct-q4_0","created_at":"2024-10-19T19:39:14.1898363Z","message":{"role":"assistant","content":"Hi"},"done":false}


```

That TCP packet is the beginning of the response, but there's no HTTP response terminator yet.

Notice that the text I just quoted is the pure payload when copied as printable text. This "HTTP/1.1 200 OK ..." is plain text inside of the TCP payload.

Then there are ~100 packets of that same type `[PSH, ACK]`.
Each `[PSH, ACK]` has an `[ACK]` from 192.168.150.1

Notice the ending double newlines. Each `[PSH, ACK]` ends with a double carriage return. More specifically, these four binary bytes: "\r\n\r\n"

I will paste some of their payloads in order:

Payload: 154 bytes
```txt
94
{"model":"deepseek-coder:1.3b-instruct-q4_0","created_at":"2024-10-19T19:39:14.2585923Z","message":{"role":"assistant","content":"!"},"done":false}


```

Payload: 157 bytes
```txt
97
{"model":"deepseek-coder:1.3b-instruct-q4_0","created_at":"2024-10-19T19:39:14.3346855Z","message":{"role":"assistant","content":" How"},"done":false}


```

Payload: 156 bytes
```txt
97
{"model":"deepseek-coder:1.3b-instruct-q4_0","created_at":"2024-10-19T19:39:14.4049587Z","message":{"role":"assistant","content":" can"},"done":false}


```

Payload: 154 bytes
```txt
94
{"model":"deepseek-coder:1.3b-instruct-q4_0","created_at":"2024-10-19T19:39:14.455463Z","message":{"role":"assistant","content":" I"},"done":false}


```

Then it continues like that for every single word of the response...
and as we approach the end:

Payload: 163 bytes
```txt
9d
{"model":"deepseek-coder:1.3b-instruct-q4_0","created_at":"2024-10-19T19:39:22.9287849Z","message":{"role":"assistant","content":" different"},"done":false}


```

Payload: 154 bytes
```txt
94
{"model":"deepseek-coder:1.3b-instruct-q4_0","created_at":"2024-10-19T19:39:23.0041127Z","message":{"role":"assistant","content":"."},"done":false}


```

Payload: 155 bytes
```txt
95
{"model":"deepseek-coder:1.3b-instruct-q4_0","created_at":"2024-10-19T19:39:23.0705385Z","message":{"role":"assistant","content":"\n"},"done":false}


```

And then, the very last packet before the zero terminator is another `[PSH, ACK]` packet, this time "done" is finally true in the application-specific data format sent here.

The content:
Payload: 326 bytes
```txt
13f
{"model":"deepseek-coder:1.3b-instruct-q4_0","created_at":"2024-10-19T19:39:23.1468105Z","message":{"role":"assistant","content":""},"done_reason":"stop","done":true,"total_duration":9033032700,"load_duration":13675700,"prompt_eval_count":70,"prompt_eval_duration":69277000,"eval_count":127,"eval_duration":8945400000}


```
Notice that done_reason is "stop" meaning, the LLM said enough, and decided to stop.

Then there's a single TCP packet:
Payload: 5 bytes
```txt
0


```
which marks the end of the HTTP response.
Notice that even the zero terminator then ends with "\r\n\r\n", as the HTTP protocol dictates.

Then after the end of the response there are more TCP packets:

1. A TCP `[ACK]` from the VS Code to the packet that marks the end of the HTTP response.

2. `[FIN, ACK]` initiated by the Ollama server

3. `[ACK]` as a response to `[FIN, ACK]`

4. `[FIN, ACK]` initiated again(?) by the VS Code

5. `[ACK]` as a response to `[FIN, ACK]`

With that, the TCP connection is done.

All of this network analysis was the result of of a single ENTER click in that chat window in `continue.dev` as it communicates with Ollama server running on the local network.
