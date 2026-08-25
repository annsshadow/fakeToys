## 调试休眠（hibernation）与挂起（suspend
	(C) 2007 Rafael J. Wysocki <rjw@sisk.pl>, GPL

## 1. 测试休眠（hibernation，即 suspend to disk STD
```

	# echo reboot > /sys/power/disk
	# echo disk > /sys/power/state

```
系统应当创建一个休眠映像，重启，恢复，并回到你启动该切换时所在的命令提示符。如果发生了这种情况休眠很可能工作正常。不过，你仍需要连续重复该测试至少几次以获得信心。[这是必要的，因为某些问题
只在第二次尝试挂起和恢复系统时才会显现。] 此外，以 "reboot" "shutdown" 模式休眠会导PM 核心
跳过一些与平台相关的回调，而在 ACPI 系统上这些回调可能是让休眠正常工作所必需的。因此，如果你的
机器"reboot" 模式下无法休眠或恢复，你应当尝试```

	# echo platform > /sys/power/disk
	# echo disk > /sys/power/state

```
这是默认且推荐的休眠模式
遗憾的是platform" 休眠模式在一BIOS 损坏的系统上无法工作。这种情况下，休眠的 "shutdown"
模式可能可以```

	# echo shutdown > /sys/power/disk
	# echo disk > /sys/power/state

```
（它"reboot" 模式类似，但需要你按下电源按钮来让系统恢复。）

如果 "platform" "shutdown" 休眠模式都无法工作，你将需要找出哪里出了问题
### a) 测试休眠模式（Test modes of hibernation
要找出你的系统上休眠为何失败，可以使用一个特殊的测试设施，前提是内核CONFIG_PM_DEBUG 编译这时会有一个文/sys/power/pm_test，可用来让休眠核心以测试模式运行。共5 种可用的测试模式
freezer
 - 测试进程的冻
devices
 - 测试进程的冻结与设备的挂
platform
 - 测试进程的冻结、设备的挂起以及平台全局控制方法 [^1^]_

processors
 - 测试进程的冻结、设备的挂起、平台全局控制方法 [^1^]_ 以及非启CPU 的禁
core
 - 测试进程的冻结、设备的挂起、平台全局控制方法\ [^1^]_、非启动 CPU 的禁用以及平系统设备	  挂起

    platform 全局控制方法只在 ACPI 系统上可用，并且只有在休眠模式设"platform" 时才会被测试

要使用其中某一种，需要把相应的字符串写入 /sys/power/pm_test（例"devices" 用于测试进程冻结与设备的挂起），并下达标准的休眠命令。例如，要配合使"devices" 测试模式与休眠的 "platform"
模式```

	# echo devices > /sys/power/pm_test
	# echo platform > /sys/power/disk
	# echo disk > /sys/power/state

```
然后，内核会尝试冻结进程、挂起设备、等待几秒（默认 5 秒，但可通过 suspend.pm_test_delay 模块
参数配置）、恢复设备并解冻进程。如果把 "platform" 写入 /sys/power/pm_test，那么在挂起设备之后内核还会额外调用用于为平台固件准备休眠的全局控制方法（例ACPI 全局控制方法）。接下来，它会等一个可配置的秒数，并调用用于取消休眠等的平台（例如 ACPI）全局方法
/sys/power/pm_test 写入 "none" 会让内核切换到正常的休眠/挂起操作。此外，当以读方式打开时，
/sys/power/pm_test 包含一个以空格分隔的所有可用测试（包括代表正常功能"none"）的列表，其当前测试级别用方括号标示
一般来说，正如你所看到的，每个测试级别都比前一个更"侵入"，"core" 级别在不开休眠映像的前提下
尽可能深入地测试硬件与驱动。显然，如果 "devices" 测试失败platform" 测试也会失败，依此类推。因此，
作为经验法则，你应当"freezer" 开始，依次经过 "devices"platform" "processors" 一直测"core"（在每个级别上重复测试几次，以避免任何随机因素的影响）
如果 "freezer" 测试失败，说明存在某个无法被冻结的任务（在这种情况下，通常可以通过分析失败测试获取dmesg 输出来识别该有问题的任务）。这个级别的失败通常意味着任务冻结器（tasks freezer子系统存在问题，应当上报
如果 "devices" 测试失败，很可能是某个驱动无法挂起或恢复其设备（在后一种情况下，系统可能会在测后挂起或变得不稳定，请务必考虑到这一点）。要找出这个驱动，可以按照以下规则进行二分查找：

- 如果测试失败，卸载当前已加载驱动的一半并重试（这可能涉及重启系统，因此请始终记录测试前加载了
  哪些驱动），
