# TimeToThink

A macOS-only Pomodoro timer application that enforces break periods through system lock screen integration.

## Features

- **25-minute Focus sessions** with automatic transitions to breaks
- **Forced 5-minute breaks** - lock screen triggers to ensure you rest
- **Long breaks** - every 3rd pomodoro triggers a 25-minute extended break
- **Fullscreen break guard** - prevents skipping breaks without confirmation
- **Crash recovery** - state persists across app restarts

## Tech Stack

- **Backend**: Rust with Tauri 2.x
- **Frontend**: Vue 3 + TypeScript

## Installation

```bash
npm install
npm run tauri:dev
```

## Requirements

- macOS (required for lock screen integration)
- Node.js 18+
- Rust toolchain
