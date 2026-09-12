import { describe, expect, it } from 'vitest'
import fixtures from './fixtures.json'
import schemas from './schemas.json'

type JsonSchema = {
  type?: 'object' | 'array' | 'string' | 'number' | 'boolean'
  const?: unknown
  enum?: unknown[]
  required?: string[]
  additionalProperties?: boolean | JsonSchema
  properties?: Record<string, JsonSchema>
  items?: JsonSchema
  minLength?: number
  minItems?: number
  minProperties?: number
  uniqueItems?: boolean
  exclusiveMinimum?: number
}

function validate(schema: JsonSchema, value: unknown, path = '$'): string[] {
  const errors: string[] = []

  if (schema.const !== undefined && value !== schema.const)
    errors.push(`${path} must equal ${JSON.stringify(schema.const)}`)
  if (schema.enum && !schema.enum.some((candidate) => candidate === value)) errors.push(`${path} is outside the enum`)

  if (schema.type === 'string') {
    if (typeof value !== 'string') return [...errors, `${path} must be a string`]
    if (schema.minLength !== undefined && value.length < schema.minLength) errors.push(`${path} is too short`)
  }

  if (schema.type === 'number') {
    if (typeof value !== 'number' || !Number.isFinite(value)) return [...errors, `${path} must be a finite number`]
    if (schema.exclusiveMinimum !== undefined && value <= schema.exclusiveMinimum) {
      errors.push(`${path} must be greater than ${schema.exclusiveMinimum}`)
    }
  }

  if (schema.type === 'boolean' && typeof value !== 'boolean') errors.push(`${path} must be a boolean`)

  if (schema.type === 'array') {
    if (!Array.isArray(value)) return [...errors, `${path} must be an array`]
    if (schema.minItems !== undefined && value.length < schema.minItems) errors.push(`${path} has too few items`)
    if (schema.uniqueItems) {
      const serialized = value.map((item) => JSON.stringify(item))
      if (new Set(serialized).size !== serialized.length) errors.push(`${path} must contain unique items`)
    }
    const itemSchema = schema.items
    if (itemSchema) {
      value.forEach((item, index) => {
        errors.push(...validate(itemSchema, item, `${path}[${index}]`))
      })
    }
  }

  if (schema.type === 'object') {
    if (typeof value !== 'object' || value === null || Array.isArray(value))
      return [...errors, `${path} must be an object`]
    const record = value as Record<string, unknown>
    const keys = Object.keys(record)
    if (schema.minProperties !== undefined && keys.length < schema.minProperties)
      errors.push(`${path} has too few properties`)
    for (const key of schema.required ?? []) {
      if (!(key in record)) errors.push(`${path}.${key} is required`)
    }
    for (const key of keys) {
      const propertySchema = schema.properties?.[key]
      if (propertySchema) errors.push(...validate(propertySchema, record[key], `${path}.${key}`))
      else if (schema.additionalProperties === false) errors.push(`${path}.${key} is not allowed`)
      else if (typeof schema.additionalProperties === 'object') {
        errors.push(...validate(schema.additionalProperties, record[key], `${path}.${key}`))
      }
    }
  }

  return errors
}

describe('S3 serialized designer contracts', () => {
  for (const contract of ['activities', 'moduleList', 'query', 'portal'] as const) {
    it(`${contract} fixture conforms to its pinned JSON schema`, () => {
      expect(validate(schemas[contract] as JsonSchema, fixtures[contract]), contract).toEqual([])
    })

    it(`${contract} survives a JSON save/load round-trip without structural drift`, () => {
      const roundTripped = JSON.parse(JSON.stringify(fixtures[contract]))
      expect(validate(schemas[contract] as JsonSchema, roundTripped), contract).toEqual([])
      expect(roundTripped).toEqual(fixtures[contract])
    })
  }

  it('keeps activity references internally consistent for engine traversal', () => {
    const definition = fixtures.activities
    const activityIds = new Set(definition.activities.map((activity) => activity.id))
    const routeIds = new Set(definition.routes.map((route) => route.id))

    expect(definition.activities.filter((activity) => activity.activityType === 'begin')).toHaveLength(1)
    for (const activity of definition.activities) {
      expect(activity.edition).toBe(definition.edition.code)
      for (const routeId of activity.routeList) expect(routeIds.has(routeId), routeId).toBe(true)
    }
    for (const route of definition.routes) {
      expect(activityIds.has(route.from), route.from).toBe(true)
      expect(activityIds.has(route.to), route.to).toBe(true)
      expect(route.edition).toBe(definition.edition.code)
    }
    for (const permission of definition.fieldPermissions) {
      expect(activityIds.has(permission.activity), permission.activity).toBe(true)
      expect(permission.hidden && (permission.readable || permission.writable || permission.required)).toBe(false)
      expect(permission.required && !permission.writable).toBe(false)
    }
  })
})
