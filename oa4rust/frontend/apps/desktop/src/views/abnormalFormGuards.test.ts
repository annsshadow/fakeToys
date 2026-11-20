// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function view(name: string): string {
  return readFileSync(resolve(import.meta.dirname, name), 'utf8')
}

function mutationBlock(source: string, marker: string): string {
  const from = source.indexOf(marker)
  if (from < 0) return ''
  const end = source.indexOf('\n})', from)
  return source.slice(from, end < 0 ? undefined : end + 3)
}

function fnBlock(source: string, marker: string): string {
  const from = source.indexOf(marker)
  if (from < 0) return ''
  const end = source.indexOf('\n}', from)
  return source.slice(from, end < 0 ? undefined : end + 2)
}

// R157 只覆盖标准 saveItem + saveM.mutationFn 模式；这批异形面（create*/reply/upload/
// modalSave）同样是「mutate 返回 promise 而 async 函数不 await」：双击时 mutate 已被调用，
// 请求尚未 settle，按钮上的 :disabled 完全来不及生效 → 重复 POST 创建重复资源。
// 守卫必须落在 mutate 调用方同步占用，复位交给 useMutation 的 onSettled（双路径）。
interface GuardSpec {
  name: string
  file: string
  ref: string
  mutationMarker: string
  fnMarker: string
  mutateExpr: string
  button: RegExp
}

const SPECS: GuardSpec[] = [
  {
    name: 'BBSForum 发布帖子',
    file: 'BBSForum.vue',
    ref: 'topicSaving',
    mutationMarker: 'const createMutation = useMutation({',
    fnMarker: 'function createTopic(): void {',
    mutateExpr: 'createMutation.mutate(',
    button: /<button type="submit" class="btn-submit"[^>]*data-testid="bbs-compose-submit"[^>]*>/,
  },
  {
    name: 'BBSForum 发送回复',
    file: 'BBSForum.vue',
    ref: 'replySaving',
    mutationMarker: 'const replyMutation = useMutation({',
    fnMarker: 'function submitReply(): void {',
    mutateExpr: 'replyMutation.mutate(',
    button: /<button class="reply-btn"[^>]*data-testid="bbs-reply-submit"[^>]*@click="submitReply">/,
  },
  {
    name: 'QueryManager 创建查询',
    file: 'QueryManager.vue',
    ref: 'createSaving',
    mutationMarker: 'const cm = useMutation({',
    fnMarker: 'function createQ() {',
    mutateExpr: 'cm.mutate(',
    button: /<button class="bs3" :disabled="[^"]*createSaving[^"]*" @click="createQ">/,
  },
  {
    name: 'QueryManagerDeep 创建查询',
    file: 'QueryManagerDeep.vue',
    ref: 'createSaving',
    mutationMarker: 'const cm = useMutation({',
    fnMarker: 'function createQuery() {',
    mutateExpr: 'cm.mutate(',
    button: /<button class="btn-save" :disabled="[^"]*createSaving[^"]*" @click="createQuery">/,
  },
  {
    name: 'MeetingApp 创建会议',
    file: 'MeetingApp.vue',
    ref: 'createSaving',
    mutationMarker: 'const cm = useMutation({',
    fnMarker: 'function createMeeting() {',
    mutateExpr: 'cm.mutate(',
    button: /<button class="bs" :disabled="[^"]*createSaving[^"]*" @click="createMeeting">/,
  },
  {
    name: 'Personal 头像上传',
    file: 'Personal.vue',
    ref: 'avatarSaving',
    mutationMarker: 'const avatarMutation = useMutation({',
    fnMarker: 'function handleAvatarUpload(e: Event): void {',
    mutateExpr: 'avatarMutation.mutate(',
    button: /<input[^>]*:disabled="avatarSaving"[^>]*@change="handleAvatarUpload"[^>]*\/>/,
  },
  {
    name: 'ProgramCenter 创建 Agent',
    file: 'ProgramCenterApp.vue',
    ref: 'agentCreateSaving',
    mutationMarker: 'const createAgentM = useMutation({',
    fnMarker: 'async function onCreateAgent() {',
    mutateExpr: 'createAgentM.mutate(',
    button: /<button class="btn-primary" :disabled="agentCreateSaving" @click="onCreateAgent">/,
  },
  {
    name: 'ProgramCenter 创建字典',
    file: 'ProgramCenterApp.vue',
    ref: 'dictCreateSaving',
    mutationMarker: 'const createDictM = useMutation({',
    fnMarker: 'function onCreateDict() {',
    mutateExpr: 'createDictM.mutate(',
    button: /<button class="btn-primary" :disabled="[^"]*dictCreateSaving[^"]*" @click="onCreateDict">/,
  },
  {
    name: 'QueryStatementDesigner 保存语句',
    file: 'QueryStatementDesigner.vue',
    ref: 'modalSaving',
    mutationMarker: 'const saveM = useMutation({',
    fnMarker: 'function modalSave() {',
    mutateExpr: 'saveM.mutate(',
    button: /<button class="btn-save" :disabled="[^"]*modalSaving[^"]*" @click="modalSave">/,
  },
]

describe('异形表单提交重入守卫（R158）', () => {
  it.each(SPECS)('$name：同步占用 / onSettled 复位 / 按钮禁用与视觉态', (spec) => {
    const source = view(spec.file)

    // 1. 必须有专属 ref(false) 占位
    expect(source).toContain(`${spec.ref} = ref(false)`)

    // 2. 复位必须由 mutation 的 onSettled 承担（成功/失败双路径），不能只靠按钮 disabled
    const mutation = mutationBlock(source, spec.mutationMarker)
    expect(mutation).not.toBe('')
    expect(mutation).toContain('onSettled: () => (' + spec.ref + '.value = false)')

    // 3. 提交函数里守卫 → 占位 → mutate 必须按此顺序同步发生
    const fn = fnBlock(source, spec.fnMarker)
    expect(fn).not.toBe('')
    const guard = fn.indexOf('if (' + spec.ref + '.value) return')
    const occupy = fn.indexOf(spec.ref + '.value = true')
    const submit = fn.indexOf(spec.mutateExpr)
    expect(guard).toBeGreaterThan(-1)
    expect(occupy).toBeGreaterThan(guard)
    expect(submit).toBeGreaterThan(occupy)

    // 4. 触发按钮禁用 + 提交中视觉态
    expect(source).toMatch(spec.button)
    expect(source).toContain(`${spec.ref} ? '`)
  })
})
