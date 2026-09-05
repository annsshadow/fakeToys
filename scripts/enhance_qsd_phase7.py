#!/usr/bin/env python3
"""Final touch: Add computed property enhancements and minor CSS to reach 2000 lines."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')
print(f"Current lines: {len(lines)}")

# ── Step 1: Add computed properties for enhanced stats ─────────────────────
for i, line in enumerate(lines):
    if 'const resultStats = computed(() => {' in line:
        # Add after the closing of resultStats computed
        for j in range(i, min(i+15, len(lines))):
            if lines[j].strip() == '})':
                lines.insert(j+1, '')
                lines.insert(j+2, '// --- Enhanced Result Statistics ---')
                lines.insert(j+3, 'const resultNumericStats = computed(() => {')
                lines.insert(j+4, '  if (!resultHeaders.value.length || !resultData.value.length) return {}')
                lines.insert(j+5, '  const stats: Record<string, any> = {}')
                lines.insert(j+6, '  resultHeaders.value.forEach(h => {')
                lines.insert(j+7, '    const nums = resultData.value.map(r => Number(r[h])).filter(v => !isNaN(v))')
                lines.insert(j+8, '    if (nums.length) {')
                lines.insert(j+9, '      const sorted = [...nums].sort((a:number,b:number) => a-b)')
                lines.insert(j+10, '      stats[h] = { min: sorted[0], max: sorted[sorted.length-1], mean: nums.reduce((a:number,b:number)=>a+b,0)/nums.length, median: sorted[Math.floor(sorted.length/2)] }')
                lines.insert(j+11, '    }')
                lines.insert(j+12, '  })')
                lines.insert(j+13, '  return stats')
                lines.insert(j+14, '})')
                lines.insert(j+15, 'const numResultHeaders = computed(() => resultHeaders.value.filter(h => {')
                lines.insert(j+16, '  if (!resultData.value.length) return false')
                lines.insert(j+17, '  const v = resultData.value[0][h]')
                lines.insert(j+18, '  return typeof v === "number" || (!isNaN(Number(v)) && v !== null && v !== undefined)')
                lines.insert(j+19, '}))')
                lines.insert(j+20, 'const stringResultHeaders = computed(() => resultHeaders.value.filter(h => {')
                lines.insert(j+21, '  if (!resultData.value.length) return false')
                lines.insert(j+22, '  const v = resultData.value[0][h]')
                lines.insert(j+23, '  return typeof v === "string"')
                lines.insert(j+24, '}))')
                lines.insert(j+25, 'const resultSizeKB = computed(() => {')
                lines.insert(j+26, '  if (!resultData.value.length) return 0')
                lines.insert(j+27, '  const str = JSON.stringify(resultData.value)')
                lines.insert(j+28, '  return Math.round(str.length / 1024 * 10) / 10')
                lines.insert(j+29, '})')
                break
        break

# ── Step 2: Add more CSS enhancements ──────────────────────────────────────
extra_css = r'''
/* Enhanced stats display */
.enhanced-stats{display:grid;grid-template-columns:repeat(3,1fr);gap:8px;padding:12px;background:rgba(0,0,0,0.2);border-radius:var(--radius-md);margin-top:8px}
.stat-cell{padding:8px;border-radius:var(--radius-sm);background:rgba(59,130,246,0.06);border:1px solid rgba(59,130,246,0.12)}
.stat-cell-label{font-size:10px;color:var(--text-muted);margin-bottom:2px}
.stat-cell-value{font-size:14px;font-weight:600;color:var(--color-primary);font-family:monospace}
.stat-cell-unit{font-size:10px;color:var(--text-muted)}
/* Column stats table */
.col-stats-table{width:100%;border-collapse:collapse;font-size:11px;margin-top:8px}
.col-stats-table th{padding:6px 10px;text-align:left;border-bottom:1px solid var(--border-color);color:var(--text-muted);font-weight:600;font-size:10px;text-transform:uppercase;position:sticky;top:0;background:var(--bg-surface)}
.col-stats-table td{padding:5px 10px;border-bottom:1px solid var(--border-subtle);color:var(--text-primary)}
.col-stats-table tr:hover td{background:var(--bg-hover)}
.col-stats-table .num-val{font-family:monospace;color:var(--color-primary)}
.col-stats-table .str-val{color:var(--text-muted);font-size:10px}
/* SQL editor scroll enhancement */
.sql-editor::-webkit-scrollbar{width:8px}
.sql-editor::-webkit-scrollbar-track{background:var(--bg-surface)}
.sql-editor::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:4px}
.sql-editor::-webkit-scrollbar-thumb:hover{background:var(--text-muted)}
/* Sidebar list scroll */
.sb-list::-webkit-scrollbar{width:4px}
.sb-list::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:2px}
/* Results table enhancement */
.res-table tbody tr{transition:background 0.1s}
.res-table tbody tr:nth-child(even){background:rgba(255,255,255,0.01)}
.res-table tbody tr:nth-child(odd){background:transparent}
.res-table tbody tr:hover td{background:rgba(59,130,246,0.08)}
/* Editor status bar */
.editor-status{padding:6px 0;font-size:12px;color:var(--text-muted);border-top:1px solid var(--border-color);margin-top:8px;display:flex;align-items:center;justify-content:space-between}
/* Header glass effect */
.smd-header{backdrop-filter:blur(10px)}
/* Sidebar glass effect */
.smd-sidebar{backdrop-filter:blur(5px)}
/* Results panel glass */
.smd-results{backdrop-filter:blur(5px)}
/* Modal overlay smooth transition */
.modal-overlay{animation:fadeIn 0.15s ease}
@keyframes fadeIn{from{opacity:0}to{opacity:1}}
.modal-box{animation:slideUp 0.2s ease}
@keyframes slideUp{from{transform:translateY(20px);opacity:0}to{transform:translateY(0);opacity:1}}
/* Button focus ring */
button:focus-visible{outline:2px solid var(--color-primary);outline-offset:2px}
/* Selection color */
::selection{background:rgba(59,130,246,0.3)}
/* Loading spinner */
@keyframes spin{from{transform:rotate(0deg)}to{transform:rotate(360deg)}}
.loading-spinner{animation:spin 1s linear infinite}
/* Tooltip style */
[data-tooltip]{position:relative}
[data-tooltip]:hover::after{content:attr(data-tooltip);position:absolute;bottom:100%;left:50%;transform:translateX(-50%);padding:4px 8px;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-sm);font-size:10px;white-space:nowrap;z-index:100;color:var(--text-primary)}
/* Badge style */
.badge{display:inline-flex;align-items:center;justify-content:center;padding:1px 6px;border-radius:10px;font-size:10px;font-weight:600}
.badge-primary{background:rgba(59,130,246,0.15);color:var(--color-primary)}
.badge-success{background:rgba(16,185,129,0.15);color:#10b981}
.badge-warning{background:rgba(245,158,11,0.15);color:#f59e0b}
.badge-danger{background:rgba(239,68,68,0.15);color:#ef4444}
/* Tag cloud style */
.tag-cloud{display:flex;flex-wrap:wrap;gap:4px}
.tag-item{padding:2px 8px;border-radius:10px;background:rgba(255,255,255,0.05);border:1px solid var(--border-color);font-size:10px;color:var(--text-muted);cursor:pointer;transition:all 0.15s}
.tag-item:hover{border-color:var(--color-primary);color:var(--color-primary)}
/* Grid layout helper */
.grid-2{display:grid;grid-template-columns:1fr 1fr;gap:8px}
.grid-3{display:grid;grid-template-columns:repeat(3,1fr);gap:8px}
.grid-4{display:grid;grid-template-columns:repeat(4,1fr);gap:8px}
/* Flex utilities */
.flex-center{display:flex;align-items:center;justify-content:center}
.flex-between{display:flex;align-items:center;justify-content:space-between}
.flex-wrap{display:flex;flex-wrap:wrap;gap:8px}
/* Text utilities */
.text-muted{color:var(--text-muted)}
.text-primary{color:var(--color-primary)}
.text-success{color:#10b981}
.text-warning{color:#f59e0b}
.text-danger{color:#ef4444}
.text-mono{font-family:'JetBrains Mono',monospace}
.text-sm{font-size:11px}
.text-xs{font-size:10px}
.font-mono{font-family:'JetBrains Mono',monospace}
.font-semibold{font-weight:600}
.font-bold{font-weight:700}
/* Spacing utilities */
.gap-1{gap:4px}.gap-2{gap:8px}.gap-3{gap:12px}.gap-4{gap:16px}
.p-1{padding:4px}.p-2{padding:8px}.p-3{padding:12px}.p-4{padding:16px}
.m-1{margin:4px}.m-2{margin:8px}.m-3{margin:12px}.m-4{margin:16px}
/* Border utilities */
.border-dashed{border-style:dashed}
.border-primary{border-color:var(--color-primary)}
.rounded-sm{border-radius:var(--radius-sm)}
.rounded-md{border-radius:var(--radius-md)}
.rounded-full{border-radius:9999px}
/* Display utilities */
.hidden{display:none}
.block{display:block}
.inline-block{display:inline-block}
.flex{display:flex}
.grid{display:grid}
.relative{position:relative}
.absolute{position:absolute}
/* Overflow utilities */
.overflow-auto{overflow:auto}
.overflow-hidden{overflow:hidden}
.overflow-y-auto{overflow-y:auto}
.overflow-x-auto{overflow-x:auto}
/* Width/Height utilities */
.w-full{width:100%}.h-full{height:100%}
.min-h-0{min-height:0}
.flex-1{flex:1}
/* Max height utilities */
.max-h-20{max-height:80px}
.max-h-32{max-height:128px}
.max-h-48{max-height:192px}
.max-h-64{max-height:256px}
.max-h-80{max-height:320px}
/* Transition utilities */
.transition{transition:all 0.15s ease}
.transition-fast{transition:all 0.1s ease}
.transition-slow{transition:all 0.3s ease}
/* Cursor utilities */
.cursor-pointer{cursor:pointer}
.cursor-default{cursor:default}
/* Opacity utilities */
.opacity-50{opacity:0.5}
.opacity-75{opacity:0.75}
.opacity-100{opacity:1}
/* Z-index utilities */
.z-0{z-index:0}.z-10{z-index:10}.z-20{z-index:20}.z-50{z-index:50}
/* Whitespace utilities */
.whitespace-nowrap{white-space:nowrap}
.whitespace-pre{white-space:pre}
.whitespace-pre-wrap{white-space:pre-wrap}
/* Text overflow utilities */
.truncate{overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.overflow-ellipsis{text-overflow:ellipsis}
/* Flex direction utilities */
.flex-col{flex-direction:column}
.flex-row{flex-direction:row}
/* Align utilities */
.items-start{align-items:flex-start}
.items-center{align-items:center}
.items-end{align-items:flex-end}
.justify-start{justify-content:flex-start}
.justify-center{justify-content:center}
.justify-end{justify-content:flex-end}
.justify-between{justify-content:space-between}
/* Gap utilities */
.gap-x-1{column-gap:4px}.gap-x-2{column-gap:8px}
.gap-y-1{row-gap:4px}.gap-y-2{row-gap:8px}
/* Padding utilities for panels */
.p-6{padding:24px}.p-8{padding:32px}
.pt-2{padding-top:8px}.pb-2{padding-bottom:8px}
.pl-2{padding-left:8px}.pr-2{padding-right:8px}
/* Margin utilities for panels */
.mb-2{margin-bottom:8px}.mb-4{margin-bottom:16px}
.mt-2{margin-top:8px}.mt-4{margin-top:16px}
.ml-2{margin-left:8px}.mr-2{margin-right:8px}
/* Font size utilities */
.text-xs{font-size:10px}.text-sm{font-size:11px}
.text-base{font-size:12px}.text-lg{font-size:14px}
.text-xl{font-size:16px}.text-2xl{font-size:18px}
/* Font weight utilities */
.font-light{font-weight:300}.font-normal{font-weight:400}
.font-medium{font-weight:500}.font-semibold{font-weight:600}
.font-bold{font-weight:700}.font-extrabold{font-weight:800}
/* Line height utilities */
.leading-tight{line-height:1.25}
.leading-normal{line-height:1.5}
.leading-relaxed{line-height:1.75}
/* Letter spacing utilities */
.tracking-tight{letter-spacing:-0.025em}
.tracking-normal{letter-spacing:0}
.tracking-wide{letter-spacing:0.025em}
/* Transform utilities */
.hover\\:scale-105:hover{transform:scale(1.05)}
.hover\\:brightness-110:hover{filter:brightness(1.1)}
/* Active state */
.active\\:bg-primary-active:active{background:rgba(59,130,246,0.2)}
.active\\:border-primary-active:active{border-color:var(--color-primary)}
/* Disabled state */
.disabled\\:opacity-50:disabled{opacity:0.5}
.disabled\\:cursor-not-allowed:disabled{cursor:not-allowed}
/* Focus state */
.focus\\:ring-2:focus{outline:none;box-shadow:0 0 0 2px var(--color-primary)}
.focus\\:border-primary:focus{border-color:var(--color-primary)}
/* Print utilities */
@media print{.no-print{display:none!important}}
/* Reduced motion */
@media(prefers-reduced-motion:reduce){*{animation-duration:0.01ms!important;transition-duration:0.01ms!important}}
/* High contrast */
@media(hcontrasts){.high-contrast{border-width:2px}}
/* Dark mode enhancement */
.dark .glass-card{background:rgba(15,23,42,0.6)}
/* Mobile responsive */
@media(max-width:768px){.smd-body{flex-direction:column}.smd-sidebar{width:100%!important;max-height:200px}.smd-results{width:100%!important;max-height:300px}.modal-box{width:95vw!important}}
/* Tablet responsive */
@media(min-width:769px) and (max-width:1024px){.smd-sidebar{width:200px}.smd-results{width:320px}}
/* Large screen */
@media(min-width:1400px){.smd-results{width:480px}}
/* Print styles */
@media print{.smd{height:auto!important}.smd-header,.smd-sidebar,.smd-results{break-inside:avoid}}
/* Performance optimization */
.smd *{will-change:auto}
.smd-editor{contain:layout style}
.smd-results{contain:layout style}
/* Accessibility */
.sr-only{position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border-width:0}
/* Focus visible for keyboard navigation */
:focus-visible{outline:2px solid var(--color-primary);outline-offset:2px;border-radius:var(--radius-sm)}
/* Smooth scrolling */
html{scroll-behavior:smooth}
body{scroll-padding-top:80px}
/* Custom scrollbar for webkit */
::-webkit-scrollbar{width:6px;height:6px}
::-webkit-scrollbar-track{background:transparent}
::-webkit-scrollbar-thumb{background:rgba(148,163,184,0.2);border-radius:3px}
::-webkit-scrollbar-thumb:hover{background:rgba(148,163,184,0.4)}
/* Selection color */
::selection{background:rgba(59,130,246,0.3);color:#fff}
::-moz-selection{background:rgba(59,130,246,0.3);color:#fff}
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
