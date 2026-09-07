/**
 * Auto-generate missing API methods from Rust routes.rs files.
 * Properly handles Rust's {\"paramName\"} -> {p} conversion
 */
const fs = require('fs');
const execSync = require('child_process').execSync;

const filePath = 'oa4rust-web/packages/apis/src/index.ts';
let content = fs.readFileSync(filePath, 'utf8');

// Read all Rust routes with proper escape handling
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
    // Remove Rust named param annotations: {\"name\"} -> {p}
    // This handles the pattern where Rust uses {\"paramName\"} in route strings
    path = path.replace(/\{\\?"[^\\"]*\\?"\}/g, '{p}');
    // Also clean any remaining stray backslashes
    path = path.replace(/\\/g, '');
    if (path.startsWith('/jaxrs/')) {
      rustPaths.add(path);
    }
  }
}

console.log('Extracted ' + rustPaths.size + ' Rust routes.');

// Verify no broken paths remain
const broken = [...rustPaths].filter(p => p.includes('{\\"') || p.endsWith('{'));
if (broken.length > 0) {
  console.log('WARNING: ' + broken.length + ' broken paths found:');
  broken.slice(0, 5).forEach(p => console.log('  ' + p));
}

// Read existing frontend paths
const apiModulePaths = new Set();
const apiRegex = /api\.(get|post|put|delete)\(['"]([^'"]+)['"]/g;
let m;
while ((m = apiRegex.exec(content)) !== null) apiModulePaths.add(m[2]);

const vuePaths = new Set();
const viewFiles = execSync('find oa4rust-web/apps/desktop/src/views -name "*.vue"').toString().trim().split('\n');
for (const file of viewFiles) {
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

function inferMethod(path) {
  const lower = path.toLowerCase();
  if (lower.includes('delete') || lower.includes('/remove')) return 'delete';
  if (lower.includes('update') || lower.includes('/save') || lower.includes('/modify')) return 'put';
  if (lower.includes('create') || lower.includes('/start') || lower.includes('/submit') || lower.includes('/apply')) return 'post';
  if (lower.includes('touch') || lower.includes('/execute')) return 'post';
  return 'get';
}

function makeMethodName(path) {
  const parts = path.split('/').filter(Boolean).slice(3);
  return parts.map(p => p.replace(/{p}/g, '')).join('_');
}

// Group missing paths by target module
const patches = {};

for (const p of rustPaths) {
  if (allFrontend.has(p)) continue;
  const parts = p.split('/').filter(Boolean);
  if (parts.length < 4) continue;
  const key = parts.slice(0, 4).join('/');

  let target = null;
  if (key.startsWith('jaxrs/processplatform/assemble/surface')) target = 'processplatformSurfaceApi';
  else if (key.startsWith('jaxrs/processplatform/assemble/designer')) target = 'processApi';
  else if (key.startsWith('jaxrs/processplatform/service/processing')) target = 'processServiceApi';
  else if (key.startsWith('jaxrs/attendance/assemble/control')) target = 'attendanceControlApi';
  else if (key.startsWith('jaxrs/meeting/assemble/control')) target = 'meetingControlApi';
  else if (key.startsWith('jaxrs/general/assemble/control')) target = 'generalControlApi';
  else if (key.startsWith('jaxrs/message/assemble/communicate')) target = 'messageCommunicateApi';
  else if (key.startsWith('javrs/person/list') || key.startsWith('javrs/group/list') || key.startsWith('javrs/unit/list')) target = 'orgApi';
  else if (key.startsWith('jaxrs/bbs/assemble/control')) target = 'extraApis';
  else if (key.startsWith('jaxrs/mind/assemble/control')) target = 'mindApi';
  else if (key.startsWith('jaxrs/file/assemble/control') || key.startsWith('jaxrs/file/attachment') || key.startsWith('javrs/file/attachment2') || key.startsWith('javrs/file/core') || key.startsWith('javrs/file/folder') || key.startsWith('javrs/file/folder2') || key.startsWith('javrs/file/list') || key.startsWith('javrs/file/share') || key.startsWith('javrs/file/upload') || key.startsWith('javrs/file/editor') || key.startsWith('javrs/file/clean') || key.startsWith('javrs/file/copy') || key.startsWith('javrs/file/complex') || key.startsWith('javrs/file/permission') || key.startsWith('javrs/file/{flag}')) target = 'fileControlApi';
  else if (key.startsWith('jaxrs/data/document')) target = 'dataApi';
  else if (key.startsWith('javrs/surface/appdict') || key.startsWith('javrs/design/appdict')) target = 'portalSurfaceApi';
  else if (key.startsWith('javrs/anonymous/surface/appdict')) target = 'anonymousApi';
  else if (key.startsWith('jaxrs/program_center/')) target = 'programCenterApi';
  else if (key.startsWith('javrs/ai/') || key.startsWith('javrs/ai_assemble_control/')) target = 'extraApis';
  else if (key.startsWith('jaxrs/hotpic/assemble/control') || key.startsWith('javrs/hotpic_assemble_control')) target = 'hotpicApi';
  else if (key.startsWith('jaxrs/calendar_assemble_control')) target = 'calendarDeepApi';
  else if (key.startsWith('jaxrs/categoryinfo/filter/list') || key.startsWith('javrs/categoryinfo/list') || key.startsWith('javrs/categoryinfo/{id}') || key.startsWith('javrs/categoryinfo/alias') || key.startsWith('javrs/categoryinfo/erase')) target = 'categoryApi';
  else if (key.startsWith('javrs/appinfo/list') || key.startsWith('javrs/appinfo/filter/list') || key.startsWith('javrs/appinfo/erase') || key.startsWith('javrs/appinfo/alias') || key.startsWith('javrs/appinfo/get/user') || key.startsWith('javrs/appinfo/{id}/permission') || key.startsWith('javrs/appinfo/{id}/control') || key.startsWith('javrs/appinfo/{id}/icon') || key.startsWith('javrs/appconfig/')) target = 'appInfoApi';
  else if (key.startsWith('javrs/comment/list')) target = 'commentApi';
  else if (key.startsWith('javrs/commend/list')) target = 'commendApi';
  else if (key.startsWith('javrs/document/filter/list') || key.startsWith('javrs/document/publish') || key.startsWith('javrs/document/batch') || key.startsWith('javrs/document/cipher') || key.startsWith('javrs/document/draft/list') || key.startsWith('javrs/document/achive') || key.startsWith('javrs/anonymous/document')) target = 'documentApi';
  else if (key.startsWith('javrs/fileinfo/download') || key.startsWith('javrs/fileinfo/upload') || key.startsWith('javrs/fileinfo/edit') || key.startsWith('javrs/fileinfo/update') || key.startsWith('javrs/fileinfo/{id}') || key.startsWith('javrs/fileinfo/batch') || key.startsWith('javrs/fileinfo/copy') || key.startsWith('javrs/fileinfo/replace') || key.startsWith('javrs/fileinfo/list') || key.startsWith('javrs/anonymous/fileinfo')) target = 'fileInfoApi';
  else if (key.startsWith('javrs/form/v2') || key.startsWith('javrs/form/filter/list') || key.startsWith('javrs/form/{id}') || key.startsWith('javrs/form/{id}/appinfo') || key.startsWith('javrs/form/{id}/mock') || key.startsWith('javrs/form/list') || key.startsWith('javrs/anonymous/form')) target = 'formApi';
  else if (key.startsWith('javrs/script/list') || key.startsWith('javrs/templateform/list') || key.startsWith('javrs/scriptversion/{id}') || key.startsWith('javrs/scriptversion/list') || key.startsWith('javrs/searchfilter/list') || key.startsWith('javrs/express')) target = 'extraApis';
  else if (key.startsWith('javrs/view/viewdata') || key.startsWith('javrs/view/list') || key.startsWith('javrs/view/{id}')) target = 'viewApi';
  else if (key.startsWith('javrs/viewcategory/list') || key.startsWith('javrs/viewcategory/{id}')) target = 'viewCategoryApi';
  else if (key.startsWith('javrs/viewfieldconfig/list') || key.startsWith('javrs/viewfieldconfig/{id}')) target = 'viewFieldConfigApi';
  else if (key.startsWith('javrs/export/appInfo') || key.startsWith('javrs/import/appInfo') || key.startsWith('javrs/output/{appInfoFlag}')) target = 'exportApi';
  else if (key.startsWith('javrs/cms_control/get') || key.startsWith('javrs/cms/view/publish') || key.startsWith('javrs/cms/view/unpublish')) target = 'cmsApi';
  else if (key.startsWith('javrs/component_assemble_control/get') || key.startsWith('javrs/component_assemble_control/create') || key.startsWith('javrs/component_assemble_control/save') || key.startsWith('javrs/component_assemble_control/delete') || key.startsWith('javrs/component_assemble_control/component') || key.startsWith('javrs/component_assemble_control/list')) target = 'componentApi';
  else if (key.startsWith('javrs/console/logs') || key.startsWith('javrs/console/cache') || key.startsWith('javrs/console/metric')) target = 'consoleApi';
  else if (key.startsWith('javrs/person') || key.startsWith('javrs/group') || key.startsWith('javrs/identity/list') || key.startsWith('javrs/role/list') || key.startsWith('javrs/personal/info') || key.startsWith('javrs/personal/update') || key.startsWith('javrs/personal/detail') || key.startsWith('javrs/person/icon') || key.startsWith('javrs/icon/{person}')) target = 'orgApi';
  else if (key.startsWith('javrs/unit/identity') || key.startsWith('javrs/unitduty/list')) target = 'unitDutyApi';
  else if (key.startsWith('javrs/personattribute')) target = 'personAttributeApi';
  else if (key.startsWith('javrs/unitattribute')) target = 'unitAttributeApi';
  else if (key.startsWith('javrs/empower/log')) target = 'empowerLogApi';
  else if (key.startsWith('javrs/empower/list')) target = 'empowerApi';
  else if (key.startsWith('javrs/jpush/device') || key.startsWith('javrs/jpush/template') || key.startsWith('javrs/jpush_assemble_control')) target = 'jpushApi';
  else if (key.startsWith('javrs/image/encode') || key.startsWith('javrs/image/resize')) target = 'imageApi';
  else if (key.startsWith('javrs/log/list') || key.startsWith('javrs/log/filter') || key.startsWith('javrs/log/{id}')) target = 'logApi';
  else if (key.startsWith('javrs/queryview/flag')) target = 'queryViewApi';
  else if (key.startsWith('javrs/recycle/resume') || key.startsWith('javrs/recycle/delete') || key.startsWith('javrs/recycle/{id}')) target = 'recycleApi';
  else if (key.startsWith('javrs/share/shield') || key.startsWith('javrs/share/list') || key.startsWith('javrs/share/{id}')) target = 'shareApi';
  else if (key.startsWith('javrs/general/worktime')) target = 'generalControlApi';
  else if (key.startsWith('javrs/correlation/doc') || key.startsWith('javrs/correlation/type')) target = 'correlationApi';
  else if (key.startsWith('javrs/viewrecord/document')) target = 'documentApi';
  else if (key.startsWith('javrs/porta/{id}') || key.startsWith('javrs/porta/page')) target = 'portalApi';
  else if (key.startsWith('javrs/work/{id}') || key.startsWith('javrs/gateway/{work_id}') || key.startsWith('javrs/process/state') || key.startsWith('javrs/process/designer/route') || key.startsWith('javrs/process/task/count') || key.startsWith('javrs/process/read/count') || key.startsWith('javrs/process/list/ids') || key.startsWith('javrs/process/{flag}') || key.startsWith('javrs/process/record')) target = 'processApi';
  else if (key.startsWith('javrs/query/service/neural')) target = 'query_serviceApi';
  else if (key.startsWith('javrs/templateform/{id}')) target = 'templateFormApi';
  else if (key.startsWith('javrs/formversion/{id}') || key.startsWith('javrs/formversion/list')) target = 'formApi';
  else if (key.startsWith('javrs/docpermission')) target = 'documentApi';
  else if (key.startsWith('javrs/complex/folder')) target = 'complexApi';
  else if (key.startsWith('javrs/folder') || key.startsWith('javrs/folder2')) target = 'fileControlApi';
  else continue;

  if (!patches[target]) patches[target] = new Set();
  patches[target].add(p);
}

// Find module boundaries
const lines = content.split('\n');
const moduleBoundaries = {};
for (let i = 0; i < lines.length; i++) {
  const match = lines[i].match(/^export const (\w+) = \{/);
  if (match) {
    const name = match[1];
    let depth = 0;
    for (let j = i; j < lines.length; j++) {
      depth += (lines[j].match(/\{/g) || []).length;
      depth -= (lines[j].match(/\}/g) || []).length;
      if (depth <= 0 && j > i) {
        moduleBoundaries[name] = { start: i, end: j };
        break;
      }
    }
  }
}

let totalAdded = 0;
const patchKeys = Object.keys(patches).sort((a, b) => (moduleBoundaries[b]?.end || 0) - (moduleBoundaries[a]?.end || 0));

for (const mod of patchKeys) {
  const boundary = moduleBoundaries[mod];
  if (!boundary) continue;

  const patterns = [...patches[mod]];
  const methods = patterns.map(p => {
    const method = inferMethod(p);
    const name = makeMethodName(p) || 'request';
    const cleanName = name.replace(/[^a-zA-Z0-9_]/g, '_') || 'req';
    if (method === 'get') return `  ${cleanName}: (params?: Record<string, unknown>) => api.get("${p}", { params }),`;
    if (method === 'post') return `  ${cleanName}: (data?: unknown) => api.post("${p}", data),`;
    if (method === 'put') return `  ${cleanName}: (data: unknown) => api.put("${p}", data),`;
    return `  ${cleanName}: () => api.delete("${p}"),`;
  });

  if (methods.length === 0) continue;

  const insertText = methods.join('\n');
  const endLine = lines[boundary.end];
  lines[boundary.end] = insertText + '\n' + endLine;
  totalAdded += methods.length;
}

content = lines.join('\n');
fs.writeFileSync(filePath, content);

// Calculate coverage
const afterPaths = new Set();
const afterRegex = /api\.(get|post|put|delete)\(['"]([^'"]+)['"]/g;
let m2;
const afterContent = fs.readFileSync(filePath, 'utf8');
while ((m2 = afterRegex.exec(afterContent)) !== null) afterPaths.add(m2[2]);
const afterVuePaths = new Set();
for (const vf of viewFiles) {
  const vt = vf.trim();
  if (!fs.existsSync(vt)) continue;
  const vc = fs.readFileSync(vt, 'utf8');
  const vm = vc.match(/api\.(get|post|put|delete)\(['"]([^'"]+)['"]/g) || [];
  for (const mm of vm) {
    const mp = mm.match(/api\.(get|post|put|delete)\(['"]([^'"]+)['"]/);
    if (mp) afterVuePaths.add(mp[2]);
  }
}
const afterAll = new Set([...afterPaths, ...afterVuePaths]);
const coverage = Math.round(afterAll.size / rustPaths.size * 100);
console.log('Generated ' + totalAdded + ' new API methods.');
console.log('Modules patched: ' + patchKeys.filter(k => patches[k].size > 0).length);
console.log('Rust paths: ' + rustPaths.size);
console.log('Frontend paths: ' + afterAll.size);
console.log('Coverage: ' + coverage + '%');
