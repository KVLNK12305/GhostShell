# StegoSim: The Steganography Simulator & Honeypot 🕵️‍♂️🍯

**StegoSim** is a dual-purpose security platform. It serves as an interactive, web-based sandbox for learning steganalysis, while simultaneously acting as a high-interaction honeypot to detect and log malicious command execution attempts.

---

## 🚀 Features

### 🛡️ For Students (The Simulator)
- **Real Web Terminal:** A fully functional, browser-based terminal powered by `Xterm.js`.
- **Pre-configured Sandbox:** Executes commands in disposable, isolated Docker containers.
- **Real Tools:** Pre-installed industrial CTF tools:
  - `zsteg` (LSB analysis for PNG/BMP)
  - `binwalk` (File signature analysis & extraction)
  - `exiftool` (Metadata analysis)
  - `strings` (ASCII string extraction)
- **Challenge Library:** Guided levels to teach core concepts (LSB, appended files, metadata).

### 🍯 For Attackers (The Honeypot)
- **Command Interception:** The backend "Warden" intercepts every command before it hits the sandbox.
- **Malice Detection:** Automatically flags sessions that attempt RCE (e.g., `wget`, `curl`, `rm -rf`, `cat /etc/passwd`).
- **High-Interaction Simulation:** Feeds attackers fake system data (e.g., a fake `/etc/passwd` file) to keep them engaged while logging their activity.
- **Silent Ban:** Automatically terminates connections after repeated malicious attempts.

---

## 🛠️ Tech Stack

- **Frontend:** React.js, Tailwind CSS, Xterm.js
- **Backend (The Warden):** Node.js, Express, Socket.io
- **Execution Engine:** Docker
- **Database:** PostgreSQL (for logging session activity)

---

## 🏗️ Architecture

[ Insert a simple architecture diagram here if you have one, otherwise delete this line ]

1.  **User** types a command in the React terminal.
2.  **Backend (Warden)** receives the command via WebSocket.
3.  **Warden** checks if the command is safe:
    - **If Safe:** Spawns a temporary Docker container, runs the command, captures output, destroys container, returns output to user.
    - **If Malicious:** Logs the incident, returns fake simulated output, and flags the session.

---

## 🏁 Getting Started (Local Development)

### Prerequisites
- Node.js (v18+) & pnpm
- Docker Desktop (must be running)

### Installation

1.  **Clone the repo:**
    ```bash
    git clone [https://github.com/KVLNK12305/StegoSim.git](https://github.com/KVLNK12305/StegoSim.git)
    cd StegoSim
    ```

2.  **Build the Sandbox Image:**
    ```bash
    cd sandbox
    docker build -t stegosim-sandbox:latest .
    cd ..
    ```

3.  **Install Dependencies:**
    ```bash
    # Frontend
    cd frontend && pnpm install
    # Backend
    cd ../backend && pnpm install
    ```

4.  **Run it:**
    - Start Backend: `cd backend && pnpm start` (Runs on port 3000)
    - Start Frontend: `cd frontend && pnpm dev` (Runs on port 5173)
    - Open `http://localhost:5173` in your browser.

---

## ⚠️ Security Note
This project is designed to execute arbitrary commands. While it uses Docker for isolation, **DO NOT** deploy this to a public server without proper resource limits, network hardening, and a rigorous security audit.

---

## 📜 License
MIT