- 如果测试成功，加载你最近卸载的驱动的一半并重试
一旦你找到了有问题的驱动（可能不止一个），你必须在每次休眠前都卸载它。在这种情况下，请务必上该驱动的问题
也有可能是，在你卸载了所有模块之"devices" 测试仍然失败。这种情况下，你可以检查内核配置中那些
可以编译为模块的驱动（并用这些驱动编译为模块的方式再次测试）。你也可以尝试使用一些特殊的命令选项，例"noapic"noacpi" 甚至 "acpi=off"
如果 "platform" 测试失败，说明你的系统在处理平台（例ACPI）固件时存在问题。这种情况下platform"
休眠模式不太可能正常工作。你可以尝试 "shutdown" 模式，但那只是一种相当简陋的权宜之计
如果 "processors" 测试失败，说明非启动 CPU 的禁启用不起作用（当然，这只SMP 系统上才可能
成为问题），并且该问题应当上报。在这种情况下，你也可以尝试使用 /sys/devices/system/cpu/cpu*/online
sysfs 属性来开关非启动 CPU，看看是否有效
如果 "core" 测试失败，意味着系统/平台设备的挂起失败了（这些设备是在一CPU 上、关闭中断的情况挂起的），问题很可能出在硬件上且比较严重，因此应当上报
"platform"processors" "core" 中任何一项测试的失败都可能导致你的系统挂起或变得不稳定，务必小心。这样的失败通常表示很可能由硬件引起的严重问题，但无论如何请上报它
### b) 测试最小配置（Testing minimal configuration
如果所有休眠测试模式都能工作，你可以用 "init=/bin/bash" 命令行参数启动系统，并尝试以 "reboot""shutdown" "platform" 模式休眠。如果那样不行，很可能是静态编译进内核的驱动存在问题，你可尝试把更多驱动编译为模块，以便对它们逐个测试。否则，问题出在某个模块化驱动上，你可以通过加载通常所用模块的一半并按如下算法进行二分查找来找到它：
- 如果加载n 个模块且挂起/恢复尝试失败，卸n/2 个模块并重试（这可能涉及重启系统），
- 如果加载n 个模块且挂起/恢复尝试成功，再加载 n/2 个模块并重试
同样，如果你找到了有问题的模块，每次休眠前都必须卸载它（们），并请上报它（们）的问题
### c) 使用 "test_resume" 休眠选项（Using the "test_resume" hibernation option
/sys/power/disk 通常告诉内核在创建休眠映像之后该做什么。其中一个可用选项"test_resume"，它让刚刚创建的映像被用于立即恢复。即```

	# echo test_resume > /sys/power/disk
	# echo disk > /sys/power/state

```
一个休眠映像会被创建，并立即触发从其恢复，而完全不涉及平台固件
该测试可用于检查恢复失败是否与平台固件的不良交互有关。也就是说，如果上述操作每次都能工作，但实际休眠中恢复却不行或不可靠，那么平台固件可能要为此负责
在那些支持使用不同内核来恢复休眠映像（即用于从存储中读取映像并将其载入内存的内核，与映像中包含的
内核不同），或支持内核地址空间随机化的架构与平台上，它也可用于检查恢复失败是否与恢复内核和映内核之间的差异有关
### d) 高级调试（Advanced debugging
如果你的系统即使在最小配置下休眠也无法工作，且把更多驱动编译为模块不切实际，或者某些模块无法卸载，
你可以使用一种更高级的调试技术来查找问题。首先，如果你的机器上有串口，你可以'no_console_suspend'
参数启动内核，并尝试使用串口控制台记录内核消息。这可能会为你提供关于挂起（恢复）失败原因的一信息。或者，也可以尝试使FireWire 端口配合 firescope（http://v3.sk/~lkundrak/firescope/）进调试。在 x86 上，还可以使Documentation/power/s2ram.rst 中记录的 PM_TRACE 机制
## 2. 测试挂起RAM（STR，Suspend to RAM
要验STR 能否工作，通常使用http://suspend.sf.net 获取并在 http://en.opensuse.org/SDB:Suspend_to_RAM
（S2RAM_LINK）有文档s2ram 工具会更方便
也就是说，在"freezer"devices"platform"processors" "core" 写入 /sys/power/pm_test
（若内核CONFIG_PM_DEBUG 编译则可用）之后，挂起代码会以与给定字符串对应的测试模式工作。STR 测试
模式的定义与休眠相同，因此有关它们的更多信息请参阅第 1 节。特别指出的是，"core" 测试允许你测试除
实际调用平台固件来让系统进入睡眠状态之外的所有内容
除其它之外，借助 /sys/power/pm_test 进行的测试可以让你识别出无法挂起或恢复其设备的驱动。它们应在每STR 切换前被卸载
接下来，你可以按S2RAM_LINK 上的说明来测试系统，但如果它不能"开箱即，你可能需要用 "init=/bin/bash"
启动，并在最小配置下测试 s2ram。在这种情况下，你可以按照与1 节所述类似的流程来搜索有问题的驱动如果你找到了一些有问题的驱动，你必须在每次 STR 切换（即运行 s2ram 之前）前卸载它们，并请上报它们的
问题
有一debugfs 条目显示挂起RAM 的统计信息。示例如下：
```

	# mount -t debugfs none /sys/kernel/debug
	# cat /sys/kernel/debug/suspend_stats
	success: 20
	fail: 5
	failed_freeze: 0
	failed_prepare: 0
	failed_suspend: 5
	failed_suspend_noirq: 0
	failed_resume: 0
	failed_resume_noirq: 0
	failures:
	  last_failed_dev:	alarm
				adc
	  last_failed_errno:	-16
				-16
	  last_failed_step:	suspend
				suspend

```
字段 success 表示挂起RAM 的成功次数，字段 fail 表示失败次数。其它字段是挂起RAM 各步骤的失败
次数。suspend_stats 仅列出最2 个失败的设备和错误号以及失败的步骤