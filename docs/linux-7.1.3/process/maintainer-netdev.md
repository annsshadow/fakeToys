

## 网络子系统（netdev）


### tl;dr


 - 将你的补丁指定到一个树 - `[PATCH net]` 或 `[PATCH net-next]`
 - 对于修复，无论哪个树，`Fixes:` 标签都是必需的
 - 不要发布大型系列（> 15 个补丁），将其拆分
 - 不要在 24 小时内重复发布你的补丁
 - 倒圣诞树（reverse xmas tree）顺序

### netdev


netdev 是一个针对所有网络相关 Linux 内容的邮件列表。这包括 Linux 源码树中 net/
（即核心代码如 IPv6）和 drivers/net（即硬件特定驱动）下的任何内容。

注意，某些流量很大的子系统（例如无线驱动）有自己的特定邮件列表和树。

像许多其他 Linux 邮件列表一样，netdev 列表托管在 kernel.org，归档位于
https://lore.kernel.org/netdev/。

除了上述这类子系统外，所有网络相关的 Linux 开发（即 RFC、审查、评论等）都在
netdev 上进行。

### 开发周期


这里提供一些关于 Linux 开发节奏的背景信息。每个新版本以一个为期两周的“合并窗口”
开始，期间主维护者将他们的新东西交给 Linus 合并到主线树。两周之后，合并窗口关闭，
被称为/打上标签 `-rc1`。此后不再有新的特性进入主线——只预期对 rc1 内容的修复。
在收集了大约一周对 rc1 内容的修复之后，发布 rc2。这以大约每周一次的频率重复，直到
rc7（通常如此；如果一切平静有时是 rc6，如果处于混乱状态有时是 rc8），而在最后一个
vX.Y-rcN 完成一周后，官方的 vX.Y 发布。

要了解我们当前处于周期的哪个阶段——加载主线（Linus）页面：

  https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git

并注意“tags”部分的顶部。如果是 rc1，则说明处于开发周期的早期。如果一周前被打上了
rc7 标签，那么很可能发布在即。如果最近的标签是一个最终发布标签（没有 `-rcN` 后缀）——
我们很可能处于合并窗口中，并且 `net-next` 已关闭。

### git 树与补丁流程


这里涉及两个网络树（git 仓库）。两者都由主网络维护者 David Miller 驱动。有 `net`
树和 `net-next` 树。正如你可能从名字中猜到的，`net` 树用于修复 Linus 主线树中已有的
现有代码，而 `net-next` 是未来版本新代码去往的地方。你可以在这里找到这些树：

- https://git.kernel.org/pub/scm/linux/kernel/git/netdev/net.git
- https://git.kernel.org/pub/scm/linux/kernel/git/netdev/net-next.git

将其与内核开发联系起来：在两周合并窗口开始时，`net-next` 树将关闭——不再有新更改/
特性。过去约 10 周累积的新内容将通过针对 vX.Y 的拉取请求传递给主线/Linus——同时，
`net` 树将开始累积与此被拉取内容相关的、针对 vX.Y 的修复

通常会向 netdev 发送一个指示 `net-next` 何时已关闭的公告，但了解了上述内容，你可以
提前预料到这一点。

  在 `net-next` 树关闭期间，不要向 netdev 发送新的 `net-next` 内容。

仅用于审查的 RFC 补丁显然随时欢迎（对 `git format-patch` 使用
`--subject-prefix='RFC net-next'`）。

两周过去后不久（并且 vX.Y-rc1 已发布），`net-next` 树重新打开以收集下一个（vX.Y+1）
版本的内容。

如果你没有订阅 netdev 和/或只是不确定 `net-next` 是否已经重新打开，只需查看上面的
`net-next` git 仓库链接，看是否有任何新的网络相关提交。你还可以查看以下网站了解
当前状态：

  https://netdev.bots.linux.dev/net-next.html

`net` 树继续累积针对 vX.Y 内容的修复，并定期（约每周）回馈给 Linus。这意味着 `net`
的重点在于稳定和错误修复。

