<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import MainView from './components/MainView.vue';
import BreakGuardView from './components/BreakGuardView.vue';
import SkipConfirmDialog from './components/SkipConfirmDialog.vue';

const currentView = ref<'main' | 'guard'>('main');
const showSkipConfirm = ref(false);
let unlistenFocusChanged: (() => void) | null = null;
let unlistenShowGuard: (() => void) | null = null;
let unlistenTimerUpdate: (() => void) | null = null;

onMounted(async () => {
  // Check which window we're in
  const currentWindow = await getCurrentWindow();
  const currentLabel = currentWindow.label;

  console.log('Current window label:', currentLabel);

  if (currentLabel === 'breakguard') {
    currentView.value = 'guard';
    console.log('Showing BreakGuardView');
  } else {
    currentView.value = 'main';
    console.log('Showing MainView');

    // Menubar-like behavior: hide main window when it loses focus.
    unlistenFocusChanged = await currentWindow.onFocusChanged((event) => {
      if (!event.payload) {
        currentWindow.hide().catch((error) => {
          console.error('Failed to auto-hide main window:', error);
        });
      }
    });
  }

  // Listen for show_guard event (handled by Rust, but we can update UI state if needed)
  unlistenShowGuard = await listen('show_guard', () => {
    // Guard window will be shown by Rust
    console.log('show_guard event received');
  });

  // Listen for timer update events
  unlistenTimerUpdate = await listen('timer_update', (event: any) => {
    // Event payload contains: state, remaining_seconds, completed_pomodoros
    // This will be handled by individual components
    console.log('timer_update:', event.payload);
  });
});

onUnmounted(() => {
  if (unlistenFocusChanged) {
    unlistenFocusChanged();
  }
  if (unlistenShowGuard) {
    unlistenShowGuard();
  }
  if (unlistenTimerUpdate) {
    unlistenTimerUpdate();
  }
});

const handleSkipRequest = () => {
  showSkipConfirm.value = true;
};

const handleSkipConfirm = async () => {
  showSkipConfirm.value = false;
  try {
    await invoke('skip_break');
  } catch (error) {
    console.error('Failed to skip break:', error);
  }
};

const handleSkipCancel = () => {
  showSkipConfirm.value = false;
};
</script>

<template>
  <div v-if="currentView === 'main'">
    <MainView />
  </div>
  <div v-else-if="currentView === 'guard'">
    <BreakGuardView @skip-request="handleSkipRequest" />
  </div>

  <SkipConfirmDialog
    v-if="showSkipConfirm"
    @confirm="handleSkipConfirm"
    @cancel="handleSkipCancel"
  />
</template>
