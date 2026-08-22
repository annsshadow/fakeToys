## 关于 Linux -stable 发布版本，你想知道的一

关于哪些补丁会被接收进stable”树、哪些不会的规则
- 它（或其等效修复）必须已经存在于 Linux 主线（upstream）中- 它必须显然正确且经过测试- 含上下文在内，它不能大于 100 行- 它必须遵  Documentation/process/submitting-patches.rst <submittingpatches>
  规则- 它必须要么修复一个困扰人们的真实 bug，要么只是添加一个设ID  对前者进一步说明：

  - 它修复诸oops、挂起、数据损坏、真实安全问题、硬件怪癖、编译错误（但不适用于标记为
    CONFIG_BROKEN 的项），或某些“哦，那不太好”的问题  - 由发行版内核用户报告的严重问题，如果它们修复了显著的性能或交互性问题，也可能被考虑    由于这些修复不那么显而易见，且存在引入微妙回归的较高风险，它们只应由发行版内    维护者提交，并包含一份附录，链接bugzilla 条目（若存在）以及关于用户可见影响的额外信息  - 不接受“这可能是个问题……”之类的东西，例如“理论上的竞态条件”，除非同时提供    关于bug 如何被利用的解释  - 不接受对用户无益的“琐碎”修复（拼写改动、空白清理等）

### -stable 树提交补丁的流程


   安全补丁不应（仅仅）-stable 审核流程处理，而应遵循
   Documentation/process/security-bugs.rst <securitybugs>
   中的流程
-stable 树提交变更有三种方式
1. 在你随后为主线程提交的补丁描述中添加一个“stable 标签”2. 请求 stable 团队拾取一个已经合入主线的补丁3. stable 团队提交一个补丁，它等效于一个已经合入主线的变更
以下各节详细描述每种方式
option_1 **强烈**推荐，它最简单也最常见option_2 主要用于提交时未考虑向后移植的情况。option_3 是前两种方式的替代方案，
适用于需要将已合入主线的补丁做调整以应用到更旧的系列（例如由API 变更）的情况
使用 option 2 3 时，你可以请求将你的变更包含进特定的 stable 系列。这样做时，
要确保该修复或其等效修复在所有仍受支持的、更新的 stable 树上都是可应用、已提交或已存在的。这旨在
防止用户后续在更新时遇到的回归，例如一个为 5.19-rc1 合并的修复被向后移植5.10.y，却未移植到 5.15.y

######## Option 1


要使你为主线程提交的补丁在之后被自动拾取，请使用
```
  Cc: stable@vger.kernel.org
```
修复未公开漏洞时，改用 `Cc: stable@kernel.org`：它会降低通过
'git send-email' 意外将修复暴露给公众的机会，因为发往该地址的邮件不会被投递到任何地方
一旦该补丁合入主线，它将被应用stable 树，无需作者或子系统维护者做任何其他事情
要向 stable 团队发送额外指令，可使shell 风格inline 注释来传递任意或预定义的说明
```
    Cc: <stable@vger.kernel.org> # 3.3.x: a1f84a3: sched: Check for idle
    Cc: <stable@vger.kernel.org> # 3.3.x: 1b9508f: sched: Rate-limit newidle
    Cc: <stable@vger.kernel.org> # 3.3.x: fd21073: sched: Fix affinity logic
    Cc: <stable@vger.kernel.org> # 3.3.x
    Signed-off-by: Ingo Molnar <mingo@elte.hu>

  标签序列的含义为::

    git cherry-pick a1f84a3
    git cherry-pick 1b9508f
    git cherry-pick fd21073
    git cherry-pick <this commit>

  注意，对于一个补丁系列，你不必将该系列本身存在的补丁列为前置依赖。例如，如果你有如下
  补丁系列::

    patch1
    patch2

  其中 patch2 依赖patch1，如果你已经patch1 标记为要纳入 stable，则不必patch1
  列为 patch2 的前置依赖
```
```
    Cc: <stable@vger.kernel.org> # 3.3.x

  该标签的含义:

    git cherry-pick <this commit>

  针对每个以指定版本开始的stable”树
  注意，如stable 团队能从 Fixes: 标签推导出合适的版本，则此类标记是不必要的
```
```
    Cc: <stable@vger.kernel.org> # after -rc3
```
```
    Cc: <stable@vger.kernel.org> # see patch description, needs adjustments for <= 6.3
```
此外还有一stable 标签的变体，可用于让 stable 团队的向后移植工具（例如 AUTOSEL 或查找提交的脚本
```
     Cc: <stable+noautosel@kernel.org> # reason goes here, and must be present
```


######## Option 2


如果补丁已经合入主线，请发送一封邮件到
stable@vger.kernel.org，其中包含补丁的主题、提ID你认为它应被应用的原因，以及你希望它应用到的内核版本

######## Option 3


在验证补丁遵循上述规则后，将补丁发送到
stable@vger.kernel.org，并说明你希望它应用到的内核版本。这样做时，
你必须在变更日志中注明上游提ID
```
  commit <sha1> upstream.
```
```
  [ Upstream commit <sha1> ]
```
如果所提交的补丁偏离了原始上游补丁（例如因为它不得不为适应更旧API 而做调整），这必须在补丁描述非常清楚地记录并说明理由

### 提交之后


当补丁被接收进队列时，发送者将收到一ACK；若补丁被拒绝，则收NAK。根stable 团队成员日程安排，此响应可能需要几天时间
如果被接受，该补丁将被加-stable 队列，供其他开发者及相关子系统维护者审核

### 审核周期


- -stable 维护者决定进行一次审核周期时，补丁将被发送给审核委员会、受影响的补丁区域维护  （除非提交者就是该区域的维护者），并抄送（CC）到 linux-kernel 邮件列表- 审核委员会有 48 小时ACK NAK 该补丁- 如果补丁被委员会某成员拒绝，linux-kernel 成员提出维护者和委员未意识到的意见，
  该补丁将从队列中丢弃- ACK 的补丁将作为候选发布（-rc）的一部分再次发布，供开发者和测试者测试- 通常只发布一-rc 版本，但如果有任何未决问题，某些补丁可能会被修改或丢弃，或额外的补丁被加入队列  随后会发布并测试额外-rc 版本，直到没有问题被发现- -rc 版本的响应可以在邮件列表上通过发送带有测试信息的“Tested-by:”邮件来完成。这些“Tested-by:  标签会被收集并添加到发布提交中- 在审核周期结束时，新-stable 发布版本将被发布，其中包含所有已排队并经过测试的补丁- 安全补丁将直接从内核安全团队被接收进 -stable 树，而不经过正常的审核周期  有关此流程的更多详情，请联系内核安全团队

### 代码

- 已完成的版本和进行中的版本的修补队列可在以下位置找到
    https://git.kernel.org/pub/scm/linux/kernel/git/stable/stable-queue.git

- 所stable 内核的最终化并打标签的发布版本可在每个版本独立分支中找到
    https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git

- 所stable 内核版本的候选发布版本可在以下位置找到：

    https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux-stable-rc.git/

```
     -stable-rc 树是 stable-queue 树在某个时刻的快照，会频繁变动，因此会经常被变基。它只应
     用于测试目的（例如供 CI 系统消费）
```


### 瀹℃牳濮斿憳浼。

- 它由若干自愿承担此任务的 kernel 开发者，以及少数非自愿的开发者组成