最后，vX.Y 发布，整个周期重新开始。

### netdev 补丁审查



#### 补丁状态


可以通过查看 netdev 的主 patchwork 队列来检查补丁的状态：

  https://patchwork.kernel.org/project/netdevbpf/list/

“State”字段会准确告诉你补丁处于什么位置：

================== =============================================================
Patch state        描述
================== =============================================================
New, Under review  待审查，补丁在维护者的审查队列中；这两个状态可互换使用（取决于
                   当时处理 patchwork 的具体共同维护者）
Accepted           补丁已应用到相应的网络树，通常由 pw-bot 自动设置
Needs ACK          等待某领域专家或测试的确认
Changes requested  补丁未通过审查，预期带有适当代码和提交说明更改的新修订
Rejected           补丁已被拒绝，不预期新修订
Not applicable     补丁预期在网络子系统之外被应用
Awaiting upstream  补丁应由相应的子维护者审查和处理，他将把它发送到网络树；在 netdev
                   的 patchwork 中被设为 `Awaiting upstream` 的补丁通常会保持此状态，
                   无论子维护者是要求更改、接受还是拒绝了补丁
Deferred           补丁需要稍后重发，通常由于依赖关系或因为它针对一个已关闭的树发布
Superseded         发布了补丁的新版本，通常由 pw-bot 设置
RFC                不应被应用，通常不在维护者的审查队列中，pw-bot 可以根据主题标签
                   自动将补丁设为该状态
================== =============================================================

补丁由承载它们的电子邮件的 `Message-ID` 头索引，因此如果你难以找到自己的补丁，请将
`Message-ID` 的值附加到上面的 URL。

#### 更新补丁状态


贡献者和审查者没有直接在 patchwork 中更新补丁状态的权限。patchwork 不暴露太多关于
补丁状态历史的信息，因此让多个人更新状态会导致混乱。

netdev 不使用委托 patchwork 权限，而是使用一个简单邮件机器人，它查找发送给邮件列表
的电子邮件中的特殊命令/行。例如要将一个系列标记为 Changes Requested：

```

  pw-bot: changes-requested

```
结果机器人会将整个系列设为 Changes Requested。当作者在自己系列中发现错误并希望阻止
其被应用时，这可能很有用。

机器人的使用完全可选，如有疑问完全忽略它的存在。维护者会自己分类并更新补丁的状态。
绝不应以与机器人通信为主要目的向列表发送任何电子邮件，机器人命令应被视为元数据。

机器人的使用仅限于补丁的作者（补丁提交和命令上的 `From:` 头必须匹配！）、根据
MAINTAINERS 文件修改代码的维护者（同样，`From:` 必须与 MAINTAINERS 条目匹配）以及
少数资深审查者。

机器人在这里记录其活动：

  https://netdev.bots.linux.dev/pw-bot.html

#### 审查时间线


一般来说，补丁会被快速分流（在 48 小时内）。但要有耐心，如果你的补丁在 patchwork
中处于活动状态（即列在项目的补丁列表中），它被正确错过的概率接近于零。

netdev 上大量的开发使得审查者相对快速地从一个讨论转向下一个。在沉默一周之后，几乎
不可能再收到新的评论和回复。如果一个补丁在 patchwork 中不再活动，并且线程空闲超过
一周——澄清下一步和/或发布下一个版本。

对于 RFC 发布特别而言，如果一周内没有人回应——审查者要么错过了发布，要么没有强烈
意见。如果代码已就绪，作为 PATCH 重新发布。

只说“ping”或“bump”的电子邮件被认为是不礼貌的。如果你无法从 patchwork 或讨论进展
弄清补丁的状态：

```

  I don't understand what the next steps are. Person X seems to be unhappy
  with A, should I do B and repost the patches?

```

#### 要求更改


标记为 `Changes Requested` 的补丁需要修订。新版本应附带变更日志：

