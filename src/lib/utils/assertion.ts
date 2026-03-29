import type { AssertionResult, AssertionRule, ResponseRecord } from '../types'
import { extractByPath } from './jsonpath'

/**
 * Evaluate all enabled assertion rules against a response.
 * Returns an array of results — one per enabled rule.
 */
export function evaluateAssertions(
  rules: AssertionRule[],
  response: ResponseRecord,
): AssertionResult[] {
  return rules.filter((r) => r.isEnabled).map((rule) => evaluateOne(rule, response))
}

function evaluateOne(rule: AssertionRule, response: ResponseRecord): AssertionResult {
  const base = { ruleId: rule.id, expected: rule.expected ?? null }

  try {
    const actual = resolveSource(rule, response)
    const passed = compare(actual, rule.operator, rule.expected ?? null)

    return {
      ...base,
      passed,
      actual,
      message: passed
        ? 'Passed'
        : `Expected ${formatOperator(rule.operator)} ${rule.expected ?? ''}, got ${actual ?? 'undefined'}`,
    }
  } catch (e) {
    return {
      ...base,
      passed: false,
      actual: null,
      message: e instanceof Error ? e.message : String(e),
    }
  }
}

function resolveSource(rule: AssertionRule, response: ResponseRecord): string | null {
  const source = rule.source

  if (source.type === 'statusCode') {
    return String(response.statusCode)
  }

  if (source.type === 'responseTime') {
    return String(Math.round(response.elapsedTime))
  }

  if (source.type === 'header') {
    const header = response.headers.find((h) => h.key.toLowerCase() === source.value.toLowerCase())
    return header?.value ?? null
  }

  if (source.type === 'jsonPath') {
    if (!response.bodyString) return null
    try {
      const parsed = JSON.parse(response.bodyString)
      return extractByPath(parsed, source.value) ?? null
    } catch {
      return null
    }
  }

  if (source.type === 'bodyContains') {
    // For bodyContains, the actual value is the body itself — comparison is done in compare()
    return response.bodyString ?? null
  }

  return null
}

function compare(
  actual: string | null,
  operator: AssertionRule['operator'],
  expected: string | null,
): boolean {
  switch (operator) {
    case 'exists':
      return actual !== null && actual !== undefined
    case 'notExists':
      return actual === null || actual === undefined
    case 'equals':
      return actual === expected
    case 'notEquals':
      return actual !== expected
    case 'contains':
      return actual !== null && expected !== null && actual.includes(expected)
    case 'notContains':
      return actual !== null && (expected === null || !actual.includes(expected))
    case 'greaterThan': {
      if (actual === null || expected === null) return false
      const a = parseFloat(actual)
      const e = parseFloat(expected)
      return !isNaN(a) && !isNaN(e) && a > e
    }
    case 'lessThan': {
      if (actual === null || expected === null) return false
      const a = parseFloat(actual)
      const e = parseFloat(expected)
      return !isNaN(a) && !isNaN(e) && a < e
    }
    case 'typeIs': {
      if (actual === null) return expected === 'null'
      return getValueType(actual) === expected
    }
    default:
      return false
  }
}

function getValueType(value: string): string {
  if (value === 'null') return 'null'
  if (value === 'true' || value === 'false') return 'boolean'
  if (!isNaN(parseFloat(value)) && isFinite(Number(value))) return 'number'
  // Check if it looks like an array or object
  try {
    const parsed = JSON.parse(value)
    if (Array.isArray(parsed)) return 'array'
    if (typeof parsed === 'object' && parsed !== null) return 'object'
  } catch {
    // Not JSON — it's a string
  }
  return 'string'
}

function formatOperator(operator: string): string {
  const labels: Record<string, string> = {
    equals: '==',
    notEquals: '!=',
    contains: 'contains',
    notContains: 'not contains',
    greaterThan: '>',
    lessThan: '<',
    exists: 'exists',
    notExists: 'not exists',
    typeIs: 'type is',
  }
  return labels[operator] ?? operator
}
