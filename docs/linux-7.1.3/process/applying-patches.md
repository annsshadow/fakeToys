
向 Linux 内核应用补丁
++++++++++++++++++++++++++++++++++++

原作者：
	Jesper Juhl，2005 年 8 月


   本文档已经过时。在大多数情况下，你几乎肯定更应该考虑使用 Git，而不是
   手动使用 `patch`。

Linux 内核邮件列表上一个经常被问到的问题是：如何把一个补丁应用到内核上，
或者更具体地说，针对众多开发树/分支之一的补丁应该基于哪个内核版本来打。
希望本文档能够为你解释清楚。

除了讲解如何应用和回退补丁之外，本文档还简要介绍了不同的内核开发树
（并举例说明如何应用它们各自的补丁）。


## 什么是补丁？

一个补丁是一个小型文本文件，包含了两份不同版本的源码树之间变更的增量。
补丁是用 `diff` 程序生成的。

要正确地应用一个补丁，你需要知道它是基于什么基线生成的，以及该补丁会把
源码树变更成哪个新版本。这两项信息都应该存在于补丁文件的元数据中，或者
能够从文件名推断出来。


## 如何应用或回退一个补丁？

你使用 `patch` 程序来应用补丁。`patch` 程序读取一个 diff（或补丁）文件，
并做出其中所描述的、对源码树的修改。

Linux 内核的补丁是相对于包含内核源码目录的父目录生成的。

这意味着补丁文件内部的文件路径会包含它所对应的内核源码目录名
（或者类似 "a/" 和 "b/" 这样的其他目录名）。

由于这不太可能和你本地机器上内核源码目录的名字一致（但对于查看一个
未标注的补丁是基于哪个版本生成的，这个信息通常很有用），你应该先切换到
你的内核源码目录，然后在应用补丁时把补丁文件里文件路径的第一个元素
去掉（`patch` 的 `-p1` 参数就是做这个的）。

要回退一个之前已经应用的补丁，请对 patch 使用 -R 参数。
```

	patch -p1 < ../patch-x.y.z

```
```
	patch -R -p1 < ../patch-x.y.z

```
## 如何把补丁/diff 文件喂给 ``patch``？

（和 Linux 以及其他类 UNIX 操作系统上一贯的情况一样）这可以通过几种
不同的方式来完成。

在下面所有的例子中，我都把文件（以未压缩的形式）喂给 patch：
```

	patch -p1 < path/to/patch-x.y.z

```
如果你只是想能够照着下面的例子做，而不想知道使用 patch 的多种方式，那么
你可以在这里停止阅读本节。

patch 也可以通过 -i 参数来获取要使用的文件名，例如：
```

	patch -p1 -i path/to/patch-x.y.z

```
如果你的补丁文件是用 gzip 或 xz 压缩的，而你又不想在应用前先解压，那么
你可以像这样把它喂给 patch：
```

	xzcat path/to/patch-x.y.z.xz | patch -p1
	bzcat path/to/patch-x.y.z.gz | patch -p1

```
如果你想在应用之前先手动解压补丁文件（我在下面的例子中假定你已经这么做了），
那么你只需运行：
```

	gunzip patch-x.y.z.gz
	xz -d patch-x.y.z.xz

```
这就会留下一个纯文本的 patch-x.y.z 文件，你可以按照自己的喜好通过 stdin
或 `-i` 参数把它喂给 patch。

patch 还有几个其它好用的参数：`-s` 会让 patch 除了错误之外保持安静，这
有助于防止错误信息滚动过快而看不清；`--dry-run` 会让 patch 只打印出将要
发生什么的一览，但不会真正做任何修改。最后，`--verbose` 会让 patch 打印
出更多关于正在进行的工作的信息。


## 打补丁时的常见错误

当 patch 应用一个补丁文件时，它会以不同的方式尝试验证该文件是否正常。

检查文件看起来是否像一个有效的补丁文件，以及检查被修改位置周围的代码
是否与补丁中提供的上下文相匹配，仅仅是 patch 所做的基本健全性检查中的
两项。

如果 patch 遇到了看起来不太对劲的情况，它有两个选择：它可以拒绝应用
修改并中止，也可以尝试找一种办法、通过少量细微调整让补丁得以应用。

