
处理回归问题
++++++++++++

**我们不会引入回归（regression* —本文档描述了这条“Linux 内核开发第一准则”对开发者在实践中意味着什么。它Documentation/admin-guide/reporting-regressions.rst 互为补充，后者从用户视角阐述该主题；如果你从未读过那篇文档，请至少先浏览一遍再继续往下看

## 重点速览（aka "The TL;DR"


#. 确保 `regression mailing list <https://lore.kernel.org/regressions/>`_
   (regressions@lists.linux.dev) 的订阅者能迅速得知任何新提交的回归报告：

    - 当收到的邮件报告没有抄送（CC）该列表时，立即发送至少一封简短的“Reply-all”（全部回复），并在抄送中加入该列表，使其进入处理流程

    - 将缺陷跟踪器中提交的任何报告转发或弹回（bounce）到该列表

#. Linux 内核回归跟踪机器人“regzbot”跟踪该问题（这一步可选，但强烈推荐）

    - 对于邮件报告，检查报告者是否包含类``#regzbot
      introduced: v5.13..v5.14-rc1`` 的行。如果没有，则发送一封回复（抄送回归列表），其中包含如下段落，用于告诉 regzbot
```
       #regzbot ^introduced: 1f2e3d4c5b6a

    * When forwarding reports from a bug tracker to the regressions list (see
      above), include a paragraph like the following::

       #regzbot introduced: v5.13..v5.14-rc1
       #regzbot from: Some N. Ice Human <some.human@example.com>
       #regzbot monitor: http://some.bugtracker.example.com/ticket?id=123456789

```
#. 在提交回归修复补丁时，向补丁描述中添加“Closes:”标签，指向报告该问题的所有位置，正如
   Documentation/process/submitting-patches.rst 鍜。
   Documentation/process/5.Posting.rst <development_posting> 所要求的那样。如果你只修复了导致回归的问题中的一部分，可以使用“Link:”标签代替。regzbot 目前对两者不做区分

#. 一旦确定了罪魁祸首（culprit），应尽快修复回归；大多数回归的修复应在两周内合入，但有些需要在两三天内解决


## 与开发者相关的 Linux 内核回归的所有细


### 重要基础，更多细


#### 收到回归报告时该怎么


确保 Linux 内核的回归跟踪者以及其`regression mailing list <https://lore.kernel.org/regressions/>`_
(regressions@lists.linux.dev) 的订阅者能得知任何新报告的回归

 - 当你通过邮件收到一份未抄送该列表的报告时，立即发送至少一封简短的“Reply-all”（全部回复），并在抄送中加入该列表，使其进入处理流程；如果你回复的是一封省略了列表的回复，请尽量确保列表再次被抄送

 - 如果提交到缺陷跟踪器的报告进入了你的收件箱，请将其转发或弹回给该列表。如果报告者已按照
   Documentation/admin-guide/reporting-issues.rst 的指示转发了报告，可考虑事先查看一下列表存档

无论采用上述哪种方式，都应考虑Linux 内核回归跟踪机器人“regzbot”立即开始跟踪该问题

 - 对于邮件报告，检查报告者是否包含了类似
   `#regzbot introduced: 1f2e3d4c5b6a` 的“regzbot 命令”。如果没有，则发送一封回复（抄
```
       #regzbot ^introduced: v5.13..v5.14-rc1

   This tells regzbot the version range in which the issue started to happen;
   you can specify a range using commit-ids as well or state a single commit-id
   in case the reporter bisected the culprit.

   Note the caret (^) before the "introduced": it tells regzbot to treat the
   parent mail (the one you reply to) as the initial report for the regression
   you want to see tracked; that's important, as regzbot will later look out
   for patches with "Closes:" tags pointing to the report in the archives on
   lore.kernel.org.

 * When forwarding a regression reported to a bug tracker, include a paragraph
   with these regzbot commands::

       #regzbot introduced: 1f2e3d4c5b6a
       #regzbot from: Some N. Ice Human <some.human@example.com>
       #regzbot monitor: http://some.bugtracker.example.com/ticket?id=123456789

   Regzbot will then automatically associate patches with the report that
   contain "Closes:" tags pointing to your mail or the mentioned ticket.

```
#### 修复回归时的重要事项


