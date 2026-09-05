#!/usr/bin/env python3
"""Phase 6: Final additions to reach ~2000 lines."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')
print(f"Current lines: {len(lines)}")

# ── Step 1: Add result stats bar in results section ────────────────────────
for i, line in enumerate(lines):
    if '<div class="results-pager"' in line and 'v-if="resultData.length > 0"' in lines[i-1]:
        lines.insert(i, '        <div class="result-stats-bar" v-if="resultData.length > 0">\n          <div class="rs-item"><span>行数:</span><span class="rs-val">{{resultData.length}}</span></div>\n          <div class="rs-item"><span>列数:</span><span class="rs-val">{{resultHeaders.length}}</span></div>\n          <div class="rs-item"><span>耗时:</span><span class="rs-val">{{lastExecDuration}}ms</span></div>\n          <div class="rs-item"><span>分页:</span><span class="rs-val">{{page}}/{{totalPages}}</span></div>\n          <button class="rap-btn" @click="copySqlWithTimestamp()">📋 复制</button>\n          <button class="rap-btn" @click="showSqlHints=true">💡 提示</button>\n        </div>')
        break

# ── Step 2: Add lastExecDuration ref ───────────────────────────────────────
for i, line in enumerate(lines):
    if 'const execTimestamp' in line:
        lines.insert(i+1, 'const lastExecDuration = ref(0)')
        break

# Also update executeSQL to set duration
for i, line in enumerate(lines):
    if 'async function executeSQL()' in line:
        # Find the end of this function and add duration tracking
        for j in range(i+1, min(i+20, len(lines))):
            if lines[j].strip() == '} finally {' or (lines[j].strip().startswith('} finally') and 'loadingResult' in lines[j]):
                lines.insert(j, '    lastExecDuration.value = Date.now() - t0')
                break
        break

# ── Step 3: Add more snippet library entries ───────────────────────────────
for i, line in enumerate(lines):
    if line.strip().endswith("}]})") and i > 1700:
        # Check if this is the snippetLibrary closing
        context = '\n'.join(lines[max(0,i-10):i])
        if 'snippetLibrary' in context:
            new_snippets = [
              '  {name:"INSERT批量",category:"admin",code:"INSERT INTO users (name, email, created_at)\\nVALUES (:name, :email, NOW())\\nON CONFLICT (email) DO NOTHING\\nRETURNING id, name"},',
              '  {name:"UPDATE分批",category:"admin",code:"UPDATE orders SET status = :new_status\\nWHERE id IN (\\n  SELECT id FROM orders\\n  WHERE status = :old_status\\n  LIMIT :batch_size\\n)\\nRETURNING id"},',
              '  {name:"窗口函数NTILE",category:"window",code:"SELECT name, salary,\\n  NTILE(4) OVER (ORDER BY salary DESC) as quartile,\\n  PERCENT_RANK() OVER (ORDER BY salary) as pct_rank\\nFROM employees"},',
              '  {name:"PIVOT行转列",category:"agg",code:"SELECT category,\\n  SUM(CASE WHEN status=\'A\' THEN 1 ELSE 0 END) as active,\\n  SUM(CASE WHEN status=\'I\' THEN 1 ELSE 0 END) as inactive\\nFROM products GROUP BY category"},',
              '  {name:"递归路径查询",category:"cte",code:"WITH RECURSIVE paths AS (\\n  SELECT id, name, ARRAY[id] as path, 1 as depth\\n  FROM categories WHERE parent_id IS NULL\\n  UNION ALL\\n  SELECT c.id, c.name, p.path || c.id, p.depth+1\\n  FROM categories c JOIN paths p ON c.parent_id = p.id\\n)\\nSELECT * FROM paths ORDER BY depth, path"},',
              '  {name:"并行聚合优化",category:"optimize",code:"-- 启用并行聚合\\nSET max_parallel_workers_per_gather = 8;\\nSELECT dept_id,\\n  SUM(salary) as total_sal,\\n  AVG(salary) as avg_sal,\\n  COUNT(*) as cnt\\nFROM employees\\nGROUP BY dept_id\\nORDER BY total_sal DESC\\nLIMIT 20"},',
            ]
            for j, s in enumerate(new_snippets):
                lines.insert(i + j, s)
            break

# ── Step 4: Add more CSS for new features ──────────────────────────────────
extra_css2 = r'''
/* Result stats bar */
.result-stats-bar{display:flex;align-items:center;gap:12px;padding:6px 12px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.15);border-radius:var(--radius-sm);font-size:11px;color:var(--text-muted);margin-top:4px}
.rs-item{display:flex;align-items:center;gap:4px}.rs-val{color:var(--color-primary);font-weight:600;font-family:monospace}
.rap-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.rap-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
/* Enhanced editor toolbar */
.editor-toolbar{display:flex;align-items:center;gap:8px;margin-bottom:8px;flex-wrap:wrap}
/* Query param bar */
.query-param-bar{display:flex;align-items:center;gap:8px;padding:6px 12px;background:rgba(245,158,11,0.05);border:1px solid rgba(245,158,11,0.15);border-radius:var(--radius-sm);font-size:11px;flex-wrap:wrap;margin-top:4px}
.param-chip{display:inline-flex;align-items:center;gap:4px;padding:2px 8px;background:rgba(245,158,11,0.15);border:1px solid rgba(245,158,11,0.3);border-radius:10px;font-size:10px;color:#f59e0b;font-family:monospace}
.query-param-input{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;width:100px}
/* Gen SQL bar */
.gen-sql-bar{display:flex;align-items:center;gap:8px;padding:8px 12px;background:rgba(16,185,129,0.05);border:1px solid rgba(16,185,129,0.2);border-radius:var(--radius-sm);margin-top:8px}
.gen-sql-label{font-size:11px;color:#10b981;font-weight:600}
.gen-sql-btn{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--color-success);background:rgba(16,185,129,0.1);color:#10b981;cursor:pointer;font-size:11px}
.gen-sql-btn:hover{background:rgba(16,185,129,0.2)}
/* Editor mode tabs */
.editor-mode-tabs{display:flex;gap:4px;margin-bottom:8px}
.emt{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px}
.emt.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}
/* Snippet card hover */
.snippet-card:hover{border-color:var(--color-primary)}
/* Visual editor fields */
.ve-panel{width:680px}.ve-body{padding:12px;max-height:440px;overflow-y:auto;display:flex;flex-direction:column;gap:10px}
.ve-fields-row{display:flex;gap:8px}.ve-field-group{display:flex;flex-direction:column;gap:4px;flex:1}
.ve-field-group label{font-size:11px;color:var(--text-muted);font-weight:600}
.ve-tags{display:flex;flex-wrap:wrap;gap:4px;padding:6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);min-height:32px}
.ve-tag{display:inline-flex;align-items:center;gap:4px;padding:2px 8px;background:rgba(59,130,246,0.15);border:1px solid rgba(59,130,246,0.3);border-radius:10px;font-size:11px;color:var(--color-primary);font-family:monospace}
.ve-tag-del{cursor:pointer;opacity:0.7;font-size:10px}.ve-tag-input{border:none;background:transparent;color:var(--text-primary);font-size:11px;outline:none;flex:1;min-width:80px}
.ve-row{display:flex;gap:8px}.ve-select{flex:1;padding:6px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}
.ve-select-sm{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.ve-input{flex:1;padding:6px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;outline:none}
.ve-input-sm{width:80px;padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.ve-conditions{display:flex;flex-direction:column;gap:4px;max-height:120px;overflow-y:auto}
.ve-cond-row{display:flex;align-items:center;gap:4px}
.ve-sel-sm{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;min-width:80px}
.ve-del-btn{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--color-danger);cursor:pointer;font-size:10px}
.ve-add-btn{padding:3px 8px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px;margin-top:4px}
.ve-add-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.ve-preview{background:rgba(0,0,0,0.3);border-radius:var(--radius-sm);padding:10px}
.ve-preview-label{font-size:11px;color:var(--text-muted);margin-bottom:4px}
.ve-sql-out{margin:0;padding:8px;background:rgba(0,0,0,0.4);color:#10b981;font-size:12px;font-family:monospace;border-radius:4px;white-space:pre-wrap;word-break:break-all;max-height:100px;overflow-y:auto}
.ve-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* Rule chain editor */
.rc-panel{width:600px}.rc-body{padding:12px;max-height:440px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}
.rc-rule-list{display:flex;flex-direction:column;gap:6px}.rc-rule{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-sm);padding:8px}
.rc-rule-disabled{opacity:0.5}.rc-rule-header{display:flex;align-items:center;gap:6px;flex-wrap:wrap}
.rc-rule-type{padding:2px 6px;border-radius:3px;font-size:10px;font-weight:700;background:rgba(59,130,246,0.2);color:var(--color-primary)}
.rc-rule-field{color:var(--text-primary);font-family:monospace;font-size:11px;min-width:80px}
.rc-rule-op{color:var(--color-primary);font-size:11px;font-weight:600}
.rc-rule-val{flex:1;padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.rc-rule-del{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--color-danger);cursor:pointer;font-size:10px}
.rc-rule-children{margin-left:16px;padding-left:8px;border-left:2px solid var(--border-color);display:flex;flex-direction:column;gap:4px;margin-top:4px}
.rc-rule-inner{display:flex;align-items:center;gap:6px;font-size:11px;color:var(--text-muted)}
.rc-btn-sm{padding:2px 6px;border-radius:3px;border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.rc-btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.rc-add-main{padding:6px 12px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px;width:100%}
.rc-add-main:hover{border-color:var(--color-primary);color:var(--color-primary)}
.rc-preview{background:rgba(0,0,0,0.3);border-radius:var(--radius-sm);padding:10px}
.rc-preview-label{font-size:11px;color:var(--text-muted);margin-bottom:4px}
.rc-sql-out{margin:0;padding:8px;background:rgba(0,0,0,0.4);color:#10b981;font-size:11px;font-family:monospace;border-radius:4px;white-space:pre-wrap;word-break:break-all;max-height:80px;overflow-y:auto}
.rc-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color)}
/* Field drag config */
.fd-panel{width:680px}.fd-body{padding:12px;max-height:460px;overflow-y:auto}
.fd-layout{display:grid;grid-template-columns:1fr 1fr;gap:12px}
.fd-col-title{font-size:12px;font-weight:600;color:var(--color-primary);margin-bottom:6px;display:flex;align-items:center;gap:6px}
.fd-count{font-size:10px;color:var(--text-muted);font-weight:400}
.fd-search{width:100%;padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;margin-bottom:6px;box-sizing:border-box}
.fd-available{max-height:180px;overflow-y:auto;display:flex;flex-direction:column;gap:2px}
.fd-item{padding:4px 8px;border-radius:var(--radius-sm);font-size:11px;font-family:monospace;cursor:pointer;color:var(--text-primary);border:1px solid transparent}
.fd-item:hover{border-color:var(--color-primary);background:rgba(59,130,246,0.1)}
.fd-item.fd-used{background:rgba(16,185,129,0.1);border-color:rgba(16,185,129,0.3);color:#10b981}
.fd-selected-list{display:flex;flex-direction:column;gap:2px;max-height:150px;overflow-y:auto}
.fd-selected-item{display:flex;align-items:center;gap:6px;padding:4px 8px;background:rgba(16,185,129,0.08);border:1px solid rgba(16,185,129,0.2);border-radius:var(--radius-sm);font-size:11px;font-family:monospace;color:#10b981}
.fd-selected-item.fd-wf{background:rgba(245,158,11,0.08);border-color:rgba(245,158,11,0.2);color:#f59e0b}
.fd-remove{padding:1px 4px;border-radius:3px;border:none;background:transparent;color:inherit;cursor:pointer;font-size:10px;opacity:0.7}
.fd-remove:hover{opacity:1}.fd-empty-hint{font-size:11px;color:var(--text-muted);text-align:center;padding:12px}
.fd-sql-preview{margin-top:12px;background:rgba(0,0,0,0.3);border-radius:var(--radius-sm);padding:10px}
.fd-preview-label{font-size:11px;color:var(--text-muted);margin-bottom:4px}
.fd-sql-out{margin:0;padding:8px;background:rgba(0,0,0,0.4);color:#10b981;font-size:11px;font-family:monospace;border-radius:4px;white-space:pre-wrap;word-break:break-all;max-height:80px;overflow-y:auto}
.fd-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Chart linkage */
.cl-panel{width:640px}.cl-body{padding:12px;max-height:480px;overflow-y:auto}
.cl-config{display:flex;flex-direction:column;gap:10px}.cl-row{display:flex;gap:8px;flex-wrap:wrap}
.cl-group{display:flex;flex-direction:column;gap:4px;flex:1;min-width:120px}
.cl-group label{font-size:11px;color:var(--text-muted);font-weight:600}
.cl-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.cl-chart-area{display:flex;align-items:flex-end;gap:4px;height:140px;padding:12px;background:rgba(255,255,255,0.02);border-radius:var(--radius-sm);border:1px solid var(--border-color)}
.cl-chart-bar{display:flex;flex-direction:column;align-items:center;justify-content:flex-end;flex:1;border-radius:3px 3px 0 0;cursor:pointer;transition:opacity 0.15s;min-width:20px;padding:2px}
.cl-chart-bar:hover{opacity:0.8}.cl-bar-label{font-size:8px;color:var(--text-muted);position:absolute;bottom:-16px;white-space:nowrap;max-width:60px;overflow:hidden;text-overflow:ellipsis}
.cl-bar-val{font-size:9px;color:var(--text-primary);margin-bottom:2px}
.cl-chart-empty{color:var(--text-muted);font-size:12px;text-align:center;width:100%;align-self:center}
.cl-filters{display:flex;align-items:center;gap:6px;flex-wrap:wrap;padding:8px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.2);border-radius:var(--radius-sm)}
.cl-filter-tag{display:flex;align-items:center;gap:4px;padding:2px 8px;background:rgba(59,130,246,0.15);border:1px solid rgba(59,130,246,0.3);border-radius:10px;font-size:10px;color:var(--color-primary);font-family:monospace}
.cl-filter-tag button{background:none;border:none;color:var(--color-primary);cursor:pointer;font-size:10px}
.cl-clear-all{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.cl-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
/* Advanced templates panel */
.at-panel{width:720px}.at-tabs{display:flex;gap:4px;padding:8px 12px;border-bottom:1px solid var(--border-color)}
.at-tab{padding:4px 12px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.at-tab.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}
.at-grid{padding:12px;max-height:400px;overflow-y:auto;display:flex;flex-direction:column;gap:8px}
.at-card{background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:var(--radius-md);overflow:hidden}
.at-card-head{display:flex;align-items:center;gap:6px;padding:8px 12px;background:rgba(59,130,246,0.08);border-bottom:1px solid var(--border-color)}
.at-icon{font-size:16px}.at-name{flex:1;color:var(--text-primary);font-size:13px;font-weight:500}
.at-cat-tag{font-size:10px;padding:1px 8px;border-radius:10px;background:rgba(245,158,11,0.15);color:#f59e0b}
.at-code{margin:0;padding:8px 12px;background:rgba(0,0,0,0.3);color:#10b981;font-size:11px;font-family:monospace;white-space:pre-wrap;word-break:break-all;max-height:80px;overflow-y:auto}
.at-card-foot{display:flex;gap:6px;padding:8px 12px;border-top:1px solid var(--border-color)}
.at-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:32px}
/* Button utilities */
.btn-xs{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.btn-danger{border-color:var(--color-danger);color:var(--color-danger)}.btn-danger:hover{background:rgba(239,68,68,0.1)}
/* Scrollbar polish */
::-webkit-scrollbar{width:6px;height:6px}
::-webkit-scrollbar-track{background:transparent}
::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:3px}
::-webkit-scrollbar-thumb:hover{background:var(--text-muted)}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, extra_css2)
        break

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print(f"Done. Lines: {len(lines)}")
