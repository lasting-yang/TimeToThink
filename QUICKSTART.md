# Quick Start Guide - TimeToThink

## Run the Application

```bash
# Navigate to project directory
cd /Users/zero/code/git/TimeToThink

# Run in development mode
npm run tauri:dev
```

This will:
1. Start the Vite dev server (http://localhost:1420)
2. Compile and launch the Tauri application
3. Open the main timer window

## Fast Testing Mode

To avoid waiting 25 minutes, enable fast timer durations with one env var:

```bash
TTT_TEST_MODE=1 npm run tauri:dev
```

When `TTT_TEST_MODE=1`:
- Focus = 10s
- Short Break = 8s
- Long Break = 12s

## Kiosk Mode (Strong Focus)

Kiosk mode is enabled by default during break time (`TTT_KIOSK_MODE=1`).

- Break window uses stronger fullscreen/workspace policy
- App exit is blocked while an enforced break is active

To disable it temporarily:

```bash
TTT_KIOSK_MODE=0 npm run tauri:dev
```

## Using the App

### Starting a Focus Session
1. Click the "Start" button
2. Timer counts down from 25:00
3. Work on your tasks!

### During Break
When focus ends:
- **Screen locks** within 5 seconds (macOS system lock)
- After unlocking, **fullscreen break guard** appears
- Timer shows remaining break time
- Click "Continue Break" to keep resting (recommended)
- Click "Skip Break" to skip (requires confirmation)

### Skip Confirmation
- Clicking "Skip Break" shows a warning dialog
- "Continue Break" is the default (prevents accidental skips)
- "Yes, Skip Break" returns to focus immediately

### Long Breaks
- After every 3 pomodoros, you get a **25-minute long break**
- Same rules as short breaks apply

## Controls

- **Start** - Begin or resume timer
- **Pause** - Pause timer (only during focus)
- **Reset** - Reset to initial state

## Testing the Features

### Test State Transitions
1. Start timer → should count down from 25:00
2. Wait for 0:00 → should auto-transition to 5:00 break
3. Lock screen should trigger within 5 seconds
4. After unlock → fullscreen guard appears
5. Wait for break to end → returns to 25:00 focus

### Test Skip Confirmation
1. During break, click "Skip Break"
2. Confirmation dialog should appear
3. Click "Continue Break" → dialog closes, break continues
4. Click "Skip Break" again, then "Yes, Skip Break" → returns to focus

### Test Long Break
1. Complete 3 pomodoro sessions
2. 3rd completion → should trigger 25:00 long break

## Development Commands

```bash
# Quick compile check
cd src-tauri && cargo check

# Run Rust tests
cd src-tauri && cargo test

# Format Rust code
cd src-tauri && cargo fmt

# Build release version
npm run tauri:build
```

## Troubleshooting

### Build fails with "frontendDist doesn't exist"
```bash
mkdir -p dist
touch dist/.gitkeep
```

### Icons don't load
- Icons are placeholders - app will work but show default icon
- Replace files in `src-tauri/icons/` with actual icons later

### Lock screen doesn't work
- Ensure you're on macOS
- Check `CGSession -suspend` works in Terminal
- Verify app has necessary permissions

### TypeScript errors
```bash
npm install
```

## Project Status

✅ Core functionality complete
✅ State machine working
✅ Lock screen integration
✅ BreakGuard enforcement
✅ Skip confirmation
⚠️ State persistence implemented but not fully tested
❌ System tray integration (deferred)

## What's Next?

1. **Test thoroughly** - Try all features and verify behavior
2. **Design icons** - Create proper app icons
3. **Enhance persistence** - Test crash recovery
4. **Customize** - Adjust colors, fonts, and animations
5. **Add features** - System tray, custom durations, sounds, etc.

Enjoy your enforced productivity! 🌿
