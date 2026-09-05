const fs = require('fs');
const path = require('path');

const confirmCode = `
// Confirmation dialog (replaces window.confirm)
function confirmMsg(msg: string): Promise<boolean> {
  return new Promise(resolve => {
    const overlay = document.createElement('div')
    overlay.style.cssText = 'position:fixed;inset:0;background:rgba(0,0,0,.6);z-index:10000;display:flex;align-items:center;justify-content:center'
    const box = document.createElement('div')
    box.style.cssText = 'background:var(--bg-surface);border:1px solid var(--border-color);border-radius:var(--radius-lg);padding:24px;max-width:360px;width:90%;display:flex;flex-direction:column;gap:16px'
    box.innerHTML = '<p style="margin:0;color:var(--text-primary);font-size:14px">' + msg + '</p>' +
      '<div style="display:flex;gap:8px;justify-content:flex-end">' +
      '<button class="tc-cancel" style="padding:6px 16px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:transparent;color:var(--text-primary);cursor:pointer">取消</button>' +
      '<button class="tc-ok" style="padding:6px 16px;border-radius:var(--radius-md);border:none;background:var(--color-primary);color:#000;cursor:pointer;font-weight:600">确认</button>' +
      '</div>'
    overlay.appendChild(box)
    document.body.appendChild(overlay)
    const ok = () => { overlay.remove(); resolve(true) }
    const cancel = () => { overlay.remove(); resolve(false) }
    box.querySelector('.tc-ok').addEventListener('click', ok)
    box.querySelector('.tc-cancel').addEventListener('click', cancel)
    overlay.addEventListener('click', e => { if (e.target === overlay) cancel() })
  })
}
`;

const viewsToEnhance = [
  'PortalDesigner.vue', 'FormDesigner.vue', 'ConfigDesignerApp.vue',
  'FormApp.vue', 'MeetingApp.vue', 'ProcessDesigner.vue',
  'QueryDesigner.vue', 'AttendanceApp.vue', 'QueryViewApp.vue',
  'RecycleApp.vue', 'RoleManager.vue', 'AIChatApp.vue',
  'DocumentApp.vue', 'FileInfoApp.vue', 'FindDesignerApp.vue',
];

let totalAlerts = 0;
let totalConfirms = 0;
let totalPrompts = 0;

for (const fname of viewsToEnhance) {
  const fpath = path.join('D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views', fname);
  if (!fs.existsSync(fpath)) continue;

  let content = fs.readFileSync(fpath, 'utf-8');
  const original = content;

  // Add toast import
  if (!content.includes("from '@/utils/toast'") && content.includes('</script>')) {
    const importMatch = content.match(/import\s+.*\s+from\s+'vue'/);
    if (importMatch) {
      const endIdx = content.indexOf('\n', importMatch.index + importMatch[0].length);
      if (endIdx > 0) {
        content = content.slice(0, endIdx + 1) + "import { toast } from '@/utils/toast'\n" + content.slice(endIdx + 1);
      }
    }
  }

  // Replace alert() - simple string cases
  let alertCount = 0;
  content = content.replace(/alert\(\s*['"`]([^'"`]+)['"`]\s*\)/g, (m, msg) => {
    alertCount++;
    return `toast.info('${msg.replace(/'/g, "\\'")}')`;
  });

  // Replace alert() - template literal cases
  content = content.replace(/alert\(\s*`([^`]+)`\s*\)/g, (m, msg) => {
    alertCount++;
    return `toast.info(\`${msg}\`)`;
  });

  // Replace alert() - expression cases (keep as toast.info(expr))
  content = content.replace(/alert\(([^)'"]+[^)]*)\)/g, (m, expr) => {
    if (expr.trim().startsWith("'") || expr.trim().startsWith('"') || expr.trim().startsWith('`')) return m;
    alertCount++;
    return `toast.info(${expr.trim()})`;
  });

  // Replace confirm() with confirmMsg()
  const confirmCount = (content.match(/confirm\(/g) || []).length;
  content = content.replace(/confirm\(/g, 'confirmMsg(');

  // Replace prompt() - keep but track
  const promptCount = (content.match(/prompt\(/g) || []).length;

  // Add confirmMsg function if needed
  if (confirmCount > 0 && !content.includes('function confirmMsg')) {
    const scriptEnd = content.lastIndexOf('</script>');
    if (scriptEnd > 0) {
      content = content.slice(0, scriptEnd) + confirmCode + '\n</script>' + content.slice(scriptEnd + 9);
    }
  }

  if (content !== original) {
    fs.writeFileSync(fpath, content, 'utf-8');
    totalAlerts += alertCount;
    totalConfirms += confirmCount;
    totalPrompts += promptCount;
    console.log(`  ${fname}: alerts=${alertCount}, confirms=${confirmCount}, prompts=${promptCount}`);
  }
}

console.log(`\nTotal replaced: ${totalAlerts} alerts, ${totalConfirms} confirms`);

// Count remaining
let remainingAlerts = 0;
let remainingConfirms = 0;
const allVue = require('child_process').execSync('find D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views -name "*.vue"').toString().trim().split('\n');
for (const f of allVue) {
  try {
    const c = fs.readFileSync(f, 'utf-8');
    remainingAlerts += (c.match(/alert\(/g) || []).length;
    remainingConfirms += (c.match(/confirm\(/g) || []).length;
  } catch {}
}
console.log(`Remaining: alerts=${remainingAlerts}, confirms=${remainingConfirms}`);
