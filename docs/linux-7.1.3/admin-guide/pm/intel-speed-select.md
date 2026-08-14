
## Intel(R) Speed Select Technology 用户指南


Intel(R) Speed Select Technology（Intel(R) SST）提供了一组强大的新特性，
可对 CPU 性能进行更精细的控制。借助 Intel(R) SST，一台服务器可以根据各种各样不同的
工作负载需求，为功耗与性能进行配置。

请参阅以下链接以概览该技术：

- https://www.intel.com/content/www/us/en/architecture-and-technology/speed-select-technology-article.html
- https://builders.intel.com/docs/networkbuilders/intel-speed-select-technology-base-frequency-enhancing-performance.pdf

这些能力在一些较新一代的服务器平台中得到了进一步增强，在这些平台上，无需通过 BIOS 设置选项预先配置，
即可动态地枚举并控制这些特性。这种动态配置是通过向硬件发送邮箱命令完成的。
枚举并配置这些特性的一种方式是使用 Intel Speed Select 工具。

本文档解释如何使用 Intel Speed Select 工具来枚举并控制 Intel(R) SST 特性。
本文档给出示例命令，并解释这些命令如何改变被测系统下的功耗与性能profile。
以这个工具为例，客户可以在他们的生产软件中复现该工具中所实现的消息交互。

## intel-speed-select 配置工具


大多数 Linux 发行版软件包可能包含 "intel-speed-select" 工具。如果没有，
可以通过从 kernel.org 下载 Linux 内核树来构建它。下载之后，无需构建完整内核即可构建该工具。

```

```
# cd tools/power/x86/intel-speed-select/
# make
# make install

### 获取帮助


```

```
# intel-speed-select --help

top-level 帮助描述了参数与特性。注意还有一个
```

```
# intel-speed-select perf-profile --help

```
# intel-speed-select perf-profile info --help

### 平台能力摘要

```

```
#intel-speed-select --info
```

```
 # intel-speed-select --info
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 Platform: API version : 1
 Platform: Driver version : 1
 Platform: mbox supported : 1
 Platform: mmio supported : 1
 Intel(R) SST-PP (feature perf-profile) is supported
 TDP level change control is unlocked, max level: 4
 Intel(R) SST-TF (feature turbo-freq) is supported
 Intel(R) SST-BF (feature base-freq) is not supported
 Intel(R) SST-CP (feature core-power) is supported

```
### Intel(R) Speed Select Technology - Performance Profile（Intel(R) SST-PP）


这个特性允许基于工作负载性能需求动态地配置一台服务器。这在部署时帮助用户，
因为他们不必静态地选择某个特定的服务器配置。这个 Intel(R) Speed Select Technology -
Performance Profile（Intel(R) SST-PP）特性引入了一种机制，允许每个系统有多个优化过的性能profile。
每个 profile 定义了一组需要在线、其余离线的 CPU，以维持一个有保证的基准频率。
一旦用户发出命令以使用某个特定的性能profile，并满足 CPU 在线/离线的要求，用户就可以预期基准频率会动态地改变。
在使用 Intel Speed Select 工具时，这个特性被称为 "perf-profile"。

#### Number or performance levels


一个系统上可以有多个性能profile。要获取性能
```

 # intel-speed-select perf-profile get-config-levels
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
        get-config-levels:4
 package-1
  die-0
    cpu-14
        get-config-levels:4

```
在这个被测系统上，除了基础性能profile（即性能级别 0）之外，还有 4 个性能profile。

#### 锁定/解锁状态


即便有多个性能profile，它们也有可能是被锁定的。如果它们被锁定，用户就无法发出命令来改变性能状态。
有可能存在某个 BIOS 设置可以解锁，或者咨询你的系统供应商。

```

 # intel-speed-select perf-profile get-lock-status
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
        get-lock-status:0
 package-1
  die-0
    cpu-14
        get-lock-status:0

```
在这种情况下，锁定状态为 0，意味着系统处于解锁状态。

#### 性能级别的属性


