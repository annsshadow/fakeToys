## SELinux


有关 SELinux 内核子系统的信息可在以下链接找到：

	https://git.kernel.org/pub/scm/linux/kernel/git/pcmoore/selinux.git/tree/README.md

	https://github.com/selinuxproject/selinux-kernel/wiki

有关 SELinux 用户空间的信息可在以下位置找到：

	https://github.com/SELinuxProject/selinux/wiki

如果你想要使用 SELinux，你很可能会想使用发行版提供的策略，或从以下位置安装最新的参考策略版本

	https://github.com/SELinuxProject/refpolicy

但是，如果你想安装一个用于测试的虚拟（dummy）策略，可以使用 scripts/selinux 下提供的 `mdp` 来完成。注意这需要安装 selinux 用户空间——特别是你需要 checkpolicy 来编译内核，以及 setfiles 和 fixfiles 来标记文件系统。

 1. 编译启用 selinux 的内核。
 2. 输入 `make` 编译 `mdp`。
 3. 确保你没有在启用 SELinux 且使用真实策略的情况下运行。如果是，请在继续之前以禁用 selinux 的方式重启。
```
		cd scripts/selinux
		sh install_policy.sh
```
第 4 步将创建一个对你的内核有效的新虚拟策略，其中只有一个 selinux 用户、角色和类型。它将编译该策略，将你的 `SELINUXTYPE` 在 `/etc/selinux/config` 中设为 `dummy`，安装编译后的策略作为 `dummy`，并重新标记你的文件系统。
