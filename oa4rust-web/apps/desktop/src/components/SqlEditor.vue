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
import { autocompletion, completionKeymap } from '@codemirror/autocomplete'

const props = defineProps<{
  modelValue: string
  readonly?: boolean
  tables?: string[]          // Available table names for autocomplete
  columns?: Record<string, string[]>  // Table -> columns mapping
}>()
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const editorRef = ref<HTMLElement | null>(null)
const wrapperRef = ref<HTMLElement | null>(null)
let view: EditorView | null = null

// SQL keywords for autocomplete
const SQL_KEYWORDS = [
  'SELECT', 'FROM', 'WHERE', 'AND', 'OR', 'NOT', 'IN', 'EXISTS',
  'INSERT', 'INTO', 'VALUES', 'UPDATE', 'SET', 'DELETE',
  'CREATE', 'TABLE', 'ALTER', 'DROP', 'INDEX',
  'JOIN', 'LEFT', 'RIGHT', 'INNER', 'OUTER', 'ON',
  'GROUP', 'BY', 'HAVING', 'ORDER', 'ASC', 'DESC',
  'LIMIT', 'OFFSET', 'UNION', 'ALL',
  'AS', 'IS', 'NULL', 'TRUE', 'FALSE',
  'COUNT', 'SUM', 'AVG', 'MAX', 'MIN',
  'CASE', 'WHEN', 'THEN', 'ELSE', 'END',
  'DISTINCT', 'TOP', 'LIKE', 'BETWEEN',
]

// Custom completion source
function createCompletionSource() {
  return (context: any) => {
    const word = context.matchBefore(/\w+/)
    if (!word) return null

    const text = context.state.doc.toString().toLowerCase()
    const cursorPos = context.pos

    // Find what's before cursor to determine context
    const beforeCursor = text.slice(0, cursorPos)

    // Keywords
    const keywordMatches = SQL_KEYWORDS.filter(k =>
      k.toLowerCase().startsWith(word.text.toLowerCase()) && k.toLowerCase() !== word.text.toLowerCase()
    )

    const completions = keywordMatches.map(k => ({
      label: k,
      type: 'keyword' as const,
      apply: k,
      detail: 'SQL关键字'
    }))

    // Table names
    if (props.tables) {
      const tableMatches = props.tables.filter(t =>
        t.toLowerCase().startsWith(word.text.toLowerCase()) && t.toLowerCase() !== word.text.toLowerCase()
      )
      completions.push(...tableMatches.map(t => ({
        label: t,
        type: 'class' as const,
        apply: t,
        detail: '表名'
      })))

      // Column suggestions when typing after dot
      if (word.text.endsWith('.')) {
        const tableName = word.text.slice(0, -1)
        if (props.columns && props.columns[tableName]) {
          completions.push(...props.columns[tableName].map(c => ({
            label: c,
            type: 'property' as const,
            apply: c,
            detail: `字段 (${tableName})`
          })))
        }
      }
    }

    return {
      from: word.from,
      options: completions,
      validFor: /^\w*$/
    }
  }
}

const theme = computed(() => ({
  '.cm-editor': { background: 'var(--bg-terminal)', color: '#7fdbca', height: '100%' },
  'cm-scroller': { fontFamily: "'JetBrains Mono', 'Fira Code', monospace", fontSize: '13px' },
  '.cm-content': { caretColor: '#7fdbca' },
  '.cm-cursor, .cm-dropCursor': { borderLeftColor: '#7fdbca' },
  '.cm-activeLine': { background: 'rgba(0,212,255,0.08)' },
  '.cm-selectionMatch': { background: 'rgba(0,212,255,0.2)' },
  '.cm-gutters': { background: 'var(--bg-elevated)', color: 'var(--text-muted)', borderRight: '1px solid var(--border-color)' },
  '.cm-tooltip': { background: 'var(--bg-elevated)', border: '1px solid var(--border-color)' },
  '.cm-tooltip-autocomplete': { minWidth: '200px' },
  '.cm-completionInfo': { background: 'var(--bg-surface)' },
}))

onMounted(() => {
  if (!editorRef.value) return
  const startState = EditorState.create({
    doc: props.modelValue,
    extensions: [
      basicSetup,
      sql(),
      autocompletion({ override: [createCompletionSource()] }),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          emit('update:modelValue', update.state.doc.toString())
        }
      }),
      EditorView.theme(theme.value as any),
      syntaxHighlighting(),
      highlightSelectionMatches(),
      keymap.of([...defaultKeymap, ...completionKeymap]),
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
