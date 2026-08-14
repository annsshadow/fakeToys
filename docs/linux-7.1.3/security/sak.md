## Linux 安全注意键（SAK）处理


:Date: 18 March 2001
:Author: Andrew Morton

操作系统的安全注意键（Secure Attention Key）是一种安全工具，用于防范特洛伊木马式的密码窃取程序。它是一种不可击败的方式，用于杀掉所有可能伪装成登录应用程序的程序。需要教导用户在登录系统之前输入此按键序列。

在 PC 键盘上，Linux 提供两种相似但不同的方式来提供 SAK。其一是 ALT-SYSRQ-K 序列。你不应使用此序列。它仅在内核编译时开启了 sysrq 支持时才可用。

生成 SAK 的正确方式是使用 `loadkeys` 定义按键序列。无论内核是否编译了 sysrq 支持，这都能工作。

当键盘处于原始（raw）模式时，SAK 才能正确工作。这意味着一旦定义，SAK 会杀掉正在运行的 X 服务器。如果系统处于运行级别 5，X 服务器会重新启动。这正是你所期望发生的情况。

应该使用什么按键序列？CTRL-ALT-DEL 用于重启机器。CTRL-ALT-BACKSPACE 对 X 服务器是特殊的。我们选择 CTRL-ALT-PAUSE。

```

	echo "control alt keycode 101 = SAK" | /bin/loadkeys

```

就这样！只有超级用户（superuser）可以重新编程 SAK 键。



  1. 据称 Linux SAK 并非实现 C2 级安全的系统所要求的"真正 SAK"。本文作者不知道原因。



  2. 在 PC 键盘上，SAK 会杀掉所有打开了 /dev/console 的应用程序。

     不幸的是，这包括一些你实际上并不想杀掉的进程。这是因为这些应用程序错误地持有了 /dev/console 的打开状态。请务必向你的 Linux 发行版供应商抱怨这件事！

     你可以用以下命令识别将被 SAK 杀掉的进程

```

	# ls -l /proc/[0-9]*/fd/* | grep console
	l-wx------    1 root     root           64 Mar 18 00:46 /proc/579/fd/0 -> /dev/console

     然后::

	# ps aux|grep 579
	root       579  0.0  0.1  1088  436 ?        S    00:43   0:00 gpm -t ps/2

     因此 ``gpm`` 会被 SAK 杀掉。这是 gpm 的一个 bug。它应该关闭标准输入。你可以通过找到启动 gpm 的 initscript 并按如下方式修改来规避：

     旧写法::

	daemon gpm

     新写法::

	daemon gpm < /dev/null

     Vixie cron 似乎也有这个问题，需要同样的修改。

     此外，某主流 Linux 发行版在其 rc.sysinit 和 rc 脚本中有以下三行::

	exec 3<&0
	exec 4>&1
	exec 5>&2

     这些命令会导致由 initscripts 启动的**所有**守护进程的文件描述符 3、4 和 5 都连接到 /dev/console。因此 SAK 会把它们全部杀掉。一种规避方法是简单地删除这几行，但这可能导致系统管理应用程序故障——请充分测试一切。

```
