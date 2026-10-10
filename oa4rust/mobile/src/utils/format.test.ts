// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { formatFileSize } from './format'

describe('formatFileSize', () => {
  it('shows bytes below 1 KiB', () => {
    expect(formatFileSize(0)).toBe('0 B')
    expect(formatFileSize(512)).toBe('512 B')
    expect(formatFileSize(1023)).toBe('1023 B')
  })

  it('shows KB for 1 KiB up to <1 MiB (regression: 500 KB used to render as raw bytes)', () => {
    expect(formatFileSize(1024)).toBe('1.0 KB')
    expect(formatFileSize(500000)).toBe('488.3 KB')
    expect(formatFileSize(1048575)).toBe('1024.0 KB')
  })

  it('shows MB for 1 MiB up to <1 GiB', () => {
    expect(formatFileSize(1048576)).toBe('1.0 MB')
    expect(formatFileSize(5 * 1048576)).toBe('5.0 MB')
  })

  it('shows GB at and above 1 GiB', () => {
    expect(formatFileSize(1073741824)).toBe('1.0 GB')
    expect(formatFileSize(3 * 1073741824)).toBe('3.0 GB')
  })

  it('returns empty string for non-number / non-finite / negative inputs', () => {
    expect(formatFileSize(undefined)).toBe('')
    expect(formatFileSize('500')).toBe('')
    expect(formatFileSize(Number.NaN)).toBe('')
    expect(formatFileSize(Number.POSITIVE_INFINITY)).toBe('')
    expect(formatFileSize(-1)).toBe('')
  })
})