```

 # intel-speed-select perf-profile info -l 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      perf-profile-level-0
        cpu-count:28
        enable-cpu-mask:000003ff,f0003fff
        enable-cpu-list:0,1,2,3,4,5,6,7,8,9,10,11,12,13,28,29,30,31,32,33,34,35,36,37,38,39,40,41
        thermal-design-power-ratio:26
        base-frequency(MHz):2600
        speed-select-turbo-freq:disabled
        speed-select-base-freq:disabled
	...
	...

```
这里 -l 选项用于指定一个性能级别。

如果省略 -l 选项，那么这个命令将打印所有性能级别的信息。上面的命令打印的是性能级别 0 的属性。

对于这个性能profile，由 "enable-cpu-mask/enable-cpu-list" 显示的最大 CPU 列表可以 "online"。
当满足这个条件时，就可以维持 2600 MHz 的基准频率。想了解更多，请执行
"intel-speed-select perf-profile info" 以查看性能
```

 # intel-speed-select perf-profile info -l 4
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      perf-profile-level-4
        cpu-count:28
        enable-cpu-mask:000000fa,f0000faf
        enable-cpu-list:0,1,2,3,5,7,8,9,10,11,28,29,30,31,33,35,36,37,38,39
        thermal-design-power-ratio:28
        base-frequency(MHz):2800
        speed-select-turbo-freq:disabled
        speed-select-base-freq:unsupported
	...
	...

```
"enable-cpu-mask/enable-cpu-list" 中的 CPU 更少。因此，如果用户只将这些 CPU 保持在线，
而将其余的 "offline"，那么基准频率就会从性能级别 0 时的 2.6 GHz 提升到 2.8 GHz。

#### 获取当前性能级别


```

 # intel-speed-select perf-profile get-config-current-level
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
        get-config-current_level:0

```

```

 # cat /sys/devices/system/cpu/cpu0/cpufreq/base_frequency
 2600000

```
这与从 "perf-profile info" 命令为性能级别 0 显示的 base-frequency (MHz) 字段值相匹配
（cpufreq 频率的单位是 KHz）。

要检查平均频率是否等于 100% 繁忙时的基准频率，可以
```

```
# echo 1 > /sys/devices/system/cpu/intel_pstate/no_turbo
```

```
#stress -c 64
```

 #turbostat -c 0-13 --show Package,Core,CPU,Bzy_MHz -i 1

  Package	Core	CPU	Bzy_MHz
		-	-	2600
  0		0	0	2600
  0		1	1	2600
  0		2	2	2600
  0		3	3	2600
  0		4	4	2600
  .		.	.	.


```
#### 改变性能级别


```

 # intel-speed-select -d perf-profile set-config-level -l 4 -o
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      perf-profile
        set_tdp_level:success

```
在上面的命令中，"-o" 是可选的。如果指定了它，那么它还会让不在这个性能级别的
enable_cpu_mask 中的 CPU 离线。

```

 #cat /sys/devices/system/cpu/cpu0/cpufreq/base_frequency
 2800000

```
这表明基准频率现在从性能级别 0 时的 2600 MHz 提升到了性能级别 4 时的 2800 MHz。
结果就是，任何能够使用更少 CPU 的工作负载，相比性能级别 0 都可以看到 200 MHz 的提升。

#### 通过 BMC 接口改变性能级别


可以使用带外（OOB）代理（通过某些远程管理控制台，经由 BMC "Baseboard Management Controller"
基板管理控制器接口）来改变 SST-PP 级别。这种模式从 Sapphire Rapids 处理器代开始支持。
支持这种模式的
内核与工具改动被加入到了 Linux 内核 5.18 版本。要启用这个特性，需要内核配置
"CONFIG_INTEL_HFI_THERMAL"。支持这个特性的工具的最低版本是 "v1.12"，它是 Linux 内核 5.18 版本的一部分。

为了支持这样的配置，这个工具可以作为守护进程使用。添加
```

 # intel-speed-select --oob
 Intel(R) Speed Select Technology
 Executing on CPU model:143[0x8f]
 OOB mode is enabled and will run as daemon

```
在这种模式下，该工具将根据新的性能级别来使 CPU 在线/离线。

