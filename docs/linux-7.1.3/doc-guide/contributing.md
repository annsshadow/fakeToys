
## 如何帮助改进内核文档

文档是任何软件开发项目的重要组成部分。好的文档有助于吸引新开发者，并帮助既有开发者更高效地工作。若没有高质量的文档，大量时间会浪费在逆向工程代码以及犯下本可避免的错误上。

遗憾的是，内核的文档目前距离支撑一个如此规模和重要性的项目所需的水准还相去甚远。

本指南面向那些希望改善这一状况的贡献者。改进内核文档可以由各种技能水平的开发者来完成；它们是学习内核开发流程、并在社区中找到自己位置的一种相对容易的方式。接下来大部分内容是文档维护者列出的、最迫切需要完成的任务清单。

### The documentation TODO list（文档待办清单）

要让我们的文档达到应有的水准，需要完成的任务无穷无尽。这份清单包含了若干重要项，但远非详尽；如果您看到另一种改进文档的方式，请不要犹豫，尽管去做！

#### Addressing warnings（处理警告）

文档构建 currently 会吐出数量惊人的警告。当警告多到那个地步时，有和没有其实没区别；人们会忽略它们，也永远不会注意到自己的改动新增了警告。因此，消除警告是文档待办清单上优先级最高的任务之一。这项任务本身相当直接，但必须以正确的方式去做才能成功。

C 代码编译器发出的警告常常可以被当作误报而打发掉，从而催生出仅仅为了让编译器闭嘴的补丁。文档构建的警告几乎总是指向一个真实的问题；要让这些警告消失，需要理解问题并在源头修复它。因此，修复文档警告的补丁，其变更日志标题大概不应写 “修复一个警告”；它们应当指出被修复的真实问题。

另一个要点是，文档警告常常是由 C 代码中的 kerneldoc 注释问题引起的。虽然文档维护者乐于被抄送这些警告的修复补丁，但文档树往往并非实际携带这些修复的正确子树；它们应当交给相关子系统的维护者。

例如，在一次文档构建中，我抓取了一对警告，大致如下
```

  ./drivers/devfreq/devfreq.c:1818: warning: bad line:
  	- Resource-managed devfreq_register_notifier()
  ./drivers/devfreq/devfreq.c:1854: warning: bad line:
  	- Resource-managed devfreq_unregister_notifier()

```
（这些行为了可读性做了折行）。

快速查看上面提到的源文件，发现了几处 kerneldoc
```

  /**
   * devm_devfreq_register_notifier()
	  - Resource-managed devfreq_register_notifier()
   * @dev:	The devfreq user device. (parent of devfreq)
   * @devfreq:	The devfreq object.
   * @nb:		The notifier block to be unregistered.
   * @list:	DEVFREQ_TRANSITION_NOTIFIER.
   */

```
问题在于缺失了 “*”，这迷惑了构建系统对 C 注释块长什么样的简单化认识。这个问题自 2016 年加入该注释以来就一直存在 —— 整整四年。修复的方法就是补上缺失的星号。快速查看该文件的历史，就能知道主题行（subject line）的常规格式，而 `scripts/get_maintainer.pl` 告诉了我该把它发给谁（将你的补丁路径作为参数传给 scripts/get_maintainer.pl）。得到的补丁
```

  [PATCH] PM / devfreq: Fix two malformed kerneldoc comments

  Two kerneldoc comments in devfreq.c fail to adhere to the required format,
  resulting in these doc-build warnings:

    ./drivers/devfreq/devfreq.c:1818: warning: bad line:
  	  - Resource-managed devfreq_register_notifier()
    ./drivers/devfreq/devfreq.c:1854: warning: bad line:
  	  - Resource-managed devfreq_unregister_notifier()

  Add a couple of missing asterisks and make kerneldoc a little happier.

  Signed-off-by: Jonathan Corbet <corbet@lwn.net>
  ---
   drivers/devfreq/devfreq.c | 4 ++--
   1 file changed, 2 insertions(+), 2 deletions(-)

  diff --git a/drivers/devfreq/devfreq.c b/drivers/devfreq/devfreq.c
  index 57f6944d65a6..00c9b80b3d33 100644
  --- a/drivers/devfreq/devfreq.c
  +++ b/drivers/devfreq/devfreq.c
  @@ -1814,7 +1814,7 @@ static void devm_devfreq_notifier_release(struct device *dev, void *res)

   /**
    * devm_devfreq_register_notifier()
  -	- Resource-managed devfreq_register_notifier()
  + *	- Resource-managed devfreq_register_notifier()
    * @dev:	The devfreq user device. (parent of devfreq)
    * @devfreq:	The devfreq object.
    * @nb:		The notifier block to be unregistered.
  @@ -1850,7 +1850,7 @@ EXPORT_SYMBOL(devm_devfreq_register_notifier);

   /**
    * devm_devfreq_unregister_notifier()
  -	- Resource-managed devfreq_unregister_notifier()
  + *	- Resource-managed devfreq_unregister_notifier()
    * @dev:	The devfreq user device. (parent of devfreq)
    * @devfreq:	The devfreq object.
    * @nb:		The notifier block to be unregistered.
  --
  2.24.1

```
整个过程只花了几分钟。当然，之后我发现别人已经在另一棵树里修复了它，这也凸显了另一条经验：动手之前，永远先检查 linux-next，看看问题是否已经被修复。

