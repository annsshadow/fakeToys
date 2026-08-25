## 创建拉取请求


本章描述维护者如何创建并向其它维护者提交拉取请求。这对于将一个维护者树中的更改
转移到另一个维护者树中很有用
本文档主要由 Tobin C. Harding（彼时他还不是一位经验丰富的维护者）根据 Greg
Kroah-Hartman Linus Torvalds LKML 上的评论撰写。由 Jonathan Corbet Mauro Carvalho Chehab 提供建议和修正。误解并非有意但不可避免，请将指责指Tobin C. Harding <me@tobin.cc>
```

	https://lore.kernel.org/r/20171114110500.GA21175@kroah.com


```
### 创建分支


首先，你需要将希望包含在拉取请求中的所有更改放在一个单独的分支上。通常你会基于
你打算发送拉取请求的开发者树中的某个分支来创建此分支
为了创建拉取请求，你必须首先为你刚刚创建的分支打上标签。建议你选择一个有意义标签名，以一种你和他人即使过一段时间也能理解的方式。一个好的做法是，在名称中包来源子系统的指示符以及目标内核版本
Greg 提供了如下建议。一个包drivers/char 各种杂项内容、要在内核版4.15-rc1
应用的拉取请求可以命名为 `char-misc-4.15-rc1`。如果这样一个标签是从一个分支产生的
```

        git tag -s char-misc-4.15-rc1 char-misc-next

```
这将创建一个名`char-misc-4.15-rc1`、基`char-misc-next` 分支最后一个提交的
带签名标签，并用你的 gpg 密钥签名（参Documentation/maintainer/configure-git.rst）
Linus 只接受基于带签名标签的拉取请求。其它维护者可能有所不同
当你运行上述命令时，`git` 会让你进入一个编辑器，并要求你描述该标签。在这种情况下，
你是在描述一个拉取请求，所以概述这里包含什么、为什么应该合并，以及（如果有的话做了什么测试。所有这些信都将最终进入标签本身，然后进入维护者在合并拉取请求时（如果/
当他们合并时）所做的合并提交中。所以要把它写好，因为它将永远留在内核树中
```

	Anyway, at least to me, the important part is the *message*. I want
	to understand what I'm pulling, and why I should pull it. I also
	want to use that message as the message for the merge, so it should
	not just make sense to me, but make sense as a historical record
	too.

	Note that if there is something odd about the pull request, that
	should very much be in the explanation. If you're touching files
	that you don't maintain, explain _why_. I will see it in the
	diffstat anyway, and if you didn't mention it, I'll just be extra
	suspicious.  And when you send me new stuff after the merge window
	(or even bug-fixes, but ones that look scary), explain not just
	what they do and why they do it, but explain the _timing_. What
	happened that this didn't go through the merge window..

	I will take both what you write in the email pull request _and_ in
	the signed tag, so depending on your workflow, you can either
	describe your work in the signed tag (which will also automatically
	make it into the pull request email), or you can make the signed
	tag just a placeholder with nothing interesting in it, and describe
	the work later when you actually send me the pull request.

	And yes, I will edit the message. Partly because I tend to do just
	trivial formatting (the whole indentation and quoting etc), but
	partly because part of the message may make sense for me at pull
	time (describing the conflicts and your personal issues for sending
	it right now), but may not make sense in the context of a merge
	commit message, so I will try to make it all make sense. I will
	also fix any speeling mistaeks and bad grammar I notice,
	particularly for non-native speakers (but also for native ones
	;^). But I may miss some, or even add some.

			Linus

```
```

	Char/Misc patches for 4.15-rc1

	Here is the big char/misc patch set for the 4.15-rc1 merge window.
	Contained in here is the normal set of new functions added to all
	of these crazy drivers, as well as the following brand new
	subsystems:
		- time_travel_controller: Finally a set of drivers for the
		  latest time travel bus architecture that provides i/o to
		  the CPU before it asked for it, allowing uninterrupted
		  processing
		- relativity_shifters: due to the affect that the
		  time_travel_controllers have on the overall system, there
		  was a need for a new set of relativity shifter drivers to
		  accommodate the newly formed black holes that would
		  threaten to suck CPUs into them.  This subsystem handles
		  this in a way to successfully neutralize the problems.
		  There is a Kconfig option to force these to be enabled
		  when needed, so problems should not occur.

	All of these patches have been successfully tested in the latest
	linux-next releases, and the original problems that it found have
	all been resolved (apologies to anyone living near Canberra for the
	lack of the Kconfig options in the earlier versions of the
	linux-next tree creations.)

	Signed-off-by: Your-name-here <your_email@domain>


```
标签消息格式就像 git 提交 id 一样。顶部一行作为“摘要主题”，并确保在底部签名
现在你有了一个本地带签名标签，你需要将其推送到一个公开位置
```

	git push origin char-misc-4.15-rc1


```
### 创建拉取请求


最后要做的是创建拉取请求消息。`git` 可以很方便地通过 `git request-pull` 命令为你
完成，但它需要一点帮助来确定你想拉取什么，以及基于什么进行拉取（以显示正确的待拉更改diffstat）```

	git request-pull master git://git.kernel.org/pub/scm/linux/kernel/git/gregkh/char-misc.git/ char-misc-4.15-rc1

```
```

	This is asking git to compare the difference from the
	'char-misc-4.15-rc1' tag location, to the head of the 'master'
	branch (which in my case points to the last location in Linus's
	tree that I diverged from, usually a -rc release) and to use the
	git:// protocol to pull from.  If you wish to use https://, that
	can be used here instead as well (but note that some people behind
	firewalls will have problems with https git pulls).

	If the char-misc-4.15-rc1 tag is not present in the repo that I am
	asking to be pulled from, git will complain saying it is not there,
	a handy way to remember to actually push it to a public location.

	The output of 'git request-pull' will contain the location of the
	git tree and specific tag to pull from, and the full text
	description of that tag (which is why you need to provide good
	information in that tag).  It will also create a diffstat of the
	pull request, and a shortlog of the individual commits that the
	pull request will provide.

```
Linus 回应说他倾向于偏`git://` 协议。其它维护者可能有不同的偏好。此外，请注意，
如果你在没有带签名标签的情况下创建拉取请求，那么 `https://` 可能是更好的选择。完讨论请参阅原始邮件线程

### 提交拉取请求


拉取请求的提交方式与普通的补丁相同。作为内联邮件发送给维护者，并抄LKML 以及任何
子系统特定的列表（如果需要）。向 Linus 提交的拉取请求通常具有以下主题```

	[GIT PULL] <subsystem> changes for v4.15-rc1

```
