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

Download the [latest release](#release-notes) executable, or build from source:

```sh
cargo build --release
```

## Alternative Builds

### Nix (MacOS, Linux)

```
nix --extra-experimental-features "nix-command flakes" profile install "github:BigBIueWhale/ollama_load_balancer"
```

### Docker

```
docker build -t bigbluewhale/ollama_load_balancer .
docker run -it --rm -p 11434:11434 bigbluewhale/ollama_load_balancer --server="http://192.168.150.134:11434=james" --server="http://192.168.150.135:11434=sara"
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

**Important for Mistral Vibe CLI users:** If experiencing 10-30 second delays after every tool call, upgrade Ollama to v0.14.0+ — see [KV Cache Bug Investigation](https://github.com/BigBIueWhale/vibe_web_terminal/tree/master/doc/ollama_kv_cache_bug_investigation.md) for details.

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

> **Note:** The VM-based setup described below is **only required to simulate production network topology** where multiple physical servers run Ollama on the same port (11434) with different IP addresses.
>
> This complexity exists because an OS typically restricts each subnet to a single network interface card. Virtual machines bypass this limitation by providing isolated network stacks with unique IP addresses.
>
> **Looking for automated testing?** See the [Testing](#testing) section for an Ollama simulator approach that doesn't require running VMs.

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

### Vision for Version 1.0.4

**The coming infrastructure reality:** Setting `OLLAMA_KEEP_ALIVE=-1` in [llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows/blob/master/on_startup.ps1) dramatically improved Chat response speed by keeping models loaded in VRAM permanently. The availability of `gpt-oss:20b` (https://ollama.com/library/gpt-oss:20b, released August 5, 2025) offers 128k context capability—enabling queries over entire codebases. Looking ahead, we face a heterogeneous GPU landscape: new hardware with greater VRAM arriving alongside existing 24GB VRAM servers that still need to contribute. Use cases like agentic models for Claude Code will be possible but only on capable hardware. **The current load balancer infrastructure—treating all servers as identical and routing model-agnostically—is not prepared for this future.**

**What's driving version 1.0.4:**
- **Heterogeneous hardware:** Mix of 24GB VRAM servers (current deployment) and future higher-capacity GPUs
- **Model diversity:** Small efficient models (gpt-oss:20b) coexist with large context-heavy models (qwen3-32b variants)
- **Quality vs. capacity tradeoffs:** Some models tolerate KV cache quantization (qwen3-32b with q8_0 for 11k context), others demand full precision (gpt-oss:20b, gemma3 require q16 for quality)
- **Server capability tiers:** Fast servers, high-VRAM servers, and legacy servers all contributing to shared load balancing pool
- **Dynamic configuration needs:** Ability to adjust KV cache settings per-model without manual server restarts
- **Agentic workflow optimization:** Tools like [Claude Code](https://github.com/anthropics/claude-code) and [Mistral Vibe CLI](https://github.com/BigBIueWhale/mistral_vibe_setup) make rapid-fire sequential API calls within a single conversation—each call benefits enormously from KV cache locality when routed to the same server that processed previous requests in that session. Users of [openwebui_config](https://github.com/BigBIueWhale/openwebui_config) also benefit from quicker follow-up responses when their conversation is routed back to the server that has their context cached

![heterogenous concept](./doc/illustrations/heterogeneous_concept.png)

### Deployment Roadmap for 1.0.4

**Infrastructure updates required across all servers:**

1. **Update Ollama to latest version** on all AI servers. Ollama's initial implementation of `gpt-oss` (released August 5, 2025) had poor performance. The fixed implementation and updated model files (https://ollama.com/library/gpt-oss:20b) were released approximately two months later. All servers must run the corrected version.

2. **Update [llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows) to latest version** (December 29, 2025+). This version adds dynamic control over Ollama's global `KV_CACHE_TYPE` configuration via `GET :11435/health` (query current state) and `POST :11435/set-kv-cache` (change setting). Implementation involves process restart because Ollama does not support runtime reconfiguration of this parameter. All servers in the network must be updated.

3. **Deploy new models** during server updates (changes require Ollama restart, so batch with llm_server_windows update):
   - **Add:** `gpt-oss:20b` with `"think": "high"` mode (configured via Ollama Modelfile in `.ollama` directory, not runtime-changeable)
   - **Add:** `qwen3-30b-a3b-thinking-2507` (updated Qwen3 variant with thinking capability)
   - **Add:** `qwen3-30b-a3b-instruct-2507` (updated Qwen3 instruct variant)
   - **Remove:** Old `qwen3-30b-a3b` models (superseded by 2507 versions)

4. **Load balancer version 1.0.4 release:** Implement model-aware routing, capability tier system, KV cache coordination, and hot-model preference (detailed below).

### Model-Aware Load Balancing & KV Cache Optimization

**What we have today (v1.0.3):** The load balancer is a **path-agnostic HTTP proxy**. It copies the request path (e.g., `/api/chat`, `/api/tags`, `/api/ps`) and forwards the entire request to whichever server passes the availability check (`Reliable` + not `busy`). It has zero knowledge of what models exist on which servers, what's loaded in memory, or hardware capabilities. When a client queries `GET /api/tags` (list available models), the load balancer forwards it to essentially a random available server—the response only reflects that single server's model inventory. When a client requests inference with model X, the load balancer blindly routes to an available server that may not even have model X installed, resulting in connection failure and retry overhead.

**What we will achieve in v1.0.4:** Transform the load balancer into a **model-aware orchestrator** that polls Ollama's APIs and [llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows) control APIs to build comprehensive cluster state, then uses that state for intelligent routing and **direct response generation**:

#### Model Discovery & Aggregation (`GET /api/tags` on :11434)
Poll each server's `/api/tags` endpoint (returns installed models: name, size, digest) to build a cluster-wide model inventory. When a client queries `GET /api/tags`, the load balancer **responds directly** with the union of all models across all servers (or intersection—models available on at least one `Reliable` server), eliminating the random-server limitation. For inference requests (`/api/chat`, `/api/generate`), parse the `model` field from request JSON and route **only to servers that have that model installed**. Fail fast with meaningful errors ("model X not available on any server") instead of trial-and-error across incompatible servers.

#### Hot Model Preference (`GET /api/ps` on :11434)
Poll each server's `/api/ps` endpoint (returns currently loaded models with VRAM usage, keep-alive status) to track which models are hot in memory. Within the server selection hierarchy (step 6), prefer servers where the requested model is already loaded to eliminate cold-start latency (5-60 seconds depending on model size and disk speed). This preference applies after filtering by availability, model availability, reliability, capability tier, and KV cache compatibility.

**Immediate response capability:** When a model is already loaded in VRAM, inference begins immediately—users see the first token within milliseconds. This is critical for user experience. The load balancer should prefer servers with the requested model already hot, but only after applying capability tier and KV cache compatibility filters—hot model preference serves as a tiebreaker among servers that have already passed these earlier selection stages (see Server Selection Hierarchy below for the complete 9-step sequence).

**Note:** [llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows) uses `OLLAMA_KEEP_ALIVE=-1`, so models remain loaded permanently after first use (no cold starts after initial warmup).

#### Heterogeneous VRAM & Server Capability Tiers
Current limitation: The load balancer assumes homogeneous servers—all have identical models with identical configurations. This breaks down with mixed VRAM capacities and performance profiles.

**Examples of heterogeneity:**
- **VRAM capacity:** Server A (24GB VRAM) runs `qwen3-32b` with q8_0 KV cache at 11k context. Server B (32GB VRAM) runs the same model with q16 KV cache at full 32k context.
- **Processing speed:** Server C has newer GPU generation, faster inference despite same VRAM.
- **Model availability:** Server D has `gpt-oss:20b` + `qwen3-32b-think`, Server E only has `qwen3-32b-think`.

Users typically work around heterogeneous hardware by creating distinct model names via Ollama Modelfile (Server A: `qwen3-32b`, Server B: `qwen3-32b-32k`), analogous to configuring `"think": "high"` or context length. OpenWebUI shows both as separate choices. **This bypass doesn't work with our load balancer today**—because it's path-agnostic and model-unaware, it routes `qwen3-32b-32k` requests to any available server including Server A, which fails with OOM. Version 1.0.4 makes this user-side bypass work by routing each model name only to servers that advertise it.

**Capability tier system:** Load balancer CLI will accept per-server capability and speed annotations embedded in the server string (e.g., `--server "http://192.168.1.10:11434=Server-A[capability=10,speed=100]"`). Capability values range from 0-100 (inclusive) where 0 represents lowest capability and 100 represents highest capability. Speed values range from 0-100 (inclusive) where 0 represents baseline/normal speed and 100 represents maximum speed. Both parameters are optional; if not specified, they default to 0. Server selection prefers **lower-capability servers** for tasks they can handle (preserving high-capability servers for demanding workloads), with additional tiebreakers (KV cache compatibility, hot model preference, conversation affinity, speed) applied sequentially as described in the complete 9-step Server Selection Hierarchy below.

#### KV Cache Optimization ([llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows))

**Why KV cache matters:** The only reason we enabled q8_0 quantization on current 24GB VRAM servers is to achieve 11k context for `qwen3-32b` (q16 would OOM). However, this degrades quality for models that don't need quantization:
- **Sensitive models:** `gemma3` is known to be extremely sensitive to KV cache quality—q8_0 significantly hurts performance even though it fits in VRAM
- **Small models:** `gpt-oss:20b` is small enough to run with q16 even on 24GB VRAM, no reason to degrade it
- **Unnecessarily degraded inference:** Applying q8_0 globally punishes models that could run at full quality

**Solution:** Load balancer CLI will accept `--kv-q8` arguments listing model names that require 8-bit KV cache quantization (e.g., `--kv-q8 qwen3-32b --kv-q8 llama3:8b`). Models not in this list default to q16.

Load balancer queries `GET :11435/health` to determine each server's current KV cache type (`{"kv_cache_type":"q8_0"}` or `"q16"`), then applies KV cache filtering (step 5 in the selection hierarchy) to route each model request only to servers with matching KV cache configuration:
- Requests for `qwen3-32b` (requires q8_0 per CLI config) → prefer q8_0 servers
- Requests for `gpt-oss:20b` (not in q8 list, defaults to q16) → prefer q16 servers
- Requests for `qwen3-32b-32k` (user-created Modelfile variant, not in q8 list) → prefer q16 servers (implies high-VRAM server)

This enables the user-side bypass to work: lower-VRAM servers run models with q8_0 (50% VRAM of q16, faster, minimal quality loss for tolerant models), higher-VRAM servers run the same or extended model with q16 (full precision, 2× VRAM used by KV-cache, slower, full context accuracy) under different names.

**Dynamic reconfiguration:** Use `POST :11435/set-kv-cache` with `{"type":"q8_0"}` or `{"type":"q16"}` (returns 202, restart takes 5-15s). This allows the load balancer to **automatically switch a server's KV cache mode** if the wrong-type servers from the base tier that can run the selected model are **all** of the wrong `KV_CACHE_TYPE` to run that model. The only reason we needed to implement this KV_CACHE switcheroo is because Ollama doesn't support switching KV cache quantization method at runtime. So our `llm_server_windows` project kills Ollama and runs it again with the correct parameters. Our load balancer will tell `llm_server_windows` to trigger this Ollama killing.

**Backwards compatibility:** If a server doesn't respond correctly to `GET :11435/health` endpoint, print a warning but still allow it while assuming that server is running at 8-bit KV cache quantization. Not all endpoints will have the newest version of [llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows) immediately.

#### Server Selection Hierarchy (v1.0.4)

**All current functionality (v1.0.3) must remain:** The existing `Reliable`/`Unreliable`/`SecondChanceGiven` failure tracking logic is **critical and unchanged**. Runtime failures remain the authoritative signal for server health—a server reporting healthy via API but failing inference is still demoted to `Unreliable`. All described edge cases need to preserve their current behaviour which is proven and has been battle-tested.

**New multi-stage selection logic for v1.0.4:**

When a client requests inference with model X, the load balancer selects a server using this **sequential hierarchy** (each stage filters the candidate pool for the next stage). Note that this entire choice has to occur by looking at pre-prepared structures in memory, because we're only allowed to stay within this cricitical section quite briefly to make our choice:

1. **Availability filter:** Only consider servers that are not `busy` (assumption: only this load balancer accesses Ollama servers, so we authoritatively track busy state)

2. **Model availability filter:** From available servers, only consider servers that have model X installed (per `/api/tags` polling). If no server has model X, that means that no server is available to serve request.

3. **Reliability filter:** From available servers with model X, prefer `Reliable` servers. Only fall back to `Unreliable` servers if no `Reliable` server is available (existing v1.0.3 behavior).

4. **Capability tier preference:** From the remaining candidates (reliable servers with model X from step 3), prefer the **lowest-capability tier**. For example, if model X is available on servers with capability 10 and capability 80, route to the capability 10 tier (preserving capability 80 tier for models that require it). Capability values are embedded in the server string (e.g., `--server "http://192.168.1.10:11434=Server-A[capability=10]"`, `--server "http://192.168.1.11:11434=Server-B[capability=80]"`). Servers without explicit capability annotation default to capability 0 (lowest). This capability tier preference only affects choice of which server will be chosen to serve a model, out of the possible servers that actually support that model—`/api/tags` model availability filtering (step 2) remains authoritative, but this addresses the case where a high-capability server also supports lower-tier models.

5. **KV cache compatibility filter:** From the lowest-capability tier candidates (from step 4), prefer servers whose KV cache type (per `GET :11435/health`) currently matches the model's requirement (q8_0 if model is in `--kv-q8` list, q16 otherwise). If model X requires q8 but only q16 servers have it, optionally trigger dynamic reconfiguration—this **must** be designed to simulate the process of an Ollama server "ingesting the prompt": the user's request must connect (simulated while the server is restarting), then the moment the server is up, flood it with the kept request and stream the result. This logic requires careful design for non-blocking operation while respecting timeout constraints and quickly detecting when the server completes restart with the requested KV cache type. Must also handle edge cases: Ollama failing to restart (power disconnect), or failing to load the model (VRAM captured by another process). This dynamic reconfiguration path represents complex, high-risk logic requiring thorough edge case handling. To summarize: in this step we filter-out non-KV-Cache compatible servers, so only in the case that there are no kv-cache compatible servers, we counterintuitively skip steps 6 and 7, and choose a server to convert its `KV_CACHE_TYPE` based on steps 8 and 9.

6. **Hot model preference:** From KV-cache-compatible candidates (from step 5), prefer servers where model X is already loaded in VRAM (per `/api/ps` polling). This eliminates cold-start latency (5-60 seconds). If multiple servers have model X hot, proceed to next tiebreaker. If no servers have model X hot, choose any server from the remaining candidates (will incur cold-start). Note: This preference only applies to servers that already have the correct `KV_CACHE_TYPE`. This step is not applied when we're in the path of choosing a server to have its `KV_CACHE_TYPE` converted.

7. **Conversation affinity (KV cache locality):** From candidates with hot model (from step 6), prefer the server whose cached conversation is a **prefix** of the incoming request.

   **Why not use client IP?** Source IP is unreliable for identifying conversations:
   - **OpenWebUI/proxy scenarios:** All users' requests arrive from the same IP (the proxy server)
   - **Multi-agent workflows:** When Claude Code spawns search agents, each agent deserves its own cache affinity despite sharing source IP
   - **NAT/corporate networks:** Multiple users share a single public IP

   **How KV cache works internally:** Ollama's KV cache stores token embeddings. Cache reuse requires the first N *token IDs* to be identical. The load balancer has no access to model tokenizers, so it cannot compute token-level overlap directly.

   **Message-level approximation:** Since template rendering and tokenization are deterministic (same model + same messages → same tokens), we compare at the **message level** instead. If `cached_messages` is a strict prefix of `new_messages`, the token prefix will match. This works because agentic clients send back the exact conversation history—the assistant's response in the new request is verbatim what the server generated.

   **How it works:** After each successful completion, store the full conversation (input messages + generated response). On new request, check if any server's cached messages array is a **strict prefix** of the incoming messages array. Compare messages field-by-field: `role`, `content`, `images`, `tool_calls`, `thinking`, `tool_call_id`. All fields must match exactly.

   **Match requirements:** For a prefix match to count:
   - `model` must match exactly
   - `tools` array must match (tool definitions are rendered into the prompt for some models)
   - `options.num_ctx` must match (affects context window/truncation)
   - Cached messages must be a strict prefix of new messages (element-by-element equality)

   **Threshold:** Only consider it a meaningful cache hit if the cached prefix covers ≥40% of the new request's messages AND ≥3 messages. Below this threshold, treat servers equally.

   This optimization is critical for agentic workflows ([Claude Code](https://github.com/anthropics/claude-code), [Mistral Vibe CLI](https://github.com/BigBIueWhale/mistral_vibe_setup)) where sequential API calls benefit enormously from KV cache reuse. It also correctly handles OpenWebUI where different users' conversations are distinguishable by content despite sharing the same source IP.

8. **Speed tiebreaker:** From remaining candidates (from step 7—servers in the same capability tier with model X hot, KV cache compatible, and conversation affinity applied), prefer the **fastest server**. Speed values are embedded in the server string (e.g., `--server "http://192.168.1.10:11434=Server-A[speed=100]"`, `--server "http://192.168.1.12:11434=Server-C[speed=100]"`). Servers without explicit speed annotation default to speed 0 (baseline/normal). Higher speed values are preferred over lower speed values.

9. **Fallback (CLI order):** From any remaining tied candidates (from step 8), choose the first server in CLI argument order (deterministic).

**Example scenario:**
```
CLI args:
  --server "http://192.168.1.10:11434=ServerA[capability=10,speed=100]"
  --server "http://192.168.1.11:11434=ServerB[capability=10]"
  --server "http://192.168.1.12:11434=ServerC[capability=80,speed=100]"
  --kv-q8 qwen3-32b

Initial state:
  ServerA: cached_request = None
  ServerB: cached_request = None
  ServerC: cached_request = None

Request: model=qwen3-32b, messages=[user:"Hello", asst:"Hi!", user:"Explain KV cache"]  (3 messages)
1. Availability: ServerA (not busy), ServerB (not busy), ServerC (busy) → [ServerA, ServerB]
2. Model installed: All have qwen3-32b → [ServerA, ServerB]
3. Reliability: All are Reliable → [ServerA, ServerB]
4. Capability: Prefer lowest tier (capability 10) → [ServerA, ServerB]
5. KV cache type: All are q8_0 (compatible) → [ServerA, ServerB]
6. Hot model: Both have qwen3-32b loaded → [ServerA, ServerB]
7. Conversation affinity: No cached requests anywhere → [ServerA, ServerB]
8. Speed: ServerA speed=100, ServerB speed=0 → [ServerA]
→ ServerA selected
→ After completion: ServerA.cached_messages = [user:"Hello", asst:"Hi!", user:"Explain KV cache", asst:"...response..."]

Request: model=qwen3-32b, messages=[user:"What's the weather?"]  (1 message, different conversation)
1-6. → [ServerA, ServerB]
7. Conversation affinity: ServerA's cache is not a prefix of this request (different first message).
   No meaningful match. Both servers equal → [ServerA, ServerB]
8. Speed: ServerA speed=100 → [ServerA]
→ ServerA selected (overwrites cache, unavoidable)
→ After completion: ServerA.cached_messages = [user:"What's the weather?", asst:"...response..."]

Request: model=qwen3-32b, messages=[user:"Hello", asst:"Hi!", user:"Explain KV cache", asst:"...", user:"Give example"]  (5 messages, continuing first conversation!)
1-6. → [ServerA, ServerB]
7. Conversation affinity:
   - ServerA cache: [user:"What's the weather?", ...] — not a prefix, no match
   - ServerB cache: None
   No server has matching prefix → [ServerA, ServerB]
8. Speed: → [ServerA]
→ ServerA selected (unfortunately first conversation's cache was overwritten)

Request (OpenWebUI scenario—all from same proxy IP 10.0.0.5):
  User Alice: model=qwen3-32b, messages=[user:"Help with Python", asst:"Sure!", user:"Show loops"]  (3 messages)
1-6. → [ServerA, ServerB]
7. Conversation affinity: ServerA's cache is not a prefix → no match
→ ServerB selected (unowned)
→ After completion: ServerB.cached_messages = [user:"Help with Python", asst:"Sure!", user:"Show loops", asst:"..."]

  User Bob: model=qwen3-32b, messages=[user:"Explain Docker"]  (1 message)
1-6. → [ServerA, ServerB]
7. Conversation affinity: Neither cache is a prefix of Bob's new conversation → [ServerA, ServerB]
8. Speed: → [ServerA]
→ ServerA selected

  User Alice continues: messages=[user:"Help with Python", asst:"Sure!", user:"Show loops", asst:"...", user:"Now show while loops"]  (5 messages)
1-6. → [ServerA, ServerB]
7. Conversation affinity:
   - ServerB cache: [user:"Help with Python", asst:"Sure!", user:"Show loops", asst:"..."] — IS a prefix! (4/5 = 80%, ≥3 messages). MATCH!
   - ServerA cache: Bob's Docker convo — not a prefix.
   → [ServerB] (cache hit!)
→ ServerB selected (KV cache reused—only new message needs processing)

Request: model=gpt-oss:20b (any conversation)
1. Availability: ServerA (busy), ServerB (not busy), ServerC (not busy) → [ServerB, ServerC]
2. Model installed: Only ServerC has gpt-oss:20b → [ServerC]
3-9. Only one candidate throughout → ServerC selected
```

**Why runtime failure tracking remains critical:** Health check APIs (e.g., [llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows) `GET :11435/health`) exist but are **not useful** for reliability assessment. A server can be network-reachable, report `{"status":"healthy"}`, and still fail every inference request due to insufficient VRAM (model loaded but KV cache allocation fails), corrupted model files (checksum passes but inference crashes), driver bugs (CUDA/ROCm errors mid-stream), or thermal throttling (starts OK, fails after 30 seconds). **The authoritative test is the result of actual inference.** The existing `Reliable`/`Unreliable`/`SecondChanceGiven` failure tracking logic (src/main.rs:244-254, 382-409) must remain the primary mechanism for server tier ranking. Model-aware routing operates **within the pool of `Reliable` servers**—it decides which `Reliable` server to route to based on model availability and load distribution, but runtime failures still demote servers to `Unreliable` regardless of what APIs report.

#### API Availability
- `/api/tags` and `/api/ps` present in Ollama since at least beginning of 2024.
- OpenAI-compatible `GET /v1/models` added via [PR #5209](https://github.com/ollama/ollama/pull/5209) (merged July 2, 2024) and may also be leveraged for aggregation
- **Anthropic-compatible `POST /v1/messages`** added in Ollama ~0.15.x (July 2025)—must be treated as inference endpoint for model-aware routing
- **OpenAI Responses API `POST /v1/responses`** added in Ollama ~0.15.x (June 2025)—must be treated as inference endpoint
- **Image generation endpoints** `POST /v1/images/generations` and `POST /v1/images/edits` added experimentally—may need special handling for very long timeouts
- [llm_server_windows](https://github.com/BigBIueWhale/llm_server_windows) control APIs (`GET :11435/health`, `POST :11435/set-kv-cache`) are custom to that deployment method (December 29, 2025+)

#### Implementation Notes
- **Background polling threads** per server (30-60s intervals for `/api/tags` and `/api/ps`, 10-30s for `/health`)
- **In-memory cluster state:** installed models per server, loaded models per server, KV cache type per server, capability tier per server, speed tier per server, cached conversation per server `{model, messages[], tools[], num_ctx, timestamp}`
- **Intercept `GET /api/tags` and `GET /v1/models`:** Respond directly with aggregated data (union of all models across all servers) instead of proxying to random server
- **Parse inference requests:** Extract `model` field from JSON body of inference endpoints: `/api/chat`, `/api/generate`, `/api/embed`, `/api/embeddings`, `/v1/chat/completions`, `/v1/completions`, `/v1/embeddings`, `/v1/responses`, `/v1/messages` (Anthropic), `/v1/images/generations`, `/v1/images/edits`. Text generation endpoints (`/api/chat`, `/api/generate`, `/v1/chat/completions`, `/v1/completions`, `/v1/responses`, `/v1/messages`) establish KV cache ownership. Embedding and image generation endpoints load models but don't benefit from KV cache affinity.
- **Conversation affinity tracking:** On successful inference completion, store the complete conversation for that server: input messages + generated response (captured from stream). Store: `{model, messages[], tools[], num_ctx, timestamp}`. On new request, check if any cached conversation is a strict prefix of the new messages. Compare messages field-by-field: `role`, `content`, `images`, `tool_calls`, `thinking`, `tool_call_id`—all must match exactly.
- **API format normalization:** Different endpoints use different message formats, but Ollama's middleware converts ALL formats to native `api.ChatRequest` before template rendering—so equivalent conversations produce **identical tokens** regardless of endpoint. The load balancer must normalize to a common internal format for cache comparison, enabling cross-endpoint cache hits (e.g., Claude Code via `/v1/messages` benefits from cache created via `/v1/chat/completions`). Conversions to implement: OpenAI tool call arguments (JSON string → object), Anthropic `tool_use`/`tool_result` content blocks (→ `tool_calls[]` and `role=tool` messages), Responses API `function_call`/`function_call_output` items (→ same).
- **Filter candidates before selection:** Apply the 9-step sequential hierarchy: availability → model availability → reliability → capability tier → KV cache compatibility → hot model → conversation affinity → speed → CLI order fallback
- **Continue using runtime failure tracking:** Inference failures demote servers to `Unreliable` regardless of API health status
- **CLI arguments:** New format: `--server "URL=NAME[capability=0-100,speed=0-100]"` where capability and speed are optional parameters embedded in the server string (both default to 0 if omitted). `--kv-q8 <model-name>` flag (repeatable) to specify models requiring 8-bit KV cache quantization. It's highly recommended to place the highest amount of servers at the lowest capability tier, because if you have (for example) only one server at the lowest capability tier, that's likely to cause that same poor low-tiered server to be converted back and forth between `KV_CACHE_TYPE` and models being constantly unloaded to switch models. This low tier is designed to serve the masses, to relieve the load- so take that into account.
- **Configurable bind address:** Add `--bind <IP:PORT>` CLI argument. The current hard-coded `0.0.0.0:11434` should become configurable. Default should be `127.0.0.1:11434` for security, with `0.0.0.0:11434` for network-wide access.
- **Increase default timeout to 2 minutes:** Radio silence before first token can be significant: model loading from a slow SSD can take 30-60 seconds, and prompt ingestion for 104k+ token contexts can take another 30-60 seconds. The current 30-second default causes premature abandonment. Change default from 30 to 120 seconds. The 1-second TCP `connect_timeout` remains unchanged—establishing a TCP connection should be immediate.
- **Standardized logging:** Add datetime timestamps to all log output, with configuration supporting multi-line log entries sharing a single timestamp (all output remains on stdout).
- **Documentation:** Update `--help` output and README with full selection hierarchy explanation and reasoning + incentives + vision behind all this stuff.
- **Testing with large VLM requests:** Verify the load balancer handles very slow, large image-based requests from tools like [convert_pdf_to_md_vl.py](https://github.com/BigBIueWhale/deep_intent_search/blob/master/convert_pdf_to_md_vl.py). Converting a 1214-page PDF (e.g., `ug1085-zynq-ultrascale-trm.pdf`) with `qwen3-vl:32b-thinking` generates 3166x4096 PNG images per page; combined with the model's extended thinking (~14k thinking tokens + ~800 response tokens per page at ~60 tok/sec generation), each page takes multiple minutes. Version 1.0.3 fails these requests; 1.0.4 must handle them correctly.

### Error Handling Philosophy

**Panicking is a sin.** If someone integrates this load balancer into a larger system, we must never crash their process. Errors we have a specific action plan for (e.g., upstream server unreachable → return HTTP 502) should be handled inline. **Every other error must propagate upward** via `Result` so higher-level code can decide what to do—log and continue, restart internal state, or escalate. This is the C++ `std::runtime_error` philosophy: detailed human-readable messages describing what was observed versus expected, with stack unwinding to let the caller decide.

We use **two layers of defense**:
1. **`Result<T, Box<dyn std::error::Error + Send + Sync>>`** for normal error propagation via `?`
2. **`FutureExt::catch_unwind` / `StreamExt::catch_unwind`** from the [`futures`](https://docs.rs/futures) crate as a panic catch-all for any unexpected panics that slip through (bugs, library panics, etc.)

#### Critical: Async Panic Catching

**`std::panic::catch_unwind` does NOT work for async code.** Because futures are lazy—they only execute when polled—wrapping a future's creation in `std::panic::catch_unwind` won't catch panics that occur during `.await`. You must use the async-aware versions from the `futures` crate:

- **For Futures:** [`FutureExt::catch_unwind()`](https://docs.rs/futures-util/latest/futures_util/future/trait.FutureExt.html#method.catch_unwind)
- **For Streams:** [`StreamExt::catch_unwind()`](https://docs.rs/futures-util/latest/futures_util/stream/trait.StreamExt.html#method.catch_unwind)

Both require the `UnwindSafe` trait bound, which can be satisfied using `std::panic::AssertUnwindSafe` wrapper.

#### Hyper 0.14 Panic Behavior

When a panic occurs in a hyper 0.14 `service_fn` handler:
1. Hyper (via tokio) catches the panic at the connection level
2. The panicking connection is **abruptly terminated** (TCP RST)
3. The server continues running—other connections are unaffected
4. The client receives **no HTTP response**, just a dropped connection

This is acceptable for crash isolation, but poor for user experience. By adding our own `catch_unwind`, we can return a proper **HTTP 500** response instead of dropping the connection. See the [hyper panic handling guide](https://vorner.github.io/2020/04/13/hyper-traps.html) for background.

#### Execution Contexts and Error Propagation

The current codebase has these execution contexts. For each, we show the current state and the v1.0.4 target:

**1. `main()` — Application entry point**
```rust
// CURRENT:
async fn main() -> Result<(), Box<dyn std::error::Error>> { ... }
```
This already returns `Box<dyn std::error::Error>`, so errors can propagate naturally. No changes needed—this is the top-level catch point.

**2. `shutdown_signal()` — Graceful shutdown future**
```rust
// CURRENT:
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl_c");  // PANICS!
}

// v1.0.4 TARGET:
async fn shutdown_signal() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tokio::signal::ctrl_c().await.map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
        format!("Failed to register OS signal handler for CTRL+C: {}", e).into()
    })?;
    println!("☠️  Received CTRL+C, shutting down gracefully...");
    Ok(())
}
```
The error propagates to `main()` via the graceful shutdown mechanism, which can then decide how to handle signal registration failure.

**Note:** The `with_graceful_shutdown` method in hyper 0.14 expects a future that resolves when shutdown is requested. If that future returns a `Result`, you'll need to handle it appropriately—either by using `unwrap_or_else` or by restructuring to log the error and continue with shutdown.

**3. `handle_request()` — Per-request HTTP handler**
```rust
// CURRENT:
async fn handle_request(...) -> Result<Response<Body>, Infallible> { ... }
//                                                      ^^^^^^^^^
//                              Claims "I never fail" — but panics bypass this!
```
The `Infallible` error type is a **lie**. The function contains `.unwrap()` calls that panic, and panics are not `Result` errors—they bypass the return type entirely.

```rust
// v1.0.4 TARGET:
use futures::FutureExt;  // For catch_unwind on futures
use std::panic::AssertUnwindSafe;

