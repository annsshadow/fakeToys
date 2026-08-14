
## 提交补丁：让你的代码进入内核的必备指南


对于一个希望向 Linux 内核提交改动的个人或公司来说，如果你不熟悉“这套系统”，
这个过程有时可能令人生畏。本文本是一系列建议的集合，可以极大地提高你的改动
被接受的机会。

本文档以相对简洁的格式包含了大量建议。关于内核开发流程如何运作的详细信息，
请参阅 Documentation/process/development-process.rst。另外，在提交代码之前，
请阅读 Documentation/process/submit-checklist.rst 以查看需要检查的事项清单。
对于设备树绑定补丁，请阅读
Documentation/devicetree/bindings/submitting-patches.rst。

本文档假设你正在使用 `git` 来准备你的补丁。如果你不熟悉 `git`，最好去学习如何
使用它，这将使你作为内核开发者的生活乃至一般生活都轻松得多。

某些子系统和维护者树有关于其工作流程和期望的额外信息，请参阅
Documentation/process/maintainer-handbooks.rst。

### 获取当前的源代码树


如果你手边没有一个包含当前内核源代码的仓库，请使用 `git` 获取一个。你会希望从
主线仓库开始，
```

  git clone git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git

```
但请注意，你可能并不想直接基于主线树进行开发。大多数子系统维护者运行他们自己的
树，并希望看到基于那些树准备的补丁。查看 MAINTAINERS 文件中该子系统的 **T:** 条目
来找到那棵树，或者如果那里没有列出树，直接询问维护者即可。

### 描述你的改动


描述你的问题。无论你的补丁是一个一行的缺陷修复，还是 5000 行的新功能，必然存在
一个促使你做这项工作的根本问题。要让审查者相信存在值得修复的问题，并且值得他们
读下去、越过第一段。

描述对用户可见的影响。直接的崩溃和死锁相当有说服力，但并非所有缺陷都那么明显。
即使问题是在代码审查中发现的，也要描述你认为它可能给用户带来的影响。请记住，
大多数 Linux 安装运行的是来自次级稳定树或供应商/产品特定树的、只从上游拣选
（cherry-pick）特定补丁的内核，因此要包含任何有助于将你的改动向下游传递的信息：
诱发的情形、来自 dmesg 的摘录、崩溃描述、性能回归、延迟尖峰、死锁等。

量化优化和权衡。如果你声称在性能、内存消耗、栈占用或二进制大小方面有所改善，要
包含支持这些说法的数字。但也要描述不明显代价。优化通常并不是免费的，而是 CPU、
内存和可读性之间的权衡；或者，在涉及启发式方法时，是不同工作负载之间的权衡。
描述你优化的预期缺点，以便审查者能够权衡代价与收益。

一旦问题确立，就要以技术细节描述你实际上在做什么。用平实的英语描述改动很重要，
以便审查者核实代码是否如你预期的那样运行。

如果你把补丁描述写成一种可以轻松被拉入 Linux 的源代码管理系统 `git` 作为
“commit log（提交日志）”的形式，维护者会感谢你。请参阅 the_canonical_patch_format。

每个补丁只解决一个问题。如果你的描述开始变得很长，那说明你可能需要拆分你的补丁。
请参阅 split_changes。

当你提交或重新提交一个补丁或补丁系列时，要包含完整的补丁描述及其理由。不要只说
这是补丁（系列）的第 N 个版本。不要指望子系统维护者去回看更早的补丁版本或引用的
URL 来寻找补丁描述并把它放进补丁里。也就是说，补丁（系列）及其描述应当是自包含的。
这对维护者和审查者都有好处。有些审查者可能根本没有收到过早期版本的补丁。

用祈使语气描述你的改动，例如“make xyzzy do frotz”，而不是“[This patch] makes
xyzzy do frotz”或“[I] changed xyzzy to do frotz”，就好像你在命令代码库改变其行为一样。

如果你想引用一个特定的提交，不要只引用该提交的 SHA-1 ID。请同时包含该提交的单行
摘要，以便审查者更容易知道它是关于什么的。
```

	Commit e21d2170f36602ae2708 ("video: remove unnecessary
	platform_set_drvdata()") removed the unnecessary
	platform_set_drvdata(), but left the variable "dev" unused,
	delete it.

```
你还应该确保至少使用 SHA-1 ID 的前十二个字符。内核仓库保存了**大量**的对象，使得
较短 ID 发生碰撞成为真实的可能。请记住，即使现在你的六字符 ID 没有碰撞，这种状况
可能在五年后改变。

