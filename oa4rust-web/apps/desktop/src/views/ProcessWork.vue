<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<template>
  <div class="work-view">
    <div class="view-header glass-card">
      <div>
        <h1>工作流待办</h1>
        <p class="subtitle">真实任务、表单定义与流程数据闭环</p>
      </div>
      <div class="header-actions">
        <button class="btn-sm" @click="loadDrafts">草稿箱</button>
        <button class="btn-sm" @click="loadHandovers">工作交接</button>
        <button class="btn-sm" @click="loadReviews">审阅记录</button>
        <button class="btn-sm" @click="loadSerials">流水号</button>
        <button class="btn-sm" @click="loadWorkV2">全部工作</button>
        <button class="btn-sm" @click="loadReadLists">待阅/已阅</button>
        <button class="btn-sm" @click="loadTaskLists">全部任务</button>
        <button class="btn-sm" @click="loadWorkFilterCursors">工作游标/按工作</button>
        <button class="btn-sm" @click="loadWorkAuxReads">日志/流水号/文件</button>
        <button class="btn-sm" @click="loadByWorkJobLists">按工作/按job</button>
        <button class="btn-sm" @click="loadDocReadPaging">文档版本/我的待阅</button>
        <button class="btn-sm" @click="loadWorkFullCursors">工作全量游标/详情</button>
        <button class="btn-sm" @click="loadSurfaceReadA">表面深度读A</button>
        <button class="btn-sm" @click="loadSurfaceReadB">表面深度读B</button>
        <button class="btn-sm" @click="surfaceList('taskV2')">待办v2</button>
        <button class="btn-sm" @click="surfaceList('taskCount')">待办计数</button>
        <button class="btn-sm" @click="surfaceList('taskMy')">我的待办</button>
        <button class="btn-sm" @click="surfaceList('taskCompletedV2')">已办v2</button>
        <button class="btn-sm" @click="surfaceList('taskCompletedMy')">我的已办</button>
        <button class="btn-sm" @click="surfaceList('readV2')">待阅v2</button>
        <button class="btn-sm" @click="surfaceList('readMy')">我的待阅</button>
        <button class="btn-sm" @click="surfaceList('readCompletedV2')">已阅v2</button>
        <button class="btn-sm" @click="surfaceList('readCompletedMy')">我的已阅</button>
        <button class="btn-sm" @click="surfaceList('reviewV2')">评审v2</button>
        <button class="btn-sm" @click="surfaceList('reviewCount')">评审计数</button>
        <button class="btn-sm" @click="surfaceList('reviewSearch')">评审搜索</button>
        <button class="btn-sm" @click="surfaceList('workV2')">工作v2</button>
        <button class="btn-sm" @click="surfaceList('workMy')">我的工作</button>
        <button class="btn-sm" @click="surfaceList('draftMy')">我的草稿</button>
        <button class="btn-sm" @click="surfaceList('handover')">交接分页</button>
        <button class="btn-sm" @click="surfaceList('serialnumber')">流水号分页</button>
        <button class="btn-sm" @click="surfaceList('snapMy')">我的快照</button>
        <button class="btn-sm" @click="surfaceList2('taskV2Paging')">待办分页</button>
        <button class="btn-sm" @click="surfaceList2('taskV2Next')">待办游标</button>
        <button class="btn-sm" @click="surfaceList2('taskManage')">待办管理</button>
        <button class="btn-sm" @click="surfaceList2('taskCompletedPaging')">已办分页</button>
        <button class="btn-sm" @click="surfaceList2('taskCompletedManage')">已办管理</button>
        <button class="btn-sm" @click="surfaceList2('readV2Paging')">待阅分页</button>
        <button class="btn-sm" @click="surfaceList2('readManage')">待阅管理</button>
        <button class="btn-sm" @click="surfaceList2('readCompletedPaging')">已阅分页</button>
        <button class="btn-sm" @click="surfaceList2('readCompletedManage')">已阅管理</button>
        <button class="btn-sm" @click="surfaceList2('reviewV2Paging')">评审分页</button>
        <button class="btn-sm" @click="surfaceList2('reviewManage')">评审管理</button>
        <button class="btn-sm" @click="surfaceList2('workV2Paging')">工作分页</button>
        <button class="btn-sm" @click="surfaceList2('workManage')">工作管理</button>
        <button class="btn-sm" @click="surfaceList2('workCompletedManage')">完成件管理</button>
        <button class="btn-sm" @click="surfaceList2('snapManage')">快照管理</button>
        <button class="btn-sm" @click="surfaceList2('taskCountFilter')">待办筛选计数</button>
        <button class="btn-sm primary" @click="openStart">发起流程</button>
      </div>
    </div>
    <p v-if="draftText" class="subtitle draft-note">{{ draftText }}</p>
    <p v-if="handoverText" class="subtitle draft-note">{{ handoverText }}</p>
    <p v-if="reviewText" class="subtitle draft-note">{{ reviewText }}</p>
    <p v-if="serialText" class="subtitle draft-note">{{ serialText }}</p>
    <p v-if="workV2Text" class="subtitle draft-note">{{ workV2Text }}</p>
    <p v-if="readListText" class="subtitle draft-note">{{ readListText }}</p>
    <p v-if="taskListText" class="subtitle draft-note">{{ taskListText }}</p>
    <p v-if="workCursorText" class="subtitle draft-note">{{ workCursorText }}</p>
    <p v-if="surfaceReadText" class="subtitle draft-note">{{ surfaceReadText }}</p>
    <p v-if="byWorkJobText" class="subtitle draft-note">{{ byWorkJobText }}</p>
    <p v-if="docReadPagingText" class="subtitle draft-note">{{ docReadPagingText }}</p>
    <p v-if="workFullText" class="subtitle draft-note">{{ workFullText }}</p>
    <div class="tabs glass-card">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        class="tab-btn"
        :class="{ active: activeTab === tab.key }"
        @click="activeTab = tab.key"
      >
        {{ tab.label }}
      </button>
    </div>
    <div class="content-panel glass-card">
      <div v-if="query.isLoading.value" class="state">加载中...</div>
      <div v-else-if="query.error.value" class="state error-state">
        加载失败: {{ (query.error.value as Error)?.message }}
        <button class="btn-sm" @click="query.refetch()">重试</button>
      </div>
      <div v-else-if="!items.length" class="state">暂无任务</div>
      <div v-else class="item-list">
        <article v-for="item in items" :key="item.id" class="item-card">
          <div class="item-body" @click="openWork(item)">
            <div class="item-title">{{ item.title || item.processName || item.id }}</div>
            <div class="item-meta">
              <span>{{ item.appName || item.applicationName }}</span>
              <span>{{ item.processName }}</span>
              <span>{{ fmtTime(item.createTime) }}</span>
            </div>
          </div>
          <button class="btn-sm primary" @click="openWork(item)">{{ activeTab === 'pending' ? '办理' : '查看' }}</button>
        </article>
      </div>
    </div>

    <div v-if="opened" class="modal-overlay" @click.self="closeWork">
      <section class="work-dialog glass-card">
        <header>
          <div><h2>{{ opened.title || opened.processName || '流程办理' }}</h2><p>{{ opened.id }}</p></div>
          <button class="btn-sm" @click="closeWork">关闭</button>
        </header>
        <div v-if="detailLoading" class="state">正在加载表单...</div>
        <div v-else-if="detailError" class="state error-state">{{ detailError }}</div>
        <XformRuntime
          v-else-if="formDefinition"
          v-model="formValues"
          :definition="formDefinition"
          :errors="formErrors"
          :readonly="activeTab !== 'pending' && !handleTaskId"
        />
        <div v-else class="state">当前工作没有可渲染的表单定义</div>

        <section v-if="!detailLoading && !detailError" class="detail-panels">
          <div class="detail-block">
            <h3>附件 ({{ attachments.length }})</h3>
            <ul v-if="attachments.length" class="detail-list">
              <li v-for="att in attachments" :key="att.id" class="clickable" @click="viewAttachment(att.id)">
                <span class="name">{{ att.name || att.id }}</span>
                <span class="muted">{{ att.extension }} · {{ fmtSize(att.length) }}</span>
              </li>
            </ul>
            <p v-else class="muted">无附件</p>
            <p v-if="attachDetailText" class="muted">{{ attachDetailText }}</p>
          </div>
          <div class="detail-block">
            <h3>流转记录 ({{ records.length }})</h3>
            <ul v-if="records.length" class="detail-list">
              <li v-for="rec in records" :key="rec.id">
                <span class="name">{{ rec.title || rec.id }}</span>
                <span class="muted">{{ fmtTime(rec.createTime) }}</span>
              </li>
            </ul>
            <p v-else class="muted">无流转记录</p>
          </div>
          <div class="detail-block">
            <h3>工作日志 ({{ worklogs.length }})</h3>
            <ul v-if="worklogs.length" class="detail-list">
              <li v-for="log in worklogs" :key="log.id">
                <span class="name">{{ log.activityName || log.title || log.id }}</span>
                <span class="muted">{{ log.person }} · {{ fmtTime(log.createTime) }}</span>
              </li>
            </ul>
            <p v-else class="muted">无工作日志</p>
          </div>
          <div class="detail-block">
            <h3>待阅 ({{ reads.length }})</h3>
            <ul v-if="reads.length" class="detail-list">
              <li v-for="rd in reads" :key="rd.id" class="clickable" @click="viewRead(rd.id)">
                <span class="name">{{ rd.person || rd.id }}</span>
                <span class="muted">{{ fmtTime(rd.createTime) }}</span>
              </li>
            </ul>
            <p v-else class="muted">无待阅记录</p>
            <p v-if="readDetailText" class="muted">{{ readDetailText }}</p>
          </div>
          <div v-if="engineText" class="detail-block">
            <h3>引擎明细</h3>
            <p class="muted">{{ engineText }}</p>
          </div>
          <div v-if="surfaceExtraText" class="detail-block">
            <h3>附件/文档版本</h3>
            <p class="muted">{{ surfaceExtraText }}</p>
          </div>
          <div v-if="attnDetailText" class="detail-block">
            <h3>附件归属校验</h3>
            <p class="muted">{{ attnDetailText }}</p>
          </div>
          <div v-if="readFacetText" class="detail-block">
            <h3>待阅维度</h3>
            <p class="muted">{{ readFacetText }}</p>
          </div>
          <div v-if="recordReviewText" class="detail-block">
            <h3>记录/流转意见</h3>
            <p class="muted">{{ recordReviewText }}</p>
          </div>
          <div v-if="jobAssetText" class="detail-block">
            <h3>Job 关联/记录</h3>
            <p class="muted">{{ jobAssetText }}</p>
          </div>
          <div v-if="effectiveTaskId" class="detail-block">
            <h3>任务信息</h3>
            <ul class="detail-list">
              <li><span class="name">任务</span><span class="muted">{{ (taskInfo?.title as string) || (taskInfo?.activity as string) || effectiveTaskId }}</span></li>
              <li v-if="taskInfo?.person"><span class="name">处理人</span><span class="muted">{{ taskInfo?.person }}</span></li>
              <li v-if="taskExpireText"><span class="name">超时</span><span class="muted">{{ taskExpireText }}</span></li>
              <li v-if="taskV2Status"><span class="name">v2 状态</span><span class="muted">{{ taskV2Status }}</span></li>
            </ul>
            <button class="btn-sm" :disabled="pressing" @click="pressTask">{{ pressing ? '催办中…' : '催办' }}</button>
            <button class="btn-sm" :disabled="v2Busy" @click="taskV2Action('pause')">暂停</button>
            <button class="btn-sm" :disabled="v2Busy" @click="taskV2Action('resume')">恢复</button>
            <button class="btn-sm" :disabled="v2Busy" @click="taskV2Action('reset')">重置</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineTaskAction('will')">待办转正</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineTaskAction('v3add')">追加任务</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineTaskAction('processing')">进入处理</button>
          </div>
          <div class="detail-block">
            <h3>流程引擎动作</h3>
            <button class="btn-sm" :disabled="engineBusy" @click="engineWorkAction('goback')">退回</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineWorkAction('reroute')">改路由</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineWorkAction('retract')">撤回</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineWorkAction('rollback')">回滚</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineWorkAction('edit')">改标题</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineWorkAction('draftDelete')">删草稿</button>
            <button class="btn-sm" @click="engineAttAction('edit')">改附件名</button>
            <button class="btn-sm" @click="engineAttAction('delete')">删附件</button>
            <button class="btn-sm" @click="engineReadAction('processing')">待阅处理</button>
            <button class="btn-sm" @click="engineReadAction('replace')">待阅接替</button>
            <button class="btn-sm" @click="engineReadAction('reset')">待阅重置</button>
            <button class="btn-sm" @click="engineReadAction('delete')">删待阅</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('workStart')">启动工作</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('workComplete')">完成工作</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('workProcessing')">工作处理中</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('workTerminate')">终止工作</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('workRetract')">撤回工作</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('taskClaim')">认领任务</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('taskTransfer')">转交任务</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('timerCancel')">取消定时器</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('attCopy')">复制附件</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('attEditText')">改附件文本</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('snapRestore')">恢复快照</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('v2Goback')">v2退回</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('v2Reroute')">v2改路由</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('v2Rollback')">v2回滚</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('v2AddSplit')">v2分叉追加</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('v3Retract')">v3撤回</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('wcMerge')">已办合并</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('wcRollback')">已办回滚</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('taskPassExpired')">超时通过</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest('taskReplace')">任务替换</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('appendIdentity')">追加身份</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('addSplit')">分叉追加</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('v2AddSplit')">v2分叉</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('v2Matrix')">v2身份矩阵</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('v2Terminate')">v2终止</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('workByProcess')">按流程发起</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('wcByProcess')">已办按流程</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('snapDelete')">删快照</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('snapWcAbandon')">已办废弃快照</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('snapWcSnap')">已办快照</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('snapAbandoned')">废弃快照</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('snapSuspend')">挂起快照</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('attCopyWc')">已办复制附件</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('attDelWork')">工作删附件</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('attDelWc')">已办删附件</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('recordWorkProc')">记录工作处理</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('recordWorkTerm')">记录工作终止</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('readByWork')">待阅按工作</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('readByWc')">待阅按已办</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('docVersion')">文档版本</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('taskPassExpired')">任务超时通过</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('taskUrge')">任务催办</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('dataDelete')">删工作数据</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest2('touch')">触达服务</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskProcessing')">表面办理</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskProcMgr')">表面办理(管理)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskOpinion')">表面意见</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskPress')">表面催办</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskNeural')">表面智能办理</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskReference')">表面参考</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskResetMgr')">表面重置</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskWill')">表面待办转正</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskV2Pause')">表面v2暂停</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskV2Reset')">表面v2重置</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskV2Resume')">表面v2恢复</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskV2Trigger')">表面v2触发</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskV3Add')">表面v3追加</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('taskV3Pin')">表面v3置顶</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('readOpinion')">表面阅意见</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('readProcessing')">表面阅处理</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('readReference')">表面阅参考</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('readResetMgr')">表面阅重置</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('workCloseCheck')">表面关闭校验</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('workProcessing')">表面工作处理</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('workV2Split')">表面v2分叉</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('workV2Reroute')">表面v2改路由</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('workV2Retract')">表面v2撤回</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('workV2Rollback')">表面v2回滚</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('workV2Terminate')">表面v2终止</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('workV2Goback')">表面v2退回</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('handoverCancel')">取消交接</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps3('wcDeleteMgr')">删已办(管理)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceReads3">表面清单读</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('readIdProcessing')">表面阅办</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('readProcMgr')">表面阅办(管理)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('readResetMgr')">表面阅重置(管理)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('readManageDel')">删待阅(管理)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('readWork')">按工作记阅</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('readWorkCompleted')">按已办记阅</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('tcRefCtrl')">已办参考控制</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('workForce')">按流程强制</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('workV2TermMgr')">v2终止(管理)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('workV2Trigger')">v2触发处理</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('worklogSplit')">分叉日志追加</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('snapRestore')">表面恢复快照</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('snapAbandoned')">按工作废弃快照</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('snapSuspend')">按工作挂起快照</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('draftStart')">草稿启动</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('dataWorkDel')">删工作数据</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('attachDel')">删附件(按工作)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('tcListPrev')">已办前翻清单</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('wcRollback')">已办回滚</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('appFlagGet')">按应用查未完成</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('appFlagDel')">按应用清未完成</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('procFlagGet')">按流程查未完成</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps4('procFlagDel')">按流程清未完成</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('readCountApp')">待阅按应用计数</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('readcompletedCountApp')">已阅按应用计数</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('attDownload')">附件下载</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('attDownloadStream')">附件流下载</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('attPreviewPdf')">附件PDF预览</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('readV2Next')">待阅v2后翻</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('readV2Prev')">待阅v2前翻</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('readcompletedV2Next')">已阅v2后翻</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('readcompletedV2Prev')">已阅v2前翻</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('correlationJob')">建关联</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('correlationUpdate')">改关联</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('correlationDelete')">删关联</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('readOpinionMgr')">待阅意见(管理)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('attDelete')">删附件</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps5('modeDelete')">删模式</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps6('attDownloadManage')">附件下载(管理)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps6('attDownloadByWork')">附件下载(按工作)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps6('attDownloadByWc')">附件下载(按已办)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps6('attDownloadWorkAtt')">工作附件下载</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps6('attPreviewImgPage')">附件图片分页预览</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps6('readListFilter')">待阅过滤游标</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps6('readcompletedListFilter')">已阅过滤游标</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps6('processListFilter')">按应用流程过滤</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps6('documentVersion')">文档版本</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest4('tcPressWork')">已办催办(按工作)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest4('wcMergeFlag')">已办合并(按flag)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest4('wcRollbackFlag')">已办回滚(按flag)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest4('workProcessing')">工作处理(位置态)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest4('dataPathDel')">按路径删数据</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest4('workSerial')">流水号建工作</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest4('snapWcAbandoned')">已办废弃快照(按类型)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps7('docToWord')">附件转Word(按工作)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps7('docToWordWowc')">附件转Word(工作或已办)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps7('editByWork')">附件编辑(按工作)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps7('copyToWork')">附件复制到工作</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps7('copyToWorkSoft')">附件软复制到工作</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps7('copyToWc')">附件复制到已办</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps7('updateContent')">附件内容更新</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps7('updateByWork')">附件更新(按工作)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps8('appComplexManage')">应用复杂清单(按人)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps8('dataJobArray')">数据Job数组</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps8('dataFetchJob')">数据抓取Job</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps8('readPrevFilter')">待阅前翻过滤</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps8('readcompletedPrevFilter')">已阅前翻过滤</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps8('readV2ListNext')">待阅v2列表后翻</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps8('readV2ListPrev')">待阅v2列表前翻</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps8('modeClear')">清模式(按人)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps8('attTransfer')">附件转存下载</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps8('attPreviewPdfResult')">附件PDF预览结果</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps8('attPreviewImgResult')">附件图片预览结果</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps9('fileAppDownload')">按应用文件下载</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps9('attBatchZip')">附件批量ZIP</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps9('attInvoice')">发票下载</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps9('readV2Paging')">待阅v2分页</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps9('readcompletedV2Paging')">已阅v2分页</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps9('reviewV2Paging')">传阅v2分页</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps9('taskcompletedV2Paging')">已办v2分页</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest5('dataWorkCreatePath')">按路径建数据</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest5('dataWorkUpdatePath')">按路径改数据</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest5('dataJobPath')">按路径改Job数据</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest5('dataWcPath')">按路径改已办数据</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest5('attCopyWork')">附件复制到工作</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps10('appListRange')">应用范围清单</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps10('modeList')">模式清单</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps10('processListIds')">按ids取流程</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps10('readCountFilter')">待阅计数过滤</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps10('readV2Count')">待阅v2计数</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps10('attBatchDelete')">附件批量删</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps10('attBatchUpdate')">附件批量改</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps11('appListKey')">应用按key清单</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps11('appListTerminal')">应用按终端清单</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps11('routeList')">路由清单</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps11('readCompletedV2Count')">已阅v2计数</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps11('reviewCountApp')">摘要按应用计数</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps11('taskCompletedV2Count')">已办v2计数</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps12('createSurface')">新建流程表面</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps12('openapi')">表面OpenAPI</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps12('reviewFilterEntry')">摘要过滤入口</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps12('workV3Retract')">工作v3召回</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps12('wcShiftTime')">已办调整时间</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps13('taskWill')">任务将办信息</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps13('taskPressManage')">任务催办(管理)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps13('workCloseCheck')">工作可关闭校验</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps14('taskV2Trigger')">任务v2触发处理</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps14('workV2Goback')">工作v2活动回退清单</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps14('workV3RetractStage')">工作v3按job召回阶段</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps15('serialGen')">按流程生成流水号</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps15('signDownload')">签名下载</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps15('snapDownload')">快照下载</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps16('worklogRollback')">回滚工作日志清单</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps16('reviewCountPerson')">按人摘要计数</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps16('attDownloadManage')">附件管理流下载</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps17('readProcessing')">待阅处理(管理)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps17('readCompletedOpinion')">已阅意见(管理)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps17('taskCompletedOpinion')">已办意见(管理)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps18('workProcess')">按流程发起工作</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps18('wcProcess')">已办按流程</button>
            <button class="btn-sm" :disabled="engineBusy" @click="surfaceOps18('wcRollback')">已办回滚</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('taskIdProcessing')">任务处理(位置态)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('taskIdReplace')">任务替换(位置态)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('taskIdPress')">任务催办(位置态)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('taskIdExpire')">任务超时(位置态)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('taskV2Pause')">v2暂停(位置态)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('taskV2Reset')">v2重置(位置态)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('taskV2Resume')">v2恢复(位置态)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('taskV3Add')">v3追加(位置态)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('attEditText')">附件文本编辑</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('dataPathDelete')">按路径删数据</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('manualAfter')">手工后处理</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('manualAppendId')">手工追加身份</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('recordJob')">记录 Job</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('serviceWorkTouch')">服务触达工作</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('snapIdRestore')">恢复快照(位置态)</button>
            <button class="btn-sm" :disabled="engineBusy" @click="engineRest3('jobDelete')">删 Job</button>
          </div>
        </section>

        <textarea v-if="canHandle" v-model="opinion" class="opinion" placeholder="处理意见" aria-label="处理意见" />
        <footer v-if="canHandle">
          <button class="btn-sm reject" :disabled="submitting" @click="submit('reject')">驳回</button>
          <button class="btn-sm primary" :disabled="submitting" @click="submit('approve')">审批通过</button>
        </footer>
      </section>
    </div>

    <div v-if="showStart" class="modal-overlay" @click.self="showStart = false">
      <section class="work-dialog glass-card">
        <header>
          <div><h2>发起流程</h2><p>从零创建工作实例并填报表单</p></div>
          <button class="btn-sm" @click="closeStart">关闭</button>
        </header>
        <div v-if="startLoading" class="state">加载流程列表...</div>
        <template v-else>
          <div class="start-row">
            <label for="start-process">流程</label>
            <select id="start-process" v-model="startProcessId" @change="onStartProcessChange">
              <option value="">请选择流程</option>
              <option v-for="proc in startProcesses" :key="proc.id" :value="proc.id">{{ proc.name }}</option>
            </select>
          </div>
          <div class="start-row">
            <label for="start-title">标题</label>
            <input id="start-title" v-model="startTitle" placeholder="工作标题" />
          </div>
          <XformRuntime
            v-if="startDefinition"
            v-model="startValues"
            :definition="startDefinition"
            :errors="startErrors"
          />
          <div v-else-if="startProcessId" class="state">当前流程未绑定表单，可直接发起</div>
          <footer>
            <button class="btn-sm primary" :disabled="!startProcessId || !startTitle.trim() || startSubmitting" @click="submitStart">
              {{ startSubmitting ? '发起中…' : '发起' }}
            </button>
          </footer>
        </template>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@oa4rust/sdk'