在提交回归修复时，你不需要做任何特殊的事情，只需记得遵循
Documentation/process/submitting-patches.rst銆。
Documentation/process/5.Posting.rst <development_posting> 以及
Documentation/process/stable-kernel-rules.rst 中已经详细解释的那些要求

```
       Closes: https://lore.kernel.org/r/30th.anniversary.repost@klaava.Helsinki.FI/
       Closes: https://bugzilla.kernel.org/show_bug.cgi?id=1234567890

   If you are only fixing part of the issue, you may use "Link:" instead as
   described in the first document mentioned above. regzbot currently treats
   both of these equivalently and considers the linked reports as resolved.

 * Add a "Fixes:" tag to specify the commit causing the regression.

 * If the culprit was merged in an earlier development cycle, explicitly mark
   the fix for backporting using the ``Cc: stable@vger.kernel.org`` tag.

```
所有这些都是对你的最低要求，并且在回归问题上非常重要，因为这些标签对于数周、数月或数年后可能还在查看该问题的所有人（包括你自己）都极有价值。这些标签对于其他内核开发者或 Linux 发行版所使用的工具和脚本也至关重要；其中之一就是 regzbot，它高度依赖“Closes:”标签来将回归报告与解决它们的变更关联起来

#### 修复回归的期望与最佳实


作为一Linux 内核开发者，你应当尽力避免这样一种局面：由你最近的改动引起的回归，让用户只剩下如下几种选择

 - 运行一个带有影响使用的回归的内核

 - 切换到更旧或更新的内核系列

 - 在回归的罪魁祸首被确认后，继续运行一个过时因而潜在不安全的内核超过三周。理想情况下应当少于两周。而如果问题严重或影响大量用户——无论是普遍情况还是在特定环境中——则应当只有几天

如何在实践中实现这一点，取决于多种因素。请使用以下经验法则作为指导

一般而言

 - 将回归相关工作的优先级置于所有其Linux 内核工作之上，除非后者涉及严重问题（例如急迫的安全漏洞、数据丢失、硬件变砖等）

 - 加快修复最近进入某个正mainline、stable longterm 版本mainline 回归（无论是直接进入还是通过 backport）

 - 不要将当前开发周期内的回归视为可以拖延到周期末的问题，因为该问题可能劝阻或阻止用户和 CI 系统现在或总体上对 mainline 进行测试

 - 以所需的谨慎开展工作，避免造成额外或更大的损害，即便这样解决问题可能比下文所述耗时更长

在回归的罪魁祸首已知后，关于时间安排

 - 如果问题严重或困扰大量用户——无论是普遍情况还是在普遍环境中（如特定的硬件环境、发行版stable/longterm 系列）——目标是在两三天内将修复合入 mainline

 - 如果罪魁祸首进入了某个最近的 mainline、stable longterm 版本（无论是直接进入还是通过 backport），目标是在下一个周日之前将修复合入 mainline；如果罪魁祸首在一周初就被发现且解决起来很简单，尽量在同一周内将修复合mainline

 - 对于其他回归，目标是在接下来三周内最靠后的那个周日之前将修复合入 mainline。如果回归是人们可以轻易忍受一段时间的——例如轻微的性能回归——晚一两个周日也是可以接受的

 - 强烈不建议将回归修复的合入拖延到下一个合并窗口（merge window），除非该修复风险极高，或者罪魁祸首在一年多以前就已合入 mainline

关于流程

 - 始终考虑回退（revert）罪魁祸首，因为这通常是最快、最安全的修复回归的方式。不要担心稍后合入一个修复过的变体：那应当很直接，因为大部分代码已经经过一轮评审了

 - 努力在当前开发周期结束前，解mainline 在过去十二个月内引入的回归：Linus 希望这类回归像当前周期的回归一样被处理，除非修复带来不寻常的风险

 - 如果某个回归看起来很棘手，考虑在讨论或补丁评审时抄送（CC）Linus。在棘手或紧急的情况下同样如此——尤其是当子系统维护者可能联系不上时。当你知道这样的回归已进入某mainline、stable longterm 版本时，也要抄stable 团队

 - 对于紧急回归，考虑请求 Linus 直接从邮件列表中拾取（pick up）修复：对于没有争议的修复，他完全没问题这样做。不过理想情况下，此类请求应当得到子系统维护者的同意，或者由他们直接提出

 - 如果你不确定某个修复是否值得在距mainline 发布仅剩几天时冒险应用，请给 Linus 发一封邮件，照例抄送相关列表和人员；在邮件中总结情况，同时请他考虑直接从列表中拾取该修复。然后由他自己做决定，必要时甚至可以推迟发布。此类请求同样理想情况下应当得到子系统维护者的同意，或者由他们直接提出

