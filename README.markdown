# Reverse-Socks5-WebSocks

RustySmuggler is a reverse SOCKS5 proxy tunneled over WebSockets, written in Rust with additional TypeScript and HTML components.

## Overview

This project provides a reverse SOCKS5 proxy that allows users to access hosts on an internal network through a WebSocket tunnel. The server runs on an internet-accessible host and listens for a reverse WebSocket connection from the client (implant) placed inside a network. Users can then proxy through the server's frontend port to reach internal network hosts.

## Features

- **Reverse SOCKS5 Proxy**: Tunnels traffic over WebSockets for secure and flexible proxying.
- **Cross-Platform**: Supports Windows and Linux (Debian-based) environments.
- **Customizable**: Configurable backend and frontend addresses for flexible deployment.
- **Lightweight**: Built with Rust for performance and minimal resource usage.

## Prerequisites

To fork, build, and run this project, you need the following software:

- **Git**: For cloning and managing the repository.
- **Rust**: For building the server and client binaries.
- **Node.js and npm** (optional): For working with TypeScript and HTML components (e.g., web interface).
- **Text Editor/IDE**: For editing configuration files (e.g., VS Code, Vim, Nano).
- **Proxychains** (optional): For testing the proxy functionality.
- **iptables** (Linux, optional): For securing the frontend port.
- **OpenSSL** (optional): For secure WebSocket connections (wss://).
- **Build Tools**:
  - Windows: Microsoft Visual C++ Build Tools.
  - Linux (Debian): `build-essential`.

## Installation Instructions

### For Windows

1. **Install Git**:

   - Download and install Git from [git-scm.com](https://git-scm.com/downloads/win).
   - Use default settings or customize as needed. Ensure Git Bash is installed.
   - Verify: `git --version`

2. **Install Rust**:

   - Download the Rust installer (`rustup`) from [rust-lang.org](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe).
   - Run the installer and select the default stable toolchain.
   - Add Rust to your PATH (usually automatic).
   - Verify: `rustc --version` and `cargo --version`

3. **Install PostgreSQL**:

   - Download the PostgreSQL installer from [postgresql.org](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads).
   - Run the installer and follow prompts:
     - Choose the latest stable version (e.g., PostgreSQL 16).
     - Select components (e.g., PostgreSQL Server, pgAdmin, Command Line Tools).
     - Set a password for the `postgres` superuser.
     - Keep default port (5432) unless conflicts exist.
   - Add PostgreSQL’s `bin` directory to PATH (e.g., `C:\Program Files\PostgreSQL\16\bin`).
   - Verify: `psql --version`
   - Start the PostgreSQL service:
     - Open Services (Run `services.msc`), find `postgresql-x64-16`, and ensure it’s running.
   - Optional: Launch pgAdmin for GUI-based database management.

4. **Install Node.js and npm** (optional):

   - Download the LTS version from nodejs.org.

   ```bash
   powershell -c "irm https://community.chocolatey.org/install.ps1|iex"
   choco install nodejs --version="22.18.0"
   node -v
   corepack enable pnpm
   pnpm -v
   ```

   - Install with default settings.
   - Verify: `node --version` and `npm --version`

5. **Install Microsoft Visual C++ Build Tools**:

   - Download from [visualstudio.microsoft.com](https://aka.ms/vs/17/release/vs_BuildTools.exe).
   - Select “Desktop development with C++” workload.
   - Required for Rust compilation.

6. **Install OpenSSL** (optional):

   - Download from [slproweb.com](https://slproweb.com/download/Win64OpenSSL-3_5_2.exe).
   - Add OpenSSL to your system PATH.

### For Linux (Debian-based)

1. **Install Git**:

   ```bash
   sudo apt update
   sudo apt install git
   ```

   - Verify: `git --version`

2. **Install Rust**:

   - Install `curl`:

     ```bash
     sudo apt install curl
     ```

   - Install Rust using `rustup`:

     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

   - Follow prompts to install the stable toolchain.
   - Add Rust to PATH:

     ```bash
     source $HOME/.cargo/env
     ```

   - Verify: `rustc --version` and `cargo --version`

3. **Install PostgreSQL**:

   - Install PostgreSQL and development libraries:

     ```bash
     sudo apt install postgresql postgresql-contrib libpq-dev
     ```

   - Start and enable the PostgreSQL service:

     ```bash
     sudo systemctl start postgresql
     sudo systemctl enable postgresql
     ```

   - Verify installation:

     ```bash
     psql --version
     ```

   - Set up the `postgres` user password (optional):

     ```bash
     sudo -u postgres psql -c "ALTER USER postgres WITH PASSWORD 'your_password';"
     ```

   - Access PostgreSQL:

     ```bash
     sudo -u postgres psql
     ```

4. **Install Node.js and npm** (optional):

   ```bash
   sudo apt install nodejs npm
   ```

   - Alternatively, use `nvm`:

     ```bash
     curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
     nvm install --lts
     ```

   - Verify: `node --version` and `npm --version`

5. **Install Build Tools**:

   ```bash
   sudo apt install build-essential
   ```

6. **Install Proxychains** (optional):

   ```bash
   sudo apt install proxychains
   ```

7. **Install iptables** (optional):

   ```bash
   sudo apt install iptables
   ```

8. **Install OpenSSL** (optional):

   ```bash
   sudo apt install libssl-dev
   ```

## Building the Project

### For Windows and Linux

1. **Build Release Binaries**:

   ```bash
   cargo build --release
   ```

   - Binaries will be in:
     - Windows: `.\target\release\server.exe` and `.\target\release\client.exe`
     - Linux: `./target/release/server` and `./target/release/client`

2. **Build with Debug Output** (not recommended for production):

   - Windows (Command Prompt):

     ```cmd
     set RUSTFLAGS=--cfg debug
     cargo build --bin client
     cargo build --bin server
     ```

   - Windows (PowerShell):

     ```powershell
     $env:RUSTFLAGS='--cfg debug'
     cargo build --bin client
     cargo build --bin server
     ```

   - Linux:

     ```bash
     RUSTFLAGS='--cfg debug' cargo build
     ```

3. **Configure the Client**:

   - Edit `client/src/config.rs` with a text editor.
   - Update `IP` and `PORT` to match your server’s backend address (e.g., `-b` address).

     ```rust
     pub const IP: &str = "192.168.1.100";
     pub const PORT: u16 = 3030;
     ```

   - Rebuild the client:

     ```bash
     cargo build --release --bin client
     ```

4. **TypeScript/HTML Components** (optional):

   - If the project includes a web interface, check for a `package.json` file.
   - Install dependencies:

     ```bash
     npm install
     ```

   - Follow any additional instructions in the relevant directory.

## Running the Project

### Running the Server

1. **Start the Server**:

   - Windows:

     ```cmd
     .\target\release\server.exe -b 0.0.0.0:3030 -f 0.0.0.0:2020
     ```

   - Linux:

     ```bash
     ./target/release/server -b 0.0.0.0:3030 -f 0.0.0.0:2020
     ```

   - Replace `0.0.0.0:3030` with the backend address/port (client connects here).
   - Replace `0.0.0.0:2020` with the frontend address/port (proxy client connects here).

2. **Secure the Frontend Port** (optional):

   - **Windows**: Use Windows Firewall to restrict access to the frontend port (e.g., `2020`).
   - **Linux**:

     ```bash
     sudo iptables -A INPUT -p tcp --dport 2020 -s <allowed-ip> -j ACCEPT
     sudo iptables -A INPUT -p tcp --dport 2020 -j DROP
     ```

### Running the Client (Implant)

1. **Copy the Client Binary**:

   - Copy `client.exe` (Windows) or `client` (Linux) to the target machine inside the internal network.

2. **Run the Client**:

   - Windows:

     ```cmd
     .\target\release\client.exe
     ```

   - Linux:

     ```bash
     ./target/release/client
     ```

### Using the Proxy

1. **Configure Proxychains** (Linux, optional):

   - Edit `/etc/proxychains.conf` or `~/.proxychains/proxychains.conf`:

     ```conf
     socks5 127.0.0.1 2020
     ```

   - Run commands through the proxy:

     ```bash
     proxychains <command>
     ```

     Example: `proxychains curl http://example.com`

2. **Configure Browser**:

   - Set the browser’s SOCKS5 proxy to `127.0.0.1:2020` (or the server’s IP and frontend port).
   - For Windows, use a proxy client like Proxifier if `proxychains` is unavailable.

## Tips

- **IP Addresses**: Use IP addresses instead of domain names for `-b` and `-f` options to avoid resolution issues.
- **Security**: Restrict access to the frontend port using firewall rules or SSH forwarding.
- **Binary Size**: For smaller binaries, refer to the linked guide in the original repository.
- **Debug Builds**: Avoid debug builds in production due to potential Indicators of Compromise (IoCs).

## About

- **Languages**: Rust (72.5%), TypeScript (18.6%), HTML (8.5%), Other (0.4%)
- **License**: See LICENSE file.

## Contributing

Contributions are welcome! Please fork the repository, make changes, and submit a pull request.

## Contact

For issues or questions, open an issue on the GitHub repository.
