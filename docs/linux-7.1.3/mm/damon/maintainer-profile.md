
## DAMON 维护者条目档
DAMON 子系统覆`MAINTAINERS` 文件`DAMON` 章节所列的文件
该子系统的邮件列表为 damon@lists.linux.dev linux-mm@kvack.org。补丁应尽可能基`mm-new tree
<https://git.kernel.org/akpm/mm/h/mm-new>`_ 制作，并发布到邮件列表
### SCM 鏍。
DAMON 开发有多个 Linux 树。处于开发或测试中的补丁DAMON 维护者排`damon/next
<https://git.kernel.org/sj/h/damon/next>`_。经过充分评审的补丁由内存管理子系统维护者排`mm-new
<https://git.kernel.org/akpm/mm/h/mm-new>`_。随着测试更加充分，补丁会移动`mm-unstable <https://git.kernel.org/akpm/mm/h/mm-unstable>`_，再移动`mm-stable <https://git.kernel.org/akpm/mm/h/mm-stable>`_。最终这些补丁会由内存管理子系统维护者以拉取请求的形式提交到主线
再次提醒，针`mm-new tree
<https://git.kernel.org/akpm/mm/h/mm-new>`_ 的补丁由内存管理子系统维护者排队。如果补丁需`damon/next tree
<https://git.kernel.org/sj/h/damon/next>`_ 中尚未合并进 mm-new 的某些补丁，请务必清楚说明该依赖关系
### 提交检查清单补
进行 DAMON 改动时，应做到以下几点
- 构建变更相关的产物，包括内核与文档- 确保构建不引入新的错误或警告- 运行 DAMON `selftests
  <https://github.com/damonitor/damon-tests/blob/master/corr/run.sh#L49>`_ 涓?`kunittests
  <https://github.com/damonitor/damon-tests/blob/master/corr/tests/kunit.sh>`_ 并确保无新失败
进一步做到以下几点并把结果附上会很有帮助
- 针对普通改动运`damon-tests/corr
  <https://github.com/damonitor/damon-tests/tree/master/corr>`_- 针对性能改动，测量对基准测试或真实世界工作负载的影响
### 关键周期日期

补丁可随时发送。`mm-new
<https://git.kernel.org/akpm/mm/h/mm-new>`_、`mm-unstable
<https://git.kernel.org/akpm/mm/h/mm-unstable>`_ 涓?`mm-stable
<https://git.kernel.org/akpm/mm/h/mm-stable>`_ 树的关键周期日期取决于内存管理子系统维护者
### 评审节奏

DAMON 维护者通常工作方式灵活，但太平洋时间（PT）清晨除外。对补丁的回复偶尔会较慢。如果在发送补丁后一周内没有收到回复，请勿犹豫，发消息提醒一下
### 邮件工具

与许多其Linux 内核子系统一样，DAMON 使用邮件列表（damon@lists.linux.dev linux-mm@kvack.org）作为主要沟通渠道。有一个名`HacKerMaiL
<https://github.com/damonitor/hackermail>`_（`hkml`）的简单工具，面向不太熟悉基于邮件列表沟通的人。该工具DAMON 社区成员尤其有用，因为它DAMON 维护者开发并维护。该工具也已正式宣布支持 DAMON 及通用Linux 内核开发流程
换言之，`hkml <https://github.com/damonitor/hackermail>`_ 是面DAMON 社区的邮件工具，DAMON 维护者承诺予以支持。请随意试用，并向维护者报告问题或功能请求
### 社区聚会

DAMON 社区为更喜欢同步对话而非邮件往来的成员举办双周聚会系列。它用于包括维护者在内的一群成员之间就特定主题展开讨论。维护者会分享可用的时段，参会者应在时段开始前至少 24 小时通过联系维护者来预约其中一个时段
日程与预约状态可Google `doc
<https://docs.google.com/document/d/1v43Kcj3ly4CYqmAkMaZzLiM2GEnWfgdGbZAH3mi2vpM/editusp=sharing>`_ 查看。还有一个公开Google `calendar
<https://calendar.google.com/calendar/u/0?cid=ZDIwOTA4YTMxNjc2MDQ3NTIyMmUzYTM5ZmQyM2U4NDA0ZGIwZjBiYmJlZGQxNDM0MmY4ZTRjOTE0NjdhZDRiY0Bncm91cC5jYWxlbmRhci5nb29nbGUuY29t>`_
包含相关事件。任何人都可以订阅。DAMON 维护者也会定期向邮件列表（damon@lists.linux.dev）发送提醒