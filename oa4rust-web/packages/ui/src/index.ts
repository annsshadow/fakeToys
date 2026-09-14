import type { App } from 'vue'
import './theme/dark.css'

import AppShell from './components/AppShell.vue'
import LoginScreen from './components/LoginScreen.vue'
import OrganizationSelector from './components/OrganizationSelector.vue'
import OAuthCallback from './views/OAuthCallback.vue'

export type {
  OrganizationSelectorItem,
  OrganizationSelectorMode,
  OrganizationSelectorSearch,
  OrganizationSelectorType,
} from './components/organization-selector'
export {
  normalizeOrganizationSearchResponse,
  normalizeOrganizationSelectorItem,
  ORGANIZATION_SELECTOR_TYPES,
  organizationSelectorSearchPath,
  searchOrganizationSelector,
} from './components/organization-selector'

// Re-export components for tree-shaking
export { AppShell, LoginScreen, OAuthCallback, OrganizationSelector }

/** Install all UI components into a Vue app */
export function installO2Ui(app: App): void {
  app.component('O2AppShell', AppShell)
  app.component('O2LoginScreen', LoginScreen)
  app.component('O2OAuthCallback', OAuthCallback)
  app.component('O2OrganizationSelector', OrganizationSelector)
}