如果相关的讨论或改动背后的任何其他背景信息可以在网上找到，请添加指向它的 'Link:'
标签。如果补丁是某些早期邮件列表讨论或网上记录的内容的结果，请指向它。

在链接到邮件列表归档时，最好使用 lore.kernel.org 的消息归档服务。要创建链接 URL，
请使用消息的 `Message-ID` 头部的内容，去掉两侧的角括号。
```

    Link: https://lore.kernel.org/30th.anniversary.repost@klaava.Helsinki.FI

```
请检查链接，确保它确实可用并指向相关的消息。

然而，要努力让你的解释在没有外部资源的情况下也能被理解。除了给出邮件列表归档或
缺陷的 URL 之外，还要总结导致所提交补丁的讨论的要点。

如果你的补丁修复了一个缺陷，请使用 'Closes:' 标签并带上引用缺陷的 URL
```

	Closes: https://example.com/issues/1234

```
某些缺陷跟踪器有能力在带有此类标签的提交被应用后自动关闭问题。一些监控邮件列表
的机器人也可以跟踪这类标签并采取某些行动。私有缺陷跟踪器和无效 URL 是被禁止的。

如果你的补丁修复了某个特定提交中的缺陷，例如你使用 `git bisect` 发现了问题，请使用
'Fixes:' 标签，至少带上 SHA-1 ID 的前 12 个字符，以及单行摘要。不要将该标签拆分成
多行，标签不受“在 75 列处换行”规则的约束
```

	Fixes: 54a4f0239f2e ("KVM: MMU: make kvm_mmu_zap_page() return the number of pages it actually freed")

```
以下 `git config` 设置可用于添加一种漂亮的格式
```

	[core]
		abbrev = 12
	[pretty]
		fixes = Fixes: %h ("%s")

```

```

	$ git log -1 --pretty=fixes 54a4f0239f2e
	Fixes: 54a4f0239f2e ("KVM: MMU: make kvm_mmu_zap_page() return the number of pages it actually freed")

```

### 拆分你的改动


把每个**逻辑改动**拆分成一个单独的补丁。

例如，如果你的改动同时包含对单个驱动程序的缺陷修复和性能增强，就把那些改动分成
两个或更多补丁。如果你的改动包含一个 API 更新，以及一个使用该新 API 的新驱动
程序，就把它们分成两补丁。

另一方面，如果你对众多文件做了一个单一改动，就把那些改动归并到一个补丁中。因此
一个单一的逻辑改动包含在一个单独的补丁内。

要记住的要点是，每个补丁都应该做出一个易于理解的、可以被审查者核实的改动。每个
补丁都应该能凭其自身的价值被证明是合理的。

如果一个补丁依赖于另一个补丁，改动才算完整，那也可以。只需在你的补丁描述中注明
**“this patch depends on patch X”** 即可。

当把你的改动分成一个补丁系列时，要特别注意确保每个补丁之后内核都能正确构建和
运行。使用 `git bisect` 来追踪问题的开发者可能在该系列的任何位置拆分你的补丁系列；
如果你在中间引入了缺陷，他们是不会感谢你的。

如果你无法把你的补丁集合压缩成更小的补丁集，那么一次只发布大约 15 个左右，并等待
审查和合并。

### 对你的改动进行风格检查


检查你的补丁是否存在基本的风格违规，其细节可以在
Documentation/process/coding-style.rst 中找到。不这样做只会浪费审查者的时间，并
会导致你的补丁被拒绝，很可能连读都没读就被拒。

一个重要的例外是，当把代码从一个文件移动到另一个文件时——在这种情况下，你不应当在
移动它的同一个补丁中修改被移动的代码。这清晰地划分了移动代码这一行为和你的改动。
这极大地有助于审查实际的差异，并允许工具更好地跟踪代码本身的历史。

在提交之前用补丁风格检查器检查你的补丁（scripts/checkpatch.pl）。不过请注意，风格
检查器应当被视为一种指南，而不是人类判断的替代品。如果你的代码在违反某个规则时
看起来更好，那么很可能最好不要动它。

检查器在三个级别报告：
 - ERROR：非常可能出错的东西
 - WARNING：需要仔细审查的东西
 - CHECK：需要思考的东西

你应该能够为补丁中仍然存在的所有违规给出理由。

### 为你的补丁选择收件人


对于任何你维护者所维护代码的补丁，你都应当总是抄送（copy）相应的子系统维护者和
列表；请查阅 MAINTAINERS 文件和源代码修订历史，看看那些维护者是谁。脚本
scripts/get_maintainer.pl 在这一步非常有用（把你的补丁路径作为参数传给
scripts/get_maintainer.pl）。如果你找不到你所工作的子系统的维护者，Andrew Morton
（akpm@linux-foundation.org）可以作为最后的维护者。

