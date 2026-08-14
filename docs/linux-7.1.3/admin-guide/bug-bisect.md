## 二分定位回归（Bisecting regression）

本文档介绍如何使用 `git bisect` 找出导致某功能失效的源码改动——例如在将 Linux 从 6.0 升级到 6.1 后，某项功能停止工作。下文聚焦于该过程的核心要点。若要从头开始对内核做二分，更建议改读 Documentation/admin-guide/verify-bugs-bisect-regressions.rst：它对整个过程从头到尾都有描述，并涵盖了多个连内核开发者偶尔也会遗忘的细节。其中还包括尽早识别“二分只会浪费时间、其结果无人关心”的情形——例如问题发生在被内核标记为“受污染（tainted）”的内核中、出现在已废弃的版本里、已经被修复，或是由 Linux 发行方所做的 .config 变更引起的。

## 使用二分查找导致内核问题的改动

*说明：以下过程假设你已经为二分做好了所有准备。这包括：拥有相应源码的 Git 克隆、安装了构建并安装内核所需的软件，以及将一份 .config 文件保存在安全的位置（下例假设为 '~/prepared_kernel_.config'），以便作为每一步二分的干净基准；理想情况下，你还应找到一个完全可靠、直截了当的方式来复现该回归。*

- 准备：开始二分，并告诉 Git 历史中的两个端点：
```
git bisect start
git bisect good v6.0
git bisect bad v6.1
```
除了像 'v6.0' 和 'v6.1' 这样的 Git 标签外，你也可以指定提交 ID。

1. 将准备好的 .config 复制到构建目录并适配：
```
cp ~/prepared_kernel_.config .config
make olddefconfig
```
2. 现在构建、安装并启动内核。它可能因无关原因而失败，例如在二分当前阶段出现了一个编译错误，而该错误会在后续的某次改动中被解决。这种情况下请运行 `git bisect skip` 并返回第 1 步。
3. 检查刚刚构建的内核中，发生回归的那项功能是否正常工作。
```
git bisect good
```
如果它坏了，则运行：
```
git bisect bad
```
请注意，只要搞错一次，就会让余下的二分彻底跑偏。为了避免日后不得不从头再来，你要确保告诉 Git 的结论是正确的；因此，当你的复现手段并不可靠时，多花几分钟做测试往往是明智的。
在发出上述两条命令之一后，Git 通常会检出另一个二分点并打印类似“Bisecting: 675 revisions left to test after this (roughly 10 steps)”的信息。此时请回到第 1 步。
如果 Git 打印的则是类似“cafecaca0c0dacafecaca0c0dacafecaca0c0da is the first bad commit”的信息，那么二分就完成了。此时请转到下面的下一个要点。注意，在显示该行后，Git 会立即展示关于“罪魁祸首（culprit）”的一些细节，包括其补丁说明；这很容易占满你的终端，因此你可能需要向上滚动才能看到提及该提交 ID 的那条消息。
如果你错过了 Git 的输出，随时可以运行 ``git bisect log`` 来打印状态：它会显示还剩多少步，或者给出二分的结果。

- 推荐的辅助步骤：将二分日志和当前的 .config 文件留作缺陷报告之用；此外让 Git 重置源码：
```
git bisect log > ~/bisection-log
cp .config ~/bisection-config-culprit
git bisect reset
```
- 推荐的备选步骤：尝试在最新的代码基之上还原“罪魁祸首”，以检查是否能修复该缺陷；若可以，则验证了二分的正确性，并让开发者能够通过还原来解决该回归。
```
git revert --no-edit cafec0cacaca0
```
Git 可能会拒绝这一操作，例如当二分落在了一个合并提交上时。此时请放弃尝试。如果 Git 因后续改动依赖于该提交而自身无法完成还原，也应同样放弃——除非你二分的是 stable 或 longterm 内核系列，这种情况下你应检出其最新代码基并在那里尝试还原。
如果还原成功，请再构建并测试一个内核，以确认还原是否解决了你的回归。

过程至此完成。现在请按 Documentation/admin-guide/reporting-issues.rst 所述的方式报告该回归。

### 对 linux-next 做二分（Bisecting linux-next）

如果问题出现在 linux-next 中，请对 linux-next 的 'stable' 与 'master' 分支做二分。以下命令用于开始：
```
git bisect start
git bisect good next/stable
git bisect bad next/master
```
'stable' 分支对应的是当前 linux-next 发布（位于 'master' 分支）所基于的 linux-mainline 状态——因此前者不含有在 -next、即 Linus 的树中才会出现的问题。
当跨越很大范围的改动做二分时，你可能会想使用更早的 linux-next 发布来规避问题。遗憾的是，并没有简单的方法可以免去核对：将一个 linux-next 发布与更晚的一个（例如 'next-20241020' 与 'next-20241021'）相互二分是不可能的，因为它们没有共同的历史。

### 延伸阅读（Additional reading material）

- `git bisect 的手册页 <https://git-scm.com/docs/git-bisect>`_
- `用 'git bisect' 对抗回归 <https://git-scm.com/docs/git-bisect-lk2009.html>`_，Git 文档。
- `使用 git bisect 工作 <https://nathanchance.dev/posts/working-with-git-bisect/>`_，内核开发者 Nathan Chancellor。
- `用 Git bisect 弄清问题是在何时引入的 <http://webchick.net/node/99>`_。
- `用 'git bisect run' 实现完全自动化的二分 <https://lwn.net/Articles/317154>`_。

..
end-content
..
本文档由 Thorsten Leemhuis <linux@leemhuis.info> 维护。如发现错别字或小的疏漏，欢迎直接告知他，他会予以修正。若你想以同样（大多为非正式）的方式贡献对正文的改动，出于版权原因请抄送（CC）linux-doc@vger.kernel.org 并附上“ sign-off”（开发者原产地证书）说明，见 Documentation/process/submitting-patches.rst 中的相关章节。
..
本文本可按 GPL-2.0+ 与 CC-BY-4.0 双重许可发布，文件顶部已注明。若你想以 CC-BY-4.0 分发本文，请使用“Linux 内核开发社区”作为作者署名，并附上来源链接：
https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/Documentation/admin-guide/bug-bisect.rst

..
说明：本 RST 文件的内容取自 Linux 内核源码，可按 CC-BY-4.0 使用；但经过处理（例如内核的构建系统）后的文本版本可能包含以更严格许可证发布的内容。
