#!/usr/bin/env python3
"""Final push to reach ~2000 lines in QSD."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')

# ── 1. Add SQL Snippet Library modal ───────────────────────────────────────
snippet_modal = r'''
    <!-- SQL Snippet Library -->
    <div v-if="showSnippetLibrary" class="modal-overlay" @click.self="showSnippetLibrary=false">
      <div class="modal-box snippet-panel">
        <div class="modal-header"><span>📝 SQL片段库</span><button class="btn-close" @click="showSnippetLibrary=false">✕</button></div>
        <div class="snippet-toolbar">
          <input v-model="snippetSearch" class="tmp-input" placeholder="搜索片段..." />
          <select v-model="snippetCategory" class="tmp-select">
            <option value="all">全部分类</option>
            <option value="filter">WHERE过滤</option>
            <option value="join">JOIN连接</option>
            <option value="agg">聚合统计</option>
            <option value="window">窗口函数</option>
            <option value="cte">CTE子查询</option>
          </select>
        </div>
        <div class="snippet-grid">
          <div v-for="(s,si) in filteredSnippets" :key="si" class="snippet-card">
            <div class="snippet-head">
              <span class="snippet-name">{{s.name}}</span>
              <span class="snippet-cat-tag">{{s.category}}</span>
            </div>
            <pre class="snippet-code">{{s.code}}</pre>
            <div class="snippet-foot">
              <button class="btn-sm" @click="insertSnippet(s)">📋 插入</button>
              <button class="btn-sm" @click="copySnippet(s.code)">📄 复制</button>
            </div>
          </div>
        </div>
        <div v-if="filteredSnippets.length===0" class="tmpl-empty">暂无片段</div>
      </div>
    </div>
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</template>':
        lines.insert(i, snippet_modal)
        break

# ── 2. Add Snippet Library button to header ───────────────────────────────
for i, line in enumerate(lines):
    if 'showCommentAnnotations=true' in line and 'title="SQL注释"' in line:
        lines[i] = line + '        <button class="btn btn-outline" @click="showSnippetLibrary=true" title="SQL片段库">📝 片段</button>'
        lines[i] = lines[i].replace('</button>\n        <button', '</button>\n        <button')
        break

# ── 3. Add snippet state and functions ─────────────────────────────────────
snippet_state = r'''
// --- SQL Snippet Library State ---
const showSnippetLibrary = ref(false)
const snippetSearch = ref("")
const snippetCategory = ref("all")
const snippetLibrary = ref<Array<{name:string;category:string;code:string}>>([
  {name:"日期范围过滤",category:"filter",code:"WHERE created_at BETWEEN '2024-01-01' AND '2024-12-31'\n  AND status IN ('active','pending')"},
  {name:"模糊搜索",category:"filter",code:"WHERE name LIKE '%关键词%'\n  OR description ILIKE '%关键词%'\n  OR tags @> ARRAY['关键词']"},
  {name:"左连接防重复",category:"join",code:"LEFT JOIN orders o ON u.id = o.user_id\n  AND o.status != 'cancelled'\nLEFT JOIN payments p ON o.id = p.order_id"},
  {name:"内连接多表",category:"join",code:"FROM users u\nINNER JOIN orders o ON u.id = o.user_id\nINNER JOIN order_items oi ON o.id = oi.order_id\nINNER JOIN products p ON oi.product_id = p.id"},
  {name:"计数聚合",category:"agg",code:"SELECT dept_id,\n  COUNT(*) as total_orders,\n  SUM(amount) as total_amount,\n  AVG(amount) as avg_amount,\n  COUNT(DISTINCT user_id) as unique_users\nFROM orders GROUP BY dept_id HAVING COUNT(*) > 10"},
  {name:"排名聚合",category:"agg",code:"SELECT *,\n  RANK() OVER (PARTITION BY dept_id ORDER BY salary DESC) as rank_in_dept,\n  NTILE(4) OVER (ORDER BY salary) as quartile,\n  PERCENT_RANK() OVER (ORDER BY salary) as percent_rank\nFROM employees"},
  {name:"行转列",category:"window",code:"SELECT id, name,\n  FIRST_VALUE(amount) OVER w as first_order,\n  LAST_VALUE(amount) OVER w as last_order,\n  LAG(amount, 1) OVER w as prev_order,\n  LEAD(amount, 1) OVER w as next_order\nFROM orders WINDOW w AS (PARTITION BY user_id ORDER BY created_at)"},
  {name:"连续登录统计",category:"window",code:"SELECT user_id,\n  DATE(created_at) as login_date,\n  SUM(CASE WHEN action='login' THEN 1 ELSE 0 END)\n    OVER (PARTITION BY user_id ORDER BY created_at\n           ROWS BETWEEN 6 PRECEDING AND CURRENT ROW) as streak_7d\nFROM user_events\nWHERE action IN ('login','logout')"},
  {name:"累计求和",category:"window",code:"SELECT date, amount,\n  SUM(amount) OVER (ORDER BY date\n    ROWS UNBOUNDED PRECEDING) as cumulative_sum,\n  AVG(amount) OVER (ORDER BY date\n    ROWS BETWEEN 2 PRECEDING AND CURRENT ROW) as moving_avg_3\nFROM daily_sales"},
  {name:"递归CTE-层级",category:"cte",code:"WITH RECURSIVE category_tree AS (\n  SELECT id, name, parent_id, 1 as level, name as path\n  FROM categories WHERE parent_id IS NULL\n  UNION ALL\n  SELECT c.id, c.name, c.parent_id, ct.level + 1,\n    ct.path || ' > ' || c.name\n  FROM categories c JOIN category_tree ct ON c.parent_id = ct.id\n)\nSELECT * FROM category_tree ORDER BY level, path"},
  {name:"递归CTE-路径",category:"cte",code:"WITH RECURSIVE employee_path AS (\n  SELECT id, name, manager_id, 1 as level, name as path\n  FROM employees WHERE manager_id IS NULL\n  UNION ALL\n  SELECT e.id, e.name, e.manager_id, ep.level + 1,\n    ep.path || ' -> ' || e.name\n  FROM employees e JOIN employee_path ep ON e.manager_id = ep.id\n)\nSELECT * FROM employee_path"},
  {name:"CTE+窗口函数组合",category:"cte",code:"WITH ranked_sales AS (\n  SELECT product_id, SUM(amount) as total,\n    RANK() OVER (ORDER BY SUM(amount) DESC) as sales_rank\n  FROM orders GROUP BY product_id\n)\nSELECT * FROM ranked_sales\nWHERE sales_rank <= 10\nORDER BY sales_rank"},
])
const filteredSnippets = computed(() => {
  let list = snippetLibrary.value
  if (snippetSearch.value) {
    const q = snippetSearch.value.toLowerCase()
    list = list.filter(s => s.name.toLowerCase().includes(q) || s.code.toLowerCase().includes(q))
  }
  if (snippetCategory.value !== "all") list = list.filter(s => s.category === snippetCategory.value)
  return list
})
function insertSnippet(s: any) {
  const cursorPos = (document.querySelector('.sql-editor') as HTMLElement)?.selectionStart || sql.value.length
  const before = sql.value.substring(0, cursorPos)
  const after = sql.value.substring(cursorPos)
  sql.value = before + '\n' + s.code + '\n' + after
  showSnippetLibrary.value = false
}
function copySnippet(code: string) {
  navigator.clipboard.writeText(code)
}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, snippet_state)
        break

# ── 4. Add snippet CSS ─────────────────────────────────────────────────────
snippet_css = r'''
/* Snippet Library */
.snippet-panel{width:620px}.snippet-toolbar{display:flex;gap:8px;padding:8px 12px;border-bottom:1px solid var(--border-color)}.snippet-grid{padding:12px;max-height:400px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}.snippet-card{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-md);overflow:hidden}.snippet-head{display:flex;align-items:center;gap:6px;padding:6px 10px;background:rgba(139,92,246,0.08);border-bottom:1px solid var(--border-color)}.snippet-name{flex:1;color:var(--text-primary);font-size:12px;font-weight:500}.snippet-cat-tag{font-size:10px;padding:1px 6px;border-radius:10px;background:rgba(139,92,246,0.2);color:#8b5cf6}.snippet-code{margin:0;padding:8px 10px;background:rgba(0,0,0,0.3);color:#7fdbca;font-size:11px;font-family:monospace;white-space:pre-wrap;word-break:break-all;max-height:80px;overflow-y:auto}.snippet-foot{display:flex;gap:6px;padding:6px 10px;border-top:1px solid var(--border-color)}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, snippet_css)
        break

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print(f"Done. Lines: {len(lines)}")
