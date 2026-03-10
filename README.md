# CLIAI
[![Release](https://github.com/SrgGrch/CLIAI/actions/workflows/release.yml/badge.svg)](https://github.com/SrgGrch/CLIAI/actions/workflows/release.yml)

A CLI tool that converts natural language descriptions into executable shell commands. Supports any OpenAI-compatible API (default: OpenRouter).

## Contents

- [Installation](#installation)
  - [Shell script (macOS / Linux)](#shell-script-macos--linux)
  - [PowerShell (Windows)](#powershell-windows)
  - [Homebrew (macOS / Linux)](#homebrew-macos--linux)
  - [Pre-built Binaries](#pre-built-binaries)
  - [Build from Source](#build-from-source)
- [Configuration](#configuration)
- [Quick Start](#quick-start)
- [Usage](#usage)

---

## Installation

### Shell script (macOS / Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/SrgGrch/CLIAI/releases/latest/download/cliai-installer.sh | sh
```

### PowerShell (Windows)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/SrgGrch/CLIAI/releases/latest/download/cliai-installer.ps1 | iex"
```

### Homebrew (macOS / Linux)

```bash
brew tap SrgGrch/cliai
brew install cliai
```

### Pre-built Binaries

Download the latest release from [Releases](https://github.com/SrgGrch/CLIAI/releases):

| Platform | Architecture |
|----------|-------------|
| Linux | x86_64, aarch64 |
| macOS | Intel (x86_64), Apple Silicon (aarch64) |
| Windows | x86_64 |

### Build from Source

**Requirements:** [Rust](https://rustup.rs/) (edition 2024, cargo 1.93+)

```bash
git clone https://github.com/SrgGrch/CLIAI.git
cd CLIAI
cargo build --release
```

The binary will be at `target/release/cliai`. To install it system-wide:

```bash
sudo cp target/release/cliai /usr/local/bin/
```

---

## Configuration

Before the first use, set your API key and select a model.

### API Key

```bash
cliai --api-key "sk-or-xxxxxxxxxxxxxx"
```

### Model

```bash
cliai --list-models                          # browse available models
cliai --model "google/gemini-2.0-flash-001"  # select a model
```

### Config File

Settings are saved automatically to a TOML file:

| OS | Path |
|----|------|
| Linux | `~/.config/cliai/config.toml` |
| macOS | `~/Library/Application Support/cliai/config.toml` |
| Windows | `%APPDATA%\cliai\config\config.toml` |

```toml
api_key = "sk-or-xxxxxxxxxxxxxx"
model_id = "google/gemini-2.0-flash-001"
endpoint = "https://openrouter.ai/api/v1"
```

---

## Quick Start

```bash
cliai "list all files in the current directory"
```

1. The prompt is sent to the AI
2. The suggested command is displayed in a colored box
3. You are asked: `Accept? (y/n):`
4. Type `y` to execute and see the output

---

## Usage

```
cliai [OPTIONS] [PROMPT]...
```

### Arguments

| Argument | Description |
|----------|-------------|
| `[PROMPT]...` | Task description in any language. Multiple words are joined into a single string. |

### Options

| Flag | Description |
|------|-------------|
| `-a`, `--api-key <KEY>` | Set the API key and save it to config |
| `-m`, `--model <MODEL>` | Set the model and save it to config |
| `-e`, `--endpoint <URL>` | Set an OpenAI-compatible API endpoint (default: `https://openrouter.ai/api/v1`) |
| `-l`, `--list-models` | List available models (current model highlighted in green) |
| `-v`, `--verbose` | Enable verbose mode (prints the raw AI response) |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

### Examples

```bash
# Simple prompt
cliai "show disk usage"

# Multiple words without quotes
cliai find all python files in src

# Override model for a single request
cliai -m "anthropic/claude-opus-4" "archive the logs folder"

# Use a custom API endpoint (e.g. local Ollama)
cliai -e "http://localhost:11434/v1" "list running processes"

# Verbose mode — shows the full AI response
cliai -v "remove temporary files"

# List available models
cliai -l

# Save API key
cliai -a "sk-or-xxxxxxxxxxxxxx"
```