### 检查其他 Intel(R) SST 特性的存在


每个性能profile 也指明了是否支持另外两个 Intel(R) SST 特性（Intel(R) Speed Select Technology -
Base Frequency（Intel(R) SST-BF）与 Intel(R) Speed Select Technology - Turbo Frequency（Intel(R) SST-TF））。

例如，从上面的 "perf-profile info" 输出中，对于级别 0 与级别 4：

```

       speed-select-turbo-freq:disabled
       speed-select-base-freq:disabled

```

```
       speed-select-turbo-freq:disabled
       speed-select-base-freq:unsupported

```
鉴于这些结果，相比性能级别 0，在级别 4 中 "speed-select-base-freq"（Intel(R) SST-BF）从 "disabled" 变成了 "unsupported"。

这意味着，在性能级别 4 时，"speed-select-base-freq" 特性不被支持。然而，在性能级别 0 时，这个特性是 "supported"，
但当前 "disabled"，意味着用户尚未激活这个特性。而 "speed-select-turbo-freq"（Intel(R) SST-TF）在两个性能级别都受支持，
但当前未被用户激活。

Intel(R) SST-BF 与 Intel(R) SST-TF 特性构建在一个被称为 Intel(R) Speed Select Technology -
Core Power（Intel(R) SST-CP）的基础技术之上。当平台上支持 Intel(R) SST-BF 或 Intel(R) SST-TF 时，
平台固件会启用这个特性。

### Intel(R) Speed Select Technology Core Power（Intel(R) SST-CP）


Intel(R) Speed Select Technology Core Power（Intel(R) SST-CP）是一个允许用户定义每核优先级的接口。
这定义了一种在存在功耗受限场景时在核之间分配功耗的机制。这定义了一种服务等级（CLOS）配置。

用户可以配置多达 4 个服务等级配置。每个 CLOS 组配置允许定义一些参数，这些参数影响频率如何被限制以及功耗如何被分配。
每个 CPU 核都可以绑定到一个服务等级，从而关联到相应的优先级。粒度是核级别，而非每个 CPU 级别。

#### 启用基于 CLOS 的优先级排序


要使用基于 CLOS 的优先级排序特性，必须告知固件启用并使用某种优先级类型。每个平台有一个默认的优先级类型，
它可以通过可选的命令行参数改变。

```

 # intel-speed-select core-power enable --help
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 Enable core-power for a package/die
	Clos Enable: Specify priority type with [--priority|-p]
		 0: Proportional, 1: Ordered

```
有两种优先级类型：

- Ordered

Ordered 节流（throttling）的优先级是根据所分配的 CLOS 组的索引定义的。其中 CLOS0 获得最高优先级（最后被节流）。

优先级顺序是：
CLOS0 > CLOS1 > CLOS2 > CLOS3。

- Proportional

当使用比例（proportional）优先级时，有一个额外的参数叫做 frequency_weight，它可以针对每个 CLOS 组指定。
比例优先级的目标是首先为每个核提供所请求的最小值，然后按照定义的权重成比例地分配所有剩余（盈余/亏空）的预算。
这个比例优先级可以使用 "core-power config" 命令来配置。

```

 # intel-speed-select core-power enable
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      core-power
        enable:success
 package-1
  die-0
    cpu-6
      core-power
        enable:success

```
这个启用的范围在每个 package 包含多个 die 时是 per package 或 die 范围的。
要检查 CLOS 是否启用并获取优先级类型，可以使用 "core-power info" 命令。
例如，要检查 core-power 特性的状态，
```

 # intel-speed-select -c 0 core-power info
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      core-power
        support-status:supported
        enable-status:enabled
        clos-enable-status:enabled
        priority-type:proportional
 package-1
  die-0
    cpu-24
      core-power
        support-status:supported
        enable-status:enabled
        clos-enable-status:enabled
        priority-type:proportional

```
#### 配置 CLOS 组