关于 stable longterm 内核

 - 如果某回归在任何时间点都没有mainline 中出现过，或者已经在 mainline 中被修复，你可以将其留给 stable 团队处理

 - 如果某个回归在过去十二个月内进入了某个正式的 mainline 版本，请确保给修复加上“Cc: stable@vger.kernel.org”标签，因为仅靠“Fixes:”标签并不能保证backport。如果你知道罪魁祸首已被 backport stable longterm 内核，也请加上同样的标签

 - 当收到关于近stable longterm 内核系列的回归报告时，请至少简要评估一下该问题是否也可能出现在当前 mainline 中——如果看起来很有可能，请接手该报告。如果有疑问，请报告者检mainline

 - 当你想迅速解决一个最近也进入了某个正mainline、stable longterm 版本的回归时，请mainline 中快速修复它；在适当时因此请 Linus 加快该修复的合入（见上文）。这是因stable 团队通常不会回退或修复那些在 mainline 中同样引起问题的改动

 - 在紧急回归修复的情况下，你可能希望在修复合入 mainline 后给 stable 团队发个提示，以确保及时 backport；这在合并窗口期间及其刚结束后尤其可取，否则该修复可能会排在一长串补丁队列的末尾

关于补丁流程

 - 开发者，在努力达到上述时间要求时，请记得为修复被测试、评审以及被 Linus 合入（理想情况下至少短暂地进入过 linux-next）所花的时间留出余量。因此，如果某个修复很紧急，请让其紧迫性显而易见，以确保他人妥善处理

 - 评审者，恳请你们及时评审回归修复，以协助开发者达到上述时间要求

 - 子系统维护者，同样鼓励你们加快回归修复的处理。因此请评估对该特定修复跳过 linux-next 是否可行。在需要时，也考虑比平常更频繁地发git pull 请求。并尽量避免将回归修复拖到周末——尤其是当该修复被标记为需backport 时


### 开发者应当了解的有关回归的更多方


#### 如何处理已知存在回归风险的变


评估回归的风险有多大，例如通过Linux 发行版和 Git 托管平台中进行代码搜索。也考虑请其他可能受影响的开发者或项目来评估甚至测试所提议的改动；如果出现问题，也许能找到各方都可接受的方案

如果最终看来回归的风险相对较小，则可以继续改动，但要让所有相关方知晓该风险。因此，请确保你的补丁描述让这一方面显而易见。一旦改动被合入，请将风险告Linux 内核的回归跟踪者以及回归邮件列表，这样若报告陆续出现，所有人都将把该改动放在关注范围内。根据风险大小，你可能还想请子系统维护者在mainline pull 请求中提及该问题

#### 关于回归还有哪些需要了解？


请查Documentation/admin-guide/reporting-regressions.rst，它涵盖了许多你可能想要了解的其他方面：

 - “no regressions”（不引入回归）规则的目

 - 哪些问题才真正算得上是回

 - 谁负责寻找回归的根本原因

 - 如何处理棘手情况，例如当回归是由某个安全修复引起，或者修复一个回归可能导致另一个回归时

#### 关于回归该向谁征求意


发送邮件到回归邮件列表（regressions@lists.linux.dev），同时抄Linux 内核的回归跟踪者（regressions@leemhuis.info）；如果该问题更适合私下处理，可以不抄送列表


### 更多关于回归跟踪regzbot


#### 为什Linux 内核有一个回归跟踪者，又为什么要使用 regzbot


像“no regressions”这样的规则需要有人来确保它们被遵守，否则它们会被无意或有意地破坏。历史表明，Linux 内核而言同样如此。这就是为什Thorsten Leemhuis 自愿Linux 内核回归跟踪者的身份来留意各种情况，偶尔会有其他人协助。他们都不为此获得报酬，因此回归跟踪是尽力而为（best effort）的

早期手动跟踪回归的尝试表明这是一项令人筋疲力尽且沮丧的工作，因此它们一段时间后就被放弃了。为了防止这种情况再次发生，Thorsten 开发了 regzbot 来简化这项工作，长远目标是为所有相关人员尽可能自动化回归跟踪