linux-kernel@vger.kernel.org 应当被默认用于所有补丁，但该列表上的邮件量已经导致
许多开发者将其屏蔽。不过，请不要向不相关的列表和不相关的人发送垃圾邮件。

许多与内核相关的列表托管在 kernel.org；你可以在 https://subspace.kernel.org 找到
它们的列表。不过，也有托管在其他地方的内核相关列表。

Linus Torvalds 是所有被接受到 Linux 内核中的改动的终审仲裁者。他的电子邮件地址是
<torvalds@linux-foundation.org>。他会收到大量电子邮件，而且到了现在，直接经过
Linus 的补丁已经很少，所以通常你应该尽力**避免**给他发电子邮件。

如果你有一个修复可被利用的安全缺陷的补丁，请把该补丁发送到 security@kernel.org。
对于严重缺陷，可以考虑短暂的 embargo（禁运），以便发行版能把补丁推送到用户手中；
在这种情况下，显然不应将该补丁发送到任何公开列表。另请参阅
Documentation/process/security-bugs.rst。

修复已发布内核中严重缺陷的补丁应当被定向
```

  Cc: stable@vger.kernel.org

```
到你的补丁的 sign-off（签署）区域（注意，不是邮件收件人）。除了本文档之外，你还
应该阅读 Documentation/process/stable-kernel-rules.rst。

如果改动影响了用户空间-内核接口，请将 MAN-PAGES 维护者（如 MAINTAINERS 文件中所列）
的 man-pages 补丁，或至少是改动的通知发送给他们，以便某些信息能进入到手册页中。
用户空间 API 的改动也应该抄送给 linux-api@vger.kernel.org。

### 不要 MIME、不要链接、不要压缩、不要附件。只要纯文本


Linus 和其他内核开发者需要能够阅读并评论你提交的改动。对于内核开发者来说，能够
使用标准的电子邮件工具“引用”你的改动以便对代码的特定部分发表评论，是很重要的。

因此，所有补丁都应该通过电子邮件“内联”提交。最简单的方法是使用 `git send-email`，
强烈推荐这样做。`git send-email` 的交互式教程可以在 https://git-send-email.io 找到。

如果你选择不使用 `git send-email`：

  beware 你的编辑器的自动换行弄坏你的补丁，如果你选择剪切粘贴你的补丁的话。

不要把补丁作为 MIME 附件发送，无论压缩与否。许多流行的电子邮件应用并不总会把 MIME
附件作为纯文本传输，使得无法对你的代码发表评论。MIME 附件也会让 Linus 多花一点
时间来处理，降低了你的 MIME 附件改动被接受的可能性。

例外：如果你的邮件程序把补丁弄乱了，那么可能有人会要求你用 MIME 重新发送它们。

请参阅 Documentation/process/email-clients.rst，以获取关于配置你的电子邮件客户端
使其原样发送补丁的提示。

### 回应审查意见


你的补丁几乎肯定会收到审查者关于如何改进该补丁的评论，以回复你电子邮件的形式。
你必须回应那些评论；忽视审查者是招致被忽视的好办法。你可以简单地回复他们的电子邮件
来回答他们的评论。那些不会导致代码改动的审查评论或问题，几乎肯定应该带来一条评论
或变更日志条目，以便下一位审查者更好地理解正在发生的事情。

一定要告诉审查者你正在做哪些改动，并感谢他们的时间。代码审查是一个令人疲倦且耗时
的过程，审查者有时会变得暴躁。即便在那种情况下，也要礼貌地回应，并解决他们指出的
问题。当发送下一个版本时，在封面信（cover letter）或各个补丁中添加一个 `patch
changelog`，说明与上一次提交之间的差异（请参阅 the_canonical_patch_format）。通过
把评论过你补丁的人加入补丁的 CC 列表，来通知他们新版本。

请参阅 Documentation/process/email-clients.rst，以获取关于电子邮件客户端和邮件列表
礼仪的建议。

### 在电子邮件讨论中使用裁剪的交错式回复


在 Linux 内核开发讨论中，强烈不鼓励在顶部回复（top-posting）。交错式（或“内联”）
回复使对话更容易跟踪。更多细节请参阅：
https://en.wikipedia.org/wiki/Posting_style#Interleaved_style

