<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
// uni-app 应用级生命周期组件（无渲染内容）。
</script>

<template></template>

<style>
/* 全局基础样式（uni-app）——与桌面端 packages/ui/src/theme/tokens.css 统一设计标准。
 * 分层原则：色彩令牌是本端唯一来源（浅色主题，值经 WCAG 契约测试守护）；
 * 间距/字号/圆角/阴影/动效/层级/触控等增补令牌与桌面 tokens.css 同名同值，
 * 命名对齐后两端组件可共享 var(--token) 写法，不引入运行时 JS 或新依赖。 */
page {
  /* ── 色彩令牌（权威：本块；不得改动色值，contract 测试钉死）───────────── */
  --color-bg-page: #f5f7fa;
  --color-primary: #2d8cf0;
  --color-text-primary: #263238;
  /* WCAG 2.1 AA 1.4.3：正文对比度≥4.5:1（实算对页面底/卡片白分别 secondary 4.80/5.16、muted 4.61/4.95） */
  --color-text-secondary: #646e7e;
  --color-text-muted: #67717f;
  --color-error: #c0392b;
  --color-error-bg: #fdecea;
  --color-surface: #ffffff;
  --color-border: #e3e6eb;
  --color-divider: #f0f2f5;
  --color-primary-soft: #f0f7ff;
  --color-error-tint: #fdf2f0;
  --color-border-strong: #dcdfe6;
  --color-warning: #ff9900;
  --color-success: #67c23a;
  --color-warning-bg: #fff4e5;
  --color-warning-text: #8a5200;
  --color-text-body: #5b6572;
  /* WCAG 2.1 AA 1.4.3：状态文字色与填充色分离，填充色仅用于背景/按钮。 */
  --color-success-text: #1e7e34;
  --color-placeholder: #c0c4cc;

  /* ── 增补令牌：与桌面端 tokens.css 一一对应 ─────────────────────────── */
  /* 断点（文档与媒体查询共用） */
  --breakpoint-xs: 0px;
  --breakpoint-sm: 576px;
  --breakpoint-md: 768px;
  --breakpoint-lg: 992px;
  --breakpoint-xl: 1200px;
  --breakpoint-2xl: 1536px;
  /* 间距：4px 栅格 */
  --space-0: 0;
  --space-1: 4px;
  --space-2: 8px;
  --space-3: 12px;
  --space-4: 16px;
  --space-5: 20px;
  --space-6: 24px;
  --space-8: 32px;
  --space-10: 40px;
  --space-12: 48px;
  --space-16: 64px;
  --space-card-padding: var(--space-4);
  --space-section-gap: var(--space-6);
  --space-page-gutter: var(--space-4);
  /* 字号与行高 */
  --font-size-xs: 12px;
  --font-size-sm: 13px;
  --font-size-base: 14px;
  --font-size-md: 16px;
  --font-size-lg: 18px;
  --font-size-xl: 20px;
  --font-size-2xl: 24px;
  --font-size-3xl: 30px;
  --line-height-tight: 1.25;
  --line-height-normal: 1.5;
  --line-height-relaxed: 1.75;
  --font-weight-normal: 400;
  --font-weight-medium: 500;
  --font-weight-semibold: 600;
  --font-weight-bold: 700;
  /* 圆角（移动端完整集合，不依赖桌面 dark.css） */
  --radius-none: 0;
  --radius-sm: 6px;
  --radius-md: 10px;
  --radius-lg: 16px;
  --radius-xl: 24px;
  --radius-full: 9999px;
  /* 阴影（移动端轻卡片为主，避免桌面霓虹辉光在浅色下发灰） */
  --shadow-card: 0 2px 12px rgba(45, 140, 240, 0.08);
  --shadow-card-mobile: 0 1px 4px rgba(0, 0, 0, 0.1);
  --shadow-panel: 0 8px 32px rgba(0, 0, 0, 0.12);
  --shadow-inset: inset 0 1px 0 rgba(255, 255, 255, 0.06);
  --shadow-dropdown: 0 4px 16px rgba(0, 0, 0, 0.28);
  /* 动效与缓动 */
  --transition-fast: 150ms;
  --transition-normal: 250ms;
  --transition-slow: 400ms;
  --duration-instant: 100ms;
  --duration-fast: 150ms;
  --duration-normal: 250ms;
  --duration-slow: 400ms;
  --ease-out: cubic-bezier(0.16, 1, 0.3, 1);
  --ease-in-out: cubic-bezier(0.4, 0, 0.2, 1);
  /* 层级标尺（避免各页面散写 999/9999） */
  --z-base: 0;
  --z-dropdown: 100;
  --z-sticky: 200;
  --z-modal: 1000;
  --z-toast: 1100;
  --z-skiplink: 1200;
  /* 控件尺寸与触控目标（WCAG 2.5.5 ≥44px） */
  --control-height-sm: 32px;
  --control-height-md: 40px;
  --control-height-lg: 48px;
  --touch-target-min: 44px;

  /* 全局原生控件配色跟随浅色主题；禁止系统深色覆盖导致割裂。 */
  color-scheme: light;
  background-color: var(--color-bg-page);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
}

