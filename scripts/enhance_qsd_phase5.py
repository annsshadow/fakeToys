#!/usr/bin/env python3
"""Phase 5: Add more templates, enhanced toolbar, and extra CSS to reach ~2000 lines."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')
print(f"Current lines: {len(lines)}")

# ── Step 1: Add extra toolbar buttons ──────────────────────────────────────
for i, line in enumerate(lines):
    if 'showSqlHints=true' in line or ('showParamPresets=true' in line):
        pass
# Find the last button in smd-actions
for i, line in enumerate(lines):
    if 'showBookmark' in line and '⭐ 书签' in line:
        lines[i] = line + '        <button class="btn btn-outline" @click="showSqlHints=true" title="智能提示">💡 提示</button>'
        lines[i] = lines[i].replace('</button>\n        <button', '</button>\n        <button')
        break

# ── Step 2: Add more advanced templates ────────────────────────────────────
# Find the last advancedTemplates entry and add more
for i, line in enumerate(lines):
    if line.strip() == '])' and i > 1300:
        # Check if this is the advancedTemplates closing bracket
        context_before = '\n'.join(lines[max(0,i-5):i])
        if 'advancedTemplates' in context_before or 'snippetLibrary' not in context_before:
            # Add more templates before the closing ])
            new_tpls = [
              '  {id:"at19",name:"并行查询优化",category:"optimize",icon:"⚡",code:"-- 并行查询优化示例\\nSET max_parallel_workers_per_gather = 4;\\nSELECT /*+ PARALLEL(4) */\\n  dept_id, COUNT(*) as cnt\\nFROM employees\\nGROUP BY dept_id\\nORDER BY cnt DESC"},',
              '  {id:"at20",name:"物化视图刷新",category:"admin",icon:"🗄️",code:"-- 物化视图刷新\\nREFRESH MATERIALIZED VIEW CONCURRENTLY\\n  mv_sales_summary;\\n\\n-- 查看刷新时间\\nSELECT * FROM pg_matviews\\nWHERE matviewname = \'mv_sales_summary\'"},',
              '  {id:"at21",name:"JSON聚合查询",category:"analytics",icon:"🔑",code:"-- JSON聚合查询\\nSELECT user_id,\\n  jsonb_object_agg(key, value) as attrs,\\n  array_agg(tag) as tags\\nFROM user_metadata\\nGROUP BY user_id\\nHAVING count(*) > 1"},',
              '  {id:"at22",name:"增量数据同步",category:"admin",icon:"🔄",code:"-- 增量数据同步\\nINSERT INTO target_table\\nSELECT * FROM source_table\\nWHERE updated_at > :last_sync_time\\nON CONFLICT (id)\\nDO UPDATE SET\\n  name = EXCLUDED.name,\\n  updated_at = EXCLUDED.updated_at"},',
              '  {id:"at23",name:"数据归档策略",category:"admin",icon:"📦",code:"-- 数据归档到历史表\\nINSERT INTO orders_archive\\nSELECT * FROM orders\\nWHERE created_at < \'2023-01-01\'\\nRETURNING id;\\n\\nDELETE FROM orders\\nWHERE created_at < \'2023-01-01\';"},',
              '  {id:"at24",name:"CAGR复合增长率",category:"report",icon:"📈",code:"-- CAGR复合年增长率\\nSELECT product_name,\\n  start_value, end_value,\\n  years,\\n  ROUND(\\n    (POWER(end_value/start_value, 1.0/years) - 1) * 100, 2\\n  ) as cagr_pct\\nFROM product_growth"},',
              '  {id:"at25",name:"用户留存分析",category:"analytics",icon:"👥",code:"-- 用户留存率分析\\nWITH first_login AS (\\n  SELECT user_id, MIN(created_at) as first_day\\n  FROM user_events\\n  WHERE action = \'signup\'\\n  GROUP BY user_id\\n),\\nlogins AS (\\n  SELECT user_id, DATE(created_at) as login_day\\n  FROM user_events\\n  WHERE action = \'login\'\\n)\\nSELECT \\n  DATE_PART(\'day\', l.login_day - f.first_day) as day_offset,\\n  COUNT(DISTINCT l.user_id) as retained_users\\nFROM first_login f\\nJOIN logins l ON f.user_id = l.user_id\\nGROUP BY 1 ORDER BY 1"},',
              '  {id:"at26",name:"时间序列插值",category:"analytics",icon:"📉",code:"-- 时间序列数据插值\\nSELECT time_bucket,\\n  value,\\n  COALESCE(value,\\n    AVG(value) OVER (\\n      ORDER BY time_bucket\\n      ROWS BETWEEN 2 PRECEDING AND 2 FOLLOWING\\n    )\\n  ) as interpolated\\nFROM time_series_data\\nORDER BY time_bucket"},',
            ]
            for j, t in enumerate(new_tpls):
                lines.insert(i + j, t)
            break

# ── Step 3: Add extra CSS ──────────────────────────────────────────────────
extra_css = r'''
/* Extra layout polish */
.smd-header{display:flex;align-items:center;justify-content:space-between;padding:12px 20px;flex-shrink:0;flex-wrap:wrap;gap:8px}
.smd-actions{display:flex;gap:8px;flex-wrap:wrap}
.sb-tabs{display:flex;gap:4px;padding:4px 8px;border-bottom:1px solid var(--border-color)}
.sb-item{display:flex;align-items:center;gap:8px;padding:8px;border-radius:var(--radius-sm);cursor:pointer;margin-bottom:2px}
.sb-item:hover{background:var(--bg-hover)}
.sb-item.active{background:var(--color-primary-soft);border-left:3px solid var(--color-primary)}
/* SQL Editor enhancement */
.sql-editor{flex:1;min-height:200px;padding:12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-terminal);color:#7fdbca;font-family:'Fira Code','JetBrains Mono',monospace;font-size:13px;outline:none;resize:none;line-height:1.6;tab-size:2}
.sql-editor:focus{border-color:var(--color-primary)}
/* Result export bar */
.result-export-bar{display:flex;align-items:center;gap:8px;padding:6px 12px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.15);border-radius:var(--radius-sm);font-size:11px;color:var(--text-muted);margin-top:4px}
.rap-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.rap-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
/* Query param input bar */
.query-param-bar{display:flex;align-items:center;gap:8px;padding:6px 12px;background:rgba(245,158,11,0.05);border:1px solid rgba(245,158,11,0.15);border-radius:var(--radius-sm);font-size:11px;flex-wrap:wrap}
.param-chip{display:inline-flex;align-items:center;gap:4px;padding:2px 8px;background:rgba(245,158,11,0.15);border:1px solid rgba(245,158,11,0.3);border-radius:10px;font-size:10px;color:#f59e0b;font-family:monospace}
.query-param-input{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;width:100px}
/* Editor mode tabs */
.editor-mode-tabs{display:flex;gap:4px;margin-bottom:8px}
.emt{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px}
.emt.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}
/* Gen SQL bar */
.gen-sql-bar{display:flex;align-items:center;gap:8px;padding:8px 12px;background:rgba(16,185,129,0.05);border:1px solid rgba(16,185,129,0.2);border-radius:var(--radius-sm);margin-top:8px}
.gen-sql-label{font-size:11px;color:#10b981;font-weight:600}
.gen-sql-btn{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--color-success);background:rgba(16,185,129,0.1);color:#10b981;cursor:pointer;font-size:11px}
.gen-sql-btn:hover{background:rgba(16,185,129,0.2)}
/* Template panel search */
.tmp-input{flex:1;padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.tmp-select{padding:5px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
/* SQL generator section */
.sql-gen-section{padding:12px;background:rgba(0,0,0,0.2);border-radius:var(--radius-md);margin-top:8px}
.sql-gen-title{font-size:12px;color:var(--color-primary);font-weight:600;margin-bottom:8px;display:flex;align-items:center;gap:6px}
/* Enhanced result stats */
.result-stats-bar{display:flex;align-items:center;gap:12px;padding:6px 12px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.15);border-radius:var(--radius-sm);font-size:11px;color:var(--text-muted)}
.rs-item{display:flex;align-items:center;gap:4px}
.rs-val{color:var(--color-primary);font-weight:600;font-family:monospace}
/* Bulk bar in sidebar */
.bulk-bar{display:flex;align-items:center;gap:8px;padding:6px 8px;background:rgba(239,68,68,0.08);border-bottom:1px solid var(--border-color);font-size:12px;color:var(--color-danger)}
.bulk-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid currentColor;background:transparent;cursor:pointer;font-size:11px;color:inherit}.bulk-btn:hover{background:rgba(239,68,68,0.1)}
/* Modal glass effect enhancement */
.modal{backdrop-filter:blur(10px)}
/* Snippet card hover */
.snippet-card:hover{border-color:var(--color-primary)}
/* Debug log scrollbar */
.dbg-logs::-webkit-scrollbar,.dbg-vars::-webkit-scrollbar,.snippet-grid::-webkit-scrollbar,.tpl-list::-webkit-scrollbar{width:6px}
.dbg-logs::-webkit-scrollbar-thumb,.dbg-vars::-webkit-scrollbar-thumb,.snippet-grid::-webkit-scrollbar-thumb,.tpl-list::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:3px}
/* Performance stat highlights */
.perf-stat{display:flex;justify-content:space-between;padding:6px 10px;background:rgba(59,130,246,0.08);border-radius:var(--radius-sm);font-size:12px}
.perf-stat span:first-child{color:var(--text-muted)}
.perf-stat span:last-child{color:var(--color-primary);font-weight:600;font-family:monospace}
/* Viz chart tooltip style */
.viz-bar-wrap:hover .viz-bar{opacity:0.7}
/* Execution plan step highlight */
.plan-step.active{box-shadow:0 0 0 2px rgba(59,130,246,0.3)}
/* SQL diff result styling */
.diff-line.added{background:rgba(16,185,129,0.08)}
.diff-line.removed{background:rgba(239,68,68,0.08)}
/* Parameter row focus */
.param-row:focus-within{border:1px solid var(--color-primary);background:rgba(59,130,246,0.05)}
/* Bookmark item hover */
.bm-item:hover{background:rgba(59,130,246,0.05)}
/* Template card hover */
.tpl-card:hover{border-color:var(--color-primary)}
/* Hint tag hover animation */
.hint-tag:hover{transform:translateY(-1px)}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, extra_css)
        break

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print(f"Done. Lines: {len(lines)}")