一个 patch 会尝试修复的"不太对劲"的例子是：所有上下文都匹配，被修改的
行也匹配，但行号不同。例如，如果补丁在文件中间做了修改，但由于某些原因
文件开头附近被添加或删除了几行，就会发生这种情况。在这种情况下，一切
看起来都正常，只是位置稍微上移或下移了一点，patch 通常会调整行号并应用
补丁。

每当 patch 应用一个它不得不稍作修改才能贴合的补丁时，它会告诉你补丁是
带 **fuzz**（模糊偏移）应用的。你应当对这类修改保持警惕，因为即使 patch
很可能做对了，它也不*总是*做对，结果有时会是错误的。

当 patch 遇到一个它无法用 fuzz 修复的修改时，它会直接拒绝，并留下一个
带有 `.rej` 扩展名（reject 文件，即拒绝文件）的文件。你可以读这个文件来
确切地看到哪些修改无法应用，从而在需要时手动去修复它们。

如果你没有对内核源码应用任何第三方补丁，只有来自 kernel.org 的补丁，并且
你按正确顺序应用补丁，同时自己没有对源码文件做过任何修改，那么你应该
永远看不到来自 patch 的 fuzz 或 reject 消息。如果你还是看到了这样的消息，
那么很可能存在较高的风险：你的本地源码树或补丁文件在某种程度上损坏了。
在这种情况下，你可能应该尝试重新下载补丁，如果情况仍然不对，则建议你从
kernel.org 完整下载一份全新的源码树重新开始。

让我们再多看一些 patch 可能产生的消息。

如果 patch 停下来并出现 `File to patch:` 提示符，那么 patch 找不到要打补丁
的文件。最可能的情况是你忘了指定 -p1，或者你所在的目录不对。较少见的
情况是，你会发现有些补丁需要用 `-p0` 而不是 `-p1` 来应用（读一下补丁文件
应该能看出是不是这种情况——如果是，那么这是创建补丁的人犯的一个错误，
但还不致命）。

如果你得到 `Hunk #2 succeeded at 1887 with fuzz 2 (offset 7 lines).` 或类似
的消息，那么它意味着 patch 不得不调整修改的位置（在本例中，它需要从它
预期做修改的地方移动 7 行才能让修改贴合）。

由此产生的文件可能正常也可能不正常，取决于文件与预期不同的原因。

如果你试图把一个基于不同内核版本生成的补丁，应用到你正要打补丁的版本
上，就经常会发生这种情况。

如果你得到类似 `Hunk #3 FAILED at 2387.` 这样的消息，那么它意味着补丁无法
被正确应用，而 patch 程序无法通过 fuzz 蒙混过去。这会生成一个 `.rej` 文件，
包含导致补丁失败的修改，同时还会生成一个 `.orig` 文件，向你展示无法被
更改的原始内容。

如果你得到 `Reversed (or previously applied) patch detected!  Assume -R? [n]`，
那么 patch 检测到补丁中所包含的修改似乎已经被做过了。

如果你确实之前应用过这个补丁，只是错误地又重新应用了一次，那么只需回答
[否]（[n]），并中止这个补丁。如果你之前应用过这个补丁，并且实际上是想
回退它，却忘了指定 -R，那么你可以在这里回答 [**是**]（[y]），让 patch 为你
回退它。

如果补丁的创建者在生成补丁时把源目录和目标目录弄反了，也会发生这种
情况，而在那种情况下，回退补丁实际上就等于应用它。

类似 `patch: **** unexpected end of file in patch` 或 `patch unexpectedly ends in middle of line` 这样的消息，意味着 patch 无法理解你喂给它的文件。要么是你的下载损坏了，要么是你试图把压缩过的补丁文件在解压之前就喂给 patch，要么是你正在使用的补丁文件在途中被某个邮件客户端或邮件传输代理弄坏了，例如把一行长行拆成了两行。通常这些警告可以通过把被拆开的两行合并（连接）起来轻松修复。

正如我上面已经提到过的，如果你把一个来自 kernel.org 的补丁应用到未修改的
源码树的正确版本上，这些错误就永远不应该发生。所以如果你在 kernel.org 的
补丁上遇到这些错误，那么你应该假定你的补丁文件或你的源码树已经损坏，我
建议你从完整内核源码树和你想要应用的补丁的全新下载重新开始。


## 有没有 ``patch`` 的替代品？

有的，有替代品。

你可以使用 `interdiff` 程序（http://cyberelk.net/tim/patchutils/）来生成一个
表示两个补丁之间差异的补丁，然后应用这个结果。

