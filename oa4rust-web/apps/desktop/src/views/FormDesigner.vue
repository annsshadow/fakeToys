<template>
  <div class="fd">
    <!-- Header -->
    <div class="fd-header glass-card">
      <div class="fd-title">
        <h1>表单设计器</h1>
        <p class="subtitle">/jaxrs/form/* — 可视化表单构建器</p>
      </div>
      <div class="fd-actions">
        <button class="btn" @click="resetForm" title="新建表单">📄 新建</button>
        <button class="btn btn-outline" @click="loadForms" title="刷新列表">🔄 刷新</button>
        <button class="btn btn-outline" :class="{ active: mode === 'preview' }" @click="togglePreview">👁 预览</button>
        <button class="btn btn-primary" :disabled="!currentForm || !currentForm.name" @click="saveForm">💾 保存</button>
      </div>
    </div>
    <div class="fd-body">
      <!-- Left: Form List -->
      <aside class="fd-list-panel glass-card">
        <div class="list-header"><span>📋 表单列表</span><button class="btn-sm" @click="loadForms">刷新</button></div>
        <div class="list-search"><input v-model="listFilter" placeholder="搜索..." class="list-input" /></div>
        <div class="list-items">
          <div v-if="formsLoading" class="list-loading">加载中...</div>
          <div v-else-if="filteredForms.length===0" class="list-empty">暂无表单</div>
          <div v-for="f in filteredForms" :key="f.id" class="list-item" :class="{active:currentForm && currentForm.id===f.id}" @click="loadForm(f)">
            <div class="li-name">{{ f.name||f.title||'未命名' }}</div>
            <div class="li-meta">{{ f.flag||f.id }}</div>
          </div>
        </div>
      </aside>
      <!-- Palette + Canvas + Props -->
      <div class="fd-center">
        <!-- Left: Field Palette -->
        <aside class="fd-palette glass-card" v-if="mode==='edit'">
          <div class="palette-title">字段组件</div>
          <div class="palette-grid">
            <div v-for="ft in fieldTypes" :key="ft.type" class="palette-item" draggable="true"
              @dragstart="onDragStart($event, ft)" @click="addField(ft)">
              <span class="pi-icon">{{ ft.icon }}</span>
              <span class="pi-label">{{ ft.label }}</span>
            </div>
          </div>
          <div class="palette-sep"></div>
          <div class="palette-title">布局组件</div>
          <div class="palette-grid">
            <div v-for="ft in layoutTypes" :key="ft.type" class="palette-item" draggable="true"
              @dragstart="onDragStart($event, ft)" @click="addField(ft)">
              <span class="pi-icon">{{ ft.icon }}</span>
              <span class="pi-label">{{ ft.label }}</span>
            </div>
          </div>
        </aside>
        <!-- Center: Canvas -->
        <main class="fd-canvas glass-card" @dragover.prevent @drop="onDrop">
          <div v-if="!currentForm" class="canvas-empty">
            <div class="ce-icon">📋</div>
            <p>选择或新建表单开始设计</p>
            <button class="btn btn-primary" @click="resetForm">+ 新建表单</button>
          </div>
          <div v-else class="canvas-form">
            <div class="cf-header">
              <input v-model="currentForm.name" placeholder="表单名称" class="cf-name" />
              <input v-model="currentForm.flag" placeholder="唯一标识(Flag)" class="cf-flag" :disabled="!!currentForm.id" />
            </div>
            <div class="cf-fields">
              <div v-if="currentForm.fields.length===0" class="fields-empty">拖拽字段到此处，或点击上方组件添加</div>
              <div v-for="(field, index) in currentForm.fields" :key="field.id" class="field-row" draggable="true"
                @dragstart="(e)=>onFieldDragStart(e,index)" @dragover="(e)=>onFieldDragOver(e,index)" @drop="(e)=>onFieldDrop(e,index)"
                :class="{ selected: selectedField && selectedField.id===field.id }" @click="selectField(field)">
                <div class="fr-handle">⠿</div>
                <div class="fr-icon">{{ getFieldIcon(field.type) }}</div>
                <div class="fr-info">
                  <div class="fr-label">{{ field.label || field.placeholder || '未命名字段' }}</div>
                  <div class="fr-meta">{{ field.type }} · {{ field.key || '—' }}</div>
                </div>
                <div class="fr-actions">
                  <button class="fa-btn" @click.stop="moveField(index,-1)" :disabled="index===0" title="上移">↑</button>
                  <button class="fa-btn" @click.stop="moveField(index,1)" :disabled="index===currentForm.fields.length-1" title="下移">↓</button>
                  <button class="fa-btn fa-del" @click.stop="removeField(index)" title="删除">✕</button>
                </div>
              </div>
            </div>
          </div>
        </main>
        <!-- Right: Property Panel -->
        <aside class="fd-props glass-card" v-if="mode==='edit' && selectedField">
          <div class="props-title"><span>属性面板</span><span class="props-type">{{ selectedField.type }}</span></div>
          <div class="props-body">
            <div class="prop-group"><label>显示名称</label><input v-model="selectedField.label" class="prop-input" placeholder="字段标签" /></div>
            <div class="prop-group"><label>字段标识 (Key)</label><input v-model="selectedField.key" class="prop-input" placeholder="唯一标识" /></div>
            <div class="prop-group"><label>占位提示</label><input v-model="selectedField.placeholder" class="prop-input" placeholder="输入提示文字" /></div>
            <div class="prop-group"><label>默认值</label><input v-model="selectedField.defaultValue" class="prop-input" placeholder="默认值" /></div>
            <div class="prop-group prop-row">
              <label class="checkbox-label"><input type="checkbox" v-model="selectedField.required" />必填</label>
              <label class="checkbox-label"><input type="checkbox" v-model="selectedField.disabled" />只读</label>
            </div>
            <div v-if="['select','checkbox_group','radio'].includes(selectedField.type)" class="prop-group">
              <label>选项配置（每行：值|显示名）</label>
              <textarea v-model="selectedField.optionsStr" class="prop-textarea" placeholder="male|男&#10;female|女"></textarea>
            </div>
            <div v-if="selectedField.type==='textarea'" class="prop-group">
              <label>行数</label><input v-model.number="selectedField.rows" type="number" class="prop-input" min="2" max="20" />
            </div>
            <div v-if="selectedField.type==='number'" class="prop-group">
              <label>范围</label>
              <div class="prop-row2">
                <input v-model.number="selectedField.min" type="number" class="prop-input" placeholder="最小" />
                <input v-model.number="selectedField.max" type="number" class="prop-input" placeholder="最大" />
              </div>
            </div>
          </div>
        </aside>
      </div>
    </div>
    <!-- Preview Modal -->
    <div v-if="mode==='preview' && currentForm" class="preview-overlay" @click.self="mode='edit'">
      <div class="preview-modal glass-card">
        <div class="preview-header">
          <h2>{{ currentForm.name || '表单预览' }}</h2>
          <button class="btn-close" @click="mode='edit'">✕</button>
        </div>
        <div class="preview-body">
          <form @submit.prevent>
            <div v-for="(field, i) in currentForm.fields" :key="i" class="pv-field">
              <label v-if="!['row_start','row_end'].includes(field.type)">
                {{ field.label }}<span v-if="field.required" class="req">*</span>
              </label>
              <!-- Text -->
              <input v-if="field.type==='text'" :placeholder="field.placeholder" :value="field.defaultValue" :disabled="field.disabled" class="pv-input" />
              <!-- Textarea -->
              <textarea v-else-if="field.type==='textarea'" :placeholder="field.placeholder" :rows="field.rows||4" :disabled="field.disabled" class="pv-input"></textarea>
              <!-- Number -->
              <input v-else-if="field.type==='number'" type="number" :placeholder="field.placeholder" :disabled="field.disabled" class="pv-input" />
              <!-- Date -->
              <input v-else-if="field.type==='date'" type="date" :disabled="field.disabled" class="pv-input" />
              <!-- Select -->
              <select v-else-if="field.type==='select'" :disabled="field.disabled" class="pv-input">
                <option value="">请选择</option>
                <option v-for="opt in parseOptions(field.optionsStr)" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
              </select>
              <!-- Checkbox -->
              <label v-else-if="field.type==='checkbox'" class="pv-cb-label">
                <input type="checkbox" :disabled="field.disabled" class="pv-cb" />{{ field.label }}
              </label>
              <!-- Checkbox group -->
              <div v-else-if="field.type==='checkbox_group'" class="pv-cb-group">
                <label v-for="opt in parseOptions(field.optionsStr)" :key="opt.value" class="pv-cb-label">
                  <input type="checkbox" :value="opt.value" :disabled="field.disabled" class="pv-cb" />{{ opt.label }}
                </label>
              </div>
              <!-- File -->
              <input v-else-if="field.type==='file'" type="file" class="pv-input" />
              <!-- Email -->
              <input v-else-if="field.type==='email'" type="email" :placeholder="field.placeholder" :disabled="field.disabled" class="pv-input" />
              <!-- Phone -->
              <input v-else-if="field.type==='phone'" type="tel" :placeholder="field.placeholder" :disabled="field.disabled" class="pv-input" />
              <!-- Section -->
              <fieldset v-else-if="field.type==='section'" class="pv-fieldset"><legend class="pv-legend">{{ field.label }}</legend></fieldset>
            </div>
            <button type="submit" class="btn btn-primary pv-submit">提交</button>
          </form>
        </div>
      </div>
    </div>
  </div>
    <!-- Field Templates Modal -->
    <div v-if="showFieldTemplates" class="modal-overlay" @click.self="showFieldTemplates=false">
      <div class="modal modal-lg glass-card">
        <div class="modal-header"><h3>📦 字段模板库</h3><button class="btn-close" @click="showFieldTemplates=false">✕</button></div>
        <div class="modal-body">
          <div class="tpl-grid">
            <div v-for="tpl in fieldTemplates2" :key="tpl.name" class="tpl-card" @click="applyTemplate2(tpl)">
              <div class="tpl-icon">{{ tpl.icon }}</div>
              <div class="tpl-name">{{ tpl.name }}</div>
              <div class="tpl-count">{{ tpl.fields.length }} 个字段</div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- Access Control Panel -->
    <div v-if="showAccessControl" class="access-panel">
      <div class="ap-header"><span>🔒 访问控制</span><button class="btn-sm" @click="showAccessControl=false">✕</button></div>
      <div class="ap-body">
        <div class="ap-add">
          <select v-model="newAccessRule.fieldKey" class="ap-select">
            <option v-for="f in currentForm?.fields" :value="f.key">{{ f.label || f.key }}</option>
          </select>
          <select v-model="newAccessRule.role" class="ap-select">
            <option value="admin">管理员</option><option value="editor">编辑</option><option value="viewer">查看</option>
          </select>
          <select v-model="newAccessRule.action" class="ap-select">
            <option value="show">显示</option><option value="hide">隐藏</option><option value="readonly">只读</option>
          </select>
          <button class="btn-sm" @click="addAccessRule(accessRules[accessRules.length-1]?.fieldKey||'', accessRules[accessRules.length-1]?.role||'', accessRules[accessRules.length-1]?.action||'')">+</button>
        </div>
        <div class="ap-list">
          <div v-for="(r, ri) in accessRules" :key="ri" class="ap-row">
            <span class="ap-field">{{ r.fieldKey }}</span>
            <span class="ap-role">{{ r.role }}</span>
            <span :class="['ap-action', 'ap-action-'+r.action]">{{ r.action }}</span>
            <button class="btn-xs btn-danger" @click="removeAccessRule(ri)">✕</button>
          </div>
        </div>
      </div>
    </div>
    <!-- Data Source Panel -->
    <div v-if="showDataSourcePanel" class="ds-panel">
      <div class="ds-header"><span>🔗 数据源配置</span><button class="btn-sm" @click="showDataSourcePanel=false">✕</button></div>
      <div class="ds-body">
        <div class="ds-add">
          <select v-model="newDataSource.fieldKey" class="ds-select">
            <option v-for="f in currentForm?.fields" :value="f.key">{{ f.label }}</option>
          </select>
          <input v-model="newDataSource.url" placeholder="API URL" class="ds-input" />
          <select v-model="newDataSource.method" class="ds-select">
            <option value="GET">GET</option><option value="POST">POST</option>
          </select>
          <button class="btn-sm" @click="addDataSource(dataSources[dataSources.length-1]?.fieldKey||'', dataSources[dataSources.length-1]?.url||'', dataSources[dataSources.length-1]?.method||'GET', '', '')">+</button>
        </div>
        <div class="ds-list">
          <div v-for="(ds, di) in dataSources" :key="di" class="ds-row">
            <span class="ds-field">{{ ds.fieldKey }}</span>
            <span class="ds-url">{{ ds.url }}</span>
            <span class="ds-method">{{ ds.method }}</span>
            <button class="btn-xs btn-danger" @click="removeDataSource(di)">✕</button>
          </div>
        </div>
      </div>
    </div>
    <!-- Condition Tree Builder -->
    <div v-if="showConditionTree" class="cond-tree-panel">
      <div class="ct-header"><span>🌳 条件树构建器</span><button class="btn-sm" @click="showConditionTree=false">✕</button></div>
      <div class="ct-body">
        <div class="ct-tree">
          <div v-if="conditionTreeRoot" class="ct-node ct-group" :class="conditionTreeRoot.logic">
            <div class="ct-node-header">
              <span class="ct-logic">{{ conditionTreeRoot.logic === "AND" ? "且" : "或" }}</span>
              <span>分组条件</span>
              <button class="btn-xs" @click="addConditionNode(conditionTreeRoot.id)">+ 条件</button>
              <button class="btn-xs" @click="addGroupNode(conditionTreeRoot.id)">+ 分组</button>
            </div>
            <div v-for="child in conditionTreeRoot.children" :key="child.id" class="ct-children">
              <cond-tree-item :node="child" :fields="currentForm?.fields||[]" @add-condition="(pid)=>addConditionNode(pid)" @add-group="(pid)=>addGroupNode(pid)" @remove="(nid)=>removeConditionNode(nid)" />
            </div>
          </div>
        </div>
        <button class="btn" @click="openConditionTree">🔄 重新生成</button>
      </div>
    </div>
    <!-- Layout Config Panel -->
    <div v-if="showLayoutConfig" class="layout-panel">
      <div class="lp-header"><span>📐 布局配置</span><button class="btn-sm" @click="showLayoutConfig=false">✕</button></div>
      <div class="lp-body">
        <div class="lp-row"><label>列数</label><div class="lp-cols">
          <button v-for="n in 3" :key="n" :class="['lp-col-btn',{active: layoutConfig.columns===n}]" @click="layoutConfig={...layoutConfig, columns: n as 1|2|3}">{{ n }}列</button>
        </div></div>
        <div class="lp-row"><label>间距</label><input type="range" v-model.number="layoutConfig.gutter" min="0" max="40" class="lp-range" /><span>{{ layoutConfig.gutter }}px</span></div>
        <div class="lp-row"><label>标签宽</label><input type="range" v-model.number="layoutConfig.labelWidth" min="60" max="200" class="lp-range" /><span>{{ layoutConfig.labelWidth }}px</span></div>
        <div class="lp-row"><label>对齐</label><div class="lp-align">
          <button v-for="a in ['left','center','right']" :key="a" :class="['lp-align-btn',{active: layoutConfig.align===a}]" @click="layoutConfig={...layoutConfig, align: a as any}">{{ a }}</button>
        </div></div>
        <button class="btn" @click="autoLayout()">📐 自动排列</button>
      </div>
    </div>
    <!-- Validation Builder -->
    <div v-if="showValidationBuilder" class="val-builder">
      <div class="vb-header"><span>✅ 验证规则构建器</span><button class="btn-sm" @click="showValidationBuilder=false">✕</button></div>
      <div class="vb-body">
        <div class="vb-rules">
          <div v-for="(vr, vi) in validationRules" :key="vi" class="vb-rule">
            <select v-model="vr.fieldKey" class="vb-select"><option v-for="f in currentForm?.fields" :value="f.key">{{ f.label }}</option></select>
            <select v-model="vr.rule" class="vb-select">
              <option value="required:">必填</option><option value="min:1">最小1</option><option value="max:100">最大100</option>
              <option value="pattern:^\d+$">数字</option><option value="pattern:^\w+@\w+\.\w+$">邮箱</option>
            </select>
            <input v-model="vr.message" placeholder="错误提示" class="vb-input" />
            <button class="btn-xs btn-danger" @click="removeValidationRule(vi)">✕</button>
          </div>
        </div>
        <button class="btn-sm" @click="addValidationRule">+ 添加规则</button>
        <button class="btn" @click="applyValidationRules">✓ 应用到选中</button>
        <button class="btn" @click="runValidation()">🔍 全部验证</button>
      </div>
    </div>
    <!-- Field History -->
    <div v-if="showFieldHistory" class="field-history">
      <div class="fh-header"><span>📜 操作历史</span><button class="btn-sm" @click="showFieldHistory=false">✕</button></div>
      <div class="fh-body">
        <div v-for="(h, hi) in fieldHistory.slice().reverse()" :key="hi" class="fh-entry">
          <span class="fh-time">{{ new Date(h.timestamp).toLocaleTimeString() }}</span>
          <span class="fh-action">{{ h.action }}</span>
          <span class="fh-detail">{{ h.details }}</span>
        </div>
        <div v-if="fieldHistory.length===0" class="fh-empty">暂无操作记录</div>
      </div>
    </div>
    <!-- Export Modal -->
    <div v-if="showExportModal" class="modal-overlay" @click.self="showExportModal=false">
      <div class="modal export-modal">
        <div class="modal-header"><span>📤 导出表单</span><button class="btn-sm" @click="showExportModal=false">✕</button></div>
        <div class="modal-body">
          <div class="export-tabs">
            <button :class="['exp-tab',{active: exportFormat==='json'}]" @click="exportFormat='json'">JSON</button>
            <button :class="['exp-tab',{active: exportFormat==='yaml'}]" @click="exportFormat='yaml'">YAML</button>
            <button :class="['exp-tab',{active: exportFormat==='html'}]" @click="exportFormat='html'">HTML</button>
          </div>
          <textarea class="export-textarea" readonly>{{ exportResult }}</textarea>
        </div>
        <div class="modal-footer">
          <button class="btn" @click="copyExportResult()">📋 复制</button>
          <button class="btn" @click="()=>{const b=new Blob([exportResult],{type:'text/plain'});const u=URL.createObjectURL(b);const a=document.createElement('a');a.href=u;a.download='form.'+exportFormat;a.click();}">💾 下载</button>
          <button class="btn btn-ghost" @click="showExportModal=false">关闭</button>
        </div>
      </div>
    </div>
    <!-- Field Stats Panel -->
    <div v-if="showFieldStats" class="field-stats-panel">
      <div class="fsp-header"><span>📊 字段统计</span><button class="btn-sm" @click="showFieldStats=false">✕</button></div>
      <div class="fsp-body">
        <div class="fsp-grid">
          <div class="fsp-item"><span class="fsp-value">{{ getFormFieldStats().total }}</span><span class="fsp-label">总字段</span></div>
          <div class="fsp-item"><span class="fsp-value" style="color:var(--color-success)">{{ getFormFieldStats().required }}</span><span class="fsp-label">必填</span></div>
          <div class="fsp-item"><span class="fsp-value" style="color:var(--color-warning)">{{ getFormFieldStats().withValidation }}</span><span class="fsp-label">有验证</span></div>
          <div class="fsp-item"><span class="fsp-value" style="color:var(--color-primary)">{{ getFormFieldStats().withCondition }}</span><span class="fsp-label">有条件</span></div>
        </div>
        <div class="fsp-types">
          <div v-for="(count, type) in getFormFieldStats().byType" :key="type" class="fsp-type-row">
            <span class="fsp-type">{{ type }}</span>
            <div class="fsp-bar"><div class="fsp-bar-fill" :style="{width: (count/getFormFieldStats().total*100)+'%'}"></div></div>
            <span class="fsp-count">{{ count }}</span>
          </div>
        </div>
      </div>
    </div>
    <!-- Bulk Edit Panel -->
    <div v-if="showBulkEdit" class="bulk-edit-panel">
      <div class="be-header">🔧 批量编辑</div>
      <div class="be-body">
        <select v-model="bulkEditAction" class="be-select">
          <option value="setRequired">设置必填</option><option value="setPlaceholder">设置占位符</option>
          <option value="setDisabled">设置禁用</option><option value="readOnly">设置只读</option>
        </select>
        <input v-model="bulkEditValue" placeholder="值" class="be-input" />
        <button class="btn" @click="bulkApplyAction(bulkEditAction, bulkEditValue)">✓ 应用</button>
      </div>
    </div>
    <!-- Validation Summary Badge -->
    <div v-if="validationSummary.errors > 0" class="val-summary-badge">
      <span class="vsb-icon">⚠️</span>
      <span>{{ validationSummary.errors }} 个验证错误</span>
    </div>
    <!-- Audit Log Panel -->
    <div v-if="showAuditPanel" class="audit-panel">
      <div class="ap-header"><span>📋 审计日志</span><button class="btn-sm" @click="showAuditPanel=false">✕</button></div>
      <div class="ap-body">
        <div v-for="(log, li) in auditLogs.slice().reverse()" :key="log.id" class="audit-entry">
          <span class="audit-time">{{ new Date(log.timestamp).toLocaleTimeString() }}</span>
          <span class="audit-user">{{ log.user }}</span>
          <span class="audit-action">{{ log.action }}</span>
          <span v-if="log.fieldKey" class="audit-field">{{ log.fieldKey }}</span>
        </div>
        <div v-if="auditLogs.length===0" class="audit-empty">暂无审计记录</div>
      </div>
    </div>

<!-- Cross-Field Calculator Panel -->
<div v-if="showCrossFieldPanel" class="modal-overlay" @click.self="showCrossFieldPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>跨字段计算</span><button class="btn-close" @click="showCrossFieldPanel=false">×</button></div>
    <div class="modal-body">
      <div class="cf-add">
        <input v-model="newCalcSource" placeholder="源字段" class="cf-input" />
        <input v-model="newCalcFormula" placeholder="公式 e.g. a*b+c" class="cf-input" />
        <input v-model="newCalcTarget" placeholder="目标字段" class="cf-input" />
        <button class="btn-sm" @click="addCrossFieldCalc()">添加</button>
      </div>
      <div class="cf-list">
        <div v-for="c in crossFieldCalcs" :key="c.id" class="cf-item">
          <label class="cf-toggle"><input type="checkbox" v-model="c.enabled"/><span></span></label>
          <span class="cf-src">{{c.sourceFields.join('+')}}</span>
          <span class="cf-op">→</span>
          <span class="cf-formula">{{c.formula}}</span>
          <span class="cf-tgt">{{c.targetField}}</span>
          <button class="btn-sm btn-danger" @click="removeCrossFieldCalc(c.id)">×</button>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Data Source Panel -->
<div v-if="showDataSourcePanel" class="modal-overlay" @click.self="showDataSourcePanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>数据源管理</span><button class="btn-close" @click="showDataSourcePanel=false">×</button></div>
    <div class="modal-body">
      <div class="ds-add"><button class="btn-sm" @click="addDataSource()">+ 添加数据源</button></div>
      <div class="ds-list">
        <div v-for="ds in dataSources" :key="ds.id" class="ds-item">
          <select v-model="ds.method" class="ds-method"><option>GET</option><option>POST</option><option>PUT</option><option>DELETE</option></select>
          <input v-model="ds.url" placeholder="API URL" class="ds-url" />
          <button class="btn-sm" @click="testDataSource(ds)">测试</button>
          <label class="ds-toggle"><input type="checkbox" v-model="ds.enabled"/><span></span></label>
          <button class="btn-sm btn-danger" @click="removeDataSource(ds.id)">×</button>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Field Permissions Panel -->
<div v-if="showFieldPermissionsPanel" class="modal-overlay" @click.self="showFieldPermissionsPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>字段权限配置</span><button class="btn-close" @click="showFieldPermissionsPanel=false">×</button></div>
    <div class="modal-body">
      <div class="fp-table">
        <div class="fp-hdr"><span>角色</span><span>只读</span><span>编辑</span><span>删除</span><span>隐藏</span></div>
        <div v-for="p in fieldPermissions" :key="p.roleId" class="fp-row">
          <span class="fp-role">{{p.role}}</span>
          <label class="fp-check"><input type="checkbox" v-model="p.canRead"/><span></span></label>
          <label class="fp-check"><input type="checkbox" v-model="p.canWrite"/><span></span></label>
          <label class="fp-check"><input type="checkbox" v-model="p.canDelete"/><span></span></label>
          <label class="fp-check"><input type="checkbox" v-model="p.canHide"/><span></span></label>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Internationalization Panel -->
<div v-if="showIntlPanel" class="modal-overlay" @click.self="showIntlPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>多语言配置</span><button class="btn-close" @click="showIntlPanel=false">×</button></div>
    <div class="modal-body">
      <div class="il-mode">
        <button v-for="m in ['zh','en','ja','ko']" :key="m" :class="['il-btn',{active:intlMode===m}]" @click="intlMode=m">{{m.toUpperCase()}}</button>
      </div>
      <div class="il-input">
        <textarea v-model="intlText" class="il-textarea" placeholder="输入翻译文本..." rows="4"></textarea>
        <div class="il-actions">
          <button class="btn-sm" @click="saveIntlText()">保存</button>
          <button class="btn-sm" @click="intlText=''">清空</button>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Performance Simulation Panel -->
<div v-if="showSimPanel" class="modal-overlay" @click.self="showSimPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>性能模拟</span><button class="btn-close" @click="showSimPanel=false">×</button></div>
    <div class="modal-body">
      <div class="sim-config">
        <div class="sim-row"><span>字段数</span><input v-model.number="simLoad.fieldCount" type="number" min="1" max="500" class="sim-num" /></div>
        <div class="sim-row"><span>并发用户</span><input v-model.number="simLoad.concurrentUsers" type="number" min="1" max="1000" class="sim-num" /></div>
        <div class="sim-row"><span>时长(秒)</span><input v-model.number="simLoad.durationSec" type="number" min="1" max="300" class="sim-num" /></div>
        <button class="btn-sm btn-primary" :disabled="simRunning" @click="runSimulation()">{{simRunning ? '模拟中...' : '开始模拟'}}</button>
      </div>
      <div v-if="simProgress > 0" class="sim-progress">
        <div class="sim-bar-wrap"><div class="sim-bar" :style="{width:simProgress+'%'}"></div></div>
        <span>{{simProgress}}%</span>
      </div>
      <div v-if="simResult" class="sim-result">
        <div class="sim-metric"><span class="sim-val">{{simResult.avgResponseMs.toFixed(1)}}</span><span class="sim-label">平均响应(ms)</span></div>
        <div class="sim-metric"><span class="sim-val">{{simResult.p99Ms.toFixed(1)}}</span><span class="sim-label">P99响应(ms)</span></div>
        <div class="sim-metric"><span class="sim-val">{{(simResult.errorRate*100).toFixed(2)}}%</span><span class="sim-label">错误率</span></div>
        <div class="sim-metric"><span class="sim-val">{{simResult.throughput.toFixed(0)}}</span><span class="sim-label">吞吐量(req/s)</span></div>
      </div>
    </div>
  </div>
</div>

<!-- Accessibility Panel -->
<div v-if="showA11yPanel" class="modal-overlay" @click.self="showA11yPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>无障碍检查</span><button class="btn-close" @click="showA11yPanel=false">×</button></div>
    <div class="modal-body">
      <div class="a11-list">
        <div v-for="iss in a11yIssues" :key="iss.fieldId" class="a11-item" :style="{borderLeftColor:iss.severity==='high'?'#ef4444':iss.severity==='medium'?'#f59e0b':'#10b981'}">
          <span class="a11-sev">{{iss.severity.toUpperCase()}}</span>
          <span class="a11-iss">{{iss.issue}}</span>
          <span v-if="iss.suggestion" class="a11-sug">{{iss.suggestion}}</span>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Dependency Graph Panel -->
<div v-if="showDepGraphPanel" class="modal-overlay" @click.self="showDepGraphPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>字段依赖图</span><button class="btn-close" @click="showDepGraphPanel=false">×</button></div>
    <div class="modal-body">
      <div class="dg-list">
        <div v-for="e in depEdges" :key="e.from+'-'+e.to" class="dg-item">
          <span class="dg-from">{{e.from}}</span><span class="dg-arrow">→</span><span class="dg-to">{{e.to}}</span><span class="dg-type">{{e.type}}</span>
        </div>
        <div v-if="!depEdges.length" class="dg-empty">暂无依赖关系</div>
      </div>
    </div>
  </div>
</div>

<!-- JSON Schema Panel -->
<div v-if="showJsonSchemaPanel" class="modal-overlay" @click.self="showJsonSchemaPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>JSON Schema 导出</span><button class="btn-close" @click="showJsonSchemaPanel=false">×</button></div>
    <div class="modal-body">
      <pre class="js-schema">{{jsonSchemaText}}</pre>
      <button class="btn-sm" @click="navigator.clipboard.writeText(jsonSchemaText);showToast('已复制','success')">复制</button>
    </div>
  </div>
</div>

<!-- Field Docs Panel -->
<div v-if="showFieldDocsPanel" class="modal-overlay" @click.self="showFieldDocsPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>字段文档</span><button class="btn-close" @click="showFieldDocsPanel=false">×</button></div>
    <div class="modal-body">
      <pre class="fd-docs">{{fieldDocsText}}</pre>
      <button class="btn-sm" @click="navigator.clipboard.writeText(fieldDocsText);showToast('已复制','success')">复制</button>
    </div>
  </div>
</div>

<!-- Bulk Edit Panel -->
<div v-if="showBulkEditPanel" class="modal-overlay" @click.self="showBulkEditPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>批量编辑</span><button class="btn-close" @click="showBulkEditPanel=false">×</button></div>
    <div class="modal-body">
      <div class="be-grid">
        <input v-model="bulkEditField" placeholder="字段key(留空=全部)" class="be-input" />
        <select v-model="bulkEditAction" class="be-select"><option value="required">设为必填</option><option value="label">修改标签</option><option value="placeholder">修改占位符</option></select>
        <input v-model="bulkEditValue" placeholder="值" class="be-input" />
        <button class="btn-sm btn-primary" @click="applyBulkEdit()">应用</button>
      </div>
    </div>
  </div>
</div>

<!-- Search Panel -->
<div v-if="showSearchPanel" class="modal-overlay" @click.self="showSearchPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>搜索字段</span><button class="btn-close" @click="showSearchPanel=false">×</button></div>
    <div class="modal-body">
      <input v-model="searchQuery" placeholder="输入关键词..." class="search-input" @input="doSearch()" />
      <div class="search-results">
        <div v-for="f in searchResults" :key="f.key" class="search-item">
          <span class="sr-key">{{f.key}}</span><span class="sr-label">{{f.label}}</span><span class="sr-type">{{f.type}}</span>
        </div>
        <div v-if="!searchResults.length && searchQuery" class="search-empty">无结果</div>
      </div>
    </div>
  </div>
</div>

<!-- Type Filter Panel -->
<div v-if="showTypeFilter" class="modal-overlay" @click.self="showTypeFilter=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>类型过滤</span><button class="btn-close" @click="showTypeFilter=false">×</button></div>
    <div class="modal-body">
      <div class="type-grid">
        <div v-for="t in ['text','textarea','number','date','select','radio','checkbox','file','phone','email']" :key="t" :class="['type-chip',{active:selectedTypes.includes(t)}]" @click="toggleType(t)">{{t}}</div>
      </div>
      <button class="btn-sm" @click="showTypeFilter=false">确认</button>
    </div>
  </div>
</div>

<!-- Export History Panel -->
<div v-if="showExportHistory" class="modal-overlay" @click.self="showExportHistory=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>导出历史</span><button class="btn-close" @click="showExportHistory=false">×</button></div>
    <div class="modal-body">
      <div class="hist-list">
        <div v-for="h in exportHistory" :key="h.id" class="hist-item">
          <span class="hist-format">{{h.format}}</span><span class="hist-time">{{new Date(h.exportedAt).toLocaleString('zh-CN')}}</span><span class="hist-size">{{h.size}}B</span>
        </div>
        <div v-if="!exportHistory.length" class="hist-empty">暂无导出记录</div>
      </div>
    </div>
  </div>
</div>

<!-- Import History Panel -->
<div v-if="showImportHistory" class="modal-overlay" @click.self="showImportHistory=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>导入历史</span><button class="btn-close" @click="showImportHistory=false">×</button></div>
    <div class="modal-body">
      <div class="hist-list">
        <div v-for="h in importHistory" :key="h.id" class="hist-item">
          <span class="hist-source">{{h.source}}</span><span class="hist-fields">{{h.fields}}字段</span><span class="hist-time">{{new Date(h.importedAt).toLocaleString('zh-CN')}}</span>
        </div>
        <div v-if="!importHistory.length" class="hist-empty">暂无导入记录</div>
      </div>
    </div>
  </div>
</div>

<!-- Validation History Panel -->
<div v-if="showValidationHistory" class="modal-overlay" @click.self="showValidationHistory=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>验证历史</span><button class="btn-close" @click="showValidationHistory=false">×</button></div>
    <div class="modal-body">
      <div class="hist-list">
        <div v-for="h in validationHistory" :key="h.id" class="hist-item">
          <span :class="h.valid?'hist-valid':'hist-invalid'">{{h.valid?'通过':'失败'}}</span>
          <span>错误:{{h.errors}}</span><span>警告:{{h.warnings}}</span><span class="hist-time">{{new Date(h.timestamp).toLocaleString('zh-CN')}}</span>
        </div>
        <div v-if="!validationHistory.length" class="hist-empty">暂无验证记录</div>
      </div>
    </div>
  </div>
</div>

<!-- Compare Panel -->
<div v-if="showComparePanel" class="modal-overlay" @click.self="showComparePanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>表单对比</span><button class="btn-close" @click="showComparePanel=false">×</button></div>
    <div class="modal-body">
      <div class="compare-grid">
        <div><span>表单A</span><select v-model="compareFormA" class="cmp-select"><option v-for="f in forms" :value="f.id">{{f.name}}</option></select></div>
        <div><span>表单B</span><select v-model="compareFormB" class="cmp-select"><option v-for="f in forms" :value="f.id">{{f.name}}</option></select></div>
      </div>
      <button class="btn-sm" @click="runCompare()">对比</button>
      <div v-if="compareResult" class="compare-result">
        <span class="comp-add">新增: {{compareResult.added}}</span>
        <span class="comp-rm">删除: {{compareResult.removed}}</span>
        <span class="comp-mod">修改: {{compareResult.modified}}</span>
      </div>
    </div>
  </div>
</div>

<!-- Restore Panel -->
<div v-if="showRestorePanel" class="modal-overlay" @click.self="showRestorePanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>版本恢复</span><button class="btn-close" @click="showRestorePanel=false">×</button></div>
    <div class="modal-body">
      <div class="ver-list">
        <div v-for="(v,i) in restoreVersions" :key="v.id" class="ver-item">
          <span>{{v.label}}</span><span>{{new Date(v.timestamp).toLocaleString('zh-CN')}}</span><span>{{v.fieldCount}}字段</span>
          <button class="btn-sm" @click="restoreVersion(i)">恢复</button>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Lock Panel -->
<div v-if="showLockPanel" class="modal-overlay" @click.self="showLockPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>字段锁定</span><button class="btn-close" @click="showLockPanel=false">×</button></div>
    <div class="modal-body">
      <div class="lock-list">
        <div v-for="f in lockedFields" :key="f.fieldId" class="lock-item">
          <span>{{f.fieldName}}</span><span>{{f.lockedBy}}</span>
          <button class="btn-sm btn-danger" @click="unlockField(f.fieldId)">解锁</button>
        </div>
        <div v-if="!lockedFields.length" class="lock-empty">暂无锁定字段</div>
      </div>
    </div>
  </div>
</div>

<!-- Publish Panel -->
<div v-if="showPublishPanel" class="modal-overlay" @click.self="showPublishPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>发布管理</span><button class="btn-close" @click="showPublishPanel=false">×</button></div>
    <div class="modal-body">
      <button class="btn-sm btn-primary" @click="publishForm()">发布当前版本</button>
      <div class="pub-list">
        <div v-for="p in publishVersions" :key="p.id" class="pub-item">
          <span>{{p.version}}</span><span>{{p.publishedBy}}</span><span :class="'pub-'+p.status">{{p.status}}</span>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Preview Panel -->
<div v-if="showPreviewPanel" class="modal-overlay" @click.self="showPreviewPanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>数据预览</span><button class="btn-close" @click="showPreviewPanel=false">×</button></div>
    <div class="modal-body">
      <pre class="preview-data">{{JSON.stringify(previewData,null,2)}}</pre>
      <button class="btn-sm" @click="generatePreview()">刷新</button>
    </div>
  </div>
</div>

<!-- Validate Panel -->
<div v-if="showValidatePanel" class="modal-overlay" @click.self="showValidatePanel=false">
  <div class="modal glass-card">
    <div class="modal-header"><span>验证结果</span><button class="btn-close" @click="showValidatePanel=false">×</button></div>
    <div class="modal-body">
      <div class="val-list">
        <div v-for="e in validateErrors" :key="e.field" class="val-item" :class="e.severity">
          <span class="val-field">{{e.field}}</span><span class="val-msg">{{e.error}}</span>
        </div>
        <div v-if="!validateErrors.length" class="val-empty">所有字段验证通过 ✓</div>
      </div>
    </div>
  </div>
</div>
</template>
<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { api } from '@oa4rust/sdk'
interface FormField {
  id: string; type: string; label: string; key: string
  placeholder?: string; defaultValue?: string; required?: boolean; disabled?: boolean
  rows?: number; min?: number; max?: number; optionsStr?: string
  description?: string; cssClass?: string; helpText?: string
  maxLength?: number; minLength?: number; pattern?: string; step?: number
  readonly?: boolean; hidden?: boolean; colSpan?: number
  conditions?: FieldCondition[]
  validation?: FieldValidation
}
interface FieldCondition { operator: string; value: string; fieldKey?: string }
interface FieldValidation { required?: boolean; min?: number; max?: number; minLength?: number; maxLength?: number; pattern?: string; patternMsg?: string; customMsg?: string }
interface FormDef {
  id?: string; name: string; flag: string; desc?: string
  layout?: 'single'|'two_col'|'three_col'
  fields: FormField[]; updatedAt?: string; version?: string
  settings?: { showReset?: boolean; showSubmit?: boolean; layoutClass?: string }
}
// ── Advanced Form Types ───────────────────────────────────────────
interface FieldDependency { sourceField: string; operator: string; values: string[]; action: 'show'|'hide' }
interface FormSection { id: string; label: string; fields: string[] }
interface FormTab { id: string; label: string; icon: string; fields: string[] }
interface FormValidationRule { fieldKey: string; rule: string; message: string }
interface FormSubmission { status: 'idle'|'submitting'|'success'|'error'; data: Record<string,any>; errors: Record<string,string> }
interface FieldTemplate { name: string; icon: string; fields: string[] }
interface FormDef {
  id?: string; name: string; flag: string; desc?: string
  fields: FormField[]; updatedAt?: string
}
const fieldTypes = [
  { type: 'text',     label: '文本',   icon: '📝' },
  { type: 'textarea', label: '多行文本', icon: '📄' },
  { type: 'number',   label: '数字',   icon: '🔢' },
  { type: 'date',     label: '日期',   icon: '📅' },
  { type: 'select',   label: '下拉',   icon: '📋' },
  { type: 'checkbox', label: '单选',   icon: '◉' },
  { type: 'checkbox_group', label: '多选', icon: '☑' },
  { type: 'file',     label: '文件',   icon: '📎' },
  { type: 'email',    label: '邮箱',   icon: '✉' },
  { type: 'phone',    label: '手机',   icon: '📱' },
]
const layoutTypes = [
  { type: 'section',  label: '分组', icon: '📁' },
  { type: 'row_start', label: '开始行', icon: '↔️' },
  { type: 'row_end',    label: '结束行', icon: '↩️' },
  { type: 'columns',    label: '多列布局', icon: '▥' },
  { type: 'spacer',     label: '间距', icon: '↕️' },
  { type: 'divider',    label: '分割线', icon: '—' },
  { type: 'html',       label: 'HTML', icon: '📝' },
]
const extraFieldTypes = [
  { type: 'rating', label: '评分', icon: '⭐' },
  { type: 'slider', label: '滑块', icon: '🎚️' },
  { type: 'color', label: '颜色', icon: '🎨' },
  { type: 'signature', label: '签字', icon: '✍️' },
  { type: 'image', label: '图片', icon: '🖼️' },
  { type: 'rich_text', label: '富文本', icon: '📝' },
  { type: 'json', label: 'JSON', icon: '{ }' },
  { type: 'cascader', label: '级联', icon: '🌳' },
  { type: 'switch', label: '开关', icon: '🔘' },
  { type: 'upload', label: '上传', icon: '📤' },
  { type: 'map', label: '地图', icon: '🗺️' },
  { type: 'code', label: '代码', icon: '</>' },
  // Additional field types
  { type: "rating", label: "评分", icon: "⭐", key: "rating_field" },
  { type: "slider", label: "滑块", icon: "🎚️", key: "slider_field" },
  { type: "color", label: "颜色选择", icon: "🎨", key: "color_field" },
  { type: "signature", label: "签名", icon: "✍️", key: "signature_field" },
  { type: "image", label: "图片上传", icon: "🖼️", key: "image_field" },
  { type: "rich_text", label: "富文本", icon: "📝", key: "rich_text_field" },
  { type: "json", label: "JSON编辑器", icon: "🔧", key: "json_field" },
  { type: "cascader", label: "级联选择", icon: "🌳", key: "cascader_field" },
  { type: "switch", label: "开关", icon: "🔘", key: "switch_field" },
  { type: "upload", label: "文件上传", icon: "📤", key: "upload_field" },
  { type: "map", label: "地图选点", icon: "📍", key: "map_field" },
  { type: "code", label: "代码编辑器", icon: "💻", key: "code_field" },
  { type: "time", label: "时间选择", icon: "🕐", key: "time_field" },
  { type: "datetime", label: "日期时间", icon: "📅", key: "datetime_field" },
  { type: "rate", label: "等级评定", icon: "📊", key: "rate_field" },
  { type: "transfer", label: "穿梭框", icon: "🔄", key: "transfer_field" },
  { type: "tree_select", label: "树形选择", icon: "🌲", key: "tree_select_field" },
  { type: "date_range", label: "日期范围", icon: "📆", key: "date_range_field" },
  { type: "time_range", label: "时间范围", icon: "⏰", key: "time_range_field" },
  { type: "number_range", label: "数字范围", icon: "🔢", key: "number_range_field" },
]
const mode = ref<'edit'|'preview'|'schema'>('edit')
const listFilter = ref('')
const showFieldTemplates = ref(false)
const showSchema = ref(false)
const showIoModal = ref(false)
const importJsonText = ref('')
const columnCount = ref<1|2|3>(1)
const formHistory = ref<{fields: FormField[]; timestamp: number; label: string}[]>([])
const historyIdx = ref(-1)
const canUndo = computed(() => historyIdx.value > 0)
const canRedo = computed(() => historyIdx.value < formHistory.value.length - 1)
const previewData = ref<Record<string,any>>({})
const previewErrors = ref<Record<string,string>>({})
const forms = ref<FormDef[]>([])
const formsLoading = ref(false)
const currentForm = ref<FormDef|null>(null)
const selectedField = ref<FormField|null>(null)
const showAdvancedProps = ref(false)
const showValidationPanel = ref(false)
const showDependencyPanel = ref(false)
const submissionResult = ref<FormSubmission|null>(null)
const sections = ref<FormSection[]>([])
const tabs = ref<FormTab[]>([])
const validationRules = ref<FormValidationRule[]>([])
const showSchemaExport = ref(false)
const fieldTemplates2: FieldTemplate[] = [
  { name: "基础表单", icon: "📋", fields: ["text", "textarea", "number", "date"] },
  { name: "选择表单", icon: "📝", fields: ["select", "checkbox", "checkbox_group", "radio"] },
  { name: "联系表单", icon: "📞", fields: ["text", "email", "phone", "textarea"] },
  { name: "反馈表单", icon: "💬", fields: ["select", "textarea", "rating", "file"] },
  { name: "注册表单", icon: "🆕", fields: ["text", "email", "password", "phone", "date"] },
  { name: "问卷表单", icon: "📊", fields: ["select", "checkbox_group", "rating", "slider", "textarea"] },
  { name: "订单表单", icon: "🛒", fields: ["select", "text", "number", "date", "textarea"] },
  { name: "审批表单", icon: "✅", fields: ["text", "select", "textarea", "file", "signature"] },
  { name: "调查表单", icon: "🔍", fields: ["radio", "checkbox", "rating", "slider", "textarea"] },
  { name: "预约表单", icon: "📅", fields: ["text", "phone", "date", "time", "textarea"] },
]
const draggedType = ref<string|null>(null)
const filteredForms = computed(() =>
  listFilter.value
    ? forms.value.filter(f => (f.name||'').toLowerCase().includes(listFilter.value.toLowerCase()) || (f.flag||'').toLowerCase().includes(listFilter.value.toLowerCase()))
    : forms.value
)
function onDragStart(e: DragEvent, ft: { type: string }) {
  draggedType.value = ft.type
  e.dataTransfer?.setData('text/plain', ft.type)
}
function onDrop(e: DragEvent) {
  e.preventDefault()
  const type = e.dataTransfer?.getData('text/plain') || draggedType.value
  if (type) addField({ type, label: '' })
  draggedType.value = null
}
function genId() { return 'f_' + Date.now() + '_' + Math.random().toString(36).slice(2,6) }
function makeField(type: string): FormField {
  const d: Record<string, Partial<FormField>> = {
    text: { label: '文本字段', key: 'text_field', placeholder: '请输入' },
    textarea: { label: '多行文本', key: 'textarea_field', placeholder: '请输入内容', rows: 4 },
    number: { label: '数字', key: 'number_field', placeholder: '请输入数字' },
    date: { label: '日期', key: 'date_field' },
    select: { label: '下拉选择', key: 'select_field', optionsStr: 'option1|选项1\noption2|选项2' },
    checkbox: { label: '单选框', key: 'checkbox_field' },
    checkbox_group: { label: '多选框', key: 'checkbox_group_field', optionsStr: 'a|A\nb|B\nc|C' },
    file: { label: '文件上传', key: 'file_field' },
    email: { label: '邮箱', key: 'email_field', placeholder: 'example@domain.com' },
    phone: { label: '手机号', key: 'phone_field', placeholder: '请输入手机号' },
    section: { label: '分组标题', key: '' },
    section_end: { label: '分组结束', key: '' },
    row_start: { label: '开始行', key: '' },
    row_end: { label: '结束行', key: '' },
    columns: { label: '多列布局', key: '' },
    spacer: { label: '间距', key: '' },
    divider: { label: '分割线', key: '' },
    html: { label: 'HTML内容', key: '' },
    rating: { label: '评分', key: 'rating_field', min: 1, max: 5 },
    slider: { label: '滑块', key: 'slider_field', min: 0, max: 100, defaultValue: '50' },
    color: { label: '颜色选择', key: 'color_field', defaultValue: '#000000' },
    signature: { label: '签字', key: 'signature_field' },
    image: { label: '图片上传', key: 'image_field' },
    rich_text: { label: '富文本', key: 'rich_text_field' },
    json: { label: 'JSON编辑器', key: 'json_field' },
    cascader: { label: '级联选择', key: 'cascader_field' },
    switch: { label: '开关', key: 'switch_field', defaultValue: 'false' },
    upload: { label: '文件上传', key: 'upload_field' },
    map: { label: '地图', key: 'map_field' },
    code: { label: '代码编辑', key: 'code_field' },
  }
  return { id: genId(), type, ...d[type], required: false, disabled: false } as FormField
}
function addField(ft: { type: string; label?: string }) {
  if (!currentForm.value) { resetForm(); return }
  const field = makeField(ft.type)
  if (ft.label) field.label = ft.label
  currentForm.value.fields.push(field)
  selectedField.value = field
}
function removeField(i: number) {
  if (!currentForm.value) return
  currentForm.value.fields.splice(i, 1)
  if (selectedField.value) selectedField.value = null
}
function moveField(i: number, d: number) {
  if (!currentForm.value) return
  const n = i + d
  if (n < 0 || n >= currentForm.value.fields.length) return
  const fs = currentForm.value.fields
  ;[fs[i], fs[n]] = [fs[n], fs[i]]
}
function selectField(f: FormField) { selectedField.value = f }
function getFieldIcon(t: string) {
  return [...fieldTypes, ...layoutTypes].find(f => f.type === t)?.icon ?? '⬜'
}
function parseOptions(s?: string) {
  if (!s?.trim()) return []
  return s.split('\n').filter(Boolean).map(l => { const [v, lb = v] = l.split('|'); return { value: v.trim(), label: lb.trim() } })
}
async function loadForms() {
  formsLoading.value = true
  try { const r: any = await api.get('/jaxrs/form/list'); forms.value = r?.data ?? [] }
  catch { forms.value = [] } finally { formsLoading.value = false }
}
async function loadForm(f: FormDef) {
  try {
    const r: any = await api.get(`/jaxrs/form/${f.id}`)
    const data = r?.data ?? f
    currentForm.value = {
      id: data.id || f.id, name: data.name||data.title||'', flag: data.flag||data.formFlag||'',
      desc: data.description||data.desc||'',
      fields: Array.isArray(data.fields) ? data.fields.map((ff: any) => ({
        id: ff.id || genId(), type: ff.type||'text', label: ff.label||ff.name||'',
        key: ff.key||ff.fieldKey||'', placeholder: ff.placeholder||'',
        defaultValue: ff.defaultValue||ff.default||'', required: ff.required||false,
        disabled: ff.disabled||false, rows: ff.rows||4, min: ff.min, max: ff.max,
        optionsStr: ff.optionsStr || (Array.isArray(ff.options) ? ff.options.map((o:any)=>`${o.value}|${o.label}`).join('\n') : ''),
      })) : [],
      updatedAt: data.updatedAt||f.updatedAt,
    }
    selectedField.value = null
  } catch { currentForm.value = { ...f, fields: [] }; selectedField.value = null }
}
function resetForm() { currentForm.value = { name: '', flag: '', fields: [] }; selectedField.value = null }
async function saveForm() {
  if (!currentForm.value?.name.trim()) { alert('请输入表单名称'); return }
  try {
    const payload = {
      name: currentForm.value.name, flag: currentForm.value.flag,
      description: currentForm.value.desc,
      fields: currentForm.value.fields.map(f => ({
        type: f.type, label: f.label, key: f.key, placeholder: f.placeholder,
        defaultValue: f.defaultValue, required: f.required, disabled: f.disabled,
        rows: f.rows, min: f.min, max: f.max,
        options: parseOptions(f.optionsStr),
      })),
    }
    if (currentForm.value.id) await api.put(`/jaxrs/form/${currentForm.value.id}`, payload)
    else await api.post('/jaxrs/form', payload)
    await loadForms(); alert('保存成功')
  } catch (e: any) { alert('保存失败: ' + (e?.message ?? '')) }
}
function togglePreview() { mode.value = mode.value === 'preview' ? 'edit' : 'preview' }
// --- Form History ---
function pushFormHistory() {
  if (!currentForm.value) return
  formHistory.value = formHistory.value.slice(0, historyIdx.value + 1)
  formHistory.value.push({ fields: JSON.parse(JSON.stringify(currentForm.value.fields)), timestamp: Date.now(), label: '自动保存 ' + new Date().toLocaleTimeString('zh-CN') })
  historyIdx.value = formHistory.value.length - 1
}
function formUndo() { if (!canUndo.value || !currentForm.value) return; historyIdx.value--; currentForm.value.fields = JSON.parse(JSON.stringify(formHistory.value[historyIdx.value].fields)) }
function formRedo() { if (!canRedo.value || !currentForm.value) return; historyIdx.value++; currentForm.value.fields = JSON.parse(JSON.stringify(formHistory.value[historyIdx.value].fields)) }
// --- Field Templates ---
const fieldTemplates = [
  { name: '姓名信息', icon: '👤', fields: [{type:'text',label:'姓名',key:'name',required:true},{type:'email',label:'邮箱',key:'email'},{type:'phone',label:'手机',key:'phone'}] },
  { name: '订单信息', icon: '📦', fields: [{type:'text',label:'订单号',key:'order_id'},{type:'date',label:'日期',key:'order_date'},{type:'number',label:'金额',key:'amount',min:0},{type:'select',label:'状态',key:'status',optionsStr:'待支付|待支付\n已支付|已支付\n已完成|已完成'}] },
  { name: '员工信息', icon: '👥', fields: [{type:'text',label:'姓名',key:'name',required:true},{type:'select',label:'部门',key:'dept',optionsStr:'技术部|技术部\n销售部|销售部\n人事部|人事部'},{type:'date',label:'入职日期',key:'hire_date'},{type:'number',label:'薪资',key:'salary',min:0}] },
  { name: '反馈表单', icon: '💬', fields: [{type:'select',label:'类型',key:'type',optionsStr:'问题|问题反馈\n建议|建议\n其他|其他'},{type:'textarea',label:'内容',key:'content',rows:5,required:true},{type:'email',label:'联系方式',key:'contact'}] },
  { name: '调查问卷', icon: '📊', fields: [{type:'text',label:'姓名',key:'name'},{type:'select',label:'年龄',key:'age',optionsStr:'18-25|18-25岁\n26-35|26-35岁\n36-50|36-50岁'},{type:'rating',label:'评分',key:'rating',min:1,max:5},{type:'textarea',label:'意见',key:'comments',rows:3}] },
  { name: '预约表单', icon: '📅', fields: [{type:'text',label:'姓名',key:'name',required:true},{type:'phone',label:'手机',key:'phone',required:true},{type:'date',label:'日期',key:'date',required:true},{type:'textarea',label:'备注',key:'notes',rows:2}] },
  { name: '地址信息', icon: '📍', fields: [{type:'text',label:'省份',key:'province'},{type:'text',label:'城市',key:'city'},{type:'textarea',label:'详细地址',key:'address',rows:3},{type:'text',label:'邮编',key:'zip'}] },
  { name: '登录表单', icon: '🔐', fields: [{type:'email',label:'邮箱',key:'email',required:true},{type:'text',label:'密码',key:'password'},{type:'checkbox',label:'记住我',key:'remember'}] },
  { name: '注册表单', icon: '📝', fields: [{type:'text',label:'用户名',key:'username',required:true},{type:'email',label:'邮箱',key:'email',required:true},{type:'text',label:'密码',key:'password',required:true},{type:'checkbox',label:'同意条款',key:'agree',required:true}] },
  { name: '联系人', icon: '📒', fields: [{type:'text',label:'姓名',key:'name'},{type:'phone',label:'手机',key:'phone'},{type:'email',label:'邮箱',key:'email'},{type:'text',label:'公司',key:'company'},{type:'textarea',label:'备注',key:'note',rows:2}] },
]
function applyTemplate(tpl: typeof fieldTemplates[0]) {
  if (!currentForm.value) return
  currentForm.value.fields = tpl.fields.map(f => makeField(f as any))
  selectedField.value = null
  pushFormHistory()
}
// --- Validation & Preview ---
function validatePreview(): boolean {
  if (!currentForm.value) return false
  previewErrors.value = {}
  for (const field of currentForm.value.fields) {
    if (['section','section_end','divider','spacer'].includes(field.type)) continue
    if (field.required && !previewData.value[field.key]) previewErrors.value[field.key] = field.label + ' 不能为空'
    if (field.pattern && previewData.value[field.key] && !new RegExp(field.pattern).test(previewData.value[field.key])) previewErrors.value[field.key] = field.label + ' 格式错误'
    if (field.minLength && previewData.value[field.key] && previewData.value[field.key].length < field.minLength) previewErrors.value[field.key] = field.label + ' 至少' + field.minLength + '字符'
    if (field.maxLength && previewData.value[field.key] && previewData.value[field.key].length > field.maxLength) previewErrors.value[field.key] = field.label + ' 最多' + field.maxLength + '字符'
    if (field.min !== undefined && previewData.value[field.key] && Number(previewData.value[field.key]) < field.min) previewErrors.value[field.key] = field.label + ' 不能小于' + field.min
    if (field.max !== undefined && previewData.value[field.key] && Number(previewData.value[field.key]) > field.max) previewErrors.value[field.key] = field.label + ' 不能大于' + field.max
  }
  return Object.keys(previewErrors.value).length === 0
}
async function submitPreview() {
  if (!validatePreview()) return
  try { await api.post('/jaxrs/form/submit', { formFlag: currentForm.value.flag, data: previewData.value }); alert('提交成功'); previewData.value = {}; previewErrors.value = {} }
  catch(e: any) { alert('提交失败: ' + (e?.message ?? '')) }
}
// --- Import/Export ---
function exportFormJson(): string { return JSON.stringify(currentForm.value, null, 2) }
function importFormJson(text: string) {
  try { const d = JSON.parse(text); currentForm.value = {...d, fields: (d.fields||[]).map((f:any)=>({...f,id:f.id||genId()}))}; selectedField.value=null; pushFormHistory() }
  catch { alert('JSON格式错误') }
}
function downloadFormJson() {
  const b = new Blob([exportFormJson()], {type:'application/json'}); const u = URL.createObjectURL(b)
  const a = document.createElement('a'); a.href=u; a.download=(currentForm.value?.flag||'form')+'.json'; a.click(); URL.revokeObjectURL(u)
}
// --- Schema ---
const schemaJson = computed(() => {
  if (!currentForm.value) return ''
  return JSON.stringify({name:currentForm.value.name,flag:currentForm.value.flag,layout:currentForm.value.layout,fields:currentForm.value.fields.map(f=>({type:f.type,label:f.label,key:f.key,required:f.required,validation:f.validation}))},null,2)
})
// --- Column Layout ---
function setColumnCount(n:1|2|3) { columnCount.value = n }
function getFieldWidth(type:string): string {
  if (['section','section_end','divider','spacer','html'].includes(type)) return '100%'
  return columnCount.value===1?'100%':columnCount.value===2?'50%':'33.33%'
}
// --- Bulk Operations ---
function batchCopyField() {
  if (!currentForm.value||!selectedField.value) return
  const orig={...selectedField.value,id:genId()}
  currentForm.value.fields.push(orig); selectedField.value=orig; pushFormHistory()
}
function batchDeleteField() {
  if (!currentForm.value||!selectedField.value) return
  const idx=currentForm.value.fields.findIndex(f=>f.id===selectedField.value!.id)
  if(idx!==-1){currentForm.value.fields.splice(idx,1);selectedField.value=null;pushFormHistory()}
}
// --- Conditional Logic ---
function addCondition() {
  if (!selectedField.value) return
  if (!(selectedField.value as any).conditions) (selectedField.value as any).conditions = []
  ;(selectedField.value as any).conditions.push({operator:'equals',value:'',fieldKey:''})
}
function removeCondition(i:number) {
  if (!selectedField.value) return
  const conds = (selectedField.value as any).conditions
  if (Array.isArray(conds)) conds.splice(i, 1)
}
onMounted(loadForms)
// ── Field Drag Reorder ────────────────────────────────────────────────
const dragFieldIdx = ref<number|null>(null)
const dragOverIdx = ref<number|null>(null)
function onFieldDragStart(e: DragEvent, idx: number) {
  dragFieldIdx.value = idx
  e.dataTransfer?.setData('text/plain', String(idx))
  if (e.dataTransfer) e.dataTransfer.effectAllowed = 'move'
}
function onFieldDragOver(e: DragEvent, idx: number) {
  e.preventDefault()
  dragOverIdx.value = idx
}
function onFieldDrop(e: DragEvent, idx: number) {
  e.preventDefault()
  if (dragFieldIdx.value === null || dragFieldIdx.value === idx || !currentForm.value) return
  const fields = currentForm.value.fields
  const from = dragFieldIdx.value
  const to = idx
  const [removed] = fields.splice(from, 1)
  fields.splice(to, 0, removed)
  dragFieldIdx.value = null
  dragOverIdx.value = null
}
// ── Conditional Display ────────────────────────────────────────────────
interface FieldCondition { operator: string; value: string; fieldKey?: string }
function fmtCondition(c: FieldCondition): string {
  const ops: Record<string, string> = { equals: '==', contains: 'includes', gt: '>', lt: '<' }
  return ops[c.operator] || c.operator
}
// ── Field Dependency Management ────────────────────────────────────
function addDependency() {
  if (!selectedField.value) return
  if (!selectedField.value.dependencies) selectedField.value.dependencies = []
  selectedField.value.dependencies.push({ sourceField: "", operator: "==", values: [""], action: "show" })
}
function removeDependency(idx: number) {
  if (!selectedField.value || !selectedField.value.dependencies) return
  selectedField.value.dependencies.splice(idx, 1)
}
// ── Section Management ─────────────────────────────────────────────
function addSection() {
  sections.value.push({ id: genId(), label: "分组 " + (sections.value.length + 1), fields: [] })
}
function removeSection(idx: number) { sections.value.splice(idx, 1) }
// ── Tab Management ─────────────────────────────────────────────────
function addTab() {
  tabs.value.push({ id: genId(), label: "页签 " + (tabs.value.length + 1), icon: "📑", fields: [] })
}
function removeTab(idx: number) { tabs.value.splice(idx, 1) }
// ── Validation Rules ────────────────────────────────────────────────
function addValidationRule() {
  validationRules.value.push({ fieldKey: "", rule: "required", message: "必填字段" })
}
function removeValidationRule(idx: number) { validationRules.value.splice(idx, 1) }
function validateAll(formData: Record<string,any>): { valid: boolean; errors: Record<string,string> } {
  const errors: Record<string,string> = {}
  for (const rule of validationRules.value) {
    const value = formData[rule.fieldKey]
    if (rule.rule === "required" && !value) errors[rule.fieldKey] = rule.message || "必填"
  }
  return { valid: Object.keys(errors).length === 0, errors }
}
// ── Form Submission Simulation ─────────────────────────────────────
function simulateSubmission(formData: Record<string,any>) {
  submissionResult.value = { status: "submitting", data: formData, errors: {} }
  setTimeout(() => {
    const validation = validateAll(formData)
    if (validation.valid) {
      submissionResult.value = { status: "success", data: formData, errors: {}, timestamp: Date.now() }
    } else {
      submissionResult.value = { status: "error", data: formData, errors: validation.errors }
    }
  }, 800)
}
// ── Schema Export ───────────────────────────────────────────────────
function exportSchema() {
  if (!currentForm.value) return
  const schema = {
    name: currentForm.value.name,
    flag: currentForm.value.flag,
    version: "1.0",
    fields: currentForm.value.fields.map(f => ({ ...f, dependencies: f.dependencies || [], validation: f.validation || null }))
  }
  schemaJson.value = JSON.stringify(schema, null, 2)
  showSchemaExport.value = true
}
// ── Field Type Helpers ──────────────────────────────────────────────
function isLayoutField(type: string): boolean {
  return ["section", "divider", "spacer", "tabs", "columns"].includes(type)
}
function getFieldCategory(type: string): string {
  const inputTypes = ["text", "textarea", "number", "date", "time", "datetime", "email", "phone", "password", "url", "color", "rating", "slider", "switch", "rate", "code", "json", "rich_text"]
  const selectTypes = ["select", "checkbox", "checkbox_group", "radio", "cascader", "transfer", "tree_select"]
  const uploadTypes = ["file", "image", "upload", "signature"]
  if (inputTypes.includes(type)) return "input"
  if (selectTypes.includes(type)) return "select"
  if (uploadTypes.includes(type)) return "upload"
  return "other"
}
// ── Bulk Operations ─────────────────────────────────────────────────
function batchSetRequired(flag: boolean) {
  if (!currentForm.value || !selectedField.value) return
  if (selectedField.value.validation) selectedField.value.validation.required = flag
  else selectedField.value.validation = { required: flag }
}
// ── Field Templates ─────────────────────────────────────────────────
function applyTemplate2(tpl: FieldTemplate) {
  if (!currentForm.value) return
  currentForm.value.fields = tpl.fields.map(type => makeField(type))
  pushFormHistory()
}
// ── Export/Import Enhancements ──────────────────────────────────────
function exportFormSchema() {
  if (!currentForm.value) return
    formName: currentForm.value.name,
    formFlag: currentForm.value.flag,
    columnCount: columnCount.value,
    fields: currentForm.value.fields.map(f => ({
      type: f.type, label: f.label, key: f.key,
      required: f.required, disabled: f.disabled,
      placeholder: f.placeholder,
      options: parseOptions(f.optionsStr),
      validation: f.validation,
    }))
  }
  const blob = new Blob([JSON.stringify(schema, null, 2)], { type: "application/json" })
  const url = URL.createObjectURL(blob)
  a.href = url; a.download = (currentForm.value.flag || "form") + "_schema.json"
  a.click(); URL.revokeObjectURL(url)
}
function importFormSchema(text: string) {
  try {
    if (!currentForm.value) resetForm()
    if (currentForm.value) {
      currentForm.value.name = schema.formName || currentForm.value.name
      currentForm.value.flag = schema.formFlag || currentForm.value.flag
      if (schema.fields) {
        currentForm.value.fields = schema.fields.map((f: any) => ({
          id: genId(), type: f.type, label: f.label, key: f.key,
          placeholder: f.placeholder, defaultValue: f.defaultValue,
          required: f.required || false, disabled: f.disabled || false,
          optionsStr: f.options ? f.options.map((o: any) => o.value + "|" + o.label).join("\n") : "",
          validation: f.validation || null,
          dependencies: f.dependencies || null,
        }))
      }
      pushFormHistory()
    }
  } catch (e) { alert("导入失败: 无效的JSON格式") }
}
// ── Advanced Interfaces ────────────────────────────────────────────
interface FieldAccessRule { fieldKey: string; role: string; action: "show"|"hide"|"readonly" }
interface FieldDataSource { fieldKey: string; url: string; method: string; keyField: string; labelField: string }
interface ConditionTree { id: string; type: "group"|"condition"; logic: "AND"|"OR"; conditions?: Array<{field: string; op: string; value: string}>; children?: ConditionTree[] }
interface FormLayoutConfig { columns: number; gutter: number; align: "left"|"center"|"right"; labelWidth: number; labelAlign: "left"|"right" }
interface FieldHistoryEntry { timestamp: number; action: "add"|"remove"|"move"|"modify"; fieldKey: string; details: string }
interface FieldOption { label: string; value: string; disabled?: boolean }
interface FormAuditLog { id: string; timestamp: number; user: string; action: string; fieldKey?: string; oldValue?: string; newValue?: string }
// ── Advanced State ──────────────────────────────────────────────────
const showAccessControl = ref(false)
const showDataSourcePanel = ref(false)
const showConditionTree = ref(false)
const showLayoutConfig = ref(false)
const showFieldHistory = ref(false)
const showValidationBuilder = ref(false)
const showPreviewDataEditor = ref(false)
const newAccessRule = ref<{fieldKey: string; role: string; action: string}>({ fieldKey: "", role: "admin", action: "show" })
const accessRules = ref<FieldAccessRule[]>([])
const newDataSource = ref<{fieldKey: string; url: string; method: string}>({ fieldKey: "", url: "", method: "GET" })
const conditionTreeRoot = ref<ConditionTree|null>(null)
const layoutConfig = ref<FormLayoutConfig>({ columns: 1, gutter: 16, align: "left", labelWidth: 100, labelAlign: "right" })
const fieldHistory = ref<FieldHistoryEntry[]>([])
const validationGroups = ref<Array<{id: string; name: string; fields: string[]; rules: string[]}>>([])
const showSectionManager = ref(false)
const newSectionName = ref('')
const showTabManager = ref(false)
const newTabName = ref('')
const newTabIcon = ref('📑')
const previewMode = ref<'form' | 'json' | 'schema'>('form')
const jsonPreview = ref('')
const schemaPreview = ref('')
const validationSummary = ref<{valid: boolean; errors: number; warnings: number}>({ valid: true, errors: 0, warnings: 0 })
const fieldDragSource = ref<number|null>(null)
const showExportModal = ref(false)
const exportFormat = ref<'json'|'yaml'|'html'>('json')
const exportResult = ref('')
const showFieldPicker = ref(false)
const fieldPickerSearch = ref('')
const showAdvancedConfig = ref(false)
const fieldPropsExpanded = ref<Record<string, boolean>>({})
const searchFieldQuery = ref('')
const showBulkEdit = ref(false)
const bulkEditAction = ref('')
const bulkEditValue = ref('')
const showFieldStats = ref(false)
const formFieldStats = ref<{total: number; byType: Record<string, number>; withValidation: number; withCondition: number; requiredCount: number}>({ total: 0, byType: {}, withValidation: 0, withCondition: 0, requiredCount: 0 })
const auditLogs = ref<FormAuditLog[]>([])
const showAuditPanel = ref(false)
const fieldOptionsCache = ref<Record<string, FieldOption[]>>({})
// ── Advanced Functions ─────────────────────────────────────────────
function toggleFieldProps(fieldKey: string) { fieldPropsExpanded.value[fieldKey] = !fieldPropsExpanded.value[fieldKey] }
function getFieldTypeStats(): Record<string, number> { const stats: Record<string, number> = {}; currentForm.value?.fields.forEach(f => { stats[f.type] = (stats[f.type] || 0) + 1 }); return stats }
function updateFieldStats() { const fields = currentForm.value?.fields || []; const byType: Record<string, number> = {}; let withValidation = 0, withCondition = 0, requiredCount = 0; fields.forEach(f => { byType[f.type] = (byType[f.type] || 0) + 1; if (f.validation?.required) requiredCount++; if (f.validation) withValidation++; if (f.condition) withCondition++; }); formFieldStats.value = { total: fields.length, byType, withValidation, withCondition, requiredCount }; }
function searchFields(query: string) { searchFieldQuery.value = query; if (!query.trim()) return []; return (currentForm.value?.fields || []).filter(f => f.label?.toLowerCase().includes(query.toLowerCase()) || f.key?.toLowerCase().includes(query.toLowerCase()) || f.type.toLowerCase().includes(query.toLowerCase())); }
function bulkApplyAction(action: string, value: string) { if (!currentForm.value) return; const fields = currentForm.value.fields; if (action === "setRequired") { fields.forEach(f => { if (f.validation) f.validation.required = value === "true"; }); } else if (action === "setPlaceholder") { fields.forEach(f => { f.placeholder = value; }); } else if (action === "setDisabled") { fields.forEach(f => { f.disabled = value === "true"; }); } else if (action === "readOnly") { fields.forEach(f => { f.readonly = value === "true"; }); } pushFormHistory(); showToast("批量操作已应用", "success"); }
function generateConditionTree(): ConditionTree { return { id: genId(), type: "group", logic: "AND", children: (currentForm.value?.fields || []).filter(f => f.condition).map(f => ({ id: genId(), type: "condition", logic: "AND", conditions: [{ field: f.condition?.field || "", op: f.condition?.operator || "=", value: f.condition?.value || "" }] })) }; }
function evaluateConditionTree(tree: ConditionTree|null, data: Record<string,any>): boolean { if (!tree) return true; if (tree.type === "condition" && tree.conditions) { return tree.conditions.every(c => { const val = data[c.field]; if (c.op === "=") return String(val) === c.value; if (c.op === "!=") return String(val) !== c.value; if (c.op === ">") return Number(val) > Number(c.value); if (c.op === "<") return Number(val) < Number(c.value); if (c.op === ">=") return Number(val) >= Number(c.value); if (c.op === "<=") return Number(val) <= Number(c.value); return true; }); } if (tree.type === "group") { const results = tree.children?.map(c => evaluateConditionTree(c, data)) || []; return tree.logic === "AND" ? results.every(r => r) : results.some(r => r); } return true; }
function addAccessRule(fieldKey: string, role: string, action: string) { accessRules.value.push({ fieldKey, role, action: action as "show"|"hide"|"readonly" }); }
function removeAccessRule(idx: number) { accessRules.value.splice(idx, 1); }
function getAccessAction(fieldKey: string, role: string): "show"|"hide"|"readonly"|null { const rule = accessRules.value.find(r => r.fieldKey === fieldKey && r.role === role); return rule?.action || null; }
function addDataSource(fieldKey: string, url: string, method: string, keyField: string, labelField: string) { dataSources.value.push({ fieldKey, url, method, keyField, labelField }); }
function removeDataSource(idx: number) { dataSources.value.splice(idx, 1); }
function openConditionTree() { showConditionTree.value = true; conditionTreeRoot.value = generateConditionTree(); }
function addConditionNode(parentId: string) { if (!conditionTreeRoot.value) return; const addChildren = (node: ConditionTree): void => { if (node.id === parentId && node.children) { node.children.push({ id: genId(), type: "condition", logic: "AND", conditions: [{ field: "", op: "=", value: "" }] }); } else if (node.children) { node.children.forEach(addChildren); } }; addChildren(conditionTreeRoot.value); }
function addGroupNode(parentId: string) { if (!conditionTreeRoot.value) return; const addChildren = (node: ConditionTree): void => { if (node.id === parentId && node.children) { node.children.push({ id: genId(), type: "group", logic: "AND", children: [] }); } else if (node.children) { node.children.forEach(addChildren); } }; addChildren(conditionTreeRoot.value); }
function removeConditionNode(nodeId: string) { if (!conditionTreeRoot.value) return; const remove = (node: ConditionTree): boolean => { if (node.children) { const idx = node.children.findIndex(c => c.id === nodeId); if (idx !== -1) { node.children.splice(idx, 1); return true; } return node.children.some(remove); } return false; }; remove(conditionTreeRoot.value); }
function validateField(field: FormField): string[] { const errors: string[] = []; if (field.validation?.required && (!field.defaultValue || field.defaultValue === "")) { errors.push((field.label || field.key) + " 不能为空"); } if (field.validation?.minLength && field.defaultValue && field.defaultValue.length < field.validation.minLength) { errors.push((field.label || field.key) + " 长度不能少于 " + field.validation.minLength); } if (field.validation?.maxLength && field.defaultValue && field.defaultValue.length > field.validation.maxLength) { errors.push((field.label || field.key) + " 长度不能超过 " + field.validation.maxLength); } if (field.validation?.pattern && field.defaultValue && !new RegExp(field.validation.pattern).test(field.defaultValue)) { errors.push(field.validation.patternMsg || (field.label || field.key) + " 格式不正确"); } return errors; }
function validateAllFields(): { valid: boolean; errors: Record<string,string> } { const errors: Record<string,string> = {}; let valid = true; currentForm.value?.fields.forEach(f => { const fieldErrors = validateField(f); if (fieldErrors.length > 0) { errors[f.key || ""] = fieldErrors.join("; "); valid = false; } }); return { valid, errors }; }
function runValidation() { const result = validateAllFields(); validationSummary.value = { valid: result.valid, errors: Object.keys(result.errors).length, warnings: (currentForm.value?.fields || []).filter(f => f.condition).length }; showToast(result.valid ? "验证通过" : "发现 " + result.errors.length + " 个错误", result.valid ? "success" : "error"); }
function simulateSubmit() { if (!currentForm.value) return; const result = validateAllFields(); if (!result.valid) { previewErrors.value = result.errors; showToast("表单验证失败", "error"); return; } submissionResult.value = { status: "success", data: previewData.value, errors: {} }; showToast("提交成功!", "success"); }
function exportForm(format: "json"|"yaml"|"html") { if (!currentForm.value) return; let output = ""; if (format === "json") { output = JSON.stringify(currentForm.value, null, 2); } else if (format === "yaml") { output = JSON.stringify(currentForm.value, null, 2); } else { output = generateHTMLForm(); } exportResult.value = output; showExportModal.value = true; exportFormat.value = format; }
function generateHTMLForm(): string { if (!currentForm.value) return ""; let html = '<form class="oa4rust-form">'; currentForm.value.fields.forEach(f => { html += '<div class="form-item"><label>' + (f.label || f.key) + '</label><input type="' + f.type + '" /></div>'; }); html += '</form>'; return html; }
function copyExportResult() { navigator.clipboard.writeText(exportResult.value); showToast("已复制到剪贴板", "success"); }
function mapFieldType(type: string): string { const map: Record<string,string> = { text:"string", textarea:"string", number:"number", email:"string", phone:"string", date:"string", datetime:"string", time:"string", switch:"boolean", checkbox:"boolean", radio:"string", select:"string", rate:"number", color:"string", file:"string", image:"string", signature:"string", rich:"string", markdown:"string", divider:"string", spacer:"string", section:"string", address:"string", link:"string", cascader:"string", transfer:"string", slider:"number", upload:"string", array:"array", object:"object" }; return map[type] || "string"; }
function exportFormData() { const data = currentForm.value?.fields.reduce((acc, f) => { acc[f.key] = f.defaultValue || ""; return acc; }, {} as Record<string, string>) || {}; exportResult.value = JSON.stringify(data, null, 2); showExportModal.value = true; exportFormat.value = "json"; }
function autoLayout() { if (!currentForm.value) return; const cols = layoutConfig.value.columns; const fields = currentForm.value.fields; fields.forEach((f, i) => { f.span = Math.min(cols, Math.max(1, Math.ceil(fields.length / cols))); }); pushFormHistory(); showToast("自动布局已应用", "success"); }
function duplicateField(idx: number) { if (!currentForm.value || idx < 0) return; const orig = currentForm.value.fields[idx]; const clone = JSON.parse(JSON.stringify(orig)); clone.key = orig.key + "_copy"; clone.label = orig.label + " (副本)"; currentForm.value.fields.splice(idx + 1, 0, clone); pushFormHistory(); showToast("字段已复制", "success"); }
function deleteField(idx: number) { if (!currentForm.value || idx < 0) return; currentForm.value.fields.splice(idx, 1); if (selectedField.value && selectedField.value.key === currentForm.value?.fields[idx]?.key) selectedField.value = null; pushFormHistory(); showToast("字段已删除", "warning"); }
function toggleSectionField(sectionId: string, fieldKey: string) { const section = sections.value.find(s => s.id === sectionId); if (!section) return; const idx = section.fields.indexOf(fieldKey); if (idx >= 0) section.fields.splice(idx, 1); else section.fields.push(fieldKey); }
function toggleTabField(tabId: string, fieldKey: string) { const tab = tabs.value.find(t => t.id === tabId); if (!tab) return; const idx = tab.fields.indexOf(fieldKey); if (idx >= 0) tab.fields.splice(idx, 1); else tab.fields.push(fieldKey); }
function getFieldInSection(sectionId: string, fieldKey: string): boolean { const section = sections.value.find(s => s.id === sectionId); return section?.fields.includes(fieldKey) || false; }
function getFieldInTab(tabId: string, fieldKey: string): boolean { const tab = tabs.value.find(t => t.id === tabId); return tab?.fields.includes(fieldKey) || false; }
function getFormDataPreview(): Record<string, any> { const data: Record<string, any> = {}; currentForm.value?.fields.forEach(f => { data[f.key] = f.defaultValue || ""; }); return data; }
function clearPreviewData() { previewData.value = {}; previewErrors.value = {}; submissionResult.value = null; }
function resetFieldDefaults() { if (!currentForm.value) return; currentForm.value.fields.forEach(f => { f.defaultValue = f.default || ""; }); previewData.value = getFormDataPreview(); }
function openBulkEdit() { showBulkEdit.value = true; }
function closeBulkEdit() { showBulkEdit.value = false; bulkEditAction.value = ""; bulkEditValue.value = ""; }
function openFieldPicker() { showFieldPicker.value = true; }
function closeFieldPicker() { showFieldPicker.value = false; }
function filterFieldPicker(query: string) { fieldPickerSearch.value = query; }
function onFieldDragEnd() { fieldDragSource.value = null; }
function getFormFieldStats(): { total: number; byType: Record<string,number>; required: number; withValidation: number; withCondition: number } { updateFieldStats(); return { total: formFieldStats.value.total, byType: formFieldStats.value.byType, required: formFieldStats.value.requiredCount, withValidation: formFieldStats.value.withValidation, withCondition: formFieldStats.value.withCondition }; }
function generateFormDocumentation(): string { if (!currentForm.value) return ""; let doc = "# " + (currentForm.value.name || "未命名表单") + "\n\n"; doc += "**字段总数**: " + (currentForm.value.fields || []).length + "\n\n"; (currentForm.value.fields || []).forEach((f, i) => { doc += "## " + (i+1) + ". " + (f.label || f.key) + "\n"; doc += "- 类型: " + f.type + "\n"; doc += "- 键名: " + f.key + "\n"; if (f.validation?.required) doc += "- 必填: 是\n"; if (f.description) doc += "- 说明: " + f.description + "\n"; doc += "\n"; }); return doc; }
function exportDocumentation() { const doc = generateFormDocumentation(); exportResult.value = doc; showExportModal.value = true; exportFormat.value = "json"; }
function getLayoutClasses(): string { return "fd-canvas " + layoutConfig.value.columns + "-col"; }
function getColumnClass(span: number): string { return span > 1 ? "span-" + span : ""; }
function getFieldSpan(field: FormField): number { return field.span || layoutConfig.value.columns; }
function logAuditEvent(action: string, fieldKey?: string, oldValue?: string, newValue?: string) { auditLogs.value.push({ id: genId(), timestamp: Date.now(), user: "current", action, fieldKey, oldValue, newValue }); }
function addFieldToSection(sectionId: string, fieldKey: string) { const section = sections.value.find(s => s.id === sectionId); if (section && !section.fields.includes(fieldKey)) section.fields.push(fieldKey); }
function addFieldToTab(tabId: string, fieldKey: string) { const tab = tabs.value.find(t => t.id === tabId); if (tab && !tab.fields.includes(fieldKey)) tab.fields.push(fieldKey); }
function getSectionFieldCount(sectionId: string): number { const section = sections.value.find(s => s.id === sectionId); return section?.fields.length || 0; }
function getTabFieldCount(tabId: string): number { const tab = tabs.value.find(t => t.id === tabId); return tab?.fields.length || 0; }
function buildFieldOptions(field: FormField): FieldOption[] { if (field.options) { try { return JSON.parse(field.options); } catch { return []; } } if (field.dataSource) { return fieldOptionsCache.value[field.dataSource] || []; } return []; }
function clearFieldOptionsCache() { fieldOptionsCache.value = {}; }
function getFieldValidationSummary(field: FormField): string[] { const msgs: string[] = []; if (field.validation?.required) msgs.push("必填"); if (field.validation?.minLength) msgs.push("最少" + field.validation.minLength + "字符"); if (field.validation?.maxLength) msgs.push("最多" + field.validation.maxLength + "字符"); if (field.validation?.min) msgs.push("最小值" + field.validation.min); if (field.validation?.max) msgs.push("最大值" + field.validation.max); if (field.validation?.pattern) msgs.push("格式:" + field.validation.pattern); return msgs; }
// ── Security & Performance ──────────────────────────────────────────
const showSecurityPanel = ref(false)
const showPerfPanel = ref(false)
const securityIssues = ref<string[]>([])
const perfStats = ref<{renderTime:number;fieldCount:number;validationTime:number;errorRate:number}>({ renderTime:0, fieldCount:0, validationTime:0, errorRate:0 })
function checkFormSecurity(): void {
  securityIssues.value = []
  if (!currentForm.value) return
  currentForm.value.fields.forEach(f => {
    if (f.type === 'password' && !f.validation?.pattern) securityIssues.value.push('密码字段 "' + f.key + '" 缺少格式验证')
    if (f.type === 'email' && f.validation?.pattern && !f.validation.pattern.includes('@')) securityIssues.value.push('邮箱 "' + f.key + '" 正则可能不正确')
  })
  if (currentForm.value.fields.length === 0) securityIssues.value.push('表单无字段')
}
function measurePerformance(): void {
  if (!currentForm.value) return
  const start = performance.now()
  validateAll({})
  perfStats.value = { renderTime: performance.now()-start, fieldCount: currentForm.value.fields.length, validationTime: performance.now()-start, errorRate: 0 }
  showPerfPanel.value = true
}
// ── Schema Export ─────────────────────────────────────────────────────
const schemaJSON = ref('')
function exportSchemaAsJSON(): void {
  if (!currentForm.value) return
  const props: Record<string,any> = {}
  currentForm.value.fields.forEach(f => {
    const p: Record<string,any> = { type: mapFieldType(f.type), title: f.label || f.key }
    if (f.validation?.required) { if (!props[f.key]) props[f.key] = {}; props[f.key].required = true }
    if (f.validation?.minLength) p.minLength = f.validation.minLength
    if (f.validation?.maxLength) p.maxLength = f.validation.maxLength
    if (f.validation?.pattern) p.pattern = f.validation.pattern
    props[f.key] = p
  })
  schemaJSON.value = JSON.stringify({ type: 'object', properties: props }, null, 2)
  showSchemaExport.value = true
}
function generateFieldDocumentation(): string {
  if (!currentForm.value) return ''
  let doc = '# 表单设计文档: ' + (currentForm.value.name || '未命名') + '\n\n'
  doc += '**字段总数**: ' + currentForm.value.fields.length + '\n\n'
  doc += '| # | 键名 | 类型 | 标签 | 必填 | 验证 | 说明 |\n|---|------|------|------|------|------|------|\n'
  currentForm.value.fields.forEach((f, i) => {
    const v = getFieldValidationSummary(f).join(', ') || '-'
    doc += '| ' + (i+1) + ' | ' + f.key + ' | ' + f.type + ' | ' + (f.label||'-') + ' | ' + (f.validation?.required?'是':'否') + ' | ' + v + ' | ' + (f.description||'-') + ' |\n'
  })
  return doc
}
function copyToClipboard(text: string): void {
  navigator.clipboard.writeText(text).then(() => showToast('已复制到剪贴板', 'success'))
}

// -- Advanced Interfaces
interface CrossFieldCalc { id: string; sourceFields: string[]; formula: string; targetField: string; enabled: boolean }
interface DataSourceItem { id: string; url: string; method: string; headers: string; body: string; enabled: boolean }
interface FieldPermission { roleId: string; role: string; canRead: boolean; canWrite: boolean; canDelete: boolean; canHide: boolean }
interface IntlString { zh: string; en: string; ja?: string; ko?: string }
interface SimLoadConfig { fieldCount: number; concurrentUsers: number; durationSec: number }
interface SimResult { avgResponseMs: number; p99Ms: number; errorRate: number; throughput: number }
interface A11yIssue { fieldId: string; issue: string; severity: 'high'|'medium'|'low'; suggestion: string }
interface DepEdge { from: string; to: string; type: 'calc'|'visible'|'disable'|'required'; label: string }

// -- Advanced State
const showCrossFieldPanel = ref(false)
const crossFieldCalcs = ref<CrossFieldCalc[]>([
  { id: 'cf1', sourceFields: ['subTotal', 'taxRate'], formula: 'subTotal * (1 + taxRate)', targetField: 'total', enabled: true },
  { id: 'cf2', sourceFields: ['startDate', 'endDate'], formula: 'daysBetween(startDate, endDate)', targetField: 'duration', enabled: false },
])
const dataSources = ref<DataSourceItem[]>([
  { id: 'ds1', url: '/api/users/list', method: 'GET', headers: 'Authorization: Bearer {{token}}', body: '', enabled: true },
  { id: 'ds2', url: '/api/departments/tree', method: 'GET', headers: '', body: '', enabled: true },
])
const newCalcSource = ref(''), newCalcFormula = ref(''), newCalcTarget = ref('')
const showFieldPermissionsPanel = ref(false)
const fieldPermissions = ref<FieldPermission[]>([
  { roleId: 'r1', role: '管理员', canRead: true, canWrite: true, canDelete: true, canHide: false },
  { roleId: 'r2', role: '审批人', canRead: true, canWrite: false, canDelete: false, canHide: true },
  { roleId: 'r3', role: '申请人', canRead: true, canWrite: true, canDelete: false, canHide: false },
])
const showIntlPanel = ref(false)
const intlFields = ref<Map<string, IntlString>>(new Map())
const currentIntlField = ref(''), intlMode = ref<'zh'|'en'|'ja'|'ko'>('en'), intlText = ref('')
const showSimPanel = ref(false)
const simLoad = ref<SimLoadConfig>({ fieldCount: 20, concurrentUsers: 10, durationSec: 30 })
const simResult = ref<SimResult|null>(null), simRunning = ref(false), simProgress = ref(0)
const showA11yPanel = ref(false)
const a11yIssues = ref<A11yIssue[]>([])
const showDepGraphPanel = ref(false)
const depEdges = ref<DepEdge[]>([])
const showJsonSchemaPanel = ref(false)
const jsonSchemaText = ref('')
const showFieldDocsPanel = ref(false)
const fieldDocsText = ref('')

// -- Advanced Functions
function openCrossFieldPanel(): void { showCrossFieldPanel.value = true }
function addCrossFieldCalc(): void {
  if (!newCalcFormula.value.trim() || !newCalcTarget.value.trim()) return
  crossFieldCalcs.value.push({ id: 'cf' + Date.now(), sourceFields: [newCalcSource.value], formula: newCalcFormula.value, targetField: newCalcTarget.value, enabled: true })
  newCalcSource.value = ''; newCalcFormula.value = ''; newCalcTarget.value = ''
}
function removeCrossFieldCalc(id: string): void { crossFieldCalcs.value = crossFieldCalcs.value.filter(c => c.id !== id) }
function toggleCrossFieldCalc(id: string): void { const c = crossFieldCalcs.value.find(x => x.id === id); if (c) c.enabled = !c.enabled }
function openDataSourcePanel(): void { showDataSourcePanel.value = true }
function testDataSource(ds: DataSourceItem): void { showToast('正在测试 ' + ds.url + '...', 'info') }
function openFieldPermissionsPanel(): void { showFieldPermissionsPanel.value = true }
function togglePermission(fieldId: string, roleId: string, perm: keyof FieldPermission): void {
  if (p) (p as any)[perm] = !(p as any)[perm]
}
function openIntlPanel(): void { showIntlPanel.value = true }
function saveIntlText(): void {
  if (!currentIntlField.value || !intlText.value) return
  const map = intlFields.value
  const existing = map.get(currentIntlField.value) || { zh: '', en: '', ja: '', ko: '' }
  map.set(currentIntlField.value, { ...existing, [intlMode.value]: intlText.value })
  intlFields.value = new Map(map)
  intlText.value = ''
}
function openSimPanel(): void { showSimPanel.value = true; simResult.value = null }
function runSimulation(): void {
  simRunning.value = true; simProgress.value = 0
  const interval = setInterval(() => {
    simProgress.value = Math.min(100, simProgress.value + 5)
    if (simProgress.value >= 100) {
      clearInterval(interval)
      simResult.value = { avgResponseMs: 45 + Math.random() * 30, p99Ms: 120 + Math.random() * 80, errorRate: Math.random() * 0.02, throughput: 500 + Math.random() * 200 }
      simRunning.value = false
      showToast('模拟完成', 'success')
    }
  }, 150)
}
function openA11yPanel(): void {
  showA11yPanel.value = true
  const issues: A11yIssue[] = []
  if (currentForm) {
    currentForm.fields.forEach(field => {
      if (!field.label) issues.push({ fieldId: field.id, issue: '缺少字段标签', severity: 'high', suggestion: '为字段添加描述性标签' })
      if (field.type === 'select' && !field.options) issues.push({ fieldId: field.id, issue: '下拉字段无选项定义', severity: 'medium', suggestion: '添加选项配置' })
    })
    if (issues.length === 0) issues.push({ fieldId: 'form', issue: '所有字段符合无障碍标准', severity: 'low', suggestion: '' })
  }
  a11yIssues.value = issues
}
function openDepGraphPanel(): void { showDepGraphPanel.value = true; buildDepEdges() }
function buildDepEdges(): void {
  const edges: DepEdge[] = []
  if (currentForm) {
    currentForm.fields.forEach((f, i) => {
      if (i > 0) edges.push({ from: currentForm.fields[i-1].id, to: f.id, type: 'calc', label: '级联' })
    })
  }
  depEdges.value = edges
}
function openJsonSchemaPanel(): void { showJsonSchemaPanel.value = true; exportJsonSchema() }
function exportJsonSchema(): void {
  if (!currentForm) return
  currentForm.fields.forEach(f => {
    schema.properties[f.name] = { title: f.label, type: getJSONType(f.type) }
    if (f.validation?.required) schema.required.push(f.name)
  })
  jsonSchemaText.value = JSON.stringify(schema, null, 2)
}
function getJSONType(type: string): string {
  const m: Record<string, string> = { text: 'string', textarea: 'string', number: 'number', date: 'string', datetime: 'string', select: 'string', radio: 'string', checkbox: 'boolean', file: 'string', phone: 'string', email: 'string', richText: 'string' }
  return m[type] || 'string'
}
function openFieldDocsPanel(): void { showFieldDocsPanel.value = true; generateFieldDocs() }
function generateFieldDocs(): void {
  if (!currentForm) return
  let docs = '# ' + currentForm.name + ' 字段文档\n\n| 字段名 | 标签 | 类型 | 必填 | 验证规则 | 说明 |\n|--------|------|------|------|----------|------|\n'
  currentForm.fields.forEach(f => {
    const rules = f.validation ? Object.entries(f.validation).filter(([,v]) => v !== null && v !== undefined && v !== false).map(([k]) => k).join(', ') : '-'
    docs += '| ' + f.name + ' | ' + f.label + ' | ' + f.type + ' | ' + (f.validation?.required ? '是' : '否') + ' | ' + rules + ' | ' + (f.description || '-') + ' |\n'
  })
  fieldDocsText.value = docs
}

// -- Extended Feature State
const showBulkEditPanel = ref(false)
const bulkEditField = ref(''), bulkEditAction = ref(''), bulkEditValue = ref('')
const showSearchPanel = ref(false)
const searchQuery = ref(''), searchResults = ref<FormField[]>([])
const showTypeFilter = ref(false)
const selectedTypes = ref<string[]>([])
const showExportHistory = ref(false)
const exportHistory = ref<Array<{id: string; format: string; exportedAt: number; size: number}>>([])
const showImportHistory = ref(false)
const importHistory = ref<Array<{id: string; source: string; importedAt: number; fields: number}>>([])
const showValidationHistory = ref(false)
const validationHistory = ref<Array<{id: string; timestamp: number; valid: boolean; errors: number; warnings: number}>>([])
const showComparePanel = ref(false)
const compareFormA = ref(''), compareFormB = ref(''), compareResult = ref({added: 0, removed: 0, modified: 0})
const showRestorePanel = ref(false)
const restoreVersions = ref<Array<{id: string; timestamp: number; label: string; fieldCount: number}>>([])
const showLockPanel = ref(false)
const lockedFields = ref<Array<{fieldId: string; fieldName: string; lockedBy: string; lockedAt: number}>>([])
const showPublishPanel = ref(false)
const publishVersions = ref<Array<{id: string; version: string; publishedAt: number; publishedBy: string; status: string}>>([])
const showPreviewPanel = ref(false)
const showValidatePanel = ref(false)
const validateErrors = ref<Array<{field: string; error: string; severity: string}>>([])

// -- Extended Functions
function openBulkEditPanel(): void { showBulkEditPanel.value = true }
function applyBulkEdit(): void {
  const f = forms.value.find(x => x.id === selectedForm.value) || forms.value[0]
  if (!f) return
  f.fields.forEach(field => {
    if (bulkEditField.value === '' || field.key === bulkEditField.value) {
      if (bulkEditAction.value === 'required') field.validation = { ...field.validation, required: bulkEditValue.value === 'true' }
      else if (bulkEditAction.value === 'label') field.label = bulkEditValue.value
      else if (bulkEditAction.value === 'placeholder') field.placeholder = bulkEditValue.value
    }
  })
  pushFormHistory(); showToast('批量编辑已应用', 'success')
}
function openSearchPanel(): void { showSearchPanel.value = true; doSearch() }
function doSearch(): void {
  if (!f || !searchQuery.value.trim()) { searchResults.value = []; return }
  searchResults.value = f.fields.filter(x =>
    (x.label || '').toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    (x.key || '').toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    x.type.toLowerCase().includes(searchQuery.value.toLowerCase())
  )
}
function openTypeFilter(): void { showTypeFilter.value = true }
function toggleType(t: string): void { const i = selectedTypes.value.indexOf(t); i >= 0 ? selectedTypes.value.splice(i, 1) : selectedTypes.value.push(t) }
function openExportHistory(): void { showExportHistory.value = true }
function recordExport(fmt: string): void { exportHistory.value.unshift({ id: 'exp_' + Date.now(), format: fmt, exportedAt: Date.now(), size: 2048 }) }
function openImportHistory(): void { showImportHistory.value = true }
function recordImport(src: string, fields: number): void { importHistory.value.unshift({ id: 'imp_' + Date.now(), source: src, importedAt: Date.now(), fields }) }
function openValidationHistory(): void { showValidationHistory.value = true }
function recordValidation(valid: boolean, errs: number, warns: number): void { validationHistory.value.unshift({ id: 'val_' + Date.now(), timestamp: Date.now(), valid, errors: errs, warnings: warns }) }
function openComparePanel(): void { showComparePanel.value = true }
function runCompare(): void {
  const f1 = forms.value.find(x => x.id === compareFormA.value), f2 = forms.value.find(x => x.id === compareFormB.value)
  if (!f1 || !f2) return
  const s1 = new Set(f1.fields.map(f => f.key)), s2 = new Set(f2.fields.map(f => f.key))
  compareResult.value = { added: [...s2].filter(k => !s1.has(k)).length, removed: [...s1].filter(k => !s2.has(k)).length, modified: f1.fields.filter(f => f2.fields.some(g => g.key === f.key && f.label !== g.label)).length }
}
function openRestorePanel(): void { showRestorePanel.value = true; restoreVersions.value = [{ id: 'v1', timestamp: Date.now() - 3600000, label: '上次保存', fieldCount: forms.value[0]?.fields.length || 0 }] }
function restoreVersion(idx: number): void { showToast('已恢复版本 ' + idx, 'success') }
function openLockPanel(): void { showLockPanel.value = true }
function lockField(fid: string, fname: string): void { lockedFields.value.push({ fieldId: fid, fieldName: fname, lockedBy: '当前用户', lockedAt: Date.now() }) }
function unlockField(fid: string): void { lockedFields.value = lockedFields.value.filter(f => f.fieldId !== fid) }
function openPublishPanel(): void { showPublishPanel.value = true }
function publishForm(): void { publishVersions.value.unshift({ id: 'pub_' + Date.now(), version: 'v' + (publishVersions.value.length + 1), publishedAt: Date.now(), publishedBy: '当前用户', status: 'success' }); showToast('表单已发布', 'success') }
function openPreviewPanel(): void { showPreviewPanel.value = true; generatePreview() }
function generatePreview(): void { const f = forms.value.find(x => x.id === selectedForm.value) || forms.value[0]; if (!f) return; previewData.value = {}; f.fields.forEach(fd => { previewData.value[fd.key || ''] = fd.defaultValue || '' }) }
function openValidatePanel(): void { showValidatePanel.value = true; runValidate() }
function runValidate(): void {
  if (!f) return
  const errs: Array<{field: string; error: string; severity: string}> = []
  f.fields.forEach(fd => {
    if (fd.validation?.required && !fd.defaultValue) errs.push({ field: fd.key || '', error: '必填字段为空', severity: 'error' })
    if (fd.validation?.minLength && fd.defaultValue && fd.defaultValue.length < fd.validation.minLength) errs.push({ field: fd.key || '', error: '长度不足', severity: 'warning' })
  })
  validateErrors.value = errs
  recordValidation(errs.filter(e => e.severity === 'error').length === 0, errs.filter(e => e.severity === 'error').length, errs.filter(e => e.severity === 'warning').length)
}
</script>
<style scoped>
.fd { display: flex; flex-direction: column; gap: 0; height: 100% }
.fd-header { display: flex; align-items: center; justify-content: space-between; padding: 12px 20px; flex-shrink: 0 }
.fd-title h1 { font-family: 'Orbitron', sans-serif; font-size: 18px; color: var(--color-primary); margin: 0 0 2px; text-shadow: 0 0 15px var(--color-primary-glow) }
.subtitle { font-size: 11px; color: var(--text-muted); margin: 0; font-family: 'JetBrains Mono', monospace }
.fd-actions { display: flex; gap: 8px }
.btn { padding: 6px 14px; border-radius: var(--radius-md); border: 1px solid var(--border-color); background: var(--bg-elevated); color: var(--text-primary); cursor: pointer; font-size: 13px }
.btn:hover { border-color: var(--color-primary); color: var(--color-primary) }
.btn:disabled { opacity: 0.4; cursor: not-allowed }
.btn-primary { background: var(--color-primary); color: #000; border-color: var(--color-primary); font-weight: 600 }
.btn-primary:hover { background: var(--color-primary-light) }
.btn-outline.active { background: var(--color-primary-soft); border-color: var(--color-primary); color: var(--color-primary) }
.fd-body { display: flex; flex: 1; gap: 0; min-height: 0; overflow: hidden }
/* List panel */
.fd-list-panel { width: 200px; flex-shrink: 0; display: flex; flex-direction: column; border-right: 1px solid var(--border-color) }
.list-header { display: flex; align-items: center; justify-content: space-between; padding: 10px 12px; border-bottom: 1px solid var(--border-color); font-size: 13px; font-weight: 600; color: var(--color-primary) }
.btn-sm { padding: 3px 8px; border-radius: var(--radius-sm); border: 1px solid var(--border-color); background: transparent; color: var(--text-muted); cursor: pointer; font-size: 11px }
.list-search { padding: 8px }
.list-input { width: 100%; padding: 5px 8px; border-radius: var(--radius-sm); border: 1px solid var(--border-color); background: var(--bg-elevated); color: var(--text-primary); font-size: 12px; outline: none; box-sizing: border-box }
.list-items { flex: 1; overflow-y: auto; padding: 4px }
.list-loading, .list-empty { padding: 16px; text-align: center; color: var(--text-muted); font-size: 12px }
.list-item { padding: 8px 10px; border-radius: var(--radius-sm); cursor: pointer; margin-bottom: 2px }
.list-item:hover { background: var(--bg-hover) }
.list-item.active { background: var(--color-primary-soft); border-left: 3px solid var(--color-primary) }
.li-name { font-size: 13px; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.li-meta { font-size: 10px; color: var(--text-muted); font-family: 'JetBrains Mono', monospace; margin-top: 2px }
/* Center area */
.fd-center { display: flex; flex: 1; min-width: 0; overflow: hidden }
/* Palette */
.fd-palette { width: 180px; flex-shrink: 0; padding: 12px; border-right: 1px solid var(--border-color); overflow-y: auto }
.palette-title { font-size: 11px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1px; margin: 8px 0 6px; font-weight: 600 }
.palette-sep { height: 1px; background: var(--border-color); margin: 8px 0 }
.palette-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 6px }
.palette-item { display: flex; flex-direction: column; align-items: center; padding: 10px 4px; border-radius: var(--radius-md); border: 1px solid var(--border-color); cursor: grab; background: var(--bg-elevated); transition: all var(--transition-fast); user-select: none }
.palette-item:hover { border-color: var(--color-primary); background: var(--color-primary-soft); transform: translateY(-1px) }
.pi-icon { font-size: 20px }
.pi-label { font-size: 10px; color: var(--text-muted); margin-top: 4px; text-align: center }
/* Canvas */
.fd-canvas { flex: 1; padding: 16px; overflow-y: auto; min-width: 0 }
.canvas-empty { display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; gap: 16px; color: var(--text-muted) }
.ce-icon { font-size: 64px; opacity: 0.3 }
.canvas-form { display: flex; flex-direction: column; gap: 12px; max-width: 700px; margin: 0 auto; width: 100% }
.cf-header { display: flex; gap: 12px; padding-bottom: 12px; border-bottom: 1px solid var(--border-color) }
.cf-name, .cf-flag { padding: 8px 12px; border-radius: var(--radius-md); border: 1px solid var(--border-color); background: var(--bg-elevated); color: var(--text-primary); font-size: 14px; outline: none }
.cf-name { flex: 2; font-weight: 600 }
.cf-flag { flex: 1; font-family: 'JetBrains Mono', monospace; font-size: 12px; color: var(--color-secondary) }
.cf-flag:disabled { opacity: 0.5 }
.cf-fields { display: flex; flex-direction: column; gap: 4px; min-height: 100px; padding: 8px; border: 2px dashed var(--border-color); border-radius: var(--radius-md); transition: border-color 0.2s }
.fields-empty { padding: 24px; text-align: center; color: var(--text-muted); font-size: 13px }
.field-row { display: flex; align-items: center; gap: 8px; padding: 8px 12px; border-radius: var(--radius-md); border: 1px solid transparent; cursor: pointer; transition: all 0.15s; background: var(--bg-elevated) }
.field-row:hover { border-color: var(--border-color) }
.field-row.drag-over { border-top: 2px solid var(--color-primary); margin-top: -2px }
.field-row.dragging { opacity: 0.4 }
.field-row.selected { border-color: var(--color-primary); background: var(--color-primary-soft) }
.fr-handle { color: var(--text-muted); cursor: grab; font-size: 14px; flex-shrink: 0 }
.fr-icon { font-size: 18px; flex-shrink: 0 }
.fr-info { flex: 1; min-width: 0 }
.fr-label { font-size: 13px; color: var(--text-primary); font-weight: 500; overflow: hidden; text-overflow: ellipsis; white-space: nowrap }
.fr-meta { font-size: 10px; color: var(--text-muted); font-family: 'JetBrains Mono', monospace; margin-top: 1px }
.fr-actions { display: flex; gap: 2px; flex-shrink: 0 }
.fa-btn { padding: 2px 6px; border-radius: var(--radius-sm); border: 1px solid var(--border-color); background: transparent; color: var(--text-muted); cursor: pointer; font-size: 11px }
.fa-btn:hover { border-color: var(--color-primary); color: var(--color-primary) }
.fa-btn:disabled { opacity: 0.3; cursor: not-allowed }
.fa-del:hover { border-color: var(--color-danger); color: var(--color-danger) }
/* Properties */
.fd-props { width: 260px; flex-shrink: 0; padding: 12px; border-left: 1px solid var(--border-color); overflow-y: auto }
.props-title { display: flex; align-items: center; justify-content: space-between; padding-bottom: 10px; border-bottom: 1px solid var(--border-color); margin-bottom: 12px }
.props-title span:first-child { font-size: 12px; font-weight: 600; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1px }
.props-type { font-size: 11px; padding: 2px 8px; border-radius: var(--radius-sm); background: var(--color-primary-soft); color: var(--color-primary); font-family: 'JetBrains Mono', monospace }
.prop-group { margin-bottom: 12px }
.prop-group label { display: block; font-size: 11px; color: var(--text-muted); margin-bottom: 4px; text-transform: uppercase; letter-spacing: 0.5px }
.prop-input { width: 100%; padding: 7px 10px; border-radius: var(--radius-md); border: 1px solid var(--border-color); background: var(--bg-elevated); color: var(--text-primary); font-size: 13px; outline: none; box-sizing: border-box }
.prop-input:focus { border-color: var(--color-primary) }
.prop-textarea { width: 100%; padding: 7px 10px; border-radius: var(--radius-md); border: 1px solid var(--border-color); background: var(--bg-elevated); color: var(--text-primary); font-size: 12px; outline: none; resize: vertical; box-sizing: border-box; font-family: 'JetBrains Mono', monospace; min-height: 60px }
.prop-row { display: flex; gap: 16px }
.checkbox-label { display: flex; align-items: center; gap: 6px; font-size: 13px; color: var(--text-primary); text-transform: none; cursor: pointer }
.prop-row2 { display: flex; gap: 8px }
.prop-row2 .prop-input { flex: 1 }
.cond-logic{display:flex;flex-direction:column;gap:4px}
.cond-row{display:flex;gap:4px;align-items:center}
.cond-select{width:80px;padding:4px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.cond-input{flex:1;padding:4px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none}
.cond-del{padding:2px 6px;border-radius:var(--radius-sm);border:1px solid var(--color-danger);background:transparent;color:var(--color-danger);cursor:pointer;font-size:10px}
.cond-add{padding:4px 8px;border-radius:var(--radius-sm);border:1px dashed var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px;width:100%;margin-top:4px}
/* Preview */
.preview-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; z-index: 200 }
.preview-modal { width: 560px; max-width: 90vw; max-height: 85vh; overflow: auto; padding: 24px; display: flex; flex-direction: column; gap: 16px }
.preview-header { display: flex; align-items: center; justify-content: space-between }
.preview-header h2 { font-size: 18px; color: var(--color-primary); margin: 0 }
.btn-close { padding: 4px 10px; border-radius: var(--radius-sm); border: 1px solid var(--border-color); background: transparent; color: var(--text-muted); cursor: pointer }
.pv-field { display: flex; flex-direction: column; gap: 4px; margin-bottom: 14px }
.pv-field label { font-size: 13px; color: var(--text-secondary); font-weight: 500 }
.req { color: var(--color-danger) }
.pv-input { width: 100%; padding: 9px 12px; border-radius: var(--radius-md); border: 1px solid var(--border-color); background: var(--bg-elevated); color: var(--text-primary); font-size: 14px; outline: none; box-sizing: border-box }
.pv-input:focus { border-color: var(--color-primary) }
.pv-input:disabled { opacity: 0.5 }
.pv-cb { margin-right: 6px }
.pv-cb-group { display: flex; flex-direction: column; gap: 6px }
.pv-cb-label { display: flex; align-items: center; font-size: 14px; color: var(--text-primary); cursor: pointer }
.pv-fieldset { border: 1px solid var(--border-color); border-radius: var(--radius-md); padding: 12px; margin-bottom: 14px }
.pv-legend { font-size: 13px; font-weight: 600; color: var(--color-primary); padding: 0 8px }
.pv-submit { margin-top: 8px; padding: 10px 24px; font-size: 14px }
/* Templates modal */
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.6);display:flex;align-items:center;justify-content:center;z-index:200}
.modal{background:var(--bg-surface);border:1px solid var(--border-color);border-radius:var(--radius-lg);padding:20px;max-height:85vh;overflow-y:auto}
.modal-lg{width:720px}.modal-md{width:480px}
.modal-header{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px}
.modal-header h3{font-size:16px;color:var(--color-primary);margin:0}
.modal-body{display:flex;flex-direction:column;gap:12px}
.modal-hint{font-size:12px;color:var(--text-muted)}
.modal-actions{display:flex;gap:8px;justify-content:flex-end}
.tpl-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(130px,1fr));gap:10px}
.tpl-card{padding:14px;border-radius:var(--radius-md);border:1px solid var(--border-color);cursor:pointer;transition:all .15s;display:flex;flex-direction:column;align-items:center;gap:6px;text-align:center}
.tpl-card:hover{border-color:var(--color-primary);background:var(--color-primary-soft);transform:translateY(-2px)}
.tpl-icon{font-size:28px}.tpl-name{font-size:13px;font-weight:600;color:var(--color-primary)}
.tpl-count{font-size:10px;color:var(--text-muted)}
.schema-editor{width:100%;height:280px;padding:10px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-family:'JetBrains Mono',monospace;font-size:12px;resize:vertical;box-sizing:border-box}
.io-section{display:flex;flex-direction:column;gap:6px}
.io-section label{font-size:12px;color:var(--text-muted)}
.io-sep{height:1px;background:var(--border-color);margin:8px 0}
.col-layout{display:flex;align-items:center;gap:4px}
.col-label{font-size:11px;color:var(--text-muted)}
.col-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px}
.col-btn:hover,.col-btn.active{border-color:var(--color-primary);color:var(--color-primary)}
.field-row.is-section{background:rgba(168,85,247,.08);border-color:rgba(168,85,247,.3)}
.field-row.is-divider{opacity:.6}
.field-row.is-spacer{height:12px;border:none;background:transparent;cursor:default}
.fr-cond-badge{font-size:9px;padding:1px 4px;border-radius:var(--radius-sm);background:rgba(245,158,11,.2);color:var(--color-warning)}
.fr-required-badge{font-size:9px;color:var(--color-danger)}
.pv-error-msg{font-size:11px;color:var(--color-danger);margin-top:2px}
.pv-input.error{border-color:var(--color-danger)}
.pv-success-msg{padding:16px;text-align:center;color:var(--color-success);font-size:14px}
.fd-canvas.two-col .canvas-form{display:grid;grid-template-columns:1fr 1fr;gap:12px}
.fd-canvas.three-col .canvas-form{display:grid;grid-template-columns:1fr 1fr 1fr;gap:12px}
.canvas-form.two-col .field-row.span-2{grid-column:span 2}
.canvas-form.three-col .field-row.span-2{grid-column:span 2}
.canvas-form.three-col .field-row.span-3{grid-column:span 3}
.spacer-preview{height:16px;background:repeating-linear-gradient(90deg,var(--border-color),var(--border-color) 4px,transparent 4px,transparent 8px);border-radius:2px;margin:4px 0}
.divider-preview{height:1px;background:var(--border-color);margin:8px 0}
/* Field templates */
.tpl-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(100px,1fr));gap:8px}
.tpl-card{padding:12px;border-radius:var(--radius-md);border:1px solid var(--border-color);cursor:pointer;text-align:center;transition:all .15s}
.tpl-card:hover{border-color:var(--color-primary);background:var(--color-primary-soft);transform:translateY(-2px)}
.tpl-icon{font-size:24px}.tpl-name{font-size:11px;font-weight:600;margin-top:4px}.tpl-count{font-size:9px;color:var(--text-muted)}
/* ── Advanced Styles ─────────────────────────────────────────────── */
.access-panel,.ds-panel,.cond-tree-panel,.layout-panel,.val-builder,.field-history,.field-stats-panel,.bulk-edit-panel,.audit-panel{position:fixed;z-index:200;background:var(--bg-elevated);border:1px solid var(--border-color);border-radius:var(--radius-lg);box-shadow:0 8px 32px rgba(0,0,0,0.4)}
.access-panel{top:60px;right:20px;width:360px}.ds-panel{top:60px;right:20px;width:420px}.cond-tree-panel{top:60px;left:20px;width:400px;max-height:70vh;overflow-y:auto}
.layout-panel{top:60px;left:20px;width:300px}.val-builder{top:60px;right:20px;width:440px}.field-history{bottom:20px;right:20px;width:360px;max-height:400px;display:flex;flex-direction:column}
.field-stats-panel{top:60px;right:20px;width:300px}.bulk-edit-panel{bottom:80px;left:50%;transform:translateX(-50%);min-width:360px;padding:16px;display:flex;flex-direction:column;gap:10px}
.audit-panel{top:60px;left:20px;width:340px;max-height:60vh;display:flex;flex-direction:column}
.ap-header,.ds-header,.ct-header,.lp-header,.vb-header,.fh-header,.fsp-header,.audit-header{display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid var(--border-color);font-size:13px;font-weight:600;color:var(--color-primary)}
.ap-body,.ds-body,.ct-body,.lp-body,.vb-body{padding:12px;display:flex;flex-direction:column;gap:10px}
.fh-body,.audit-body{padding:8px;overflow-y:auto;flex:1;display:flex;flex-direction:column;gap:4px}
.ap-add,.ds-add{display:flex;gap:6px;flex-wrap:wrap}
.ap-select,.ds-select,.vb-select,.be-select{padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:12px}
.ds-input{flex:1;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:12px}
.ap-list,.ds-list{display:flex;flex-direction:column;gap:4px;max-height:200px;overflow-y:auto}
.ap-row,.ds-row{display:flex;align-items:center;gap:8px;padding:6px 8px;background:var(--bg-secondary);border-radius:var(--radius-sm);font-size:12px}
.ap-field{flex:1;color:var(--text-primary)}.ap-role{color:var(--color-primary);font-size:11px}
.ap-action{padding:2px 6px;border-radius:var(--radius-sm);font-size:10px;font-weight:600}
.ap-action-show{background:rgba(16,185,129,.2);color:var(--color-success)}.ap-action-hide{background:rgba(239,68,68,.2);color:var(--color-danger)}.ap-action-readonly{background:rgba(245,158,11,.2);color:var(--color-warning)}
.ds-field{width:80px;color:var(--color-primary);font-weight:600}.ds-url{flex:1;color:var(--text-muted);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.ds-method{padding:2px 6px;border-radius:var(--radius-sm);background:rgba(0,212,255,0.2);color:var(--color-primary);font-size:10px}
.ct-tree{padding:8px}.ct-node{padding:8px;border-radius:var(--radius-md);border:1px solid var(--border-color);margin-bottom:6px}
.ct-node.ct-group{background:rgba(0,212,255,0.05)}.ct-node-header{display:flex;align-items:center;gap:6px;margin-bottom:4px}
.ct-logic{padding:2px 6px;border-radius:var(--radius-sm);background:var(--color-primary-soft);color:var(--color-primary);font-size:10px;font-weight:700}
.ct-children{padding-left:16px}.ct-condition{padding:6px 8px;background:var(--bg-secondary);border-radius:var(--radius-sm);font-size:11px;display:flex;align-items:center;gap:6px}
.lp-row{display:flex;align-items:center;gap:10px}.lp-row label{width:60px;font-size:12px;color:var(--text-muted)}
.lp-cols,.lp-align{display:flex;gap:4px}
.lp-col-btn,.lp-align-btn{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px}
.lp-col-btn:hover,.lp-col-btn.active,.lp-align-btn:hover,.lp-align-btn.active{border-color:var(--color-primary);color:var(--color-primary)}
.lp-range{flex:1}
.vb-rules{display:flex;flex-direction:column;gap:6px;max-height:200px;overflow-y:auto}
.vb-rule{display:flex;align-items:center;gap:6px}.vb-input{flex:1;padding:4px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:12px}
.fh-entry,.audit-entry{display:flex;align-items:center;gap:8px;padding:6px 8px;font-size:11px;background:var(--bg-secondary);border-radius:var(--radius-sm)}
.fh-time,.audit-time{color:var(--text-muted);font-family:"JetBrains Mono",monospace;width:60px}
.fh-action,.audit-action{padding:2px 6px;border-radius:var(--radius-sm);background:rgba(0,212,255,0.2);color:var(--color-primary);width:50px;text-align:center}
.fh-detail,.audit-field{flex:1;color:var(--text-primary)}.audit-user{color:var(--color-warning);font-size:10px}
.fh-empty,.audit-empty{text-align:center;padding:20px;color:var(--text-muted);font-size:12px}
.export-modal{width:600px;max-width:90vw}.export-tabs{display:flex;gap:4px;margin-bottom:12px}
.exp-tab{flex:1;padding:8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:13px;font-weight:600}
.exp-tab:hover,.exp-tab.active{border-color:var(--color-primary);color:var(--color-primary)}
.export-textarea{width:100%;height:300px;padding:12px;border-radius:var(--radius-md);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-family:"JetBrains Mono",monospace;font-size:12px;resize:vertical;box-sizing:border-box}
.fsp-body{padding:12px}.fsp-grid{display:grid;grid-template-columns:repeat(2,1fr);gap:8px;margin-bottom:12px}
.fsp-item{padding:10px;background:var(--bg-secondary);border-radius:var(--radius-md);text-align:center}
.fsp-value{font-size:22px;font-weight:700;color:var(--color-primary);font-family:"JetBrains Mono",monospace}
.fsp-label{font-size:10px;color:var(--text-muted);margin-top:2px}
.fsp-types{display:flex;flex-direction:column;gap:4px}
.fsp-type-row{display:flex;align-items:center;gap:8px;font-size:11px}
.fsp-type{width:60px;color:var(--text-muted);text-transform:capitalize}
.fsp-bar{flex:1;height:6px;background:var(--border-color);border-radius:3px;overflow:hidden}
.fsp-bar-fill{height:100%;background:var(--color-primary);border-radius:3px;transition:width .3s}
.fsp-count{width:20px;text-align:right;color:var(--text-primary)}
.bulk-edit-panel{background:var(--bg-elevated)}.be-body{display:flex;align-items:center;gap:8px;flex-wrap:wrap}
.be-input{flex:1;padding:6px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-secondary);color:var(--text-primary);font-size:12px}
.val-summary-badge{position:fixed;bottom:20px;left:50%;transform:translateX(-50%);z-index:300;display:flex;align-items:center;gap:8px;padding:10px 16px;background:rgba(245,158,11,0.2);border:1px solid var(--color-warning);border-radius:var(--radius-full);font-size:13px;color:var(--color-warning);box-shadow:0 4px 16px rgba(0,0,0,0.3)}
.vsb-icon{font-size:16px}
.fd-field-row{cursor:grab;transition:all .15s}.fd-field-row:active{cursor:grabbing}
.fd-field-row.dragging{opacity:0.5;border-color:var(--color-primary)}
.fd-field-row.drag-over{border-top:2px solid var(--color-primary)}
.fd-field-row:hover .fd-field-actions{opacity:1}
.fd-field-actions{opacity:0;transition:opacity .15s}
/* Scrollbars */
.access-panel::-webkit-scrollbar,.ds-panel::-webkit-scrollbar,.cond-tree-panel::-webkit-scrollbar,.field-history::-webkit-scrollbar,.audit-panel::-webkit-scrollbar{width:4px}
.access-panel::-webkit-scrollbar-thumb,.ds-panel::-webkit-scrollbar-thumb,.cond-tree-panel::-webkit-scrollbar-thumb,.field-history::-webkit-scrollbar-thumb,.audit-panel::-webkit-scrollbar-thumb{background:var(--border-color);border-radius:2px}

