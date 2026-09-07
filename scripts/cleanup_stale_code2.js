/**
 * Second-pass cleanup: remove remaining mock blocks and dead api_jaxrs_pr blocks
 * in ProcessWork.vue (and similar large files).
 * Pattern: blocks where the URL contains mockputtopost/mockdeletetoget,
 * OR blocks where ref/useQuery/watch exist but _data is never read in template.
 */
const fs = require('fs');
const path = require('path');

const VIEWS_DIR = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views';

const files = [
  'ProcessWork.vue',
  'DocumentApp.vue',
  'ConfigDesignerApp.vue',
  'IMChat.vue',
  'QueryViewApp.vue',
  'UnitApp.vue',
  'PortalDesigner.vue',
];

const MOCK_URL_PATTERN = /mockputtopost|mockdeletetoget/;

for (const fileName of files) {
  const filePath = path.join(VIEWS_DIR, fileName);
  if (!fs.existsSync(filePath)) continue;

  let content = fs.readFileSync(filePath, 'utf8');
  const lines = content.split('\n');
  const originalLineCount = lines.length;
  let changed = true;
  let iterations = 0;

  while (changed && iterations < 20) {
    changed = false;
    iterations++;
    const newLines = [];
    let i = 0;

    while (i < lines.length) {
      const line = lines[i];

      // Pattern A: Remove blocks with mock URLs
      if (MOCK_URL_PATTERN.test(line)) {
        // Find start of this block (go back to const ..._ref)
        let start = i;
        for (let j = i; j >= Math.max(0, i - 15); j--) {
          if (lines[j].match(/^\s*const\s+\w+_ref\s*=\s*ref/)) {
            start = j;
            break;
          }
        }
        // Find end (closing });)
        let end = i;
        for (let j = i; j < Math.min(i + 20, lines.length); j++) {
          if (lines[j].includes('});') || lines[j].includes('  },')) {
            end = j + 1;
            break;
          }
        }
        // Skip empty lines before next block
        while (end < lines.length && lines[end].trim() === '') end++;
        newLines.push(`// [REMOVED] stale mock block ${lines[start].trim().slice(0, 60)}...`);
        i = end;
        changed = true;
        continue;
      }

      // Pattern B: Remove api_*_data ref/useQuery/watch triplets (dead code)
      const apiDataMatch = line.match(/^\s*const (api_\w+_data)\s*=\s*ref<any>\[\]\(\);$/);
      if (apiDataMatch && i + 2 < lines.length) {
        const nextLine = lines[i + 1];
        const watchLine = lines[i + 2];
        const hasUseQuery = nextLine && nextLine.includes('= useQuery({');
        const hasWatch = watchLine && watchLine.includes('watch(');
        if (hasUseQuery && hasWatch) {
          // Find end of watch block
          let end = i + 2;
          for (let j = i + 2; j < Math.min(i + 5, lines.length); j++) {
            if (lines[j].match(/;\s*\n?\s*\}/)) { end = j + 1; break; }
          }
          // Also handle single-line useQuery
          if (nextLine.includes('}});')) end = i + 2;
          newLines.push(lines[i]); // keep the ref declaration line (it's harmless)
          i += (end - i);
          changed = true;
          continue;
        }
      }

      newLines.push(line);
      i++;
    }

    lines.length = 0;
    lines.push(...newLines);
  }

  const removed = originalLineCount - lines.length;
  if (removed > 0) {
    fs.writeFileSync(filePath, lines.join('\n'), 'utf8');
    console.log(`${fileName}: ${originalLineCount} → ${lines.length} lines (-${removed})`);
  } else {
    console.log(`${fileName}: no changes`);
  }
}

console.log('Done.');
