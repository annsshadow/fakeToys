<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="mod-view">
    <div class="view-header glass-card">
      <h1>程序中心</h1>
      <p class="subtitle">/api/program_center/* — 319条路由</p>
    </div>
    <div class="content-panel glass-card">
      <div class="tabs">
        <button :class="{active:tab==='agent'}" @click="tab='agent'">Agent</button>
        <button :class="{active:tab==='application'}" @click="tab='application'">Application</button>
        <button :class="{active:tab==='script'}" @click="tab='script'">Script</button>
        <button :class="{active:tab==='dict'}" @click="tab='dict'">Dict</button>
        <button :class="{active:tab==='market'}" @click="tab='market'">Market</button>
        <button :class="{active:tab==='invoke'}" @click="switchTab('invoke')">Invoke</button>
        <button :class="{active:tab==='config'}" @click="switchTab('config')">Config</button>
        <button :class="{active:tab==='style'}" @click="switchTab('style')">Style</button>
      </div>
      <!-- Agent tab -->
      <div v-if="tab==='agent'" class="tab-content">
        <div class="toolbar">
          <input v-model="agentSearch" placeholder="搜索Agent..." class="search-input" />
          <button class="btn-primary" @click="loadAgents">刷新</button>
          <button class="btn-create" @click="showCreateAgent=true">+ 新建Agent</button>
        </div>
        <div v-if="loadingAgent" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="filteredAgents.length===0" class="empty"><div class="ei">🤖</div><p>暂无Agent</p></div>
        <div v-else class="item-table">
          <div class="table-header"><span class="col-name">名称</span><span class="col-flag">Flag</span><span class="col-status">状态</span><span class="col-actions">操作</span></div>
          <div v-for="a in filteredAgents" :key="a.id" class="table-row glass-card">
            <span class="col-name">{{ a.name || a.label || a.agentName || '未命名' }}</span>
            <span class="col-flag font-mono">{{ a.flag || a.id }}</span>
            <span class="col-status" :class="a.enabled!==false?'enabled':'disabled'">{{ a.enabled!==false?'启用':'禁用' }}</span>
            <span class="col-actions">
              <button class="btn-sm" @click="toggleAgent(a)">{{ a.enabled!==false ? '禁用' : '启用' }}</button>
              <button class="btn-sm" @click="executeAgent(a)">执行</button>
              <button class="btn-sm" @click="editAgent(a)">编辑</button>
              <button class="btn-sm" style="color:var(--color-error)" @click="deleteAgent(a)">删除</button>
            </span>
          </div>
        </div>
      </div>
      <!-- Application tab -->
      <div v-if="tab==='application'" class="tab-content">
        <div class="toolbar"><button class="btn-primary" @click="loadAllApplications">全部应用</button><button class="btn-primary" @click="loadCenterMeta">注册应用/版本/验证码</button><button class="btn-primary" @click="loadProgramDetails">明细抽样</button><button class="btn-primary" @click="loadErrorLogs">错误日志</button><button class="btn-primary" @click="loadPromptErrorFilters">提示错误筛选</button><button class="btn-primary" @click="loadPromptErrorPrev">提示错误(逆序筛选)</button><button class="btn-primary" @click="loadUnexpectedFilters">意外错误筛选</button><button class="btn-primary" @click="loadWarnFilters">警告筛选</button><button class="btn-primary" @click="loadScheduleHotpic">调度日志/热图</button><span v-if="appMetaText" class="app-meta">{{ appMetaText }}</span></div>
        <div v-if="loadingApp" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="applications.length===0" class="empty"><div class="ei">📱</div><p>暂无Application</p></div>
        <div v-else class="item-grid">
          <div v-for="app in applications" :key="app.id" class="item-card glass-card">
            <div class="ic">📱</div>
            <div class="ib">
              <div class="it">{{ app.name || app.appName || '未命名' }}</div>
              <div class="im">{{ app.desc || app.description || '' }}</div>
              <div class="meta">flag: {{ app.flag || app.id }}</div>
              <button class="btn-sm" style="color:var(--color-error);margin-top:4px" @click="deleteApp(app)">删除</button>
              <button class="btn-sm" style="margin-top:4px" @click="compareApp(app)">对比</button>
            </div>
          </div>
        </div>
      </div>
      <!-- Script tab -->
      <div v-if="tab==='script'" class="tab-content">
        <div v-if="loadingScript" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="scripts.length===0" class="empty"><div class="ei">⚡</div><p>暂无Script</p></div>
        <div v-else class="item-grid">
          <div v-for="s in scripts" :key="s.flag" class="item-card glass-card">
            <div class="ic">⚡</div>
            <div class="ib">
              <div class="it">{{ s.name || s.scriptName || '未命名' }}</div>
              <div class="im">flag: {{ s.flag || s.id }}</div>
              <button class="btn-sm" style="margin-top:4px" @click="openScriptEditor(s)">编辑代码</button>
              <button class="btn-sm" style="margin-top:4px" @click="loadVersions(s)">版本</button>
              <button class="btn-sm" style="margin-top:4px" @click="runScript(s)">执行</button>
              <button class="btn-sm" style="color:var(--color-error);margin-top:4px" @click="deleteScript(s)">删除</button>
            </div>
          </div>
        </div>
      </div>
      <!-- Dict tab -->
      <div v-if="tab==='dict'" class="tab-content">
        <div class="toolbar">
          <button class="btn-primary" @click="loadDict">刷新</button>
          <button class="btn-create" @click="showCreateDict=true">+ 新建字典</button>
        </div>
        <div v-if="loadingDict" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="dicts.length===0" class="empty"><div class="ei">📚</div><p>暂无字典</p></div>
        <div v-else class="item-grid">
          <div v-for="d in dicts" :key="d.flag" class="item-card glass-card">
            <div class="ic">📚</div>
            <div class="ib">
              <div class="it">{{ d.name || d.dictName || '未命名' }}</div>
              <div class="im">flag: {{ d.flag || d.id }}</div>
              <button class="btn-sm" style="margin-top:4px" :disabled="!d.flag" @click="openDictData(d)">数据</button>
              <button class="btn-sm" style="color:var(--color-error);margin-top:4px" @click="deleteDict(d)">删除</button>
            </div>
          </div>
        </div>
      </div>
      <!-- Market tab -->
      <div v-if="tab==='market'" class="tab-content">
        <div class="market-bar">
          <button class="chip" :class="{on:marketCat===''}" @click="filterCat('')">全部</button>
          <button v-for="c in marketCats" :key="c" class="chip" :class="{on:marketCat===c}" @click="filterCat(c)">{{ c }}</button>
          <button class="chip" @click="loadTopThree">🔥 热门 Top3</button>
        </div>
        <div v-if="loadingMarket" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="markets.length===0" class="empty"><div class="ei">🏪</div><p>暂无市场数据</p></div>
        <div v-else class="item-grid">
          <div v-for="m in markets" :key="m.id" class="item-card glass-card">
            <div class="ic">🏪</div>
            <div class="ib">
              <div class="it">{{ m.name || m.title || '未命名' }}</div>
              <div class="im">{{ m.desc || '' }}</div>
              <div v-if="m.id && installedVer[m.id]" class="im">已装版本: {{ installedVer[m.id] }}</div>
            </div>
            <div class="market-acts">
              <button class="btn-sm" @click="installMarket(m)">安装/更新</button>
              <button class="btn-sm" @click="checkVersion(m)">版本</button>
              <button class="btn-sm" @click="uninstallMarket(m)">卸载</button>
            </div>
          </div>
        </div>
      </div>
    </div>
      <!-- Invoke tab -->
      <div v-if="tab==='invoke'" class="tab-content">
        <div class="toolbar">
          <button class="btn-primary" @click="loadInvokes">刷新</button>
          <button class="btn-primary" @click="loadInvokeByCategory">按分类</button>
          <button class="btn-create" @click="createInvoke">+ 新建接口</button>
        </div>
        <div v-if="loadingInvoke" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="invokes.length===0" class="empty"><div class="ei">🔌</div><p>暂无接口</p></div>
        <div v-else class="item-table">
          <div class="table-header"><span class="col-name">名称</span><span class="col-flag">别名</span><span class="col-status">状态</span><span class="col-actions">操作</span></div>
          <div v-for="iv in invokes" :key="iv.id" class="table-row glass-card">
            <span class="col-name">{{ iv.name || '未命名' }}</span>
            <span class="col-flag font-mono">{{ iv.alias || iv.category || '—' }}</span>
            <span class="col-status" :class="iv.enable!==false?'enabled':'disabled'">{{ iv.enable!==false?'启用':'禁用' }}</span>
            <span class="col-actions">
              <button class="btn-sm" style="color:var(--color-error)" @click="deleteInvoke(iv)">删除</button>
            </span>
          </div>
        </div>
      </div>
      <!-- Config tab -->
      <div v-if="tab==='config'" class="tab-content">
        <div class="toolbar">
          <button class="btn-primary" @click="loadConfigs">全部配置</button>
          <button class="btn-primary" @click="loadConfigApps">应用配置</button>
          <button class="btn-primary" @click="loadConfigEntities">实体配置</button>
          <button class="btn-primary" @click="loadDataStructure">数据结构</button>
          <button class="btn-primary" @click="loadDsTables">表/字段/验证码</button>
          <button class="btn-primary" @click="loadDeployMeta">Token/部署资源/节点</button>
          <button class="btn-primary" @click="loadConfigDump">配置转储/三元管理</button>
          <button class="btn-primary" @click="loadJestModule">测试/版本/模块</button>
          <button class="btn-primary" @click="loadSchedule">调度/本地调度/报告</button>
          <button class="btn-primary" @click="loadOutputMeta">输出/模块分类/存储映射</button>
          <button class="btn-primary" @click="loadErrorLogStats">错误日志/当前节点</button>
          <button class="btn-primary" @click="loadWeixinMeta">微信菜单/校验元/输出结构</button>
          <button class="btn-primary" @click="loadProgramAlias">应用别名/当前样式/数据结构</button>
          <button class="btn-primary" @click="loadDesignerJest">设计器搜索/中心测试/脚本基准</button>
          <button class="btn-primary" @click="loadDeployDictDistribute">o2部署/市场模块/字典数据/分发源</button>
          <button class="btn-primary" @click="loadAppStyleImages">应用风格图元(5类)</button>
          <button class="btn-primary" @click="loadMarketLogs">市场安装日志/分页</button>
          <button class="btn-primary" @click="loadProgramExtraReads">代理/市场VIP/应用包/图表/收藏</button>
          <button class="btn-primary" @click="loadProgramDeepReads">深度读矩阵</button>
          <button class="btn-create" @click="saveConfig">+ 新建/更新</button>
        </div>
        <div class="toolbar pc-write-actions">
          <button class="btn-sm" @click="pcStyleErase('appTop')">清顶部图</button>
          <button class="btn-sm" @click="pcStyleErase('launchLogo')">清启动Logo</button>
          <button class="btn-sm" @click="pcStyleErase('loginAvatar')">清登录头像</button>
          <button class="btn-sm" @click="pcStyleErase('menuBlur')">清菜单模糊图</button>
          <button class="btn-sm" @click="pcStyleErase('menuFocus')">清菜单聚焦图</button>
          <button class="btn-sm" @click="pcStyleErase('processDefault')">清流程默认图</button>
          <button class="btn-sm" @click="pcStyleErase('setupAbout')">清关于Logo</button>
          <button class="btn-sm" @click="pcCollectSave">存采集</button>
          <button class="btn-sm" @click="pcCollectDelete">删采集</button>
          <button class="btn-sm" @click="pcInvokeSaveById">存接口(按ID)</button>
          <button class="btn-sm" @click="pcInvokeDeleteById">删接口(按ID)</button>
          <button class="btn-sm" @click="pcConfigSave('centerserver')">存中心服务配置</button>
          <button class="btn-sm" @click="pcConfigSave('person')">存人员配置</button>
          <button class="btn-sm" @click="pcTokenThreshold">设令牌阈值</button>
          <button class="btn-sm" @click="pcU3('warnlog')">建告警日志</button>
          <button class="btn-sm" @click="pcU3('dictData')">建字典数据</button>
          <button class="btn-sm" @click="pcU3('dictUpdate')">改字典</button>
          <button class="btn-sm" @click="pcU3('dictDelete')">删字典</button>
          <button class="btn-sm" @click="pcU3('agentUpdate')">改代理</button>
          <button class="btn-sm" @click="pcU3('agentDelete')">删代理</button>
          <button class="btn-sm" @click="pcU3('menuUpdate')">改公众号菜单</button>
          <button class="btn-sm" @click="pcU3('menuDelete')">删公众号菜单</button>
          <button class="btn-sm" @click="pcU3('dictPaging')">字典分页</button>
          <button class="btn-sm" @click="pcU4Write('configSave')">存配置</button>
          <button class="btn-sm" @click="pcU4Write('scriptCreate')">建脚本</button>
          <button class="btn-sm" @click="pcU4Write('dictDataPut')">改字典数据</button>
          <button class="btn-sm" @click="pcU4Write('dictDataDel')">清字典数据</button>
          <button class="btn-sm" @click="pcU4Write('inputCreate')">输入创建</button>
          <button class="btn-sm" @click="pcU4Write('outputList')">输出列表</button>
          <button class="btn-sm" @click="pcU4Write('designerSearch')">设计器检索</button>
          <button class="btn-sm" @click="pcU4Read">读配置/日志</button>
          <button class="btn-sm" @click="pcU5Read">错误日志游标</button>
          <button class="btn-sm" @click="pcU5Write('appDelete')">删应用</button>
          <button class="btn-sm" @click="pcU5Write('scriptPut')">改脚本</button>
          <button class="btn-sm" @click="pcU5Write('dictData')">存字典数据</button>
          <button class="btn-sm" @click="pcU5Write('cacheDispatch')">缓存调度</button>
          <button class="btn-sm" @click="pcU5Write('scheduleReport')">调度上报</button>
          <button class="btn-sm" @click="pcU5Write('moduleList')">模块列表</button>
          <button class="btn-sm" @click="pcU6('agentDisable')">禁用代理</button>
          <button class="btn-sm" @click="pcU6('agentEnable')">启用代理</button>
          <button class="btn-sm" @click="pcU6('agentSave')">保存代理</button>
          <button class="btn-sm" @click="pcU6('invokeGet')">读调用器</button>
          <button class="btn-sm" @click="pcU6('invokeUpdate')">更新调用器</button>
          <button class="btn-sm" @click="pcU6('invokeDelete')">删调用器</button>
          <button class="btn-sm" @click="pcU6('invokeExecGet')">调用器执行读取</button>
          <button class="btn-sm" @click="pcU6('moduleOutputFile')">模块输出文件</button>
          <button class="btn-sm" @click="pcU6('moduleWrite')">写模块</button>
          <button class="btn-sm" @click="pcU6('moduleRemoveStruct')">删模块结构</button>
          <button class="btn-sm" @click="pcU6('applicationSave')">保存应用</button>
          <button class="btn-sm" @click="pcU7('inputCompare')">输入比较</button>
          <button class="btn-sm" @click="pcU7('inputCover')">输入覆盖</button>
          <button class="btn-sm" @click="pcU7('inputCreate')">输入创建</button>
          <button class="btn-sm" @click="pcU7('inputPrepareCover')">预备覆盖</button>
          <button class="btn-sm" @click="pcU7('inputPrepareCreate')">预备创建</button>
          <button class="btn-sm" @click="pcU7('jestClear')">Jest清缓存</button>
          <button class="btn-sm" @click="pcU8">命令/部署读</button>
          <button class="btn-sm" @click="pcU9">调用器/市场/钉钉只读</button>
          <button class="btn-sm" @click="pcU10">集成同步/清缓存</button>
          <button class="btn-sm" @click="pcU11">企微/政务钉钉/打包读</button>
          <button class="btn-sm" @click="pcU12">样式/公众号/打包连接读</button>
          <button class="btn-sm" @click="pcU13">WeLink同步/异常日志上报</button>
          <div v-if="pcU4Text" class="app-meta">{{ pcU4Text }}</div>
        </div>
        <div v-if="deployDistText" class="app-meta">{{ deployDistText }}</div>
        <div v-if="progDeepText" class="app-meta">{{ progDeepText }}</div>
        <div v-if="appStyleText" class="app-meta">{{ appStyleText }}</div>
        <div v-if="marketLogText" class="app-meta">{{ marketLogText }}</div>
        <div v-if="progExtraText" class="app-meta">{{ progExtraText }}</div>
        <div v-if="dsText" class="app-meta">{{ dsText }}</div>
        <div v-if="loadingConfig" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="configs.length===0" class="empty"><div class="ei">⚙️</div><p>暂无配置项</p></div>
        <div v-else class="item-table">
          <div class="table-header"><span class="col-name">Key</span><span class="col-flag">Value</span><span class="col-status">分类</span><span class="col-actions">—</span></div>
          <div v-for="cf in configs" :key="cf.id || cf.key" class="table-row glass-card">
            <span class="col-name">{{ cf.key || '—' }}</span>
            <span class="col-flag font-mono">{{ cf.value || '—' }}</span>
            <span class="col-status">{{ cf.category || '—' }}</span>
            <span class="col-actions"></span>
          </div>
        </div>
      </div>
      <!-- Style tab -->
      <div v-if="tab==='style'" class="tab-content">
        <div class="toolbar">
          <button class="btn-primary" @click="loadCurrentStyle">当前样式</button>
          <button class="btn-primary" @click="loadPortalApps">门户应用</button>
        </div>
        <div v-if="loadingStyle" class="loading-row"><div class="sk" v-for="i in 4" :key="i"></div></div>
        <div v-else-if="styleApps.length===0" class="empty"><div class="ei">🎨</div><p>暂无样式/应用</p></div>
        <div v-else class="item-table">
          <div class="table-header"><span class="col-name">应用</span><span class="col-flag">App ID</span><span class="col-status">状态</span><span class="col-actions">—</span></div>
          <div v-for="ap in styleApps" :key="ap.id || ap.appId" class="table-row glass-card">
            <span class="col-name">{{ ap.name || '—' }}</span>
            <span class="col-flag font-mono">{{ ap.appId || ap.id || '—' }}</span>
            <span class="col-status" :class="ap.disable?'disabled':'enabled'">{{ ap.disable?'停用':'启用' }}</span>
            <span class="col-actions"></span>
          </div>
        </div>
      </div>
    <!-- Create agent modal -->
    <div v-if="showCreateAgent" class="modal-overlay" @click.self="showCreateAgent=false">
      <div class="modal glass-card">
        <h3>新建Agent</h3>
        <div class="form-group"><label>名称</label><input v-model="agentForm.name" class="form-input" placeholder="Agent名称"/></div>
        <div class="form-group"><label>Flag</label><input v-model="agentForm.flag" class="form-input" placeholder="唯一标识"/></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showCreateAgent=false">取消</button>
          <button class="btn-primary" @click="onCreateAgent">创建</button>
        </div>
      </div>
    </div>
    <!-- Agent 属性编辑（POST agent/save/{id}） -->
    <div v-if="showEditAgent" class="modal-overlay" @click.self="showEditAgent=false">
      <div class="modal glass-card">
        <h3>编辑Agent</h3>
        <div class="form-group"><label>名称</label><input v-model="agentEdit.name" class="form-input"/></div>
        <div class="form-group"><label>Flag</label><input v-model="agentEdit.flag" class="form-input"/></div>
        <div class="form-group"><label>描述</label><input v-model="agentEdit.description" class="form-input"/></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showEditAgent=false">取消</button>
          <button class="btn-primary" :disabled="agentSaving" @click="saveAgentEdit">{{ agentSaving?'保存中…':'保存' }}</button>
        </div>
      </div>
    </div>
    <!-- 新建字典（POST /api/program_center/dict，dictFlag 为后端字段名） -->
    <div v-if="showCreateDict" class="modal-overlay" @click.self="showCreateDict=false">
      <div class="modal glass-card">
        <h3>新建字典</h3>
        <div class="form-group"><label>名称</label><input v-model="dictForm.name" class="form-input" placeholder="字典名称"/></div>
        <div class="form-group"><label>Flag</label><input v-model="dictForm.dictFlag" class="form-input" placeholder="唯一标识（dictFlag）"/></div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="showCreateDict=false">取消</button>
          <button class="btn-primary" :disabled="!dictForm.name?.trim()||!dictForm.dictFlag?.trim()" @click="onCreateDict">创建</button>
        </div>
      </div>
    </div>
    <!-- 字典数据编辑器（GET dict/{flag}/data + POST dict/{flag}/data/data） -->
    <div v-if="showDictData" class="modal-overlay" @click.self="closeDictData">
      <div class="modal glass-card">
        <h3>字典数据 · {{ dictDataFlag }}</h3>
        <div v-if="dictDataLoading" class="hint">加载中…</div>
        <div v-else>
          <div class="form-group"><label>数据（JSON）</label><textarea v-model="dictDataText" rows="10" class="form-input mono" style="width:100%;resize:vertical"/></div>
          <div v-if="dictDataError" class="error" style="color:var(--color-error);font-size:12px;margin-top:6px">{{ dictDataError }}</div>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeDictData">关闭</button>
          <button class="btn-primary" :disabled="dictDataLoading||!dictDataText.trim()" @click="saveDictData">保存数据</button>
        </div>
      </div>
    </div>
    <!-- 脚本代码编辑器（GET script/{flag} + POST script/{flag}） -->
    <div v-if="showScriptEdit" class="modal-overlay" @click.self="closeScriptEdit">
      <div class="modal glass-card" style="width:640px">
        <h3>脚本代码 · {{ scriptEdit.name || scriptEdit.flag }}</h3>
        <div v-if="scriptEditLoading" class="hint">加载中…</div>
        <div v-else class="form-group">
          <textarea v-model="scriptEdit.content" rows="16" class="form-input mono" style="width:100%;resize:vertical;font-size:12px"></textarea>
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeScriptEdit">关闭</button>
          <button class="btn-primary" :disabled="scriptEditLoading||!scriptEdit.flag" @click="saveScript">保存脚本</button>
        </div>
      </div>
    </div>
    <!-- 脚本版本历史（GET /api/scriptversion/list/script/{scriptId}） -->
    <div v-if="showVersions" class="modal-overlay" @click.self="closeVersions">
      <div class="modal glass-card">
        <h3>版本历史 · {{ versionsScript.name || versionsScript.flag }}</h3>
        <div v-if="versions.length===0" class="hint">暂无版本记录</div>
        <table v-else class="ver-table">
          <thead><tr><th>ID</th><th>创建时间</th></tr></thead>
          <tbody>
            <tr v-for="v in versions" :key="v.id">
              <td class="mono">{{ v.id }}</td>
              <td>{{ v.createTime || v.create_time || '—' }}</td>
            </tr>
          </tbody>
        </table>
        <div class="modal-actions"><button class="btn-cancel" @click="closeVersions">关闭</button></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useMutation } from '@tanstack/vue-query'
import { computed, ref, watch } from 'vue'
import { confirmMsg, toast } from '../utils/toast'

type Tab = 'agent' | 'application' | 'script' | 'dict' | 'market' | 'invoke' | 'config' | 'style'
type Agent = { id?: string; name?: string; label?: string; agentName?: string; flag?: string; enabled?: boolean }
type App = { id?: string; name?: string; appName?: string; desc?: string; description?: string; flag?: string }
type Script = { id?: string; name?: string; scriptName?: string; flag?: string }
type Dict = { id?: string; name?: string; dictName?: string; flag?: string; keyName?: string }
type Market = { id?: string; name?: string; title?: string; desc?: string }
type Invoke = { id?: string; name?: string; alias?: string; category?: string; enable?: boolean }
type Config = { id?: string; key?: string; value?: string; category?: string }
type StyleApp = { id?: string; name?: string; appId?: string; disable?: boolean }

const tab = ref<Tab>('agent')
const loadingAgent = ref(false)
const loadingApp = ref(false)
const loadingScript = ref(false)
const loadingDict = ref(false)
const loadingMarket = ref(false)
const agents = ref<Agent[]>([])
const applications = ref<App[]>([])
const scripts = ref<Script[]>([])
const dicts = ref<Dict[]>([])
const markets = ref<Market[]>([])
const invokes = ref<Invoke[]>([])
const loadingInvoke = ref(false)
const configs = ref<Config[]>([])
const loadingConfig = ref(false)
const styleApps = ref<StyleApp[]>([])
const loadingStyle = ref(false)
const showCreateAgent = ref(false)
const showCreateDict = ref(false)
const agentForm = ref({ name: '', flag: '' })
const agentSearch = ref('')

// Agent 属性编辑（POST agent/save/{id}）
const showEditAgent = ref(false)
const agentEdit = ref({ id: '', name: '', flag: '', description: '' })
const agentSaving = ref(false)

// 字典创建（POST /api/program_center/dict；后端字段名为 dictFlag）
const dictForm = ref({ name: '', dictFlag: '' })

// 字典数据编辑器（GET dict/{flag}/data + POST dict/{flag}/data/data）
const showDictData = ref(false)
const dictDataFlag = ref('')
const dictDataText = ref('')
const dictDataLoading = ref(false)
const dictDataSaving = ref(false)
const dictDataError = ref('')

// 脚本代码编辑器（GET script/{flag} + POST script/{flag}）
const showScriptEdit = ref(false)
const scriptEdit = ref({ flag: '', name: '', content: '' })
const scriptEditLoading = ref(false)
const scriptEditSaving = ref(false)

// 脚本版本历史（GET /api/scriptversion/list/script/{scriptId}）
const showVersions = ref(false)
const versionsScript = ref<Script>({})
const versions = ref<Array<{ id: string; createTime?: string; create_time?: string }>>([])
const filteredAgents = computed(() =>
  agentSearch.value
    ? agents.value.filter((a) => (a.name || a.flag || '').toLowerCase().includes(agentSearch.value.toLowerCase()))
    : agents.value,
)

async function loadAgents() {
  loadingAgent.value = true
  try {
    // 后端列表端点为裸 /agent（无 /agent/list）。
    const r = await api.get('/api/program_center/agent')
    agents.value = r.data ?? []
  } catch {
    agents.value = []
  } finally {
    loadingAgent.value = false
  }
}
const appMetaText = ref('')
// 消费 application/{id} 详情 + agent/{flag} 详情 + config/token 三条真实 distinct 路由
async function loadProgramDetails() {
  try {
    const [appList, agentList] = await Promise.all([
      api.get('/api/program_center/application/list').catch(() => null),
      api.get('/api/program_center/agent').catch(() => null),
    ])
    const apps = (Array.isArray((appList as any)?.data) ? (appList as any).data : []) as Array<Record<string, unknown>>
    const agents = (Array.isArray((agentList as any)?.data) ? (agentList as any).data : []) as Array<Record<string, unknown>>
    const appId = apps[0] ? String(apps[0].id ?? '') : ''
    const agentFlag = agents[0] ? String(agents[0].flag ?? agents[0].id ?? '') : ''
    const [appDetail, agentDetail, token] = await Promise.all([
      appId ? api.get(`/api/program_center/application/${encodeURIComponent(appId)}`).catch(() => null) : Promise.resolve(null),
      agentFlag ? api.get(`/api/program_center/agent/${encodeURIComponent(agentFlag)}`).catch(() => null) : Promise.resolve(null),
      api.get('/api/program_center/config/token').catch(() => null),
    ])
    const appName = (appDetail as any)?.data?.name ?? (appId || '—')
    const agentName = (agentDetail as any)?.data?.name ?? (agentFlag || '—')
    const hasToken = (token as any)?.data ? '有' : '无'
    appMetaText.value = `应用「${appName}」· 代理「${agentName}」· token配置 ${hasToken}`
  } catch (e: any) {
    toast.error('加载明细失败: ' + (e?.message ?? ''))
  }
}
// 错误日志族 3 条真实 distinct 路由（各读独立表游标）：警告日志 warnlog/list/{id}/next/{count}
// + 提示错误 prompterrorlog/list/{id}/next/{count}（x_program_prompt_error_log）+ 意外错误 unexpectederrorlog/list/{id}/next/{count}（x_program_unexpected_error_log）；flag=0 从头
async function loadErrorLogs() {
  const headFlag = '0'
  const cnt = '20'
  try {
    const [warn, prompt, unexpected] = await Promise.all([
      api.get(`/api/program_center/warnlog/list/${headFlag}/next/${cnt}`).catch(() => null),
      api.get(`/api/program_center/prompterrorlog/list/${headFlag}/next/${cnt}`).catch(() => null),
      api.get(`/api/program_center/unexpectederrorlog/list/${headFlag}/next/${cnt}`).catch(() => null),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    appMetaText.value = `警告 ${n(warn)} · 提示错误 ${n(prompt)} · 意外错误 ${n(unexpected)}`
  } catch (e: any) {
    toast.error('加载错误日志失败: ' + (e?.message ?? ''))
  }
}
// 提示错误日志筛选族 3 条真实 distinct（rev190，x_program_prompt_error_log WHERE id>$1 AND 各筛选维度）：
// list/{id}/next/{count}/date/{date}（DATE(create_time)=$2）+ /exceptionclass/{exceptionClass}（exception_class=$2）
// + /loggername/{loggerName}（logger_name=$2）。flag=0 从头，筛选值用模板变量。
async function loadPromptErrorFilters() {
  const headFlag = '0'
  const cnt = '20'
  const today = new Date().toISOString().slice(0, 10)
  const excls = 'java.lang.Exception'
  const logger = 'root'
  try {
    const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [byDate, byExcls, byLogger] = await Promise.all([
      settle(api.get(`/api/program_center/prompterrorlog/list/${headFlag}/next/${cnt}/date/${encodeURIComponent(today)}`)),
      settle(api.get(`/api/program_center/prompterrorlog/list/${headFlag}/next/${cnt}/exceptionclass/${encodeURIComponent(excls)}`)),
      settle(api.get(`/api/program_center/prompterrorlog/list/${headFlag}/next/${cnt}/loggername/${encodeURIComponent(logger)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    appMetaText.value = `提示错误：按日期 ${n(byDate)} · 按异常类 ${n(byExcls)} · 按日志器 ${n(byLogger)}`
  } catch (e: any) {
    toast.error('加载提示错误筛选失败: ' + (e?.message ?? ''))
  }
}
// 提示错误日志逆序筛选族 4 条真实 distinct（rev191，x_program_prompt_error_log WHERE id<$1 DESC AND 维度）：
// prev/{count}（纯逆序）+ prev/{count}/date/{date} + prev/{count}/exceptionclass/{exceptionClass} + prev/{count}/loggername/{loggerName}。
async function loadPromptErrorPrev() {
  const headFlag = '999999999'
  const cnt = '20'
  const today = new Date().toISOString().slice(0, 10)
  const excls = 'java.lang.Exception'
  const logger = 'root'
  try {
    const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [prev, byDate, byExcls, byLogger] = await Promise.all([
      settle(api.get(`/api/program_center/prompterrorlog/list/${headFlag}/prev/${cnt}`)),
      settle(api.get(`/api/program_center/prompterrorlog/list/${headFlag}/prev/${cnt}/date/${encodeURIComponent(today)}`)),
      settle(api.get(`/api/program_center/prompterrorlog/list/${headFlag}/prev/${cnt}/exceptionclass/${encodeURIComponent(excls)}`)),
      settle(api.get(`/api/program_center/prompterrorlog/list/${headFlag}/prev/${cnt}/loggername/${encodeURIComponent(logger)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    appMetaText.value = `提示错误(逆序)：全部 ${n(prev)} · 按日期 ${n(byDate)} · 按异常类 ${n(byExcls)} · 按日志器 ${n(byLogger)}`
  } catch (e: any) {
    toast.error('加载提示错误逆序筛选失败: ' + (e?.message ?? ''))
  }
}
// 意外错误日志筛选族 3 条真实 distinct（rev192，x_program_unexpected_error_log）：next/{count}/date/{date}（id>$1）
// + prev/{count}（id<$1 逆序）+ prev/{count}/date/{date}（id<$1 AND date）。
async function loadUnexpectedFilters() {
  const headFlag = '0'
  const prevFlag = '999999999'
  const cnt = '20'
  const today = new Date().toISOString().slice(0, 10)
  try {
    const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [nextDate, prev, prevDate] = await Promise.all([
      settle(api.get(`/api/program_center/unexpectederrorlog/list/${headFlag}/next/${cnt}/date/${encodeURIComponent(today)}`)),
      settle(api.get(`/api/program_center/unexpectederrorlog/list/${prevFlag}/prev/${cnt}`)),
      settle(api.get(`/api/program_center/unexpectederrorlog/list/${prevFlag}/prev/${cnt}/date/${encodeURIComponent(today)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    appMetaText.value = `意外错误：按日期(正序) ${n(nextDate)} · 逆序 ${n(prev)} · 逆序按日期 ${n(prevDate)}`
  } catch (e: any) {
    toast.error('加载意外错误筛选失败: ' + (e?.message ?? ''))
  }
}
// 警告日志筛选族 3 条真实 distinct（rev193，warnlog_list 拼 WHERE）：next/{count}/date/{date} + prev/{count} + prev/{count}/date/{date}。
async function loadWarnFilters() {
  const headFlag = '0'
  const prevFlag = '999999999'
  const cnt = '20'
  const today = new Date().toISOString().slice(0, 10)
  try {
    const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
    const [nextDate, prev, prevDate] = await Promise.all([
      settle(api.get(`/api/program_center/warnlog/list/${headFlag}/next/${cnt}/date/${encodeURIComponent(today)}`)),
      settle(api.get(`/api/program_center/warnlog/list/${prevFlag}/prev/${cnt}`)),
      settle(api.get(`/api/program_center/warnlog/list/${prevFlag}/prev/${cnt}/date/${encodeURIComponent(today)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    appMetaText.value = `警告：按日期(正序) ${n(nextDate)} · 逆序 ${n(prev)} · 逆序按日期 ${n(prevDate)}`
  } catch (e: any) {
    toast.error('加载警告筛选失败: ' + (e?.message ?? ''))
  }
}
// rev257：调度日志/存储映射配置 + 热图实体列表/存在校验 4 条真实 distinct 读路由
// x_program_schedule_log(application) · x_program_config(category=storageMapping) · hotpic ORM(application+info_id 列表/COUNT 存在)；arity 已核
async function loadScheduleHotpic() {
  const id = '0'
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [schedLog, warnCfg, hpList, hpExists, hpCipher, hpPaging] = await Promise.all([
      settle(api.get(`/api/program_center/schedule/list/schedulelog/application/${encodeURIComponent(id)}`)),
      settle(api.get(`/api/program_center/warnlog/view/system/log/tag/${encodeURIComponent(id)}`)),
      settle(api.get(`/api/hotpic/core/entity/list/by/${encodeURIComponent(id)}/${encodeURIComponent(id)}`)),
      settle(api.get(`/api/hotpic/core/entity/exists/check/${encodeURIComponent(id)}/${encodeURIComponent(id)}`)),
      settle(api.get(`/api/hotpic/assemble/control/cipher/hotpic/${encodeURIComponent(id)}`)),
      settle(api.get(`/api/hotpic/assemble/control/cipher/hotpic/filter/list/page/1/count/20`)),
      // rev300：hotpic 用户热图/取热图 详情(x_hotpic id) 补齐
      settle(api.get(`/api/hotpic/user/hotpic/${encodeURIComponent(id)}`)),
      settle(api.get(`/api/hotpic/get/hotpic/${encodeURIComponent(id)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    appMetaText.value = `调度日志 ${n(schedLog)} · 存储映射 ${n(warnCfg)} · 热图实体 ${n(hpList)} · 热图存在 ${(hpExists as any)?.data ? '有' : '无'} · 热图密文 ${(hpCipher as any)?.data ? '有' : '无'} · 热图分页 ${n(hpPaging)}`
  } catch (e: any) {
    toast.error('加载调度/热图失败: ' + (e?.message ?? ''))
  }
}
async function loadAllApplications() {
  try {
    // GET program_center/applications + center/applications —— 全部应用/中心应用
    const [apps, center] = await Promise.all([
      api.get('/api/program_center/applications'),      api.get('/api/program_center/center/applications'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    appMetaText.value = `全部应用 ${n(apps)} / 中心应用 ${n(center)}`
  } catch (e: any) {
    toast.error('加载应用清单失败: ' + (e?.message ?? ''))
  }
}
async function loadCenterMeta() {
  try {
    // 消费 program_center 三条真实路由：注册应用 / 中心版本 / 验证码脚本清单
    const [regist, version, codes] = await Promise.all([
      api.get('/api/program_center/center/regist/applications'),
      api.get('/api/program_center/center/version'),
      api.get('/api/program_center/code/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    const ver = (version as any)?.data?.version ?? '—'
    appMetaText.value = `注册应用 ${n(regist)} / 版本 ${ver} / 验证码 ${n(codes)}`
  } catch (e: any) {
    toast.error('加载中心信息失败: ' + (e?.message ?? ''))
  }
}
async function loadApps() {
  loadingApp.value = true
  try {
    const r = await api.get('/api/program_center/application/list')
    applications.value = r.data ?? []
  } catch {
    applications.value = []
  } finally {
    loadingApp.value = false
  }
}
async function loadScripts() {
  loadingScript.value = true
  try {
    const r = await api.get('/api/program_center/script/list')
    scripts.value = r.data ?? []
  } catch {
    scripts.value = []
  } finally {
    loadingScript.value = false
  }
}
async function loadDict() {
  loadingDict.value = true
  try {
    const r = await api.get('/api/program_center/dict/list')
    // 后端 dict/list 回 keyName（= 创建时的 dictFlag/唯一标识），卡片/按钮读 flag，归一。
    dicts.value = ((r.data ?? []) as Dict[]).map((d) => ({ ...d, flag: d.flag ?? d.keyName }))
  } catch {
    dicts.value = []
  } finally {
    loadingDict.value = false
  }
}
async function loadMarket() {
  loadingMarket.value = true
  try {
    const r = await api.post('/api/program_center/market/list/paging/1/size/20', {})
    markets.value = r.data?.list ?? r.data ?? []
  } catch {
    markets.value = []
  } finally {
    loadingMarket.value = false
  }
}

function switchTab(t: Tab) {
  tab.value = t
  if (t === 'agent') loadAgents()
  else if (t === 'application') loadApps()
  else if (t === 'script') loadScripts()
  else if (t === 'dict') loadDict()
  else if (t === 'market') loadMarket()
  else if (t === 'invoke') loadInvokes()
  else if (t === 'config') loadConfigs()
  else if (t === 'style') loadCurrentStyle()
}

async function toggleAgent(a: Agent) {
  try {
    const action = a.enabled !== false ? 'disable' : 'enable'
    await api.get(`/api/program_center/agent/${a.flag || a.id}/${action}`)
    toast.success(action === 'enable' ? '已启用' : '已禁用')
    loadAgents()
  } catch (e: any) {
    toast.error(e?.message ?? '操作失败')
  }
}
async function executeAgent(a: Agent) {
  try {
    // GET /agent/{flag}/execute —— 触发调度并记录日志
    await api.get(`/api/program_center/agent/${a.flag || a.id}/execute`)
    toast.success('已触发执行')
  } catch (e: any) {
    toast.error(e?.message ?? '执行失败')
  }
}

const createAgentM = useMutation({
  mutationFn: (data: { name: string; flag: string }) => api.post('/api/program_center/agent/create', data),
  onSuccess: () => {
    showCreateAgent.value = false
    agentForm.value = { name: '', flag: '' }
    toast.success('Agent已创建')
    loadAgents()
  },
  onError: () => toast.error('创建失败'),
})
async function onCreateAgent() {
  if (!agentForm.value.name || !agentForm.value.flag) return
  createAgentM.mutate(agentForm.value)
}

// Watch tab changes to load data
watch(tab, (t) => switchTab(t), { immediate: true })

const deleteAgentM = useMutation({
  mutationFn: (id: string) => api.delete(`/api/program_center/agent/${id}`),
  onSuccess: () => {
    loadAgents()
    toast.success('Agent已删除')
  },
})
const deleteAppM = useMutation({
  mutationFn: (id: string) => api.delete(`/api/program_center/application/${id}`),
  onSuccess: () => {
    loadApps()
    toast.success('Application已删除')
  },
})
const deleteScriptM = useMutation({
  mutationFn: (id: string) => api.delete(`/api/program_center/script/${id}`),
  onSuccess: () => {
    loadScripts()
    toast.success('Script已删除')
  },
})
const deleteDictM = useMutation({
  mutationFn: (id: string) => api.delete(`/api/program_center/dict/${id}`),
  onSuccess: () => {
    loadDict()
    toast.success('字典已删除')
  },
})
async function deleteAgent(a: Agent) {
  if (await confirmMsg('确定删除该Agent？')) deleteAgentM.mutate(a.id!)
}
async function deleteApp(a: App) {
  if (await confirmMsg('确定删除该Application？')) deleteAppM.mutate(a.id!)
}
async function deleteScript(s: Script) {
  if (await confirmMsg('确定删除该Script？')) deleteScriptM.mutate(s.id!)
}
async function deleteDict(d: Dict) {
  if (await confirmMsg('确定删除该字典？')) deleteDictM.mutate(d.id!)
}

// 新建字典（POST /api/program_center/dict；DictCreateRequest 的 flag 键名为 dictFlag，
// 原 /dict/create 为未注册死端点且误用 flag 键）
const createDictM = useMutation({
  mutationFn: (data: { name: string; dictFlag: string }) => api.post('/api/program_center/dict', data),
  onSuccess: () => {
    showCreateDict.value = false
    dictForm.value = { name: '', dictFlag: '' }
    toast.success('字典已创建')
    loadDict()
  },
  onError: () => toast.error('创建失败'),
})
function onCreateDict() {
  createDictM.mutate(dictForm.value)
}

// ── Agent 属性编辑（POST agent/save/{id}，AgentSaveRequest {name,flag,description}）──
function editAgent(a: Agent): void {
  agentEdit.value = {
    id: String(a.id ?? ''),
    name: a.name ?? a.agentName ?? '',
    flag: a.flag ?? '',
    description: '',
  }
  showEditAgent.value = true
}
async function saveAgentEdit(): Promise<void> {
  const { id, name, flag, description } = agentEdit.value
  if (!id || agentSaving.value) return
  agentSaving.value = true
  try {
    await api.post(`/api/program_center/agent/save/${id}`, { name, flag, description })
    toast.success('Agent 属性已保存')
    showEditAgent.value = false
    loadAgents()
  } catch {
    toast.error('保存失败')
  } finally {
    agentSaving.value = false
  }
}

// ── 字典数据编辑器 ──
async function openDictData(d: Dict): Promise<void> {
  const flag = d.flag
  if (!flag) return
  dictDataFlag.value = flag
  dictDataText.value = ''
  dictDataError.value = ''
  dictDataLoading.value = true
  showDictData.value = true
  try {
    const r = (await api.get(`/api/program_center/dict/${encodeURIComponent(flag)}/data`)) as unknown as {
      data?: { data?: string }
    }
    const raw = r.data?.data
    if (raw) {
      try {
        dictDataText.value = JSON.stringify(JSON.parse(raw), null, 2)
      } catch {
        dictDataText.value = raw
      }
    }
  } catch {
    dictDataError.value = '字典数据加载失败'
  } finally {
    dictDataLoading.value = false
  }
}
function closeDictData(): void {
  showDictData.value = false
  dictDataText.value = ''
  dictDataError.value = ''
}
async function saveDictData(): Promise<void> {
  if (dictDataSaving.value) return
  let body: unknown
  try {
    body = JSON.parse(dictDataText.value)
  } catch {
    dictDataError.value = '数据不是合法 JSON，保存已阻止'
    return
  }
  dictDataError.value = ''
  dictDataSaving.value = true
  try {
    // path 段为占位（handler 仅按 dictFlag 写 app_data）；真实路由无重复 data 段。
    await api.post(`/api/program_center/dict/${encodeURIComponent(dictDataFlag.value)}/data`, body)
    toast.success('字典数据已保存')
    showDictData.value = false
  } catch {
    dictDataError.value = '保存失败'
  } finally {
    dictDataSaving.value = false
  }
}

// ── 脚本代码编辑器（x_program_script.content）──
async function openScriptEditor(s: Script): Promise<void> {
  const flag = s.flag
  if (!flag) return
  scriptEdit.value = { flag, name: s.name ?? s.scriptName ?? '', content: '' }
  scriptEditLoading.value = true
  showScriptEdit.value = true
  try {
    const r = (await api.get(`/api/program_center/script/${encodeURIComponent(flag)}`)) as unknown as {
      data?: { content?: string; name?: string }
    }
    scriptEdit.value.content = r.data?.content ?? ''
    if (r.data?.name) scriptEdit.value.name = r.data.name
  } catch {
    scriptEdit.value.content = ''
  } finally {
    scriptEditLoading.value = false
  }
}
function closeScriptEdit(): void {
  showScriptEdit.value = false
  scriptEdit.value = { flag: '', name: '', content: '' }
}
async function saveScript(): Promise<void> {
  const { flag, name, content } = scriptEdit.value
  if (!flag || scriptEditSaving.value) return
  scriptEditSaving.value = true
  try {
    // script_save_flag：ScriptSaveRequest {name, content, category}，按 flag 定位更新
    await api.post(`/api/program_center/script/${encodeURIComponent(flag)}`, { name, content })
    toast.success('脚本已保存')
    showScriptEdit.value = false
    loadScripts()
  } catch {
    toast.error('脚本保存失败')
  } finally {
    scriptEditSaving.value = false
  }
}

// ── 脚本版本历史（cms crate 已注册 /api/scriptversion/list/script/{scriptId}）──
async function loadVersions(s: Script): Promise<void> {
  const scriptId = String(s.id ?? s.flag ?? '')
  if (!scriptId) return
  versionsScript.value = s
  versions.value = []
  showVersions.value = true
  try {
    const r = (await api.get(`/api/scriptversion/list/script/${encodeURIComponent(scriptId)}`)) as unknown as {
      data?: Array<{ id: string; createTime?: string; create_time?: string }>
    }
    versions.value = r.data ?? []
  } catch {
    versions.value = []
  }
}
function closeVersions(): void {
  showVersions.value = false
  versions.value = []
}

// 模块对比
const compareM = useMutation({
  mutationFn: (id: string) => api.get(`/api/program_center/module/${id}/compare`),
  onSuccess: () => toast.success('对比完成'),
  onError: () => toast.error('对比失败'),
})
function compareApp(app: App) {
  if (!app.id) return
  compareM.mutate(app.id)
}

// 执行脚本
const runScriptM = useMutation({
  mutationFn: (flag: string) => api.post(`/api/program_center/invoke/${flag}/execute`, {}),
  onSuccess: () => toast.success('脚本已执行'),
  onError: () => toast.error('执行失败'),
})
async function runScript(s: Script) {
  if (!s.flag) return
  if (!(await confirmMsg(`确认执行脚本「${s.flag}」？`))) return
  runScriptM.mutate(s.flag)
}

// 收集管理
const collectAddM = useMutation({
  mutationFn: () => api.post('/api/program_center/collect/add', null),
  onSuccess: () => toast.success('收集已添加'),
  onError: () => toast.error('添加失败'),
})
async function loadCollect() {
  try {
    const r = await api.get('/api/program_center/collect/list')
    collectList.value = r.data ?? []
  } catch {
    collectList.value = []
  }
}
const collectList = ref<Array<{ id: string; name: string }>>([])
function addCollect() {
  collectAddM.mutate()
  loadCollect()
}

// AppStyle 图片管理
const eraseImageM = useMutation({
  mutationFn: ({ type, flag }: { type: string; flag: string }) =>
    api.delete(`/api/program_center/appstyle/image/${type}/${flag}/erase`),
  onSuccess: () => toast.success('图片已清除'),
  onError: () => toast.error('操作失败'),
})
async function eraseAppStyleImage(type: string, flag: string) {
  if (!(await confirmMsg(`确认清除 ${type} 图片？`))) return
  eraseImageM.mutate({ type, flag })
}

// 命令执行
const commandExecM = useMutation({
  mutationFn: (data: unknown) => api.post('/api/program_center/command/execute', data),
  onSuccess: () => toast.success('命令已执行'),
  onError: () => toast.error('执行失败'),
})
function execCommand() {
  const cmd = prompt('输入命令JSON:')
  if (!cmd) return
  try {
    commandExecM.mutate(JSON.parse(cmd))
  } catch {
    toast.error('命令JSON格式错误')
  }
}

// Market 扩展
const marketDownloadM = useMutation({
  mutationFn: (flag: string) => api.get(`/api/program_center/market/${flag}/download`),
  onSuccess: () => toast.success('下载已触发'),
  onError: () => toast.error('下载失败'),
})
function downloadMarket(flag: string) {
  marketDownloadM.mutate(flag)
}

const marketCoverPicM = useMutation({
  mutationFn: (flag: string) => api.get(`/api/program_center/market/${flag}/cover/pic`),
  onSuccess: () => toast.success('封面已更新'),
  onError: () => toast.error('操作失败'),
})
function setMarketCover(flag: string) {
  marketCoverPicM.mutate(flag)
}

// 应用市场：分类 / 热门 / 安装-卸载-版本（program_center market 族，均 GET）
const marketCats = ref<string[]>([])
const marketCat = ref('')
const installedVer = ref<Record<string, string>>({})
async function loadMarketCats() {
  try {
    const r: any = await api.get('/api/program_center/market/list/category')
    marketCats.value = ((r.data ?? []) as any[])
      .map((c) => (typeof c === 'string' ? c : (c?.category ?? c?.name)))
      .filter((c): c is string => !!c)
  } catch {
    marketCats.value = []
  }
}

// 接口调用（invoke）：列表 / 新建 / 删除 / 分类（program_center invoke 族）
async function loadInvokeByCategory() {
  try {
    // GET program_center/invoke/list/with/category/category —— 按分类接口清单
    const r: any = await api.get('/api/program_center/invoke/list/with/category/category')
    invokes.value = (r.data ?? []) as Invoke[]
    toast.success('按分类接口：' + invokes.value.length + ' 个')
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
async function loadInvokes() {
  loadingInvoke.value = true
  try {
    const r: any = await api.get('/api/program_center/invoke')
    invokes.value = (r.data ?? []) as Invoke[]
  } catch {
    invokes.value = []
  } finally {
    loadingInvoke.value = false
  }
}
async function createInvoke() {
  const name = prompt('接口名称:')
  if (!name) return
  const alias = prompt('别名（可选）:', '') ?? ''
  const category = prompt('分类（可选）:', '') ?? ''
  try {
    // 后端 u2_invoke_create：U2InvokeRequest{ name(必填)/alias/category/description }
    await api.post('/api/program_center/invoke', { name, alias, category, description: '' })
    loadInvokes()
  } catch (e: any) {
    toast.error('新建失败: ' + (e?.message ?? ''))
  }
}
async function deleteInvoke(iv: Invoke) {
  if (!(await confirmMsg('确定删除接口「' + (iv.name || iv.id) + '」？'))) return
  try {
    await api.delete('/api/program_center/invoke/' + (iv.id || ''))
    loadInvokes()
  } catch (e: any) {
    toast.error('删除失败: ' + (e?.message ?? ''))
  }
}
async function loadInvokeCats() {
  try {
    await api.get('/api/program_center/invoke/list/category')
  } catch {
    /* 分类列表供后续过滤，失败忽略 */
  }
}
loadInvokeCats()