async fn handle_request_inner(
    req: Request<Body>,
    servers: SharedServerList,
    remote_addr: std::net::SocketAddr,
    timeout_secs: u32,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    // All .unwrap() replaced with ? — errors propagate upward
    let client = builder.build().map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
        format!("Failed to build HTTP client (TLS or DNS resolver error): {}", e).into()
    })?;

    let servers_lock = servers.lock().map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
        format!("Server list mutex poisoned (previous handler panicked): {}", e).into()
    })?;

    // ... etc
}

// Wrapper that converts errors to HTTP 500 AND catches panics:
async fn handle_request(
    req: Request<Body>,
    servers: SharedServerList,
    remote_addr: std::net::SocketAddr,
    timeout_secs: u32,
) -> Result<Response<Body>, Infallible> {
    // CRITICAL: Use FutureExt::catch_unwind for async, NOT std::panic::catch_unwind
    let future = handle_request_inner(req, servers, remote_addr, timeout_secs);
    let result = AssertUnwindSafe(future).catch_unwind().await;

    let response = match result {
        Ok(Ok(response)) => response,
        Ok(Err(e)) => {
            // Normal error — propagated via Result from handle_request_inner
            eprintln!("Request handler error: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Internal error: {}", e)))
                .unwrap_or_else(|_| Response::new(Body::from("Internal error")))
        }
        Err(panic_payload) => {
            // Panic was caught — extract message and return HTTP 500
            let msg = panic_payload
                .downcast_ref::<&str>().map(|s| s.to_string())
                .or_else(|| panic_payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "Unknown panic".to_string());
            eprintln!("PANIC in request handler: {}", msg);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal server error"))
                .unwrap_or_else(|_| Response::new(Body::from("Internal error")))
        }
    };
    Ok(response)
}
```

**Why `FutureExt::catch_unwind` works:** It wraps the future and catches any panic that occurs during any of its `poll()` calls. The panic (if any) becomes the `Err` variant of the result. See [futures-util documentation](https://docs.rs/futures-util/latest/futures_util/future/trait.FutureExt.html#method.catch_unwind).

**4. `select_available_server()` — Server selection logic**
```rust
// CURRENT:
async fn select_available_server(...) -> Option<String> {
    let mut servers_lock = servers.lock().unwrap();  // PANICS on poison!
    // ...
}

