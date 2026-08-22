### 被污点的内核


当发生某些可能在日后排查问题时相关的事情时，内核会标记自己为“tainted”（被污点）。不必对此过于担心，大多数情况下运行一个被污点的内核并不是问题；这个信息主要在有人想调查某个问题时才有意义，因为其真正原因可能就是导致内核被污点的那个事件。这也是为什么来自被污点内核的缺陷报告经常被开发者忽略，因此请尽量用未被污点的内核来复现问题
注意，即使在你撤销了导致污点的原因（即卸载了专有内核模块）之后，内核仍会保持被污点状态，以表明该内核仍然不可信。这也是为什么当内核注意到一个内部问题（“kernel bug”）、一个可恢复错误（“kernel oops”）或一个不可恢复错误（“kernel panic”）时，会打印污点状态，并将有关此事的调试信息写`dmesg` 输出的日志中。也可以在运行时通过 `/proc/` 中的一个文件来检查污点状态
#### 缺陷、oops panic 消息中的污点标志


你可以在靠近顶部、以“CPU:”开头的那一行找到污点状态；内核为何或被污点的原因显示在进程 ID（“PID:”）之后，以及一个缩短的

```
	BUG: unable to handle kernel NULL pointer dereference at 0000000000000000
	Oops: 0002 [#1] SMP PTI
	CPU: 0 PID: 4424 Comm: insmod Tainted: P        W  O      4.20.0-0.rc6.fc30 #1
	Hardware name: Red Hat KVM, BIOS 0.5.1 01/01/2011
	RIP: 0010:my_oops_init+0x13/0x1000 [kpanic]
	[...]
```

如果该事件发生时内核未被污点，你会在那里找到“Not tainted: ”；如果曾被污点，则会打印“Tainted: ”以及字
```
	Tainted: P        W  O
```

这些字符的含义在下表中解释。在此例中，内核此前被污点，是因为加载了一个专有模块（`P`）、发生了一次警告（`W`），以及加载了一个外部构建的模块（`O`）。要解码其他字母，请使用下表
#### 在运行时解码污点状

在运行时，你可以通过读取 `cat /proc/sys/kernel/tainted` 来查询污点状态。如果返`0`，则内核未被污点；任何其他数字都表示它被污点的原因。解码该数字最简单的方法是使用脚`tools/debugging/kernel-chktaint`，你的发行版可能将其作为名为 `linux-tools` `kernel-tools` 的软件包的一部分提供；如果没有，你可以从 `git.kernel.org <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/tools/debugging/kernel-chktaint>`_ 下载该脚本，并用 `sh kernel-chktaint` 执行，它会打印出类似如下内容

```
	Kernel is Tainted for following reasons:
	 * Proprietary module was loaded (#0)
	 * Kernel issued warning (#9)
	 * Externally-built ('out-of-tree') module was loaded  (#12)
	See Documentation/admin-guide/tainted-kernels.rst in the Linux kernel or
	 https://www.kernel.org/doc/html/latest/admin-guide/tainted-kernels.html for
	 a more details explanation of the various taint flags.
	Raw taint value as int/string: 4609/'P        W  O     '
```

你也可以尝试自己解码该数字。如果只有单一原因导致内核被污点，那很简单，因为你可以对照下表找到该数字。如果有多个原因，你需要解码该数字，因为它是一个位域，其中每一位指示某类污点的缺失或存在。最好把它交给上述脚本处理，但如果你需要一个快速方法，可以使用这个 shell 命令来检
```
	$ for i in $(seq 20); do echo $(($i-1)) $(($(cat /proc/sys/kernel/tainted)>>($i-1)&1));done
```

#### 污点状态解码表


===  ===  ======  ========================================================
  日志  数字    导致内核被污点的原因
