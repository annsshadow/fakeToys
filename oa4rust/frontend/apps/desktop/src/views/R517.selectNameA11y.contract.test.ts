// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R517 ConfigDesignerApp 双 select 可访问名称契约（WCAG 2.1 SC 4.1.2 Name）：
 * editorLang 与 editorTheme 两个 select 必须各有非空可访问名（aria-label 或 for/id 关联）。
 * 真父版必 RED（核心断言失败、守卫通过），修复版必 GREEN。
 */
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = readFileSync(resolve(import.meta.dirname, 'ConfigDesignerApp.vue'), 'utf8');

/** 取 v-model 对应 select 的开标签（从 <select 到 >） */
function selectTag(model: string): string {
  const needle = 'v-model="' + model + '"';
  const at = source.indexOf(needle);
  if (at < 0) return '';
  const start = source.lastIndexOf('<select', at);
  const end = source.indexOf('>', at);
  if (start < 0 || end < 0) return '';
  return source.slice(start, end + 1);
}

/** 开标签带 aria-label/aria-labelledby，或带 id 且有 label[for] 关联 */
function hasName(tag: string, sourceText: string): boolean {
  if (/aria-label=/.test(tag) || /aria-labelledby=/.test(tag)) return true;
  const idMatch = tag.match(/id="([^"]+)"/);
  if (idMatch) {
    if (sourceText.includes('for="' + idMatch[1] + '"')) return true;
  }
  return false;
}

describe('R517 ConfigDesignerApp 双 select 可访问名称契约', () => {
  const lang = selectTag('editorLang');
  const theme = selectTag('editorTheme');

  it('配置格式 select 带非空可访问名（核心）', () => {
    expect(lang, '必须找到 editorLang select 开标签').not.toBe('');
    expect(hasName(lang, source)).toBe(true);
  });

  it('编辑器主题 select 带非空可访问名（核心）', () => {
    expect(theme, '必须找到 editorTheme select 开标签').not.toBe('');
    expect(hasName(theme, source)).toBe(true);
  });

  it('业务零改守卫：两 select 的 v-model/class/option 列表/value 保留', () => {
    expect(lang).toContain('v-model="editorLang"');
    expect(theme).toContain('v-model="editorTheme"');
    expect(lang).toContain('class="tb-select"');
    expect(theme).toContain('class="tb-select"');
    expect(source).toContain('<option value="json">JSON</option>');
    expect(source).toContain('<option value="yaml">YAML</option>');
    expect(source).toContain('<option value="properties">Properties</option>');
    expect(source).toContain('<option value="dark">暗色主题</option>');
    expect(source).toContain('<option value="light">亮色主题</option>');
  });

  it('业务零改守卫：模板结构/工具栏/编辑器布局保留', () => {
    expect(source).toContain('data-testid="config-designer-page"');
    expect(source).toContain('class="toolbar glass-card"');
    expect(source).toContain('class="toolbar-left"');
    expect(source).toContain('class="toolbar-right"');
    expect(source).toContain('class="editor-layout glass-card"');
    expect(source).toContain('<textarea v-model="config"');
  });

  it('业务零改守卫：事件处理器/脚本 ref 默认值保留', () => {
    expect(source).toContain('@click="preview"');
    expect(source).toContain('@click="formatConfig"');
    expect(source).toContain('@click="validateConfig"');
    expect(source).toContain("editorLang = ref('json')");
    expect(source).toContain("editorTheme = ref('dark')");
  });
});