```

  A: http://en.wikipedia.org/wiki/Top_post
  Q: Where do I find info about this thing called top-posting?
  A: Because it messes up the order in which people normally read text.
  Q: Why is top-posting such a bad thing?
  A: Top-posting.
  Q: What is the most annoying thing in e-mail?

```
类似地，请裁剪掉所有与你的回复无关的、不需要的引用。这使回复更容易找到，并节省
时间
```

  A: No.
  Q: Should I include quotations after my reply?

```

### 不要气馁——也不要不耐烦


在你提交改动之后，要有耐心并等待。审查者是很忙的人，可能不会马上处理你的补丁。

曾几何时，补丁会毫无评论地消失在虚空之中，但现在的开发流程运作得比那顺畅多了。你
应该会在几周内（通常 2-3 周）收到评论；如果那没有发生，请确保你已把补丁发送到了
正确的地方。在重新提交或 ping 审查者之前，至少等待一周——在合并窗口等繁忙时期可能
要更长。

在过了几个
```

   [PATCH Vx RESEND] sub/sys: Condensed patch summary

```
之后重新发送补丁或补丁系列也是可以的。当你提交的是修改过的版本时，不要加上
“RESEND”——“RESEND”只适用于重新提交一个与上一次提交相比没有任何修改的补丁或
补丁系列。

### 在主题中包含 PATCH


由于发给 Linus 和 linux-kernel 的电子邮件流量很高，用 [PATCH] 作为主题行前缀是
一种常见约定。这让 Linus 和其他内核开发者更容易地将补丁与其他电子邮件讨论区分开
来。

`git send-email` 会自动为你做这件事。

### 签署你的工作——开发者原产地证书（Developer's Certificate of Origin）


为了改善对“谁做了什么”的追踪，特别是对于可能经过好几层维护者才渗透到其在内核中
的最终归宿的补丁，我们引入了一种在通过电子邮件往来的补丁上的“sign-off（签署）”
程序。

签署是补丁说明末尾的一行简单文字，它证明你编写了它，或者你有权以开源补丁的形式
将其传递下去。规则相当简单：如果你能够证明以下几点：

##### 开发者原产地证书 1.1


通过向本项目作出贡献，我证明：

        (a) 该贡献全部或部分由我创建，并且我有权在文件中指明的开源许可下提交它；或者

        (b) 该贡献基于我尽我所知受适当开源许可覆盖的先前工作，并且我有权在该许可下
            提交该工作及其修改（无论全部还是部分由我创建），使用与文件中指明的相同
            的开源许可（除非我被允许在不同的许可下提交），或者

        (c) 该贡献是由某个其他直接向我提供、并证明了 (a)、(b) 或 (c) 的人提供的，
            且我未对其做出修改。

        (d) 我理解并同意，本项目及该贡献是公开的，并且该贡献的记录（包括我随之一并
            提交的所有的个人信息，包括我的签署）将被无限期保留，并可能根据本项目或
            相关的开源许可被重新分发。

```

	Signed-off-by: Random J Developer <random@developer.example.org>

```
使用一个已知的身份（抱歉，不接受匿名贡献）。如果你使用 `git commit -s`，这会为你
自动完成。Revert（回退）也应该包含 “Signed-off-by”。`git revert -s` 会为你做这件事。

有些人还在末尾加上额外的标签。它们现在只会被忽略，但你可以这样做来标记公司内部
流程，或者仅仅是指出关于签署的某个特殊细节。

在作者签署之后的任何其他 SoB（Signed-off-by:）都来自处理和传递补丁、但并未参与其
开发的人。SoB 链应当反映补丁在传播给维护者并最终到达 Linus 的过程中所经历的**真实**
路径，第一个 SoB 条目表示单一作者的首要作者身份。

### 何时使用 Acked-by:、Cc: 和 Co-developed-by:


Signed-off-by: 标签表示签署者参与了补丁的开发，或者他/她处于补丁的传递路径中。

如果一个人没有直接参与补丁的准备或处理，但希望表明并记录他们对它的认可，那么他们
可以要求在其补丁的变更日志中添加一个 Acked-by: 行。

Acked-by: 用于那些以某种方式对所影响的代码负责或参与其中者。最常见的情况是，当
维护者既没有贡献也没有转发该补丁时，由该维护者使用。

Acked-by: 也可能被其他利益相关者使用，例如具有领域知识的人（例如被修改代码的原始
作者）、内核 uAPI 补丁的用户空间侧审查者，或某个功能的关键用户。可选情况下，在
```

	Acked-by: The Stakeholder <stakeholder@example.org> # As primary user

```
Acked-by: 不如 Signed-off-by: 正式。它是一条记录，表明 ack 者至少审查过该补丁并
表示了接受。因此，补丁合并者有时会手动将一个 ack 者的“yep, looks good to me”转换
为 Acked-by:（但请注意，通常最好要求一个明确的 ack）。