// v1.0.4 TARGET:
async fn select_available_server(
    servers: &SharedServerList,
    remote_addr: &std::net::SocketAddr,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut servers_lock = servers.lock().map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
        format!("Cannot select server: mutex poisoned while acquiring lock. \
                 A previous request handler panicked, leaving server list in unknown state. \
                 Poison error: {}", e).into()
    })?;
    // ... rest of logic unchanged, but returns Ok(Some(...)) or Ok(None)
}
```
The `Option<String>` for "no servers available" is a **handled case** (we return HTTP 503). The mutex poison is an **unhandled case** that propagates upward.

**5. `ServerGuard::drop()` — Cleanup on scope exit**
```rust
// CURRENT:
impl Drop for ServerGuard {
    fn drop(&mut self) {
        let mut servers_lock = self.servers.lock().unwrap();  // PANICS!
        // ...
    }
}
```
**This is dangerous.** If `drop()` is called during unwinding from another panic, and `drop()` itself panics (from `.unwrap()`), Rust **aborts the process**. Double-panic = abort, no recovery possible.

```rust
// v1.0.4 TARGET:
impl Drop for ServerGuard {
    fn drop(&mut self) {
        // In Drop, we CANNOT propagate errors via Result. We MUST handle inline.
        // Use match to recover from poisoning rather than double-panic.
        let mut servers_lock = match self.servers.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                // Recover the guard despite poisoning — better than aborting
                eprintln!("WARNING: ServerGuard::drop() encountered poisoned mutex. \
                           Recovering to mark server {} as available, but state may be inconsistent.",
                           self.key);
                poisoned.into_inner()
            }
        };
        // ... rest of cleanup
    }
}
```

**6. `ResponseBodyWithGuard::poll_next()` — Stream polling during response**
```rust
// CURRENT:
fn poll_next(...) -> Poll<Option<Self::Item>> {
    // ...
    let mut servers_lock = self.servers.lock().unwrap();  // PANICS!
    // ...
}
```
This is called by hyper's runtime while streaming the response body. A panic here propagates to hyper's connection handler.

```rust
// v1.0.4 TARGET:
fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    // ...
    // Our Stream's Item is Result<bytes::Bytes, std::io::Error>, so we CAN propagate errors.
    // However, for mutex poisoning during cleanup, recovery is preferred:
    let mut servers_lock = match self.servers.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            // Option A: Recover and continue (preferred for state-update operations)
            eprintln!("WARNING: Mutex poisoned during stream poll, recovering");
            poisoned.into_inner()

            // Option B: Return error through the stream (terminates response mid-stream)
            // return Poll::Ready(Some(Err(std::io::Error::new(
            //     std::io::ErrorKind::Other,
            //     "Internal state corrupted: mutex poisoned during streaming"
            // ))));
        }
    };
    // ...
}
```

**Alternative: `StreamExt::catch_unwind`** — If you want to catch panics that occur during stream polling (e.g., from the underlying `reqwest` stream), you can wrap the stream:

```rust
use futures::StreamExt;
use std::panic::AssertUnwindSafe;

