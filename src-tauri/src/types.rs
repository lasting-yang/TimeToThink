use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerState {
    Focus,
    ShortBreak,
    LongBreak,
}

impl TimerState {
    pub fn duration_seconds(&self) -> u32 {
        match self {
            TimerState::Focus => 25 * 60,
            TimerState::ShortBreak => 5 * 60,
            TimerState::LongBreak => 25 * 60,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TimerState::Focus => "FOCUS",
            TimerState::ShortBreak => "SHORT BREAK",
            TimerState::LongBreak => "LONG BREAK",
        }
    }

    pub fn is_break(&self) -> bool {
        matches!(self, TimerState::ShortBreak | TimerState::LongBreak)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerUpdate {
    pub state: TimerState,
    pub remaining_seconds: u32,
    pub completed_pomodoros: u32,
    pub is_running: bool,
}