Acked-by: 也不如 Reviewed-by: 正式。例如，维护者可能用它来表示他们同意补丁落地，
但他们可能没有像提供 Reviewed-by: 那样彻底地审查过它。类似地，一个关键用户可能
没有对补丁进行技术审查，但他们可能对该总体方法、功能或面向用户的接口感到满意。

Acked-by: 并不一定表示对整个补丁的认可。例如，如果一个补丁影响多个子系统，并且
有一个来自某个子系统维护者的 Acked-by:，那么这通常表示只认可影响该维护者代码的
那部分。这里应当运用判断力。当有疑问时，人们应该参考邮件列表归档中的原始讨论。
在这种情况下也可以使用 “# Suffix” 来澄清。

如果一个人曾经有机会对一个补丁发表评论，但没有提供此类评论，你可以选择性地添加一个
`Cc:` 标签到该补丁。该标签记录了潜在的利益相关方已被包含在讨论中。注意，这是你
可能能够在未获被点名者明确许可的情况下使用的仅有的三种标签之一（详见下文“标记人员
需要许可”）。

Co-developed-by: 声明该补丁由多名开发者共同创建；当多个人在一个补丁上协作时，它用于
向共同作者（除了由 From: 标签归属的作者之外）归属贡献。由于 Co-developed-by: 表示
作者身份，每个 Co-developed-by: 必须紧跟着相应共同作者的 Signed-off-by:。适用标准的
签署程序，即 Signed-off-by: 标签的排序应当尽可能反映补丁的时间顺序历史，无论作者是
通过 From: 还是 Co-developed-by: 归属的。值得注意的是，最后一个 Signed-off-by: 必须
始终是提交该补丁的开发者的。

注意，当 From: 作者同时也是电子邮件头 From: 行中列出的那个人（和那个电子邮件）时，
From: 标签是可选的。

```

	<changelog>

	Co-developed-by: First Co-Author <first@coauthor.example.org>
	Signed-off-by: First Co-Author <first@coauthor.example.org>
	Co-developed-by: Second Co-Author <second@coauthor.example.org>
	Signed-off-by: Second Co-Author <second@coauthor.example.org>
	Signed-off-by: From Author <from@author.example.org>

```

```

	From: From Author <from@author.example.org>

	<changelog>

	Co-developed-by: Random Co-Author <random@coauthor.example.org>
	Signed-off-by: Random Co-Author <random@coauthor.example.org>
	Signed-off-by: From Author <from@author.example.org>
	Co-developed-by: Submitting Co-Author <sub@coauthor.example.org>
	Signed-off-by: Submitting Co-Author <sub@coauthor.example.org>


```
### 使用 Reported-by:、Tested-by:、Reviewed-by:、Suggested-by: 和 Fixes:


Reported-by 标签向发现并报告缺陷的人致谢，并有望激励他们在未来再次帮助我们。该标签
用于缺陷；请不要用它来为功能请求致谢。该标签后面应跟随一个指向报告的 Closes: 标签，
除非该报告在网上不可用。如果补丁修复的只是所报告问题的一部分，可以使用 Link: 标签
代替 Closes:。注意，Reported-by 标签是你可能能够在未获被点名者明确许可的情况下使用的
仅有的三种标签之一（详见下文“标记人员需要许可”）。

Tested-by: 标签表示该补丁已被点名的人（在某个环境中）成功测试过。该标签通知维护者
已经执行了某些测试，提供了一种为未来补丁寻找测试者的手段，并确保测试者获得致谢。

Reviewed-by: 则相反，表示补丁已经过审查，并根据审查者声明被认为是可接受的：

##### 审查者监督声明


通过提供我的 Reviewed-by: 标签，我声明：

	 (a) 我已经对该补丁进行了技术审查，以评估其是否适合并准备好被纳入
	     主线内核。

	 (b) 任何与补丁相关的问题、关切或疑问都已经反馈给了提交者。我对
	     提交者对我评论的回应感到满意。

	 (c) 尽管该提交中可能有可以改进的地方，但我相信，在此时，(1) 它是
	     对内核有价值的一处修改，并且 (2) 不存在反对将其纳入的已知问题。

	 (d) 虽然我已经审查了该补丁并相信它是健全的，但我（除非在别处明确
	     说明）不对其将在任何给定情况下达到其既定目的或正常运行做出
	     任何明示担保或保证。

