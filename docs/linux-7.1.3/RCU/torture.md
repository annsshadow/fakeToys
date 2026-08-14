
## RCU 压力测试操作


## CONFIG_RCU_TORTURE_TEST


所有 RCU 实现都提供了 `CONFIG_RCU_TORTURE_TEST` 配置选项。它会创建一个 `rcutorture` 内核模块，加载该模块即可运行一项压力测试。测试会周期性地通过 printk() 输出状态信息，可通过 dmesg 命令（或许配合 grep "torture"）查看。模块加载时测试开始，模块卸载时测试停止。

模块参数在 `Documentation/admin-guide/kernel-parameters.txt` 中以 "rcutorture." 为前缀。

## 输出


```
	rcu-torture:--- Start of test: nreaders=16 nfakewriters=4 stat_interval=30 verbose=0 test_no_idle_hz=1 shuffle_interval=3 stutter=5 irqreader=1 fqs_duration=0 fqs_holdoff=0 fqs_stutter=3 test_boost=1/0 test_boost_interval=7 test_boost_duration=4
	rcu-torture: rtc:           (null) ver: 155441 tfle: 0 rta: 155441 rtaf: 8884 rtf: 155440 rtmbe: 0 rtbe: 0 rtbke: 0 rtbre: 0 rtbf: 0 rtb: 0 nt: 3055767
	rcu-torture: Reader Pipe:  727860534 34213 0 0 0 0 0 0 0 0 0
	rcu-torture: Reader Batch:  727877838 17003 0 0 0 0 0 0 0 0 0
	rcu-torture: Free-Block Circulation:  155440 155440 155440 155440 155440 155440 155440 155440 155440 155440 0
	rcu-torture:--- End of test: SUCCESS: nreaders=16 nfakewriters=4 stat_interval=30 verbose=0 test_no_idle_hz=1 shuffle_interval=3 stutter=5 irqreader=1 fqs_duration=0 fqs_holdoff=0 fqs_stutter=3 test_boost=1/0 test_boost_interval=7 test_boost_duration=4
```

在大多数系统上，命令 "dmesg | grep torture:" 即可提取这些信息。在较为特殊的配置下，可能需要使用其他命令来访问 RCU 压力测试所用的 printk() 输出。这些 printk() 使用 KERN_ALERT，因此应当很明显。 ;-)

首行与末行显示了 rcutorture 模块参数，末行则根据 rcutorture 对 RCU 是否正确运行的自动判定，显示 "SUCCESS" 或 "FAILURE"。

各条目含义如下：

- "rtc"：当前对读者可见的结构体的十六进制地址。

- "ver"：自启动以来，RCU 写者任务更改读者可见结构的次数。

- "tfle"：若非零，表示用于放入 "rtc" 区域的 "torture freelist"（空闲链表）为空。此状况很重要，因为它可能让你误以为 RCU 在工作而实际并非如此。 :-/

- "rta"：从 torture 空闲链表分配的结构数量。

- "rtaf"：因链表为空而从 torture 空闲链表分配失败的次数。该值非零并不罕见，但若占到 "rta" 所指示值的很大比例则是不好的。

- "rtf"：释放回 torture 空闲链表的数量。

- "rtmbe"：非零值表示 rcutorture 认为 rcu_assign_pointer() 与 rcu_dereference() 工作不正常。该值应为零。

- "rtbe"：非零值表示 rcu_barrier() 系列函数之一工作不正常。

- "rtbke"：rcutorture 无法创建用于强制 RCU 优先级反转的实时 kthread。该值应为零。

- "rtbre"：尽管 rcutorture 成功创建了用于强制 RCU 优先级反转的 kthread，但无法将其设置为实时优先级 1。该值应为零。

- "rtbf"：RCU 优先级提升未能解决 RCU 优先级反转的次数。

- "rtb"：rcutorture 尝试强制 RCU 优先级反转条件的次数。若你正通过 "test_boost" 模块参数测试 RCU 优先级提升，该值应为非零。

- "nt"：rcutorture 在定时器处理程序中运行 RCU 读端代码的次数。仅当你指定了 "irqreader" 模块参数时，该值才应为非零。