let safe_stream = AssertUnwindSafe(response.bytes_stream()).catch_unwind();
// Now safe_stream yields Result<Result<Bytes, reqwest::Error>, Box<dyn Any + Send>>
// where the outer Err indicates a panic occurred
```

However, this changes the stream's item type and requires additional handling. For the custom `ResponseBodyWithGuard`, directly handling poison errors in `poll_next` (as shown above) is simpler.

#### The Panic Catch-All (Belt and Suspenders)

Even with all `.unwrap()` removed, panics can still occur:
- Bugs in our logic (index out of bounds, integer overflow with `panic` profile, etc.)
- Panics from library code we call
- `assert!()` failures

**Approach 1: `FutureExt::catch_unwind` (Recommended)**

The simplest and most direct approach for async handlers:

```rust
use futures::FutureExt;
use std::panic::AssertUnwindSafe;

async fn handle_with_panic_catching(req: Request<Body>) -> Response<Body> {
    let result = AssertUnwindSafe(actual_handler(req)).catch_unwind().await;

    match result {
        Ok(response) => response,
        Err(panic_payload) => {
            let msg = extract_panic_message(&panic_payload);
            eprintln!("PANIC caught: {}", msg);
            error_response(500, "Internal server error")
        }
    }
}

fn extract_panic_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    payload.downcast_ref::<&str>().map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "Unknown panic".to_string())
}
```

**Approach 2: `tokio::spawn` with `JoinHandle` (Alternative)**

Spawning into a separate task provides panic isolation, but adds overhead and complexity:

```rust
let handle = tokio::spawn(async move {
    handle_request_inner(req, servers, remote_addr, timeout).await
});