其他修复会花更长时间，尤其是那些涉及缺乏文档的结构体成员或函数参数的修复。在这种情况下，有必要弄清楚这些成员或参数的作用，并正确地描述它们。总的来说，这项任务有时会有点乏味，但它极其重要。如果我们真的能消除文档构建中的警告，那么我们就能开始期望开发者避免新增警告。

除了常规文档构建产生的警告之外，您也可以运行 `make refcheckdocs` 来查找对不存在的文档文件的引用。

#### Languishing kerneldoc comments（被闲置的 kerneldoc 注释）

鼓励开发者为他们的代码编写 kerneldoc 注释，但其中许多注释从未被纳入文档构建。这使得这些信息更难找到，例如，也使 Sphinx 无法生成指向该文档的链接。在文档中加入 `kernel-doc` 指令以引入这些注释，可以帮助社区充分获取创建它们所投入工作的价值。

可以使用 `tools/docs/find-unused-docs.sh` 工具来查找这些被忽视的注释。

请注意，最大的价值来自引入已导出函数和数据库结构的文档。许多子系统也为内部使用编写了 kerneldoc 注释；除非这些注释被放在专门面向相关子系统内部开发者的文档中，否则不应被引入文档构建。

#### Typo fixes（错别字修复）

修复文档中的拼写或格式错误，是弄清楚如何创建并发送补丁的一种快捷方式，而且是一项有用的服务。我总是乐于接受这类补丁。话虽如此，一旦您修复了几个，请考虑转向更进阶的任务，把一些错别字留给下一位初学者去处理。

请注意，有些东西 **不是** 错别字，不应被 “修复”：

 - 美式英语和英式英语的拼写在内核文档中都是允许的。无需用其中一种去替换另一种。

 - 句号之后应当跟一个还是两个空格的问题，在内核文档的语境下不应被争论。其他理性分歧的领域，例如 “牛津逗号”，在此同样属于题外话。

与对任一项目的任何补丁一样，请考虑您的改动是否真的让事情变得更好。

#### Ancient documentation（陈旧的文档）

有些内核文档是新鲜的、有人维护的、有用的。有些文档则……不是。布满灰尘、陈旧且不准确的文档会误导读者，并使我们整体文档的可信度受到怀疑。任何能解决这类问题的做法都再欢迎不过。

每当您在处理一份文档时，请考虑它是否仍然新鲜、是否需要更新，或者是否应当被彻底删除。这里有一些您可留意的警示信号：

 - 对 2.x 内核的引用
 - 指向 SourceForge 仓库的指针
 - 多年来历史中除了错别字修复别无他物
 - 对 Git 之前工作流的讨论