每个 CLOS 组都有自己的属性，包括 min、max、freq_weight 与 desired。
这些参数可以用 "core-power config" 命令来配置。如果用户跳过了设置某个参数（除了 clos id 之外），
将使用默认值，clos id 是
```

 # intel-speed-select core-power config --help
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 Set core-power configuration for one of the four clos ids
	Specify targeted clos id with [--clos|-c]
	Specify clos Proportional Priority [--weight|-w]
	Specify clos min in MHz with [--min|-n]
	Specify clos max in MHz with [--max|-m]

```

```

 # intel-speed-select core-power config -c 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 clos epp is not specified, default: 0
 clos frequency weight is not specified, default: 0
 clos min is not specified, default: 0 MHz
 clos max is not specified, default: 25500 MHz
 clos desired is not specified, default: 0
 package-0
  die-0
    cpu-0
      core-power
        config:success
 package-1
  die-0
    cpu-6
      core-power
        config:success

```
用户可以选择改变默认值。例如，用户可以改变 "min" 并将基准频率设为总能获得有保证的基准频率。

#### 获取当前 CLOS 配置


要检查当前配置，可以使用 "core-power get-config"。对于
```

 # intel-speed-select core-power get-config -c 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      core-power
        clos:0
        epp:0
        clos-proportional-priority:0
        clos-min:0 MHz
        clos-max:Max Turbo frequency
        clos-desired:0 MHz
 package-1
  die-0
    cpu-24
      core-power
        clos:0
        epp:0
        clos-proportional-priority:0
        clos-min:0 MHz
        clos-max:Max Turbo frequency
        clos-desired:0 MHz

```
#### 将一个 CPU 与一个 CLOS 组关联


```

 # intel-speed-select core-power assoc --help
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 Associate a clos id to a CPU
	Specify targeted clos id with [--clos|-c]


```

 # intel-speed-select -c 10 core-power assoc -c 3
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-10
      core-power
        assoc:success

```
一旦一个 CPU 被关联，它的兄弟 CPU 也会被关联到一个 CLOS 组。一旦关联，要避免改变 Linux "cpufreq"
子系统的缩放频率限制。

要检查一个 CPU 已有的关联，可以使用 "core-power get-assoc" 命令，
```

 # intel-speed-select -c 10 core-power get-assoc
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-1
  die-0
    cpu-10
      get-assoc
        clos:3

```
这表明 CPU 10 是 CLOS 组 3 的一部分。


#### 禁用基于 CLOS 的优先级排序


```

```
# intel-speed-select core-power disable

像 Intel(R) SST-TF 这样的一些特性只有在启用了基于 CLOS 的优先级排序时才能被启用。
出于这个原因，在 Intel(R) SST-TF 已启用时禁用它可能导致 Intel(R) SST-TF 失败。
如果 Intel(R) SST-TF 已经启用，这将导致 "disable" 命令显示错误。
相应地，要禁用它，必须先禁用 Intel(R) SST-TF 特性。

### Intel(R) Speed Select Technology - Base Frequency（Intel(R) SST-BF）


Intel(R) Speed Select Technology - Base Frequency（Intel(R) SST-BF）特性让用户能够控制基准频率。
如果某些关键的工作负载线程要求恒定的高保证性能，那么这个特性可以用来在特定的 CPU 集合（高优先级 CPU）上以更高的基准频率
执行该线程，代价是其他 CPU 上较低的基准频率（低优先级 CPU）。这个特性不需要低优先级 CPU 离线。

Intel(R) SST-BF 的支持依赖于 Intel(R) Speed Select Technology -
Performance Profile（Intel(R) SST-PP）性能级别配置。有可能只有某些性能级别支持 Intel(R) SST-BF。
也有可能只有基础性能级别（level = 0）支持 Intel(R) SST-BF。
因此，首先选择想要的性能级别来启用这个特性。

在这个被测系统中，Intel(R) SST-BF 在基础
```

 # intel-speed-select -c 0 perf-profile info -l 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      perf-profile-level-0
        ...

        speed-select-base-freq:disabled
	...