match handle.await {
    Ok(Ok(response)) => response,
    Ok(Err(e)) => {
        // Normal error — propagated via Result
        eprintln!("Request error: {}", e);
        error_response(500, &format!("Internal error: {}", e))
    }
    Err(join_error) => {
        // Task panicked or was cancelled
        if join_error.is_panic() {
            let panic_payload = join_error.into_panic();
            let msg = extract_panic_message(&panic_payload);
            eprintln!("PANIC in spawned task: {}", msg);
            error_response(500, "Internal server error (panic)")
        } else {
            // Task was cancelled (e.g., runtime shutdown)
            eprintln!("Task cancelled");
            error_response(500, "Request cancelled")
        }
    }
}
```

See [`JoinError` documentation](https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html) for details on `is_panic()` and `into_panic()`.

**Why prefer `FutureExt::catch_unwind`?** It's simpler, doesn't require spawning a new task, and integrates naturally with hyper's service model. Use `tokio::spawn` when you need true task isolation (e.g., the handler should continue even if the connection is dropped).

**Important Caveats:**
- `catch_unwind` only catches **unwinding** panics. If `panic = "abort"` is set in `Cargo.toml`, panics abort immediately and cannot be caught. We rely on the default `panic = "unwind"`.
- `AssertUnwindSafe` is a promise to the compiler that the wrapped value is safe across panic boundaries. This is generally true for our use case (HTTP handlers), but be aware that state may be inconsistent after a caught panic.

#### Mutex Choice: `std::sync::Mutex` vs `tokio::sync::Mutex`

The current codebase uses `std::sync::Mutex`, which has **poisoning semantics**: if a thread panics while holding the lock, the mutex becomes "poisoned" and subsequent `lock()` calls return `Err(PoisonError)`.

**Alternative: `tokio::sync::Mutex`**

[`tokio::sync::Mutex`](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html) does **NOT** poison:
- On panic, it simply unlocks (the data may be inconsistent)
- `lock().await` always succeeds (returns the guard directly, not a `Result`)
- Trade-off: You lose the "something went wrong" signal that poisoning provides

For this load balancer, `std::sync::Mutex` is preferred because:
1. Lock hold times are very short (just reading/updating state)
2. We never `.await` while holding the lock (so no risk of holding across yield points)
3. Poisoning alerts us to bugs that corrupted state

If you want to eliminate all `PoisonError` handling, switch to `tokio::sync::Mutex`—but accept that corrupted state may silently persist.

#### Library Error Types (Cargo.toml versions: hyper 0.14, reqwest 0.12, tokio 1, futures-util 0.3)

All error types in our dependency tree implement `std::error::Error + Display`, making them compatible with `Box<dyn std::error::Error>`:

| API | Error Type | Propagates via `?` | Notes |
|-----|------------|-------------------|-------|
| [`Mutex::lock()`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) | `PoisonError<MutexGuard<T>>` | Yes | Use `.into_inner()` to recover data in Drop contexts |
| [`reqwest::ClientBuilder::build()`](https://docs.rs/reqwest/0.12/reqwest/struct.ClientBuilder.html) | `reqwest::Error` | Yes | Fails on TLS backend init or DNS resolver config. Can also fail with `hickory-dns` feature on malformed system DNS config. |
| [`Response::builder().body()`](https://docs.rs/http/latest/http/response/struct.Builder.html#method.body) | `http::Error` | Yes | Fails if **earlier** builder calls set invalid data (e.g., malformed headers). With our static/validated inputs, this won't fail. |
| [`tokio::signal::ctrl_c()`](https://docs.rs/tokio/latest/tokio/signal/fn.ctrl_c.html) | `std::io::Error` | Yes | Fails if OS signal handler can't be registered |
| [`HeaderValue::to_str()`](https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.to_str) | `ToStrError` | Yes | Fails if header contains non-visible ASCII (bytes outside 32-126, except tab). This is **not** a UTF-8 check—HTTP headers can contain bytes 128-255 which are valid but not "visible ASCII". Use `as_bytes()` and convert as needed. |
| [`FutureExt::catch_unwind()`](https://docs.rs/futures-util/latest/futures_util/future/trait.FutureExt.html#method.catch_unwind) | `Box<dyn Any + Send>` | Via match | Catches panics during future polling. The `Err` variant contains the panic payload. |
| [`StreamExt::catch_unwind()`](https://docs.rs/futures-util/latest/futures_util/stream/trait.StreamExt.html#method.catch_unwind) | `Box<dyn Any + Send>` | Via match | Same as above, but for streams. Caught panic becomes the final stream item. |

**Note on `HeaderValue::to_str()` in current code:** The existing code at line 219 uses `.to_str().unwrap()` when copying response headers. This can panic if the upstream Ollama server returns a header with non-visible ASCII characters. The fix is:
```rust
// Instead of:
resp_builder = resp_builder.header(key_h.to_string(), value.to_str().unwrap());

