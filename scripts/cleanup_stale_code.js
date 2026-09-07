/**
 * Clean stale mock/dead code from Vue views — safe version.
 * Removes whole blocks (not partial) to avoid syntax errors.
 */
const fs = require('fs');
const path = require('path');

const VIEWS_DIR = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views';

const files = [
  'Dashboard.vue',
  'BBSForum.vue',
  'DocumentApp.vue',
  'ProcessWork.vue',
  'FormApp.vue',
  'DesignCenterApp.vue',
  'ConfigDesignerApp.vue',
  'AppInfoApp.vue',
  'CmsModuleApp.vue',
  'PortalDesigner.vue',
  'IMChat.vue',
  'QueryViewApp.vue',
  'UnitApp.vue',
];

let totalRemoved = 0;

for (const fileName of files) {
  const filePath = path.join(VIEWS_DIR, fileName);
  if (!fs.existsSync(filePath)) continue;

  let content = fs.readFileSync(filePath, 'utf8');
  const lines = content.split('\n');
  const originalCount = lines.length;
  const result = [];
  let i = 0;

  while (i < lines.length) {
    const line = lines[i];
    let skip = false;

    // Pattern A: mock blocks — find "const xxx_ref = ref" then skip until "});"
    if ((line.includes('mockdeletetoget_ref') || line.includes('mockputtopost_ref'))
      && line.match(/const\s+\w+_ref\s*=\s*ref/)) {
      let j = i;
      while (j < lines.length && !lines[j].includes('});')) j++;
      // skip the block plus trailing blank lines
      while (j < lines.length && lines[j].trim() === '') j++;
      i = j + 1;
      skip = true;
    }

    // Pattern B: api_*_data ref + useQuery + watch (3-line dead triplet)
    if (!skip && line.match(/^const api_\w+_data\s*=\s*ref<any>\[\]\(\);$/)) {
      const l1 = lines[i + 1] || '';
      const l2 = lines[i + 2] || '';
      if (l1.includes('= useQuery({') && l2.match(/^watch\(api_\w+_q,/)) {
        // skip all 3 lines, plus any following blank lines
        let j = i + 3;
        while (j < lines.length && lines[j].trim() === '') j++;
        i = j;
        skip = true;
      }
    }

    if (!skip) {
      result.push(line);
      i++;
    }
  }

  const removed = originalCount - result.length;
  if (removed > 0) {
    fs.writeFileSync(filePath, result.join('\n'), 'utf8');
    totalRemoved += removed;
    console.log(`${fileName}: ${originalCount} → ${result.length} (-${removed})`);
  } else {
    console.log(`${fileName}: no changes`);
  }
}

console.log(`\nTotal removed: ${totalRemoved} lines`);