```
在启用 Intel(R) SST-BF 并测量其对工作负载性能的影响之前，先执行一些工作负载并测量性能，得到一个用于比较的基线性能。

这里用户想要更多有保证的性能。出于这个原因，很可能
```

```
#echo 1 > /sys/devices/system/cpu/intel_pstate/no_turbo
```
基于 "intel-speed-select perf-profile info -l 0" 的输出，有保证频率的基准频率为 2600 MHz。


#### 测量基线性能以进行比较


为了比较，挑选一个多线程工作负载，其中每个线程可以被调度到不同的 CPU 上。
"Hackbench pipe" 测试是如何使用 Intel(R) SST-BF 提升性能的一个好例子。

下面，这个工作负载测量的是平均调度器唤醒延迟，因此一个更低
```

 # taskset -c 3,4 perf bench -r 100 sched pipe
 # Running 'sched/pipe' benchmark:
 # Executed 1000000 pipe operations between two processes
     Total time: 6.102 [sec]
       6.102445 usecs/op
         163868 ops/sec

```
在运行上面的测试时，如果我们取 turbostat 的输出，它将向我们显示有 2 个 CPU 很繁忙并达到了最大频率
（即基准
```

 #turbostat -c 0-13 --show Package,Core,CPU,Bzy_MHz -i 1
 Package	Core	CPU	Bzy_MHz
 0		0	0	1000
 0		1	1	1005
 0		2	2	1000
 0		3	3	2600
 0		4	4	2600
 0		5	5	1000
 0		6	6	1000
 0		7	7	1005
 0		8	8	1005
 0		9	9	1000
 0		10	10	1000
 0		11	11	995
 0		12	12	1000
 0		13	13	1000

```
从上面的 turbostat 输出可以看出，CPU 3 与 4 都非常繁忙，并达到了 2600 MHz 的完整有保证频率。

#### Intel(R) SST-BF 能力


要获取当前性能级别 0 下 Intel(R) SST-BF 的能力，
```

 # intel-speed-select base-freq info -l 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      speed-select-base-freq
        high-priority-base-frequency(MHz):3000
        high-priority-cpu-mask:00000216,00002160
        high-priority-cpu-list:5,6,8,13,33,34,36,41
        low-priority-base-frequency(MHz):2400
        tjunction-temperature(C):125
        thermal-design-power(W):205

```
上述能力表明，这个系统上有一些 CPU 可以提供 3000 MHz 的基准频率，而在这个性能级别下的标准基准频率是
（不同）。尽管如此，这些 CPU 是固定的，它们通过 high-priority-cpu-list/high-priority-cpu-mask 呈现。
但如果选择了这个 Intel(R) SST-BF 特性，低优先级 CPU（不在 high-priority-cpu-list 中的）最多只能提供 2400 MHz。
因此，如果这种对低优先级 CPU 的截断是可以接受的，那么用户可以针对上面这个 "sched pipe" 工作负载启用 Intel(R)
SST-BF 特性，因为只使用了两个 CPU，它们可以被调度到高优先级 CPU 上，并获得 400 MHz 的提升。

#### 启用 Intel(R) SST-BF


```

 # intel-speed-select base-freq enable -a
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      base-freq
        enable:success
 package-1
  die-0
    cpu-14
      base-freq
        enable:success

```
在这种情况下，-a 选项是可选的。这不仅启用了 Intel(R) SST-BF，还使用 Intel(R) Speed Select Technology
Core Power（Intel(R) SST-CP）特性来调整核的优先级。这个选项将每个 Intel(R) Speed Select Technology -
Performance Profile（Intel(R) SST-PP）类的性能设为最大性能，以便硬件为每个 CPU 提供尽可能最大的性能。

如果不使用 -a 选项，那么在启用 Intel(R) SST-BF 之前需要以下步骤：

- 发现 Intel(R) SST-BF 并记下低优先级与高优先级基准频率
- 记下高优先级 CPU 列表
- 使用 core-power 特性集启用 CLOS
- 配置 CLOS 参数。使用 CLOS.min 设为最小性能
- 将期望的 CPU 订阅到 CLOS 组

