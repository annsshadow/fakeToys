#!/usr/bin/env python3
"""Replace alert()/confirm()/prompt() with toast/confirm utilities in top views."""
import re, os, glob

# Create confirm utility
confirm_code = '''
// Confirmation dialog (replaces window.confirm)
function confirmMsg(msg: string): Promise<boolean> {
  return new Promise(resolve => {
    const overlay = document.createElement('div')
    overlay.style.cssText = 'position:fixed;inset:0;background:rgba(0,0,0,.6);z-index:10000;display:flex;align-items:center;justify-content:center'
    const box = document.createElement('div')
    box.style.cssText = 'background:var(--bg-surface);border:1px solid var(--border-color);border-radius:var(--radius-lg);padding:24px;max-width:360px;width:90%;display:flex;flex-direction:column;gap:16px'
    box.innerHTML = `<p style="margin:0;color:var(--text-primary);font-size:14px">${msg}</p>
      <div style="display:flex;gap:8px;justify-content:flex-end">
        <button class="tc-cancel" style="padding:6px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer">取消</button>
        <button class="tc-ok" style="padding:6px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:#000;cursor:pointer;font-weight:600">确认</button>
      </div>`
    overlay.appendChild(box)
    document.body.appendChild(overlay)
    const ok = () => { overlay.remove(); resolve(true) }
    const cancel = () => { overlay.remove(); resolve(false) }
    box.querySelector('.tc-ok')!.addEventListener('click', ok)
    box.querySelector('.tc-cancel')!.addEventListener('click', cancel)
    overlay.addEventListener('click', e => { if (e.target === overlay) cancel() })
  })
}
'''

# Views to enhance with toast replacements
views_to_enhance = [
    'PortalDesigner.vue',
    'FormDesigner.vue',
    'ConfigDesignerApp.vue',
    'FormApp.vue',
    'MeetingApp.vue',
    'ProcessDesigner.vue',
    'QueryDesigner.vue',
    'AttendanceApp.vue',
    'QueryViewApp.vue',
    'RecycleApp.vue',
    'RoleManager.vue',
    'AIChatApp.vue',
    'DocumentApp.vue',
    'FileInfoApp.vue',
    'FindDesignerApp.vue',
]

replacements_made = 0
for fname in views_to_enhance:
    fpath = f'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/{fname}'
    if not os.path.exists(fpath):
        continue
    
    with open(fpath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    
    # Add import for toast
    has_toast_import = 'from \'@/utils/toast\'' in content or 'from "@/utils/toast"' in content
    has_script = '</script>' in content
    
    if not has_toast_import and has_script:
        # Find existing imports from vue
        import_match = re.search(r"import\s+.*\s+from\s+'vue'", content)
        if import_match:
            # Add toast import after vue imports
            end_idx = content.find('\n', import_match.end())
            if end_idx > 0:
                content = content[:end_idx] + "\nimport { toast } from '@/utils/toast'\n" + content[end_idx:]
    
    # Replace alert() calls - find patterns like alert('message') or alert("message")
    # Handle nested template literals and string concatenation
    alert_count = 0
    
    # Pattern 1: Simple string alerts: alert('...') or alert("...")
    def replace_alert_simple(m):
        nonlocal alert_count
        alert_count += 1
        msg = m.group(1)
        # Escape for JS template literal
        msg_escaped = msg.replace('`', '\\`').replace('${', '\\${')
        return f'toast.info(`{msg_escaped}`)'
    
    content = re.sub(r"alert\(['\"]([^'\"]*)['\"]\)", replace_alert_simple, content)
    
    # Pattern 2: Template literal alerts: alert(`...`)
    def replace_alert_template(m):
        nonlocal alert_count
        alert_count += 1
        return f'toast.info({m.group(1)})'
    
    content = re.sub(r'alert\(`([^`]*)`\)', replace_alert_template, content)
    
    # Pattern 3: String concatenation alerts: alert('msg' + var + 'more')
    def replace_alert_concat(m):
        nonlocal alert_count
        alert_count += 1
        expr = m.group(1).strip()
        # Try to convert to template literal
        if expr.startswith("'") and expr.endswith("'"):
            inner = expr[1:-1].replace('"', '\\"')
            return f'toast.info(`{inner}`)'
        return f'toast.info({expr})'
    
    content = re.sub(r'alert\(([^)]+)\)', replace_alert_concat, content)
    
    # Replace confirm() with confirmMsg()
    confirm_count = content.count('confirm(')
    content = content.replace('confirm(', 'confirmMsg(')
    
    # Replace prompt() - keep as-is for now (need user input)
    # But wrap in toast if it's just for display
    
    # Add confirmMsg function before </script>
    if confirm_count > 0 and 'function confirmMsg' not in content:
        script_end = content.rfind('</script>')
        if script_end > 0:
            # Find existing imports area to add confirmMsg
            # Add after the last function or before </script>
            content = content[:script_end] + confirm_code + '\n</script>' + content[script_end+9:]
    
    if content != original:
        with open(fpath, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"  {fname}: {alert_count} alerts → toast, {confirm_count} confirms → confirmMsg")
        replacements_made += 1

print(f"\nTotal views enhanced: {replacements_made}")

# Also check remaining alerts
remaining = 0
for f in glob.glob('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/*.vue'):
    c = open(f, encoding='utf-8', errors='ignore').read()
    remaining += len(re.findall(r'alert\(', c))
print(f"Remaining alert() calls: {remaining}")