#### regzbot 的回归跟踪是如何工作的？


该机器人会监视对已跟踪回归报告的回复。此外，它还会寻找引用了此类报告（带有“Closes:”标签）的已发布或已提交补丁；对此类补丁发布的回复也会被跟踪。结合起来的数据提供了关于修复过程当前状态的良好洞察

regzbot 试图以尽可能少的开销为报告者和开发者完成工作。事实上，只有报告者被额外增加了一项职责：他们需要使用上文概述的 ``#regzbot
introduced`` 命令来告regzbot 回归报告；如果他们不这样做，其他人可以使`#regzbot ^introduced` 来处理

对开发者来说，通常不涉及额外工作，他们只需要确保去做在 regzbot 出现之前就早该做的事：在补丁描述中添加指向所有关于所修复问题的报告的链接

#### 我必须使regzbot 吗？


如果你使用它，这符合每个人的利益，因为内核维护者（Linus
Torvalds）在工作中部分依regzbot 的跟踪——例如在决定是否发布新版本或延长开发阶段时。为此，他们需要知晓所有未修复的回归；众所周知，Linus 会查regzbot 发送的每周报告

#### 我必须把我遇到的每一个回归都告诉 regzbot 吗？


理想情况下是的：我们都是人，当更重要的事情意外出现时——例Linux 内核中的一个更大问题，或者现实生活中让我们暂时离开键盘的事情——我们很容易忘记问题。因此，最好把每一个回归都告诉 regzbot，除非你立即写了一个修复并将其提交到一个定期合入受影响内核系列的树中

#### 如何查看 regzbot 当前跟踪哪些回归


请查`regzbot's web-interface <https://linux-regtracking.leemhuis.info/regzbot/>`_
获取最新信息；或者，`search for the latest regression report
<https://lore.kernel.org/lkml/?q=%22Linux+regressions+report%22+f%3Aregzbot>`_锛。
regzbot 通常会在周日傍晚（UTC）发送一次，也就Linus 通常发布新（预）发布版本前几个小时

#### regzbot 在监控哪些地方？


regzbot 正在监视最重要Linux 邮件列表，以linux-next、mainline stable/longterm git 仓库

#### 哪些类型的问题应当由 regzbot 跟踪


该机器人旨在跟踪回归，因此请不要为常规问题引regzbot。但如果你使regzbot 来跟踪严重问题（如关于挂起、数据损坏或内部错误（Panic、Oops、BUG()、warning 等）的报告），Linux 内核的回归跟踪者是可以接受的

#### 我可以把 CI 系统发现的回归加regzbot 的跟踪吗


如果特定回归很可能对实际使用场景产生影响，从而可能被用户注意到，请随意这样做；因此，请不要为不太可能在真实世界使用中显现的理论性回归引regzbot

#### 如何regzbot 交互


通过在带有回归报告的邮件的直接或间接回复中使用“regzbot 命令”。这些命令需要位于各自独立的段落中（IOW：需要用空行与邮件其余部分分隔）

其中一个命令是 `#regzbot introduced: <version or commit>`，它会让 regzbot 将你的邮件视为加入跟踪的回归报告，如上文所述；`#regzbot ^introduced: <version or commit>` 是另一个此类命令，它会regzbot 将父邮件视为它开始跟踪的回归的报告

一旦使用了这两个命令之一，就可以在对该报告的直接或间接回复中使用其他 regzbot 命令。你可以将它们写在某`introduced` 命令下方，或者在使用了其中一个命令的邮件的回复中，或者本身是对该邮件的回复中

```
       #regzbot title: foo

 * Monitor a discussion or bugzilla.kernel.org ticket where additions aspects of
   the issue or a fix are discussed -- for example the posting of a patch fixing
   the regression::

       #regzbot monitor: https://lore.kernel.org/all/30th.anniversary.repost@klaava.Helsinki.FI/

   Monitoring only works for lore.kernel.org and bugzilla.kernel.org; regzbot
   will consider all messages in that thread or ticket as related to the fixing
   process.

 * Point to a place with further details of interest, like a mailing list post
   or a ticket in a bug tracker that are slightly related, but about a different
   topic::

       #regzbot link: https://bugzilla.kernel.org/show_bug.cgi?id=123456789

 * Mark a regression as fixed by a commit that is heading upstream or already
   landed::

       #regzbot fix: 1f2e3d4c5d

 * Mark a regression as a duplicate of another one already tracked by regzbot::

       #regzbot dup-of: https://lore.kernel.org/all/30th.anniversary.repost@klaava.Helsinki.FI/

 * Mark a regression as invalid::

       #regzbot invalid: wasn't a regression, problem has always existed

```
#### 关于 regzbot 及其命令还有什么可说的吗？