/* ── 响应式页面边距（小程序忽略媒体查询，退回 var(--space-4)）──────────── */
@media (min-width: 576px) { page { --space-page-gutter: var(--space-5); } }
@media (min-width: 992px) { page { --space-page-gutter: var(--space-6); } }

/* ── 全局无障碍守卫（与桌面 dark.css 同口径）───────────────────────────── */
/* 键盘焦点环：仅键盘可见，不打扰鼠标/触控。 */
:focus-visible {
  outline: 2px solid var(--color-primary);
  outline-offset: 2px;
}
/* 触控目标下限：按钮与可点击列表行至少 44px（WCAG 2.5.5）。 */
button, .o2-btn, .o2-list-item {
  min-height: var(--touch-target-min);
}
/* 动效收敛：尊重系统 reduced-motion 偏好。 */
@media (prefers-reduced-motion: reduce) {
  page, page * {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}

/* ── 基础组件：统一 .o2-* 类名契约（与桌面 UnoCSS shortcut 同名）───────── */
/* R293 触控打磨：去除 WebView 默认点击高亮，按压反馈统一交给 :active 与按钮态。 */
page,
view,
button,
input,
textarea {
  -webkit-tap-highlight-color: transparent;
}

.o2-card {
  background: var(--color-surface);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-card-mobile);
  padding: var(--space-card-padding);
}
.o2-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: var(--control-height-md);
  padding: 0 var(--space-4);
  border-radius: var(--radius-md);
  background: var(--color-primary);
  color: var(--color-surface);
  font-size: var(--font-size-md);
  line-height: var(--line-height-normal);
  transition: opacity var(--transition-fast) var(--ease-out);
}
.o2-btn[disabled] { opacity: 0.6; }
.o2-btn:active:not([disabled]) {
  opacity: 0.85;
  transform: scale(0.98);
}
.o2-input {
  min-height: var(--control-height-md);
  padding: 0 var(--space-3);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  background: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--font-size-base);
}
.o2-input:focus { border-color: var(--color-primary); }
.o2-tag {
  display: inline-flex;
  align-items: center;
  padding: var(--space-1) var(--space-3);
  border-radius: var(--radius-full);
  background: var(--color-primary-soft);
  color: var(--color-primary);
  font-size: var(--font-size-xs);
}
.o2-divider {
  height: 1px;
  background: var(--color-divider);
}
/* 长列表渲染优化：仅渲染可视区布局，跳过离屏绘制。
 * contain-intrinsic-size 提供占位高度，避免滚动条跳动。 */
.o2-list-item {
  content-visibility: auto;
  contain-intrinsic-size: auto 72px;
  transition: background-color var(--transition-fast) var(--ease-out);
}
.o2-list-item:active {
  background: var(--color-primary-soft);
}
.o2-skeleton {
  border-radius: var(--radius-sm);
  background: linear-gradient(90deg, var(--color-divider) 25%, var(--color-surface) 50%, var(--color-divider) 75%);
  background-size: 200% 100%;
  animation: o2-shimmer 1.4s ease-in-out infinite;
}
@keyframes o2-shimmer {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}
/* 统一空状态：图标 + 主文案，替代单行灰字提示。 */
.o2-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-3);
  padding: var(--space-16) var(--space-4);
  text-align: center;
  color: var(--color-text-muted);
  font-size: var(--font-size-sm);
}
.o2-empty-icon {
  position: relative;
  width: 96rpx;
  height: 96rpx;
  border-radius: var(--radius-full);
  background: var(--color-primary-soft);
}
.o2-empty-icon::before {
  content: '';
  position: absolute;
  inset: 26rpx 22rpx;
  border: 3rpx solid var(--color-primary);
  border-radius: var(--radius-sm);
  opacity: 0.5;
}
.o2-empty-icon::after {
  content: '';
  position: absolute;
  top: 40rpx;
  left: 22rpx;
  right: 22rpx;
  height: 3rpx;
  background: var(--color-primary);
  opacity: 0.5;
}
.o2-empty-text {
  color: var(--color-text-muted);
}
</style>
