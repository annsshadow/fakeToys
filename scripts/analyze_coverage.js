const fs = require('fs');
const execSync = require('child_process').execSync;

const rustPaths = new Set();
const rustFiles = execSync('find oa4rust -name "routes.rs"').toString().trim().split('\n');
for (const file of rustFiles) {
  const trimmed = file.trim();
  if (!fs.existsSync(trimmed)) continue;
  const rc = fs.readFileSync(trimmed, 'utf8');
  const lines = rc.split('\n');
  for (const line of lines) {
    const idx = line.indexOf('.route("');
    if (idx === -1) continue;
    let pathStart = idx + 8, depth = 1, endIdx = pathStart;
    while (endIdx < line.length && depth > 0) {
      if (line[endIdx] === '\\' && endIdx + 1 < line.length) { endIdx += 2; continue; }
      if (line[endIdx] === '"') { depth--; if (depth === 0) break; }
      endIdx++;
    }
    if (depth !== 0) continue;
    let path = line.substring(pathStart, endIdx);
    path = path.replace(/\\"/g, '');
    if (path.startsWith('/jaxrs/')) rustPaths.add(path);
  }
}

const content = fs.readFileSync('oa4rust-web/packages/apis/src/index.ts', 'utf8');
const apiModulePaths = new Set();
const regex = /api\.(get|post|put|delete)\(['"]([^'"]+)['"]/g;
let m;
while ((m = regex.exec(content)) !== null) apiModulePaths.add(m[2]);

const vuePaths = new Set();
for (const file of execSync('find oa4rust-web/apps/desktop/src/views -name "*.vue"').toString().trim().split('\n')) {
  const trimmed = file.trim();
  if (!fs.existsSync(trimmed)) continue;
  const vc = fs.readFileSync(trimmed, 'utf8');
  const matches = vc.match(/api\.(get|post|put|delete)\(['"]([^'"]+)['"]/g) || [];
  for (const match of matches) {
    const pathMatch = match.match(/api\.(get|post|put|delete)\(['"]([^'"]+)['"]/);
    if (pathMatch) vuePaths.add(pathMatch[2]);
  }
}

const allFrontend = new Set([...apiModulePaths, ...vuePaths]);
const missing = [...rustPaths].filter(p => !allFrontend.has(p));
const cleanMissing = missing.filter(p => !p.includes('\\"'));

console.log('Rust paths: ' + rustPaths.size);
console.log('Frontend paths: ' + allFrontend.size);
console.log('Total missing: ' + missing.length);
console.log('Clean missing (no escaped quotes): ' + cleanMissing.length);
console.log('Paths with Rust escaped quotes: ' + (missing.length - cleanMissing.length));

// Group clean missing by module
const modules = {};
for (const p of cleanMissing) {
  const parts = p.split('/').filter(Boolean);
  const key = parts.slice(0, 4).join('/');
  if (!modules[key]) modules[key] = 0;
  modules[key]++;
}
console.log('\nClean missing by module:');
Object.entries(modules).sort((a,b) => b[1]-a[1]).slice(0, 15).forEach(([k,v]) => console.log(k + ': ' + v));
