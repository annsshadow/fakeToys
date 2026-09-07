/**
 * Remove dead api_*_data ref/useQuery/watch triplets from ProcessWork.vue
 */
const fs = require('fs');
const filePath = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/ProcessWork.vue';
let content = fs.readFileSync(filePath, 'utf8');
const lines = content.split('\n');
const originalCount = lines.length;

const result = [];
let i = 0;
while (i < lines.length) {
  const line = lines[i];
  // Match: const api_XXX_data = ref<any[]>([]);
  if (/^const api_\w+_data\s*=\s*ref<any>\[\]\(\);/.test(line) && i + 2 < lines.length) {
    const next = lines[i + 1];
    const watch = lines[i + 2];
    // Verify next line has useQuery and watch line references the query
    if (next.includes('= useQuery({') && watch.includes(`watch(api_`) && watch.includes('_q,')) {
      // Skip these 3 lines (dead code)
      i += 3;
      continue;
    }
  }
  result.push(line);
  i++;
}

fs.writeFileSync(filePath, result.join('\n'), 'utf8');
console.log(`${filePath}: ${originalCount} → ${result.length} lines (-${originalCount - result.length})`);