Reviewed-by 标签是一种意见声明，即该补丁是对内核适当的修改，不存在剩余严重技术问题。
任何感兴趣的审查者（已经做了这项工作的人）都可以为一个补丁提供一个 Reviewed-by 标签。
该标签用于向审查者致谢，并通知维护者对该补丁已经进行的审查程度。由了解该主题领域并
进行彻底审查的审查者提供的 Reviewed-by: 标签，通常会增加你的补丁进入内核的可能性。

Tested-by 和 Reviewed-by 标签，一旦在邮件列表上从测试者或审查者处收到，就应当由作者
在发送下一版本时添加到适用的补丁上。然而，如果补丁在后续版本中发生了实质性变化，这些
标签可能不再适用，因此应当被移除。通常，移除某人的 Acked-by、Tested-by 或 Reviewed-by
标签应当在补丁变更日志中附带解释来说明（在 '---' 分隔符之后）。

Suggested-by: 标签表示该补丁的想法是由被点名的人提出的，并确保该人因其想法获得致谢：
如果我们勤勉地向我们的想法报告者致谢，他们就有望受到激励，在未来再次帮助我们。注意，
这是你可能在未获被点名者明确许可的情况下使用的仅有的三种标签之一（详见下文“标记人员
需要许可”）。

Fixes: 标签表示该补丁修复了先前某个提交中的缺陷。它用于便于确定问题起源于何处，这
有助于审查缺陷修复。该标签还帮助稳定内核团队确定哪些稳定内核版本应该收到你的修复。
这是表明由补丁修复的缺陷的首选方法。更多细节请参阅 describe_changes。

注意：附加 Fixes: 标签并不会规避稳定内核规则流程，也不会规避在所有稳定补丁候选上
Cc: stable@vger.kernel.org 的要求。更多信息，请阅读
Documentation/process/stable-kernel-rules.rst。

最后，虽然提供标签是受欢迎的、通常也非常受赞赏，但请注意，签署者（即提交者和维护者）
可以酌情决定是否应用所提供的标签。

### 标记人员需要许可


在把上述标签添加到你的补丁时要小心，因为除了 Cc:、Reported-by: 和 Suggested-by: 之外，
全部都需要被点名者的明确许可。对于这三项，如果此人根据 lore 归档或提交历史，使用该
姓名和电子邮件地址为 Linux 内核做出过贡献，则隐式许可就足够了——而对于 Reported-by:
和 Suggested-by:，则必须是在公开场合进行了报告或建议。注意，bugzilla.kernel.org 在这个
意义上是一个公共场所，但在那里使用的电子邮件地址是私有的；因此不要在标签中暴露它们，
除非此人曾在先前的贡献中使用过它们。

### 使用 Assisted-by:


如果你在创建补丁时使用了任何类型的高级编码工具，你需要通过添加一个 Assisted-by 标签
来承认这一使用。不这样做可能会妨碍你的工作被接受。有关编码助手致谢的详细信息，请
参阅 Documentation/process/coding-assistants.rst。

### 规范的补丁格式


本节描述补丁本身应当如何格式化。请注意，如果你的补丁存储在 `git` 仓库中，可以用
`git format-patch` 获得正确的补丁格式。不过，工具不能创建必要的文本，所以还是请阅读
下面的说明。

##### 主题行


```

    Subject: [PATCH 001/123] subsystem: summary phrase

```
规范的补丁消息正文包含以下内容：

  - 一个 `from` 行，指定补丁作者，后跟一个空行（仅当发送补丁的人不是作者时才需要）。

  - 说明的主体，在 75 列处换行，它将被复制到永久的变更日志中描述此补丁。

  - 一个空行。

  - 上面描述的 `Signed-off-by:` 行，它们也将进入变更日志。

  - 一个仅包含 `---` 的标记行。

  - 任何不适合放入变更日志的额外评论。

  - 实际的补丁（`diff` 输出）。

主题行格式使得按主题行进行字母排序的电子邮件变得非常容易——几乎任何电子邮件阅读器
都会支持这一点——因为序列号是零填充的，数字排序和字母排序是相同的。

电子邮件主题中的 `subsystem` 应当标识正在被打补丁的内核的区域或子系统。

电子邮件主题中的 `summary phrase` 应当简明地描述该电子邮件所包含的补丁。`summary
phrase` 不应是一个文件名。不要对一整个补丁系列（这里的 `patch series` 是多个相关补丁
的有序序列）中的每个补丁使用相同的 `summary phrase`。