- "Reader Pipe"：读者所见结构体 "age"（年龄）的直方图。若前两项之外的任何条目非零，则 RCU 已损坏。rcutorture 会打印错误标志字符串 "!!!" 以确保你注意到。新分配结构体的 age 为零，从读者可见性中移除时变为 1，之后每经过一个宽限期递增一次——并在经过 (RCU_TORTURE_PIPE_LEN-2) 个宽限期后被释放。

上面显示的输出取自正常工作的 RCU。若想看看损坏时的样子，自己弄坏它即可。 ;-)

- "Reader Batch"：读者所见结构体 "age" 的另一份直方图，但按计数器翻转（或批次）而非宽限期来统计。合法的非零条目数量同样为两个。之所以提供这一独立视图，是因为有时更容易让第三个条目出现在 "Reader Batch" 列表中，而非 "Reader Pipe" 列表。

- "Free-Block Circulation"：显示到达流水线中给定位置的 torture 结构数量。第一个元素应大致对应已分配的结构数量，第二个对应已从读者视图移除的数量，其余（除最后一个外）对应经过相应次数宽限期的数量。最后一个条目应为零，因为它仅在某个 torture 结构的计数器被错误地递增超过应有范围时才递增。

不同的 RCU 实现可以提供特定于实现的额外信息。例如，Tree SRCU 提供如下内容

```
	srcud-torture: Tree SRCU per-CPU(idx=0): 0(35,-21) 1(-4,24) 2(1,1) 3(-26,20) 4(28,-47) 5(-9,4) 6(-10,14) 7(-14,11) T(1,6)
```

该行显示了每 CPU 计数器状态，此处为使用动态分配的 srcu_struct 的 Tree SRCU（因此前缀为 "srcud-" 而非 "srcu-"）。括号中的数字是对应 CPU 的 "old" 与 "current" 计数器值。"idx" 值将 "old" 与 "current" 值映射到底层数组，对调试很有用。最后的 "T" 条目包含计数器的合计值。

## 在特定内核构建上的用法


有时需要在特定的内核构建上对 RCU 进行压力测试，例如准备将该内核构建投入生产环境时。此时，内核应以 CONFIG_RCU_TORTURE_TEST=m 构建，从而可以使用 modprobe 启动测试、使用 rmmod 终止测试。

```
	#!/bin/sh

	modprobe rcutorture
	sleep 3600
	rmmod rcutorture
	dmesg | grep torture:
```

输出可人工检查其中的 "!!!" 错误标志。当然，也可以编写更完善的脚本来自动检查此类错误。"rmmod" 命令会强制通过 printk() 打印 "SUCCESS"、"FAILURE" 或 "RCU_HOTPLUG" 指示。前两者不言自明，最后一个表示虽然没有 RCU 失败，但检测到了 CPU 热插拔问题。


## 在主線内核上的用法


当使用 rcutorture 测试对 RCU 自身的改动时，往往有必要构建多个内核，以在相关 Kconfig 选项与内核启动参数的大量组合下测试该改动。在这种情况下，使用 modprobe 与 rmmod 可能相当耗时且容易出错。

因此，提供了 `tools/testing/selftests/rcutorture/bin/kvm.sh` 脚本用于 x86、arm64 和 powerpc 的主线测试。默认情况下，它会运行 `tools/testing/selftests/rcutorture/configs/rcu/CFLIST` 所指定的一系列测试，每个测试在客户机操作系统中运行 30 分钟，使用自动生成的 initrd 提供的最小 userspace。测试完成后，会对生成的构建产物与控制台输出进行错误分析，并汇总运行结果。

在较大的系统上，可通过向 kvm.sh 传递 --cpus 参数来加速 rcutorture 测试。例如，在 64 CPU 的系统上，"--cpus 43" 会使用最多 43 个 CPU 并发运行测试，自 v5.4 起可在两批内完成全部场景，将完成时间从约八小时缩短到约一小时（不含构建十六个内核所需的时间）。"--dryrun sched" 参数不会运行测试，而是指示测试将如何调度分批。这在确定 --cpus 参数应指定多少个 CPU 时很有用。

