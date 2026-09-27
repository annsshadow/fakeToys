import { describe, expect, it } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import { defineComponent, ref } from 'vue'

// 基座护栏：证明 wrapper.vm 能访问 <script setup> 的内部状态（后续 admin 测试都按这个写法）。
// ⚠️ `wrapper.nextTick()` 是 VTU **v1** 的 API，v2 上不存在 ⇒ `TypeError: w.nextTick is not a function`；
//    本仓库 miniapp 侧一律用 `flushPromises()`（见 `src/App.test.ts`），这里跟着同一写法。
const Probe = defineComponent({
  setup() {
    const count = ref(1)
    function inc() {
      count.value = 2
    }
    function guarded(x: number | null): number {
      return x ?? 3
    }
    return { count, inc, guarded }
  },
  template: '<div>{{ count }}</div>',
})

describe('sanity', () => {
  it('vm exposes script setup state', async () => {
    const w = mount(Probe)
    expect(w.text()).toBe('1')
    const vm = w.vm as unknown as { count: number; inc: () => void; guarded: (x: number | null) => number }
    vm.inc()
    await flushPromises()
    expect(w.text()).toBe('2')
    expect(vm.count).toBe(2)
    expect(vm.guarded(null)).toBe(3)
    expect(vm.guarded(5)).toBe(5)
  })
})