```

  [PATCH net-next v3] net: make cows go moo

  Even users who don't drink milk appreciate hearing the cows go "moo".

  The amount of mooing will depend on packet rate so should match
  the diurnal cycle quite well.

  Signed-off-by: Joe Defarmer <joe@barn.org>
  ---
  v3:
    - add a note about time-of-day mooing fluctuation to the commit message
  v2: https://lore.kernel.org/netdev/123themessageid@barn.org/
    - fix missing argument in kernel doc for netif_is_bovine()
    - fix memory leak in netdev_register_cow()
  v1: https://lore.kernel.org/netdev/456getstheclicks@barn.org/

```
提交说明应被修订，以回答审查者在前面的讨论中不得不提出的问题。有时更新提交说明将是
新版本中唯一的变化。

#### 部分重发


请始终重发整个补丁系列，并确保你的补丁有编号，以便清楚这是可以应用的最新、最完整
的一组补丁。不要试图只重发发生更改的补丁。

#### 处理被错误应用的补丁


有时一个补丁系列在收到关键反馈之前就被应用了，或者应用的系列版本是错误的。

一旦补丁被推出就无法让它消失，netdev 树中的提交历史是不可变的。请基于已合并的内容
发送增量版本，以将补丁修复成仿佛你最新的补丁系列被合并时的样子。

在需要完全回退的情况下，回退必须作为补丁提交到列表，提交说明解释被回退提交的技术
问题。回退应作为最后手段使用，当原始更改完全错误时；增量修复更受青睐。

#### 稳定树


虽然过去 netdev 提交不应携带显式的 `CC: stable@vger.kernel.org` 标签，但今天已不再
如此。请遵循 Documentation/process/stable-kernel-rules.rst <stable_kernel_rules> 中的
标准稳定规则，并确保包含适当的 Fixes 标签！

#### 安全修复


如果你认为自己发现了一个可能有安全影响的 bug，不要直接给 netdev 维护者发电子邮件。
当前的 netdev 维护者一直要求人们使用邮件列表而不是直接联系。如果你对此不满意，那么
或许考虑给 security@kernel.org 发邮件，或阅读
http://oss-security.openwall.org/wiki/mailing-lists/distros 作为可能的替代机制。


#### 协同发布对用户空间组件的更改


演练内核特性的用户空间代码应与内核补丁一起发布。这让审查者有机会看到任何新接口
如何被使用以及效果如何。

当用户空间代码位于内核仓库本身中时，所有更改通常应作为一个系列。如果系列变得太大，
或者用户空间项目不在 netdev 上审查，则包含一个可以看到用户空间补丁的公共仓库的链接。

如果用户空间工具位于单独的仓库中但在 netdev 上审查（例如对 `iproute2` 工具的补丁），
内核和用户空间补丁在发布时应形成独立的系列（线程）：

```

  [PATCH net-next 0/3] net: some feature cover letter
   └─ [PATCH net-next 1/3] net: some feature prep
   └─ [PATCH net-next 2/3] net: some feature do it
   └─ [PATCH net-next 3/3] selftest: net: some feature

  [PATCH iproute2-next] ip: add support for some feature

```
不建议作为一个线程发布，因为这会让 patchwork 困惑（截至 patchwork 2.2.2）。

#### 协同发布 selftest


selftest 应与代码更改属于同一个系列。特别是对于修复，代码更改和相关测试都应进入
同一个树（测试可能缺少 Fixes 标签，这是预期的）。不鼓励在单个提交中混合代码更改和
测试更改。

### 准备更改


注重细节很重要。像你是审查者一样重读你自己的作品。你可以从使用 `checkpatch.pl`
开始，甚至可能带上 `--strict` 标志。但这样做时不要盲目地机械执行。如果你的更改是
一个 bug 修复，请确保你的提交日志指出最终用户可见的症状、它发生的根本原因，然后在
必要时解释为什么所提议的修复是完成此事的最佳方式。不要弄乱空白，并且通常说来，不要
错误缩进跨多行的函数参数。如果这是你的第一个补丁，把它发邮件给你自己，这样你可以
测试将它应用到一棵未打补丁的树，以确认基础设施没有弄乱它。

