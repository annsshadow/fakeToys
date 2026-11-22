# oa4rust R358 H5 文档页双来源清单只读审计

> 身份：qa-e2e｜轮次：R358 / t169｜性质：只读甄别（不改源码/契约/后端/SDK）
> 测试环境：<YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST>，测试服部署；浏览器 390x844 headless（isMobile/hasTouch，DPR2），非物理真机，截图均为真浏览器实测渲染而非真机截屏。
> 凭据：运行期经 ssh 读取，不回显、不入库；API 前缀 /oa4rust-api。

## 1. 任务与结论

甄别 H5 文档页 mobile/src/pages/doc/doc.vue 的两个独立来源（附件存储 FILE_FILE 与文档库 x_file）清单解包、并发加载、失败/空态文案、附件上传回显是否存在解包失配或状态缺陷。

最终判定：0 个真缺陷（通过）。两来源均单层裸数组 data 解包正确，无 R340/R351 分组信封型缺陷；docMetaSize 兼容 size/length；上传真实回显；失败与空态文案可区分；0 pageerror。

## 2. 覆盖甄别

- docs/ops 对 doc.vue / docFeed / 双来源清单命中 2 篇：oa4rust-h5-empty-state-guide-r300.md（R300 空态样式基线）、oa4rust-r352-h5-search-envelope-fix.md（搜索页信封修复），均非本页双来源真浏览器专项。
- R258/R269 为上传刷新与并发守卫单测，R314 为 PC FileManager（415），R317 为源码级静态梳理，均未做 H5 doc.vue 双来源真实信封渲染与上传回显实测。
- 本轮为 doc.vue 双来源首次真浏览器专测，属有效增量。

## 3. 源码梳理

### 3.1 页面链（doc.vue，389 行）

- load() :46-63：unique = session.user?.unique（:48，空则不请求避免无谓 401）；loadDocLists 并发拉两来源（:53-56），回填 rows/attachRows 与 filesError/attachError（:57-60）。
- filesView / attachView :21-26：listState({loading, loaded, count, error}) 统一状态机。
- onShow :65-68：ensureAuthenticated 后 load()。
- doUpload :118-139：uploading 守卫（:119）+ folder 检查（:121-123）+ fileApi.upload（:128）+ toast 上传成功（:130）+ await load() 即时刷新（:132）+ catch toast 错误（:135）+ finally uploading=false。
- pickFromAlbumOrCamera :141-150 用 uni.chooseImage；pickFromFilePicker :152-168 在 H5 回退图片选择（uni.chooseMessageFile H5 不可用）。
- 模板两 section：附件（FILE_FILE，上传落点）:186-219，空态「还没有附件 / 拍照或上传文件后，附件会显示在这里」；文件（x_file）:220+，空态「文档库暂无文件 / 上传一份文件，或刷新查看最新文档」；error 且 rows>0 时 error-banner + 重试。
- downloadAndOpen :80-96 / downloadAttachment :99-115：仅 downloadFile status 200 才 openDocument。

### 3.2 工具层

- docFeed.ts（41 行）：docMetaSize = formatFileSize(row.size ?? row.length)（:13-15，兼容 x_file 的 size 与 FILE_FILE 的 length）；loadDocLists（:32-41）Promise.all 并发两 loadList，label 分别「文件加载失败」「附件加载失败」。
- loadList.ts（59 行）：asRows（:39-41）仅 Array.isArray(data) 才返回数组否则 []；loadList（:43-59）try fetchList 成功置 ok:true，catch 置 ok:false 并保留 previousRows + describeLoadError（:55-57，刷新失败不丢旧行）；listState（:32-37）优先级 loading>error>empty>list；describeLoadError（:25-29）返回 label + 冒号 + detail。

### 3.3 服务层（services/index.ts）

