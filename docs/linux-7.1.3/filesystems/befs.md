
## Linux 下的 BeOS 文件系统


文档最后更新：2001 年 12 月 6 日

## 警告


请务必明白这是 alpha 阶段的软件。这意味着该实现既不完整，也未经过充分测试。

对于此代码可能造成的任何不良后果，本人不承担任何责任！

## 许可证


本软件受 GNU 通用公共许可证保护。完整许可证文本请见 COPYING 文件。
或访问 GNU 网站：<http://www.gnu.org/licenses/licenses.html>

## 作者


代码的大部分由 Will Dyson <will_dyson@pobox.com> 编写。他自 2001 年 8 月 13 日起一直
从事该代码的工作。详见 changelog。

原始作者：Makoto Kato <m_kato@ga2.so-net.ne.jp>

他的原始代码仍可在以下位置找到：
<http://hp.vector.co.jp/authors/VA008030/bfs/>

有人知道 Makoto 更近期的电子邮件地址吗？他对上述地址已不再回复……

该文件系统目前没有维护者。

## 这个驱动是什么？


该模块为 Linux 2.4.1 及更新版本的内核实现了 BeOS（http://www.beincorporated.com/）的原生
文件系统。目前它是一个只读实现。

## 到底叫 BFS 还是 BEFS？


Be, Inc. 曾表示，“BeOS 文件系统在官方上称为 BFS，而非 BeFS”。但 Unixware 的 Boot Filesystem
也叫 bfs，而且它们已经在内核中了。由于这一命名冲突，在 Linux 上 BeOS 文件系统被称为 befs。

## 如何安装


步骤 1. 将 BeFS 补丁安装到 linux 源码树中。

将补丁文件应用到你的内核源码树。假设你的内核源码位于 /foo/bar/linux，补丁文件名为
patch-befs-xxx，则应执行如下操作：

	cd /foo/bar/linux
	patch -p1 < /path/to/patch-befs-xxx

如果打补丁步骤失败（即出现被拒绝的 hunk），你可以尝试自己解决（这并不难），或发邮件向维护者
（Will Dyson <will_dyson@pobox.com>）求助。

步骤 2. 配置并编译内核

Linux 内核有许多编译期选项，其中大多数超出了本文档的范围。我推荐将 Kernel-HOWTO 文档作为
这一主题的良好通用参考。http://www.linuxdocs.org/HOWTOs/Kernel-HOWTO-4.html

```

	cd /foo/bar/linux
	make menuconfig (或 xconfig)

```
BefS 模块并非 Linux 内核的标准组成部分，因此你必须先在“Code maturity level”菜单下启用对
实验性代码的支持。

然后，在“Filesystems”菜单下会出现一个名为“BeFS filesystem (experimental)”或类似名称的选项。
启用该选项（将其编译为模块亦可）。

保存你的内核配置，然后编译内核。

步骤 3. 安装

关于这一关键步骤的说明，请参见内核 howto <http://www.linux.com/howto/Kernel-HOWTO.html>。

## 使用 BFS


要使用 BeOS 文件系统，请使用文件系统类型 ‘befs’。

```

    mount -t befs /dev/fd0 /beos

```
## 挂载选项


=============  ===========================================================
uid=nnn        All files in the partition will be owned by user id nnn.
gid=nnn	       All files in the partition will be in group nnn.
iocharset=xxx  Use xxx as the name of the NLS translation table.
debug          The driver will output debugging information to the syslog.
=============  ===========================================================

## 如何获取最新版本


最新版本目前可在以下位置获取：
<http://befs-driver.sourceforge.net/>

## 已知缺陷？


截至 2002 年 1 月 20 日：

	None

## 特别致谢


Dominic Giampalo …… 撰写了《Practical file system design with Be filesystem》

Hiroyuki Yamada …… 测试了 LinuxPPC。
