<template>
  <div class="xform-runtime" :class="`xform-${mode}`">
    <div
      v-for="module in visibleModules"
      :key="module.id"
      class="xform-field"
      :class="`xform-${module.type.toLowerCase()}`"
    >
      <label v-if="label(module)" :for="module.id">
        {{ label(module) }}<span v-if="module.required" class="required">*</span>
      </label>
      <textarea
        v-if="kind(module) === 'textarea'"
        :id="module.id"
        :value="stringValue(module)"
        :placeholder="placeholder(module)"
        :readonly="module.readonly"
        :disabled="module.disabled || readonly"
        @input="update(module, ($event.target as HTMLTextAreaElement).value)"
      />
      <select
        v-else-if="kind(module) === 'select'"
        :id="module.id"
        :value="stringValue(module)"
        :disabled="module.disabled || readonly"
        @change="update(module, ($event.target as HTMLSelectElement).value)"
      >
        <option value="">请选择</option>
        <option v-for="option in options(module)" :key="option.value" :value="option.value">
          {{ option.label }}
        </option>
      </select>
      <div v-else-if="kind(module) === 'checkboxes'" class="xform-options">
        <label v-for="option in options(module)" :key="option.value">
          <input
            type="checkbox"
            :checked="arrayValue(module).includes(option.value)"
            :disabled="module.disabled || readonly"
            @change="toggle(module, option.value)"
          />
          {{ option.label }}
        </label>
      </div>
      <input
        v-else-if="kind(module) !== 'display'"
        :id="module.id"
        :type="kind(module)"
        :value="stringValue(module)"
        :placeholder="placeholder(module)"
        :readonly="module.readonly"
        :disabled="module.disabled || readonly"
        @input="update(module, inputValue(module, $event))"
      />
      <div v-else class="xform-display">{{ stringValue(module) || label(module) }}</div>
      <p v-if="errors[fieldKey(module)]" class="xform-error">{{ errors[fieldKey(module)] }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { FormMode, FormValue, XformDefinition, XformModule } from '../contracts/xform'

const props = withDefaults(defineProps<{
  definition: XformDefinition
  modelValue: Record<string, FormValue>
  errors?: Record<string, string>
  mode?: FormMode
  readonly?: boolean
}>(), { errors: () => ({}), mode: 'desktop', readonly: false })

const emit = defineEmits<{ 'update:modelValue': [value: Record<string, FormValue>] }>()
const layout = computed(() => props.definition.layouts[props.mode] ?? props.definition.layouts.desktop)
const visibleModules = computed(() => {
  const ordered = layout.value?.root.map((id) => props.definition.moduleList[id]).filter(Boolean) ?? []
  return (ordered.length ? ordered : Object.values(props.definition.moduleList)).filter((module) => !module.hidden)
})

function fieldKey(module: XformModule): string {
  return String(module.name || module.key || module.id)
}

function label(module: XformModule): string {
  return String(module.label || module.name || '')
}

function placeholder(module: XformModule): string {
  return String(module.placeholder || '')
}

function options(module: XformModule): Array<{ value: string; label: string }> {
  return Array.isArray(module.options) ? module.options : []
}

function kind(module: XformModule): string {
  const type = module.type.toLowerCase()
  if (type.includes('textarea')) return 'textarea'
  if (type.includes('select')) return 'select'
  if (type.includes('checkbox') || type.includes('checkgroup')) return 'checkboxes'
  if (type.includes('date') || type.includes('calendar')) return 'date'
  if (type.includes('number') || type.includes('currency')) return 'number'
  if (type.includes('email')) return 'email'
  if (type.includes('label') || type === 'div' || type === 'html' || type === 'actionbar') return 'display'
  return 'text'
}

function stringValue(module: XformModule): string {
  const value = props.modelValue[fieldKey(module)]
  return Array.isArray(value) ? value.join(',') : value === null || value === undefined ? '' : String(value)
}

function arrayValue(module: XformModule): string[] {
  const value = props.modelValue[fieldKey(module)]
  return Array.isArray(value) ? value : []
}

function inputValue(module: XformModule, event: Event): FormValue {
  const value = (event.target as HTMLInputElement).value
  return kind(module) === 'number' && value !== '' ? Number(value) : value
}

function update(module: XformModule, value: FormValue): void {
  if (props.readonly) return
  emit('update:modelValue', { ...props.modelValue, [fieldKey(module)]: value })
}

function toggle(module: XformModule, value: string): void {
  const current = arrayValue(module)
  update(module, current.includes(value) ? current.filter((item) => item !== value) : [...current, value])
}
</script>

<style scoped>
.xform-runtime {
  display: grid;
  grid-template-columns: repeat(v-bind('layout?.columns || 1'), minmax(0, 1fr));
  gap: v-bind("`${layout?.gutter || 16}px`");
}
.xform-mobile { grid-template-columns: 1fr; }
.xform-field { display: flex; flex-direction: column; gap: 6px; min-width: 0; }
.xform-field > label { color: var(--text-secondary); font-size: 12px; }
.xform-field input, .xform-field textarea, .xform-field select {
  box-sizing: border-box; width: 100%; padding: 9px 10px; border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm); background: var(--bg-elevated); color: var(--text-primary);
}
.xform-options { display: flex; flex-wrap: wrap; gap: 12px; }
.xform-display { color: var(--text-primary); min-height: 20px; }
.required, .xform-error { color: var(--color-error); }
.xform-error { font-size: 11px; margin: 0; }
</style>
