import { defineConfig, presetUno, presetIcons } from 'unocss';

export default defineConfig({
  presets: [presetUno(), presetIcons()],
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
      sm: '6px',
      md: '10px',
      lg: '16px',
      xl: '24px',
    },
  },
  shortcuts: {
    'glass-card': 'bg-glass backdrop-blur-16 border border-white/10 rounded-lg shadow-card',
  },
});