最后，回过头阅读
Documentation/process/submitting-patches.rst <submittingpatches>
以确保你没有重复其中记录的某些常见错误。

#### 指示目标树


为了帮助维护者和 CI 机器人，你应该显式标记你的补丁针对哪个树。假设你可以使用 git，
使用前缀：

```

  git format-patch --subject-prefix='PATCH net-next' start..finish

```
在上述命令中，对于 bug 修复 `net` 内容，使用 `net` 而非 `net-next`（始终小写）。

#### 将工作拆分为补丁


设身处地为审查者着想。每个补丁是单独阅读的，因此应构成一个朝向你既定目标的可理解
步骤。

避免发送长于 15 个补丁的系列。更大的系列需要更长时间审查，因为审查者会推迟查看它，
直到他们找到一大块时间。一个小的系列可以在短时间内审查完，所以维护者直接就做了。
结果，一系列较小的系列会以更快的速度合并，并获得更好的审查覆盖。重新发布大型系列
也会增加邮件列表流量。

### 限制邮件列表上待处理的补丁数


避免跨所有系列、针对单个树在邮件列表上待审查的补丁超过 15 个。换句话说，net 上最多
15 个补丁在审查中，net-next 上最多 15 个补丁在审查中。

此限制旨在将开发者的精力集中在上游审查之前的补丁测试上。有助于提高上游提交的
质量，并减轻审查者的负担。


#### 局部变量排序（“倒圣诞树”，“RCS”）


netdev 对于函数内局部变量的排序有一个约定：

```

  struct scatterlist *sg;
  struct sk_buff *skb;
  int err, i;

```
如果变量之间存在妨碍排序的依赖关系，将初始化移到行外。

#### 格式优先级


在处理使用非标准格式化的现有代码时，让你的代码遵循最新的指南，以便最终 netdev 领域
内的所有代码都采用首选格式。

#### 使用设备管理和 cleanup.h 构造


netdev 历史上一直对所有“自动清理” API（甚至包括 `devm_` 辅助函数）的承诺持怀疑态度。
它们不是首选的实现风格，仅仅是可接受的风格。

在任何长于 20 行的函数内不鼓励使用 `guard()`，`scoped_guard()` 被认为更具可读性。
仍然（较弱地）偏好使用普通的锁/解锁。

低级清理构造（例如 `__free()`）可以在构建 API 和辅助函数时使用，特别是作用域迭代器。
然而，在网络核心和驱动中直接使用 `__free()` 是不鼓励的。类似的指导也适用于在函数
中间声明变量。

#### 清理补丁


netdev 不鼓励执行简单清理、且不在其他工作上下文中的补丁。例如：

- 处理 `checkpatch.pl` 以及其他琐碎的编码风格警告
- 处理局部变量排序<rcs>问题
- 转换为设备管理 API（`devm_` 辅助函数）

这是因为人们认为此类更改产生的扰动所带来的代价大于此类清理的价值。

相反，拼写和语法修复不被劝阻。

#### 审查后重发


在两次发布之间至少留出 24 小时。这将确保所有地理位置的审查者都有机会插话。也不要在
两次发布之间等待太久（数周），因为那会让审查者更难回忆起所有上下文。

确保你在新的发布中处理所有反馈。如果关于前一版本的讨论仍在进行中，不要发布新版本的
代码，除非审查者直接指示。

新版本的补丁应作为一个独立的线程发布，而不是作为对前一次发布的回复。变更日志应包含
指向前一次发布的链接（见 Changes requested）。

### 测试


#### 预期的测试级别


至少，你的更改必须能在设置了 `W=1` 的情况下，通过一个 `allyesconfig` 和一个
`allmodconfig` 构建而没有任何新的警告或失败。

理想情况下，你会针对你的更改进行特定于更改的运行时测试，并且补丁系列包含一组针对
`tools/testing/selftests/net` 的内核 selftest，或使用 KUnit 框架。

你应在相关的网络树（`net` 或 `net-next`）之上而不是例如稳定树或 `linux-next` 之上
测试你的更改。

