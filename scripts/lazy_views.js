/**
 * Convert static view imports to lazy-loaded dynamic imports
 * Reduces initial bundle size by deferring component loading
 */
const fs = require('fs');

const filePath = 'oa4rust-web/apps/desktop/src/main.ts';
let content = fs.readFileSync(filePath, 'utf8');

// Collect all view import names
const viewImportRegex = /import (\w+) from '\.\/views\/(\w+)\.vue'/g;
const viewNames = new Map();
let m;
while ((m = viewImportRegex.exec(content)) !== null) {
  viewNames.set(m[1], m[2]);
}
console.log('Found ' + viewNames.size + ' view imports');

// Remove import lines
const withoutImports = content.replace(/import \w+ from '\.\/views\/\w+\.vue';\n/g, '');

// Replace component: Name with lazy import using negative lookahead
let result = withoutImports;
for (const [name, filename] of viewNames) {
  // Use word boundary and negative lookahead to avoid partial matches
  // e.g., "QueryManager" won't match "QueryManagerDeep"
  const regex = new RegExp('component:\\s*' + name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') + '(?=[\\s,}])', 'g');
  result = result.replace(regex, "component: () => import('./views/" + filename + ".vue')");
}

fs.writeFileSync(filePath, result);
console.log('Converted all view components to lazy loading.');