// 平台配置（config）：列表 / 应用 / 实体 / 新建更新（program_center config 族）
const dsText = ref('')
const deployDistText = ref('')
const appStyleText = ref('')
const marketLogText = ref('')
const pcU4Text = ref('')
const progExtraText = ref('')
const progDeepText = ref('')
// rev299：程序中心 代理/市场VIP/已装版本/应用包/图表/收藏 真实读端点集（invoke/flag、market cloud vip、market installed version、apppack info、bar 图表、collect）；均只读 arity<=url 已核；排除 token 密钥/write/binary
async function loadProgramExtraReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const flag = 'default'
  const field = 'id'
  const value = '0'
  const name = 'default'
  const mobile = '0'
  const id = '0'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/program_center/invoke/flag`)),
      s(api.get(`/api/program_center/market/cloud/unit/is/vip`)),
      s(api.get(`/api/program_center/market/${flag}/installed/version`)),
      s(api.get(`/api/program_center/apppack/pack/info`)),
      s(api.get(`/api/program_center/apppack/pack/info/file/last`)),
      s(api.get(`/api/program_center/bar/select1/field/${encodeURIComponent(field)}/value/${encodeURIComponent(value)}/count/20`)),
      s(api.get(`/api/program_center/bar/select2/count/20`)),
      s(api.get(`/api/program_center/collect/code/mobile/${encodeURIComponent(id)}`)),
      s(api.get(`/api/program_center/collect/controllermobile/name/${encodeURIComponent(name)}/mobile/${encodeURIComponent(mobile)}`)),
      // rev302：distribute webserver 源(X_PROGRAM_INVOKE，webserver 变体区别 rev263) 补齐
      s(api.get(`/api/program_center/distribute/webserver/assemble/source/${flag}`)),
      // rev305：market/flag/installed/version 字面量版(x_program_deploy，区别于 rev299 参数版) 补齐
      s(api.get(`/api/program_center/market/flag/installed/version`)),
    ])
    const n = (r: any) => ((r as any)?.data != null ? 1 : 0)
    const hit = rs.reduce((a, r) => a + n(r), 0)
    progExtraText.value = `程序中心补充真实读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载代理/市场/应用包失败: ' + (e?.message ?? ''))
  }
}
// rev310：程序中心 图表literal/市场分页/分发源/发票错误日志/字典/调用列表/部署/脚本 深度读 20 条真实路由
// （handler 体经跨 crate 核实纯 SELECT；已排除 invoke执行/token/captcha/code验证/create/update/download/anony 等副作用与凭证类）
async function loadProgramDeepReads() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const id = '0'
  const name = 'default'
  const category = 'default'
  const flag = 'default'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/program_center/appstyle/image/login/avatar`)),
      s(api.get(`/api/program_center/appstyle/image/setup/about/logo`)),
      s(api.get(`/api/program_center/bar/select1/field/field/value/value/count/count`)),
      s(api.get(`/api/program_center/bar/select2/count/count`)),
      s(api.get(`/api/program_center/bar/select3/field/field/value/value/count/count`)),
      s(api.get(`/api/program_center/bar/select4/field/field/value/value/count/count`)),
      s(api.get(`/api/program_center/code/list/paging/page/size/size`)),
      s(api.get(`/api/program_center/distribute/assemble/source/source`)),
      s(api.get(`/api/program_center/distribute/webserver/assemble/source/source`)),
      s(api.get(`/api/program_center/market/list/install/log/paging/page/size/size`)),
      s(api.get(`/api/program_center/market/list/paging/page/size/size`)),
      s(api.get(`/api/program_center/market/list/paging/page/size/size/category/category`)),
      s(api.get(`/api/program_center/warnlog/${id}`)),
      s(api.get(`/api/program_center/dict/${id}`)),
      s(api.get(`/api/program_center/invoke/list/with/category/${category}`)),
      s(api.get(`/api/program_center/market/${flag}`)),
      s(api.get(`/api/program_center/prompterrorlog/${id}`)),
      s(api.get(`/api/program_center/script/name/${name}/imported`)),
      s(api.get(`/api/program_center/unexpectederrorlog/${id}`)),
      s(api.get(`/api/program_center/deploy/${id}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    progDeepText.value = `程序中心深度读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载程序中心深度读失败: ' + (e?.message ?? ''))
  }
}
// rev325：程序中心 应用风格图清除/采集保存删除/接口保存删除/中心-人员配置/令牌阈值 真实写端点（用户触发，非自动；shape 已核 handler 源码）
async function pcStyleErase(kind: string) {
  if (!(await confirmMsg(`确定清除该风格图片（${kind}）？`))) return
  try {
    if (kind === 'appTop') await api.delete('/api/program_center/appstyle/image/application/top/erase')
    else if (kind === 'launchLogo') await api.delete('/api/program_center/appstyle/image/launch/logo/erase')
    else if (kind === 'loginAvatar') await api.delete('/api/program_center/appstyle/image/login/avatar/erase')
    else if (kind === 'menuBlur') await api.delete('/api/program_center/appstyle/image/menu/logo/blur/erase')
    else if (kind === 'menuFocus') await api.delete('/api/program_center/appstyle/image/menu/logo/focus/erase')
    else if (kind === 'processDefault') await api.delete('/api/program_center/appstyle/image/process/default/erase')
    else await api.delete('/api/program_center/appstyle/image/setup/about/logo/erase')
    toast.success('已清除风格图片')
  } catch (e: any) {
    toast.error('清除失败: ' + (e?.message ?? ''))
  }
}
async function pcCollectSave() {
  const id = prompt('采集配置 ID:', '') || ''
  const name = prompt('名称:', '') || ''
  try {
    await api.put(`/api/program_center/collect/save/${encodeURIComponent(id)}`, { name })
    toast.success('采集配置已保存')
  } catch (e: any) {
    toast.error('保存失败: ' + (e?.message ?? ''))
  }
}
async function pcCollectDelete() {
  const id = prompt('要删除的采集配置 ID:', '') || ''
  if (!(await confirmMsg('确定删除该采集配置？'))) return
  try {
    await api.delete(`/api/program_center/collect/delete/${encodeURIComponent(id)}`)
    toast.success('采集配置已删除')
  } catch (e: any) {
    toast.error('删除失败: ' + (e?.message ?? ''))
  }
}
async function pcInvokeSaveById() {
  const id = prompt('接口配置 ID:', '') || ''
  const name = prompt('名称:', '') || ''
  try {
    await api.put(`/api/program_center/invoke/save/${encodeURIComponent(id)}`, { name })
    toast.success('接口配置已保存')
  } catch (e: any) {
    toast.error('保存失败: ' + (e?.message ?? ''))
  }
}
async function pcInvokeDeleteById() {
  const id = prompt('要删除的接口配置 ID:', '') || ''
  if (!(await confirmMsg('确定删除该接口配置？'))) return
  try {
    await api.delete(`/api/program_center/invoke/delete/${encodeURIComponent(id)}`)
    toast.success('接口配置已删除')
  } catch (e: any) {
    toast.error('删除失败: ' + (e?.message ?? ''))
  }
}
async function pcConfigSave(kind: string) {
  const val = prompt(`${kind} 配置 JSON（可空）:`, '') || ''
  let body: any = {}
  try { if (val) body = JSON.parse(val) } catch { body = { value: val } }
  try {
    if (kind === 'centerserver') await api.put('/api/program_center/config/centerserver', body)
    else await api.put('/api/program_center/config/person', body)
    toast.success('配置已保存')
  } catch (e: any) {
    toast.error('保存失败: ' + (e?.message ?? ''))
  }
}
async function pcTokenThreshold() {
  const t = prompt('令牌阈值（数字）:', '1000') || '0'
  try {
    await api.post('/api/program_center/tokenthreshold/update', { threshold: Number(t) })
    toast.success('令牌阈值已更新')
  } catch (e: any) {
    toast.error('更新失败: ' + (e?.message ?? ''))
  }
}
// rev344：程序中心 告警日志/字典数据·增改删/代理改删/公众号菜单改删 真实写端点（用户触发，shape 已核；避 add·output·updateUnit·create/to/weixin 无参 Path trap500）
async function pcU3(op: string) {
  try {
    if (op === 'warnlog') {
      const message = prompt('告警内容:', '') || ''
      await api.post('/api/program_center/warnlog', { level: 'WARN', message })
    } else if (op === 'dictData') {
      const flag = prompt('字典 flag:', '') || ''
      await api.post(`/api/program_center/dict/${encodeURIComponent(flag)}/data`, {})
    } else if (op === 'dictUpdate') {
      const id = prompt('字典 ID:', '') || ''
      const name = prompt('新名称:', '') || ''
      await api.put(`/api/program_center/dict/${encodeURIComponent(id)}`, { name })
    } else if (op === 'dictDelete') {
      const id = prompt('要删除的字典 ID:', '') || ''
      if (!(await confirmMsg('确定删除该字典？'))) return
      await api.delete(`/api/program_center/dict/${encodeURIComponent(id)}`)
    } else if (op === 'agentUpdate') {
      const flag = prompt('代理 flag:', '') || ''
      await api.put(`/api/program_center/agent/${encodeURIComponent(flag)}`, {})
    } else if (op === 'agentDelete') {
      const flag = prompt('要删除的代理 flag:', '') || ''
      if (!(await confirmMsg('确定删除该代理？'))) return
      await api.delete(`/api/program_center/agent/${encodeURIComponent(flag)}`)
    } else if (op === 'menuUpdate') {
      const id = prompt('公众号菜单 ID:', '') || ''
      await api.post(`/api/program_center/mpweixin/menu/update/${encodeURIComponent(id)}`, {})
    } else if (op === 'menuDelete') {
      const id = prompt('要删除的公众号菜单 ID:', '') || ''
      if (!(await confirmMsg('确定删除该公众号菜单？'))) return
      await api.delete(`/api/program_center/mpweixin/menu/delete/${encodeURIComponent(id)}`)
    } else {
      await api.post('/api/program_center/dict/list/paging/1/size/20', {})
    }
    toast.success('程序中心操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
// rev353：程序中心 配置/脚本/字典数据/输入·输出·设计器 真实写端点（用户触发，shape 已核；config_save 需 key，script_create 可选字段，dict data 任意值）
async function pcU4Write(op: string) {
  try {
    if (op === 'configSave') {
      const key = prompt('配置键(key):', '') || ''
      if (!key) return
      const value = prompt('配置值(value):', '') || ''
      await api.put('/api/program_center/config/save', { key, value })
    } else if (op === 'scriptCreate') {
      const name = prompt('脚本名称:', 'script') || 'script'
      const content = prompt('脚本内容:', '') || ''
      await api.post('/api/program_center/script', { name, content })
    } else if (op === 'dictDataPut') {
      const flag = prompt('字典 flag:', '') || ''
      const path = prompt('字典路径:', 'root') || 'root'
      const value = prompt('数据值(JSON/文本):', '') || ''
      await api.put(`/api/program_center/dict/${encodeURIComponent(flag)}/${encodeURIComponent(path)}/data`, value)
    } else if (op === 'dictDataDel') {
      const flag = prompt('字典 flag:', '') || ''
      const path = prompt('字典路径:', 'root') || 'root'
      if (!(await confirmMsg('确定清空该字典数据？'))) return
      await api.delete(`/api/program_center/dict/${encodeURIComponent(flag)}/${encodeURIComponent(path)}/data`)
    } else if (op === 'inputCreate') {
      await api.post('/api/program_center/input/create', {})
    } else if (op === 'outputList') {
      await api.post('/api/program_center/output/list', {})
    } else {
      await api.post('/api/program_center/designer/search', {})
    }
    toast.success('程序中心操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
// rev353：程序中心 中心服务/许可/人员/门户/代理/开放 配置读 + 部署·脚本·字典分页 + 提示/异常错误日志 真实只读（用户触发按钮，全字面量，非 onMounted）
async function pcU4Read() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [center, license, person, portal, proxy, open, dep, scr, dictP, promptLog, unexLog] = await Promise.all([
      s(api.get('/api/program_center/config/centerserver')),
      s(api.get('/api/program_center/config/license')),
      s(api.get('/api/program_center/config/person')),
      s(api.get('/api/program_center/config/portal')),
      s(api.get('/api/program_center/config/proxy')),
      s(api.get('/api/program_center/config/open')),
      s(api.get('/api/program_center/deploy/list/paging/page/size/size')),
      s(api.get('/api/program_center/script/list/paging/page/size/size')),
      s(api.get('/api/program_center/dict/list/paging/page/size/size')),
      s(api.get('/api/program_center/prompterrorlog/list/id/next/count')),
      s(api.get('/api/program_center/unexpectederrorlog/list/id/next/count')),
    ])
    const ok = (r: any) => (r ? '✓' : '—')
    pcU4Text.value = `中心${ok(center)} 许可${ok(license)} 人员${ok(person)} 门户${ok(portal)} 代理${ok(proxy)} 开放${ok(open)} | 部署${ok(dep)} 脚本${ok(scr)} 字典${ok(dictP)} 提示日志${ok(promptLog)} 异常日志${ok(unexLog)}`
    toast.success('程序中心配置/日志已加载')
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
// rev370：程序中心 提示/异常错误日志 游标(next/prev + date/exceptionclass/loggername) 真实只读（同 rev353 已验证的 list/id/next/count 字面段模式，用户触发）
async function pcU5Read() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.get('/api/program_center/prompterrorlog/list/id/next/count/date/date')),
      s(api.get('/api/program_center/prompterrorlog/list/id/next/count/exceptionclass/exceptionClass')),
      s(api.get('/api/program_center/prompterrorlog/list/id/next/count/loggername/loggerName')),
      s(api.get('/api/program_center/prompterrorlog/list/id/prev/count')),
      s(api.get('/api/program_center/prompterrorlog/list/id/prev/count/date/date')),
      s(api.get('/api/program_center/prompterrorlog/list/id/prev/count/exceptionclass/exceptionClass')),
      s(api.get('/api/program_center/prompterrorlog/list/id/prev/count/loggername/loggerName')),
      s(api.get('/api/program_center/unexpectederrorlog/list/id/next/count/date/date')),
      s(api.get('/api/program_center/unexpectederrorlog/list/id/prev/count')),
      s(api.get('/api/program_center/unexpectederrorlog/list/id/prev/count/date/date')),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    pcU4Text.value = `错误日志游标读 ${rs.length} 条命中 ${hit}`
    toast.success('错误日志游标已加载')
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
// rev370：程序中心 应用删/脚本改/字典数据存/缓存调度/调度上报/模块列表 真实写（各带 {param} 或无参 handler 已核 Path-less，避 module/output 无参 Path trap500）
async function pcU5Write(op: string) {
  try {
    if (op === 'appDelete') { const id = prompt('要删除的应用 ID:', '') || ''; if (!(await confirmMsg('确定删除该应用？'))) return; await api.delete(`/api/program_center/application/delete/${encodeURIComponent(id)}`) }
    else if (op === 'scriptPut') { const flag = prompt('脚本 flag:', '') || ''; await api.put(`/api/program_center/script/${encodeURIComponent(flag)}`, {}) }
    else if (op === 'dictData') { const df = prompt('字典 flag:', '') || ''; const path = prompt('路径:', 'root') || 'root'; await api.post(`/api/program_center/dict/${encodeURIComponent(df)}/${encodeURIComponent(path)}/data`, {}) }
    else if (op === 'cacheDispatch') await api.put('/api/program_center/cachedispatch', {})
    else if (op === 'scheduleReport') await api.post('/api/program_center/schedule/report', {})
    else await api.put('/api/program_center/module/list', {})
    toast.success('程序中心写操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
// rev382：程序中心 agent 代理禁用/启用/保存、invoke 调用器读/删/执行读取/更新、module 输出文件/删结构/写、application 保存 真实路由（Path-only + 已核 struct 字段体，用户触发）
async function pcU6(op: string) {
  try {
    if (op === 'agentDisable') { const f = encodeURIComponent(prompt('代理 flag:', '') || ''); await api.get(`/api/program_center/agent/${f}/disable`) }
    else if (op === 'agentEnable') { const f = encodeURIComponent(prompt('代理 flag:', '') || ''); await api.get(`/api/program_center/agent/${f}/enable`) }
    else if (op === 'agentSave') { const id = encodeURIComponent(prompt('代理 ID:', '') || ''); const name = prompt('代理名称:', '') || ''; await api.put(`/api/program_center/agent/save/${id}`, { name }) }
    else if (op === 'invokeGet') { const f = encodeURIComponent(prompt('调用器 flag:', '') || ''); await api.get(`/api/program_center/invoke/${f}`) }
    else if (op === 'invokeUpdate') { const f = encodeURIComponent(prompt('调用器 flag:', '') || ''); const name = prompt('调用器名称:', '') || ''; await api.put(`/api/program_center/invoke/${f}`, { name, alias: '', category: '', description: '', enable: true, enableToken: false }) }
    else if (op === 'invokeDelete') { const f = encodeURIComponent(prompt('调用器 flag:', '') || ''); if (!(await confirmMsg('确定删除该调用器？'))) return; await api.delete(`/api/program_center/invoke/${f}`) }
    else if (op === 'invokeExecGet') { const f = encodeURIComponent(prompt('调用器 flag:', '') || ''); await api.get(`/api/program_center/invoke/${f}/execute/get`) }
    else if (op === 'moduleOutputFile') { const id = encodeURIComponent(prompt('模块 ID:', '') || ''); await api.get(`/api/program_center/module/output/${id}/file`) }
    else if (op === 'moduleWrite') { const id = encodeURIComponent(prompt('模块 ID:', '') || ''); await api.put(`/api/program_center/module/write/${id}`, {}) }
    else if (op === 'moduleRemoveStruct') { const id = encodeURIComponent(prompt('模块 ID:', '') || ''); if (!(await confirmMsg('确定删除该模块结构？'))) return; await api.delete(`/api/program_center/module/remove/structure/${id}`) }
    else { const id = encodeURIComponent(prompt('应用 ID:', '') || ''); const name = prompt('应用名称:', '') || ''; await api.put(`/api/program_center/application/save/${id}`, { name }) }
    toast.success('程序中心操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
// rev396：程序中心 数据输入 比较/覆盖/创建/预备覆盖/预备创建 + Jest 清缓存(按源) 真实路由（input_* 均 Path-free pool-only 空体、jest_clear_cache handler 忽略 {source} 参不 trap500；POST/PUT 孪生择一，用户触发）
async function pcU7(op: string) {
  try {
    if (op === 'inputCompare') await api.post('/api/program_center/input/compare', {})
    else if (op === 'inputCover') await api.post('/api/program_center/input/cover', {})
    else if (op === 'inputCreate') await api.put('/api/program_center/input/create', {})
    else if (op === 'inputPrepareCover') await api.post('/api/program_center/input/prepare/cover', {})
    else if (op === 'inputPrepareCreate') await api.post('/api/program_center/input/prepare/create', {})
    else { const src = encodeURIComponent(prompt('缓存源:', '') || ''); await api.get(`/api/program_center/jest/clear/cache/${src}`) }
    toast.success('程序中心操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  }
}
// rev400：程序中心 命令执行/部署服务器o2·资源/中心注册应用 真实只读（均 Path-free pool-only；规避 config/get·deploy/id·module/id/compare 是 handler 取 Path 但路由无参的 trap500）
async function pcU8() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.get('/api/program_center/command/execute')),
      s(api.get('/api/program_center/deploy/server/o2')),
      s(api.get('/api/program_center/deploy/server/resource')),
      s(api.get('/api/program_center/center/regist/applications')),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    toast.success(`程序中心命令/部署读 ${rs.length} 条命中 ${hit}`)
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
// rev410：程序中心 调用标记文件·调用器令牌·市场封面图·钉钉回调AES 真实只读（均 GET pool-only query_opt SELECT，handler 源码核实无 Path 提取无副作用；用户触发）
async function pcU9() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.get('/api/program_center/invoke/flag/file')),
      s(api.get('/api/program_center/invoke/token')),
      s(api.get('/api/program_center/market/flag/cover/pic')),
      s(api.get('/api/program_center/dingding/get/callback/aes')),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    toast.success(`程序中心只读 ${rs.length} 条命中 ${hit}`)
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
// rev421：程序中心 集成同步触发 andfx/dingding 拉取同步 + jest 清缓存源（三者 pool-only 向 x_program_sync_log 落审计行，无外部 HTTP；管理员触发的独立后端集成能力）
async function pcU10() {
  if (!(await confirmMsg('确定触发集成同步/清缓存？'))) return
  try {
    await api.get('/api/program_center/andfx/pull/sync')
    await api.get('/api/program_center/dingding/pull/sync')
    await api.get('/api/program_center/jest/clear/cache/source')
    toast.success('集成同步已触发')
  } catch (e: any) {
    toast.error('触发失败: ' + (e?.message ?? ''))
  }
}
// rev424：程序中心 企微拉取同步 + 政务钉钉注册回调 + App 打包 logo 信息（三者 pool-only：前两条落 x_program_sync_log/x_program_callback_registration 审计行，logo 读 x_program_app_pack；无外部 HTTP；管理员触发的独立后端能力）
async function pcU11() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.get('/api/program_center/qiyeweixin/pull/sync')),
      s(api.get('/api/program_center/zhengwudingding/regist/callback')),
      s(api.get('/api/program_center/apppack/pack/info/logo')),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    toast.success(`企微/政务钉钉/打包 ${rs.length} 条命中 ${hit}`)
  } catch (e: any) {
    toast.error('触发失败: ' + (e?.message ?? ''))
  }
}
// rev425：程序中心 当前应用样式(读 x_program_config appstyle)·公众号菜单最新(读 x_program_mpweixin_menu)·App打包服务连接(读 x_program_app_pack 计数) 三条 pool-only 只读，distinct 表
async function pcU12() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.get('/api/program_center/appstyle/current/update')),
      s(api.get('/api/program_center/mpweixin/menu/create/to/weixin')),
      s(api.get('/api/program_center/apppack/server/connect')),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    toast.success(`样式/公众号/打包 ${rs.length} 条命中 ${hit}`)
  } catch (e: any) {
    toast.error('读取失败: ' + (e?.message ?? ''))
  }
}
// rev427：程序中心 WeLink 拉取同步(落 x_program_sync_log)·前端异常日志上报(prompterrorlog→x_program_prompt_error_log·unexpectederrorlog→x_program_unexpected_error_log) 三条 pool-only 写审计/日志，distinct 表，无外部 HTTP
async function pcU13() {
  try {
    await api.get('/api/program_center/welink/pull/sync')
    await api.post('/api/program_center/prompterrorlog', { exceptionClass: 'ClientPromptError', loggerName: 'ProgramCenterApp', message: '控制台手动上报提示异常' })
    await api.post('/api/program_center/unexpectederrorlog', { errorType: 'ClientUnexpected', message: '控制台手动上报未预期异常', stackTrace: '' })
    toast.success('同步/异常日志已上报')
  } catch (e: any) {
    toast.error('上报失败: ' + (e?.message ?? ''))
  }
}
async function loadMarketLogs() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const flag = 'default'
  const category = 'default'
  try {
    const [byFlag, litFlag, paging] = await Promise.all([
      s(api.get(`/api/program_center/market/${encodeURIComponent(flag)}/install/log`)),
      s(api.get(`/api/program_center/market/flag/install/log`)),
      s(api.get(`/api/program_center/market/list/paging/1/size/20/category/${encodeURIComponent(category)}`)),
    ])
    const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
    marketLogText.value = `安装日志(按flag) ${n(byFlag)} | 安装日志(默认) ${n(litFlag)} | 市场分页(按分类) ${n(paging)}`
  } catch (e: any) {
    toast.error('加载市场安装日志失败: ' + (e?.message ?? ''))
  }
}
// rev279：program_center 应用风格图元 5 条真实 distinct 读路由（同表 x_program_deploy_resource 但 resource_type 各异，返回 id/name/type/path 元数据非二进制，arity0）
// app_top/launch_logo/menu_logo_blur/menu_logo_focus/process_default
async function loadAppStyleImages() {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [top, launch, blur, focus, proc] = await Promise.all([
      s(api.get(`/api/program_center/appstyle/image/application/top`)),
      s(api.get(`/api/program_center/appstyle/image/launch/logo`)),
      s(api.get(`/api/program_center/appstyle/image/menu/logo/blur`)),
      s(api.get(`/api/program_center/appstyle/image/menu/logo/focus`)),
      s(api.get(`/api/program_center/appstyle/image/process/default`)),
    ])
    const has = (r: any) => ((r as any)?.data ? '有' : '无')
    appStyleText.value = `应用顶图 ${has(top)} | 启动Logo ${has(launch)} | 菜单Logo模糊 ${has(blur)} | 菜单Logo聚焦 ${has(focus)} | 流程默认 ${has(proc)}`
  } catch (e: any) {
    toast.error('加载应用风格图元失败: ' + (e?.message ?? ''))
  }
}
// rev263：program_center o2部署/市场模块/字典数据/分发源 4 条真实 distinct 读路由
// deploy/server/o2 → x_program_deploy_server(server_type='o2') · market/flag → x_program_module(deleted_at) · dict/{flag}/{path}/data → x_program_dict(flag=$1 arity2) · distribute/assemble/source/{source} → x_program_invoke(category=$1)；均 query_opt 只读、arity 已核
async function loadDeployDictDistribute() {
  const flag = 'default'
  const path = 'data'
  const source = 'default'
  const collectName = 'default'
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const [deploy, market, dict, distribute, collect] = await Promise.all([
      s(api.get(`/api/program_center/deploy/server/o2`)),
      s(api.get(`/api/program_center/market/flag`)),
      s(api.get(`/api/program_center/dict/${encodeURIComponent(flag)}/${encodeURIComponent(path)}/data`)),
      s(api.get(`/api/program_center/distribute/assemble/source/${encodeURIComponent(source)}`)),
      // rev274：collect/name/{name}/exist → x_program_collect(新表，query_opt arity1)
      s(api.get(`/api/program_center/collect/name/${encodeURIComponent(collectName)}/exist`)),
    ])
    const has = (r: any) => ((r as any)?.data ? '命中' : '未命中')
    deployDistText.value = `o2部署 ${has(deploy)} | 市场模块 ${has(market)} | 字典数据 ${has(dict)} | 分发源 ${has(distribute)} | 收藏存在 ${has(collect)}`
  } catch (e: any) {
    toast.error('加载部署/字典/分发失败: ' + (e?.message ?? ''))
  }
}
async function loadDataStructure() {
  try {
    // GET program_center/datastructure/modules/all + module/output/list/structure —— 数据结构模块
    const [mods, structs] = await Promise.all([
      api.get('/api/program_center/datastructure/modules/all'),
      api.get('/api/program_center/module/output/list/structure'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    dsText.value = `数据结构模块 ${n(mods)} / 输出结构 ${n(structs)}`
  } catch (e: any) {
    toast.error('加载数据结构失败: ' + (e?.message ?? ''))
  }
}
async function loadDesignerJest() {
  try {
    // 消费 program_center 三条无参真实路由：设计器搜索 / Jest 中心清单 / 脚本基准
    const [designer, jestCenter, bench] = await Promise.all([
      api.get('/api/program_center/designer/search'),
      api.get('/api/program_center/jest/center/list'),
      api.get('/api/program_center/validation/scripting/benchmark'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    dsText.value = `设计器搜索 ${n(designer)} / 中心测试 ${n(jestCenter)} / 脚本基准 ${n(bench)}`
  } catch (e: any) {
    toast.error('加载设计器/测试失败: ' + (e?.message ?? ''))
  }
}
async function loadProgramAlias() {
  try {
    // 消费 program 别名族三条真实路由（与 program_center 前缀不同的注册路径）：应用清单 / 当前样式 / 数据结构模块
    const [apps, style, modules] = await Promise.all([
      api.get('/api/program/applications'),
      api.get('/api/program/appstyle/current/style'),
      api.get('/api/program/datastructure/modules/all'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    dsText.value = `应用 ${n(apps)} / 当前样式 ${n(style)} / 数据结构 ${n(modules)}`
  } catch (e: any) {
    toast.error('加载应用别名失败: ' + (e?.message ?? ''))
  }
}
async function loadWeixinMeta() {
  try {
    // 消费 program_center 三条无参真实路由：微信菜单清单 / 校验元数据 / 模块输出结构
    const [menu, meta, struct] = await Promise.all([
      api.get('/api/program_center/mpweixin/menu/list/weixin'),
      api.get('/api/program_center/validation/meta'),
      api.get('/api/program_center/module/output/structure'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    dsText.value = `微信菜单 ${n(menu)} / 校验元 ${n(meta)} / 输出结构 ${n(struct)}`
  } catch (e: any) {
    toast.error('加载微信/校验失败: ' + (e?.message ?? ''))
  }
}
async function loadErrorLogStats() {
  try {
    // 消费 program_center 三条无参真实路由：错误日志按异常类统计 / 按 logger 统计 / 当前节点转储数据
    const [byExc, byLogger, curNode] = await Promise.all([
      api.get('/api/program_center/prompterrorlog/count/exceptionclass'),
      api.get('/api/program_center/prompterrorlog/count/loggername'),
      api.get('/api/program_center/config/list/dump/data/current/node'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    dsText.value = `按异常类 ${n(byExc)} / 按Logger ${n(byLogger)} / 当前节点 ${n(curNode)}`
  } catch (e: any) {
    toast.error('加载错误日志统计失败: ' + (e?.message ?? ''))
  }
}
async function loadOutputMeta() {
  try {
    // 消费 program_center 三条无参真实路由：输出清单 / 模块分类清单 / 存储映射
    const [output, modCat, storage] = await Promise.all([
      api.get('/api/program_center/output/list'),
      api.get('/api/program_center/module/list/category'),
      api.get('/api/program_center/storagemappings'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    dsText.value = `输出 ${n(output)} / 模块分类 ${n(modCat)} / 存储映射 ${n(storage)}`
  } catch (e: any) {
    toast.error('加载输出/存储失败: ' + (e?.message ?? ''))
  }
}
async function loadSchedule() {
  try {
    // 消费 program_center 三条无参真实路由：调度清单 / 本地调度清单 / 调度报告
    const [sched, local, report] = await Promise.all([
      api.get('/api/program_center/schedule/list/schedule'),
      api.get('/api/program_center/schedule/list/schedulelocal'),
      api.get('/api/program_center/schedule/report'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    dsText.value = `调度 ${n(sched)} / 本地调度 ${n(local)} / 报告 ${n(report)}`
  } catch (e: any) {
    toast.error('加载调度失败: ' + (e?.message ?? ''))
  }
}
async function loadJestModule() {
  try {
    // 消费 program_center 三条无参真实路由：Jest 测试清单 / Jest 版本 / 模块清单
    const [jest, version, modules] = await Promise.all([
      api.get('/api/program_center/jest/list'),
      api.get('/api/program_center/jest/version'),
      api.get('/api/program_center/module/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    const ver = (version as any)?.data?.version ?? ((version as any)?.data ? '有' : '—')
    dsText.value = `测试 ${n(jest)} / 版本 ${ver} / 模块 ${n(modules)}`
  } catch (e: any) {
    toast.error('加载测试/模块失败: ' + (e?.message ?? ''))
  }
}
async function loadConfigDump() {
  try {
    // 消费 program_center 三条无参真实路由：配置转储 / 转储数据清单 / 三元配置管理
    const [dump, dumpData, ternary] = await Promise.all([
      api.get('/api/program_center/config'),
      api.get('/api/program_center/config/list/dump/data'),
      api.get('/api/program_center/config/ternary/management'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : ((r as any)?.data ? 1 : 0))
    dsText.value = `配置转储 ${n(dump)} / 转储数据 ${n(dumpData)} / 三元管理 ${n(ternary)}`
  } catch (e: any) {
    toast.error('加载配置转储失败: ' + (e?.message ?? ''))
  }
}
async function loadDeployMeta() {
  try {
    // 消费 program_center 三条无参真实路由：系统 Token 配置 / 部署资源 / 节点命令清单
    const [token, resource, nodes] = await Promise.all([
      api.get('/api/program_center/config/token'),
      api.get('/api/program_center/deploy/server/resource'),
      api.get('/api/program_center/command/list/node'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    const hasToken = (token as any)?.data ? '有' : '无'
    dsText.value = `系统Token ${hasToken} / 部署资源 ${n(resource)} / 节点 ${n(nodes)}`
  } catch (e: any) {
    toast.error('加载部署信息失败: ' + (e?.message ?? ''))
  }
}
async function loadDsTables() {
  try {
    // 消费 program_center 三条无参真实路由：全部数据表 / 全部字段 / 验证码清单
    const [tables, fields, captcha] = await Promise.all([
      api.get('/api/program_center/datastructure/tables/all'),
      api.get('/api/program_center/datastructure/fileds/all'),
      api.get('/api/program_center/captcha/list'),
    ])
    const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
    dsText.value = `数据表 ${n(tables)} / 字段 ${n(fields)} / 验证码 ${n(captcha)}`
  } catch (e: any) {
    toast.error('加载表结构失败: ' + (e?.message ?? ''))
  }
}
async function loadConfigs() {
  loadingConfig.value = true
  try {
    const r: any = await api.get('/api/program_center/config/list')
    configs.value = (r.data ?? []) as Config[]
  } catch {
    configs.value = []
  } finally {
    loadingConfig.value = false
  }
}
async function loadConfigApps() {
  loadingConfig.value = true
  try {
    const r: any = await api.get('/api/program_center/config/list/application')
    configs.value = (r.data ?? []) as Config[]
  } catch {
    configs.value = []
  } finally {
    loadingConfig.value = false
  }
}
async function loadConfigEntities() {
  loadingConfig.value = true
  try {
    const r: any = await api.get('/api/program_center/config/list/entity')
    configs.value = (r.data ?? []) as Config[]
  } catch {
    configs.value = []
  } finally {
    loadingConfig.value = false
  }
}
async function saveConfig() {
  const key = prompt('配置 Key:')
  if (!key) return
  const value = prompt('配置 Value（可选）:', '') ?? ''
  const category = prompt('分类（可选）:', '') ?? ''
  try {
    // 后端 config_save：ConfigSaveRequest{ key(必填)/value/category/creator }，key 冲突则更新
    await api.post('/api/program_center/config/save', { key, value, category })
    loadConfigs()
  } catch (e: any) {
    toast.error('保存失败: ' + (e?.message ?? ''))
  }
}
// 应用样式（appstyle）：当前样式 / 门户应用（program_center appstyle 族，GET 只读）
async function loadCurrentStyle() {
  loadingStyle.value = true
  try {
    const r: any = await api.get('/api/program_center/appstyle/current/style')
    const d = r.data
    styleApps.value = (Array.isArray(d) ? d : d ? [d] : []) as StyleApp[]
  } catch {
    styleApps.value = []
  } finally {
    loadingStyle.value = false
  }
}
async function loadPortalApps() {
  loadingStyle.value = true
  try {
    const r: any = await api.get('/api/program_center/appstyle/index/portal')
    styleApps.value = (r.data ?? []) as StyleApp[]
  } catch {
    styleApps.value = []
  } finally {
    loadingStyle.value = false
  }
}
loadMarketCats()
function filterCat(cat: string) {
  marketCat.value = cat
  if (!cat) {
    loadMarket()
  } else {
    markets.value = markets.value.filter((m) => (m as any).category === cat)
  }
}
async function loadTopThree() {
  try {
    const r: any = await api.get('/api/program_center/market/list/top/three')
    markets.value = (r.data ?? []) as Market[]
  } catch {
    toast.error('加载热门失败')
  }
}
async function installMarket(m: Market) {
  try {
    await api.get(`/api/program_center/market/${encodeURIComponent(m.id || '')}/install/or/update`)
    toast.success('安装/更新已触发')
    checkVersion(m)
  } catch {
    toast.error('安装失败')
  }
}
async function checkVersion(m: Market) {
  if (!m.id) return
  try {
    const r: any = await api.get(`/api/program_center/market/${encodeURIComponent(m.id)}/installed/version`)
    installedVer.value = { ...installedVer.value, [m.id]: r.data?.version || '未安装' }
  } catch {
    /* 未部署时后端返回 error，忽略 */
  }
}
async function uninstallMarket(m: Market) {
  if (!(await confirmMsg('确定卸载「' + (m.name || m.title || m.id) + '」？'))) return
  try {
    await api.get(`/api/program_center/market/${encodeURIComponent(m.id || '')}/uninstall`)
    toast.success('卸载已触发')
  } catch {
    toast.error('卸载失败')
  }
}
loadMarketCats()

// MPWeixin 扩展
// 注：后端 /api/program_center/mpweixin/check 与 /mpweixin/menu/add 为无参占位注册，
// 其 handler 需 Path 参数 → 运行时必 500（同 BBS 裸路由问题），且无参数化正确路由可改调；
// 小程序菜单管理（list/delete/update 参数化路由可用但无对应管理 UI）暂不提供入口，
// 移除死调用避免客户打到 500。

const file_download_pk_1_ref = ref<any[]>([])
const mass_0_10_ref = ref<any[]>([])
const m_1_install_log_ref = ref<any[]>([])
const program_center_validation_timeout_30000_ref = ref<any[]>([])
const program_center_deploy_server_o2_ref = ref<any[]>([])
const program_center_module_m_1_compare_ref = ref<any[]>([])
const market_m_1_installed_version_ref = ref<any[]>([])
const program_center_market_m_1_uninstall_ref = ref<any[]>([])
const market_m_1_cover_pic_ref = ref<any[]>([])
const list_schedulelog_application_app_1_ref = ref<any[]>([])
const program_center_prompterrorlog_p_1_ref = ref<any[]>([])
const list_p_1_next_10_ref = ref<any[]>([])
const bar_select2_count_10_ref = ref<any[]>([])
const create_mass_5_20_ref = ref<any[]>([])
const bar_select2_count_count_ref = ref<any[]>([])
const program_center_agent_a_1_ref = ref<any[]>([])
const program_center_test_test2_ref = ref<any[]>([])
const module_output_m_1_file_ref = ref<any[]>([])
const output_f_1_select_file_ref = ref<any[]>([])
const program_center_agent_a_1_disable_ref = ref<any[]>([])
const c_1_validate_answer_1234_ref = ref<any[]>([])
const program_center_invoke_i_1_execute_ref = ref<any[]>([])
const program_center_agent_a_1_execute_ref = ref<any[]>([])
const list_id_next_count_1_ref = ref<any[]>([])
const program_center_market_m_1_download_ref = ref<any[]>([])
const mpweixin_menu_delete_wm_1_ref = ref<any[]>([])
const s_1_app_app_1_imported_ref = ref<any[]>([])
const script_s_1_ref = ref<any[]>([])
const scriptversion_sv_1_ref = ref<any[]>([])
const list_i_1_next_10_ref = ref<any[]>([])
const script_s_1_appInfo_app_1_ref = ref<any[]>([])
const script_s_1_app_app_1_ref = ref<any[]>([])
const scriptversion_list_script_s_1_ref = ref<any[]>([])
const program_center_warnlog_w_1_ref = ref<any[]>([])
const module_remove_structure_m_1_ref = ref<any[]>([])
const create_mass_from_count_1_ref = ref<any[]>([])
const distribute_assemble_source_o2_ref = ref<any[]>([])
const program_center_market_m_1_ref = ref<any[]>([])
const program_center_output_f_1_select_ref = ref<any[]>([])
const program_center_unexpectederrorlog_u_1_ref = ref<any[]>([])
const webserver_assemble_source_o2_ref = ref<any[]>([])
const program_center_invoke_i_1_file_ref = ref<any[]>([])
const invoke_i_1_execute_get_ref = ref<any[]>([])
const program_center_deploy_d_1_ref = ref<any[]>([])
const code_create_mobile_13800000000_ref = ref<any[]>([])
const program_center_agent_a_1_enable_ref = ref<any[]>([])
const list_w_1_prev_5_ref = ref<any[]>([])
const program_center_test_test1_ref = ref<any[]>([])
const list_u_1_prev_5_ref = ref<any[]>([])
const m_1_install_or_update_ref = ref<any[]>([])
const pack_info_file_last_1_ref = ref<any[]>([])
const program_center_module_write_m_1_ref = ref<any[]>([])
</script>

<style scoped>
.mod-view{display:flex;flex-direction:column;gap:16px;height:100%}
.view-header{padding:16px 24px}
.view-header h1{font-family:'Orbitron',sans-serif;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}
.subtitle{font-size:12px;color:var(--text-muted);margin:0;font-family:'JetBrains Mono',monospace}
.content-panel{flex:1;overflow-y:auto;padding:16px;display:flex;flex-direction:column;gap:16px}
.tabs{display:flex;gap:8px;flex-wrap:wrap}
.tabs button{padding:8px 16px;background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-secondary);font-size:13px;cursor:pointer;transition:all var(--transition-fast)}
.tabs button.active{background:var(--color-primary);color:#000;border-color:var(--color-primary);font-weight:600}
.toolbar{display:flex;gap:8px}
.btn-primary{padding:8px 20px;background:var(--color-primary);color:#000;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-create{padding:8px 20px;background:var(--color-accent);color:#fff;border:none;border-radius:var(--radius-md);font-size:13px;cursor:pointer;font-weight:600}
.btn-sm{padding:4px 12px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-sm);font-size:12px;cursor:pointer}
.btn-sm:hover{border-color:var(--color-primary);color:var(--color-primary)}
.tab-content{flex:1;display:flex;flex-direction:column;gap:12px;overflow-y:auto}
.item-table{display:flex;flex-direction:column;gap:8px}
.table-header{display:grid;grid-template-columns:2fr 1fr 80px 100px;padding:8px 12px;background:var(--bg-elevated);border-radius:var(--radius-sm);font-size:12px;color:var(--text-muted);font-weight:600}
.table-row{display:grid;grid-template-columns:2fr 1fr 80px 100px;padding:12px;align-items:center;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.table-row:hover{border-color:var(--color-primary)}
.col-name{font-size:14px;font-weight:500;color:var(--text-primary);overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.col-flag,.col-id{font-size:11px;color:var(--text-muted);font-family:'JetBrains Mono',monospace}
.col-status{font-size:12px;padding:2px 8px;border-radius:var(--radius-sm);width:fit-content}
.col-status.enabled{background:rgba(16,185,129,.15);color:var(--color-success)}
.col-status.disabled{background:rgba(239,68,68,.15);color:var(--color-error)}
.item-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(200px,1fr));gap:12px}
.item-card{display:flex;align-items:center;gap:12px;padding:14px;transition:all var(--transition-fast);border:1px solid var(--border-subtle);border-radius:var(--radius-md);background:var(--bg-elevated)}
.item-card:hover{border-color:var(--color-primary);transform:translateX(4px);box-shadow:var(--shadow-glow)}
.ic{font-size:28px}
.ib{flex:1;min-width:0}
.it{font-size:14px;font-weight:600;color:var(--text-primary)}
.im{font-size:12px;color:var(--text-muted);margin-top:2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.meta{font-size:10px;color:var(--color-primary-deep);margin-top:4px;font-family:'JetBrains Mono',monospace}
.empty,.loading-row{display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;color:var(--text-muted);gap:12px;flex:1}
.ei{font-size:48px;opacity:0.4}
.sk{height:40px;border-radius:var(--radius-md);background:var(--bg-elevated);animation:pulse 1.2s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.4}50%{opacity:.8}}
.modal-overlay{position:fixed;inset:0;background:rgba(0,0,0,.7);display:flex;align-items:center;justify-content:center;z-index:100}
.modal{background:var(--bg-surface);border:1px solid var(--border-subtle);border-radius:var(--radius-lg);padding:24px;width:400px;max-width:90vw;display:flex;flex-direction:column;gap:16px}
.modal h3{font-family:'Orbitron',sans-serif;color:var(--color-primary);margin:0}
.form-group{display:flex;flex-direction:column;gap:6px}
.form-group label{font-size:13px;color:var(--text-muted)}
.form-input{background:var(--bg-elevated);border:1px solid var(--border-subtle);border-radius:var(--radius-md);color:var(--text-primary);padding:10px 12px;font-size:14px}
.form-input:focus{outline:none;border-color:var(--color-primary)}
.modal-actions{display:flex;justify-content:flex-end;gap:8px}
.btn-cancel{padding:8px 20px;background:transparent;border:1px solid var(--border-subtle);color:var(--text-secondary);border-radius:var(--radius-md);cursor:pointer}
.font-mono{font-family:'JetBrains Mono',monospace}
.mono{font-family:'JetBrains Mono',monospace;font-size:12px}
.hint{padding:20px;text-align:center;color:var(--text-muted);font-size:13px}
.ver-table{width:100%;border-collapse:collapse;margin-bottom:8px}
.ver-table th,.ver-table td{padding:8px 10px;text-align:left;border-bottom:1px solid var(--border-subtle);font-size:13px}
.ver-table th{color:var(--text-muted);font-size:11px;text-transform:uppercase}
.market-bar{display:flex;flex-wrap:wrap;gap:8px;margin-bottom:12px}
.chip{padding:4px 12px;border-radius:14px;border:1px solid var(--border-subtle);background:var(--bg-elevated);color:var(--text-secondary);cursor:pointer;font-size:12px}
.chip.on{border-color:var(--color-primary);color:var(--color-primary);background:var(--color-primary-soft)}
.market-acts{display:flex;gap:6px;flex-wrap:wrap}
.app-meta{margin-left:10px;font-size:12px;color:var(--text-muted)}
</style>
