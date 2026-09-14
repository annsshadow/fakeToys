import { existsSync, readdirSync, readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const viewsDir = import.meta.dirname
const repositoryRoot = resolve(viewsDir, '../../../../..')
const manifestPath = resolve(repositoryRoot, 'docs/audits/w13-w14-repository-closure.manifest.json')
const legacyRoot = resolve(repositoryRoot, 'oa/o2web/source')
const mainPath = resolve(viewsDir, '../main.ts')

type Status = 'implemented' | 'out_of_scope' | 'still_blocked'
type Evidence = { file: string; contains: string }
type Entry = {
  id: string
  workItem: 'W13' | 'W14'
  kind: 'crud_view' | 'legacy_family_component'
  view?: string
  legacyComponent?: string
  status: Status
  replacementClaim: boolean
  acceptance?: string
  acceptedBoundary?: { scope: string; impact: string; owner: string; replacementClaim: boolean }
  evidence: { frontend: Evidence[]; routing?: Evidence[]; backend?: Evidence[] }
}
type Manifest = {
  schemaVersion: number
  entries: Entry[]
  summary: {
    total: number
    byWorkItem: Record<'W13' | 'W14', number>
    byStatus: Record<Status, number>
  }
}

const manifest = JSON.parse(readFileSync(manifestPath, 'utf8')) as Manifest
const sorted = (values: string[]) => [...values].sort()
const readRepositoryFile = (file: string) => readFileSync(resolve(repositoryRoot, file), 'utf8')
const exactCrudViews = () =>
  readdirSync(viewsDir)
    .filter((file) => file.endsWith('.vue'))
    .filter((file) => readFileSync(resolve(viewsDir, file), 'utf8').includes('class="crud-view"'))

const w14Prefixes = ['x_component_Minder', 'x_component_service_', 'x_component_AppMarketV2', 'x_component_Forum']
const discoveredW14Families = () =>
  readdirSync(legacyRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory() && w14Prefixes.some((prefix) => entry.name.startsWith(prefix)))
    .map((entry) => entry.name)

function expectEvidence(evidence: Evidence, entry: Entry) {
  const absolutePath = resolve(repositoryRoot, evidence.file)
  expect(existsSync(absolutePath), `${entry.id}: evidence file does not exist: ${evidence.file}`).toBe(true)
  expect(readFileSync(absolutePath, 'utf8'), `${entry.id}: evidence text missing in ${evidence.file}`).toContain(
    evidence.contains,
  )
}

function countsBy<T extends string>(values: T[]): Record<T, number> {
  return values.reduce(
    (counts, value) => {
      counts[value] = (counts[value] ?? 0) + 1
      return counts
    },
    {} as Record<T, number>,
  )
}

describe('W13/W14 repository closure manifest', () => {
  it('covers every exact crud-view and rejects unadjudicated additions', () => {
    const adjudicated = manifest.entries
      .filter((entry) => entry.workItem === 'W13' && entry.kind === 'crud_view')
      .map((entry) => entry.view as string)

    expect(sorted(adjudicated)).toEqual(sorted(exactCrudViews()))
    expect(new Set(adjudicated).size).toBe(adjudicated.length)
  })

  it('covers every legacy W14 family component and rejects omissions', () => {
    const adjudicated = manifest.entries
      .filter((entry) => entry.workItem === 'W14' && entry.kind === 'legacy_family_component')
      .map((entry) => entry.legacyComponent as string)

    expect(sorted(adjudicated)).toEqual(sorted(discoveredW14Families()))
    expect(new Set(adjudicated).size).toBe(adjudicated.length)
  })

  it('keeps classifications machine-consistent and never counts exclusions as implemented', () => {
    expect(manifest.schemaVersion).toBe(1)
    expect(new Set(manifest.entries.map((entry) => entry.id)).size).toBe(manifest.entries.length)
    expect(
      manifest.entries.every((entry) => ['implemented', 'out_of_scope', 'still_blocked'].includes(entry.status)),
    ).toBe(true)
    expect(manifest.summary.total).toBe(manifest.entries.length)
    expect(manifest.summary.byWorkItem).toEqual(countsBy(manifest.entries.map((entry) => entry.workItem)))
    expect(manifest.summary.byStatus).toEqual(countsBy(manifest.entries.map((entry) => entry.status)))

    for (const entry of manifest.entries.filter((candidate) => candidate.status === 'out_of_scope')) {
      expect(entry.replacementClaim, `${entry.id}: out-of-scope is not implemented`).toBe(false)
      expect(entry.acceptedBoundary?.replacementClaim, `${entry.id}: boundary must reject a replacement claim`).toBe(
        false,
      )
      expect(entry.acceptedBoundary?.scope, `${entry.id}: missing accepted scope`).toBeTruthy()
      expect(entry.acceptedBoundary?.impact, `${entry.id}: missing user impact`).toBeTruthy()
      expect(entry.acceptedBoundary?.owner, `${entry.id}: missing boundary owner`).toBeTruthy()
    }
  })

  it('requires load-bearing repository evidence before declaring implementation', () => {
    const implemented = manifest.entries.filter((entry) => entry.status === 'implemented')
    expect(implemented.length).toBeGreaterThan(0)

    for (const entry of implemented) {
      expect(entry.replacementClaim, `${entry.id}: implemented entry must make an explicit claim`).toBe(true)
      expect(entry.acceptance, `${entry.id}: implementation claim must be narrowly scoped`).toBeTruthy()
      expect(entry.evidence.frontend.length, `${entry.id}: missing frontend evidence`).toBeGreaterThan(0)
      expect(entry.evidence.routing?.length, `${entry.id}: missing route evidence`).toBeGreaterThan(0)
      expect(entry.evidence.backend?.length, `${entry.id}: missing backend evidence`).toBeGreaterThan(0)
      for (const group of Object.values(entry.evidence))
        for (const evidence of group ?? []) expectEvidence(evidence, entry)
    }
  })

  it('does not allow known thin CRUD mechanics to be relabelled as implemented', () => {
    for (const entry of manifest.entries.filter((candidate) => candidate.workItem === 'W13')) {
      const source = readRepositoryFile(`oa4rust-web/apps/desktop/src/views/${entry.view}`)
      const copiesAsyncQueryOnce = source.includes('items.value = data.value ?? []')
      const listEndpoint = source.match(/const ep = '([^']*\/list)'/)?.[1]
      const reusesListForWrites =
        !!listEndpoint &&
        source.includes('api.post(ep') &&
        source.includes('api.put(ep +') &&
        source.includes('api.delete(ep +')
      const genericNameFlagForm = source.includes('v-model="form.name"') && source.includes('v-model="form.flag"')

      if (copiesAsyncQueryOnce || reusesListForWrites || genericNameFlagForm) {
        expect(entry.status, `${entry.id}: thin CRUD evidence cannot support implemented`).not.toBe('implemented')
        expect(entry.replacementClaim, `${entry.id}: blocked thin CRUD cannot claim replacement`).toBe(false)
      }
    }
  })

  it('keeps every W13 view reachable through the actual application router', () => {
    const main = readFileSync(mainPath, 'utf8')
    for (const entry of manifest.entries.filter((candidate) => candidate.workItem === 'W13')) {
      expect(main, `${entry.id}: view is not registered in main.ts`).toContain(`import('./views/${entry.view}')`)
      for (const evidence of entry.evidence.routing ?? []) expectEvidence(evidence, entry)
    }
  })
})
