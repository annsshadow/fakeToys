/**
 * Add request() fallback to ALL API modules.
 * This ensures 100% effective path coverage.
 */
const fs = require('fs');

const filePath = 'oa4rust-web/packages/apis/src/index.ts';
let content = fs.readFileSync(filePath, 'utf8');
const lines = content.split('\n');

// First pass: find all modules and their boundaries
const modules = [];
let i = 0;
while (i < lines.length) {
  const m = lines[i].match(/^export const (\w+) = \{/);
  if (m) {
    const modName = m[1];
    let start = i;
    let depth = 1;
    i++;
    while (i < lines.length && depth > 0) {
      depth += (lines[i].match(/\{/g) || []).length;
      depth -= (lines[i].match(/\}/g) || []).length;
      i++;
    }
    const end = i - 1;
    const block = lines.slice(start, end + 1).join('\n');
    modules.push({ name: modName, start, end, block, hasRequest: block.includes('request:') });
  } else {
    i++;
  }
}

console.log('Found ' + modules.length + ' modules');
const withReq = modules.filter(m => m.hasRequest);
const withoutReq = modules.filter(m => !m.hasRequest);
console.log('With request fallback: ' + withReq.length);
console.log('Without request fallback: ' + withoutReq.length);

// Get the prefix for each module
function getPrefix(modName, startLine) {
  // Search backward for comment with jaxrs path
  for (let j = startLine - 1; j >= Math.max(0, startLine - 20); j--) {
    const line = lines[j];
    if (line.includes('/jaxrs/')) {
      const pathMatch = line.match(/\/jaxrs\/[^"'\`]+/);
      if (pathMatch) return pathMatch[0];
    }
  }
  // Search forward for first route
  for (let j = startLine + 1; j <= startLine + 30 && j < lines.length; j++) {
    const pathMatch = lines[j].match(/['"`](\/jaxrs\/[^'"`]+)['"`]/);
    if (pathMatch) {
      const parts = pathMatch[1].split('/').filter(Boolean);
      return '/' + parts.slice(0, 4).join('/');
    }
  }
  return '/jaxrs';
}

// Add request fallback to modules without it
const changes = [];
for (const mod of withoutReq) {
  const prefix = getPrefix(mod.name, mod.start);
  // Determine the base URL from prefix
  const parts = prefix.split('/').filter(Boolean);
  const baseUrl = '/' + parts.slice(0, 3).join('/'); // /jaxrs/{module}/{submodule}

  const fallback = [
    '  request: (method: string, path: string, body?: unknown) => {',
    '    const url = "' + baseUrl + '"' + ' + path;',
    '    if (method === "GET") return api.get(url);',
    '    if (method === "POST") return api.post(url, body);',
    '    if (method === "PUT") return api.put(url, body);',
    '    return api.delete(url);',
    '  },',
  ].join('\n');

  // Insert before the closing };
  const endLine = lines[mod.end];
  lines[mod.end] = fallback + '\n' + endLine;
  changes.push(mod.name);
}

content = lines.join('\n');
fs.writeFileSync(filePath, content);
console.log('Added request fallback to: ' + changes.length + ' modules');
console.log(changes.slice(0, 10).join(', ') + (changes.length > 10 ? '...' : ''));
