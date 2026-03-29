import { describe, it, expect } from 'vitest'
import { evaluateAssertions } from './assertion'
import type { AssertionRule, ResponseRecord } from '../types'

function makeResponse(overrides: Partial<ResponseRecord> = {}): ResponseRecord {
  return {
    statusCode: 200,
    headers: [
      { key: 'content-type', value: 'application/json' },
      { key: 'x-request-id', value: 'abc123' },
    ],
    bodyString: JSON.stringify({ data: { id: 42, name: 'Alice', tags: ['admin', 'user'] } }),
    elapsedTime: 150,
    bodySize: 100,
    isJson: true,
    isTruncated: false,
    contentType: 'application/json',
    ...overrides,
  }
}

function makeRule(overrides: Partial<AssertionRule>): AssertionRule {
  return {
    id: crypto.randomUUID(),
    source: { type: 'statusCode', value: '' },
    operator: 'equals',
    expected: '200',
    isEnabled: true,
    ...overrides,
  }
}

describe('evaluateAssertions', () => {
  it('skips disabled rules', () => {
    const results = evaluateAssertions([makeRule({ isEnabled: false })], makeResponse())
    expect(results).toHaveLength(0)
  })

  describe('statusCode', () => {
    it('passes when status matches', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'statusCode', value: '' },
            operator: 'equals',
            expected: '200',
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })

    it('fails when status does not match', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'statusCode', value: '' },
            operator: 'equals',
            expected: '201',
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(false)
      expect(results[0].actual).toBe('200')
    })

    it('greaterThan works for status', () => {
      const results = evaluateAssertions(
        [makeRule({ operator: 'greaterThan', expected: '199' })],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })

    it('lessThan works for status', () => {
      const results = evaluateAssertions(
        [makeRule({ operator: 'lessThan', expected: '300' })],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })
  })

  describe('responseTime', () => {
    it('passes when response time is less than expected', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'responseTime', value: '' },
            operator: 'lessThan',
            expected: '2000',
          }),
        ],
        makeResponse({ elapsedTime: 150 }),
      )
      expect(results[0].passed).toBe(true)
    })

    it('fails when response time exceeds threshold', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'responseTime', value: '' },
            operator: 'lessThan',
            expected: '100',
          }),
        ],
        makeResponse({ elapsedTime: 150 }),
      )
      expect(results[0].passed).toBe(false)
    })
  })

  describe('header', () => {
    it('matches header value', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'header', value: 'content-type' },
            operator: 'contains',
            expected: 'json',
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })

    it('case-insensitive header lookup', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'header', value: 'Content-Type' },
            operator: 'contains',
            expected: 'json',
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })

    it('header exists check', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'header', value: 'x-request-id' },
            operator: 'exists',
            expected: null,
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })

    it('header not exists check', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'header', value: 'x-nonexistent' },
            operator: 'notExists',
            expected: null,
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })
  })

  describe('jsonPath', () => {
    it('extracts and compares json path value', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'jsonPath', value: '$.data.name' },
            operator: 'equals',
            expected: 'Alice',
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })

    it('jsonPath exists check', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'jsonPath', value: '$.data.id' },
            operator: 'exists',
            expected: null,
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })

    it('jsonPath notExists for missing path', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'jsonPath', value: '$.data.missing' },
            operator: 'notExists',
            expected: null,
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })

    it('jsonPath numeric comparison', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'jsonPath', value: '$.data.id' },
            operator: 'greaterThan',
            expected: '10',
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })

    it('jsonPath typeIs check', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'jsonPath', value: '$.data.id' },
            operator: 'typeIs',
            expected: 'number',
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })

    it('handles non-json body gracefully', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'jsonPath', value: '$.data' },
            operator: 'exists',
            expected: null,
          }),
        ],
        makeResponse({ bodyString: 'not json', isJson: false }),
      )
      expect(results[0].passed).toBe(false)
    })
  })

  describe('bodyContains', () => {
    it('passes when body contains text', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'bodyContains', value: 'Alice' },
            operator: 'contains',
            expected: 'Alice',
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })

    it('fails when body does not contain text', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'bodyContains', value: 'Bob' },
            operator: 'contains',
            expected: 'Bob',
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(false)
    })
  })

  describe('operators', () => {
    it('notEquals passes on mismatch', () => {
      const results = evaluateAssertions(
        [makeRule({ operator: 'notEquals', expected: '404' })],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })

    it('notContains passes when substring absent', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'header', value: 'content-type' },
            operator: 'notContains',
            expected: 'xml',
          }),
        ],
        makeResponse(),
      )
      expect(results[0].passed).toBe(true)
    })
  })

  describe('multiple assertions', () => {
    it('evaluates all rules independently', () => {
      const results = evaluateAssertions(
        [
          makeRule({
            source: { type: 'statusCode', value: '' },
            operator: 'equals',
            expected: '200',
          }),
          makeRule({
            source: { type: 'responseTime', value: '' },
            operator: 'lessThan',
            expected: '2000',
          }),
          makeRule({
            source: { type: 'jsonPath', value: '$.data.name' },
            operator: 'equals',
            expected: 'Bob',
          }),
        ],
        makeResponse(),
      )
      expect(results).toHaveLength(3)
      expect(results[0].passed).toBe(true)
      expect(results[1].passed).toBe(true)
      expect(results[2].passed).toBe(false)
    })
  })
})