- fileApi（:220）：fileList = list(mapi.get(/api/file/assemble/control/file/list/{folderId}))（:222）；upload = mapi.upload(/api/attachment/upload/folder/{folderId})（:230-231）；attachmentList = list(mapi.get(/api/attachment/list/editor/{owner}))（:236）。list()（:24-26）仅类型断言不解包。
- 传输层 2xx resolve(envelope)、非 2xx reject，故失败必带 HTTP 状态，端点不产生 200 + type:error 无 data 形状。

## 4. 真浏览器实测

环境：390x844 headless，isMobile/hasTouch，DPR2，非物理真机；两轮脚本（首轮只读+失败/空注入，v2 真实上传回显），0 pageerror。

| 场景 | 方法 | 结果 |
| --- | --- | --- |
| 信封探针 | 页面内 fetch 直调两来源 | file/list=200 dataIsArray=true 0 条；attachment/list/editor=200 dataIsArray=true 17 条（字段 id/name/length/person，length=0）；单层裸数组，asRows 解包正确 |
| 首屏渲染 | 真实 UI | 附件 17 行、文件 section 空态「文档库暂无文件」；附件 length=0 显示「0 B」（docMetaSize 兼容生效） |
| 失败注入 | route 注入两来源 HTTP 500 | 附件 section「附件加载失败：HTTP 500」+ 重试；文件 section「文件加载失败：HTTP 500」+ 重试；与空态可区分 |
| 空注入 | route 注入 data=[] | 附件「还没有附件 / 拍照或上传文件后，附件会显示在这里」；文件「文档库暂无文件」；errTip=0 |
| 真实上传回显 | filechooser + setFiles(70B PNG) 点「拍照 / 相册」 | toast「上传成功」；附件 17→18；新条目「r358-probe.png 70 B · 时间戳」即时回显（length 字段显示 70 B）；截图肉眼坐实 |
| 并发守卫 | 上传期间采样按钮态 | doUpload uploading 守卫源码确证；脚本采样为非上传瞬间未抓到 disabled 态，列为取证限制 |

response 事件监听回调在两轮脚本的 api 数组未落地（页面内探针 fetch 直接执行 status=200 有效），报告以探针原始 body 与 UI 渲染为准。

## 5. 三分判定

### 5.1 真缺陷（0）

无。两来源单层解包、并发渲染、size/length 兼容、上传即时回显、失败/空态文案五项均通过；0 pageerror。

### 5.2 取证限制

- downloadAndOpen / downloadAttachment 仅验证入口与 status=200 才 openDocument 的条件，未真下载/外部打开（破坏性/外部应用打开未授权）。
- 上传并发双击守卫仅源码确证 + 非上传瞬间按钮态采样，未即时抓到 uploading=true 时的 disabled。
- 文件（x_file）来源当前 0 条，其有数据时的 size 字段显示路径未实测；docMetaSize 的 size 兼容由源码与附件 length 兼容共同确证。

### 5.3 产品语义差异

- uni.chooseMessageFile 在 H5 不可用，回退图片选择属平台设计（非缺陷）。
- loadList catch 保留 previousRows（刷新失败不丢旧行）属容错取舍，配合 error-banner 提示，非缺陷。

## 6. 证据清单（gitignored，未入库）

- test-results/r358-h5-doc-dual-source/evidence.json + evidence-v2.json。
- 截图：doc-initial.png / doc-fail500.png / doc-empty.png / doc-v2-uploaded.png，均 390x844 headless 非物理真机。
- 上传探针文件 r358-probe.png（70B）。
- 脚本：r358-h5-doc-dual-source.cjs / -v2.cjs 及对应 run.sh。

## 7. 复现方法

测试服部署后，以真浏览器 390x844（isMobile/hasTouch）登录进入文档页，核对附件/文件两 section 渲染；用 route 注入两来源 500 与空数组验证失败/空态；点「拍照 / 相册」触发 filechooser 上传小文件验证即时回显。所有域名/主机以占位符表示，真值仅运行期从环境读取。