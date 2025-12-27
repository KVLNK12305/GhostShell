# 🧪 GhostShell  
### A Deceptive Command Execution & Interaction Lab

GhostShell is an interactive, web-based platform designed to explore how humans interact with systems that **execute commands, encode meaning, and present information that may not be entirely truthful**.

It combines a real terminal environment with controlled deception, symbolic encoding, and interpretive challenges to study **trust, intent, and assumption-making** in technical interfaces.

GhostShell is not a traditional CTF, not a pure sandbox, and not a single-purpose tool.  
It is a **multi-module interaction lab** where behavior matters more than outcomes.

---

## 🧠 Core Idea

> What if interfaces didn’t always tell the truth — but stayed believable?

GhostShell allows users to interact with realistic systems while the platform:
- Observes how users explore and probe
- Alters or abstracts representations of data
- Introduces ambiguity, encoding, and deception
- Encourages reasoning over rote tool usage

The focus is not “getting the right answer”, but understanding **how users think** when certainty is removed.

---

## 🎭 Dual Reality Model

### 👤 What the User Sees
- A clean web interface
- A real terminal
- Familiar commands and tools
- Clearly scoped learning modules

### 🧠 What the System Does
- Intercepts and evaluates interactions
- Controls how information is revealed
- Introduces believable inconsistencies
- Logs behavioral patterns and decisions

At no point is the user explicitly told whether the system is being fully honest.

---

## 🧩 Modules

GhostShell is composed of independent but thematically linked modules.  
Each module explores a different layer of interaction, interpretation, or trust.

---

### 🖥️ 1. Terminal Foundations

A guided introduction to basic command-line interaction.

**Focus**
- Core shell commands
- File navigation
- Output interpretation

**Purpose**
- Establishes a shared baseline
- Ensures users are comfortable with terminal semantics
- Serves as the entry point to later deception

This module is intentionally straightforward — the “truthful” baseline.

---

### 🧩 2. Emoji Encoder

A symbolic text encoder implemented as a dedicated React component.

**What It Does**
- Converts plain text into emoji-based representations
- Decodes emoji sequences back into text
- Supports deterministic, reversible mappings

**What It Explores**
- Meaning through symbols
- Human pattern recognition
- The illusion of harmless or playful data

This module is not steganography — it is **semantic encoding**.  
Nothing is hidden in bits; meaning is hidden in interpretation.

---

### 🖼️ 3. Hidden Data Principles

A minimal, modern take on classic data-hiding ideas.

**Tools Included**
- `binwalk`
- `exiftool`
- `strings`
- `zsteg`

**Focus**
- Appended data
- Metadata leakage
- Least-significant-bit concepts (at a high level)

Steganography here is treated as **one historical example** of concealed meaning — not the platform’s identity.

---

### 🪞 4. Reality Check (Out-of-the-Box Module)

A module focused entirely on **trusting output**.

**Concept**
Users execute simple commands and receive output that may be:
- Accurate
- Slightly altered
- Selectively incomplete
- Entirely fabricated but plausible

No warning is given.

**What It Explores**
- Over-trust in interfaces
- Assumption-based reasoning
- Verification habits vs intuition
- How small inconsistencies are ignored

This module represents GhostShell’s core philosophy most directly.

---

## 🛠️ Execution Environment

### Interactive Terminal
- Browser-based terminal powered by Xterm.js
- Real-time input and output
- Familiar Linux command behavior

### Controlled Execution
- Commands executed inside disposable Docker containers
- Session-scoped environments
- No persistence between runs

### Command Mediation
- All commands intercepted server-side
- Routed to:
  - Real execution
  - Simulated output
  - Controlled failure responses

---

## 🏗️ High-Level Architecture

1. User interacts with a module (terminal or UI component)
2. Input is sent to the backend via WebSocket
3. The GhostShell engine evaluates intent and context
4. A response path is chosen:
   - Real execution
   - Symbolic transformation
   - Deceptive but consistent output
5. Output is returned to the frontend
6. Interaction metadata is logged

The user is never explicitly told which path was taken.

---

## 🧬 Design Principles

- Believability over restriction  
- Observation over prevention  
- Interpretation over instruction  
- Behavior over correctness  

GhostShell is designed to provoke thought, not enforce rules.

---

## 🏁 Getting Started (Local Development)

### Prerequisites
- Node.js v18+
- pnpm
- Docker (running)

### Clone the Repository
```bash
git clone https://github.com/KVLNK12305/StegoSim.git
cd StegoSim
````

### Build the Runtime Image

```bash
cd sandbox
docker build -t ghostshell-runtime:latest .
cd ..
```

### Install Dependencies

```bash
# Frontend
cd frontend && pnpm install

# Backend
cd ../backend && pnpm install
```

### Run the Platform

```bash
# Backend (port 3000)
cd backend && pnpm start

# Frontend (port 5173)
cd frontend && pnpm dev
```

Open `http://localhost:5173` in your browser.

---

## ⚠️ Security Notice

GhostShell intentionally executes user-supplied commands in controlled environments.

Before any public deployment:

* Enforce strict resource limits
* Disable outbound networking
* Harden container isolation
* Perform a full security audit

This project is intended for **learning, experimentation, and research**, not production use.

---

## 📜 License

MIT
