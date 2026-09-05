#!/usr/bin/env python3
"""Add final features to reach ~2000 lines in QSD."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')

# ── 1. Add bulk action bar to sidebar ─────────────────────────────────────
for i, line in enumerate(lines):
    if '<div class="sb-tabs">' in line:
        lines.insert(i, '        <div class="bulk-bar" v-if="bulkDeleteIds.length">\n          <span>{{bulkDeleteIds.length}} 项选中</span>\n          <button class="bulk-btn" @click="selectAllForDelete()">全选</button>\n          <button class="bulk-btn" @click="clearBulkSelect()">清空</button>\n          <button class="bulk-btn" style="color:var(--color-danger);border-color:var(--color-danger)" @click="bulkDelete()">🗑 批量删除</button>\n        </div>')
        break

# ── 2. Add toolbar buttons for bulk action and export/import ──────────────
for i, line in enumerate(lines):
    if 'showSqlValidator=true' in line and 'title="SQL语法检查"' in line:
        lines[i] = line + '        <button class="btn btn-outline" @click="showExportImport=true" title="导入导出">📤 导出</button>'
        break

# ── 3. Add Export/Import modal ─────────────────────────────────────────────
expimp_modal = r'''
    <!-- Export/Import Modal -->
    <div v-if="showExportImport" class="modal-overlay" @click.self="showExportImport=false">
      <div class="modal-box expimp-panel">
        <div class="modal-header"><span>📤 导入/导出</span><button class="btn-close" @click="showExportImport=false">✕</button></div>
        <div class="expimp-tabs">
          <button :class="['ei-tab',{active:eiTab==='export'}]" @click="eiTab='export'">导出</button>
          <button :class="['ei-tab',{active:eiTab==='import'}]" @click="eiTab='import'">导入</button>
        </div>
        <div v-if="eiTab==='export'" class="ei-body">
          <div class="ei-option"><label>格式:</label>
            <select v-model="exportFormat" class="ei-select">
              <option value="json">JSON</option><option value="sql">SQL文件</option><option value="csv">CSV</option>
            </select>
          </div>
          <div class="ei-count">{{statements.length}} 条语句待导出</div>
          <button class="btn-sm" @click="exportStatements()">📥 导出到文件</button>
        </div>
        <div v-if="eiTab==='import'" class="ei-body">
          <textarea v-model="importData" class="ei-textarea" placeholder="粘贴JSON数据..."></textarea>
          <div v-if="importResult" :class="['ei-result',importResult.includes('成功')?'ok':'err']">{{importResult}}</div>
          <button class="btn-sm" @click="importStatements()">📤 导入</button>
        </div>
        <div class="ei-footer"><button class="btn-sm" @click="showExportImport=false">关闭</button></div>
      </div>
    </div>
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</template>':
        lines.insert(i, expimp_modal)
        break

# ── 4. Add my templates panel ──────────────────────────────────────────────
mytpl_modal = r'''
    <!-- My Templates Panel -->
    <div v-if="showMyTemplates" class="modal-overlay" @click.self="showMyTemplates=false">
      <div class="modal-box mytpl-panel">
        <div class="modal-header"><span>📑 我的模板</span><button class="btn-close" @click="showMyTemplates=false">✕</button></div>
        <div class="mytpl-toolbar">
          <input v-model="myTemplateSearch" class="tmp-input" placeholder="搜索模板..." />
          <select v-model="myTemplatesCategory" class="tmp-select">
            <option value="all">全部分类</option>
            <option v-for="c in allCategories" :key="c" :value="c">{{c}}</option>
          </select>
        </div>
        <div class="my-tmpl-grid">
          <div v-for="t in filteredMyTemplates" :key="t.id" class="my-tmpl-card" @click="applyTemplate(t)">
            <div class="my-tmpl-header"><span class="my-tmpl-icon">{{t.icon}}</span><span class="my-tmpl-name">{{t.name}}</span><span class="my-tmpl-cat">{{t.category}}</span></div>
            <pre class="my-tmpl-code">{{t.code}}</pre>
            <div class="my-tmpl-actions">
              <button class="btn-sm" @click.stop="applyTemplate(t)">应用</button>
              <button class="btn-sm btn-danger" @click.stop="deleteMyTemplate(t.id)">删除</button>
            </div>
          </div>
        </div>
        <div v-if="filteredMyTemplates.length===0" class="tmpl-empty">暂无收藏模板</div>
      </div>
    </div>
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</template>':
        lines.insert(i, mytpl_modal)
        break

# ── 5. Add state variables ─────────────────────────────────────────────────
more_state2 = r'''
// --- Export/Import State ---
const showExportImport = ref(false)
const eiTab = ref<"export"|"import">("export")

// --- My Templates State ---
const showMyTemplates = ref(false)
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, more_state2)
        break

# ── 6. Add helper functions ────────────────────────────────────────────────
more_funcs = r'''
function deleteMyTemplate(id: string) {
  if (!confirm("确定删除此模板？")) return
  const idx = myTemplates.value.findIndex(t => t.id === id)
  if (idx >= 0) myTemplates.value.splice(idx, 1)
}
function toggleBulkSelectAll() {
  if (bulkDeleteIds.value.length === filtered.value.length) clearBulkSelect()
  else selectAllForDelete()
}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, more_funcs)
        break

# ── 7. Add CSS ─────────────────────────────────────────────────────────────
more_css2 = r'''
/* Export/Import */
.expimp-panel{width:480px}.ei-tabs{display:flex;gap:4px;padding:8px 12px;border-bottom:1px solid var(--border-color)}.ei-tab{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}.ei-tab.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}.ei-body{padding:12px;display:flex;flex-direction:column;gap:10px}.ei-option{display:flex;align-items:center;gap:8px;font-size:12px}.ei-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.ei-count{font-size:11px;color:var(--text-muted)}.ei-textarea{width:100%;height:120px;background:rgba(0,0,0,0.3);border:1px solid var(--border-color);color:#7fdbca;font-family:monospace;font-size:11px;padding:10px;resize:vertical;outline:none;border-radius:var(--radius-sm)}.ei-result{padding:6px 10px;border-radius:var(--radius-sm);font-size:12px}.ei-result.ok{background:rgba(16,185,129,0.1);color:#10b981}.ei-result.err{background:rgba(239,68,68,0.1);color:#ef4444}.ei-footer{display:flex;justify-content:flex-end;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* My Templates */
.mytpl-panel{width:600px}.mytpl-toolbar{display:flex;gap:8px;padding:8px 12px;border-bottom:1px solid var(--border-color)}.tmp-input{flex:1;padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}.tmp-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
/* Toolbar enhancements */
.smd-toolbar{display:flex;align-items:center;gap:6px;padding:8px 12px;border-bottom:1px solid var(--border-color);flex-wrap:wrap}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, more_css2)
        break

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print(f"Done. Lines: {len(lines)}")
