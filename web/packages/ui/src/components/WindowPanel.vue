<template>
  <Teleport to="body">
    <div
      v-show="modelValue"
      class="window-panel"
      :style="{ left: x + 'px', top: y + 'px', zIndex: zIndex }"
      @mousedown="startDrag"
    >
      <div class="window-header" @mousedown="startDrag">
        <span class="window-title">{{ title }}</span>
        <div class="window-controls">
          <button class="win-btn" @click="$emit('minimize')" title="最小化">─</button>
          <button class="win-btn" @click="maximized = !maximized" title="最大化">□</button>
          <button class="win-btn close" @click="$emit('update:modelValue', false)" title="关闭">✕</button>
        </div>
      </div>
      <div class="window-body" :style="bodyStyle">
        <slot />
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps<{
  modelValue: boolean;
  title: string;
  x?: number;
  y?: number;
  zIndex?: number;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  minimize: [];
}>();

const maximized = ref(false);
const bodyStyle = ref<Record<string, string>>({});

// Simple drag implementation
function startDrag(e: MouseEvent): void {
  if ((e.target as HTMLElement).classList.contains('win-btn')) return;
  const startX = e.clientX - (props.x ?? 0);
  const startY = e.clientY - (props.y ?? 0);
  const onMove = (ev: MouseEvent) => {
    document.body.style.cursor = 'move';
    document.body.style.userSelect = 'none';
  };
  const onUp = () => {
    document.body.style.cursor = '';
    document.body.style.userSelect = '';
    window.removeEventListener('mousemove', onMove);
    window.removeEventListener('mouseup', onUp);
  };
  window.addEventListener('mousemove', onMove);
  window.addEventListener('mouseup', onUp);
}
</script>

<style scoped>
.window-panel {
  position: fixed;
  width: 600px;
  background: var(--bg-surface);
  border: 1px solid var(--border-active);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-panel), var(--border-glow);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
.window-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  background: var(--bg-elevated);
  border-bottom: 1px solid var(--border-subtle);
  cursor: move;
  user-select: none;
}
.window-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-primary);
  font-family: 'Orbitron', sans-serif;
}
.window-controls { display: flex; gap: 4px; }
.win-btn {
  width: 24px; height: 24px; border: none; border-radius: var(--radius-sm);
  background: var(--bg-elevated); color: var(--text-secondary); cursor: pointer;
  font-size: 12px; display: flex; align-items: center; justify-content: center;
  transition: all var(--transition-fast);
}
.win-btn:hover { background: var(--color-primary-soft); color: var(--color-primary); }
.win-btn.close:hover { background: var(--color-error-glow); color: var(--color-error); }
.window-body { flex: 1; overflow: auto; min-height: 200px; }
</style>