在这种配置下，如果通过绑定来执行相同的工作负载，
```

 #taskset -c 5,6 perf bench -r 100 sched pipe
 # Running 'sched/pipe' benchmark:
 # Executed 1000000 pipe operations between two processes
     Total time: 5.627 [sec]
       5.627922 usecs/op
         177685 ops/sec

```
这样，通过启用 Intel(R) SST-BF，这个基准测试的性能提升了（延迟降低了）7.79%。从 turbostat 输出可以观察到，
高优先级 CPU 达到了 3000 MHz，而之前是 2600 MHz。
```

 #turbostat -c 0-13 --show Package,Core,CPU,Bzy_MHz -i 1
 Package	Core	CPU	Bzy_MHz
 0		0	0	2151
 0		1	1	2166
 0		2	2	2175
 0		3	3	2175
 0		4	4	2175
 0		5	5	3000
 0		6	6	3000
 0		7	7	2180
 0		8	8	2662
 0		9	9	2176
 0		10	10	2175
 0		11	11	2176
 0		12	12	2176
 0		13	13	2661

```
#### 禁用 Intel(R) SST-BF


```

```
# intel-speed-select base-freq disable -a


### Intel(R) Speed Select Technology - Turbo Frequency（Intel(R) SST-TF）


这个特性使得能够基于优先级为核设置不同的 "All core turbo ratio limits"（全核睿频比限制）。
通过使用这个特性，一些核可以被配置为通过指定它们为高优先级来获得更高的睿频频率，
代价是低优先级核上较低或没有睿频频率。

出于这个原因，这个特性只有当系统正忙于利用所有 CPU、但用户想要某个可配置的选项以在某些 CPU 上获得高性能时才有用。

Intel(R) Speed Select Technology - Turbo Frequency（Intel(R) SST-TF）的支持依赖于
Intel(R) Speed Select Technology - Performance Profile（Intel(R) SST-PP）性能级别配置。
有可能只有某个特定的性能级别支持 Intel(R) SST-TF。也有可能只有基础性能级别（level = 0）支持 Intel(R) SST-TF。
因此，首先选择想要的性能级别来启用这个特性。

在这个被测系统中，Intel(R) SST-TF 在基础
```

 # intel-speed-select -c 0 perf-profile info -l 0
 Intel(R) Speed Select Technology
 package-0
  die-0
    cpu-0
      perf-profile-level-0
        ...
        ...
        speed-select-turbo-freq:disabled
        ...
        ...


```
要检查使用 Intel(R) SST-TF 特性能否改善性能，请获取启用 Intel(R) SST-TF 时的睿频频率属性，
并与这个系统的基准睿频能力进行比较。

#### 获取基准睿频能力


```

 # intel-speed-select perf-profile info -l 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      perf-profile-level-0
        ...
        ...
        turbo-ratio-limits-sse
          bucket-0
            core-count:2
            max-turbo-frequency(MHz):3200
          bucket-1
            core-count:4
            max-turbo-frequency(MHz):3100
          bucket-2
            core-count:6
            max-turbo-frequency(MHz):3100
          bucket-3
            core-count:8
            max-turbo-frequency(MHz):3100
          bucket-4
            core-count:10
            max-turbo-frequency(MHz):3100
          bucket-5
            core-count:12
            max-turbo-frequency(MHz):3100
          bucket-6
            core-count:14
            max-turbo-frequency(MHz):3100
          bucket-7
            core-count:16
            max-turbo-frequency(MHz):3100

```
基于上面的数据，当所有 CPU 都繁忙时，可以达到 3100 MHz 的最大频率。如果 cpu 0 - 11 上有些繁忙的工作负载（例如 stress），
```

 # taskset -c 12,13 perf bench -r 100 sched pipe
 # Running 'sched/pipe' benchmark:
 # Executed 1000000 pipe operations between two processes
     Total time: 5.705 [sec]
       5.705488 usecs/op
         175269 ops/sec

```

