// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { defineConfig } from 'unocss'

export default defineConfig({
  theme: {
    colors: {
      primary: 'var(--color-primary)',
      accent: 'var(--color-accent)',
      success: 'var(--color-success)',
      warning: 'var(--color-warning)',
      error: 'var(--color-error)',
      info: 'var(--color-info)',
      base: 'var(--bg-base)',
      surface: 'var(--bg-surface)',
      elevated: 'var(--bg-elevated)',
      glass: 'var(--bg-glass)',
      text: {
        primary: 'var(--text-primary)',
        secondary: 'var(--text-secondary)',
        muted: 'var(--text-muted)',
      },
    },
    fontFamily: {
      sans: ['Inter', 'HarmonyOS Sans SC', 'PingFang SC', 'Microsoft YaHei', 'system-ui', 'sans-serif'],
      mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
      display: ['Orbitron', 'Rajdhani', 'sans-serif'],
    },
    borderRadius: {
      none: 'var(--radius-none)',
      sm: 'var(--radius-sm)',
      md: 'var(--radius-md)',
      lg: 'var(--radius-lg)',
      xl: 'var(--radius-xl)',
      full: 'var(--radius-full)',
    },
    spacing: {
      xs: 'var(--space-1)',
      sm: 'var(--space-2)',
      md: 'var(--space-3)',
      lg: 'var(--space-4)',
      xl: 'var(--space-6)',
      '2xl': 'var(--space-8)',
    },
    fontSize: {
      xs: 'var(--font-size-xs)',
      sm: 'var(--font-size-sm)',
      base: 'var(--font-size-base)',
      md: 'var(--font-size-md)',
      lg: 'var(--font-size-lg)',
      xl: 'var(--font-size-xl)',
      '2xl': 'var(--font-size-2xl)',
      '3xl': 'var(--font-size-3xl)',
    },
    lineHeight: {
      tight: 'var(--line-height-tight)',
      normal: 'var(--line-height-normal)',
      relaxed: 'var(--line-height-relaxed)',
    },
    zIndex: {
      dropdown: 'var(--z-dropdown)',
      sticky: 'var(--z-sticky)',
      modal: 'var(--z-modal)',
      toast: 'var(--z-toast)',
      skiplink: 'var(--z-skiplink)',
    },
  },
  shortcuts: {
    'glass-card': 'bg-glass backdrop-blur-16 border border-white/10 rounded-lg shadow-card',
    'neon-border': 'border border-primary/20 hover:border-primary/50 transition-all duration-200',
    'glow-text': 'text-primary drop-shadow-[0_0_8px_rgba(0,212,255,0.5)]',
    /* 统一基础组件快捷类：令牌驱动，两端视觉一致（详见 design-tokens.md） */
    'o2-card': 'bg-surface rounded-lg shadow-card p-4',
    'o2-btn':
      'inline-flex items-center justify-center gap-2 h-10 px-4 rounded-md font-medium transition-all duration-150',
    'o2-input':
      'h-10 px-3 rounded-md border border-[var(--border-subtle)] bg-base text-primary transition-colors duration-150',
    'o2-tag': 'inline-flex items-center px-2 py-1 rounded-full text-xs bg-primary/10 text-primary',
    'o2-divider': 'h-px w-full bg-[var(--border-subtle)]',
  },
  // 不注入远程字体外链：测试服/离网环境下 @import 会阻塞首屏渲染，
  // fontFamily 已按 Inter → HarmonyOS Sans SC → 系统栈顺序回退。
})
