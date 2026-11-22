// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R339 契约：PortalApp.vue 页面头部不得保留无实现、无消费点的死 CTA（+ 新建页面）。
 * R338 t141 审计坐实：点击零 UI、零请求（newPageModalCount=0 / newPageEndpointCalls=0）。
 * 队长裁决方案 A：无编辑器时移除该 CTA；下方端点组建页入口保持原样。
 * 断言为源码模板静态结构（遵循仓库 *.contract.test.ts 惯例，不引入新依赖）。
 */
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const src = readFileSync(resolve(import.meta.dirname, 'PortalApp.vue'), 'utf8');
const tpl = src.slice(src.indexOf('<template>'), src.lastIndexOf('</template>'));

function headerBlock(template: string): string {
  const open = template.indexOf('<AdminPageHeader');
  if (open < 0) return '';
  const close = template.indexOf('</AdminPageHeader>', open);
  return template.slice(open, close < 0 ? template.length : close);
}

describe('R339 PortalApp.vue 移除死 CTA 契约', () => {
  it('页面头部 AdminPageHeader 不再有 @click showEditor=true 的死 CTA', () => {
    const h = headerBlock(tpl);
    expect(h, '头部仍有 showEditor 死绑定').not.toContain('showEditor = true');
    expect(h, '头部仍有 + 新建页面 死按钮').not.toContain('+ 新建页面');
    expect(h, '头部仍有 new-page-btn 主操作').not.toContain('class="new-page-btn"');
  });

  it('showEditor 死变量彻底消除（无编辑器时不应残留无消费点绑定）', () => {
    expect(src, '源码仍有 showEditor 死绑定').not.toContain('showEditor');
  });

  it('AdminPageHeader 标题/副标题保持（不误删头部与具名 actions 插槽）', () => {
    const h = headerBlock(tpl);
    expect(h).toContain('<AdminPageHeader');
    expect(h).toContain('title="门户管理"');
    expect(h).toContain('接入 /api/portal/*');
  });

  it('下方端点操作四组 details 与建页入口保持原样（方案 A 不删端点组）', () => {
    expect(tpl).toContain('端点操作 · 门户与表面');
    expect(tpl).toContain('端点操作 · 设计器增删');
    expect(tpl, '设计器建页面入口应保持').toContain("designerCreate('page')");
    expect(tpl, '表面建门户入口应保持').toContain("surfaceOps('create')");
  });

  it('端点调用逻辑写读链未回退（designerCreate/surfaceOps/designerMisc 方法体仍在）', () => {
    for (const fn of ['designerCreate', 'designerUpdate', 'designerDelete', 'designerMisc', 'surfaceOps', 'loadPortalList']) {
      expect(src, '缺方法: ' + fn).toContain(fn);
    }
    expect(src).toContain('/api/portal/assemble/designer/page');
    expect(src, 'surface/create 端点应保持').toContain('/api/portal/assemble/surface/create');
  });
});
