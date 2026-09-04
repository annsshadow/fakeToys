<template>
  <div class="cm-wrapper" ref="wrapperRef">
    <div ref="editorRef" class="cm-editor"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch, computed } from 'vue'
import { EditorState } from '@codemirror/state'
import { EditorView, basicSetup } from 'codemirror'
import { sql } from '@codemirror/lang-sql'
import { syntaxHighlighting, highlightSelectionMatches, keymap } from '@codemirror/view'
import { defaultKeymap } from '@codemirror/commands'

const props = defineProps<{
  modelValue: string
  readonly?: boolean
}>()
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const editorRef = ref<HTMLElement | null>(null)
const wrapperRef = ref<HTMLElement | null>(null)
let view: EditorView | null = null

const theme = computed(() => ({
  // Dark theme for CodeMirror
  '.cm-editor': { background: 'var(--bg-terminal)', color: '#7fdbca', height: '100%' },
  'cm-scroller': { fontFamily: "'JetBrains Mono', 'Fira Code', monospace", fontSize: '13px' },
  '.cm-content': { caretColor: '#7fdbca' },
  '.cm-cursor, .cm-dropCursor': { borderLeftColor: '#7fdbca' },
  '.cm-activeLine': { background: 'rgba(0,212,255,0.08)' },
  '.cm-selectionMatch': { background: 'rgba(0,212,255,0.2)' },
  '.cm-gutters': { background: 'var(--bg-elevated)', color: 'var(--text-muted)', borderRight: '1px solid var(--border-color)' },
}))

onMounted(() => {
  if (!editorRef.value) return
  const startState = EditorState.create({
    doc: props.modelValue,
    extensions: [
      basicSetup,
      sql(),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          emit('update:modelValue', update.state.doc.toString())
        }
      }),
      EditorView.theme(theme.value as any),
      syntaxHighlighting(),
      highlightSelectionMatches(),
      keymap.of(defaultKeymap),
    ],
  })
  view = new EditorView({ state: startState, parent: editorRef.value })
})

watch(() => props.modelValue, (newVal) => {
  if (view && newVal !== view.state.doc.toString()) {
    view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: newVal } })
  }
})

onBeforeUnmount(() => {
  view?.destroy()
  view = null
})
</script>

<style scoped>
.cm-wrapper { position: relative; width: 100%; height: 100%; border-radius: var(--radius-md); overflow: hidden; }
.cm-editor { height: 100%; min-height: 200px; }
</style>