并非所有改动都需要运行全部场景。例如，对 Tree SRCU 的改动可能只运行 SRCU-N 与 SRCU-P 场景，通过 kvm.sh 的 --configs 参数实现： "--configs 'SRCU-N SRCU-P'"。大型系统可以运行完整场景集的多份副本，例如，拥有 448 个硬件线程的系统可运行五份实例

```
	kvm.sh --cpus 448 --configs '5*CFLIST'
```

或者，此类系统可运行单个场景的 56 个并发实例

```
	kvm.sh --cpus 448 --configs '56*TREE04'
```

```
	kvm.sh --cpus 448 --configs '28*TREE03 28*TREE04'
```

当然，每个并发实例都会占用内存，可通过 --memory 参数限制，其默认为 512M。较小的内存值可能需要使用下文讨论的 --bootargs 参数禁用回调洪泛测试。

有时额外的调试很有用，此时可使用 kvm.sh 的 --kconfig 参数，例如 `--kconfig 'CONFIG_RCU_EQS_DEBUG=y'`。此外还有 --gdb、--kasan 和 --kcsan 参数。注意 --gdb 会将每次 kvm.sh 运行限制为单个场景，并要求你打开另一个窗口，按照脚本指示从中运行 `gdb`。

也可以提供内核启动参数，例如用于控制 rcutorture 的模块参数。例如，要测试对 RCU CPU stall 警告代码的改动，可使用 "--bootargs 'rcutorture.stall_cpu=30'"。这当然会导致脚本报告失败，即所产生的 RCU CPU stall 警告。如上所述，减少内存可能

```
	kvm.sh --cpus 448 --configs '56*TREE04' --memory 128M \
		--bootargs 'rcutorture.fwd_progress=0'
```

有时所需的只是完整的一组内核构建。这正是 --buildonly 参数的作用。

--duration 参数可覆盖默认的 30 分钟运行时间。例如，`--duration 2d` 运行两天，`--duration 3h` 运行三小时，`--duration 5m` 运行五分钟，`--duration 45s` 运行 45 秒。最后这一项对追踪罕见的启动期失败很有用。

最后，--trust-make 参数允许每个内核构建复用上一个内核构建中可复用的内容。请注意，若不使用 --trust-make 参数，你的 tags 文件可能会被清除。

kvm.sh 脚本的源代码中还记录了其他更为隐秘的参数。

如果某次运行包含失败，构建期与运行期失败的数量会列在 kvm.sh 输出的末尾，你确实应当将其重定向到文件。每次运行的构建产物与控制台输出保存在 `tools/testing/selftests/rcutorture/res` 中的带时间戳目录里。可将某个目录提供给 kvm-find-errors.sh 以

```
	tools/testing/selftests/rcutorture/bin/kvm-find-errors.sh \
		tools/testing/selftests/rcutorture/res/2020.01.20-15.54.23
```

不过，直接访问这些文件通常更方便。与某次运行中所有场景相关的文件位于顶层目录（上例中的 2020.01.20-15.54.23），而与单个场景相关的文件位于以该场景命名的子目录中（例如 "TREE04"）。若某个场景运行了多次（如上例中的 "--configs '56*TREE04'"），对应第二次及后续运行的目录会包含序号，例如 "TREE04.2"、"TREE04.3" 等。

顶层目录中最常用的文件是 testid.txt。如果测试运行于 git 仓库中，则该文件包含被测试的 commit 以及任何以 diff 格式存在的未提交改动。

每个单场景运行目录中最常用的文件有：

.config:
	该文件包含 Kconfig 选项。

Make.out:
	该文件包含特定场景的构建输出。

console.log:
	该文件包含特定场景的控制台输出。内核启动后可供查看，但如果构建失败它可能不存在。

vmlinux:
	该文件包含内核，可与 objdump 和 gdb 等工具配合使用。

还有其他若干文件可用，但使用频率较低。许多专为调试 rcutorture 自身或其脚本而设。

自 v5.4 起，使用默认场景集的成功运行会产生

