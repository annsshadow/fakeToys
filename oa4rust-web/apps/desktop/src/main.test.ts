import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

describe('desktop routes', () => {
  it('registers the SQL statement designer path once', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'main.ts'), 'utf8');
    const matches = source.match(/path: 'query-statement-designer'/g) ?? [];

    expect(matches).toHaveLength(1);
    expect(source).toContain("name: 'QueryStatementDesigner'");
  });
});
