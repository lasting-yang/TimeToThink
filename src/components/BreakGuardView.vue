<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed } from 'vue';
import { listen } from '@tauri-apps/api/event';

interface TimerUpdate {
  state: string;
  remaining_seconds: number;
  completed_pomodoros: number;
}

const remainingSeconds = ref<number>(5 * 60);
const state = ref<string>('ShortBreak');
let unlistenTimerUpdate: (() => void) | null = null;

const formattedTime = computed(() => {
  const minutes = Math.floor(remainingSeconds.value / 60);
  const seconds = remainingSeconds.value % 60;
  return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
});

const breakTitle = computed(() => {
  return state.value === 'LongBreak' ? 'Long Break Time' : 'Break Time';
});

const breakMessage = computed(() => {
  return state.value === 'LongBreak'
    ? 'You\'ve earned a longer rest. Take your time!'
    : 'Time to rest your eyes and stretch.';
});

const progressPercent = computed(() => {
  const total = state.value === 'LongBreak' ? 1500 : 300;
  return `${Math.max(0, Math.min(100, (remainingSeconds.value / total) * 100))}%`;
});

const emit = defineEmits<{
  skipRequest: [];
}>();

onMounted(async () => {
  // Listen for timer updates
  unlistenTimerUpdate = await listen<TimerUpdate>('timer_update', (event) => {
    remainingSeconds.value = event.payload.remaining_seconds;
    state.value = event.payload.state;
  });
});

onUnmounted(() => {
  if (unlistenTimerUpdate) {
    unlistenTimerUpdate();
  }
});

const handleSkipBreak = () => {
  emit('skipRequest');
};

const handleContinueBreak = () => {
  // Just ensure the window stays focused
  // The polling mechanism will handle this automatically
};
</script>

<template>
  <div class="break-guard">
    <div class="background-aura"></div>
    <div class="content">
      <div class="break-icon">⏸</div>
      <h1 class="break-title">{{ breakTitle }}</h1>
      <p class="break-message">{{ breakMessage }}</p>

      <div class="timer-display">
        {{ formattedTime }}
      </div>

      <div class="progress-bar">
        <div
          class="progress-fill"
          :style="{ width: progressPercent }"
        ></div>
      </div>

      <div class="actions">
        <button @click="handleContinueBreak" class="btn btn-primary">
          Continue Break
        </button>
        <button @click="handleSkipBreak" class="btn btn-secondary">
          Skip Break
        </button>
      </div>

      <p class="hint">
        Taking regular breaks improves focus and productivity
      </p>
    </div>
  </div>
</template>

<style scoped>
.break-guard {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  padding: 0;
}

.background-aura {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background:
    radial-gradient(circle at 18% 8%, rgba(0, 113, 227, 0.22), transparent 36%),
    radial-gradient(circle at 82% 100%, rgba(100, 210, 255, 0.24), transparent 42%),
    linear-gradient(180deg, #f9fbff 0%, #eef3fb 100%);
  z-index: -1;
}

.content {
  width: 100%;
  height: 100%;
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: clamp(24px, 5vw, 56px);
}

.break-icon {
  margin: 0 auto 16px;
  width: 60px;
  height: 60px;
  border-radius: 999px;
  display: grid;
  place-items: center;
  font-size: 30px;
  color: var(--accent);
  background: rgba(0, 113, 227, 0.1);
}

.break-title {
  font-size: clamp(32px, 7vw, 54px);
  line-height: 1.02;
  letter-spacing: -0.03em;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 10px;
}

.break-message {
  margin: 0 auto 26px;
  max-width: 480px;
  font-size: 18px;
  color: var(--text-secondary);
}

.timer-display {
  font-size: clamp(72px, 16vw, 120px);
  line-height: 1;
  letter-spacing: -0.05em;
  font-weight: 700;
  color: var(--text-primary);
  margin: 12px 0 28px;
  font-variant-numeric: tabular-nums;
}

.progress-bar {
  max-width: 560px;
  width: 100%;
  margin: 0 auto 30px;
  border-radius: 999px;
  border: 1px solid rgba(255, 255, 255, 0.45);
  height: 8px;
  background: rgba(15, 23, 42, 0.08);
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #64d2ff 0%, var(--accent) 75%);
  transition: width 1s linear;
}

.actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
  justify-content: center;
  margin-bottom: 22px;
}

.btn {
  min-width: 170px;
  padding: 13px 22px;
  border-radius: 999px;
  border: 1px solid transparent;
  font-size: 15px;
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
  box-shadow: 0 10px 22px rgba(0, 113, 227, 0.3);
}

.btn-primary:hover {
  background: var(--accent-pressed);
  transform: translateY(-1px);
}

.btn-secondary {
  background: var(--surface-strong);
  color: var(--text-secondary);
  border-color: var(--border-soft);
  box-shadow: var(--shadow-soft);
}

.btn-secondary:hover {
  background: #ffffff;
  color: var(--text-primary);
  transform: translateY(-1px);
}

.hint {
  font-size: 14px;
  color: var(--text-tertiary);
}

@media (max-width: 600px) {
  .content {
    padding: 24px 16px;
  }

  .actions {
    flex-direction: column;
  }

  .btn {
    width: 100%;
  }
}
</style>
