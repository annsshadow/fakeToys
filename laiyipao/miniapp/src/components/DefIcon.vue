<script lang="ts">
// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 物品 / 定义类图标组件（2026-11-17 全物品视觉化）。
 *
 * # 与 Icon.vue 的分工
 *
 * Icon.vue 是「界面操作」图标（返回 / 购物车 / 音量……）；
 * 本组件是「游戏内容」图标 —— 元素、装备部位、宝石、敌型、皮肤。
 * 战斗画布里的元素图形（render/canvas.ts drawElementGlyph）与本组件的
 * elem:* 同形，保证玩家在图鉴里学的语言在战场上一致。
 *
 * # 工艺约束（与 Icon.vue 相同）
 *
 * - 渲染函数产出原生 svg 节点，绕开 uni 模板编译器对非 uni 标签的不可靠处理；
 * - ⚠️ 注释里不要出现任何形如标签的字面量（SFC 解析器扫整文件文本）；
 * - 零依赖手写线性图标，stroke 继承 currentColor —— 颜色由父级 class 决定，
 *   元素色 / 稀有度色的映射留在使用方页面（各自有自己的权威表）。
 *
 * 用法：`<DefIcon name="elem:fire" :size="56" />`（size 单位 rpx）。
 */
import { h, defineComponent } from 'vue'

/** 每个图标是若干 SVG 子元素字符串（24×24 viewBox，stroke 继承外层）。 */
const DEFS: Record<string, string[]> = {
  // ── 五元素（与战斗画布 glyph 同形）──
  'elem:fire': [
    '<path d="M12 3.5c3.4 3.6 5.5 6.6 5.5 10a5.5 5.5 0 0 1-11 0c0-3.4 2.1-6.4 5.5-10z"/>',
    '<path d="M12 10.5c1.5 1.7 2.5 3.1 2.5 4.7a2.5 2.5 0 0 1-5 0c0-1.6 1-3 2.5-4.7z"/>',
  ],
  'elem:ice': [
    '<path d="M12 3v18M4.2 7.5l15.6 9M19.8 7.5l-15.6 9"/>',
    '<path d="M12 6.5l-2-1.5M12 6.5l2-1.5M12 17.5l-2 1.5M12 17.5l2 1.5"/>',
  ],
  'elem:lightning': [
    '<path d="M13 3L6 13.5h5L11 21l7-10.5h-5z"/>',
  ],
  'elem:corrosion': [
    '<path d="M12 3.5c3 4.4 5.5 7.7 5.5 11.2a5.5 5.5 0 0 1-11 0C6.5 11.2 9 7.9 12 3.5z"/>',
    '<path d="M9.5 14.5a2.5 2.7 0 0 0 2 2.6"/>',
  ],
  'elem:kinetic': [
    '<path d="M12 3l1.8 7.2L21 12l-7.2 1.8L12 21l-1.8-7.2L3 12l7.2-1.8z"/>',
  ],
  // ── 装备六部位 ──
  'slot:weapon': [
    '<path d="M4.5 19.5l.9-3.6L16.6 4.7a2.05 2.05 0 0 1 2.9 2.9L8.3 18.8z"/>',
    '<path d="M13.8 7.5l2.9 2.9"/>',
  ],
  'slot:helmet': [
    '<path d="M5 13a7 7 0 0 1 14 0v4.5h-4.2l-.8-2.8h-4l-.8 2.8H5z"/>',
    '<path d="M5 13h14"/>',
  ],
  'slot:armor': [
    '<path d="M12 3.5l3.5 1.7 3.5-.7-.8 6.9L15.5 20 12 18.2 8.5 20l-2.7-8.6L5 4.5l3.5.7z"/>',
    '<path d="M12 5.2v13"/>',
  ],
  'slot:gloves': [
    '<path d="M8 20.5V12a4 4 0 0 1 8 0v3.2h.8a1.9 1.9 0 0 1 0 3.8H16v1.5z"/>',
    '<path d="M8 17h8"/>',
  ],
  'slot:boots': [
    '<path d="M8.5 3.5H13v8.2l5.3 2.6a2.2 2.2 0 0 1 1.2 2V19H8.5z"/>',
    '<path d="M8.5 19h11"/>',
  ],
  'slot:charm': [
    '<circle cx="12" cy="14.5" r="5"/>',
    '<path d="M12 9.5V4.5M10 6h4"/>',
  ],
  // ── 宝石 ──
  gem: [
    '<path d="M7 4.5h10l4 5-9 12-9-12z"/>',
    '<path d="M3 9.5h18M12 21.5L8.5 9.5l3.5-5 3.5 5z"/>',
  ],
  // ── 敌型（与战斗画布形状一一对应）──
  'enemy:normal': [
    '<circle cx="12" cy="12" r="7"/>',
    '<circle cx="9.5" cy="11" r="0.9" fill="currentColor" stroke="none"/>',
    '<circle cx="14.5" cy="11" r="0.9" fill="currentColor" stroke="none"/>',
  ],
  'enemy:ranged': [
    '<path d="M12 4l8 8-8 8-8-8z"/>',
    '<circle cx="12" cy="12" r="1" fill="currentColor" stroke="none"/>',
  ],
  'enemy:flying': [
    '<path d="M12 4.5L19 19H5z"/>',
    '<path d="M8.5 12.5C7 11.5 6 11.5 4.5 12M15.5 12.5c1.5-1 2.5-1 4-.5"/>',
  ],
  'enemy:special': [
    '<path d="M12 4l6.9 4v8L12 20l-6.9-4V8z"/>',
    '<circle cx="12" cy="12" r="1" fill="currentColor" stroke="none"/>',
  ],
  'enemy:elite': [
    '<path d="M12 3.8l6.2 3.9.3 7.3-5.4 4.6H10.9l-5.4-4.6.3-7.3z"/>',
    '<path d="M9.5 12.5h5"/>',
  ],
  'enemy:boss': [
    '<path d="M12 6l5.7 3.4 1.6 6.3-4.4 5H9.1l-4.4-5 1.6-6.3z"/>',
    '<path d="M8 5.5l2 2 2-3 2 3 2-2"/>',
  ],
  // ── 皮肤 ──
  skin: [
    '<path d="M5 6c0-1.1.9-2 2-2h10a2 2 0 0 1 2 2v6.5c0 4-3 7.5-7 7.5s-7-3.5-7-7.5z"/>',
    '<circle cx="9" cy="11" r="1.1" fill="currentColor" stroke="none"/>',
    '<circle cx="15" cy="11" r="1.1" fill="currentColor" stroke="none"/>',
    '<path d="M9 15.5c1.8 1.4 4.2 1.4 6 0"/>',
  ],
  // ── 反应 ──
  reaction: [
    '<circle cx="9" cy="13.5" r="5.5"/>',
    '<circle cx="15" cy="10" r="4.5"/>',
  ],
  // ── 技能书 ──
  skillbook: [
    '<path d="M5 4.5h9.5a2.5 2.5 0 0 1 2.5 2.5v12.5a1 1 0 0 0-1-1H7.5A2.5 2.5 0 0 1 5 16z"/>',
    '<path d="M17 18.5a1 1 0 0 1 1 1V6.5"/>',
    '<path d="M8.5 9h5M8.5 12h5"/>',
  ],
}

