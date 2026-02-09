<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface TimerUpdate {
  state: string;  // 'Focus', 'ShortBreak', 'LongBreak' (now matches Rust enum)
  remaining_seconds: number;
  completed_pomodoros: number;
  is_running: boolean;
}

const state = ref<string>('Focus');
const remainingSeconds = ref<number>(25 * 60);
const completedPomodoros = ref<number>(0);
const isRunning = ref<boolean>(false);
let unlistenTimerUpdate: (() => void) | null = null;

const formattedTime = computed(() => {
  const minutes = Math.floor(remainingSeconds.value / 60);
  const seconds = remainingSeconds.value % 60;
  return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
});

const stateDisplay = computed(() => {
  switch (state.value) {
    case 'Focus':
      return 'FOCUS';
    case 'ShortBreak':
      return 'SHORT BREAK';
    case 'LongBreak':
      return 'LONG BREAK';
    default:
      return 'FOCUS';
  }
});

const stateColor = computed(() => {
  switch (state.value) {
    case 'Focus':
      return '#0071e3';
    case 'ShortBreak':
      return '#34c759';
    case 'LongBreak':
      return '#0a84ff';
    default:
      return '#0071e3';
  }
});

onMounted(async () => {
  console.log('MainView mounted');

  // Get initial state
  try {
    const initialState = await invoke<TimerUpdate>('get_timer_state');
    console.log('Initial state:', initialState);
    console.log('Initial state type:', typeof initialState);
    console.log('Initial state.state:', initialState.state);
    console.log('Initial state.state type:', typeof initialState.state);
    state.value = initialState.state;
    remainingSeconds.value = initialState.remaining_seconds;
    completedPomodoros.value = initialState.completed_pomodoros;
    isRunning.value = initialState.is_running;
  } catch (error) {
    console.error('Failed to get initial state:', error);
  }

  // Listen for timer updates
  try {
    unlistenTimerUpdate = await listen<TimerUpdate>('timer_update', (event) => {
      console.log('=== Timer update received ===');
      console.log('Event:', event);
      console.log('Payload:', event.payload);
      console.log('Payload.state:', event.payload.state);
      console.log('Payload.state type:', typeof event.payload.state);
      console.log('Payload.remaining_seconds:', event.payload.remaining_seconds);
      console.log('Payload.is_running:', event.payload.is_running);

      state.value = event.payload.state;
      remainingSeconds.value = event.payload.remaining_seconds;
      completedPomodoros.value = event.payload.completed_pomodoros;
      isRunning.value = event.payload.is_running;
    });
    console.log('Timer update listener registered successfully');
  } catch (error) {
    console.error('Failed to register timer update listener:', error);
  }
});

onUnmounted(() => {
  if (unlistenTimerUpdate) {
    unlistenTimerUpdate();
  }
});

const handleStart = async () => {
  console.log('Start button clicked');
  try {
    await invoke('start_timer');
    console.log('Timer started');
    isRunning.value = true;
  } catch (error) {
    console.error('Failed to start timer:', error);
  }
};

const handlePause = async () => {
  console.log('Pause button clicked');
  try {
    await invoke('pause_timer');
    console.log('Timer paused');
    isRunning.value = false;
  } catch (error) {
    console.error('Failed to pause timer:', error);
  }
};

const handleReset = async () => {
  console.log('Reset button clicked');
  try {
    await invoke('reset_timer');
    console.log('Timer reset');
    isRunning.value = false;
  } catch (error) {
    console.error('Failed to reset timer:', error);
  }
};
</script>

<template>
  <div class="main-view">
    <div class="background-aura"></div>
    <div class="container">
      <div class="status-header">
        <div class="state-badge" :style="{ backgroundColor: stateColor }">
          {{ stateDisplay }}
        </div>
        <p class="session-count">{{ completedPomodoros }} completed</p>
      </div>

      <div class="timer-panel" :class="{ running: isRunning }">
        <div class="timer-display">
          {{ formattedTime }}
        </div>
      </div>

      <div class="controls">
        <button
          v-if="!isRunning"
          @click="handleStart"
          class="btn btn-primary"
        >
          Start
        </button>
        <button
          v-else
          @click="handlePause"
          class="btn btn-primary"
        >
          Pause
        </button>
        <button
          @click="handleReset"
          class="btn btn-tertiary"
        >
          Reset
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.main-view {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 10px;
  overflow: hidden;
}

.background-aura {
  position: absolute;
  inset: 0;
  background:
    radial-gradient(circle at 16% 12%, rgba(0, 113, 227, 0.24), transparent 36%),
    radial-gradient(circle at 84% 100%, rgba(100, 210, 255, 0.2), transparent 42%);
  pointer-events: none;
}

.container {
  position: relative;
  width: min(420px, 100%);
  max-height: 100%;
  overflow: auto;
  padding: 16px 14px 14px;
  text-align: center;
  border-radius: 18px;
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.82) 0%, rgba(255, 255, 255, 0.74) 100%);
  border: 1px solid var(--border-soft);
  backdrop-filter: blur(16px);
  box-shadow: var(--shadow-soft);
}

.status-header {
  display: grid;
  gap: 5px;
  justify-items: center;
}

.state-badge {
  display: inline-block;
  padding: 8px 16px;
  border-radius: 999px;
  color: white;
  font-weight: 600;
  font-size: 13px;
  letter-spacing: 0.08em;
  box-shadow: 0 6px 14px rgba(15, 23, 42, 0.16);
}

.session-count {
  margin: 0;
  font-size: 11px;
  color: var(--text-secondary);
}

.timer-panel {
  margin: 10px 0 10px;
  padding: 12px 8px;
  border-radius: 14px;
  border: 1px solid rgba(15, 23, 42, 0.08);
  background:
    linear-gradient(180deg, rgba(255, 255, 255, 0.72) 0%, rgba(247, 250, 255, 0.68) 100%);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.6);
  transition: box-shadow 0.2s ease, border-color 0.2s ease;
}

.timer-panel.running {
  border-color: rgba(0, 113, 227, 0.2);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.72),
    0 10px 24px rgba(0, 113, 227, 0.14);
}

.timer-display {
  margin: 0;
  font-size: clamp(52px, 15vw, 82px);
  line-height: 0.95;
  letter-spacing: -0.05em;
  font-weight: 700;
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}

.controls {
  display: flex;
  gap: 8px;
  flex-wrap: nowrap;
  justify-content: center;
}

.btn {
  min-width: 0;
  flex: 1;
  padding: 9px 12px;
  border-radius: 999px;
  border: 1px solid transparent;
  font-size: 13px;
  font-weight: 600;
  font-family: inherit;
  cursor: pointer;
  transition: transform 0.15s ease, box-shadow 0.2s ease, background-color 0.2s ease, border-color 0.2s ease;
}

.btn:active {
  transform: scale(0.97);
}

.btn-primary {
  background: var(--accent);
  color: white;
  box-shadow: 0 8px 20px rgba(0, 113, 227, 0.32);
}

.btn-primary:hover {
  background: var(--accent-pressed);
  transform: translateY(-1px);
}

.btn-tertiary {
  background: transparent;
  color: var(--text-secondary);
  border-color: var(--border-soft);
}

.btn-tertiary:hover {
  color: var(--danger);
  border-color: rgba(255, 59, 48, 0.3);
  background: rgba(255, 59, 48, 0.08);
}

@media (max-width: 420px) {
  .container {
    border-radius: 14px;
    padding: 12px 10px 10px;
  }
}
</style>