当然，最好的做法是让文档保持新鲜，补充所需的任何信息。这样的工作常常需要熟悉相关子系统的开发者的配合。当被礼貌地请求时、且他们的回答被倾听并付诸行动时，开发者通常非常乐意与致力于改进文档的人合作。

有些文档已无可救药；例如我们有时会发现引用了早已从内核中移除的代码的文档。对于移除过时文档有着令人惊讶的抵触情绪，但我们无论如何都应该这么做。文档中的多余垃圾对谁都没好处。

对于那些在严重过时文档中可能还包含一些有用信息、而您又无力更新它的情况，最好的做法也许是
```

  .. warning ::
  	This document is outdated and in need of attention.  Please use
	this information with caution, and please consider sending patches
	to update it.

```
那样，至少我们长期受苦的读者已经被警告该文档可能会把他们引向歧途。

#### Documentation coherency（文档的连贯性）

这里的老手们会记得 1990 年代出现在书架上的那些 Linux 书籍。它们不过是从网上各处搜罗来的文档文件的简单合集。自那以后书籍（大体上）有了改进，但内核文档仍然主要建立在这种模式之上。它是成千上万个文件，几乎每一个都是与其他所有文件隔离写就的。我们并没有一个连贯的内核文档整体；我们只有成千上万份独立的文档。

我们一直试图通过创建一组面向特定读者的 “书”（books）来改善这一状况。这些包括：

 - Documentation/admin-guide/index.rst
 - Documentation/core-api/index.rst
 - Documentation/driver-api/index.rst
 - Documentation/userspace-api/index.rst

以及这份关于文档本身的 “书”。

将文档移动进恰当的 “书” 是一项重要任务，需要持续进行。不过，这项工作有若干挑战。移动文档文件会给那些处理这些文件的人带来短期的痛苦；他们对这类改动缺乏热情是可以理解的。通常可以为移动一份文档找到理由，做一次；但我们实在不想一直把它们搬来搬去。

即便所有文档都待在了正确的位置，我们也只是成功地把一大堆变成了一组较小的堆。试图将所有这些文档编织成一个整体的工作尚未开始。如果您对我们在这一前沿如何推进有高明的主意，我们将非常乐意倾听。

#### Stylesheet improvements（样式表改进）

随着采用 Sphinx，我们得到了比以往美观得多的 HTML 输出。但它仍大有改进空间；Donald Knuth 和 Edward Tufte 是不会被打动的。这需要对我们的样式表做调整，以产生在排版上更合理、更易于访问、更易读的输出。

先提醒一句：如果您接下这项任务，您就步入了经典的 “ bikeshed（无关紧要之事争论）” 领地。即便相对明显的改动，也要准备好面对大量意见和讨论。唉，这就是我们所处的世界的本性。

#### Non-LaTeX PDF build（非 LaTeX 的 PDF 构建）

对于既有大量时间、又具备 Python 技能的人来说，这是一项明显不简单的任务。Sphinx 工具链相对较小且自洽；很容易添加到开发系统中。但构建 PDF 或 EPUB 输出需要安装 LaTeX，而这既不小也不自洽。消除这一点会是一件好事。

最初的希望是使用 rst2pdf 工具（https://rst2pdf.org/）来生成 PDF，但事实证明它不足以胜任该任务。不过，近期 rst2pdf 的开发工作似乎又重新活跃起来，这是一个令人鼓舞的迹象。如果一位有足够动力的开发者能与该项目协作，让 rst2pdf 配合内核文档构建工作，全世界都会永远感激。

#### Write more documentation（撰写更多文档）

自然，内核中大量部分存在严重的文档不足。如果您具备为某个特定内核子系统编写文档的知识，并且有此意愿，请不要犹豫，动手写一些并将其贡献给内核。无数的内核开发者和用户会感谢您。
