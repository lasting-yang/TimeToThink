<script setup lang="ts">
const emit = defineEmits<{
  confirm: [];
  cancel: [];
}>();

const handleConfirm = () => {
  emit('confirm');
};

const handleCancel = () => {
  emit('cancel');
};

const handleBackdropClick = (event: MouseEvent) => {
  if (event.target === event.currentTarget) {
    emit('cancel');
  }
};
</script>

<template>
  <div class="dialog-overlay" @click="handleBackdropClick">
    <div class="dialog-content">
      <div class="dialog-icon">!</div>
      <h2 class="dialog-title">Skip Break?</h2>
      <p class="dialog-message">
        You still have time remaining in your break. Are you sure you want to skip it?
      </p>
      <p class="dialog-warning">
        Taking regular breaks is important for your health and productivity.
      </p>

      <div class="dialog-actions">
        <button @click="handleCancel" class="btn btn-cancel" autofocus>
          Continue Break
        </button>
        <button @click="handleConfirm" class="btn btn-confirm">
          Yes, Skip Break
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(15, 23, 42, 0.34);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  z-index: 9999;
}

.dialog-content {
  width: min(520px, 100%);
  background: var(--surface);
  border: 1px solid var(--border-soft);
  border-radius: 28px;
  padding: 30px 28px 24px;
  box-shadow: var(--shadow-card);
  backdrop-filter: blur(20px);
  text-align: center;
}

.dialog-icon {
  margin: 0 auto 16px;
  width: 56px;
  height: 56px;
  border-radius: 999px;
  display: grid;
  place-items: center;
  font-size: 30px;
  line-height: 1;
  font-weight: 600;
  color: var(--danger);
  background: rgba(255, 59, 48, 0.1);
}

.dialog-title {
  font-size: 30px;
  letter-spacing: -0.02em;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 12px;
}

.dialog-message {
  font-size: 16px;
  color: var(--text-secondary);
  margin-bottom: 12px;
  line-height: 1.5;
}

.dialog-warning {
  font-size: 13px;
  color: var(--danger);
  font-weight: 500;
  margin-bottom: 24px;
}

.dialog-actions {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
  justify-content: center;
}

.btn {
  min-width: 150px;
  padding: 12px 18px;
  border-radius: 999px;
  border: 1px solid transparent;
  font-size: 15px;
  font-weight: 600;
  font-family: inherit;
  cursor: pointer;
  transition: transform 0.15s ease, box-shadow 0.2s ease, background-color 0.2s ease;
}

.btn:active {
  transform: scale(0.97);
}

.btn-cancel {
  background: var(--accent);
  color: white;
  box-shadow: 0 10px 22px rgba(0, 113, 227, 0.3);
}

.btn-cancel:hover {
  background: var(--accent-pressed);
  transform: translateY(-1px);
}

.btn-confirm {
  background: transparent;
  color: var(--danger);
  border-color: rgba(255, 59, 48, 0.25);
}

.btn-confirm:hover {
  background: rgba(255, 59, 48, 0.1);
  transform: translateY(-1px);
}

@media (max-width: 520px) {
  .dialog-content {
    border-radius: 22px;
    padding: 24px 18px 18px;
  }

  .dialog-actions {
    flex-direction: column;
  }

  .btn {
    width: 100%;
  }
}
</style>