#### patchwork 检查


patchwork 中的检查大多是现有内核脚本的简单包装，源码位于：

https://github.com/linux-netdev/nipa/tree/master/tests

**不要**仅仅为了运行检查而发布你的补丁。你必须通过在本地测试确保你的补丁已就绪，再
发布到邮件列表。patchwork 构建机器人实例很容易过载，如果我们可以避免，netdev@vger
实在不需要更多流量。

#### netdevsim


`netdevsim` 是一个测试驱动，可用于在不要求有能力的硬件的情况下演练驱动配置 API。
在栈中添加具有复杂逻辑的新 API 时，鼓励基于 `netdevsim` 的 Mock 和测试。测试应当
编写成既可以针对 `netdevsim` 也可以针对真实设备运行（见
`tools/testing/selftests/drivers/net/README.rst`）。仅 `netdevsim` 的测试应专注于
测试核心中难以用真实驱动演练的边界情况和失败路径。

`netdevsim` 本身**不**被视为一个用例/用户。你还必须在真实驱动中实现新 API。

我们不保证 `netdevsim` 未来不会以通常会破坏正常被视为 uAPI 的方式发生变化。

`netdevsim` 仅保留供上游测试使用，因此任何新的 `netdevsim` 特性必须附带
`tools/testing/selftests/` 下的 selftest。

### 驱动的受支持状态



netdev 为希望在 MAINTAINERS 文件中获得 `Supported` 状态的驱动定义了额外要求。`Supported`
驱动必须运行所有上游驱动测试，并每天报告两次结果。不遵守此要求的驱动应使用 `Maintained`
状态。目前 `Supported` 和 `Maintained` 驱动在上游的对待方式没有区别。

驱动为了获得 `Supported` 状态必须遵循的确切规则：

1. 必须运行 Linux selftest 的 `drivers/net` 和 `drivers/net/hw` 目标下的所有测试。
   运行并报告私有/内部测试也很受欢迎，但上游测试是必须的。

2. 最小运行频率为每 12 小时一次。必须测试来自所选分支供给（branch feed）的指定分支。
   注意分支是自动构建的，并暴露于有意的恶意补丁发布，因此测试系统必须被隔离。

3. 支持多代设备的驱动必须测试每一代中至少一个设备。一个测试床清单（确切格式待定）
   应描述所测试的设备型号。

4. 测试必须可靠运行，如果多个分支被跳过或测试由于执行环境的问题而失败，`Supported`
   状态将被撤销。

5. 由于驱动或测试本身中的 bug、或缺乏对所测试目标特性的支持而导致的测试失败，**不**
   是失去 `Supported` 状态的理由。

netdev CI 将维护一个受支持设备的官方页面，列出它们近期的测试结果。

驱动维护者可以安排其他人运行测试，不要求列为维护者的人（或其雇主）负责运行测试。
供应商之间的协作、托管 GH CI、linux-netdev 下的其他仓库等都非常受欢迎。

有关 netdev CI 的更多信息，请参阅 https://github.com/linux-netdev/nipa/wiki。如有任何
问题，随时联系维护者或列表。

### 审查者指南


强烈鼓励在列表上审查其他人的补丁，无论专业水平如何。有关一般指南和有益提示，请参见
development_advancedtopics_reviews。

可以安全地假设 netdev 维护者了解社区以及审查者的专业水平。审查者不应担心他们的评论
会阻碍或脱轨补丁流程。Reviewed-by 标签被理解为“我已尽我所能审查了这段代码”之意，
而非“我可以证明这段代码是正确的”。

强烈鼓励审查者对提交进行更深入的审查，而不是只专注于像代码格式、标签等这样琐碎或
主观的流程问题。

### 推荐信 / 反馈


一些公司在员工绩效审查中使用同行反馈。请随时向 netdev 维护者请求反馈，特别是如果你
花费大量时间审查代码并格外努力地改进共享基础设施。

反馈必须由你、贡献者本人请求，并且将始终与你共享（即使你要求将其提交给你的经理）。