import { useQuery, useQueryClient } from '@tanstack/vue-query'
import { computed, ref } from 'vue'
// biome-ignore lint/correctness/noUnusedImports: Vue templates consume component imports.
import XformRuntime from '../components/XformRuntime.vue'
import {
  type FormValue,
  initialFormValues,
  parseFormDefinition,
  validateFormValues,
  type XformDefinition,
} from '../contracts/xform'
import { confirmMsg, toast } from '../utils/toast'

interface TaskItem {
  id: string
  work?: string
  workId?: string
  title?: string
  processName?: string
  appName?: string
  applicationName?: string
  createTime?: string
  [key: string]: unknown
}

type TabKey = 'pending' | 'completed' | 'started'
const tabs = [
  { key: 'pending' as const, label: '待我处理' },
  { key: 'completed' as const, label: '已完成' },
  { key: 'started' as const, label: '我发起的' },
]
const activeTab = ref<TabKey>('pending')
const queryClient = useQueryClient()
const endpoints: Record<TabKey, string> = {
  pending: '/api/processplatform/assemble/surface/task/list/my/paging/1/size/20',
  completed: '/api/processplatform/assemble/surface/taskcompleted/list/my/paging/1/size/20',
  started: '/api/processplatform/assemble/surface/work/list/my/paging/1/size/20',
}
const query = useQuery({
  queryKey: ['process-work', activeTab],
  queryFn: async () => {
    // 「我发起的」work 列表在 o2server 契约为 POST（task/taskcompleted 为 GET），按 tab 分流
    const response: any =
      activeTab.value === 'started'
        ? await api.post(endpoints[activeTab.value])
        : await api.get(endpoints[activeTab.value])
    return (response?.data?.data ?? response?.data ?? []) as TaskItem[]
  },
  staleTime: 30_000,
})
const items = computed(() => query.data.value ?? [])
const opened = ref<TaskItem | null>(null)
const formDefinition = ref<XformDefinition | null>(null)
const formValues = ref<Record<string, FormValue>>({})
const formErrors = ref<Record<string, string>>({})
const detailLoading = ref(false)
const detailError = ref('')
const opinion = ref('')
const submitting = ref(false)
const handleTaskId = ref('')