请记住，你的电子邮件的 `summary phrase` 成为该补丁的全球唯一标识符。它一直传播到
`git` 变更日志中。`summary phrase` 之后可能用于引用该补丁的开发者讨论中。人们会想要
google 该 `summary phrase` 来阅读关于该补丁的讨论。几个月或三个月后，当他们使用 `gitk`
或 ``git log --oneline`` 这样的工具翻阅或许数千个补丁时，它也将是人们可能快速看到的
唯一东西。

由于这些原因，`summary` 必须不超过 70-75 个字符，并且它必须既描述补丁改变了什么，也
描述为什么补丁可能是必要的。既要简洁又要具有描述性是很有挑战性的，但那正是一个写得
好的摘要应该做到的。

`summary phrase` 可以以方括号括起来的标签作为前缀：“Subject: [PATCH <tag>...]
<summary phrase>”。这些标签不被认为是 summary phrase 的一部分，而是描述补丁应当如何
被对待。常见的标签可能包括一个版本描述符，如果补丁的多个版本是作为对评论的回应而发出
的（即“v1, v2, v3”），或者“RFC”以表示请求评论。

如果一个补丁系列中有四个补丁，各个补丁可以这样编号：1/4、2/4、3/4、4/4。这确保开发者
理解补丁应该被应用的顺序，以及他们已经审查或应用了该补丁系列中的所有补丁。

```

    Subject: [PATCH 2/5] ext2: improve scalability of bitmap searching
    Subject: [PATCH v2 01/27] x86: fix eflags tracking
    Subject: [PATCH v2] sub/sys: Condensed patch summary
    Subject: [PATCH v2 M/N] sub/sys: Condensed patch summary

```
##### From 行


`from` 行必须是消息正文的第一行，并且具有以下形式：

        From: Patch Author <author@example.com>

`from` 行指定谁将在永久变更日志中被记为补丁的作者。如果缺少 `from` 行，则将使用
电子邮件头中的 `From:` 行来确定变更日志中的补丁作者。

作者可以通过向 `from` 和 `SoB` 行添加组织名称来表明其隶属关系或工作的赞助方，例如：

	From: Patch Author (Company) <author@example.com>

##### 说明主体


说明主体将被提交到永久的源代码变更日志，因此对于一个早已忘记了可能导致此补丁的
讨论即时细节的合格读者来说，应该说得通。包含补丁所解决的失败的**症状**（内核日志
消息、oops 消息等）对于可能在提交日志中搜索适用补丁的人特别有用。文本应当写得足够
详细，以便当在几周、几个月甚至几年后阅读时，它能给读者提供把握补丁**为何**被创建的
所需细节。

如果一个补丁修复了编译失败，可能没有必要包含**所有**的编译失败；只要足够让搜索该补丁
的人能找到它即可。正如 `summary phrase` 一样，既简洁又具有描述性很重要。

###### 提交消息中的回溯（Backtraces）


回溯有助于记录导致问题的调用链。然而，并非所有的回溯都有帮助。例如，早期启动的调用
链是独特而明显的。然而，逐字复制完整的 dmesg 输出，会增加像时间戳、模块列表、寄存器和
栈转储这样分散注意力的信息。

因此，最有用的回溯应该从转储中提取相关信息，这使得更容易聚焦于真正的
```

  unchecked MSR access error: WRMSR to 0xd51 (tried to write 0x0000000000000064)
  at rIP: 0xffffffffae059994 (native_write_msr+0x4/0x20)
  Call Trace:
  mba_wrmsr
  update_domains
  rdtgroup_mkdir

```
##### 评论（Commentary）


`---` 标记行起着关键的作用，即为补丁处理工具标记变更日志消息在哪里结束。

`---` 标记之后额外评论的一个好用途是用于 `diffstat`，以显示哪些文件发生了变化，以及
每个文件插入和删除的行数。`diffstat` 在较大的补丁上特别有用。如果你打算在 `---` 标记
之后包含一个 `diffstat`，请使用 `diffstat` 选项 `-p 1 -w 70`，以便文件名从内核源代码
树的顶部列出，并且不要占用过多的横向空间（轻松适配 80 列，或许加上一些缩进）。（`git`
默认生成适当的 diffstat。）

其他只与当下或维护者相关、不适合放入永久变更日志的评论，也应该放在这里。这类评论的
一个好例子是 `patch changelogs`，用于描述补丁 v1 和 v2 版本之间的变化。

