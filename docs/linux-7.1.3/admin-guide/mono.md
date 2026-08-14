### Mono(tm) 二进制内核支持（Linux）


要配置 Linux 以自动执行基于 Mono 的 .NET 二进制文件（以 .exe 文件形式），而无需
使用 mono CLR 包装器，你可以使用 BINFMT_MISC 内核支持。

完成以下步骤后，这将允许你像执行其他任何程序一样执行基于 Mono 的 .NET 二进制文件：

1) 你必须首先安装 Mono CLR 支持，可以通过下载二进制包、源代码 tarball 或从 Git
   安装。若干发行版的二进制包可在以下位置找到：

	https://www.mono-project.com/download/

   Mono 的编译说明可在以下位置找到：

	https://www.mono-project.com/docs/compiling-mono/linux/

   一旦安装了 Mono CLR 支持，只需确认 `/usr/bin/mono`（可能位于别处，例如
   `/usr/local/bin/mono`）可以正常工作。

2) 你必须将 BINFMT_MISC 编译为模块或编入内核（`CONFIG_BINFMT_MISC`）并正确设置。
   如果你选择将其编译为模块，则必须使用 modprobe/insmod 手动插入，因为 kmod 无法
   轻易地由 binfmt_misc 支持。阅读本目录中的 `binfmt_misc.txt` 文件以了解更多
   关于配置过程的信息。

3) 将以下条目添加到 `/etc/rc.local` 或类似的在系统启动时运行的脚本中：

   .. code-block:: sh

    # Insert BINFMT_MISC module into the kernel
    if [ ! -e /proc/sys/fs/binfmt_misc/register ]; then
        /sbin/modprobe binfmt_misc
	# Some distributions, like Fedora Core, perform
	# the following command automatically when the
	# binfmt_misc module is loaded into the kernel
	# or during normal boot up (systemd-based systems).
	# Thus, it is possible that the following line
	# is not needed at all.
	mount -t binfmt_misc none /proc/sys/fs/binfmt_misc
    fi

    # Register support for .NET CLR binaries
    if [ -e /proc/sys/fs/binfmt_misc/register ]; then
	# Replace /usr/bin/mono with the correct pathname to
	# the Mono CLR runtime (usually /usr/local/bin/mono
	# when compiling from sources or CVS).
        echo ':CLR:M::MZ::/usr/bin/mono:' > /proc/sys/fs/binfmt_misc/register
    else
        echo "No binfmt_misc support"
        exit 1
    fi

4) 确认 `.exe` 二进制文件无需包装脚本即可运行，只需直接启动该 `.exe` 文件。
```

	/usr/bin/xsd.exe

   .. note::

      If this fails with a permission denied error, check
      that the ``.exe`` file has execute permissions.

```
