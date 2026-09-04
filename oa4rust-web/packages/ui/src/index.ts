import type { App } from 'vue';
import './theme/dark.css';

// Re-export components for tree-shaking
export { default as AppShell } from './components/AppShell.vue';
export { default as LoginScreen } from './components/LoginScreen.vue';
export { default as OAuthCallback } from './views/OAuthCallback.vue';

/** Install all UI components into a Vue app */
export function installO2Ui(app: App): void {
  app.component('O2AppShell', AppShell);
  app.component('O2LoginScreen', LoginScreen);
  app.component('O2OAuthCallback', OAuthCallback);
}