```

 #turbostat -c 0-13 --show Package,Core,CPU,Bzy_MHz -i 1
 Package	Core	CPU	Bzy_MHz
 0		0	0	3000
 0		1	1	3000
 0		2	2	3000
 0		3	3	3000
 0		4	4	3000
 0		5	5	3100
 0		6	6	3100
 0		7	7	3000
 0		8	8	3100
 0		9	9	3000
 0		10	10	3000
 0		11	11	3000
 0		12	12	3100
 0		13	13	3100

```
基于 turbostat 输出，性能受到了 3100 MHz 的频率上限的限制。要检查能否为 CPU 12 与 CPU 13
改善 hackbench 性能，首先查看这个性能级别下 Intel(R) SST-TF 特性的能力。

#### 获取 Intel(R) SST-TF 能力


```

 # intel-speed-select turbo-freq info -l 0
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-0
      speed-select-turbo-freq
          bucket-0
            high-priority-cores-count:2
            high-priority-max-frequency(MHz):3200
            high-priority-max-avx2-frequency(MHz):3200
            high-priority-max-avx512-frequency(MHz):3100
          bucket-1
            high-priority-cores-count:4
            high-priority-max-frequency(MHz):3100
            high-priority-max-avx2-frequency(MHz):3000
            high-priority-max-avx512-frequency(MHz):2900
          bucket-2
            high-priority-cores-count:6
            high-priority-max-frequency(MHz):3100
            high-priority-max-avx2-frequency(MHz):3000
            high-priority-max-avx512-frequency(MHz):2900
          speed-select-turbo-freq-clip-frequencies
            low-priority-max-frequency(MHz):2600
            low-priority-max-avx2-frequency(MHz):2400
            low-priority-max-avx512-frequency(MHz):2100

```
基于上面的输出，有一个 Intel(R) SST-TF bucket，其中有 2 个高优先级核。
如果只设置 2 个高优先级核，那么这些核上的最大睿频频率可以提升到 3200 MHz。
这比所有核的基准睿频能力高了 100 MHz。

相应地，对于 hackbench 工作负载，可以将两个 CPU 设为高优先级，其余为低优先级。
一个副作用是，一旦启用，低优先级核将被截断到较低的 2600 MHz 频率。

#### 启用 Intel(R) SST-TF


```

 # intel-speed-select -c 12,13 turbo-freq enable -a
 Intel(R) Speed Select Technology
 Executing on CPU model: X
 package-0
  die-0
    cpu-12
      turbo-freq
        enable:success
 package-0
  die-0
    cpu-13
      turbo-freq
        enable:success
 package--1
  die-0
    cpu-63
      turbo-freq --auto
        enable:success

```
在这种情况下，选项 "-a" 是可选的。如果设置，它会启用 Intel(R) SST-TF 特性，
并使用 Intel Speed Select Technology Core Power（Intel(R) SST-CP）特性将 CPU 设为高优先级与低优先级。
通过 "-c" 参数传入的 CPU 编号被标记为高优先级，包括其兄弟核。

如果不使用 -a 选项，那么在启用 Intel(R) SST-TF 之前需要以下步骤：

- 发现 Intel(R) SST-TF 并记下高优先级核的 bucket 与最大频率

- 使用 core-power 特性集启用 CLOS - 配置 CLOS 参数

- 将期望的 CPU 订阅到 CLOS 组，确保高优先级核被设为最大频率

如果执行相同的 hackbench 工作负载，将 hackbench 线程调度到高优先级核上，
```

 #taskset -c 12,13 perf bench -r 100 sched pipe
 # Running 'sched/pipe' benchmark:
 # Executed 1000000 pipe operations between two processes
     Total time: 5.510 [sec]
       5.510165 usecs/op
         180826 ops/sec

```
这在繁忙的系统上带来了约 3.3% 的性能提升。这里 turbostat 输出将显示 CPU 12 与 CPU 13 获得了 100 MHz 的提升。
```

 #turbostat -c 0-13 --show Package,Core,CPU,Bzy_MHz -i 1
 Package	Core	CPU	Bzy_MHz
 ...
 0		12	12	3200
 0		13	13	3200

```