这将让你能够一步从类似 5.7.2 的版本移动到 5.7.3。interdiff 的 -z 标志甚至
允许你直接把 gzip 或 bzip2 压缩形式的补丁喂给它，而无需使用 zcat、bzcat 或
手动解压。

```

	interdiff -z ../patch-5.7.2.gz ../patch-5.7.3.gz | patch -p1

```
尽管 interdiff 可能会帮你省掉一两个步骤，但通常建议你还是去做那些额外的
步骤，因为 interdiff 在某些情况下可能会出错。

另一个替代品是 `ketchup`，它是一个用于自动下载和应用补丁的 python 脚本
（https://www.selenic.com/ketchup/）。

其它好用的工具有：diffstat，它会显示补丁所做修改的摘要；lsdiff，它会显示
补丁文件中受影响文件的简短列表，以及（可选地）每个补丁开始处的行号；还有
grepdiff，它会显示在补丁中被修改、且补丁包含给定正则表达式的文件列表。


## 我在哪里可以下载到这些补丁？

补丁可在 https://kernel.org/ 获取。大多数最新的补丁都从首页链接出去，但它们
也有各自专门的存放位置。

5.x.y（-stable）和 5.x 补丁存放在

	https://www.kernel.org/pub/linux/kernel/v5.x/

5.x.y 增量补丁存放在

	https://www.kernel.org/pub/linux/kernel/v5.x/incr/

-rc 补丁并不存储在 Web 服务器上，而是根据 git 标签按需生成，例如

	https://git.kernel.org/torvalds/p/v5.1-rc1/v5.0

stable -rc 补丁存放在

	https://www.kernel.org/pub/linux/kernel/v5.x/stable-review/


## 5.x 内核

这些是 Linus 发布的稳定基础版本。编号最高的发布版本是最新的。

如果发现回归或其它严重缺陷，那么会在这个基础之上发布一个 -stable 修复补丁
（见下文）。一旦一个新的 5.x 基础内核发布，就会提供一个补丁，它是前一个
5.x 内核与新 5.x 内核之间的增量。

要应用一个从 5.6 移动到 5.7 的补丁，你要做以下事情（注意此类补丁**不**应用于
5.x.y 内核之上，而是应用基础 5.x 内核之上——如果你需要从 5.x.y 移动到
5.x+1，你需要先回退 5.x.y 补丁）。

```

	# moving from 5.6 to 5.7

	$ cd ~/linux-5.6		# change to kernel source dir
	$ patch -p1 < ../patch-5.7	# apply the 5.7 patch
	$ cd ..
	$ mv linux-5.6 linux-5.7	# rename source dir

	# moving from 5.6.1 to 5.7

	$ cd ~/linux-5.6.1		# change to kernel source dir
	$ patch -p1 -R < ../patch-5.6.1	# revert the 5.6.1 patch
					# source dir is now 5.6
	$ patch -p1 < ../patch-5.7	# apply new 5.7 patch
	$ cd ..
	$ mv linux-5.6.1 linux-5.7	# rename source dir

```
## 5.x.y 内核

带三位版本号的内核是 -stable 内核。它们包含针对某个给定 5.x 内核中发现
的安全问题或重大回归的（较小的）关键修复。

对于想要最新的稳定内核、而又不想帮忙测试开发/实验版本的用户，这是推荐的
分支。

如果没有可用的 5.x.y 内核，那么编号最高的 5.x 内核就是当前的稳定内核。

-stable 团队提供普通补丁和增量补丁。下面是如何应用这些补丁的方法。

#### 普通补丁

这些补丁不是增量的，也就是说，例如 5.7.3 补丁不是应用在 5.7.2 内核源码之上，
而是应用在基础 5.7 内核源码之上。

因此，为了把 5.7.3 补丁应用到你现有的 5.7.2 内核源码上，你必须先回退
5.7.2 补丁（这样你就剩下了一个基础 5.7 内核源码），然后再应用新的 5.7.3
补丁。

```

	$ cd ~/linux-5.7.2		# change to the kernel source dir
	$ patch -p1 -R < ../patch-5.7.2	# revert the 5.7.2 patch
	$ patch -p1 < ../patch-5.7.3	# apply the new 5.7.3 patch
	$ cd ..
	$ mv linux-5.7.2 linux-5.7.3	# rename the kernel source dir

```
#### 增量补丁

增量补丁则不同：它们不是应用在基础 5.x 内核之上，而是应用在之前稳定内核
（5.x.y-1）之上。

