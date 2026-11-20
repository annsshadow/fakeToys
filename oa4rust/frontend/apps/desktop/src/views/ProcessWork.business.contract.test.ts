import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const src = readFileSync(fileURLToPath(new URL('./ProcessWork.vue', import.meta.url)), 'utf8')
const template = src.slice(src.indexOf('<template>'), src.indexOf('</template>'))

describe('ProcessWork 业务契约', () => {
  it('默认视图提供待我处理 / 我发起的 / 已办三个业务区块', () => {
    expect(template).toContain('class="work-pending"')
    expect(template).toContain('class="work-started"')
    expect(template).toContain('class="work-completed"')
    expect(template).toContain('data-testid="work-pending-list"')
    expect(template).toContain('data-testid="work-started-list"')
    expect(template).toContain('data-testid="work-completed-list"')
    expect(template).toContain('>待我处理<')
    expect(template).toContain('>我发起的<')
    expect(template).toContain('>已办<')
  })

  it('待办列表与发起申请使用真实业务控件', () => {
    expect(template).toContain('<form')
    expect(template).toContain('type="submit"')
    expect(template).toContain('<ul')
    expect(template).toMatch(/v-for="[^"]*\bitem\b/)
    expect(template).toContain('item-body')
  })

  it('默认视图不渲染 /api/* 端点调试按钮', () => {
    expect(template).not.toContain('/api/')
    for (const handler of [
      'getProcessDefinitionList',
      'getProcessTodoList',
      'getProcessDoneList',
      'getProcessMyStartList',
      'startProcess',
      'getBpmnModel',
      'assignTask',
      'completeTask',
      'transferTask',
      'rollbackTask',
      'getProcessComments',
      'addProcessComment',
      'getProcessVariables',
      'updateProcessVariables',
      'deleteProcessDefinition',
      'suspendProcessDefinition',
      'activateProcessDefinition',
    ]) {
      expect(template).not.toContain(handler)
    }
  })
})
