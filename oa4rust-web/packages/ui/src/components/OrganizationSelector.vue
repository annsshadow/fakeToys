<template>
  <div class="organization-selector" :aria-busy="loading">
    <div v-if="selectableTypes.length > 1" class="selector-types" aria-label="可选类型">
      <button
        v-for="type in selectableTypes"
        :key="type"
        type="button"
        class="type-button"
        :class="{ active: activeType === type }"
        @click="activeType = type"
      >
        {{ typeLabels[type] }}
      </button>
    </div>

    <form class="search-row" role="search" @submit.prevent="search">
      <input
        v-model="keyword"
        class="search-input"
        type="search"
        :placeholder="placeholder || `搜索${typeLabels[activeType]}`"
        :disabled="disabled"
        aria-label="搜索组织、人员或身份"
      />
      <button class="search-button" type="submit" :disabled="disabled || loading">
        {{ loading ? '搜索中…' : '搜索' }}
      </button>
    </form>

    <p v-if="errorMessage" class="selector-status error" role="alert">{{ errorMessage }}</p>
    <p v-else-if="hasSearched && !loading && results.length === 0" class="selector-status">
      没有匹配的{{ typeLabels[activeType] }}
    </p>

    <ul v-if="results.length > 0" class="result-list" aria-label="搜索结果">
      <li v-for="item in results" :key="itemKey(item)">
        <button
          type="button"
          class="result-item"
          :class="{ selected: isSelected(item) }"
          :disabled="disabled"
          @click="toggle(item)"
        >
          <span class="type-badge">{{ typeLabels[item.type] }}</span>
          <span class="result-main">
            <strong>{{ item.name }}</strong>
            <small>{{ itemDetail(item) }}</small>
          </span>
          <span class="selection-mark" aria-hidden="true">{{ isSelected(item) ? '✓' : '+' }}</span>
        </button>
      </li>
    </ul>

    <div class="selected-section">
      <div class="selected-heading">
        <span>已选择</span>
        <span>{{ modelValue.length }}{{ mode === 'multiple' && max ? ` / ${max}` : '' }}</span>
      </div>
      <p v-if="modelValue.length === 0" class="selector-status">尚未选择</p>
      <ul v-else class="selected-list" aria-label="已选择项目">
        <li v-for="item in modelValue" :key="itemKey(item)" class="selected-item">
          <span class="type-badge">{{ typeLabels[item.type] }}</span>
          <span class="selected-name">{{ item.name }}</span>
          <button
            type="button"
            class="remove-button"
            :disabled="disabled"
            :aria-label="`移除${item.name}`"
            @click="remove(item)"
          >
            ×
          </button>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import {
  ORGANIZATION_SELECTOR_TYPES,
  type OrganizationSelectorItem,
  type OrganizationSelectorMode,
  type OrganizationSelectorSearch,
  type OrganizationSelectorType,
  searchOrganizationSelector,
} from './organization-selector'

const props = withDefaults(
  defineProps<{
    modelValue: OrganizationSelectorItem[]
    types?: OrganizationSelectorType[]
    mode?: OrganizationSelectorMode
    placeholder?: string
    disabled?: boolean
    max?: number
    searcher?: OrganizationSelectorSearch
  }>(),
  {
    types: () => [...ORGANIZATION_SELECTOR_TYPES],
    mode: 'multiple',
    placeholder: '',
    disabled: false,
    max: undefined,
    searcher: searchOrganizationSelector,
  },
)

const emit = defineEmits<{
  'update:modelValue': [value: OrganizationSelectorItem[]]
  change: [value: OrganizationSelectorItem[]]
  error: [error: unknown]
}>()

// biome-ignore lint/correctness/noUnusedVariables: Vue template consumes this binding.
const typeLabels: Record<OrganizationSelectorType, string> = {
  unit: '组织',
  person: '人员',
  identity: '身份',
}
const selectableTypes = computed<OrganizationSelectorType[]>(() => {
  const unique = ORGANIZATION_SELECTOR_TYPES.filter((type) => props.types.includes(type))
  return unique.length > 0 ? unique : [...ORGANIZATION_SELECTOR_TYPES]
})
const activeType = ref<OrganizationSelectorType>(selectableTypes.value[0])
const keyword = ref('')
const loading = ref(false)
const hasSearched = ref(false)
const errorMessage = ref('')
const results = ref<OrganizationSelectorItem[]>([])
let searchSequence = 0

watch(selectableTypes, (types) => {
  if (!types.includes(activeType.value)) activeType.value = types[0]
})

watch(activeType, () => {
  results.value = []
  hasSearched.value = false
  errorMessage.value = ''
})

function itemKey(item: OrganizationSelectorItem): string {
  return `${item.type}:${item.id}`
}

function isSelected(item: OrganizationSelectorItem): boolean {
  const key = itemKey(item)
  return props.modelValue.some((selected) => itemKey(selected) === key)
}

