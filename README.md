# TimeToThink

A macOS-only Pomodoro timer application built with Tauri (Rust backend + Vue 3 frontend) that enforces break periods through system lock screen integration and fullscreen overlays.

## Features

- **25-minute Focus sessions** with automatic transitions to breaks
- **Smart break scheduling**: Every 3rd pomodoro triggers a 25-minute long break
- **Enforced breaks**: macOS lock screen triggers within 5 seconds of break start
- **Fullscreen break guard**: Enforced overlay that prevents skipping breaks without confirmation
- **Skip confirmation**: Two-step verification to skip breaks (prevents accidental skips)
- **Crash recovery**: State persistence ensures incomplete breaks continue after restart

## Architecture

### State Machine

The app uses a three-state finite state machine:

- **FOCUS** (25:00) - Work period
- **SHORT_BREAK** (5:00) - Regular break
- **LONG_BREAK** (25:00) - Extended break after every 3 pomodoros

### Tech Stack

- **Backend**: Rust with Tauri 2.x
- **Frontend**: Vue 3 (Composition API) + TypeScript
- **State Management**: Direct Rust → Vue communication via Tauri Events

## Project Structure

```
TimeToThink/
├── src/                          # Vue frontend
│   ├── components/
│   │   ├── MainView.vue          # Main timer UI
│   │   ├── BreakGuardView.vue    # Fullscreen break overlay
│   │   └── SkipConfirmDialog.vue # Skip confirmation modal
│   ├── composables/              # (预留)
│   ├── App.vue
│   ├── main.ts
│   └── styles.css
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs               # Tauri entry point
│   │   ├── lib.rs                # Module exports
│   │   ├── timer_engine.rs       # Core state machine
│   │   ├── macos_lock.rs         # Lock screen implementation
│   │   ├── guard_control.rs      # BreakGuard window control
│   │   ├── storage.rs            # State persistence
│   │   └── types.rs              # Shared types
│   ├── Cargo.toml
│   ├── tauri.conf.json           # Window configs
│   └── build.rs
├── package.json
└── tsconfig.json
```

## Development

### Prerequisites

- Node.js 18+ and npm
- Rust toolchain (rustc, cargo)
- macOS (required for lock screen integration)

### Installation

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri:dev

# Build for production
npm run tauri:build
```

### Fast Testing Mode

Use `TTT_TEST_MODE=1` to run short timer cycles during development:

```bash
TTT_TEST_MODE=1 npm run tauri:dev
```

Fast mode durations:
- Focus = 10s
- Short Break = 8s
- Long Break = 12s

### Kiosk Mode (Strong Focus)

`TTT_KIOSK_MODE` controls strict break enforcement:

- `TTT_KIOSK_MODE=1` (default): stronger break fullscreen policy and block app exit during active break
- `TTT_KIOSK_MODE=0`: disable strict kiosk behavior

### Development Commands

```bash
# Start Vite dev server
npm run dev

# Run Tauri in development
npm run tauri:dev

# Build release
npm run tauri:build

# Run Rust tests
cargo test

# Format Rust code
cargo fmt

# Quick compile check
cargo check
```

## Acceptance Criteria

- [x] 25:00 focus countdown works correctly
- [x] Focus ends → auto-enter break
- [x] Enter break → auto-lock screen within 5 seconds (once only)
- [x] After unlock → fullscreen break guard appears immediately
- [x] During break, "Skip Break" shows secondary confirmation
- [x] Cancel confirmation → continue break
- [x] Confirm skip → immediately return to focus
- [x] Every 3rd pomodoro → next break is 25:00 long break
- [ ] (Recommended) App restart → state recovers correctly

## How It Works

### Timer Engine

The Rust `TimerEngine` maintains the application state:
- 1-second tick loop using `tokio::time::interval`
- State transitions trigger Tauri events to the frontend
- Lock screen triggered 5 seconds after break starts (once per break)

### Lock Screen Integration

Uses macOS `CGSession -suspend` command:
```bash
CGSession -suspend
```

Executed only once per break, during the first 5 seconds.

### BreakGuard Window

- Fullscreen overlay with `alwaysOnTop: true`
- Polling mechanism (500ms interval) ensures window stays focused
- Shows remaining break time with progress bar
- "Continue Break" (primary) and "Skip Break" (secondary) buttons

### State Persistence

State saved to `~/.config/timetothink/state.json`:
```json
{
  "state": "ShortBreak",
  "remaining_seconds": 270,
  "completed_pomodoros": 2,
  "lock_triggered": true,
  "end_timestamp": null
}
```

## License

MIT License - See LICENSE file for details

## Contributing

This is a personal project focused on macOS productivity tools. Contributions welcome!

## Roadmap

- [ ] System tray integration (menu bar residency)
- [ ] Custom break durations
- [ ] Sound notifications
- [ ] Statistics and analytics
- [ ] Custom themes