// Use:
resp_builder = resp_builder.header(key_h.as_str(), value.as_bytes());
// or with graceful handling:
if let Ok(v) = value.to_str() {
    resp_builder = resp_builder.header(key_h.as_str(), v);
}
```

#### Summary: What Gets Handled vs. What Propagates

| Error Scenario | Action Plan? | Handling |
|---------------|--------------|----------|
| Upstream Ollama server unreachable | **Yes** | Return HTTP 502 Bad Gateway |
| No servers available | **Yes** | Return HTTP 503 Service Unavailable |
| Invalid HTTP method from client | **Yes** | Return HTTP 405 Method Not Allowed |
| Mutex poisoned | **No** | Propagate `Box<dyn Error>` upward (or recover in Drop via `into_inner()`) |
| TLS/DNS init failed | **No** | Propagate upward |
| Signal handler registration failed | **No** | Propagate upward |
| Unexpected panic | **No** | Catch via `FutureExt::catch_unwind`, log, return HTTP 500 |
| Header contains non-visible ASCII | **Context-dependent** | Use `as_bytes()` to copy verbatim, or skip/transform the header |

The principle: **handle what you have a plan for; propagate everything else.** Never panic, never abort, never crash the caller's process.

#### References

- [Hyper Traps: Panic Handling](https://vorner.github.io/2020/04/13/hyper-traps.html) — Essential reading on catching panics in hyper services
- [futures-util FutureExt](https://docs.rs/futures-util/latest/futures_util/future/trait.FutureExt.html) — `catch_unwind` for async futures
- [futures-util StreamExt](https://docs.rs/futures-util/latest/futures_util/stream/trait.StreamExt.html) — `catch_unwind` for async streams
- [tokio JoinError](https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html) — Panic handling for spawned tasks
- [Rust Mutex Poisoning](https://doc.rust-lang.org/std/sync/struct.Mutex.html#poisoning) — When and why mutexes poison
- [tokio::sync::Mutex](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html) — Non-poisoning alternative (note: more expensive, designed for holding across `.await`)
- [hyper 0.14 Upgrade Guide](https://hyper.rs/guides/1/upgrading/) — Preparing for hyper 1.0 migration

## Testing

### Basic Testing

For basic validation of load balancer functionality with non-streaming HTTP POST requests and GET requests, use the [test script](./test/test_static_http_post_request.py):

```sh
python3 -m pip install requests
python3 test/test_static_http_post_request.py
```

This script sends a non-streaming `/api/chat` request and a `/api/tags` GET request to verify the load balancer is proxying correctly.

### Testing with Real Ollama Servers

For comprehensive testing against real Ollama servers, the [Lab testing](#lab-testing) section describes a multi-VM approach. This involves running multiple virtual machines on a single host, each with its own network interface and IP address, allowing you to simulate a production environment where multiple physical Ollama servers exist on the network.

This approach is ideal for:
- End-to-end integration testing with actual model inference
- Testing reliability features (server failure, recovery, `Unreliable`/`SecondChanceGiven` states)
- Performance testing with real streaming responses
- Validating behavior under concurrent load from multiple clients

### Automated Testing with Ollama Simulator

The [Ollama simulator](./test/ollama_simulator/) provides automated testing without requiring real GPU hardware or Ollama installations.

```bash
cd test/ollama_simulator
cargo run --release --bin load_balancer_test
```

The test suite validates 15 scenarios including basic routing, load balancing, failure handling, server recovery, streaming responses, KV cache prefix matching, embeddings endpoints, and TCP shutdown behavior (radio silence via SIGSTOP, RST, graceful FIN). See [test/ollama_simulator/README.md](./test/ollama_simulator/README.md) for details.

The simulator implements Ollama endpoints (`/api/chat`, `/api/generate`, `/api/embed`, `/api/embeddings`, `/api/tags`, `/api/ps`, `/api/version`, `/api/show`) and compatibility layers (`/v1/chat/completions`, `/v1/embeddings`, `/v1/models`, `/v1/messages`) with configurable behaviors and realistic KV cache simulation.

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