关于 Linux 内核回归跟踪机器人的更详细、更及时的信息可以在
`project page <https://gitlab.com/knurd42/regzbot>`_ 上找到，其中包括
`getting started guide <https://gitlab.com/knurd42/regzbot/-/blob/main/docs/getting_started.md>`_
鍜?`reference documentation <https://gitlab.com/knurd42/regzbot/-/blob/main/docs/reference.md>`_锛。
两者涵盖的细节都多于上面这一节

### Linus 关于回归的语


以下 Linus Torvalds 的言论提供了一些关Linux
“no regressions”（不引入回归）规则以及他期望回归如何被处理的洞见：

#### 关于回归应当多快被修


```
    But a user complaining should basically result in an immediate fix -
    possibly a "revert and rethink".

  With a later clarification on `2026-01-28 <https://lore.kernel.org/all/CAHk-%3Dwi86AosXs66-yi54%2BmpQjPu0upxB8ZAfG%2BLsMyJmcuMSA@mail.gmail.com/>`_::

    It's also worth noting that "immediate" obviously doesn't mean "right
    this *second* when the problem has been reported".

    But if it's a regression with a known commit that caused it, I think
    the rule of thumb should generally be "within a week", preferably
    before the next rc.

```
```
    Known-broken commits either
     (a) get a timely fix that doesn't have other questions
    or
     (b) get reverted

```
```
    [...] review shouldn't hold up reported regressions of existing code. That's
    just basic _testing_ - either the fix should be applied, or - if the fix is
    too invasive or too ugly - the problematic source of the regression should
    be reverted.

    Review should be about new code, it shouldn't be holding up "there's a
    bug report, here's the obvious fix".

```
```
    If something doesn't even build, it should damn well be fixed ASAP.

```
#### 关于用回退来修复回归如何有助于防止维护者倦


```
    > So how can I/we make "immediate fixes" happen more often without
    > contributing to maintainer burnout?

    [...] the "revert and rethink" model [...] often a good idea in general [...]

    Exactly so that maintainers don't get stressed out over having a pending
    problem report that people keep pestering them about.

    I think people are sometimes a bit too bought into whatever changes
    they made, and reverting is seen as "too drastic", but I think it's
    often the quick and easy solution for when there isn't some obvious
    response to a regression report.

```
#### 关于在最后一-rc 或新版本临近时合入修


```
    So I think I'd rather see them hit rc8 (later today) and have a week
    of testing in my tree and be reverted if they cause problems, than
    have them go in after rc8 and then cause problems in the 6.19 release
    instead.

```
```
    But something like this, where the regression was in the previous release
    and it's just a clear fix with no semantic subtlety, I consider to be just a
    regular regression that should be expedited - partly to make it into stable,
    and partly to avoid having to put the fix into _another_ stable kernel.

```
#### 关于只提交一个修复的合并请求


```
    If the issue is just that there's nothing else happening, I think people
    should just point me to the patch and say "can you apply this single fix?"

```
```
    I'm always open to direct fixes when there is no controversy about the fix.
    No problem. I still happily deal with individual patches.

```
#### 关于使用 Link:/Closes: 标签指向缺陷报告的重要


```
    [...] revert like this, it really would be good to link to the problems, so
    that when people try to re-enable it, they have the history for why it
    didn't work the first time.

```
```
    So I have to once more complain [...]

    [...] There's no link to the actual problem the patch fixes.

```
```
    See, *that* link [to the report] would have been useful in the commit.

