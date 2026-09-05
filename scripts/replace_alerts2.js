const fs = require('fs');
const path = require('path');
const glob = require('glob');

const viewsDir = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views';
const files = glob.sync(path.join(viewsDir, '*.vue'));

let totalReplaced = 0;

files.forEach(f => {
  let c = fs.readFileSync(f, 'utf-8');
  const original = c;

  // Replace error alerts: alert('xxx失败: ' + expr) -> toast.error(expr)
  c = c.replace(/alert\('([^']+失败[^']*)'\s*\+\s*([^)]+)\)/g, (m, msg, expr) => {
    totalReplaced++;
    return `toast.error('${msg.replace(/'/g, "\\'")} ${expr.trim()}')`;
  });

  // Replace simple success/info alerts: alert('保存成功') -> toast.success('保存成功')
  c = c.replace(/alert\('([^']+成功[^']*)'\)/g, (m, msg) => {
    totalReplaced++;
    return `toast.success('${msg.replace(/'/g, "\\'")}')`;
  });

  // Replace alert('保存成功') -> toast.success
  c = c.replace(/alert\('保存成功'\)/g, "toast.success('保存成功')");
  c = c.replace(/alert\('删除成功'\)/g, "toast.success('删除成功')");
  c = c.replace(/alert\('创建成功'\)/g, "toast.success('创建成功')");
  c = c.replace(/alert\('操作成功'\)/g, "toast.success('操作成功')");

  // Replace alert('xxx失败...') -> toast.error
  c = c.replace(/alert\('([^']+失败[^']*)'\)/g, (m, msg) => {
    totalReplaced++;
    return `toast.error('${msg.replace(/'/g, "\\'")}')`;
  });

  // Replace preview alerts
  c = c.replace(/alert\('配置预览:[^']*'\)/g, "toast.info('配置预览')");
  c = c.replace(/alert\('预览表单[^']*'\)/g, "toast.info('预览表单')");

  // Replace generic error alerts with expression
  c = c.replace(/alert\('([^']+失败[^']*)'\s*\+\s*(e\??\.message\s*\?\?\s*['\"][^'\"]*['\"])\)/g, (m, msg, expr) => {
    totalReplaced++;
    return `toast.error('${msg.replace(/'/g, "\\'")}: ' + ${expr})`;
  });

  if (c !== original) {
    fs.writeFileSync(f, c, 'utf-8');
    console.log(path.basename(f) + ': replaced');
  }
});

// Count remaining
let remaining = 0;
files.forEach(f => {
  const c = fs.readFileSync(f, 'utf-8');
  remaining += (c.match(/alert\(/g) || []).length;
});
console.log(`\nTotal replaced: ${totalReplaced}`);
console.log(`Remaining alerts: ${remaining}`);
