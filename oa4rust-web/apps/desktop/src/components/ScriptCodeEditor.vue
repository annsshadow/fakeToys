<template>
  <div ref="editorRef" class="script-code-editor"></div>
</template>

<script setup lang="ts">
import { autocompletion, completionKeymap, type CompletionContext } from '@codemirror/autocomplete'
import { defaultKeymap } from '@codemirror/commands'
import { EditorState } from '@codemirror/state'
import { keymap } from '@codemirror/view'
import { basicSetup, EditorView } from 'codemirror'
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'

const props = withDefaults(
  defineProps<{
    modelValue: string
    readonly?: boolean
  }>(),
  { readonly: false },
)

const emit = defineEmits<(event: 'update:modelValue', value: string) => void>()
const editorRef = ref<HTMLElement | null>(null)
let view: EditorView | null = null

export const XSCRIPT_COMPLETIONS = [
  { label: 'this', type: 'keyword', detail: '当前脚本上下文' },
  { label: 'workContext', type: 'variable', detail: '流程工作上下文' },
  { label: 'effectivePerson', type: 'variable', detail: '当前用户' },
  { label: 'organization', type: 'variable', detail: '组织服务' },
  { label: 'resources', type: 'variable', detail: '资源服务' },
  { label: 'applications', type: 'variable', detail: '应用服务' },
  { label: 'console', type: 'variable', detail: '控制台' },
  { label: 'JSON', type: 'class', detail: 'JSON 工具' },
  { label: 'Promise', type: 'class', detail: '异步结果' },
  { label: 'include', type: 'function', apply: 'include()' },
  { label: 'print', type: 'function', apply: 'print()' },
  { label: 'getValue', type: 'function', apply: 'getValue()' },
  { label: 'setValue', type: 'function', apply: 'setValue()' },
  { label: 'require', type: 'function', apply: 'require()' },
]

function xscriptCompletion(context: CompletionContext) {
  return { from: word.from, options: XSCRIPT_COMPLETIONS, validFor: /^[\w$]*$/ }
}

function extensions(readonly: boolean) {
  return [
    basicSetup,
    autocompletion({ override: [xscriptCompletion] }),
    keymap.of([...defaultKeymap, ...completionKeymap]),
    EditorState.readOnly.of(readonly),
    EditorView.editable.of(!readonly),
    EditorView.lineWrapping,
    EditorView.updateListener.of((update) => {
      if (update.docChanged) emit('update:modelValue', update.state.doc.toString())
    }),
    EditorView.theme({
      '&': { height: '100%', background: 'var(--bg-terminal)', color: 'var(--text-primary)' },
      '.cm-scroller': { fontFamily: "'JetBrains Mono', 'Fira Code', monospace", fontSize: '13px' },
      '.cm-content': { caretColor: 'var(--color-primary)' },
      '.cm-activeLine': { background: 'rgba(0, 212, 255, 0.08)' },
      '.cm-gutters': {
        background: 'var(--bg-elevated)',
        color: 'var(--text-muted)',
        borderRight: '1px solid var(--border-color)',
      },
      '.cm-tooltip': { background: 'var(--bg-elevated)', border: '1px solid var(--border-color)' },
    }),
  ]
}

onMounted(() => {
  if (!editorRef.value) return
  view = new EditorView({
    parent: editorRef.value,
    state: EditorState.create({ doc: props.modelValue, extensions: extensions(props.readonly) }),
  })
})

watch(
  () => props.modelValue,
  (value) => {
    if (view && value !== view.state.doc.toString()) {
      view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: value } })
    }
  },
)

watch(
  () => props.readonly,
  (readonly) => {
    if (!view) return
    const doc = view.state.doc.toString()
    view.setState(EditorState.create({ doc, extensions: extensions(readonly) }))
  },
)

onBeforeUnmount(() => {
  view?.destroy()
  view = null
})
</script>

<style scoped>
.script-code-editor { height: 100%; min-height: 320px; overflow: hidden; border-radius: var(--radius-md); }
</style>
