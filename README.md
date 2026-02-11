# TimeToThink

A macOS-only Pomodoro timer application that enforces break periods through fullscreen overlay.

## Features

- **25-minute Focus sessions** with automatic transitions to breaks
- **Forced 5-minute breaks** - fullscreen guard prevents you from skipping breaks
- **Long breaks** - every 3rd pomodoro triggers a 25-minute extended break
- **Break guard** - fullscreen overlay with skip confirmation
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

- macOS
- Node.js 18+
- Rust toolchain