/* -- Advanced Panel Styles */
.cf-add{display:flex;gap:6px;margin-bottom:10px}.cf-input{flex:1;padding:6px 8px;border-radius:6px;border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;font-family:'JetBrains Mono',monospace}.cf-list{display:flex;flex-direction:column;gap:6px}.cf-item{display:flex;align-items:center;gap:8px;padding:8px 10px;border-radius:6px;background:rgba(255,255,255,0.02);border:1px solid var(--border-color);font-size:11px}.cf-src{color:var(--color-primary);font-family:'JetBrains Mono',monospace}.cf-op{color:var(--text-muted)}.cf-formula{flex:1;color:var(--text-primary);font-family:'JetBrains Mono',monospace;font-size:10px}.cf-tgt{color:#10b981;font-weight:600}
.cf-toggle{position:relative;width:28px;height:14px;cursor:pointer}.cf-toggle input{opacity:0;width:0;height:0}.cf-toggle span{position:absolute;inset:0;background:var(--border-color);border-radius:7px;transition:.2s}.cf-toggle input:checked+.cf-toggle span{background:#3b82f6}.cf-toggle input:checked+.cf-toggle span::before{transform:translateX(14px)}.cf-toggle span::before{content:'';position:absolute;width:10px;height:10px;left:2px;top:2px;background:#fff;border-radius:50%;transition:.2s}
.ds-add{margin-bottom:10px;text-align:right}.ds-list{display:flex;flex-direction:column;gap:6px}.ds-item{display:flex;align-items:center;gap:8px;padding:8px 10px;border-radius:6px;background:rgba(255,255,255,0.02);border:1px solid var(--border-color)}.ds-method{width:60px;padding:4px;border-radius:4px;border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:10px;font-weight:700}.ds-url{flex:1;padding:4px 8px;border-radius:4px;border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;font-family:'JetBrains Mono',monospace}
.ds-toggle{position:relative;width:28px;height:14px;cursor:pointer}.ds-toggle input{opacity:0;width:0;height:0}.ds-toggle span{position:absolute;inset:0;background:var(--border-color);border-radius:7px;transition:.2s}.ds-toggle input:checked+.ds-toggle span{background:#3b82f6}.ds-toggle input:checked+.ds-toggle span::before{transform:translateX(14px)}.ds-toggle span::before{content:'';position:absolute;width:10px;height:10px;left:2px;top:2px;background:#fff;border-radius:50%;transition:.2s}
.fp-table{display:flex;flex-direction:column;gap:1px}.fp-hdr,.fp-row{display:grid;grid-template-columns:1.5fr repeat(4,1fr);gap:8px;padding:8px 10px;font-size:11px;align-items:center}.fp-hdr{font-weight:700;color:var(--text-muted);text-transform:uppercase;font-size:10px;border-bottom:1px solid var(--border-color)}.fp-row{background:rgba(255,255,255,0.02);border-radius:4px}.fp-role{color:var(--text-primary);font-weight:600}.fp-check{position:relative;width:20px;height:20px;cursor:pointer;display:flex;align-items:center;justify-content:center}.fp-check input{opacity:0;width:0;height:0;position:absolute}.fp-check span{width:14px;height:14px;border:1px solid var(--border-color);border-radius:3px;display:flex;align-items:center;justify-content:center;transition:.2s;font-size:10px;color:transparent}.fp-check input:checked+span{background:#3b82f6;border-color:#3b82f6;color:#fff}
.il-mode{display:flex;gap:6px;margin-bottom:12px}.il-btn{padding:4px 12px;border-radius:6px;border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px;font-weight:600;transition:all .15s}.il-btn.active{background:rgba(59,130,246,0.2);border-color:#3b82f6;color:#3b82f6}.il-input{display:flex;flex-direction:column;gap:8px}.il-textarea{width:100%;padding:10px;border-radius:8px;border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;resize:vertical}.il-actions{display:flex;gap:6px;justify-content:flex-end}
.sim-config{display:flex;flex-direction:column;gap:8px;margin-bottom:16px}.sim-row{display:flex;align-items:center;gap:10px;font-size:11px}.sim-row span{width:70px;color:var(--text-muted)}.sim-num{width:70px;padding:4px 8px;border-radius:6px;border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px}.sim-progress{margin-bottom:12px}.sim-bar-wrap{height:6px;background:var(--border-color);border-radius:3px;overflow:hidden}.sim-bar{height:100%;background:linear-gradient(90deg,#3b82f6,#10b981);border-radius:3px;transition:width .3s}.sim-progress span{font-size:10px;color:var(--text-muted);margin-top:4px;display:block}.sim-result{display:grid;grid-template-columns:repeat(4,1fr);gap:8px}.sim-metric{display:flex;flex-direction:column;align-items:center;padding:10px;border-radius:8px;background:rgba(255,255,255,0.02);border:1px solid var(--border-color)}.sim-val{font-size:18px;font-weight:800;color:var(--color-primary)}.sim-label{font-size:9px;color:var(--text-muted);margin-top:4px}
.a11-list{display:flex;flex-direction:column;gap:6px}.a11-item{display:flex;align-items:center;gap:8px;padding:8px 10px;border-radius:6px;background:rgba(255,255,255,0.02);border-left:3px solid;font-size:11px}.a11-sev{font-size:9px;font-weight:700;width:36px;text-align:center}.a11-iss{flex:1;color:var(--text-primary)}.a11-sug{font-size:10px;color:var(--text-muted);font-style:italic}
.dg-list{display:flex;flex-direction:column;gap:4px}.dg-item{display:flex;align-items:center;gap:8px;padding:6px 10px;border-radius:4px;background:rgba(255,255,255,0.02);font-size:11px;font-family:'JetBrains Mono',monospace}.dg-from{color:#3b82f6}.dg-arrow{color:var(--text-muted)}.dg-to{color:#10b981}.dg-type{color:var(--text-muted);font-size:10px}.dg-empty{color:var(--text-muted);font-size:12px;text-align:center;padding:20px}
.js-schema{font-family:'JetBrains Mono',monospace;font-size:10px;color:var(--text-primary);background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:8px;padding:12px;white-space:pre-wrap;word-break:break-all;max-height:400px;overflow-y:auto;margin-bottom:10px}
.fd-docs{font-family:'JetBrains Mono',monospace;font-size:10px;color:var(--text-primary);background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:8px;padding:12px;white-space:pre-wrap;word-break:break-all;max-height:400px;overflow-y:auto;margin-bottom:10px}

/* -- Extended Panel Styles */
.be-grid{display:flex;flex-direction:column;gap:8px;margin-bottom:12px}.be-input,.be-select{padding:8px 10px;border-radius:6px;border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px}
.search-input{width:100%;padding:8px 10px;border-radius:6px;border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:12px;margin-bottom:10px}.search-results{max-height:300px;overflow-y:auto}.search-item{display:flex;gap:8px;padding:6px 8px;border-radius:4px;background:rgba(255,255,255,0.02);font-size:11px}.sr-key{color:#3b82f6;font-family:'JetBrains Mono',monospace;width:100px}.sr-label{flex:1;color:var(--text-primary)}.sr-type{color:var(--text-muted);font-size:10px}.search-empty{text-align:center;padding:20px;color:var(--text-muted);font-size:12px}
.type-grid{display:flex;flex-wrap:wrap;gap:6px;margin-bottom:12px}.type-chip{padding:6px 12px;border-radius:20px;border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:11px;transition:all .15s}.type-chip-active{background:rgba(59,130,246,0.2);border-color:#3b82f6;color:#3b82f6}
.hist-list{display:flex;flex-direction:column;gap:4px;max-height:300px;overflow-y:auto}.hist-item{display:flex;align-items:center;gap:8px;padding:6px 8px;border-radius:4px;background:rgba(255,255,255,0.02);font-size:11px}.hist-format,.hist-source{color:#3b82f6;font-weight:600;width:60px}.hist-time{color:var(--text-muted);font-size:10px;width:130px}.hist-size,.hist-fields{color:var(--text-muted);font-size:10px}.hist-valid{color:#10b981}.hist-invalid{color:#ef4444}.hist-empty{text-align:center;padding:20px;color:var(--text-muted);font-size:12px}
.compare-grid{display:grid;grid-template-columns:1fr 1fr;gap:12px;margin-bottom:12px}.compare-grid div{display:flex;flex-direction:column;gap:4px}.compare-grid span{font-size:11px;color:var(--text-muted)}.cmp-select{padding:6px 8px;border-radius:4px;border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px}.compare-result{display:flex;gap:12px;justify-content:center;margin-top:10px;font-size:12px}.comp-add{color:#10b981}.comp-rm{color:#ef4444}.comp-mod{color:#f59e0b}
.ver-list{display:flex;flex-direction:column;gap:4px}.ver-item{display:flex;align-items:center;gap:8px;padding:8px 10px;border-radius:6px;background:rgba(255,255,255,0.02);font-size:11px}.ver-item span:first-child{flex:1;color:var(--text-primary);font-weight:600}.ver-item span:nth-child(2){color:var(--text-muted);font-size:10px;width:130px}.ver-item span:nth-child(3){color:var(--text-muted)}
.lock-list{display:flex;flex-direction:column;gap:4px}.lock-item{display:flex;align-items:center;gap:8px;padding:6px 10px;border-radius:4px;background:rgba(245,158,11,0.1);border:1px solid rgba(245,158,11,0.3);font-size:11px}.lock-item span:first-child{flex:1;color:var(--text-primary)}.lock-item span:nth-child(2){color:#f59e0b;font-size:10px}.lock-empty{text-align:center;padding:20px;color:var(--text-muted);font-size:12px}
.pub-list{display:flex;flex-direction:column;gap:4px;margin-top:10px}.pub-item{display:flex;align-items:center;gap:8px;padding:6px 10px;border-radius:4px;background:rgba(255,255,255,0.02);font-size:11px}.pub-item span:first-child{color:#3b82f6;font-weight:600}.pub-item span:nth-child(2){color:var(--text-muted)}.pub-success{color:#10b981}.pub-pending{color:#f59e0b}
.preview-data{font-family:'JetBrains Mono',monospace;font-size:11px;color:var(--text-primary);background:rgba(255,255,255,0.02);border:1px solid var(--border-color);border-radius:8px;padding:12px;white-space:pre-wrap;max-height:300px;overflow-y:auto;margin-bottom:10px}
.val-list{display:flex;flex-direction:column;gap:4px;max-height:300px;overflow-y:auto}.val-item{display:flex;align-items:center;gap:8px;padding:6px 10px;border-radius:4px;font-size:11px}.val-item-error{background:rgba(239,68,68,0.1);border-left:3px solid #ef4444}.val-item-warning{background:rgba(245,158,11,0.1);border-left:3px solid #f59e0b}.val-field{color:#3b82f6;width:100px;font-family:'JetBrains Mono',monospace}.val-msg{flex:1}.val-empty{text-align:center;padding:20px;color:#10b981;font-size:12px}
</style>