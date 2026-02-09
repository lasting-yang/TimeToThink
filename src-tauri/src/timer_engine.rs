use crate::types::{TimerState, TimerUpdate};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::time::interval;

const LOCK_DELAY_SECONDS: u32 = 5;
const TEST_MODE_ENV: &str = "TTT_TEST_MODE";
const TEST_FOCUS_SECONDS: u32 = 10;
const TEST_SHORT_BREAK_SECONDS: u32 = 8;
const TEST_LONG_BREAK_SECONDS: u32 = 12;

#[derive(Clone, Copy)]
struct TimerDurations {
    focus_seconds: u32,
    short_break_seconds: u32,
    long_break_seconds: u32,
}

impl TimerDurations {
    fn load() -> Self {
        let test_mode_raw = std::env::var(TEST_MODE_ENV).unwrap_or_default();
        let test_mode_enabled = matches!(
            test_mode_raw.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        );

        if test_mode_enabled {
            println!(
                "{} enabled. Using fast timer durations: focus={}s, short_break={}s, long_break={}s",
                TEST_MODE_ENV, TEST_FOCUS_SECONDS, TEST_SHORT_BREAK_SECONDS, TEST_LONG_BREAK_SECONDS
            );
            return Self {
                focus_seconds: TEST_FOCUS_SECONDS,
                short_break_seconds: TEST_SHORT_BREAK_SECONDS,
                long_break_seconds: TEST_LONG_BREAK_SECONDS,
            };
        }

        Self {
            focus_seconds: TimerState::Focus.duration_seconds(),
            short_break_seconds: TimerState::ShortBreak.duration_seconds(),
            long_break_seconds: TimerState::LongBreak.duration_seconds(),
        }
    }

    fn for_state(&self, state: TimerState) -> u32 {
        match state {
            TimerState::Focus => self.focus_seconds,
            TimerState::ShortBreak => self.short_break_seconds,
            TimerState::LongBreak => self.long_break_seconds,
        }
    }
}

pub struct TimerEngine {
    state: TimerState,
    remaining_seconds: u32,
    completed_pomodoros: u32,
    lock_triggered: bool,
    is_running: bool,
    break_start_time: Option<SystemTime>,
    durations: TimerDurations,
}

impl TimerEngine {
    pub fn new() -> Self {
        let durations = TimerDurations::load();
        Self {
            state: TimerState::Focus,
            remaining_seconds: durations.for_state(TimerState::Focus),
            completed_pomodoros: 0,
            lock_triggered: false,
            is_running: false,
            break_start_time: None,
            durations,
        }
    }

    pub fn start(&mut self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        println!("Timer start called");
        if self.remaining_seconds == 0 {
            self.reset_to_focus();
        }
        self.is_running = true;
        println!("Timer started: running={}, remaining={}", self.is_running, self.remaining_seconds);
        // Emit update immediately so UI reflects the change without waiting for next tick
        self.emit_update(app)?;
        Ok(())
    }

    pub fn pause(&mut self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        println!("Timer pause called");
        self.is_running = false;
        // Emit update immediately
        self.emit_update(app)?;
        Ok(())
    }

    pub fn reset(&mut self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        println!("Timer reset called");
        let was_break = self.state.is_break();
        self.reset_to_focus();
        if was_break {
            app.emit("hide_guard", ())?;
        }
        // Emit update immediately
        self.emit_update(app)?;
        Ok(())
    }

    pub fn skip_break(&mut self) {
        if self.state.is_break() {
            self.transition_to_focus();
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn get_state(&self) -> TimerState {
        self.state
    }

    pub fn get_remaining_seconds(&self) -> u32 {
        self.remaining_seconds
    }

    pub fn get_completed_pomodoros(&self) -> u32 {
        self.completed_pomodoros
    }

    pub fn get_update(&self) -> TimerUpdate {
        TimerUpdate {
            state: self.state,
            remaining_seconds: self.remaining_seconds,
            completed_pomodoros: self.completed_pomodoros,
            is_running: self.is_running,
        }
    }

    pub async fn tick(&mut self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_running {
            return Ok(());
        }

        if self.remaining_seconds > 0 {
            self.remaining_seconds -= 1;

            println!("Timer tick: state={:?}, remaining={}, running={}",
                self.state, self.remaining_seconds, self.is_running);

            // Check if we need to trigger lock screen (first 5 seconds of break)
            if self.state.is_break() && !self.lock_triggered {
                if let Some(start_time) = self.break_start_time {
                    let elapsed = start_time
                        .elapsed()
                        .map_err(|e| format!("Timer error: {}", e))?;

                    if elapsed.as_secs() >= LOCK_DELAY_SECONDS as u64 {
                        self.trigger_lock_screen(app).await?;
                        self.lock_triggered = true;
                    }
                }
            }

            self.emit_update(app)?;
        } else {
            self.transition_next_state(app).await?;
        }

        Ok(())
    }

    async fn transition_next_state(&mut self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        match self.state {
            TimerState::Focus => {
                self.completed_pomodoros += 1;
                self.transition_to_break(app).await?;
            }
            TimerState::ShortBreak | TimerState::LongBreak => {
                self.transition_to_focus();
                app.emit("hide_guard", ())?;
            }
        }
        self.emit_update(app)?;
        Ok(())
    }

    async fn transition_to_break(&mut self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        // Determine if long break (every 3rd pomodoro)
        let next_break = if (self.completed_pomodoros % 3) == 0 {
            TimerState::LongBreak
        } else {
            TimerState::ShortBreak
        };

        self.state = next_break;
        self.remaining_seconds = self.durations.for_state(next_break);
        self.lock_triggered = false;
        self.break_start_time = Some(SystemTime::now());

        // Show guard window
        app.emit("show_guard", ())?;

        Ok(())
    }

    fn transition_to_focus(&mut self) {
        self.state = TimerState::Focus;
        self.remaining_seconds = self.durations.for_state(TimerState::Focus);
        self.lock_triggered = false;
        self.break_start_time = None;
    }

    fn reset_to_focus(&mut self) {
        self.state = TimerState::Focus;
        self.remaining_seconds = self.durations.for_state(TimerState::Focus);
        self.completed_pomodoros = 0;
        self.lock_triggered = false;
        self.break_start_time = None;
        self.is_running = false;
    }

    async fn trigger_lock_screen(&self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        app.emit("lock_screen", ())?;
        Ok(())
    }

    pub fn emit_update(&self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        let update = self.get_update();
        println!("Emitting update: state={:?}, remaining={}, running={}",
            update.state, update.remaining_seconds, update.is_running);
        app.emit("timer_update", update)?;
        Ok(())
    }
}

impl Default for TimerEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub type SharedTimerEngine = Arc<Mutex<TimerEngine>>;

pub async fn start_timer_loop(app: AppHandle, engine: SharedTimerEngine) {
    let mut timer = interval(Duration::from_secs(1));
    loop {
        timer.tick().await;

        let mut engine_guard = engine.lock().await;
        if let Err(e) = engine_guard.tick(&app).await {
            eprintln!("Timer tick error: {}", e);
        }
    }
}