===  ===  ======  ========================================================
  0  G/P       1  加载了专有模  1  _/F       2  模块被强制加  2  _/S       4  内核运行在超出规格的系统  3  _/R       8  模块被强制卸  4  _/M      16  处理器报告了机器检测异常（MCE  5  _/B      32  引用了坏页或某些意外的页标志
  6  _/U      64  由用户空间应用程序请求的污点
  7  _/D     128  内核最近死亡，即发生了 OOPS BUG
  8  _/A     256  ACPI 表被用户覆盖
  9  _/W     512  内核发出了警 10  _/C    1024  加载了暂存区（staging）驱 11  _/I    2048  对平台固件中的缺陷应用了规避措施
 12  _/O    4096  加载了外部构建（“out-of-tree”）的模 13  _/E    8192  加载了未签名的模 14  _/L   16384  发生了软锁死（soft lockup 15  _/K   32768  内核已被实时补丁（live patched 16  _/X   65536  辅助污点，由发行版定义并使用
 17  _/T  131072  内核是使struct 随机化插件构建的
 18  _/N  262144  运行了一个内核内测试
 19  _/J  524288  用户空间fwctl 中使用了变异调试操作
===  ===  ======  ========================================================

注：字符 `_` 在此表中表示空白，以便于阅读
#### 关于污点的更详细说明


 0) 如果加载的所有模块都具有 GPL 或兼容许可证，则`G`；如果加载了任何专有模块，则`P`。没MODULE_LICENSE 或带insmod 不识别为 GPL 兼容MODULE_LICENSE 的模块，会被假定为专有模块
 1) 如果任何模块通过 `insmod -f` 被强制加载，则为 `F`；如果所有模块都正常加载，则`' '`
 2) 如果内核运行在超出规格的处理器或系统上，则为 `S`：硬件被置于不受支持的配置中，因此无法保证正确执行。例如，在下列情况下内核会被污点
     - x86 上：在英特尔 CPU（如 Pentium M）上通过 forcepae 强制启用 PAE，这CPU 不报PAE 但可能有可用的实现；SMP 内核运行在官方不支持 SMP Athlon CPU 上；从用户空间拨MSR     - arm 上：内核运行在某些缺少某些内核特性启用的 CPU（如 Keystone 2）上     - arm64 上：CPU 之间的硬件特性不匹配，引导加载程序以不同模式启动CPU     - 在某些不受支持的架构上使用了某些驱动（如 scsi/snic 用在x86_64 上、scsi/ips 用非 x86/x86_64/itanium 上、arm64 irqchip/irq-gic 的固件设置有误……）     - x86/x86_64：微码延迟加载是危险的，会导致内核被污点。它要求所CPU 会合（rendezvous）以确保更新在系统尽可能安静时进行。然而，更高优先级的 MCE/SMI/NMI 可能将控制流从会合点移开并中断更新，这可能对机器造成损害
 3) 如果模块通过 `rmmod -f` 被强制卸载，则为 `R`；如果所有模块都正常卸载，则`' '`
 4) 如果任何处理器报告了机器检测异常，则为 `M`；如果没有发生过机器检测异常，则为 `' '`
 5) 如果页释放函数发现了坏页引用或某些意外的页标志，则为 `B`。这表明存在硬件问题或内核缺陷；日志中应有其他信息说明为何发生此污点
 6) 如果用户或用户应用程序明确请求设置污点标志，则为 `U`；否则为 `' '`
 7) 如果内核最近死亡，即发生了 OOPS BUG，则`D`
 8) 如果 ACPI 表被覆盖，则`A`
 9) 如果内核此前发出过警告，则为 `W`。（不过某些警告可能会设置更具体的污点标志。）

 10) 如果加载了暂存区（staging）驱动，则为 `C`
 11) 如果内核正在规避平台固件（BIOS 或类似物）中的严重缺陷，则为 `I`
 12) 如果加载了外部构建（“out-of-tree”）的模块，则为 `O`
 13) 如果在支持模块签名的内核中加载了未签名的模块，则`E`
 14) 如果系统上此前发生过软锁死，则为 `L`
 15) 如果内核已被实时补丁，则`K`
 16) `X` 辅助污点，由 Linux 发行版定义并使用
 17) `T` 内核在构建时使用randstruct 插件，该插件会有意产生极其不寻常的内核结构体布局（甚至性能上病态的布局），这在调试时很重要。在构建时设置
 18) 如果运行了内核内测试，例KUnit 测试，则`N`
 19) 如果用户空间打开/dev/fwctl/* 并执行了 FWTCL_RPC_DEBUG_WRITE 以使用设备的调试功能，则`J`。设备调试功能可能以未定义的方式导致设备 malfunction（故障）