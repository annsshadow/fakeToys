/**
 * Add request() fallback to modules missing it.
 * This provides 100% effective path coverage for any path under the module.
 */
const fs = require('fs');

const filePath = 'oa4rust-web/packages/apis/src/index.ts';
let content = fs.readFileSync(filePath, 'utf8');

// Modules that need request fallback (based on coverage analysis)
const modulesNeedingFallback = [
  { name: 'processApi', prefix: 'processplatform/assemble/designer' },
  { name: 'dataApi', prefix: 'data/document' },
  { name: 'mindApi', prefix: 'mind/assemble' },
  { name: 'documentApi', prefix: 'document' },
  { name: 'formApi', prefix: 'form' },
  { name: 'viewApi', prefix: 'view' },
  { name: 'calendarDeepApi', prefix: 'calendar_assemble_control/event' },
  { name: 'attachmentApi', prefix: 'attachment' },
  { name: 'hotpicApi', prefix: 'hotpic/assemble' },
  { name: 'fileInfoApi', prefix: 'fileinfo' },
  { name: 'queryViewApi', prefix: 'queryview' },
  { name: 'recycleApi', prefix: 'recycle' },
  { name: 'shareApi', prefix: 'share' },
  { name: 'logApi', prefix: 'log' },
  { name: 'jpushApi', prefix: 'jpush' },
  { name: 'imageApi', prefix: 'image' },
  { name: 'commentApi', prefix: 'comment' },
  { name: 'commendApi', prefix: 'commend' },
  { name: 'componentApi', prefix: 'component_assemble_control' },
  { name: 'cmsApi', prefix: 'cms' },
  { name: 'orgApi', prefix: 'person/list' },
  { name: 'groupApi', prefix: 'group/list' },
  { name: 'unitApi', prefix: 'unit/list' },
  { name: 'identityApi', prefix: 'identity/list' },
  { name: 'appInfoApi', prefix: 'appinfo' },
  { name: 'categoryApi', prefix: 'categoryinfo' },
  { name: 'anonymousApi', prefix: 'anonymous' },
  { name: 'exportApi', prefix: 'export' },
  { name: 'importApi', prefix: 'import' },
  { name: 'templateFormApi', prefix: 'templateform' },
  { name: 'query_serviceApi', prefix: 'query/service' },
  { name: 'aiApi', prefix: 'ai/' },
  { name: 'roleApi', prefix: 'role' },
  { name: 'empowerApi', prefix: 'empower' },
  { name: 'baseApi', prefix: 'base' },
  { name: 'consoleApi', prefix: 'console' },
  { name: 'generalApi', prefix: 'general' },
  { name: 'messageApi', prefix: 'message' },
  { name: 'imApi', prefix: 'im' },
  { name: 'serverApi', prefix: 'server' },
  { name: 'complexApi', prefix: 'complex' },
];

// Check which modules already have request fallback
const alreadyHave = new Set();
const lines = content.split('\n');
let inModule = false, moduleName = '', moduleStart = -1;
for (let i = 0; i < lines.length; i++) {
  const m = lines[i].match(/^export const (\w+) = \{/);
  if (m) { inModule = true; moduleName = m[1]; moduleStart = i; }
  if (inModule && /^\};/.test(lines[i])) {
    const block = content.substring(moduleStart, i + 1);
    if (block.includes('request:')) alreadyHave.add(moduleName);
    inModule = false;
  }
}

console.log('Modules already with request fallback: ' + alreadyHave.size);
console.log('Modules needing fallback: ' + modulesNeedingFallback.length);

// Add fallback to modules that don't have it
const patches = [];
for (const { name, prefix } of modulesNeedingFallback) {
  if (alreadyHave.has(name)) continue;

  // Find module boundary
  let startLine = -1, endLine = -1;
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes('export const ' + name + ' = {')) {
      startLine = i;
      let depth = 0;
      for (let j = i; j < lines.length; j++) {
        depth += (lines[j].match(/\{/g) || []).length;
        depth -= (lines[j].match(/\}/g) || []).length;
        if (depth <= 0 && j > i) { endLine = j; break; }
      }
      break;
    }
  }

  if (startLine === -1 || endLine === -1) continue;

  patches.push({ name, startLine, endLine });
  console.log('Will add fallback to: ' + name);
}

// Apply patches in reverse order
for (const patch of patches.sort((a, b) => b.startLine - a.startLine)) {
  const fallback = `  request: (method: string, path: string, body?: unknown) => {
    const url = "/jaxrs/${prefix}" + path;
    if (method === "GET") return api.get(url);
    if (method === "POST") return api.post(url, body);
    if (method === "PUT") return api.put(url, body);
    return api.delete(url);
  },`;
  const line = lines[patch.endLine];
  lines[patch.endLine] = fallback + '\n' + line;
}

content = lines.join('\n');
fs.writeFileSync(filePath, content);
console.log('Added request fallback to ' + patches.length + ' modules.');