export type DefIconName = keyof typeof DEFS

export default defineComponent({
  name: 'LypDefIcon',
  props: {
    /**
     * 图标名。类型放宽为 string：页面的图标名大多是 `'slot:' + item.slot`
     * 这类动态拼接，模板字符串在 vue-tsc 下过不了字面量联合类型。
     * 未知名字回退为空 svg（不炸）—— 拼错会在页面上立刻看出图标缺失。
     */
    name: { type: String, required: true },
    /** 尺寸（rpx）。默认 48。 */
    size: { type: Number, default: 48 },
  },
  setup(props) {
    // ⚠️ 内联 style 不支持 rpx（uni 只在编译期 CSS 做 rpx 换算，见 Icon.vue 同款注释）。
    const px = (rpx: number): string => {
      let base = 375
      try {
        const info = uni.getSystemInfoSync()
        base = Math.min(Number(info?.windowWidth) || 375, 560)
      } catch {
        /* 取不到就按 375 宽兜底 */
      }
      return `${Math.round(((rpx * base) / 750) * 100) / 100}px`
    }
    return () =>
      h('svg', {
        viewBox: '0 0 24 24',
        style: `width:${px(props.size)};height:${px(props.size)};display:block`,
        fill: 'none',
        stroke: 'currentColor',
        'stroke-width': '1.6',
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'aria-hidden': 'true',
        innerHTML: (DEFS[props.name] ?? []).join(''),
      })
  },
})
</script>
