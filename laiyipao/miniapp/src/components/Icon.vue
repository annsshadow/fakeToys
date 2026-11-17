<script lang="ts">
// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 线性 SVG 图标组件（H5 端 v2 视觉重构，2026-11-16）。
 *
 * # 为什么用渲染函数而不是模板
 *
 * uni 的模板编译器对模板里的非 uni 标签处理不可靠（svg/path 会被当成
 * 待关闭的元素，SFC 解析直接报「missing end tag」）；渲染函数直接产出
 * 原生 svg 节点，完全绕开模板编译。
 *
 * ⚠️ 因此本文件的注释里**不要出现任何形如标签的字面量**（含在反引号里也不行）——
 * SFC 解析器扫的是整文件文本，注释并不豁免。
 *
 * # 为什么不用 emoji
 *
 * emoji 在三端（iOS / 安卓各家字体 / 桌面）字形、基线、彩色程度完全不同，
 * 是「乱哄哄」观感的最直接来源；线性图标统一描边粗细与圆角端点，
 * 跟随 currentColor，可以放进任何底色。
 *
 * # 为什么不用图标库
 *
 * 本项目零 UI 依赖策略（见计划文档 §2.1）；20 个手写线性图标总量很小，
 * 没有引入依赖的必要。
 *
 * 用法：`<Icon name="target" :size="44" />`（size 单位 rpx）。
 */
import { h, defineComponent, type PropType } from 'vue'

