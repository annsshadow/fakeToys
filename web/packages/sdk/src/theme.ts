import { ref, type Ref } from 'vue';

export type ThemeMode = 'dark' | 'light';

let _theme = ref<ThemeMode>('dark');

/**
 * 主题管理
 * o2web 不支持主题切换，这是新栈的增强能力
 */
export function createThemeProvider() {
  function setTheme(mode: ThemeMode): void {
    _theme.value = mode;
    document.documentElement.setAttribute('data-theme', mode);
    localStorage.setItem('oa4rust_theme', mode);
  }

  function toggleTheme(): void {
    setTheme(_theme.value === 'dark' ? 'light' : 'dark');
  }

  function init(): void {
    const stored = localStorage.getItem('oa4rust_theme') as ThemeMode | null;
    if (stored) {
      _theme.value = stored;
    }
    document.documentElement.setAttribute('data-theme', _theme.value);
  }

  return {
    theme: _theme as Ref<ThemeMode>,
    setTheme,
    toggleTheme,
    init,
  };
}

export const themeProvider = createThemeProvider();

/** Composable 入口 */
export function useTheme() {
  return themeProvider;
}
