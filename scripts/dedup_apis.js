/**
 * Deduplicate properties in oa4rust-web/packages/apis/src/index.ts
 * Fixes TS1117 (duplicate properties) and TS7006 (implicit any) errors
 */
const fs = require('fs');

const filePath = 'oa4rust-web/packages/apis/src/index.ts';
let content = fs.readFileSync(filePath, 'utf8');

// Split into export const blocks
const lines = content.split('\n');
const blocks = []; // { name, startLine, endLine, bodyLines }

let inBlock = false;
let blockStart = -1;
let blockName = '';
let braceDepth = 0;

for (let i = 0; i < lines.length; i++) {
  const line = lines[i];
  if (!inBlock && line.match(/^export const \w+ = \{/)) {
    inBlock = true;
    blockStart = i;
    blockName = line.match(/export const (\w+)/)[1];
    braceDepth = (line.match(/\{/g) || []).length - (line.match(/\}/g) || []).length;
    if (braceDepth === 0) {
      // Single-line object, skip
      inBlock = false;
    }
  } else if (inBlock) {
    braceDepth += (line.match(/\{/g) || []).length - (line.match(/\}/g) || []).length;
    if (braceDepth <= 0) {
      blocks.push({
        name: blockName,
        start: blockStart,
        end: i,
        line: lines.slice(blockStart, i + 1)
      });
      inBlock = false;
    }
  }
}

let changed = false;
let totalRemoved = 0;
let totalTyped = 0;

for (const block of blocks) {
  const bodyLines = block.line.slice(1, -1); // exclude opening/closing braces
  const seenProps = new Map(); // propName -> first occurrence index in bodyLines
  const keptIndices = new Set();
  const removedIndices = new Set();

  for (let i = 0; i < bodyLines.length; i++) {
    const trimmed = bodyLines[i].trim();
    if (!trimmed || trimmed.startsWith('//')) continue;
    const m = trimmed.match(/^([a-zA-Z_][a-zA-Z0-9_]*):\s*/);
    if (m) {
      const propName = m[1];
      if (seenProps.has(propName)) {
        removedIndices.add(i);
        totalRemoved++;
      } else {
        seenProps.set(propName, i);
        keptIndices.add(i);
      }
    }
  }

  if (removedIndices.size > 0) {
    // Filter out removed lines
    const newBody = bodyLines.filter((_, i) => !removedIndices.has(i));
    // Also fix implicit any types
    let typedCount = 0;
    const finalBody = newBody.map(line => {
      // Add types to parameterless arrow functions that need them
      const paramMatch = line.match(/:\s*\(\s*([^)]*)\s*\)\s*=>/);
      if (paramMatch) {
        const params = paramMatch[1];
        if (params && !params.includes(':')) {
          // Needs type annotation
          const typedParams = params.split(',').map(p => {
            const trimmed = p.trim();
            if (!trimmed) return trimmed;
            if (trimmed.includes('?')) {
              const parts = trimmed.split('?');
              return parts[0].trim() + ': string' + (parts[1] ? '?' : '') + (parts[1] || '');
            }
            // Try to infer type
            if (trimmed === 'id') return 'id: string';
            if (trimmed === 'data') return 'data: unknown';
            if (trimmed === 'page' || trimmed === 'size') return trimmed + ': number';
            if (trimmed === 'start' || trimmed === 'end') return trimmed + ': string';
            return trimmed + ': string';
          }).join(', ');
          line = line.replace(paramMatch[0], `: (${typedParams}) =>`);
          typedCount++;
        }
      }
      return line;
    });
    totalTyped += typedCount;

    const newBlockLines = [block.line[0], ...finalBody, block.line[block.line.length - 1]];
    blocks[blocks.indexOf(block)] = { ...block, line: newBlockLines };
    changed = true;
  }
}

if (changed) {
  // Reconstruct the file
  let result = [];
  let lineIdx = 0;
  for (const block of blocks) {
    // Add lines before this block
    while (lineIdx < block.start) {
      result.push(lines[lineIdx]);
      lineIdx++;
    }
    // Add the block
    result = result.concat(block.line);
    lineIdx = block.end + 1;
  }
  // Add remaining lines
  while (lineIdx < lines.length) {
    result.push(lines[lineIdx]);
    lineIdx++;
  }

  fs.writeFileSync(filePath, result.join('\n'), 'utf8');
  console.log(`Deduplicated ${totalRemoved} duplicate properties and added ${totalTyped} type annotations.`);
} else {
  console.log('No changes needed.');
}
