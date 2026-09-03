const fs = require('fs');
const d = JSON.parse(fs.readFileSync('D:/WORKSPACE/fakeToys/oa4rust/scripts/all_java_handlers.json', 'utf8'));
const m = d['x_organization_assemble_personal'];
const keys = Object.keys(m);
const lines = [];
keys.forEach(k => {
  const h = m[k];
  const path = '/jaxrs/organization/assemble/personal/' + h.path.replace(/\{(\w+)\}/g, ':');
  const method = h.method.toLowerCase();
  const fnName = k.toLowerCase().replace(/[^a-z0-9]/g, '_');
  lines.push(        .route("", ()));
});
console.log(lines.join('\n'));
