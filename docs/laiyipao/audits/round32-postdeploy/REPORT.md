# R32 部署后独立回归报告（Wiki 技能机制）

## 第一节 结论

判定 PASS。本次在现有产物上另起独立真实 Chromium 会话，经 390/768/1280 三视口对公开配置逐字段对账 24 项技能、CTA 真点击、搜索聚焦、深链与 R31 三字段回归，全部实测通过；8 张新截图 SHA 两两不同，7 张与部署前 QA 图逐字节不同（1 张静态合成技能页同像，见第八节诚实边界）。后端与 H5 产物哈希沿用未回退。

## 第二节 环境与产物基线

- 服务：API 8080 / H5 5174 / admin 5175 三端健康探测全 200（healthz、readyz、首页）。
- 后端产物 lyp-server-amd64 SHA256 前缀 3d82635f（R21 沿用，未重建）。
- H5 产物 index-DCBbR3t9.js SHA256 前缀 aba4256c（R30 沿用，未重建）。
- admin 新产物 WikiView-UYmPI2LH.js SHA256 前缀 e712bed，含类型/穿透/范围/连锁四字段与 AI 结论文案。
- 公开配置 24 技能、7 反应，版本前缀 6237e0b6。
- 本次仅恢复开发服务进程，不重建、不回退任何产物。

## 第三节 三视口 24 技能对账

- 独立会话分别以 390x844、768x1024、1280x800 打开 Wiki 技能页，注入管理令牌后重载。
- 每行 13 列：ID/名称/元素/系别/伤害/热量/冷却/类型/穿透/范围/连锁/层数/说明，与公开配置逐项一致。
- 三视口 reconcile 结果：ROWS=24，MISMATCHES=0，AI_OK=True，BAD=False，DOC 宽度等于视口宽度无横向溢出，CTA 链接为 hash 路由安全的合成技能锚点，ALL_RECONCILE_OK=True。
- AI 概览文案由配置聚合：共 24 个技能，主动 22 个，被动 2 个，多目标 20 个。
- 表格容器可内部滚动，末列在窄屏不被裁切。

## 第四节 CTA、搜索与深链

- CTA：先用 eval 记录前置 hash 为技能页；通过 CLI 真实点击合成技能链接，RC=0；后置 hash 变为技能页加 anchor=composite，atComposite=true，anchorComposite=true，dashboard=false。截图亲读显示合成技能表（ID 31 至 45），非登录页、非看板。
- 搜索：聚焦搜索框、输入温压弹并回车，RC=0；探针 focused 为 skill:3，输入值温压弹，hash 含 focus=skill:3；截图亲读 ID3 行高亮。
- 深链：直接打开敌人页，RC=0；hash 为 tab=enemies，导航高亮敌人抗性图鉴，敌人区可见，docOverflow=0；截图亲读敌人抗性表。

## 第五节 R31 不回退

- 反应页独立探针：7 行反应，表头 11 列，含层数传播、削甲、击退三字段（索引 7、8、9）。
- 真实值抽样：传播 1 层、削甲 150‰、击退 4px，其余行为占位横线，三字段非空可用。
- 截图亲读为反应机制页与 AI 运营概览，非登录页。

## 第六节 截图证据与 SHA256

8 张新图，SHA256 全 64 位两两不同：
- r32post-390-skills = 7711f70b1795b4867e97094cac842f4e90fb1a91c2ca730cd80197d1747f87fb
- r32post-768-skills = 45436d40a8460378517df8e84f37bb6388754251c9508f9939c48a5b4d1dd8d8
- r32post-1280-skills = cda82d35525c17b2b8299b57c28a26b56cf832f8b96fd70c44302ea6cc668260
- r32post-cta-before = 2328050d88bcd172ec33fccdde4e1303a354e6694c1c4f029b7fe4a8363e6cfd
- r32post-cta-after = 1d7849927bd7cd608beaa175904df2f1332f437269563bedd91db5f2e1929836
- r32post-search = 4b14dbd3ded6ea19565133c1541e60400dbbfca4cbd85063a7b0db9345e12126
- r32post-deeplink = 9c3d277edb40733e8c5113e764c81ffbbebe6ac44ac118a8fc4ad469e153de18
- r32post-r31-reactions = e109b3b6fee443ece8669584bee2b7404e47a1c2151e923c99170f2d0e7c7d71

与部署前 QA 图比对：7 张逐字节不同；cta-after 与部署前合成技能锚点图同像，属同一静态页面确定性像素（诚实边界见第八节）。本次全部截图均为新会话新采，非复用 QA 原始图。

## 第七节 本地门禁与零真值

- admin 目录全量测试 71 文件 795 用例全过（以九万毫秒超时消除并发毛刺）。
- Vue 类型检查与构建退出码 0；Go 领域守卫 TestNoInfraIdentifiersInRepo PASS。
- TestNoRealSecretsInRepo 因本机缺少真值密钥文件而打印未执行提示后跳过，未判为通过（见第八节）。
- 三份浏览器脚本语法检查通过。
- 本次入库文件以确定性扫描逐项检查，未发现令牌、私钥、带口令连接串、真实主机地址或个人目录等真值；管理令牌临时文件仅存在于被忽略的数据目录，提交前由收尾脚本删除。
- QA 数据库在收尾脚本中还原至回归前基线。

## 第八节 诚实边界

- CTA after 图与部署前合成技能图 SHA 相同：同一静态页面确定性像素，非复用旧图；CTA 生效由独立会话的真实 CLI 点击与前后 hash 变化证据坐实。
- TestNoRealSecretsInRepo 本机跳过，零真值结论来自静态扫描而非该守卫，不冒充其通过。
- 早期会话曾因服务进程中断产生空白页证据，已全部作废并在服务恢复后重采；本报告的证据均为恢复后新采。
- 终端对中文输出偶见乱码，属 shell 编码显示问题，JSON 原始字节与 Python ascii 转义复核一致，不影响判定。
