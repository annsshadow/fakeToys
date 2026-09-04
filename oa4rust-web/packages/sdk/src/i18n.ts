import { createI18n } from 'vue-i18n';
import type { I18n } from 'vue-i18n';
import { ref } from 'vue';

export type SupportedLocale = 'zh-cn' | 'en' | 'es';
const DEFAULT_LOCALE: SupportedLocale = 'zh-cn';

let _i18n: I18n | null = null;
let _locale = ref<SupportedLocale>(DEFAULT_LOCALE);

export function useI18nInstance(): I18n {
  if (!_i18n) {
    _i18n = createI18n({
      legacy: false,
      locale: DEFAULT_LOCALE,
      fallbackLocale: 'en',
      messages: { 'zh-cn': {}, en: {}, es: {} },
    });
  }
  return _i18n;
}

export { useI18n } from 'vue-i18n';

export function getLocale(): SupportedLocale { return _locale.value; }

export function setLocale(locale: SupportedLocale): void {
  _locale.value = locale;
  if (_i18n) (_i18n.global as any).locale.value = locale;
}

export function registerMessages(locale: SupportedLocale, messages: Record<string, string>): void {
  if (!_i18n) return;
  const inst = _i18n as any;
  const existing = inst.global.getLocaleMessage(locale) ?? {};
  inst.global.setLocaleMessage(locale, { ...existing, ...messages });
}
