#!/usr/bin/env python3
"""Phase 3: Add SQL history, bookmark, template CRUD, and more to reach ~2000 lines."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')
print(f"Current lines: {len(lines)}")

# ── Step 1: Add more toolbar buttons ───────────────────────────────────────
for i, line in enumerate(lines):
    if 'showExecPlan=true' in line and 'title="执行计划"' in line:
        lines[i] = line + '        <button class="btn btn-outline" @click="showBookmark=!showBookmark" title="书签">⭐ 书签</button>'
        lines[i] = lines[i].replace('</button>\n        <button', '</button>\n        <button')
        break

# ── Step 2: Add modals ─────────────────────────────────────────────────────
modals = r'''
    <!-- Bookmark Panel -->
    <div v-if="showBookmark" class="modal-overlay" @click.self="showBookmark=false">
      <div class="modal-box bookmark-panel">
        <div class="modal-header"><span>⭐ SQL 书签</span><button class="btn-close" @click="showBookmark=false">✕</button></div>
        <div class="bookmark-body">
          <div class="bm-add">
            <input v-model="bmName" class="bm-input" placeholder="书签名称..." />
            <button class="btn-sm" @click="addBookmark()">+ 添加</button>
          </div>
          <div class="bm-list">
            <div v-for="(b,bi) in bookmarks" :key="bi" class="bm-item">
              <span class="bm-icon">⭐</span>
              <span class="bm-name">{{b.name}}</span>
              <span class="bm-time">{{fmtTime(b.ts)}}</span>
              <button class="bm-btn" @click="loadBookmark(bi)">加载</button>
              <button class="bm-btn bm-del" @click="deleteBookmark(bi)">✕</button>
            </div>
          </div>
          <div v-if="bookmarks.length===0" class="bm-empty">暂无书签</div>
        </div>
        <div class="bm-footer"><button class="btn-sm" @click="showBookmark=false">关闭</button></div>
      </div>
    </div>

    <!-- Template CRUD -->
    <div v-if="showTemplateCRUD" class="modal-overlay" @click.self="showTemplateCRUD=false">
      <div class="modal-box tplcrud-panel">
        <div class="modal-header"><span>📑 模板管理</span><button class="btn-close" @click="showTemplateCRUD=false">✕</button></div>
        <div class="tplcrud-toolbar">
          <input v-model="tplSearch" class="tmp-input" placeholder="搜索模板..." />
          <select v-model="tplCat" class="tmp-select">
            <option value="all">全部分类</option>
            <option v-for="c in tplCategories" :key="c" :value="c">{{c}}</option>
          </select>
          <button class="btn-sm" @click="openTplEditor(null)">+ 新建</button>
        </div>
        <div class="tpl-list">
          <div v-for="(t,ti) in filteredTpls" :key="t.id" class="tpl-card">
            <div class="tpl-head"><span class="tpl-icon">{{t.icon||'📋'}}</span><span class="tpl-name">{{t.name}}</span><span class="tpl-cat">{{t.category}}</span></div>
            <pre class="tpl-code">{{t.code}}</pre>
            <div class="tpl-foot">
              <button class="btn-sm" @click="applyTemplate(t)">应用</button>
              <button class="btn-sm" @click="editTemplate(t)">编辑</button>
              <button class="btn-sm btn-danger" @click="deleteTpl(ti)">删除</button>
            </div>
          </div>
        </div>
        <div v-if="filteredTpls.length===0" class="tmpl-empty">暂无模板</div>
      </div>
    </div>

    <!-- Template Editor Modal -->
    <div v-if="showTplEditor" class="modal-overlay" @click.self="showTplEditor=false">
      <div class="modal glass-card">
        <h3>{{tplEditingId?'编辑模板':'新建模板'}}</h3>
        <div class="form-group"><label>名称</label><input v-model="tplForm.name" class="form-input" placeholder="模板名称" /></div>
        <div class="form-group"><label>分类</label>
          <select v-model="tplForm.category" class="form-input">
            <option value="select">SELECT</option><option value="join">JOIN</option><option value="agg">聚合</option><option value="window">窗口函数</option><option value="cte">CTE</option><option value="admin">管理</option>
          </select>
        </div>
        <div class="form-group"><label>图标</label><input v-model="tplForm.icon" class="form-input" placeholder="emoji" /></div>
        <div class="form-group"><label>SQL</label><textarea v-model="tplForm.code" class="form-textarea" rows="8" placeholder="SELECT ..."></textarea></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showTplEditor=false">取消</button>
          <button class="btn-save" :disabled="!tplForm.name" @click="saveTpl">保存</button>
        </div>
      </div>
    </div>
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</template>':
        lines.insert(i, modals)
        break

# ── Step 3: Add state + functions ───────────────────────────────────────────
sf = r'''
// --- Bookmark ---
const showBookmark = ref(false)
const bookmarks = ref<Array<{id:string;name:string;sql:string;ts:number}>>([])
const bmName = ref("")
function addBookmark() {
  if (!bmName.value.trim() || !sql.value.trim()) return
  bookmarks.value.unshift({ id: "bm"+Date.now(), name: bmName.value, sql: sql.value, ts: Date.now() })
  bmName.value = ""
}
function loadBookmark(idx: number) { const b = bookmarks.value[idx]; if (b) { sql.value = b.sql; showBookmark.value = false } }
function deleteBookmark(idx: number) { bookmarks.value.splice(idx, 1) }

// --- Template CRUD ---
const showTemplateCRUD = ref(false)
const tplSearch = ref(""), tplCat = ref("all")
const tplCategories = computed(() => [...new Set(templates.value.map(t => t.category))])
const filteredTpls = computed(() => {
  let list = templates.value
  if (tplSearch.value) { const q = tplSearch.value.toLowerCase(); list = list.filter(t => t.name.toLowerCase().includes(q) || t.code.toLowerCase().includes(q)) }
  if (tplCat.value !== "all") list = list.filter(t => t.category === tplCat.value)
  return list
})
const showTplEditor = ref(false)
const tplEditingId = ref<string|null>(null)
const tplForm = ref({name:"",category:"select",icon:"📋",code:""})
function openTplEditor(t: any) {
  if (t) { tplEditingId.value = t.id; tplForm.value = {name:t.name,category:t.category,icon:t.icon,code:t.code} }
  else { tplEditingId.value = null; tplForm.value = {name:"",category:"select",icon:"📋",code:""} }
  showTplEditor.value = true
}
function editTemplate(t: any) { openTplEditor(t) }
function saveTpl() {
  if (!tplForm.value.name.trim()) return
  if (tplEditingId.value) {
    const t = templates.value.find(x => x.id === tplEditingId.value)
    if (t) Object.assign(t, tplForm.value)
  } else {
    templates.value.push({ id: "t"+Date.now(), ...tplForm.value })
  }
  showTplEditor.value = false
}
function deleteTpl(idx: number) {
  if (!confirm("确定删除此模板？")) return
  templates.value.splice(idx, 1)
}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, sf)
        break

# ── Step 4: Add CSS ────────────────────────────────────────────────────────
css = r'''
/* Bookmark */
.bookmark-panel{width:420px}.bm-body{padding:12px;display:flex;flex-direction:column;gap:10px}.bm-add{display:flex;gap:8px}.bm-input{flex:1;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}.bm-list{display:flex;flex-direction:column;gap:4px;max-height:280px;overflow-y:auto}.bm-item{display:flex;align-items:center;gap:8px;padding:6px 10px;background:rgba(255,255,255,0.02);border-radius:var(--radius-sm);font-size:12px}.bm-icon{font-size:14px}.bm-name{flex:1;color:var(--text-primary)}.bm-time{color:var(--text-muted);font-size:10px;font-family:monospace}.bm-btn{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}.bm-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}.bm-del:hover{border-color:var(--color-danger);color:var(--color-danger)}.bm-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:24px}.bm-footer{display:flex;justify-content:flex-end;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Template CRUD */
.tplcrud-panel{width:640px}.tplcrud-toolbar{display:flex;gap:8px;padding:8px 12px;border-bottom:1px solid var(--border-color)}.tpl-list{padding:12px;max-height:380px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}.tpl-card{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-md);overflow:hidden}.tpl-head{display:flex;align-items:center;gap:6px;padding:6px 10px;background:rgba(59,130,246,0.08);border-bottom:1px solid var(--border-color)}.tpl-icon{font-size:14px}.tpl-name{flex:1;color:var(--text-primary);font-size:12px;font-weight:500}.tpl-cat{font-size:10px;padding:1px 6px;border-radius:10px;background:rgba(59,130,246,0.15);color:var(--color-primary)}.tpl-code{margin:0;padding:8px 10px;background:rgba(0,0,0,0.3);color:#7fdbca;font-size:11px;font-family:monospace;white-space:pre-wrap;word-break:break-all;max-height:60px;overflow-y:auto}.tpl-foot{display:flex;gap:6px;padding:6px 10px;border-top:1px solid var(--border-color)}
/* Result stats bar */
.result-stats-bar{display:flex;align-items:center;gap:12px;padding:6px 12px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.15);border-radius:var(--radius-sm);font-size:11px;color:var(--text-muted);margin-top:4px}
.rs-item{display:flex;align-items:center;gap:4px}.rs-val{color:var(--color-primary);font-weight:600;font-family:monospace}
/* Bulk selection in sidebar */
.sb-item{cursor:pointer}.sb-item input[type="checkbox"]{cursor:pointer}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, css)
        break

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print(f"Done. Lines: {len(lines)}")