function updateModel(value: OrganizationSelectorItem[]): void {
  emit('update:modelValue', value)
  emit('change', value)
}

// biome-ignore lint/correctness/noUnusedVariables: Vue template consumes this handler.
function toggle(item: OrganizationSelectorItem): void {
  if (isSelected(item)) {
    remove(item)
    return
  }
  if (props.mode === 'single') {
    updateModel([item])
    return
  }
  if (props.max && props.modelValue.length >= props.max) return
  updateModel([...props.modelValue, item])
}

function remove(item: OrganizationSelectorItem): void {
  const key = itemKey(item)
  updateModel(props.modelValue.filter((selected) => itemKey(selected) !== key))
}

// biome-ignore lint/correctness/noUnusedVariables: Vue template consumes this formatter.
function itemDetail(item: OrganizationSelectorItem): string {
  if (item.type === 'person') return item.mobile || item.email || item.unitId || item.id
  if (item.type === 'identity') return item.unitId || item.personId || item.id
  return item.parentId || item.id
}

async function search(): Promise<void> {
  const sequence = ++searchSequence
  loading.value = true
  hasSearched.value = true
  errorMessage.value = ''
  try {
    const found = await props.searcher(activeType.value, keyword.value.trim())
    if (sequence === searchSequence) results.value = found
  } catch (error) {
    if (sequence !== searchSequence) return
    results.value = []
    errorMessage.value = error instanceof Error ? error.message : '搜索失败，请稍后重试'
    emit('error', error)
  } finally {
    if (sequence === searchSequence) loading.value = false
  }
}

defineExpose({ search })
</script>

<style scoped>
.organization-selector {
  display: flex;
  flex-direction: column;
  gap: 12px;
  color: var(--text-primary, #e5e7eb);
}
.selector-types,
.search-row {
  display: flex;
  gap: 8px;
}
.type-button,
.search-button,
.result-item,
.remove-button {
  border: 1px solid var(--border-color, #374151);
  color: inherit;
  cursor: pointer;
}
.type-button {
  padding: 7px 14px;
  border-radius: var(--radius-md, 8px);
  background: var(--bg-elevated, #1f2937);
}
.type-button.active {
  border-color: var(--color-primary, #3b82f6);
  color: var(--color-primary, #60a5fa);
  background: var(--color-primary-soft, rgb(59 130 246 / 15%));
}
.search-input {
  min-width: 0;
  flex: 1;
  padding: 9px 12px;
  border: 1px solid var(--border-color, #374151);
  border-radius: var(--radius-md, 8px);
  outline: none;
  background: var(--bg-elevated, #111827);
  color: inherit;
}
.search-input:focus {
  border-color: var(--color-primary, #3b82f6);
}
.search-button {
  padding: 9px 18px;
  border: 0;
  border-radius: var(--radius-md, 8px);
  background: var(--color-primary, #2563eb);
  color: white;
  font-weight: 600;
}
.search-button:disabled,
.result-item:disabled,
.remove-button:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}
.result-list,
.selected-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin: 0;
  padding: 0;
  list-style: none;
}
.result-list {
  max-height: 320px;
  overflow: auto;
}
.result-item {
  display: flex;
  width: 100%;
  align-items: center;
  gap: 10px;
  padding: 10px;
  border-radius: var(--radius-md, 8px);
  background: var(--bg-elevated, #111827);
  text-align: left;
}
.result-item:hover,
.result-item.selected {
  border-color: var(--color-primary, #3b82f6);
  background: var(--bg-hover, #1f2937);
}
.type-badge {
  flex: none;
  padding: 2px 7px;
  border-radius: 999px;
  background: var(--color-primary-soft, rgb(59 130 246 / 15%));
  color: var(--color-primary, #60a5fa);
  font-size: 11px;
}
.result-main {
  display: flex;
  min-width: 0;
  flex: 1;
  flex-direction: column;
  gap: 3px;
}
.result-main strong,
.result-main small {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.result-main small,
.selector-status,
.selected-heading {
  color: var(--text-muted, #9ca3af);
  font-size: 12px;
}
.selection-mark {
  color: var(--color-primary, #60a5fa);
  font-weight: 700;
}
.selector-status {
  margin: 0;
  padding: 12px;
  text-align: center;
}
.selector-status.error {
  color: var(--color-danger, #ef4444);
}
.selected-section {
  padding-top: 12px;
  border-top: 1px solid var(--border-color, #374151);
}
.selected-heading,
.selected-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.selected-heading {
  margin-bottom: 8px;
  font-weight: 600;
}
.selected-item {
  gap: 8px;
  padding: 7px 10px;
  border-radius: var(--radius-md, 8px);
  background: var(--bg-elevated, #111827);
}
.selected-name {
  min-width: 0;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.remove-button {
  padding: 0 4px;
  border: 0;
  background: transparent;
  color: var(--text-muted, #9ca3af);
  font-size: 18px;
}
.remove-button:hover {
  color: var(--color-danger, #ef4444);
}
</style>

