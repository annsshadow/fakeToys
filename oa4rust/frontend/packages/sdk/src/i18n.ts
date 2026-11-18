// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { ref } from 'vue'
import type { Composer, I18n } from 'vue-i18n'
import { createI18n } from 'vue-i18n'

export type SupportedLocale = 'zh-cn' | 'en' | 'es'
const DEFAULT_LOCALE: SupportedLocale = 'zh-cn'

/**
 * WCAG 3.1.1 Language of Page：语言切换必须同步 <html lang>，否则屏幕阅读器
 * 会继续用旧语言（index.html 写死 zh-CN）发音朗读英文/西语界面文本。
 */
const DOC_LANG_BY_LOCALE: Record<SupportedLocale, string> = {
  'zh-cn': 'zh-cn',
  en: 'en',
  es: 'es',
}

function syncDocumentLanguage(locale: SupportedLocale): void {
  if (typeof document === 'undefined') return
  document.documentElement.setAttribute('lang', DOC_LANG_BY_LOCALE[locale])
}

let _i18n: I18n | null = null
const _locale = ref<SupportedLocale>(DEFAULT_LOCALE)

export function useI18nInstance(): I18n {
  if (!_i18n) {
    _i18n = createI18n({
      legacy: false,
      locale: _locale.value,
      fallbackLocale: 'en',
      messages: { 'zh-cn': {}, en: {}, es: {} },
    })
  }
  return _i18n
}

export { useI18n } from 'vue-i18n'

export function getLocale(): SupportedLocale {
  return _locale.value
}

export function setLocale(locale: SupportedLocale): void {
  _locale.value = locale
  if (_i18n) (_i18n.global as Composer).locale.value = locale
  syncDocumentLanguage(locale)
}

export function registerMessages(locale: SupportedLocale, messages: Record<string, string>): void {
  if (!_i18n) return
  const inst = _i18n.global as Composer
  const existing = inst.getLocaleMessage(locale) ?? {}
  inst.setLocaleMessage(locale, { ...existing, ...messages })
}