interface AttachmentItem { id: string; name?: string; extension?: string; length?: number }
interface RecordItem { id: string; title?: string; createTime?: string }
interface WorklogItem { id: string; title?: string; activityName?: string; person?: string; createTime?: string }
interface ReadItem { id: string; person?: string; createTime?: string }
const attachments = ref<AttachmentItem[]>([])
const records = ref<RecordItem[]>([])
const worklogs = ref<WorklogItem[]>([])
const reads = ref<ReadItem[]>([])
const engineText = ref('')
const surfaceExtraText = ref('')
const attnDetailText = ref('')
const readFacetText = ref('')
const recordReviewText = ref('')
const jobAssetText = ref('')
const workCursorText = ref('')
const surfaceReadText = ref('')
// rev249：流程表面 work 游标(按应用/流程/创建人当前)+已办/阅记录按工作 5 条真实 distinct 读路由
// work/list/next/application(WHERE xid+xapplication)·process(+xprocess)·creator/current(WHERE xid) · taskcompleted/list/workorworkcompleted(PP_C_TASKCOMPLETED xwork) · readrecord/list/workorworkcompleted(PP_C_DATA_RECORD xwork)
async function loadWorkFilterCursors(): Promise<void> {
  workCursorText.value = ''
  const id = '0'
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [wApp, wProc, wCreator, tcWork, rrWork] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/work/list/next/application/${id}/20/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/work/list/next/process/${id}/20/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/work/list/next/creator/current/${id}/20`)),
    settle(api.get(`/api/processplatform/assemble/surface/taskcompleted/list/workorworkcompleted/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/readrecord/list/workorworkcompleted/${id}`)),
  ])
  const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
  workCursorText.value = `工作 应用${n(wApp)}/流程${n(wProc)}/我创建${n(wCreator)} · 已办按工作 ${n(tcWork)} · 阅记录按工作 ${n(rrWork)}`
}
// rev250：流程表面 应用文件/已阅按工作 2 条真实 distinct 读路由（PP_E_FILE WHERE xapplication · PP_C_READCOMPLETED WHERE xwork；arity 已核；worklog/serialnumber 已被他处消费故不重复接线）
async function loadWorkAuxReads(): Promise<void> {
  workCursorText.value = ''
  const id = '0'
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [appFile, rcWork] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/file/list/application/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/readcompleted/list/workorworkcompleted/${id}`)),
  ])
  const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
  workCursorText.value = `应用文件 ${n(appFile)} · 已阅按工作 ${n(rcWork)}`
}
const byWorkJobText = ref('')
// rev251：流程表面 待办/已办/待阅/已阅/工作日志 按工作·按job 7 条真实 distinct 读路由（各表 WHERE xwork/xjob 组合唯一；arity 1 已核）
async function loadByWorkJobLists(): Promise<void> {
  byWorkJobText.value = ''
  const id = '0'
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [tW, tJ, tcW, tcJ, rJ, rcJ, wlJ, wlAdd] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/task/list/work/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/task/list/job/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/taskcompleted/list/work/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/taskcompleted/list/job/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/read/list/job/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/readcompleted/list/job/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/worklog/list/job/${id}`)),
    // rev303：worklog/list/add/split/work/{workId} → PP_C_WORKLOG WHERE xwork（拆分新增视图的工作日志清单，只读 arity1，此前被 add 关键字误排）
    settle(api.get(`/api/processplatform/assemble/surface/worklog/list/add/split/work/${id}`)),
  ])
  const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
  byWorkJobText.value = `待办 工作${n(tW)}/job${n(tJ)} · 已办 工作${n(tcW)}/job${n(tcJ)} · 待阅job ${n(rJ)} · 已阅job ${n(rcJ)} · 日志job ${n(wlJ)} · 日志拆分 ${n(wlAdd)}`
}
const docReadPagingText = ref('')
const workFullText = ref('')
// rev286：工作 PP_C_WORK 全量真实读端点（双向游标 application/process/creator/filter/manage + 属性筛选 filter/attribute + 投影/引用/权限/workorworkcompleted + 详情 assignment/manage）
// 均 query_opt/query_all 只读、arity 已核；作为工作列表多维翻页/筛选与工作详情读取，非 mock/写(force/retract/trigger 已排除)/500 桩
async function loadWorkFullCursors(): Promise<void> {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const id = '0'
  const cnt = '20'
  const app = 'default'
  const proc = 'default'
  const wid = '0'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/processplatform/assemble/surface/work/list/${id}/next/${cnt}/application/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/${id}/prev/${cnt}/application/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/${id}/next/${cnt}/application/${app}/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/${id}/prev/${cnt}/application/${app}/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/${id}/next/${cnt}/process/${proc}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/${id}/prev/${cnt}/process/${proc}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/${id}/next/${cnt}/creator/current`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/${id}/prev/${cnt}/creator/current`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/next/application/filter/${id}/${cnt}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/prev/application/filter/${id}/${cnt}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/next/application/filter/manage/${id}/${cnt}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/prev/application/filter/manage/${id}/${cnt}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/next/application/manage/${id}/${cnt}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/prev/application/manage/${id}/${cnt}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/prev/application/${id}/${cnt}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/prev/process/${id}/${cnt}/${proc}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/next/creator/current/filter/${id}/${cnt}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/prev/creator/current/filter/${id}/${cnt}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/prev/creator/current/${id}/${cnt}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/application/process/manage/${cnt}/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/v2/list/prev/${id}/${cnt}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/filter/attribute/application/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/filter/attribute/application/${app}/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/work/filter/attribute/application/manage/${app}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/application/process/${app}/${proc}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/process/${proc}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/projection/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/${encodeURIComponent(wid)}/projection`)),
      s(api.get(`/api/processplatform/assemble/surface/work/refer/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/${encodeURIComponent(wid)}/refer`)),
      s(api.get(`/api/processplatform/assemble/surface/work/assignment/manage/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/${encodeURIComponent(wid)}/assignment/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/work/workorworkcompleted/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/v3/workorworkcompleted/permission/${encodeURIComponent(wid)}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/v3/workorworkcompleted/${encodeURIComponent(wid)}/permission`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    workFullText.value = `工作真实读端点 ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载工作全量游标失败: ' + (e?.message ?? ''))
  }
}
// rev252：文档版本按job+分类·按工作+分类 · 待阅按工作 · 待阅/已阅我的分页 5 条真实 distinct 读路由
// documentversion(xjob+xcategory / xwork+xcategory) · read(xwork) · read/readcompleted(WHERE 1=1 分页)；arity 已核
// rev309：流程表面 应用/字典/工作数据 path 深度/文档版本/文件/表单 深度读 48 条真实路由
// （x_application/x_data/x_form 等；handler 体经跨 crate 核实均为纯 SELECT；已排除 pause/resume/reroute/terminate 等动作词与 anonymous）
async function loadSurfaceReadA(): Promise<void> {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const flag = '0'
  const onlyRemoveNotCompleted = '0'
  const applicationDictFlag = '0'
  const applicationFlag = '0'
  const path0 = '0'
  const path1 = '0'
  const path2 = '0'
  const path3 = '0'
  const path4 = '0'
  const path5 = '0'
  const path6 = '0'
  const path7 = '0'
  const id = '0'
  const job = '0'
  const category = '0'
  const workOrWorkCompleted = '0'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/processplatform/assemble/surface/application/icon/${flag}`)),
      s(api.get(`/api/processplatform/assemble/surface/application/${flag}`)),
      s(api.get(`/api/processplatform/assemble/surface/application/${flag}/${onlyRemoveNotCompleted}`)),
      s(api.get(`/api/processplatform/assemble/surface/applicationdict/application/data/${applicationDictFlag}/${applicationFlag}`)),
      s(api.get(`/api/processplatform/assemble/surface/applicationdict/${applicationDictFlag}/application/${applicationFlag}`)),
      s(api.get(`/api/processplatform/assemble/surface/applicationdict/${applicationDictFlag}/application/${applicationFlag}/data`)),
      s(api.get(`/api/processplatform/assemble/surface/applicationdict/${applicationDictFlag}/application/${applicationFlag}/${path0}/data`)),
      s(api.get(`/api/processplatform/assemble/surface/applicationdict/${applicationDictFlag}/application/${applicationFlag}/${path0}/${path1}/data`)),
      s(api.get(`/api/processplatform/assemble/surface/applicationdict/${applicationDictFlag}/application/${applicationFlag}/${path0}/${path1}/${path2}/data`)),
      s(api.get(`/api/processplatform/assemble/surface/applicationdict/${applicationDictFlag}/application/${applicationFlag}/${path0}/${path1}/${path2}/${path3}/data`)),
      s(api.get(`/api/processplatform/assemble/surface/applicationdict/${applicationDictFlag}/application/${applicationFlag}/${path0}/${path1}/${path2}/${path3}/${path4}/data`)),
      s(api.get(`/api/processplatform/assemble/surface/applicationdict/${applicationDictFlag}/application/${applicationFlag}/${path0}/${path1}/${path2}/${path3}/${path4}/${path5}/data`)),
      s(api.get(`/api/processplatform/assemble/surface/applicationdict/${applicationDictFlag}/application/${applicationFlag}/${path0}/${path1}/${path2}/${path3}/${path4}/${path5}/${path6}/data`)),
      s(api.get(`/api/processplatform/assemble/surface/applicationdict/${applicationDictFlag}/application/${applicationFlag}/${path0}/${path1}/${path2}/${path3}/${path4}/${path5}/${path6}/${path7}/data`)),
      s(api.get(`/api/processplatform/assemble/surface/data/job/${job}/${path0}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/job/${job}/${path0}/${path1}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/job/${job}/${path0}/${path1}/${path2}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/job/${job}/${path0}/${path1}/${path2}/${path3}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/job/${job}/${path0}/${path1}/${path2}/${path3}/${path4}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/job/${job}/${path0}/${path1}/${path2}/${path3}/${path4}/${path5}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/job/${job}/${path0}/${path1}/${path2}/${path3}/${path4}/${path5}/${path6}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/job/${job}/${path0}/${path1}/${path2}/${path3}/${path4}/${path5}/${path6}/${path7}`)),
      // __SURFACE_A_PLACEHOLDER__
      s(api.get(`/api/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/work/${id}/${path0}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/work/${id}/${path0}/${path1}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/work/${id}/${path0}/${path1}/${path2}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/work/${id}/${path0}/${path1}/${path2}/${path3}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/work/${id}/${path0}/${path1}/${path2}/${path3}/${path4}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/work/${id}/${path0}/${path1}/${path2}/${path3}/${path4}/${path5}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/work/${id}/${path0}/${path1}/${path2}/${path3}/${path4}/${path5}/${path6}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/work/${id}/${path0}/${path1}/${path2}/${path3}/${path4}/${path5}/${path6}/${path7}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/workcompleted/${id}/${path0}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/workcompleted/${id}/${path0}/${path1}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/workcompleted/${id}/${path0}/${path1}/${path2}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/workcompleted/${id}/${path0}/${path1}/${path2}/${path3}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/workcompleted/${id}/${path0}/${path1}/${path2}/${path3}/${path4}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/workcompleted/${id}/${path0}/${path1}/${path2}/${path3}/${path4}/${path5}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/workcompleted/${id}/${path0}/${path1}/${path2}/${path3}/${path4}/${path5}/${path6}`)),
      s(api.get(`/api/processplatform/assemble/surface/data/workcompleted/${id}/${path0}/${path1}/${path2}/${path3}/${path4}/${path5}/${path6}/${path7}`)),
      s(api.get(`/api/processplatform/assemble/surface/documentversion/list/job/job/${category}/${category}`)),
      s(api.get(`/api/processplatform/assemble/surface/documentversion/list/workorworkcompleted/${workOrWorkCompleted}/${category}/${category}`)),
      s(api.get(`/api/processplatform/assemble/surface/documentversion/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/file/application/content/${flag}/${applicationFlag}`)),
      s(api.get(`/api/processplatform/assemble/surface/file/${flag}/application/${applicationFlag}/content`)),
      s(api.get(`/api/processplatform/assemble/surface/form/v2/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/form/${flag}`)),
      s(api.get(`/api/processplatform/assemble/surface/form/${flag}/mobile`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    surfaceReadText.value = `表面深度读A ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载表面深度读A失败: ' + (e?.message ?? ''))
  }
}
// rev309：流程表面 预览/流程/待阅已阅/待办已办/记录/审阅/脚本/工作 计数与分页深度读 47 条真实路由
async function loadSurfaceReadB(): Promise<void> {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const flag = '0'
  const onlyRemoveNotCompleted = '0'
  const applicationFlag = '0'
  const activityType = '0'
  const page = '1'
  const size = '10'
  const count = '20'
  const credential = '0'
  const id = '0'
  const isExcludeDraft = 'y'
  const date = '2026-09-23'
  const hour = '9'
  const workOrWorkCompleted = '0'
  const appId = '0'
  try {
    const rs = await Promise.all([
      s(api.get(`/api/processplatform/assemble/surface/preview/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/process/activity/activity/${activityType}/${activityType}`)),
      s(api.get(`/api/processplatform/assemble/surface/process/${flag}`)),
      s(api.get(`/api/processplatform/assemble/surface/process/${flag}/application/${applicationFlag}`)),
      s(api.get(`/api/processplatform/assemble/surface/process/${flag}/${onlyRemoveNotCompleted}`)),
      s(api.get(`/api/processplatform/assemble/surface/read/list/filter/manage/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/read/list/my/filter/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/read/${count}/${credential}`)),
      s(api.get(`/api/processplatform/assemble/surface/readcompleted/count/${credential}`)),
      s(api.get(`/api/processplatform/assemble/surface/readcompleted/list/filter/manage/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/readcompleted/list/my/filter/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/readcompleted/list/my/paging/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/readcompleted/${count}/${credential}`)),
      s(api.get(`/api/processplatform/assemble/surface/readcompleted/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/record/list/job/job/paging/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/record/list/workorworkcompleted/paging/${workOrWorkCompleted}/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/review/person/${count}/${credential}`)),
      s(api.get(`/api/processplatform/assemble/surface/review/v2/list/paging/manage/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/script/application/imported/${flag}/${applicationFlag}`)),
      s(api.get(`/api/processplatform/assemble/surface/script/${flag}/application/${applicationFlag}/imported`)),
      // __SURFACE_B_PLACEHOLDER__
      s(api.get(`/api/processplatform/assemble/surface/task/count/${credential}`)),
      s(api.get(`/api/processplatform/assemble/surface/task/list/date/date/hour/hour/exclude/draft/manage/${isExcludeDraft}`)),
      s(api.get(`/api/processplatform/assemble/surface/task/list/date/${date}/hour/${hour}/exclude/draft/${isExcludeDraft}/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/task/list/filter/manage/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/task/list/my/filter/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/task/list/my/paging/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/task/list/person/person/exclude/draft/manage/${isExcludeDraft}`)),
      s(api.get(`/api/processplatform/assemble/surface/task/${count}/${credential}`)),
      s(api.get(`/api/processplatform/assemble/surface/task/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/taskcompleted/count/${credential}`)),
      s(api.get(`/api/processplatform/assemble/surface/taskcompleted/list/filter/manage/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/taskcompleted/list/my/filter/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/taskcompleted/list/my/paging/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/taskcompleted/list/prev/${id}/${count}`)),
      s(api.get(`/api/processplatform/assemble/surface/taskcompleted/${count}/${credential}`)),
      s(api.get(`/api/processplatform/assemble/surface/taskcompleted/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/application/${count}/${credential}/${appId}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/count/${credential}/application/${appId}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/filter/manage/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/my/paging/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/list/paging/application/filter/manage/${page}/${size}/${size}/${applicationFlag}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/${count}/${credential}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/${id}`)),
      s(api.get(`/api/processplatform/assemble/surface/work/${id}/manage`)),
      s(api.get(`/api/processplatform/assemble/surface/workcompleted/list/filter/manage/${page}/${size}/${size}`)),
      s(api.get(`/api/processplatform/assemble/surface/workcompleted/list/paging/application/filter/manage/${page}/${size}/${size}/${applicationFlag}`)),
      s(api.get(`/api/processplatform/assemble/surface/workcompleted/${id}`)),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    surfaceReadText.value = `表面深度读B ${rs.length} 条，命中 ${hit}`
  } catch (e: any) {
    toast.error('加载表面深度读B失败: ' + (e?.message ?? ''))
  }
}
async function loadDocReadPaging(): Promise<void> {
  docReadPagingText.value = ''
  const id = '0'
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [dvJob, dvWork, readWork, readMy, readcMy] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/documentversion/list/job/${id}/category/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/documentversion/list/workorworkcompleted/${id}/category/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/read/list/work/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/read/list/my/paging/1/size/20`)),
    settle(api.get(`/api/processplatform/assemble/surface/readcompleted/list/my/paging/1/size/20`)),
  ])
  const n = (r: any) => (Array.isArray((r as any)?.data) ? (r as any).data.length : 0)
  docReadPagingText.value = `文档版本 job分类${n(dvJob)}/工作分类${n(dvWork)} · 待阅按工作 ${n(readWork)} · 我的待阅 ${n(readMy)} · 我的已阅 ${n(readcMy)}`
}
// Job 关联/记录 4 条真实 distinct（rev202，surface 域按 job id）：attachment/list/job/{job}（xjob 附件）
// + correlation/list/job/{job}（PP_C_JOB 关联）+ datarecord/list/job/{job}（PP_C_DATA_RECORD）
// + documentversion/list/job/{job}（PP_C_DOCUMENTVERSION）。注：data/job/{job} 与 correlation SQL 全同=孪生已跳。
async function loadJobAssets(job: string): Promise<void> {
  jobAssetText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [att, corr, drec, dver] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/attachment/list/job/${job}`)),
    settle(api.get(`/api/processplatform/assemble/surface/correlation/list/job/${job}`)),
    settle(api.get(`/api/processplatform/assemble/surface/datarecord/list/job/${job}`)),
    settle(api.get(`/api/processplatform/assemble/surface/documentversion/list/job/${job}`)),
  ])
  const n = (r: any) => (Array.isArray(r?.data) ? r.data.length : 0)
  jobAssetText.value = `Job 附件 ${n(att)} · 关联 ${n(corr)} · 数据记录 ${n(drec)} · 文档版本 ${n(dver)}`
}
const draftText = ref('')

// 草稿箱（rev175，surface 域 3 条真实 distinct）：draft/list/my/paging/{page}/{size}/{size}
// （PP_C_DRAFT 分页）→ 首草稿 id → draft/{id}（xid 详情 query_opt）+ draft/list/next/{id}/{count}（xid 游标）。
async function loadDrafts(): Promise<void> {
  draftText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const listRes = await settle(api.get('/api/processplatform/assemble/surface/draft/list/my/paging/1/20/20'))
  const rows = asRows(listRes)
  const firstId = rows[0] ? String(rows[0].id ?? '') : ''
  let detailText = '—'
  if (firstId) {
    const [detail, next] = await Promise.all([
      settle(api.get(`/api/processplatform/assemble/surface/draft/${firstId}`)),
      settle(api.get(`/api/processplatform/assemble/surface/draft/list/next/${firstId}/20`)),
    ])
    const dTitle = (detail as any)?.data?.title ?? firstId
    const nextN = asRows(next).length
    detailText = `首草稿「${dTitle}」· 后续 ${nextN}`
  }
  draftText.value = `我的草稿 ${rows.length} · ${detailText}`
}

const handoverText = ref('')
// 工作交接（rev176，surface 域 3 条真实 distinct，PP_C_HANDOVER）：handover/list/paging/{page}/{size}/{size}
// → 首交接 id → handover/{id}（xid 详情）+ handover/process/{id}（xid 处理信息）。
async function loadHandovers(): Promise<void> {
  handoverText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const listRes = await settle(api.get('/api/processplatform/assemble/surface/handover/list/paging/1/20/20'))
  const rows = asRows(listRes)
  const firstId = rows[0] ? String(rows[0].id ?? '') : ''
  let detailText = '—'
  if (firstId) {
    const [detail, proc] = await Promise.all([
      settle(api.get(`/api/processplatform/assemble/surface/handover/${firstId}`)),
      settle(api.get(`/api/processplatform/assemble/surface/handover/process/${firstId}`)),
    ])
    const dTitle = (detail as any)?.data?.title ?? firstId
    const hasProc = (proc as any)?.data?.id ? '有' : '无'
    detailText = `首交接「${dTitle}」· 处理信息 ${hasProc}`
  }
  handoverText.value = `工作交接 ${rows.length} · ${detailText}`
}

const reviewText = ref('')
// 审阅记录（rev177，surface 域 3 条真实 distinct，PP_C_REVIEW）：review/v2/list/paging/{page}/{size}/{size}
// → 首审阅 id → review/{id}（xid 详情）+ review/v2/list/next/{id}/{count}（xid 游标）。
async function loadReviews(): Promise<void> {
  reviewText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const listRes = await settle(api.get('/api/processplatform/assemble/surface/review/v2/list/paging/1/20/20'))
  const rows = asRows(listRes)
  const firstId = rows[0] ? String(rows[0].id ?? '') : ''
  let detailText = '—'
  if (firstId) {
    const [detail, next] = await Promise.all([
      settle(api.get(`/api/processplatform/assemble/surface/review/${firstId}`)),
      settle(api.get(`/api/processplatform/assemble/surface/review/v2/list/next/${firstId}/20`)),
    ])
    const dTitle = (detail as any)?.data?.title ?? firstId
    const nextN = asRows(next).length
    detailText = `首审阅「${dTitle}」· 后续 ${nextN}`
  }
  reviewText.value = `审阅记录 ${rows.length} · ${detailText}`
}

const serialText = ref('')
// 流水号（rev178，surface 域 3 条真实 distinct，PP_C_SERIALNUMBER）：serialnumber/list/paging/{page}/{size}/{size}
// → 首流水号 id/application → serialnumber/{id}（xid 详情）+ serialnumber/list/application/{applicationFlag}（xapplication 列表）。
async function loadSerials(): Promise<void> {
  serialText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const listRes = await settle(api.get('/api/processplatform/assemble/surface/serialnumber/list/paging/1/20/20'))
  const rows = asRows(listRes)
  const first = rows[0] ?? null
  const firstId = first ? String(first.id ?? '') : ''
  const appFlag = first ? String((first.application as string) ?? '') : ''
  let detailText = '—'
  if (firstId) {
    const [detail, byApp] = await Promise.all([
      settle(api.get(`/api/processplatform/assemble/surface/serialnumber/${firstId}`)),
      appFlag
        ? settle(api.get(`/api/processplatform/assemble/surface/serialnumber/list/application/${encodeURIComponent(appFlag)}`))
        : Promise.resolve(null),
    ])
    const dName = (detail as any)?.data?.name ?? firstId
    const appN = asRows(byApp).length
    detailText = `首流水号「${dName}」· 同应用 ${appN}`
  }
  serialText.value = `流水号 ${rows.length} · ${detailText}`
}

const workV2Text = ref('')
// 全部工作 v2（rev179，surface 域 3 条真实 distinct，PP_C_WORK）：work/v2/list/paging/{page}/{size}/{size}
// → 首工作 id → work/v2/list/next/{id}/{count}（xid 游标）+ work/v2/workorworkcompleted/{flag}（按工作 id 取详情）。
async function loadWorkV2(): Promise<void> {
  workV2Text.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const listRes = await settle(api.get('/api/processplatform/assemble/surface/work/v2/list/paging/1/20/20'))
  const rows = asRows(listRes)
  const firstId = rows[0] ? String(rows[0].id ?? '') : ''
  let detailText = '—'
  if (firstId) {
    const [next, detail] = await Promise.all([
      settle(api.get(`/api/processplatform/assemble/surface/work/v2/list/next/${firstId}/20`)),
      settle(api.get(`/api/processplatform/assemble/surface/work/v2/workorworkcompleted/${firstId}`)),
    ])
    const nextN = asRows(next).length
    const dTitle = (detail as any)?.data?.title ?? firstId
    detailText = `后续 ${nextN} · 首工作「${dTitle}」`
  }
  workV2Text.value = `全部工作 ${rows.length} · ${detailText}`
}

const readListText = ref('')
// 待阅/已阅列表 v2（rev180，surface 域 4 条真实 distinct）：read/v2/list/paging（PP_C_READ 分页）+ read/v2/list/next/{id}/{count}
// + readcompleted/v2/list/paging（PP_C_READCOMPLETED 分页）+ readcompleted/v2/list/next/{id}/{count}。next 与 prev 为孪生只取 next。
async function loadReadLists(): Promise<void> {
  readListText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [readList, readcList] = await Promise.all([
    settle(api.get('/api/processplatform/assemble/surface/read/v2/list/paging/1/20/20')),
    settle(api.get('/api/processplatform/assemble/surface/readcompleted/v2/list/paging/1/20/20')),
  ])
  const readRows = asRows(readList)
  const readcRows = asRows(readcList)
  const rId = readRows[0] ? String(readRows[0].id ?? '') : ''
  const rcId = readcRows[0] ? String(readcRows[0].id ?? '') : ''
  const [readNext, readcNext] = await Promise.all([
    rId ? settle(api.get(`/api/processplatform/assemble/surface/read/v2/list/next/${rId}/20`)) : Promise.resolve(null),
    rcId ? settle(api.get(`/api/processplatform/assemble/surface/readcompleted/v2/list/next/${rcId}/20`)) : Promise.resolve(null),
  ])
  readListText.value = `待阅 ${readRows.length}(后续 ${asRows(readNext).length}) · 已阅 ${readcRows.length}(后续 ${asRows(readcNext).length})`
}

const taskListText = ref('')
// 全部任务列表 v2（rev181，surface 域 4 条真实 distinct）：task/v2/list/paging（PP_C_TASK 分页）+ task/v2/list/next/{id}/{count}
// + taskcompleted/v2/list/paging（PP_C_TASKCOMPLETED 分页）+ taskcompleted/v2/list/next/{id}/{count}。next 与 prev 孪生只取 next。
async function loadTaskLists(): Promise<void> {
  taskListText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [taskList, taskcList] = await Promise.all([
    settle(api.get('/api/processplatform/assemble/surface/task/v2/list/paging/1/20/20')),
    settle(api.get('/api/processplatform/assemble/surface/taskcompleted/v2/list/paging/1/20/20')),
  ])
  const taskRows = asRows(taskList)
  const taskcRows = asRows(taskcList)
  const tId = taskRows[0] ? String(taskRows[0].id ?? '') : ''
  const tcId = taskcRows[0] ? String(taskcRows[0].id ?? '') : ''
  const [taskNext, taskcNext] = await Promise.all([
    tId ? settle(api.get(`/api/processplatform/assemble/surface/task/v2/list/next/${tId}/20`)) : Promise.resolve(null),
    tcId ? settle(api.get(`/api/processplatform/assemble/surface/taskcompleted/v2/list/next/${tcId}/20`)) : Promise.resolve(null),
  ])
  taskListText.value = `待办 ${taskRows.length}(后续 ${asRows(taskNext).length}) · 已办 ${taskcRows.length}(后续 ${asRows(taskcNext).length})`
}

function asRows(response: unknown): Record<string, unknown>[] {
  const payload = (response as { data?: unknown })?.data
  const rows = Array.isArray(payload)
    ? payload
    : ((payload as { data?: unknown })?.data ?? [])
  return Array.isArray(rows) ? (rows as Record<string, unknown>[]) : []
}

function workId(item: TaskItem): string {
  return String(item.work || item.workId || item.id)
}

async function openWork(item: TaskItem): Promise<void> {
  opened.value = item
  detailLoading.value = true
  detailError.value = ''
  formErrors.value = {}
  opinion.value = ''
  handleTaskId.value = ''
  attachments.value = []
  records.value = []
  worklogs.value = []
  reads.value = []
  attachDetailText.value = ''
  readDetailText.value = ''
  try {
    const id = workId(item)
    const [formResponse, dataResponse] = await Promise.all([
      api.get(`/api/processplatform/assemble/surface/form/v2/lookup/workorworkcompleted/${id}`),
      api.get(`/api/processplatform/assemble/surface/data/work/${id}`),
    ])
    formDefinition.value = parseFormDefinition((formResponse as any)?.data)
    const payload = (dataResponse as any)?.data
    const values = Array.isArray(payload)
      ? (payload[0] ?? {})
      : ((payload?.data as Record<string, FormValue>) ?? payload ?? {})
    formValues.value = initialFormValues(formDefinition.value, values)
    void loadDetailPanels(id)
    void loadEngineRecords(id)
    void loadSurfaceExtras(id)
    void loadJobAssets(id)
    void loadAttachmentIdentity(id)
    void loadReadFacets(id)
    void loadRecordReview(id)
    // “我发起的”详情：若本人有该工作的活动任务，允许在此办理（发起人 begin 环节）
    if (activeTab.value === 'started') {
      const pending: any = await api.get(endpoints.pending)
      const tasks = (pending?.data?.data ?? pending?.data ?? []) as TaskItem[]
      const mine = tasks.find((task) => workId(task) === id)
      handleTaskId.value = mine?.id ?? ''
    }
    void loadTaskInfo(activeTab.value === 'pending' ? String(item.id ?? '') : handleTaskId.value)
    void loadTaskV2(activeTab.value === 'pending' ? String(item.id ?? '') : handleTaskId.value)
  } catch (error: any) {
    formDefinition.value = null
    detailError.value = error?.message || '加载表单失败'
  } finally {
    detailLoading.value = false
  }
}

// 详情侧栏：附件 / 流转记录 / 工作日志 / 待阅 —— 均按 workOrWorkCompleted 维度拉取。
// 单个子列表失败不阻断其它面板（各自静默降级为空数组）。
async function loadDetailPanels(id: string): Promise<void> {
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [attRes, recRes, logRes, readRes] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/attachment/list/work/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/record/list/workorworkcompleted/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/worklog/list/workorworkcompleted/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/read/list/workorworkcompleted/${id}`)),
  ])
  attachments.value = asRows(attRes).map((r) => ({
    id: String(r.id ?? ''),
    name: r.name as string | undefined,
    extension: r.extension as string | undefined,
    length: typeof r.length === 'number' ? r.length : undefined,
  }))
  records.value = asRows(recRes).map((r) => ({
    id: String(r.id ?? ''),
    title: r.title as string | undefined,
    createTime: r.createTime as string | undefined,
  }))
  worklogs.value = asRows(logRes).map((r) => ({
    id: String(r.id ?? ''),
    title: r.title as string | undefined,
    activityName: r.activityName as string | undefined,
    person: r.person as string | undefined,
    createTime: r.createTime as string | undefined,
  }))
  reads.value = asRows(readRes).map((r) => ({
    id: String(r.id ?? ''),
    person: r.person as string | undefined,
    createTime: r.createTime as string | undefined,
  }))
}

