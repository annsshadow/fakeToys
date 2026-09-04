import { defineConfig } from 'unocss';

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
      sans: [
        'Inter',
        'HarmonyOS Sans SC',
        'PingFang SC',
        'Microsoft YaHei',
        'system-ui',
        'sans-serif',
      ],
      mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
      display: ['Orbitron', 'Rajdhani', 'sans-serif'],
    },
    borderRadius: {
      sm: 'var(--radius-sm)',
      md: 'var(--radius-md)',
      lg: 'var(--radius-lg)',
      xl: 'var(--radius-xl)',
    },
  },
  shortcuts: {
    'glass-card':
      'bg-glass backdrop-blur-16 border border-white/10 rounded-lg shadow-card',
    'neon-border': 'border border-primary/20 hover:border-primary/50 transition-all duration-200',
    'glow-text': 'text-primary drop-shadow-[0_0_8px_rgba(0,212,255,0.5)]',
  },
  preflights: [
    {
      getCSS: () => `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=Orbitron:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap');
      `,
    },
  ],
});
