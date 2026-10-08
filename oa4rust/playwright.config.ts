// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { defineConfig, devices } from '@playwright/test'

const BASE_URL = process.env.BASE_URL ?? 'http://localhost:3000'

export default defineConfig({
  testDir: './e2e',
  testMatch: '**/*.spec.ts',
  fullyParallel: false,
  forbidOnly: true,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  timeout: 60_000,
  expect: { timeout: 10_000 },
  globalSetup: './e2e/global-setup.ts',
  reporter: process.env.COLLECT_COVERAGE
    ? [
        ['list'],
        ['html', { open: 'never', outputFolder: 'playwright-coverage/html' }],
        ['json', { outputFile: 'playwright-coverage/results.json' }],
      ]
    : [['list'], ['html', { open: 'never' }], ['json', { outputFile: 'test-results/results.json' }]],
  use: {
    baseURL: BASE_URL,
    trace: 'retain-on-failure',
    screenshot: 'only-on-failure',
    video: 'only-on-failure',
    // Coverage collection - only enabled when COLLECT_COVERAGE=true
    collectCoverage: process.env.COLLECT_COVERAGE === 'true',
    coverageDir: 'playwright-coverage',
    coverageReporters: ['lcov', 'json', 'html'],
  },
  projects: [
    {
      name: 'live-chromium',
      use: {
        ...devices['Desktop Chrome'],
        collectCoverage: process.env.COLLECT_COVERAGE === 'true',
        baseURL: BASE_URL,
      },
    },
    {
      name: 'coverage-chromium',
      use: {
        ...devices['Desktop Chrome'],
        collectCoverage: true,
        baseURL: BASE_URL,
      },
    },
  ],
  outputDir: 'test-results',
})

/**
 * To run coverage-enabled E2E tests:
 * COLLECT_COVERAGE=true E2E_USERNAME=admin E2E_PASSWORD=admin123 npx playwright test
 *
 * Requires:
 * 1. Live backend server running at BASE_URL (default: http://localhost:3000)
 * 2. Database accessible
 * 3. E2E_USERNAME and E2E_PASSWORD env vars set
 */