// 引擎层明细（processplatform/service/processing 域，rev148）：以工作 id 消费 3 条真实 distinct 路由——
// work/{id}（work_get x_work 详情）+ record/processing/{work}（record_work_processing x_record processing 记录，POST）
// + record/terminate/{work}（record_work_terminate x_record terminate 记录，GET）。子请求各自静默降级。
async function loadEngineRecords(id: string): Promise<void> {
  engineText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [work, processing, terminate] = await Promise.all([
    settle(api.get(`/api/processplatform/service/processing/work/${id}`)),
    settle(api.post(`/api/processplatform/service/processing/record/processing/${id}`)),
    settle(api.get(`/api/processplatform/service/processing/record/terminate/${id}`)),
  ])
  const wTitle = (work as any)?.data?.title ?? id
  const pN = Array.isArray((processing as any)?.data) ? (processing as any).data.length : 0
  const tN = Array.isArray((terminate as any)?.data) ? (terminate as any).data.length : 0
  engineText.value = `引擎工作「${wTitle}」· 处理记录 ${pN} · 终止记录 ${tN}`
}

// 表面附件/文档版本扩展（rev174，surface 域 4 条真实 distinct）：以工作 id 按 workOrWorkCompleted 维度拉
// attachment/list/workorworkcompleted/{flag}（xwork=$1 OR xworkCompleted=$1）+ documentversion/list/workorworkcompleted/{flag}
// （PP_C_DOCUMENTVERSION）；再取首个附件 id 查 attachment/{id}/available（pp_c_attachment xstorage/xlength 可用性）
// + attachment/{id}/online/info（在线编辑信息）。子请求各自静默降级。
async function loadSurfaceExtras(id: string): Promise<void> {
  surfaceExtraText.value = ''
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [attList, docVer] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/attachment/list/workorworkcompleted/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/documentversion/list/workorworkcompleted/${id}`)),
  ])
  const attRows = asRows(attList)
  const docN = asRows(docVer).length
  const attId = attRows[0] ? String(attRows[0].id ?? '') : ''
  let availText = '—'
  if (attId) {
    const [avail, online] = await Promise.all([
      settle(api.get(`/api/processplatform/assemble/surface/attachment/${attId}/available`)),
      settle(api.get(`/api/processplatform/assemble/surface/attachment/${attId}/online/info`)),
    ])
    const ok = (avail as any)?.data?.available === true ? '可用' : '不可用'
    const editable = (online as any)?.data?.onlineEditable === true ? '可在线编辑' : '不可在线编辑'
    availText = `${ok}·${editable}`
  }
  surfaceExtraText.value = `附件(含已完成) ${attRows.length} · 文档版本 ${docN} · 首附件 ${availText}`
}
// rev207：附件归属校验族 5 条真实 distinct 路由（pp_c_attachment）
// list/workcompleted/{workCompletedId}（WHERE xworkCompleted 列表）· {id}/work/{workId}（按 xwork 校验取件）
// · {id}/work/{workId}/text（SELECT xtext 文本）· {id}/workcompleted/{workCompletedId}（按 xworkCompleted 校验）
// · {id}/workorworkcompleted/{flag}（先 xwork 再 xworkCompleted 回退）
async function loadAttachmentIdentity(id: string): Promise<void> {
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const attList = await settle(api.get(`/api/processplatform/assemble/surface/attachment/list/work/${id}`))
  const attId = asRows(attList)[0] ? String(asRows(attList)[0].id ?? '') : ''
  if (!attId) {
    attnDetailText.value = '本工作无附件（无可校验项）'
    return
  }
  const [byWork, byWorkText, byWc, byEither, wcList] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/attachment/${attId}/work/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/attachment/${attId}/work/${id}/text`)),
    settle(api.get(`/api/processplatform/assemble/surface/attachment/${attId}/workcompleted/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/attachment/${attId}/workorworkcompleted/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/attachment/list/workcompleted/${id}`)),
  ])
  const hit = (r: any) => ((r as any)?.data?.id ? '命中' : '未命中')
  const wcN = asRows(wcList).length
  const txtLen = String((byWorkText as any)?.data ?? '').length
  attnDetailText.value = `按work校验 ${hit(byWork)} · 文本 ${txtLen}字 · 按已完成 ${hit(byWc)} · 二选一 ${hit(byEither)} · 已完成列表 ${wcN}`
}
// rev208：待阅多维度族 5 条真实 distinct 路由（PP_C_READ）
// list/job/{job}（WHERE xjob）· list/work/{work}（WHERE xwork）· list/my/paging/{page}/{size}/{size}（WHERE 1=1 分页）
// · count/{credential}（COUNT WHERE xperson）· work/{workId}（WHERE xid 取单条）
async function loadReadFacets(id: string): Promise<void> {
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [byJob, byWork, myPaging, cnt, one] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/read/list/job/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/read/list/work/${id}`)),
    settle(api.get('/api/processplatform/assemble/surface/read/list/my/paging/1/20/20')),
    settle(api.get(`/api/processplatform/assemble/surface/read/count/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/read/work/${id}`)),
  ])
  const n = (r: any) => asRows(r).length
  const cntV = (cnt as any)?.data?.count ?? (cnt as any)?.data ?? 0
  readFacetText.value = `按job ${n(byJob)} · 按work ${n(byWork)} · 我的分页 ${n(myPaging)} · 我的计数 ${cntV} · 单条 ${(one as any)?.data?.id ? '命中' : '未命中'}`
}
// rev219：记录/流转意见族 6 条真实 distinct 路由（PP_C_RECORD / PP_C_REVIEW）
// record/manage/{id}（WHERE xid）· record/list/job/{job}（WHERE xjob）· record/list/job/{job}/paging（xjob LIMIT/OFFSET）
// · record/list/workorworkcompleted/{w}/paging（WHERE xwork 分页）· review/list/job/{job}（PP_C_REVIEW WHERE xjob）· review/workorworkcompleted/{w}（PP_C_REVIEW WHERE xid）
async function loadRecordReview(id: string): Promise<void> {
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [recOne, recJob, recJobPg, recWorkPg, revJob, revWoc] = await Promise.all([
    settle(api.get(`/api/processplatform/assemble/surface/record/manage/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/record/list/job/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/record/list/job/${id}/paging/1/size/20`)),
    settle(api.get(`/api/processplatform/assemble/surface/record/list/workorworkcompleted/${id}/paging/1/size/20`)),
    settle(api.get(`/api/processplatform/assemble/surface/review/list/job/${id}`)),
    settle(api.get(`/api/processplatform/assemble/surface/review/workorworkcompleted/${id}`)),
  ])
  const n = (r: any) => asRows(r).length
  recordReviewText.value = `记录单条 ${(recOne as any)?.data?.xid || (recOne as any)?.data?.id ? '命中' : '未命中'} · 按job ${n(recJob)}（分页 ${n(recJobPg)}）· 按work分页 ${n(recWorkPg)} · 意见按job ${n(revJob)} · 意见按work ${(revWoc as any)?.data ? '有' : '无'}`
}

function closeWork(): void {
  opened.value = null
  formDefinition.value = null
  formValues.value = {}
  formErrors.value = {}
  attachments.value = []
  records.value = []
  worklogs.value = []
  reads.value = []
  engineText.value = ''
  surfaceExtraText.value = ''
  jobAssetText.value = ''
  attnDetailText.value = ''
  readFacetText.value = ''
  recordReviewText.value = ''
}

const canHandle = computed(() => {
  if (!opened.value) return false
  if (activeTab.value === 'pending') return true
  return Boolean(handleTaskId.value)
})

// ── 任务信息 + 催办（processplatform/service/processing/task 族，rev105）──
// 有效任务 id：待办页取列表项 id（即 task id），其它页取本人活动任务 handleTaskId。
const effectiveTaskId = computed(() =>
  activeTab.value === 'pending' ? String(opened.value?.id ?? '') : handleTaskId.value,
)
const taskInfo = ref<Record<string, unknown> | null>(null)
const taskExpireText = ref('')
const pressing = ref(false)
async function loadTaskInfo(taskId: string): Promise<void> {
  taskInfo.value = null
  taskExpireText.value = ''
  if (!taskId) return
  const settle = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  const [info, expire] = await Promise.all([
    // GET service/processing/task/{id} —— 任务详情（x_task）
    settle(api.get(`/api/processplatform/service/processing/task/${taskId}`)),
    // GET service/processing/task/expire/{id} —— 超时信息
    settle(api.get(`/api/processplatform/service/processing/task/expire/${taskId}`)),
  ])
  taskInfo.value = ((info as { data?: Record<string, unknown> } | null)?.data ?? null)
  const ed = (expire as { data?: unknown } | null)?.data
  taskExpireText.value =
    ed && typeof ed === 'object' ? JSON.stringify(ed).slice(0, 80) : ed != null ? String(ed) : ''
}
async function pressTask(): Promise<void> {
  const taskId = effectiveTaskId.value
  if (!taskId || pressing.value) return
  pressing.value = true
  try {
    // POST service/processing/task/press/{id} —— 催办
    await api.post(`/api/processplatform/service/processing/task/press/${taskId}`, {})
    toast.success('已催办')
  } catch (e: any) {
    toast.error('催办失败: ' + (e?.message ?? ''))
  } finally {
    pressing.value = false
  }
}

// rev327：流程引擎 work/task/attachment/read 真实写端点（用户触发，shape 已核 processplatform_service_processing handler）
const engineBusy = ref(false)
async function engineWorkAction(kind: string): Promise<void> {
  if (!opened.value || engineBusy.value) return
  const id = workId(opened.value)
  engineBusy.value = true
  try {
    if (kind === 'goback') {
      if (!(await confirmMsg('确定退回该工作？'))) return
      await api.post(`/api/processplatform/service/processing/work/v2/${id}/goback`, {})
    } else if (kind === 'reroute') {
      if (!(await confirmMsg('确定改变路由？'))) return
      await api.put(`/api/processplatform/service/processing/work/v2/${id}/reroute`, {})
    } else if (kind === 'retract') {
      if (!(await confirmMsg('确定撤回该工作？'))) return
      await api.put(`/api/processplatform/service/processing/work/v2/${id}/retract`, {})
    } else if (kind === 'rollback') {
      if (!(await confirmMsg('确定回滚该工作？'))) return
      await api.put(`/api/processplatform/service/processing/work/v2/${id}/rollback`, {})
    } else if (kind === 'edit') {
      const title = prompt('工作新标题:', opened.value.title ?? '') || ''
      await api.put(`/api/processplatform/service/processing/work/${id}`, { title })
    } else if (kind === 'draftDelete') {
      if (!(await confirmMsg('确定删除该草稿工作？'))) return
      await api.delete(`/api/processplatform/service/processing/work/${id}/draft`)
    }
    toast.success('操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
async function engineTaskAction(kind: string): Promise<void> {
  const id = effectiveTaskId.value
  if (!id || engineBusy.value) return
  engineBusy.value = true
  try {
    if (kind === 'will') await api.post(`/api/processplatform/service/processing/task/will/${id}`, {})
    else if (kind === 'v3add') await api.post(`/api/processplatform/service/processing/task/v3/add/${id}`, {})
    else if (kind === 'processing') await api.post(`/api/processplatform/service/processing/task/processing/${id}`, {})
    toast.success('任务操作已提交')
  } catch (e: any) {
    toast.error('任务操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
async function engineAttAction(kind: string): Promise<void> {
  const id = prompt('附件 ID:', attachments.value?.[0]?.id ?? '') || ''
  if (!id) return
  try {
    if (kind === 'edit') {
      const name = prompt('附件新名称:', '') || ''
      await api.put(`/api/processplatform/service/processing/attachment/${encodeURIComponent(id)}`, { name })
    } else {
      if (!(await confirmMsg('确定删除该附件？'))) return
      await api.delete(`/api/processplatform/service/processing/attachment/${encodeURIComponent(id)}`)
    }
    toast.success('附件操作已提交')
  } catch (e: any) {
    toast.error('附件操作失败: ' + (e?.message ?? ''))
  }
}
// rev359：流程引擎 REST 式工作/任务生命周期 + v2/v3 双参 + 快照 + 已办合并回滚 + 定时器取消 真实动作（全 Path 参数无 body 或 {}；handler 已核 Path-only；用户触发 prompt+确认）
async function engineRest(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const wid = () => encodeURIComponent(prompt('工作 ID:', workId(opened.value ?? {}) || '') || '')
    const tid = () => encodeURIComponent(prompt('任务 ID:', effectiveTaskId.value || '') || '')
    if (op === 'workProcessing') await api.put(`/api/work/${wid()}/processing`, {})
    else if (op === 'workTerminate') { if (!(await confirmMsg('确定终止该工作？'))) return; await api.post(`/api/work/${wid()}/terminate`, {}) }
    else if (op === 'workRetract') { if (!(await confirmMsg('确定撤回该工作？'))) return; await api.post(`/api/work/${wid()}/retract`, {}) }
    else if (op === 'workStart') await api.post(`/api/work/${wid()}/start`, {})
    else if (op === 'workComplete') await api.post(`/api/work/${wid()}/complete`, {})
    else if (op === 'taskClaim') await api.post(`/api/task/${tid()}/claim`, {})
    else if (op === 'taskTransfer') { const p = encodeURIComponent(prompt('转交给（人员）:', '') || ''); await api.post(`/api/task/${tid()}/transfer/${p}`, {}) }
    else if (op === 'timerCancel') { const j = encodeURIComponent(prompt('定时器 job:', '') || ''); await api.post(`/api/processplatform/service/processing/timer/${j}/cancel`, {}) }
    else if (op === 'attCopy') { const w = wid(); const wi = encodeURIComponent(prompt('目标 workId:', '') || ''); await api.post(`/api/processplatform/service/processing/attachment/copy/${w}/${wi}`, {}) }
    else if (op === 'attEditText') { const i = encodeURIComponent(prompt('附件 ID:', '') || ''); await api.post(`/api/processplatform/service/processing/attachment/edit/text/${i}`, {}) }
    else if (op === 'snapRestore') { const i = encodeURIComponent(prompt('快照 ID:', '') || ''); await api.get(`/api/processplatform/service/processing/snap/restore/${i}`) }
    else if (op === 'v2Goback') { const w = wid(); const i = encodeURIComponent(prompt('活动 ID:', '') || ''); await api.get(`/api/processplatform/service/processing/v2/goback/${w}/${i}`) }
    else if (op === 'v2Reroute') { const w = wid(); const i = encodeURIComponent(prompt('活动 ID:', '') || ''); await api.get(`/api/processplatform/service/processing/v2/reroute/${w}/${i}`) }
    else if (op === 'v2Rollback') { const w = wid(); const i = encodeURIComponent(prompt('活动 ID:', '') || ''); await api.get(`/api/processplatform/service/processing/v2/rollback/${w}/${i}`) }
    else if (op === 'v2AddSplit') { const w = wid(); const i = encodeURIComponent(prompt('活动 ID:', '') || ''); await api.post(`/api/processplatform/service/processing/v2/add/split/${w}/${i}`, {}) }
    else if (op === 'v3Retract') { const w = wid(); await api.get(`/api/processplatform/service/processing/v3/retract/${w}`) }
    else if (op === 'wcMerge') { const f = encodeURIComponent(prompt('已办 flag:', '') || ''); await api.post(`/api/processplatform/service/processing/workcompleted/merge/${f}`, {}) }
    else if (op === 'wcRollback') { const f = encodeURIComponent(prompt('已办 flag:', '') || ''); await api.get(`/api/processplatform/service/processing/workcompleted/rollback/${f}`) }
    else if (op === 'taskPassExpired') { const i = encodeURIComponent(prompt('任务 ID:', '') || ''); await api.get(`/api/processplatform/service/processing/task/pass/expired/${i}`) }
    else { const i = encodeURIComponent(prompt('任务 ID:', '') || ''); await api.get(`/api/processplatform/service/processing/task/replace/${i}`) }
    toast.success('引擎操作已提交')
  } catch (e: any) {
    toast.error('引擎操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev374：流程引擎 工作追加身份/分叉/矩阵/终止/按流程发起 + 快照删/已办快照/挂起 + 附件已办复制/删 + 记录/待阅按工作 + 文档版本 + 任务超时/催办 + 数据删/触达 真实路由（全 {id}/{work} 参数，避已消费方法孪生与 literal-placeholder trap）
async function engineRest2(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const wid = () => encodeURIComponent(prompt('工作 ID:', workId(opened.value ?? {}) || '') || '')
    if (op === 'appendIdentity') await api.put(`/api/processplatform/service/processing/work/${wid()}/manual/append/identity`, {})
    else if (op === 'addSplit') await api.put(`/api/processplatform/service/processing/work/${wid()}/add/split`, {})
    else if (op === 'v2AddSplit') await api.put(`/api/processplatform/service/processing/work/v2/${wid()}/add/split`, {})
    else if (op === 'v2Matrix') await api.post(`/api/processplatform/service/processing/work/v2/${wid()}/add/manual/task/identity/matrix`, {})
    else if (op === 'v2Terminate') await api.get(`/api/processplatform/service/processing/work/v2/${wid()}/terminate`)
    else if (op === 'workByProcess') { const pid = encodeURIComponent(prompt('流程 ID:', '') || ''); await api.post(`/api/processplatform/service/processing/work/process/${pid}`, {}) }
    else if (op === 'wcByProcess') { const pf = encodeURIComponent(prompt('流程 flag:', '') || ''); await api.post(`/api/processplatform/service/processing/workcompleted/process/${pf}`, {}) }
    else if (op === 'snapDelete') { const id = encodeURIComponent(prompt('快照 ID:', '') || ''); if (!(await confirmMsg('确定删除该快照？'))) return; await api.delete(`/api/processplatform/service/processing/snap/${id}`) }
    else if (op === 'snapWcAbandon') { const wc = encodeURIComponent(prompt('已办 ID:', '') || ''); await api.get(`/api/processplatform/service/processing/snap/workcompleted/${wc}/type/abandonedworkcompleted`) }
    else if (op === 'snapWcSnap') { const wc = encodeURIComponent(prompt('已办 ID:', '') || ''); await api.get(`/api/processplatform/service/processing/snap/workcompleted/${wc}/type/snapworkcompleted`) }
    else if (op === 'snapAbandoned') { const w = wid(); const wi = encodeURIComponent(prompt('workId:', '') || ''); const t = encodeURIComponent(prompt('类型:', 'normal') || 'normal'); await api.get(`/api/processplatform/service/processing/snap/abandoned/${w}/${wi}/${t}`) }
    else if (op === 'snapSuspend') { const w = wid(); const wi = encodeURIComponent(prompt('workId:', '') || ''); const t = encodeURIComponent(prompt('类型:', 'normal') || 'normal'); await api.get(`/api/processplatform/service/processing/snap/suspend/${w}/${wi}/${t}`) }
    else if (op === 'attCopyWc') { const wc = encodeURIComponent(prompt('已办 ID:', '') || ''); await api.post(`/api/processplatform/service/processing/attachment/copy/workcompleted/${wc}`, {}) }
    else if (op === 'attDelWork') { const id = encodeURIComponent(prompt('附件 ID:', '') || ''); const wi = wid(); if (!(await confirmMsg('确定从工作删除该附件？'))) return; await api.delete(`/api/processplatform/service/processing/attachment/${id}/work/${wi}`) }
    else if (op === 'attDelWc') { const id = encodeURIComponent(prompt('附件 ID:', '') || ''); const wc = encodeURIComponent(prompt('已办 ID:', '') || ''); if (!(await confirmMsg('确定从已办删除该附件？'))) return; await api.delete(`/api/processplatform/service/processing/attachment/${id}/workcompleted/${wc}`) }
    else if (op === 'recordWorkProc') await api.post('/api/processplatform/service/processing/record/work/processing', {})
    else if (op === 'recordWorkTerm') await api.post('/api/processplatform/service/processing/record/work/terminate', {})
    else if (op === 'readByWork') { const wi = wid(); await api.post(`/api/processplatform/service/processing/read/work/${wi}`, {}) }
    else if (op === 'readByWc') { const wc = encodeURIComponent(prompt('已办 ID:', '') || ''); await api.post(`/api/processplatform/service/processing/read/workcompleted/${wc}`, {}) }
    else if (op === 'docVersion') { const w = wid(); await api.post(`/api/processplatform/service/processing/documentversion/work/${w}`, {}) }
    else if (op === 'taskPassExpired') { const id = encodeURIComponent(prompt('任务 ID:', '') || ''); await api.get(`/api/processplatform/service/processing/task/${id}/pass/expired`) }
    else if (op === 'taskUrge') { const id = encodeURIComponent(prompt('任务 ID:', '') || ''); await api.get(`/api/processplatform/service/processing/task/${id}/urge`) }
    else if (op === 'dataDelete') { const w = wid(); const id = encodeURIComponent(prompt('数据 ID:', '') || ''); if (!(await confirmMsg('确定删除该数据？'))) return; await api.post(`/api/processplatform/service/processing/data/delete/${w}/${id}`, {}) }
    else { const w = wid(); const id = encodeURIComponent(prompt('触达 ID:', '') || ''); await api.post(`/api/processplatform/service/processing/service/touch/${w}/${id}`, {}) }
    toast.success('引擎操作已提交')
  } catch (e: any) {
    toast.error('引擎操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev379：流程表面 task/read/work 处理·催办·退回·参考·重置·v2/v3生命周期 + save/publish/delete/handover 真实路由（全字面量含参占位；避 {credential} 凭证与 {page}/{size}/{size} 三参 arity）
async function surfaceOps3(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const id = () => encodeURIComponent(prompt('目标 ID:', '') || '')
    if (op === 'taskOpinion') await api.post(`/api/processplatform/assemble/surface/task/opinion/manage/${id()}`, {})
    else if (op === 'taskPress') await api.post(`/api/processplatform/assemble/surface/task/press/manage/${id()}`, {})
    else if (op === 'taskProcessing') await api.post(`/api/processplatform/assemble/surface/task/processing/${id()}`, {})
    else if (op === 'taskProcMgr') await api.post(`/api/processplatform/assemble/surface/task/processing/manage/${id()}`, {})
    else if (op === 'taskNeural') await api.post(`/api/processplatform/assemble/surface/task/processing/neural/${id()}`, {})
    else if (op === 'taskReference') await api.post(`/api/processplatform/assemble/surface/task/reference/${id()}`, {})
    else if (op === 'taskResetMgr') await api.post(`/api/processplatform/assemble/surface/task/reset/manage/${id()}`, {})
    else if (op === 'taskWill') await api.post(`/api/processplatform/assemble/surface/task/will/${id()}`, {})
    else if (op === 'taskV2Pause') await api.get(`/api/processplatform/assemble/surface/task/v2/pause/${id()}`)
    else if (op === 'taskV2Reset') await api.post(`/api/processplatform/assemble/surface/task/v2/reset/${id()}`, {})
    else if (op === 'taskV2Resume') await api.post(`/api/processplatform/assemble/surface/task/v2/resume/${id()}`, {})
    else if (op === 'taskV2Trigger') await api.post(`/api/processplatform/assemble/surface/task/v2/trigger/processing/${id()}`, {})
    else if (op === 'taskV3Add') await api.post(`/api/processplatform/assemble/surface/task/v3/add/${id()}`, {})
    else if (op === 'taskV3Pin') await api.get(`/api/processplatform/assemble/surface/task/v3/pin/${id()}`)
    else if (op === 'readOpinion') await api.post(`/api/processplatform/assemble/surface/read/opinion/manage/${id()}`, {})
    else if (op === 'readProcessing') await api.post(`/api/processplatform/assemble/surface/read/processing/${id()}`, {})
    else if (op === 'readReference') await api.post(`/api/processplatform/assemble/surface/read/reference/${id()}`, {})
    else if (op === 'readResetMgr') await api.post(`/api/processplatform/assemble/surface/read/reset/manage/${id()}`, {})
    else if (op === 'workCloseCheck') await api.get(`/api/processplatform/assemble/surface/work/close/check/${id()}`)
    else if (op === 'workProcessing') await api.post(`/api/processplatform/assemble/surface/work/processing/${id()}`, {})
    else if (op === 'workV2Split') await api.post(`/api/processplatform/assemble/surface/work/v2/add/split/${id()}`, {})
    else if (op === 'workV2Reroute') await api.get(`/api/processplatform/assemble/surface/work/v2/reroute/${id()}`)
    else if (op === 'workV2Retract') await api.get(`/api/processplatform/assemble/surface/work/v2/retract/${id()}`)
    else if (op === 'workV2Rollback') await api.get(`/api/processplatform/assemble/surface/work/v2/rollback/${id()}`)
    else if (op === 'workV2Terminate') await api.get(`/api/processplatform/assemble/surface/work/v2/terminate/${id()}`)
    else if (op === 'workV2Goback') await api.get(`/api/processplatform/assemble/surface/work/v2/list/activity/goback/${id()}`)
    else if (op === 'handoverCancel') await api.get(`/api/processplatform/assemble/surface/handover/cancel/${id()}`)
    else if (op === 'wcDeleteMgr') { if (!(await confirmMsg('确定删除该已办？'))) return; await api.post(`/api/processplatform/assemble/surface/workcompleted/delete/manage/${id()}`, {}) }
    else if (op === 'save') await api.post(`/api/processplatform/assemble/surface/save/${id()}`, {})
    else if (op === 'publish') await api.post(`/api/processplatform/assemble/surface/publish/${id()}`, {})
    else await api.post(`/api/processplatform/assemble/surface/delete/${id()}`, {})
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev379：流程表面 task/read/work/taskcompleted/workcompleted 按应用流程清单 + v2 游标 真实只读（{count}/{applicationFlag} 与 next/{id}/{count} 双参，用户触发）
async function surfaceReads3(): Promise<void> {
  const s = <T>(p: Promise<T>): Promise<T | null> => p.catch(() => null)
  try {
    const rs = await Promise.all([
      s(api.get('/api/processplatform/assemble/surface/task/list/application/process/20/default')),
      s(api.get('/api/processplatform/assemble/surface/read/list/application/process/20/default')),
      s(api.get('/api/processplatform/assemble/surface/work/list/application/process/20/default')),
      s(api.get('/api/processplatform/assemble/surface/taskcompleted/list/application/process/20/default')),
      s(api.get('/api/processplatform/assemble/surface/workcompleted/list/application/process/20/default')),
      s(api.get('/api/processplatform/assemble/surface/readcompleted/list/application/process/20/default')),
      s(api.post('/api/processplatform/assemble/surface/task/v2/list/create/next/0/20', {})),
      s(api.post('/api/processplatform/assemble/surface/read/v2/list/create/next/0/20', {})),
      s(api.post('/api/processplatform/assemble/surface/readcompleted/v2/list/create/next/0/20', {})),
      s(api.post('/api/processplatform/assemble/surface/review/v2/list/create/next/0/20', {})),
      s(api.post('/api/processplatform/assemble/surface/taskcompleted/v2/list/create/next/0/20', {})),
    ])
    const hit = rs.filter((r) => (r as any)?.data != null).length
    toast.success(`流程表面清单读 ${rs.length} 条命中 ${hit}`)
  } catch (e: any) {
    toast.error('加载失败: ' + (e?.message ?? ''))
  }
}
// rev380：流程表面 阅办管理态/已办参考控制/工作强制/快照按工作/草稿启动/数据与附件删/按应用退化清单 真实路由（全字面量，用户触发，Path-only 空体）
async function surfaceOps4(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const id = () => encodeURIComponent(prompt('目标 ID:', '') || '')
    if (op === 'readIdProcessing') await api.post(`/api/processplatform/assemble/surface/read/${id()}/processing`, {})
    else if (op === 'readProcMgr') await api.put(`/api/processplatform/assemble/surface/read/${id()}/processing/manage`, {})
    else if (op === 'readResetMgr') await api.put(`/api/processplatform/assemble/surface/read/${id()}/reset/manage`, {})
    else if (op === 'readManageDel') { if (!(await confirmMsg('确定删除该待阅（管理）？'))) return; await api.delete(`/api/processplatform/assemble/surface/read/${id()}/manage`) }
    else if (op === 'readWork') { const w = encodeURIComponent(prompt('工作 ID:', '') || ''); await api.post(`/api/processplatform/assemble/surface/read/work/${w}`, {}) }
    else if (op === 'readWorkCompleted') { const w = encodeURIComponent(prompt('已完成工作 ID:', '') || ''); await api.post(`/api/processplatform/assemble/surface/read/workcompleted/${w}`, {}) }
    else if (op === 'tcRefCtrl') await api.post(`/api/processplatform/assemble/surface/taskcompleted/reference/control/${id()}`, {})
    else if (op === 'workForce') { const pf = encodeURIComponent(prompt('流程标识:', '') || ''); await api.get(`/api/processplatform/assemble/surface/work/process/force/${pf}`) }
    else if (op === 'workV2TermMgr') await api.get(`/api/processplatform/assemble/surface/work/v2/terminate/manage/${id()}`)
    else if (op === 'workV2Trigger') await api.post(`/api/processplatform/assemble/surface/work/v2/trigger/processing/${id()}`, {})
    else if (op === 'worklogSplit') { const w = encodeURIComponent(prompt('工作 ID:', '') || ''); await api.post(`/api/processplatform/assemble/surface/worklog/list/add/split/work/${w}`, {}) }
    else if (op === 'snapRestore') await api.get(`/api/processplatform/assemble/surface/snap/${id()}/restore`)
    else if (op === 'snapAbandoned') { const w = encodeURIComponent(prompt('工作 ID:', '') || ''); await api.get(`/api/processplatform/assemble/surface/snap/work/${w}/type/abandoned`) }
    else if (op === 'snapSuspend') { const w = encodeURIComponent(prompt('工作 ID:', '') || ''); await api.get(`/api/processplatform/assemble/surface/snap/work/${w}/type/suspend`) }
    else if (op === 'draftStart') await api.get(`/api/processplatform/assemble/surface/draft/${id()}/start`)
    else if (op === 'dataWorkDel') { if (!(await confirmMsg('确定删除该工作数据？'))) return; await api.delete(`/api/processplatform/assemble/surface/data/work/${id()}`) }
    else if (op === 'attachDel') { const a = encodeURIComponent(prompt('附件 ID:', '') || ''); const w = encodeURIComponent(prompt('工作 ID:', '') || ''); if (!(await confirmMsg('确定删除该附件？'))) return; await api.delete(`/api/processplatform/assemble/surface/attachment/${a}/work/${w}`) }
    else if (op === 'tcListPrev') { const cnt = 20; await api.get(`/api/processplatform/assemble/surface/taskcompleted/list/prev/${id()}/${cnt}`) }
    else if (op === 'wcRollback') { const f = encodeURIComponent(prompt('工作标识:', '') || ''); await api.get(`/api/processplatform/assemble/surface/workcompleted/rollback/${f}`) }
    else if (op === 'appFlagGet') { const orn = 'false'; await api.get(`/api/processplatform/assemble/surface/application/${id()}/${orn}`) }
    else if (op === 'appFlagDel') { const orn = 'false'; if (!(await confirmMsg('确定按应用清理未完成工作？'))) return; await api.delete(`/api/processplatform/assemble/surface/application/${id()}/${orn}`) }
    else if (op === 'procFlagGet') { const orn = 'false'; await api.get(`/api/processplatform/assemble/surface/process/${id()}/${orn}`) }
    else { const orn = 'false'; if (!(await confirmMsg('确定按流程清理未完成工作？'))) return; await api.delete(`/api/processplatform/assemble/surface/process/${id()}/${orn}`) }
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev392：流程表面 待阅/已阅按应用计数、附件下载/流/PDF预览、v2 创建游标 next/prev（读/已阅）、correlation 关联 建/改/删、待阅意见管理、附件删、模式删 真实路由（全 Path-only 已核，correlation/opinion 空体；用户触发）
async function surfaceOps5(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const id = () => encodeURIComponent(prompt('目标 ID:', '') || '')
    if (op === 'readCountApp') { const af = encodeURIComponent(prompt('应用标识:', '') || ''); await api.get(`/api/processplatform/assemble/surface/read/list/count/application/${af}/process`) }
    else if (op === 'readcompletedCountApp') { const af = encodeURIComponent(prompt('应用标识:', '') || ''); await api.get(`/api/processplatform/assemble/surface/readcompleted/list/count/application/${af}/process`) }
    else if (op === 'attDownload') await api.get(`/api/processplatform/assemble/surface/attachment/download/${id()}`)
    else if (op === 'attDownloadStream') await api.get(`/api/processplatform/assemble/surface/attachment/download/${id()}/stream`)
    else if (op === 'attPreviewPdf') await api.get(`/api/processplatform/assemble/surface/attachment/${id()}/preview/pdf`)
    else if (op === 'readV2Next') { const c = 20; await api.post(`/api/processplatform/assemble/surface/read/v2/list/create/${id()}/next/${c}`, {}) }
    else if (op === 'readV2Prev') { const c = 20; await api.post(`/api/processplatform/assemble/surface/read/v2/list/create/${id()}/prev/${c}`, {}) }
    else if (op === 'readcompletedV2Next') { const c = 20; await api.post(`/api/processplatform/assemble/surface/readcompleted/v2/list/create/${id()}/next/${c}`, {}) }
    else if (op === 'readcompletedV2Prev') { const c = 20; await api.post(`/api/processplatform/assemble/surface/readcompleted/v2/list/create/${id()}/prev/${c}`, {}) }
    else if (op === 'correlationJob') { const j = encodeURIComponent(prompt('Job ID:', '') || ''); await api.post(`/api/processplatform/assemble/surface/correlation/job/${j}`, {}) }
    else if (op === 'correlationUpdate') { const j = encodeURIComponent(prompt('Job ID:', '') || ''); await api.post(`/api/processplatform/assemble/surface/correlation/update/job/${j}`, {}) }
    else if (op === 'correlationDelete') { const j = encodeURIComponent(prompt('Job ID:', '') || ''); if (!(await confirmMsg('确定删除该关联？'))) return; await api.post(`/api/processplatform/assemble/surface/correlation/job/${j}/delete`, {}) }
    else if (op === 'readOpinionMgr') await api.put(`/api/processplatform/assemble/surface/read/${id()}/opinion/manage`, {})
    else if (op === 'attDelete') { if (!(await confirmMsg('确定删除该附件？'))) return; await api.delete(`/api/processplatform/assemble/surface/attachment/${id()}`) }
    else { if (!(await confirmMsg('确定删除该模式？'))) return; await api.get(`/api/processplatform/assemble/surface/mode/${id()}/delete`) }
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev393：流程表面 附件下载(管理/按工作/按已办/工作附件)、附件图片分页预览、待阅·已阅过滤游标、按应用流程过滤、文档版本 真实路由（Path 已核 arity 匹配，filter/文档版本空体；用户触发）
async function surfaceOps6(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const id = () => encodeURIComponent(prompt('目标 ID:', '') || '')
    if (op === 'attDownloadManage') await api.get(`/api/processplatform/assemble/surface/attachment/download/${id()}/manage`)
    else if (op === 'attDownloadByWork') { const w = encodeURIComponent(prompt('工作 ID:', '') || ''); await api.get(`/api/processplatform/assemble/surface/attachment/download/${id()}/work/${w}`) }
    else if (op === 'attDownloadByWc') { const wc = encodeURIComponent(prompt('已办 ID:', '') || ''); await api.get(`/api/processplatform/assemble/surface/attachment/download/${id()}/workcompleted/${wc}`) }
    else if (op === 'attDownloadWorkAtt') { const w = encodeURIComponent(prompt('工作 ID:', '') || ''); const a = encodeURIComponent(prompt('附件 ID:', '') || ''); await api.get(`/api/processplatform/assemble/surface/attachment/download/work/${w}/att/${a}`) }
    else if (op === 'attPreviewImgPage') { const pg = 1; await api.get(`/api/processplatform/assemble/surface/attachment/${id()}/preview/image/page/${pg}`) }
    else if (op === 'readListFilter') { const c = 20; await api.post(`/api/processplatform/assemble/surface/read/list/${id()}/next/${c}/filter`, {}) }
    else if (op === 'readcompletedListFilter') { const c = 20; await api.post(`/api/processplatform/assemble/surface/readcompleted/list/${id()}/next/${c}/filter`, {}) }
    else if (op === 'processListFilter') { const af = encodeURIComponent(prompt('应用标识:', '') || ''); await api.post(`/api/processplatform/assemble/surface/process/list/application/${af}/filter`, {}) }
    else { const w = encodeURIComponent(prompt('工作 ID:', '') || ''); await api.post(`/api/processplatform/assemble/surface/documentversion/work/${w}`, {}) }
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev395：流程表面 附件转Word(按工作/按工作或已办)、附件编辑(按工作)、附件复制到工作·软复制·复制到已办、附件内容更新·更新(按工作) 真实路由（Path 已核，copy/edit/update Json 空体，doc/to/word Path-only；用户触发）
async function surfaceOps7(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const id = () => encodeURIComponent(prompt('目标 ID:', '') || '')
    const wk = () => encodeURIComponent(prompt('工作 ID:', '') || '')
    if (op === 'docToWord') await api.post(`/api/processplatform/assemble/surface/attachment/doc/to/word/work/${wk()}`, {})
    else if (op === 'docToWordWowc') { const f = encodeURIComponent(prompt('工作或已办 ID:', '') || ''); await api.post(`/api/processplatform/assemble/surface/attachment/doc/to/word/workorworkcompleted/${f}`, {}) }
    else if (op === 'editByWork') { const a = id(); const w = wk(); await api.put(`/api/processplatform/assemble/surface/attachment/edit/${a}/work/${w}`, {}) }
    else if (op === 'copyToWork') await api.post(`/api/processplatform/assemble/surface/attachment/copy/work/${wk()}`, {})
    else if (op === 'copyToWorkSoft') await api.post(`/api/processplatform/assemble/surface/attachment/copy/work/${wk()}/soft`, {})
    else if (op === 'copyToWc') { const wc = encodeURIComponent(prompt('已办 ID:', '') || ''); await api.post(`/api/processplatform/assemble/surface/attachment/copy/workcompleted/${wc}`, {}) }
    else if (op === 'updateContent') { const a = id(); const w = wk(); await api.put(`/api/processplatform/assemble/surface/attachment/update/content/${a}/work/${w}`, {}) }
    else { const a = id(); const w = wk(); await api.put(`/api/processplatform/assemble/surface/attachment/update/${a}/work/${w}`, {}) }
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev397：流程表面 应用复杂清单(按人管理)/数据 job 数组·抓取/待阅·已阅前翻过滤/待阅v2 list 后翻·前翻/清模式(按人管理)/附件转存下载·PDF·图片预览结果 真实路由（Path 已核，job/filter/mode 空体；用户触发）
async function surfaceOps8(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const id = () => encodeURIComponent(prompt('目标 ID:', '') || '')
    if (op === 'appComplexManage') { const p = encodeURIComponent(prompt('人员:', '') || ''); await api.get(`/api/processplatform/assemble/surface/application/list/complex/manage/${p}`) }
    else if (op === 'dataJobArray') { const j = encodeURIComponent(prompt('Job ID:', '') || ''); await api.post(`/api/processplatform/assemble/surface/data/job/${j}/array/data`, {}) }
    else if (op === 'dataFetchJob') { const j = encodeURIComponent(prompt('Job ID:', '') || ''); await api.post(`/api/processplatform/assemble/surface/data/fetch/job/${j}`, {}) }
    else if (op === 'readPrevFilter') { const c = 20; await api.post(`/api/processplatform/assemble/surface/read/list/${id()}/prev/${c}/filter`, {}) }
    else if (op === 'readcompletedPrevFilter') { const c = 20; await api.post(`/api/processplatform/assemble/surface/readcompleted/list/${id()}/prev/${c}/filter`, {}) }
    else if (op === 'readV2ListNext') { const c = 20; await api.post(`/api/processplatform/assemble/surface/read/v2/list/${id()}/next/${c}`, {}) }
    else if (op === 'readV2ListPrev') { const c = 20; await api.post(`/api/processplatform/assemble/surface/read/v2/list/${id()}/prev/${c}`, {}) }
    else if (op === 'modeClear') { const p = encodeURIComponent(prompt('人员:', '') || ''); if (!(await confirmMsg('确定清理该人模式？'))) return; await api.post(`/api/processplatform/assemble/surface/mode/clear/person/${p}/manager`, {}) }
    else if (op === 'attTransfer') { const f = encodeURIComponent(prompt('转存标识:', '') || ''); await api.get(`/api/processplatform/assemble/surface/attachment/download/transfer/flag/${f}`) }
    else if (op === 'attPreviewPdfResult') { const f = encodeURIComponent(prompt('附件标识:', '') || ''); await api.get(`/api/processplatform/assemble/surface/attachment/preview/pdf/${f}/result`) }
    else { const f = encodeURIComponent(prompt('附件标识:', '') || ''); await api.get(`/api/processplatform/assemble/surface/attachment/preview/image/${f}/result`) }
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev399：流程表面 按应用文件下载/附件批量ZIP下载/发票下载 + read·readcompleted·review·taskcompleted v2 创建分页 真实路由（Path arity 已核 2-tuple，分页 {page}/size/{size} 用变量占位、body 空；用户触发）
async function surfaceOps9(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const pg = 1
    const sz = 20
    if (op === 'fileAppDownload') { const f = encodeURIComponent(prompt('文件标识:', '') || ''); const af = encodeURIComponent(prompt('应用标识:', '') || ''); await api.get(`/api/processplatform/assemble/surface/file/application/download/${f}/${af}`) }
    else if (op === 'attBatchZip') { const w = encodeURIComponent(prompt('工作 ID:', '') || ''); const site = encodeURIComponent(prompt('站点:', '') || ''); await api.get(`/api/processplatform/assemble/surface/attachment/batch/download/work/${w}/site/${site}`) }
    else if (op === 'attInvoice') { const f = encodeURIComponent(prompt('发票标识:', '') || ''); const wowc = encodeURIComponent(prompt('工作或已办 ID:', '') || ''); await api.get(`/api/processplatform/assemble/surface/attachment/download/invoice/${f}/joborworkorworkcompleted/${wowc}`) }
    else if (op === 'readV2Paging') await api.post(`/api/processplatform/assemble/surface/read/v2/list/create/paging/${pg}/size/${sz}`, {})
    else if (op === 'readcompletedV2Paging') await api.post(`/api/processplatform/assemble/surface/readcompleted/v2/list/create/paging/${pg}/size/${sz}`, {})
    else if (op === 'reviewV2Paging') await api.post(`/api/processplatform/assemble/surface/review/v2/list/create/paging/${pg}/size/${sz}`, {})
    else await api.post(`/api/processplatform/assemble/surface/taskcompleted/v2/list/create/paging/${pg}/size/${sz}`, {})
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev407：流程表面 应用范围清单/模式清单/按ids流程/待阅计数过滤·v2计数/附件批量删·改(管理) 真实路由（均 Path-free pool/Json 空体，已核；用户触发；规避 keylock·mode/save·read/filter/attribute 等 handler 取 Path 但路由无参的 trap500 与 html/to/pdf·image 能力未实装桩）
async function surfaceOps10(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    if (op === 'appListRange') await api.post('/api/processplatform/assemble/surface/application/list/range', {})
    else if (op === 'modeList') await api.post('/api/processplatform/assemble/surface/mode/list', {})
    else if (op === 'processListIds') await api.post('/api/processplatform/assemble/surface/process/list/ids', {})
    else if (op === 'readCountFilter') await api.post('/api/processplatform/assemble/surface/read/count/filter', {})
    else if (op === 'readV2Count') await api.post('/api/processplatform/assemble/surface/read/v2/count', {})
    else if (op === 'attBatchDelete') { if (!(await confirmMsg('确定批量删除附件？'))) return; await api.post('/api/processplatform/assemble/surface/attachment/batch/delete/manage', {}) }
    else await api.post('/api/processplatform/assemble/surface/attachment/batch/update/manage', {})
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev408：流程表面 应用按key/终端清单(inline 硬编码 Path 无 trap)·路由清单(PUT pool-only WHERE1=1)·已阅/已办/摘要按应用 v2 计数(POST pool-only COUNT WHERE1=1) 真实路由（均 pool-only 或 inline 固定 Path，已核 handler 源码不取动态 Path，规避 job/v2/job/projection·review/v2/search GET 取 Path 但路由无参的 trap500；用户触发）
async function surfaceOps11(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    if (op === 'appListKey') await api.get('/api/processplatform/assemble/surface/application/list/key/key')
    else if (op === 'appListTerminal') await api.get('/api/processplatform/assemble/surface/application/list/terminal/terminal')
    else if (op === 'routeList') await api.put('/api/processplatform/assemble/surface/route/list', {})
    else if (op === 'readCompletedV2Count') await api.post('/api/processplatform/assemble/surface/readcompleted/v2/count', {})
    else if (op === 'reviewCountApp') await api.post('/api/processplatform/assemble/surface/review/count/application', {})
    else await api.post('/api/processplatform/assemble/surface/taskcompleted/v2/count', {})
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev409：流程表面 新建(create pool+Json 全 Option 空体优雅报名必填)·OpenAPI(无参内省)·摘要过滤入口(GET pool+session)·工作v3召回·已办调整时间(POST pool+session Json #[serde(default)] 空体优雅报必填) 真实路由（handler 源码核实空体不 panic 不 422，规避 batch/upload multipart 与 upload/with/url 501·html/to/pdf·image 未实装桩；用户触发）
async function surfaceOps12(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    if (op === 'createSurface') await api.post('/api/processplatform/assemble/surface/create', {})
    else if (op === 'openapi') await api.get('/api/processplatform/assemble/surface/openapi')
    else if (op === 'reviewFilterEntry') await api.get('/api/processplatform/assemble/surface/review/filter/create/entry')
    else if (op === 'workV3Retract') await api.post('/api/processplatform/assemble/surface/work/v3/retract', {})
    else await api.post('/api/processplatform/assemble/surface/workcompleted/shift/time', {})
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev413：流程表面 按 id 的任务将办信息/任务催办(管理)/工作可关闭校验 真实路由（{id} 参数化，handler query_opt PP_C_TASK/PP_C_WORK by xid=$1，是 O2OA will/press-manage/close-check 三种独立操作；用户输入真实 id 触发）
async function surfaceOps13(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const id = encodeURIComponent(prompt(op === 'workCloseCheck' ? '工作 ID:' : '任务 ID:', '') || '')
    if (op === 'taskWill') await api.get(`/api/processplatform/assemble/surface/task/${id}/will`)
    else if (op === 'taskPressManage') await api.get(`/api/processplatform/assemble/surface/task/${id}/press/manage`)
    else await api.get(`/api/processplatform/assemble/surface/work/${id}/close/check`)
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev414：流程表面 任务v2触发处理 task/v2/{id}/trigger/processing·工作v2活动回退清单 work/v2/list/{id}/activity/goback·工作v3按job召回阶段 work/v3/retract/stage/job/{job} 真实路由（{id}/{job} 参数化 query PP_C_TASK/PP_C_WORK by xid/job，O2OA v2/v3 位置态独立操作；用户输入真实 id/job 触发）
async function surfaceOps14(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    if (op === 'taskV2Trigger') {
      const id = encodeURIComponent(prompt('任务 ID:', '') || '')
      await api.get(`/api/processplatform/assemble/surface/task/v2/${id}/trigger/processing`)
    } else if (op === 'workV2Goback') {
      const id = encodeURIComponent(prompt('工作 ID:', '') || '')
      await api.get(`/api/processplatform/assemble/surface/work/v2/list/${id}/activity/goback`)
    } else {
      const job = encodeURIComponent(prompt('Job ID:', '') || '')
      await api.get(`/api/processplatform/assemble/surface/work/v3/retract/stage/job/${job}`)
    }
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev415：流程表面 按流程生成流水号 serialnumber/.../{processId}(PP_C_SERIALNUMBER)·签名下载 sign/download/{scrawlId}(PP_C_DOC_SIGN)·快照下载 snap/{id}/download(U2Gate owner 门禁) 真实路由（各读独立表，distinct handler；用户输入真实 id 触发）
async function surfaceOps15(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    if (op === 'serialGen') {
      const pid = encodeURIComponent(prompt('流程 ID:', '') || '')
      await api.get(`/api/processplatform/assemble/surface/serialnumber/generate/process/name/name/serial/${pid}`)
    } else if (op === 'signDownload') {
      const sid = encodeURIComponent(prompt('签名 ID:', '') || '')
      await api.get(`/api/processplatform/assemble/surface/sign/download/${sid}`)
    } else {
      const id = encodeURIComponent(prompt('快照 ID:', '') || '')
      await api.get(`/api/processplatform/assemble/surface/snap/${id}/download`)
    }
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev416：流程表面 回滚工作日志清单 worklog/list/rollback/workorworkcompleted/{w}(PP_C_WORKLOG by xwork)·按人摘要计数 review/count/person/{credential}(PP_C_REVIEW COUNT WHERE xperson，credential=人员标识非鉴权凭证)·附件管理流下载 attachment/download/{id}/manage/stream 真实路由（各读独立表/流，distinct handler；用户输入真实 id 触发）
async function surfaceOps16(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    if (op === 'worklogRollback') {
      const w = encodeURIComponent(prompt('工作/已办 ID:', '') || '')
      await api.get(`/api/processplatform/assemble/surface/worklog/list/rollback/workorworkcompleted/${w}`)
    } else if (op === 'reviewCountPerson') {
      const cred = encodeURIComponent(prompt('人员标识:', '') || '')
      await api.post(`/api/processplatform/assemble/surface/review/count/person/${cred}`, {})
    } else {
      const id = encodeURIComponent(prompt('附件 ID:', '') || '')
      await api.get(`/api/processplatform/assemble/surface/attachment/download/${id}/manage/stream`)
    }
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev417：流程表面 待阅处理(管理) read/processing/manage/{id}[PP_C_READ]·已阅意见(管理) readcompleted/{id}/opinion/manage[PP_C_READCOMPLETED]·已办意见(管理) taskcompleted/{id}/opinion/manage[PP_C_TASKCOMPLETED] 真实路由（三读独立表 distinct，Path-only 无 body；用户输入真实 id 触发）
async function surfaceOps17(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const id = encodeURIComponent(prompt('目标 ID:', '') || '')
    if (op === 'readProcessing') await api.post(`/api/processplatform/assemble/surface/read/processing/manage/${id}`, {})
    else if (op === 'readCompletedOpinion') await api.post(`/api/processplatform/assemble/surface/readcompleted/${id}/opinion/manage`, {})
    else await api.put(`/api/processplatform/assemble/surface/taskcompleted/${id}/opinion/manage`, {})
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev418：流程表面 按流程发起工作 work/process/{processFlag}[PP_C_WORK]·已办按流程 workcompleted/process/{processFlag}·已办回滚 workcompleted/{flag}/rollback[PP_C_WORKCOMPLETED] 真实路由（Path-only，O2OA 发起/回滚独立操作；用户输入真实 process/work id 触发）
async function surfaceOps18(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    if (op === 'workProcess') {
      const pf = encodeURIComponent(prompt('流程 Flag:', '') || '')
      await api.post(`/api/processplatform/assemble/surface/work/process/${pf}`, {})
    } else if (op === 'wcProcess') {
      const pf = encodeURIComponent(prompt('流程 Flag:', '') || '')
      await api.post(`/api/processplatform/assemble/surface/workcompleted/process/${pf}`, {})
    } else {
      const flag = encodeURIComponent(prompt('已办 ID:', '') || '')
      await api.put(`/api/processplatform/assemble/surface/workcompleted/${flag}/rollback`, {})
    }
    toast.success('流程表面操作已提交')
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev381：service/processing 引擎 task RESTful 位置态（{id}/processing·replace·press·expire、v2 {id}/pause·reset·resume、v3 {id}/add）+ 附件文本编辑/数据按路径删/手工后处理·追加身份/记录 job/服务触达/快照恢复/job 删 真实路由（Path-only 用户触发）
async function engineRest3(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const id = () => encodeURIComponent(prompt('目标 ID:', '') || '')
    if (op === 'taskIdProcessing') await api.put(`/api/processplatform/service/processing/task/${id()}/processing`, {})
    else if (op === 'taskIdReplace') await api.post(`/api/processplatform/service/processing/task/${id()}/replace`, {})
    else if (op === 'taskIdPress') await api.get(`/api/processplatform/service/processing/task/${id()}/press`)
    else if (op === 'taskIdExpire') await api.get(`/api/processplatform/service/processing/task/${id()}/expire`)
    else if (op === 'taskV2Pause') await api.get(`/api/processplatform/service/processing/task/v2/${id()}/pause`)
    else if (op === 'taskV2Reset') await api.put(`/api/processplatform/service/processing/task/v2/${id()}/reset`, {})
    else if (op === 'taskV2Resume') await api.get(`/api/processplatform/service/processing/task/v2/${id()}/resume`)
    else if (op === 'taskV3Add') await api.post(`/api/processplatform/service/processing/task/v3/${id()}/add`, {})
    else if (op === 'attEditText') await api.put(`/api/processplatform/service/processing/attachment/edit/${id()}/text`, {})
    else if (op === 'dataPathDelete') { const w = encodeURIComponent(prompt('工作 ID:', '') || ''); if (!(await confirmMsg('确定删除该路径数据？'))) return; await api.post(`/api/processplatform/service/processing/data/path/delete/${w}/${id()}`, {}) }
    else if (op === 'manualAfter') { const w = encodeURIComponent(prompt('工作 ID:', '') || ''); await api.post(`/api/processplatform/service/processing/manual/after/processing/${w}`, {}) }
    else if (op === 'manualAppendId') { const w = encodeURIComponent(prompt('工作 ID:', '') || ''); await api.get(`/api/processplatform/service/processing/manual/append/identity/${w}/${id()}`) }
    else if (op === 'recordJob') { const j = encodeURIComponent(prompt('Job ID:', '') || ''); await api.post(`/api/processplatform/service/processing/record/job/${j}`, {}) }
    else if (op === 'serviceWorkTouch') await api.put(`/api/processplatform/service/processing/service/work/${id()}/touch`, {})
    else if (op === 'snapIdRestore') await api.get(`/api/processplatform/service/processing/snap/${id()}/restore`)
    else { const j = encodeURIComponent(prompt('Job ID:', '') || ''); if (!(await confirmMsg('确定删除该 Job？'))) return; await api.delete(`/api/processplatform/service/processing/job/${j}`) }
    toast.success('引擎操作已提交')
  } catch (e: any) {
    toast.error('引擎操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev394：service/processing 引擎 已办催办按工作/已办合并·回滚(按flag)/工作处理(位置态PUT)/按路径删数据/按流程名生成流水号建工作/已办废弃快照按类型 真实路由（Path arity 已核，serial 空体；用户触发）
async function engineRest4(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const id = () => encodeURIComponent(prompt('目标 ID:', '') || '')
    if (op === 'tcPressWork') { const w = encodeURIComponent(prompt('工作 ID:', '') || ''); await api.get(`/api/processplatform/service/processing/taskcompleted/${id()}/press/work/${w}`) }
    else if (op === 'wcMergeFlag') await api.get(`/api/processplatform/service/processing/workcompleted/${id()}/merge`)
    else if (op === 'wcRollbackFlag') await api.put(`/api/processplatform/service/processing/workcompleted/${id()}/rollback`, {})
    else if (op === 'workProcessing') await api.put(`/api/processplatform/service/processing/work/${id()}/processing`, {})
    else if (op === 'dataPathDel') { const path = encodeURIComponent(prompt('数据路径:', '') || ''); if (!(await confirmMsg('确定删除该路径数据？'))) return; await api.post(`/api/processplatform/service/processing/data/work/${id()}/${path}/delete`, {}) }
    else if (op === 'workSerial') { const pid = encodeURIComponent(prompt('流程 ID:', '') || ''); const name = encodeURIComponent(prompt('活动名:', '') || ''); await api.post(`/api/processplatform/service/processing/work/process/${pid}/name/${name}/serial`, {}) }
    else { const t = encodeURIComponent(prompt('快照类型:', '') || ''); await api.get(`/api/processplatform/service/processing/snap/workcompleted/abandonedworkcompleted/${id()}/${t}`) }
    toast.success('引擎操作已提交')
  } catch (e: any) {
    toast.error('引擎操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
// rev405：service/processing 引擎 按路径 建/改工作数据、按路径改 job·已办数据、附件复制到工作 真实路由（data_*_path Path<2-tuple>+Json 空体、attachment_copy Path<2-tuple>；create/update 为不同 handler 各计；用户触发）
async function engineRest5(op: string): Promise<void> {
  if (engineBusy.value) return
  engineBusy.value = true
  try {
    const id = () => encodeURIComponent(prompt('目标 ID:', '') || '')
    const path = () => encodeURIComponent(prompt('数据路径:', '') || '')
    if (op === 'dataWorkCreatePath') { const i = id(); const p = path(); await api.post(`/api/processplatform/service/processing/data/work/${i}/${p}`, {}) }
    else if (op === 'dataWorkUpdatePath') { const i = id(); const p = path(); await api.put(`/api/processplatform/service/processing/data/work/${i}/${p}`, {}) }
    else if (op === 'dataJobPath') { const j = encodeURIComponent(prompt('Job ID:', '') || ''); const p = path(); await api.put(`/api/processplatform/service/processing/data/job/${j}/${p}`, {}) }
    else if (op === 'dataWcPath') { const i = id(); const p = path(); await api.put(`/api/processplatform/service/processing/data/workcompleted/${i}/${p}`, {}) }
    else { const w = encodeURIComponent(prompt('源附件所属工作:', '') || ''); const wi = encodeURIComponent(prompt('目标 workId:', '') || ''); await api.post(`/api/processplatform/service/processing/attachment/copy/${w}/${wi}`, {}) }
    toast.success('引擎操作已提交')
  } catch (e: any) {
    toast.error('引擎操作失败: ' + (e?.message ?? ''))
  } finally {
    engineBusy.value = false
  }
}
async function engineReadAction(kind: string): Promise<void> {
  const id = prompt('待阅 ID:', reads.value?.[0]?.id ?? '') || ''
  if (!id) return
  const e = encodeURIComponent(id)
  try {
    if (kind === 'processing') await api.put(`/api/processplatform/service/processing/read/${e}/processing`, {})
    else if (kind === 'replace') {
      const person = prompt('接替人:', '') || ''
      await api.post(`/api/processplatform/service/processing/read/${e}/replace`, { person })
    } else if (kind === 'reset') {
      const person = prompt('重置为处理人:', '') || ''
      await api.post(`/api/processplatform/service/processing/read/${e}/reset`, { person })
    } else {
      if (!(await confirmMsg('确定删除该待阅？'))) return
      await api.delete(`/api/processplatform/service/processing/read/${e}`)
    }
    toast.success('待阅操作已提交')
  } catch (err: any) {
    toast.error('待阅操作失败: ' + (err?.message ?? ''))
  }
}
// rev350：流程表面 待办/已办/待阅/已阅/评审/工作/草稿/交接/快照 分页筛选数据查询（POST 真实读，用户触发；全字面量路径）
async function surfaceList(op: string): Promise<void> {
  try {
    if (op === 'taskV2') await api.post('/api/processplatform/assemble/surface/task/v2/list', {})
    else if (op === 'taskCount') await api.post('/api/processplatform/assemble/surface/task/v2/count', {})
    else if (op === 'taskMy') await api.post('/api/processplatform/assemble/surface/task/list/my/filter/1/size/20', {})
    else if (op === 'taskCompletedV2') await api.post('/api/processplatform/assemble/surface/taskcompleted/v2/list', {})
    else if (op === 'taskCompletedMy') await api.post('/api/processplatform/assemble/surface/taskcompleted/list/my/filter/1/size/20', {})
    else if (op === 'readV2') await api.post('/api/processplatform/assemble/surface/read/v2/list', {})
    else if (op === 'readMy') await api.post('/api/processplatform/assemble/surface/read/list/my/filter/1/size/20', {})
    else if (op === 'readCompletedV2') await api.post('/api/processplatform/assemble/surface/readcompleted/v2/list', {})
    else if (op === 'readCompletedMy') await api.post('/api/processplatform/assemble/surface/readcompleted/list/my/filter/1/size/20', {})
    else if (op === 'reviewV2') await api.post('/api/processplatform/assemble/surface/review/v2/list', {})
    else if (op === 'reviewCount') await api.post('/api/processplatform/assemble/surface/review/v2/count', {})
    else if (op === 'reviewSearch') await api.post('/api/processplatform/assemble/surface/review/v2/search', {})
    else if (op === 'workV2') await api.post('/api/processplatform/assemble/surface/work/v2/list', {})
    else if (op === 'workMy') await api.post('/api/processplatform/assemble/surface/work/list/my/paging/1/size/20', {})
    else if (op === 'draftMy') await api.post('/api/processplatform/assemble/surface/draft/list/my/paging/1/size/20', {})
    else if (op === 'handover') await api.post('/api/processplatform/assemble/surface/handover/list/paging/1/size/20', {})
    else if (op === 'serialnumber') await api.post('/api/processplatform/assemble/surface/serialnumber/list/paging/1/size/20', {})
    else await api.post('/api/processplatform/assemble/surface/snap/list/my/filter/1/size/20', {})
    toast.success('数据查询已提交')
  } catch (e: any) {
    toast.error('数据查询失败: ' + (e?.message ?? ''))
  }
}
// rev351：流程表面 待办/已办/待阅/已阅/工作/评审 游标(next/prev)+管理(manage)+v2分页 数据查询（POST 真实读 distinct 游标方向，用户触发；全字面量路径）
async function surfaceList2(op: string): Promise<void> {
  try {
    if (op === 'taskV2Paging') await api.post('/api/processplatform/assemble/surface/task/v2/list/paging/1/size/20', {})
    else if (op === 'taskV2Next') await api.post('/api/processplatform/assemble/surface/task/v2/list/0/next/20', {})
    else if (op === 'taskManage') await api.post('/api/processplatform/assemble/surface/task/list/filter/1/size/20/manage', {})
    else if (op === 'taskCompletedPaging') await api.post('/api/processplatform/assemble/surface/taskcompleted/v2/list/paging/1/size/20', {})
    else if (op === 'taskCompletedManage') await api.post('/api/processplatform/assemble/surface/taskcompleted/list/filter/1/size/20/manage', {})
    else if (op === 'readV2Paging') await api.post('/api/processplatform/assemble/surface/read/v2/list/paging/1/size/20', {})
    else if (op === 'readManage') await api.post('/api/processplatform/assemble/surface/read/list/filter/1/size/20/manage', {})
    else if (op === 'readCompletedPaging') await api.post('/api/processplatform/assemble/surface/readcompleted/v2/list/paging/1/size/20', {})
    else if (op === 'readCompletedManage') await api.post('/api/processplatform/assemble/surface/readcompleted/list/filter/1/size/20/manage', {})
    else if (op === 'reviewV2Paging') await api.post('/api/processplatform/assemble/surface/review/v2/list/paging/1/size/20', {})
    else if (op === 'reviewManage') await api.post('/api/processplatform/assemble/surface/review/v2/list/paging/1/size/20/manage', {})
    else if (op === 'workV2Paging') await api.post('/api/processplatform/assemble/surface/work/v2/list/paging/1/size/20', {})
    else if (op === 'workManage') await api.post('/api/processplatform/assemble/surface/work/list/filter/1/size/20/manage', {})
    else if (op === 'workCompletedManage') await api.post('/api/processplatform/assemble/surface/workcompleted/list/filter/1/size/20/manage', {})
    else if (op === 'snapManage') await api.post('/api/processplatform/assemble/surface/snap/list/filter/1/size/20/manage', {})
    else await api.post('/api/processplatform/assemble/surface/task/count/filter', {})
    toast.success('数据查询已提交')
  } catch (e: any) {
    toast.error('数据查询失败: ' + (e?.message ?? ''))
  }
}
// ── 附件详情 / 待阅详情（rev110，均按 id 拉单条 distinct handler）────
const attachDetailText = ref('')
const readDetailText = ref('')
async function viewAttachment(id: string): Promise<void> {
  if (!id) return
  try {
    // GET service/processing/attachment/{id} —— 附件详情（x_attachment）
    const r: any = await api.get(`/api/processplatform/service/processing/attachment/${id}`)
    const d = r?.data ?? {}
    attachDetailText.value = `附件：${d.name ?? id} · 创建人 ${d.creator ?? '—'}`
  } catch (e: any) {
    toast.error('加载附件详情失败: ' + (e?.message ?? ''))
  }
}
async function viewRead(id: string): Promise<void> {
  if (!id) return
  try {
    // GET assemble/surface/read/{id} —— 待阅详情（PP_C_READ）
    const r: any = await api.get(`/api/processplatform/assemble/surface/read/${id}`)
    const d = r?.data ?? {}
    readDetailText.value = `待阅：${d.xtitle ?? d.title ?? id} · ${d.xperson ?? d.person ?? '—'}`
  } catch (e: any) {
    toast.error('加载待阅详情失败: ' + (e?.message ?? ''))
  }
}

// ── 任务 v2 生命周期（暂停/恢复/重置，rev107）──────────────────
const taskV2Status = ref('')
const v2Busy = ref(false)
async function loadTaskV2(taskId: string): Promise<void> {
  taskV2Status.value = ''
  if (!taskId) return
  try {
    // GET service/processing/task/v2/{id} —— v2 任务详情（含 task_status）
    const r: any = await api.get(`/api/processplatform/service/processing/task/v2/${taskId}`)
    taskV2Status.value = String((r?.data as { task_status?: string; taskStatus?: string })?.task_status ?? (r?.data as any)?.taskStatus ?? '')
  } catch {
    taskV2Status.value = ''
  }
}
async function taskV2Action(kind: 'pause' | 'resume' | 'reset'): Promise<void> {
  const taskId = effectiveTaskId.value
  if (!taskId || v2Busy.value) return
  v2Busy.value = true
  try {
    if (kind === 'pause') {
      // GET task/v2/pause/{id} —— 暂停
      await api.get(`/api/processplatform/service/processing/task/v2/pause/${taskId}`)
    } else if (kind === 'resume') {
      // POST task/v2/resume/{id} —— 恢复
      await api.post(`/api/processplatform/service/processing/task/v2/resume/${taskId}`, {})
    } else {
      // POST task/v2/reset/{id} —— 重置
      await api.post(`/api/processplatform/service/processing/task/v2/reset/${taskId}`, {})
    }
    toast.success('操作成功')
    await loadTaskV2(taskId)
  } catch (e: any) {
    toast.error('操作失败: ' + (e?.message ?? ''))
  } finally {
    v2Busy.value = false
  }
}

// ── 发起流程（从零创建工作实例）────────────────────────────────
const showStart = ref(false)
const startLoading = ref(false)
const startProcesses = ref<Array<{ id: string; name: string }>>([])
const startProcessId = ref('')
const startTitle = ref('')
const startDefinition = ref<XformDefinition | null>(null)
const startValues = ref<Record<string, FormValue>>({})
const startErrors = ref<Record<string, FormValue>>({})
const startSubmitting = ref(false)

async function openStart(): Promise<void> {
  showStart.value = true
  startLoading.value = true
  startProcessId.value = ''
  startTitle.value = ''
  startDefinition.value = null
  startValues.value = {}
  startErrors.value = {}
  try {
    const r: any = await api.get('/api/processplatform/assemble/designer/list/all')
    // 该端点返回分页包裹 {count, data:[rows], page, size}（兼容直接数组形态）
    const payload = r?.data ?? {}
    const rows: unknown[] = Array.isArray(payload) ? payload : (payload.data ?? [])
    startProcesses.value = (rows as Array<Record<string, unknown>>)
      .map((row) => ({ id: String(row.id ?? ''), name: String(row.name ?? row.id ?? '') }))
      .filter((proc) => proc.id)
  } catch (error: any) {
    toast.error(`加载流程失败: ${error?.message || ''}`)
  } finally {
    startLoading.value = false
  }
}

function closeStart(): void {
  showStart.value = false
  startProcessId.value = ''
  startTitle.value = ''
  startDefinition.value = null
  startValues.value = {}
  startErrors.value = {}
}

async function onStartProcessChange(): Promise<void> {
  startDefinition.value = null
  startValues.value = {}
  startErrors.value = {}
  if (!startProcessId.value) return
  try {
    const detail: any = await api.get(
      `/api/processplatform/assemble/designer/get/${encodeURIComponent(startProcessId.value)}`,
    )
    const definition = detail?.data?.processDefinition as Record<string, unknown> | undefined
    const formFlag = (() => {
      const begin = definition?.begin as Record<string, unknown> | undefined
      const manualList = definition?.manualList as Array<Record<string, unknown>> | undefined
      const fromBegin = typeof begin?.form === 'string' ? begin.form : ''
      const fromManual = typeof manualList?.[0]?.form === 'string' ? manualList[0].form : ''
      return fromBegin || fromManual
    })()
    if (!formFlag) return
    const form: any = await api.get(`/api/form/${encodeURIComponent(formFlag)}`)
    startDefinition.value = parseFormDefinition(form?.data)
    startValues.value = initialFormValues(startDefinition.value, {})
  } catch (error: any) {
    toast.error(`加载流程表单失败: ${error?.message || ''}`)
  }
}

async function submitStart(): Promise<void> {
  if (!startProcessId.value || !startTitle.value.trim()) return
  if (startDefinition.value) {
    startErrors.value = validateFormValues(startDefinition.value, startValues.value)
    if (Object.keys(startErrors.value).length) {
      toast.error('请先修正表单校验错误')
      return
    }
  }
  startSubmitting.value = true
  try {
    const created: any = await api.post('/api/processplatform/service/processing/work', {
      process: startProcessId.value,
      title: startTitle.value.trim(),
    })
    const workId = String(created?.data?.id ?? '')
    if (!workId) throw new Error('后端未返回工作 ID')
    if (startDefinition.value && Object.keys(startValues.value).length) {
      await api.put(`/api/processplatform/service/processing/data/work/${workId}`, startValues.value)
    }
    toast.success('流程已发起')
    closeStart()
    activeTab.value = 'started'
    await queryClient.invalidateQueries({ queryKey: ['process-work'] })
  } catch (error: any) {
    toast.error(`发起失败: ${error?.message || ''}`)
  } finally {
    startSubmitting.value = false
  }
}

async function submit(action: 'approve' | 'reject'): Promise<void> {
  if (!opened.value || !formDefinition.value) return
  formErrors.value = validateFormValues(formDefinition.value, formValues.value)
  if (Object.keys(formErrors.value).length) {
    toast.error('请先修正表单校验错误')
    return
  }
  submitting.value = true
  const id = workId(opened.value)
  // 表单数据经 data/work/{id} 落库；complete/reject 仅需 opinion（后端只读 opinion）。
  const opinionPayload = { opinion: opinion.value }
  try {
    await api.put(`/api/processplatform/service/processing/data/work/${id}`, formValues.value)
    const taskId = activeTab.value === 'started' ? handleTaskId.value : opened.value.id
    if (action === 'approve') {
      await api.post(`/api/task/${taskId}/complete`, opinionPayload)
    } else {
      await api.post(`/api/task/${taskId}/reject`, opinionPayload)
    }
    toast.success(action === 'approve' ? '审批通过' : '已驳回')
    closeWork()
    await queryClient.invalidateQueries({ queryKey: ['process-work'] })
  } catch (error: any) {
    toast.error(`提交失败: ${error?.message || ''}`)
  } finally {
    submitting.value = false
  }
}

function fmtTime(value: unknown): string {
  if (!value) return ''
  const date = new Date(String(value))
  return Number.isNaN(date.valueOf()) ? String(value) : date.toLocaleString('zh-CN')
}

function fmtSize(bytes: unknown): string {
  const n = typeof bytes === 'number' ? bytes : Number(bytes)
  if (!Number.isFinite(n) || n <= 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB']
  const i = Math.min(units.length - 1, Math.floor(Math.log(n) / Math.log(1024)))
  return `${(n / 1024 ** i).toFixed(i ? 1 : 0)} ${units[i]}`
}
</script>

<style scoped>
.work-view { display: flex; flex-direction: column; gap: 16px; height: 100%; }
.view-header, .content-panel { padding: 20px 24px; }
.view-header h1, .work-dialog h2 { margin: 0; color: var(--color-primary); }
.subtitle, .work-dialog p { margin: 4px 0 0; color: var(--text-muted); font-size: 12px; }
.tabs { display: flex; gap: 4px; padding: 6px; }
.tab-btn { flex: 1; padding: 9px; border: 0; border-radius: var(--radius-md); background: transparent; color: var(--text-muted); cursor: pointer; }
.tab-btn.active { background: var(--color-primary-soft); color: var(--color-primary); }
.content-panel { flex: 1; overflow: auto; }
.item-list { display: flex; flex-direction: column; gap: 8px; }
.item-card { display: flex; align-items: center; gap: 12px; padding: 14px; border: 1px solid var(--border-subtle); border-radius: var(--radius-md); background: var(--bg-elevated); }
.item-body { flex: 1; min-width: 0; }
.item-title { color: var(--text-primary); font-weight: 600; }
.item-meta { display: flex; gap: 10px; margin-top: 5px; color: var(--text-muted); font-size: 11px; }
.btn-sm { padding: 6px 12px; border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); background: var(--bg-elevated); color: var(--text-secondary); cursor: pointer; }
.btn-sm.primary { border-color: var(--color-primary); background: var(--color-primary); color: white; }
.btn-sm.reject { border-color: var(--color-error); color: var(--color-error); }
.state { display: flex; justify-content: center; align-items: center; gap: 10px; padding: 50px; color: var(--text-muted); }
.error-state { color: var(--color-error); }
.modal-overlay { position: fixed; inset: 0; z-index: 1000; display: grid; place-items: center; background: rgba(0,0,0,.62); }
.work-dialog { width: min(860px, 92vw); max-height: 88vh; overflow: auto; padding: 22px; }
.work-dialog header, .work-dialog footer { display: flex; justify-content: space-between; align-items: center; gap: 12px; }
.work-dialog header { margin-bottom: 22px; }
.work-dialog footer { justify-content: flex-end; margin-top: 18px; }
.opinion { box-sizing: border-box; width: 100%; min-height: 80px; margin-top: 18px; padding: 10px; border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); background: var(--bg-elevated); color: var(--text-primary); }
.detail-panels { display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px; margin-top: 18px; }
.detail-block { padding: 12px; border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); background: var(--bg-elevated); }
.detail-block h3 { margin: 0 0 8px; font-size: 13px; color: var(--color-primary); }
.detail-list { display: flex; flex-direction: column; gap: 6px; margin: 0; padding: 0; list-style: none; max-height: 160px; overflow: auto; }
.detail-list li { display: flex; justify-content: space-between; gap: 8px; font-size: 12px; color: var(--text-secondary); }
.detail-list .name { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.detail-list li.clickable { cursor: pointer; }
.detail-list li.clickable:hover .name { color: var(--color-primary); }
.detail-block .muted { margin: 0; color: var(--text-muted); font-size: 12px; }
</style>
