import { describe, it, expect } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const source = readFileSync(resolve(__dirname, 'doc/doc.vue'), 'utf8');
const lines = source.split('\n');

function tagLine(needle: string) {
  const line = lines.find((l) => l.includes(needle));
  return line ?? '';
}
function tagAfter(anchor: string, pattern: RegExp) {
  const i = source.indexOf(anchor);
  if (i < 0) return '';
  const m = source.slice(i, i + 700).match(pattern);
  return m ? m[0] : '';
}
function expectA11y(tag: string, label: string, handler: string) {
  expect(tag.length, '标签存在').toBeGreaterThan(0);
  expect(tag).toContain('role="button"');
  expect(tag).toContain('tabindex="0"');
  expect(tag).toContain('aria-label=');
  expect(tag).toContain(label);
  expect(tag).toContain('@keydown.enter=');
  expect(tag).toContain('@keydown.space.prevent=');
  expect(tag).toContain(handler);
}
const RETRY = /<view class="retry-btn"[^>]*>/;
const REFRESH = /<view class="empty-guide-link ai-pressable"[^>]*>/;

describe('R560 doc.vue 8类@tap伪按钮键盘可达修复', () => {
  it('附件错误态 retry 键盘可达并复用 load()', () => {
    expectA11y(tagAfter('{{ attachError }}', RETRY), '重试', 'load()');
  });
  it('附件空态刷新链接键盘可达并复用 load()', () => {
    expectA11y(tagAfter('拍照或上传文件后', REFRESH), '刷新', 'load()');
  });
  it('附件 banner retry 键盘可达并复用 load()', () => {
    expectA11y(tagAfter('attachError && attachRows.length', RETRY), '重试', 'load()');
  });
  it('附件项动态 aria-label 且 Enter/Space 复用 downloadAttachment(row)', () => {
    expectA11y(tagLine('v-for="(row, i) in attachRows"'), "'查看' + nameOf(row) + '附件'", 'downloadAttachment(row)');
  });
  it('文档库错误态 retry 键盘可达并复用 load()', () => {
    expectA11y(tagAfter('{{ filesError }}', RETRY), '重试', 'load()');
  });
  it('文档库空态刷新链接键盘可达并复用 load()', () => {
    expectA11y(tagAfter('上传一份文件，或刷新查看最新文档', REFRESH), '刷新', 'load()');
  });
  it('文档库 banner retry 键盘可达并复用 load()', () => {
    expectA11y(tagAfter('filesError && rows.length', RETRY), '重试', 'load()');
  });
  it('文档项动态 aria-label 且 Enter/Space 复用 downloadAndOpen(row)', () => {
    expectA11y(tagLine('v-for="(row, i) in rows"'), "'查看' + nameOf(row) + '文档'", 'downloadAndOpen(row)');
  });
  it('@tap 全保留，业务 handler 与 fileApi 零改', () => {
    expect((source.match(/@tap=/g) ?? []).length).toBeGreaterThanOrEqual(8);
    expect(source).toContain('async function load()');
    expect(source).toContain('async function downloadAttachment(row: Record<string, unknown>)');
    expect(source).toContain('async function downloadAndOpen(row: Record<string, unknown>)');
    expect(source).toContain('fileApi.fileList(unique)');
    expect(source).toContain('fileApi.attachmentList(unique)');
  });
  it('同页5处原生<button>零改（uni-button另一族排除）；无全局增强器/他页污染', () => {
    const btns = source.match(/<button[^>]*>/g) ?? [];
    expect(btns.length).toBe(5);
    for (const b of btns) {
      expect(b).not.toMatch(/role="button"/);
      expect(b).not.toMatch(/tabindex=/);
      expect(b).not.toMatch(/aria-label=/);
      expect(b).not.toMatch(/@keydown/);
    }
    expect(source).not.toContain('installH5NativeInputNameA11y');
    expect(source).not.toContain('calendar-create-btn');
  });
});