```
#### 关于“no regressions”规则为何存


```
    But the basic rule is: be so good about backwards compatibility that
    users never have to worry about upgrading. They should absolutely feel
    confident that any kernel-reported problem will either be solved, or
    have an easy solution that is appropriate for *them* (ie a
    non-technical user shouldn't be expected to be able to do a lot).

    Because the last thing we want is people holding back from trying new
    kernels.

```
```
    I introduced that "no regressions" rule something like two decades
    ago, because people need to be able to update their kernel without
    fear of something they relied on suddenly stopping to work.

```
```
    The whole point of "we do not regress" is so that people can upgrade
    the kernel and never have to worry about it.

    [...]

    Because the only thing that matters IS THE USER.

```
```
    If the kernel used to work for you, the rule is that it continues to work
    for you.

    [...]

    People should basically always feel like they can update their kernel
    and simply not have to worry about it.

    I refuse to introduce "you can only update the kernel if you also
    update that other program" kind of limitations. If the kernel used to
    work for you, the rule is that it continues to work for you.

```
#### 关于“no regressions”规则的例外


```
    There are _very_ few exceptions to that rule, the main one being "the
    problem was a fundamental huge and gaping security issue and we *had* to
    make that change, and we couldn't even make your limited use-case just
    continue to work".

    The other exception is "the problem was reported years after it was
    introduced, and now most people rely on the new behavior".

    [...]

    Now, if it's one or two users and you can just get them to recompile,
    that's one thing. Niche hardware and odd use-cases can sometimes be
    solved that way, and regressions can sometimes be fixed by handholding
    every single reporter if the reporter is willing and able to change
    his or her workflow.

```
```
    And yes, I do consider "regression in an earlier release" to be a
    regression that needs fixing.

    There's obviously a time limit: if that "regression in an earlier
    release" was a year or more ago, and just took forever for people to
    notice, and it had semantic changes that now mean that fixing the
    regression could cause a _new_ regression, then that can cause me to
    go "Oh, now the new semantics are what we have to live with".

```
```
    There have been exceptions, but they are few and far between, and they
    generally have some major and fundamental reasons for having happened,
    that were basically entirely unavoidable, and people _tried_hard_ to
    avoid them. Maybe we can't practically support the hardware any more
    after it is decades old and nobody uses it with modern kernels any
    more. Maybe there's a serious security issue with how we did things,
    and people actually depended on that fundamentally broken model. Maybe
    there was some fundamental other breakage that just _had_ to have a
    flag day for very core and fundamental reasons.

```
#### 关于更新用户空间中的某些东西可以解决回归的情


```
    And dammit, we upgrade the kernel ALL THE TIME without upgrading any
    other programs at all. It is absolutely required, because flag-days
    and dependencies are horribly bad.

    And it is also required simply because I as a kernel developer do not
    upgrade random other tools that I don't even care about as I develop the
    kernel, and I want any of my users to feel safe doing the same time.

```
```
    But if something actually breaks, then the change must get fixed or
    reverted. And it gets fixed in the *kernel*. Not by saying "well, fix your
    user space then". It was a kernel change that exposed the problem, it needs
    to be the kernel that corrects for it, because we have a "upgrade in place"
    model. We don't have a "upgrade with new user space".

    And I seriously will refuse to take code from people who do not understand
    and honor this very simple rule.

    This rule is also not going to change.

    And yes, I realize that the kernel is "special" in this respect. I'm proud
    of it.

```
```
    If you break existing user space setups THAT IS A REGRESSION.

    It's not ok to say "but we'll fix the user space setup".

    Really. NOT OK.

```
#### 关于什么算作用户空间接口、ABI、API、已文档化的接口


```
    So I absolutely detest the whole notion of "ABI changes". It's a
    meaningless concept, and I hate it with a passion, [...]

    The Linux rule for regressions is basically based on the philosophical
    question of "If a tree falls in the forest, and nobody is around to
    hear it, does it make a sound?".

    So the only thing that matters is if something breaks user-*conscious*
    behavior.

    And when that happens, the distinction between "bug fix" and "new
    feature" and "ABI change" matters not one whit, and the change needs
    to be done differently.

    [...]

    I just wanted to point out that the argument about whether it's an ABI
    change or not is irrelevant. If it turns out that some program - not a test
    script, but something with relevance to conscious user expectations ~
    depended on the old broken behavior, then it needs to be done some other
    way.

```
```
    > [...] this should not fall under the don't break user space rule [...]

    Note that the rule is about breaking *users*, not breaking user space per
    se. [...]

    If some user setup breaks, things need fixing.

    [...] but I want to make it very clear that there are no excuses about "user
    space applications".

```
```
    [...] a regression is a bit like Schr枚dinger's cat - if nobody is around
    to notice it and it doesn't actually affect any real workload, then you
    can treat the regression as if it doesn't exist.

```
```
    The rules about regressions have never been about any kind of documented
    behavior, or where the code lives.

    The rules about regressions are always about "breaks user workflow".

    Users are literally the _only_ thing that matters.

```
```
    One _particularly_ last-minute revert is the top-most commit (ignoring
    the version change itself) done just before the release, and while
    it's very annoying, it's perhaps also instructive.

    What's instructive about it is that I reverted a commit that wasn't
    actually buggy. In fact, it was doing exactly what it set out to do,
    and did it very well. In fact it did it _so_ well that the much
    improved IO patterns it caused then ended up revealing a user-visible
    regression due to a real bug in a completely unrelated area.

    The actual details of that regression are not the reason I point that
    revert out as instructive, though. It's more that it's an instructive
    example of what counts as a regression, and what the whole "no
    regressions" kernel rule means.

    [...] The reverted commit didn't change any API's, and it didn't introduce
    any new bugs. But it ended up exposing another problem, and as such caused
    a kernel upgrade to fail for a user. So it got reverted.

    The point here being that we revert based on user-reported _behavior_, not
    based on some "it changes the ABI" or "it caused a bug" concept. The problem
    was really pre-existing, and it just didn't happen to trigger before. [...]

    Take-away from the whole thing: it's not about whether you change the
    kernel-userspace ABI, or fix a bug, or about whether the old code
    "should never have worked in the first place". It's about whether
    something breaks existing users' workflow.

```
```
    And our regression rule has never been "behavior doesn't change".
    That would mean that we could never make any changes at all.

```
```
    No amount of "you shouldn't have used this" or "that behavior was
    undefined, it's your own fault your app broke" or "that used to work
    simply because of a kernel bug" is at all relevant.

```
```
    But no, "that was documented to be broken" (whether it's because the code
    was in staging or because the man-page said something else) is irrelevant.
    If staging code is so useful that people end up using it, that means that
    it's basically regular kernel code with a flag saying "please clean this
    up".

    [...]

    The other side of the coin is that people who talk about "API stability" are
    entirely wrong. API's don't matter either. You can make any changes to an
    API you like - as long as nobody notices.

    Again, the regression rule is not about documentation, not about API's, and
    not about the phase of the moon.

```
```
    > Now this got me wondering if Debian _unstable_ actually qualifies as a
    > standard distro userspace.

    Oh, if the kernel breaks some standard user space, that counts. Tons
    of people run Debian unstable

```
```
    It's clearly NOT an internal tracepoint. By definition. It's being
    used by powertop.

```
#### 关于用户或测试套CI 注意到的回归


```
    Users complaining is the only real line in the end.

    [...] a test-suite complaining is then often a *very* good indication that
    maybe users will hit some problem, and test suite issues should be taken
    very seriously [...]

    But a test-suite error isn't necessarily where you have to draw the
    line - it's a big red flag [...]

```
```
    The "no regressions" rule is not about made-up "if I do this, behavior
    changes".

    The "no regressions" rule is about *users*.

    If you have an actual user that has been doing insane things, and we
    change something, and now the insane thing no longer works, at that
    point it's a regression, and we'll sigh, and go "Users are insane" and
    have to fix it.

    But if you have some random test that now behaves differently, it's
    not a regression. It's a *warning* sign, sure: tests are useful.

```
#### 关于承认回归已经发生


```
    But starting to argue about users reporting breaking changes is
    basically the final line for me. I have a couple of people that I have
    in my spam block-list and refuse to have anything to do with, and they
    have generally been about exactly that.

    Note how it's not about making mistakes and _causing_ the regression.
    That's normal. That's development. But then arguing about it is a
    no-no.

```
```
    We don't introduce regressions and then blame others.

    There's a very clear rule in kernel development: things that break
    other things ARE NOT FIXES.

    EVER.

    They get reverted, or the thing they broke gets fixed.

```
```
    THERE ARE NO VALID ARGUMENTS FOR REGRESSIONS.

    Honestly, security people need to understand that "not working" is not
    a success case of security. It's a failure case.

    Yes, "not working" may be secure. But security in that case is *pointless*.

```
```
    [...] when regressions *do* occur, we admit to them and fix them, instead of
    blaming user space.

    The fact that you have apparently been denying the regression now for
    three weeks means that I will revert, and I will stop pulling apparmor
    requests until the people involved understand how kernel development
    is done.

```
#### 关于来回拉锯


```
    The "no regressions" rule is that we do not introduce NEW bugs.

    It *literally* came about because we had an endless dance of "fix two
    bugs, introduce one new one", and that then resulted in a system that
    you cannot TRUST.

```
```
    And the thing that makes regressions special is that back when I
    wasn't so strict about these things, we'd end up in endless "seesaw
    situations" where somebody would fix something, it would break
    something else, then that something else would break, and it would
    never actually converge on anything reliable at all.

```
```
    The strict policy of no regressions actually originally started mainly wrt
    suspend/resume issues, where the "fix one machine, break another" kind of
    back-and-forth caused endless problems, and meant that we didn't actually
    necessarily make any forward progress, just moving a problem around.

```
#### 关于有引发回归风险的变化


```
    So what I think you should do is to fix the bug right, with a clean
    patch, and no crazy hacks. That is something we can then apply and
    test. All the while knowing full well that "uhhuh, this is a visible
    change, we may have to revert it".

    If then some *real* load ends up showing a regression, we may just be
    screwed. Our current behavior may be buggy, but we have the rule that
    once user space depends on kernel bugs, they become features pretty
    much by definition, however much we might dislike it.

```
#### 关于内核内的变通方案以避免回归


```
    Behavioral changes happen, and maybe we don't even support some
    feature any more. There's a number of fields in /proc/<pid>/stat that
    are printed out as zeroes, simply because they don't even *exist* in
    the kernel any more, or because showing them was a mistake (typically
    an information leak). But the numbers got replaced by zeroes, so that
    the code that used to parse the fields still works. The user might not
    see everything they used to see, and so behavior is clearly different,
    but things still _work_, even if they might no longer show sensitive
    (or no longer relevant) information.

```
#### 关于由缺陷修复引起的回归


```
    > Kernel had a bug which has been fixed

    That is *ENTIRELY* immaterial.

    Guys, whether something was buggy or not DOES NOT MATTER.

    [...]

    It's basically saying "I took something that worked, and I broke it,
    but now it's better". Do you not see how f*cking insane that statement
    is?

```
#### 关于内部 API 变更


```
    We do API breakage _inside_ the kernel all the time. We will fix
    internal problems by saying "you now need to do XYZ", but then it's
    about internal kernel API's, and the people who do that then also
    obviously have to fix up all the in-kernel users of that API. Nobody
    can say "I now broke the API you used, and now _you_ need to fix it
    up". Whoever broke something gets to fix it too.

```
#### 关于很久以后才被发现的回


```
    I'm definitely not reverting a patch from almost a decade ago as a
    regression.

    If it took that long to find, it can't be that critical of a regression.

    So yes, let's treat it as a regular bug.

```
#### 关于linux-next 中测试回归修


```
   So running fixes though linux-next is just a waste of time.

```
#### 关于与回归相关的其他几个方面


- From `2025-07-29(2) <https://lore.kernel.org/all/CAHk-=wjj9DvOZtmTkoLtyfHmy5mNKy6q_96d9=4FUEDXre=cww@mail.gmail.com/>`_
```
    I no longer have sound.

    I also suspect that it's purely because "make oldconfig" doesn't work,
    and probably turned off my old Intel HDA settings. Or something.

    Renaming config parameters is *bad*. I've harped on the Kconfig phase
    of the kernel build probably being our nastiest point, and a real pain
    point to people getting involved with development simply because
    building your own kernel can be so daunting with hundreds of fairly
    esoteric questions.

```
..
   end-of-content
..
   This text is available under GPL-2.0+ or CC-BY-4.0, as stated at the top
   of the file. If you want to distribute this text under CC-BY-4.0 only,
   please use "The Linux kernel developers" for author attribution and link
   this as source:
   https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/Documentation/process/handling-regressions.rst
..
   Note: Only the content of this RST file as found in the Linux kernel sources
   is available under CC-BY-4.0, as versions of this text that were processed
   (for example by the kernel's build system) might contain content taken from
   files which use a more restrictive license.