```
    SRCU-N ------- 804233 GPs (148.932/s) [srcu: g10008272 f0x0 ]
    SRCU-P ------- 202320 GPs (37.4667/s) [srcud: g1809476 f0x0 ]
    SRCU-t ------- 1122086 GPs (207.794/s) [srcu: g0 f0x0 ]
    SRCU-u ------- 1111285 GPs (205.794/s) [srcud: g1 f0x0 ]
    TASKS01 ------- 19666 GPs (3.64185/s) [tasks: g0 f0x0 ]
    TASKS02 ------- 20541 GPs (3.80389/s) [tasks: g0 f0x0 ]
    TASKS03 ------- 19416 GPs (3.59556/s) [tasks: g0 f0x0 ]
    TINY01 ------- 836134 GPs (154.84/s) [rcu: g0 f0x0 ] n_max_cbs: 34198
    TINY02 ------- 850371 GPs (157.476/s) [rcu: g0 f0x0 ] n_max_cbs: 2631
    TREE01 ------- 162625 GPs (30.1157/s) [rcu: g1124169 f0x0 ]
    TREE02 ------- 333003 GPs (61.6672/s) [rcu: g2647753 f0x0 ] n_max_cbs: 35844
    TREE03 ------- 306623 GPs (56.782/s) [rcu: g2975325 f0x0 ] n_max_cbs: 1496497
    CPU count limited from 16 to 12
    TREE04 ------- 246149 GPs (45.5831/s) [rcu: g1695737 f0x0 ] n_max_cbs: 434961
    TREE05 ------- 314603 GPs (58.2598/s) [rcu: g2257741 f0x2 ] n_max_cbs: 193997
    TREE07 ------- 167347 GPs (30.9902/s) [rcu: g1079021 f0x0 ] n_max_cbs: 478732
    CPU count limited from 16 to 12
    TREE09 ------- 752238 GPs (139.303/s) [rcu: g13075057 f0x0 ] n_max_cbs: 99011
```

## 重复运行


假设你正在追踪一个罕见的启动期失败。尽管可以使用 kvm.sh，但这样每次运行都会重新构建内核。如果你需要（比如）运行 1,000 次以确信已修复该 bug，这些无意义的重建会变得极其烦人。

这正是 kvm-again.sh 存在的原因。

```
	tools/testing/selftests/rcutorture/res/2022.11.03-11.26.28
```

```
	kvm-again.sh tools/testing/selftests/rcutorture/res/2022.11.03-11.26.28
```

可以覆盖原始运行的部分 kvm.sh 参数，例如

```
	kvm-again.sh tools/testing/selftests/rcutorture/res/2022.11.03-11.26.28 \
		--duration 45s
```

将重新运行之前的测试，但只运行 45 秒，从而便于追踪前述罕见的启动期失败。


## 分布式运行


尽管 kvm.sh 相当有用，但其测试局限于单一系统。使用你喜欢的框架让（比如）5 个 kvm.sh 实例在你的 5 个系统上运行并不算难，但这极有可能会不必要地重建内核。此外，手动将所需的 rcutorture 场景分布到可用系统上既费力又容易出错。

这正是 kvm-remote.sh 脚本存在的原因。

```
	ssh system0 date
```

如果它对 system1、system2、system3、system4 和 system5 也有效，

```
	kvm-remote.sh "system0 system1 system2 system3 system4 system5" \
		--cpus 64 --duration 8h --configs "5*CFLIST"
```

这将在本地系统上构建每个默认场景的内核，然后将每个场景的五个实例分布到所列系统上，每个场景运行八小时。运行结束时，结果会被收集、记录并打印。kvm.sh 可接受的大部分参数都可传递给 kvm-remote.sh，但系统列表必须放在最前。

kvm.sh 的 `--dryrun scenarios` 参数有助于确定在一组系统上一批可运行多少个场景。

```
	kvm-remote.sh "system0 system1 system2 system3 system4 system5" \
		tools/testing/selftests/rcutorture/res/2022.11.03-11.26.28-remote \
		--duration 24h
```

在这种情况下，大多数 kvm-again.sh 参数可在旧运行结果目录的路径名之后提供。