/** 每个图标是若干 SVG 子元素字符串（24×24 viewBox，stroke 继承外层）。 */
const ICONS: Record<string, string[]> = {
  target: [
    '<circle cx="12" cy="12" r="8"/>',
    '<circle cx="12" cy="12" r="3.5"/>',
    '<circle cx="12" cy="12" r="1" fill="currentColor" stroke="none"/>',
  ],
  bag: [
    '<path d="M8 7V6a4 4 0 0 1 8 0v1"/>',
    '<rect x="4.5" y="7" width="15" height="13" rx="3"/>',
    '<path d="M9 12.5h6"/>',
  ],
  cart: [
    '<circle cx="9.5" cy="19.5" r="1.4"/>',
    '<circle cx="16.5" cy="19.5" r="1.4"/>',
    '<path d="M3 4.5h2l2.3 10.5h9.9l1.9-7H6"/>',
  ],
  mastery: [
    '<circle cx="5" cy="5.5" r="2.3"/>',
    '<circle cx="19" cy="5.5" r="2.3"/>',
    '<circle cx="12" cy="18.5" r="2.3"/>',
    '<path d="M6.3 7.5l4.5 8.8M17.7 7.5l-4.5 8.8M7.3 5.5h9.4"/>',
  ],
  calendar: [
    '<rect x="4" y="5.5" width="16" height="15" rx="2.5"/>',
    '<path d="M4 10.5h16M8.5 3.5v4M15.5 3.5v4"/>',
    '<circle cx="12" cy="15" r="1.2" fill="currentColor" stroke="none"/>',
  ],
  scroll: [
    '<rect x="5" y="4" width="14" height="16" rx="2"/>',
    '<path d="M9 9h6M9 12.5h6M9 16h4"/>',
  ],
  trophy: [
    '<path d="M7 4.5h10V9a5 5 0 0 1-10 0z"/>',
    '<path d="M7 5.5H4.5c0 2.8 1.4 4.3 3.1 4.7M17 5.5h2.5c0 2.8-1.4 4.3-3.1 4.7"/>',
    '<path d="M12 14v3M8.5 20.5h7M10 17h4v3.5h-4z"/>',
  ],
  verify: [
    '<circle cx="10.5" cy="10.5" r="6"/>',
    '<path d="M15 15l5.5 5.5"/>',
    '<path d="M8 10.5l1.8 1.8 3.2-3.2"/>',
  ],
  user: [
    '<circle cx="12" cy="8" r="3.6"/>',
    '<path d="M5 20c1.2-3.4 3.9-5 7-5s5.8 1.6 7 5"/>',
  ],
  flask: [
    '<path d="M9.5 3.5h5"/>',
    '<path d="M10.5 3.5v6L5.2 18.6A1.8 1.8 0 0 0 6.8 21.5h10.4a1.8 1.8 0 0 0 1.6-2.9L13.5 9.5v-6"/>',
    '<path d="M7.5 15h9"/>',
  ],
  ghost: [
    '<path d="M12 3.5c-3.9 0-6.5 3-6.5 7v9.5l2.2-1.8 2.1 1.8 2.2-1.8 2.2 1.8 2.1-1.8 2.2 1.8V10.5c0-4-2.6-7-6.5-7z"/>',
    '<circle cx="9.5" cy="10" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="14.5" cy="10" r="1" fill="currentColor" stroke="none"/>',
  ],
  sword: [
    '<path d="M4.5 19.5l.9-3.6L16.6 4.7a2.05 2.05 0 0 1 2.9 2.9L8.3 18.8z"/>',
    '<path d="M13.8 7.5l2.9 2.9"/>',
  ],
  shield: [
    '<path d="M12 3.5l7 2.4v6.1c0 4.4-3 7.6-7 8.9-4-1.3-7-4.5-7-8.9V5.9z"/>',
  ],
  gem: [
    '<path d="M7 4.5h10l4 5-9 12-9-12z"/>',
    '<path d="M3 9.5h18M12 21.5L8.5 9.5l3.5-5 3.5 5z"/>',
  ],
  palette: [
    '<path d="M12 3.5a8.5 8.5 0 1 0 0 17c1.3 0 1.9-.9 1.9-1.9s-.8-1.7-.8-2.6c0-1 .8-1.7 1.9-1.7h2.3a3 3 0 0 0 3-3c0-4.3-3.6-7.8-8.3-7.8z"/>',
    '<circle cx="7.6" cy="10.6" r="1.1" fill="currentColor" stroke="none"/>',
    '<circle cx="10.6" cy="7.2" r="1.1" fill="currentColor" stroke="none"/>',
    '<circle cx="15" cy="7.6" r="1.1" fill="currentColor" stroke="none"/>',
  ],
  coin: [
    '<circle cx="12" cy="12" r="8"/>',
    '<circle cx="12" cy="12" r="4.2"/>',
  ],
  gemcurrency: [
    '<path d="M7 5h10l3.5 4.5L12 20.5 3.5 9.5z"/>',
    '<path d="M3.5 9.5h17M12 20.5L9 9.5l3-4.5 3 4.5z"/>',
  ],
  bolt: [
    '<path d="M13 3L5.5 13.5h5L11 21l7.5-10.5h-5z"/>',
  ],
  lock: [
    '<rect x="5.5" y="10.5" width="13" height="9" rx="2"/>',
    '<path d="M8.5 10.5V8a3.5 3.5 0 0 1 7 0v2.5"/>',
  ],
  eye: [
    '<path d="M2.5 12s3.5-6 9.5-6 9.5 6 9.5 6-3.5 6-9.5 6-9.5-6-9.5-6z"/>',
    '<circle cx="12" cy="12" r="2.6"/>',
  ],
  'eye-off': [
    '<path d="M2.5 12s3.5-6 9.5-6 9.5 6 9.5 6-3.5 6-9.5 6-9.5-6-9.5-6z"/>',
    '<path d="M4.5 4.5l15 15"/>',
  ],
  'chevron-left': [
    '<path d="M14.5 5.5L8 12l6.5 6.5"/>',
  ],
  'chevron-right': [
    '<path d="M9.5 5.5L16 12l-6.5 6.5"/>',
  ],
  'volume-on': [
    '<path d="M4 9.5v5h3l4.5 3.5v-12L7 9.5z"/>',
    '<path d="M15.5 9.2a4 4 0 0 1 0 5.6M18 6.8a7.6 7.6 0 0 1 0 10.4"/>',
  ],
  'volume-off': [
    '<path d="M4 9.5v5h3l4.5 3.5v-12L7 9.5z"/>',
    '<path d="M16 9.8l4.6 4.6M20.6 9.8L16 14.4"/>',
  ],
  pause: [
    '<rect x="7" y="5.5" width="3.4" height="13" rx="1.2"/>',
    '<rect x="13.6" y="5.5" width="3.4" height="13" rx="1.2"/>',
  ],
  flag: [
    '<path d="M6 21V4.5"/>',
    '<path d="M6 5h10.5l-2 3.5 2 3.5H6"/>',
  ],
  star: [
    '<path d="M12 4.5l2.3 4.7 5.2.7-3.8 3.6.9 5.1L12 16.2l-4.6 2.4.9-5.1-3.8-3.6 5.2-.7z"/>',
  ],
}

export type IconName = keyof typeof ICONS

export default defineComponent({
  name: 'LypIcon',
  props: {
    name: { type: String as PropType<IconName>, required: true },
    /** 尺寸（rpx）。默认 44。 */
    size: { type: Number, default: 44 },
  },
  setup(props) {
    // ⚠️ 内联 style 不支持 rpx：uni 的 rpx→rem 转换只发生在编译期 CSS 上，
    // 运行时内联样式里的 rpx 是非法单位，会被浏览器丢弃 → svg 宽 0。
    // 这里按 uni 同一套口径手工换算（750 基准；桌面 H5 内容列限宽 560 时同步封顶）。
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
        'stroke-width': '1.8',
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'aria-hidden': 'true',
        innerHTML: (ICONS[props.name] ?? []).join(''),
      })
  },
})
</script>
