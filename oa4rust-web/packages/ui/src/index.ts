import type { App } from 'vue'
import './theme/dark.css'

import AppShell from './components/AppShell.vue'
import LoginScreen from './components/LoginScreen.vue'
import OAuthCallback from './views/OAuthCallback.vue'

// Re-export components for tree-shaking
export { AppShell, LoginScreen, OAuthCallback }

/** Install all UI components into a Vue app */
export function installO2Ui(app: App): void {
  app.component('O2AppShell', AppShell)
  app.component('O2LoginScreen', LoginScreen)
  app.component('O2OAuthCallback', OAuthCallback)
}