```

	$ cd ~/linux-5.7.2		# change to the kernel source dir
	$ patch -p1 < ../patch-5.7.2-3	# apply the new 5.7.3 patch
	$ cd ..
	$ mv linux-5.7.2 linux-5.7.3	# rename the kernel source dir

```
## -rc 内核

这些是发布候选（release-candidate）内核。它们是 Linus 在他认为当前的 git
（内核的源码管理工具）树处于一个足够合理、适合测试的状态时发布的开发内核。

这些内核并不稳定，如果你打算运行它们，应该预期偶尔会出现故障。然而这是
主要开发分支中最稳定的一个，而且它最终也会变成下一个稳定内核，因此让
尽可能多的人来测试它是很重要的。

对于想要帮忙测试开发内核、却又不想运行某些真正实验性东西的人来说，这是
一个很好的分支（这类人应该去看下面关于 -next 和 -mm 内核的小节）。

-rc 补丁不是增量的，它们应用于基础 5.x 内核，就像上面描述的 5.x.y 补丁
一样。-rcN 后缀之前的内核版本号，表示这个 -rc 内核最终会变成的版本。

所以，5.8-rc5 意味着这是 5.8 内核的第五个发布候选，而该补丁应该应用在
5.7 内核源码之上。

```

	# first an example of moving from 5.7 to 5.8-rc3

	$ cd ~/linux-5.7			# change to the 5.7 source dir
	$ patch -p1 < ../patch-5.8-rc3		# apply the 5.8-rc3 patch
	$ cd ..
	$ mv linux-5.7 linux-5.8-rc3		# rename the source dir

	# now let's move from 5.8-rc3 to 5.8-rc5

	$ cd ~/linux-5.8-rc3			# change to the 5.8-rc3 dir
	$ patch -p1 -R < ../patch-5.8-rc3	# revert the 5.8-rc3 patch
	$ patch -p1 < ../patch-5.8-rc5		# apply the new 5.8-rc5 patch
	$ cd ..
	$ mv linux-5.8-rc3 linux-5.8-rc5	# rename the source dir

	# finally let's try and move from 5.7.3 to 5.8-rc5

	$ cd ~/linux-5.7.3			# change to the kernel source dir
	$ patch -p1 -R < ../patch-5.7.3		# revert the 5.7.3 patch
	$ patch -p1 < ../patch-5.8-rc5		# apply new 5.8-rc5 patch
	$ cd ..
	$ mv linux-5.7.3 linux-5.8-rc5		# rename the kernel source dir

```
## -mm 补丁和 linux-next 树

-mm 补丁是 Andrew Morton 发布的实验性补丁。

过去，-mm 树也被用来测试子系统补丁，但这一功能现在通过
`linux-next`（https://www.kernel.org/doc/man-pages/linux-next.html）
树来完成。子系统维护者先把他们的补丁推送到 linux-next，然后在合并窗口期间
直接把它们发送给 Linus。

-mm 补丁充当了新特性和其它未通过子系统树合并的实验性补丁的一种试验场。
一旦此类补丁在 -mm 中证明自身价值一段时间之后，Andrew 就会把它推给 Linus
以合并进主线。

linux-next 树每天更新，并且包含了 -mm 补丁。两者都处于不断变化之中，包含了
许多实验性特性、大量不适合主线的调试补丁等等，而且是本文档所描述的分支中
实验性最强的。

这些补丁不适合用在应当保持稳定的系统上，而且它们比任何其它分支都更具风险
（请确保你有最新的备份——这对任何实验性内核都成立，但对于 -mm 补丁或使用
来自 linux-next 树的内核来说更是如此）。

对 -mm 补丁和 linux-next 的测试非常受欢迎，因为它们的全部意义就在于：在
更改被合并到更稳定、由 Linus 维护的主线树之前，把回归、崩溃、数据损坏 bug、
构建失败（以及任何其它的一般性 bug）揪出来。

但 -mm 和 linux-next 的测试者应当意识到，故障比任何其它树都更常见。


本文档对各种内核树的说明到此结束。希望你现在清楚了如何应用各种补丁并帮忙
测试内核。

感谢 Randy Dunlap、Rolf Eike Beer、Linus Torvalds、Bodo Eggert、Johannes
Stezenbach、Grant Coady、Pavel Machek 以及其它我可能忘记了的、为本文档的
审阅和贡献付出努力的人。