请把这些信息放在 `---` 行的**后面**，该行将变更日志与补丁的其余部分分隔开。版本信息不是
被提交到 git 树的变更日志的一部分。它是给审查者的额外信息。如果它放在提交标签上方，
就需要手动交互来移除它。如果它在分隔线下方，则在应用补丁时会自动被剥离。如果可用，
添加指向前几版补丁的链接（例如，
```

  <commit message>
  ...
  Signed-off-by: Author <author@mail>
  ---
  V2 -> V3: Removed redundant helper function
  V1 -> V2: Cleaned up coding style and addressed review comments

  v2: https://lore.kernel.org/bar
  v1: https://lore.kernel.org/foo

  path/to/file | 5+++--
  ...

```
关于正确补丁格式的更多细节，请参阅以下参考资料。

### 显式 In-Reply-To 头部


手动添加 In-Reply-To: 头部到补丁（例如，当使用 `git send-email` 时）以将补丁与先前的
相关讨论关联起来，是很有帮助的，例如将缺陷修复链接到带有缺陷报告的电子邮件。然而，对于
多补丁系列，通常最好避免使用 In-Reply-To: 链接到该系列的旧版本。这样，补丁的多个版本
就不会在电子邮件客户端中变成一片无法管理的引用森林。如果链接有帮助，你可以使用
https://lore.kernel.org/ 重定向器（例如，在封面电子邮件文本中）来链接到补丁系列的早期
版本。

### 提供基础树信息


当其他开发者收到你的补丁并开始审查流程时，考虑到如今存在的大量维护者树，他们绝对有
必要知道你的工作所基于的基础提交/分支是什么。请再次注意上面解释过的 MAINTAINERS 文件
中的 **T:** 条目。

对于那些试图在维护者开始审查之前运行一系列测试以确立你的提交质量的自动化 CI 流程来说，
这一点更为重要。

如果你使用 `git format-patch` 来生成补丁，你可以通过使用 `--base` 标志，在你的提交中
自动包含基础树信息。最简单最方便的使用方式
```

    $ git checkout -t -b my-topical-branch master
    Branch 'my-topical-branch' set up to track local branch 'master'.
    Switched to a new branch 'my-topical-branch'

    [perform your edits and commits]

    $ git format-patch --base=auto --cover-letter -o outgoing/ master
    outgoing/0000-cover-letter.patch
    outgoing/0001-First-Commit.patch
    outgoing/...

```
当你打开 `outgoing/0000-cover-letter.patch` 进行编辑时，你会注意到它在最底部会有
`base-commit:` 尾注，这为审查者和 CI 工具提供了足够的信息
```

    $ git checkout -b patch-review [base-commit-id]
    Switched to a new branch 'patch-review'
    $ git am patches.mbox
    Applying: First Commit
    Applying: ...

```
请参阅 `man git-format-patch` 以了解更多关于此选项的信息。

    The `--base` feature was introduced in git version 2.9.0.

如果你不是使用 git 来格式化你的补丁，你仍然可以包含相同的 `base-commit` 尾注，以指示
你的工作所基于的树的提交哈希。你应该把它加在封面信中，或者加在系列的第一个补丁中，并且
它应该放在 `---` 行下方，或者所有其他内容的最底部，紧在你的电子邮件签名之前。

请确保基础提交是在一个官方的维护者/主线树中，而不是在某个只有你能访问的内部树中——
否则它就毫无价值了。

### 工具（Tooling）


这个过程的许多技术方面可以使用 b4 自动化，其文档位于 <https://b4.docs.kernel.org/en/latest/>。
这可以帮助处理诸如跟踪依赖、运行 checkpatch 以及格式化和发送邮件等事情。

### 参考资料


Andrew Morton, "The perfect patch" (tpp).
  <https://www.ozlabs.org/~akpm/stuff/tpp.txt>

Jeff Garzik, "Linux kernel patch submission format".
  <https://web.archive.org/web/20180829112450/http://linux.yyz.us/patch-format.html>

Greg Kroah-Hartman, "How to piss off a kernel subsystem maintainer".
  <http://www.kroah.com/log/linux/maintainer.html>

  <http://www.kroah.com/log/linux/maintainer-02.html>

  <http://www.kroah.com/log/linux/maintainer-03.html>

  <http://www.kroah.com/log/linux/maintainer-04.html>

  <http://www.kroah.com/log/linux/maintainer-05.html>

  <http://www.kroah.com/log/linux/maintainer-06.html>

Kernel Documentation/process/coding-style.rst

Linus Torvalds's mail on the canonical patch format:
  <https://lore.kernel.org/r/Pine.LNX.4.58.0504071023190.28951@ppc970.osdl.org>

Andi Kleen, "On submitting kernel patches"
  Some strategies to get difficult or controversial changes in.

  http://halobates.de/on-submitting-patches.pdf
