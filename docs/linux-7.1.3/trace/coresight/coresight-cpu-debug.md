## Coresight CPU 调试模块


   :Author:   Leo Yan <leo.yan@linaro.org>
   :Date:     April 5th, 2017

### 简介


Coresight CPU 调试模块定义于 ARMv8-a 架构参考手册（ARM DDI 0487A.k）的
“Part H: External debug” 章节，CPU 可集成调试模块，主要用于两种模式：
self-hosted debug（自托管调试）和 external debug（外部调试）。通常 external
debug 模式广为人知，即外部调试器通过 JTAG 端口连接到 SoC；另一方面，程序
也可依赖 self-hosted debug 模式来探索调试方法，本文档重点关注这一部分。

该调试模块提供基于采样的性能剖析（profiling）扩展，可用于对 CPU 程序计数器、
安全状态和异常级别等进行采样；通常每个 CPU 都有一个专用的调试模块与之连接。
基于 self-hosted debug 机制，Linux 内核可在内核发生 panic 时从 mmio 区域
访问这些相关寄存器。内核 panic 的回调通知器（callback notifier）会为每个 CPU
转储相关寄存器；这最终有助于对 panic 进行辅助分析。


### 实现


- 在驱动注册期间，它使用 EDDEVID 和 EDDEVID1 这两个设备 ID 寄存器来判断是否
  实现了基于采样的性能剖析。在某些平台上，该硬件特性被完全或部分实现；若不
  支持该特性，则注册将失败。

- 在编写本文档时，调试驱动主要依赖内核 panic 回调通知器从三个采样寄存器收集
  的信息：EDPCSR、EDVIDSR 和 EDCIDSR：从 EDPCSR 可获取程序计数器；EDVIDSR
  包含安全状态、异常级别、位宽等信息；EDCIDSR 是上下文 ID 值，包含
  CONTEXTIDR_EL1 的采样值。

- 该驱动支持运行于 AArch64 或 AArch32 模式的 CPU。两者寄存器命名约定略有不同，
  AArch64 使用 'ED' 作为寄存器前缀（ARM DDI 0487A.k，H9.1 章），AArch32 使用
  'DBG' 作为前缀（ARM DDI 0487A.k，G5.1 章）。驱动统一采用 AArch64 命名约定。

- ARMv8-a（ARM DDI 0487A.k）和 ARMv7-a（ARM DDI 0406C.b）的寄存器位定义不同。
  因此驱动整合了两者的差异：

  若 PCSROffset=0b0000，在 ARMv8-a 上 EDPCSR 特性未实现；但 ARMv7-a 定义为
  “PCSR 采样值会根据指令集状态偏移一个值”。对于 ARMv7-a，驱动进一步检查 CPU
  运行于 ARM 还是 thumb 指令集，并对 PCSR 值进行校准，关于偏移的详细说明见
  ARMv7-a ARM（ARM DDI 0406C.b）C11.11.34 章 “DBGPCSR, Program Counter
  Sampling Register”。

  若 PCSROffset=0b0010，ARMv8-a 定义为“已实现的 EDPCSR，采样不应用偏移，且不
  在 AArch32 状态下采样指令集状态”。因此在 ARMv8 上，若 EDDEVID1.PCSROffset
  为 0b0010 且 CPU 运行于 AArch32 状态，则不对 EDPCSR 采样；当 CPU 运行于
  AArch64 状态时，EDPCSR 被采样且不应用偏移。


### 时钟与电源域


在访问调试寄存器之前，应确保时钟和电源域已正确使能。在 ARMv8-a ARM
（ARM DDI 0487A.k）的 'H9.1 Debug registers' 章节中，调试寄存器分布在两个域中：
debug 域和 CPU 域。

```

                                +---------------+
                                |               |
                                |               |
                     +----------+--+            |
        dbg_clock -->|          |**|            |<-- cpu_clock
                     |    Debug |**|   CPU      |
 dbg_power_domain -->|          |**|            |<-- cpu_power_domain
                     +----------+--+            |
                                |               |
                                |               |
                                +---------------+

```
对于 debug 域，用户使用 DT binding（设备树绑定）“clocks” 和 “power-domains”
来为调试逻辑指定相应的时钟源和电源。驱动按需调用 pm_runtime_{put|get} 操作来
处理调试电源域。

对于 CPU 域，不同的 SoC 设计有不同的电源管理方案，最终会严重影响 external
debug 模块。因此可分为以下几种情况：

- 在具有合理电源控制器、能正确处理 CPU 电源域的系统中，CPU 电源域可由驱动中的
  EDPRCR 寄存器控制。驱动首先写 EDPRCR.COREPURQ 位为 CPU 上电，然后写
  EDPRCR.CORENPDRQ 位以模拟 CPU 掉电。这样可以确保 CPU 电源域在访问调试相关
  寄存器期间被正确上电；

- 某些设计在集群中所有 CPU 掉电时会关闭整个集群——包括本应在 debug 电源域中
  保持供电的调试寄存器部分。这些情况不会遵循 EDPRCR 中的位，因此这些设计无法
  以 CoreSight / Debug 设计者预期的方式支持掉电调试。这意味着即使检查 EDPRSR，
  若目标寄存器未上电，也可能导致总线挂起（bus hang）。

  在这种情况下，在调试寄存器未上电时访问它们无异于灾难；因此我们需要在启动时就
  阻止 CPU 低功耗状态，或在用户运行时启用模块时阻止。详细用法请参见
  “How to use the module” 章节。


### 设备树绑定


有关详细信息，请参阅 Documentation/devicetree/bindings/arm/arm,coresight-cpu-debug.yaml。


### 如何使用该模块


若要在启动时就启用调试功能，可在内核命令行参数中添加 “coresight_cpu_debug.enable=1”。

该驱动也可作为模块工作，因此可在 insmod 时启用调试

```

  # insmod coresight_cpu_debug.ko debug=1

```
若在启动或 insmod 模块时未启用调试，驱动会使用 debugfs 文件系统提供一个旋钮，
用于动态启用或禁用调试：

```

  # echo 1 > /sys/kernel/debug/coresight_cpu_debug/enable

```

```

  # echo 0 > /sys/kernel/debug/coresight_cpu_debug/enable

```
如 “Clock and power domain” 章节所述，若你使用的平台具有会关闭调试逻辑的空闲
状态，且电源控制器无法很好地响应来自 EDPRCR 的请求，则应在启用 CPU 调试功能
之前先限制 CPU 空闲状态；这样才能确保对调试逻辑的访问。

若要在启动时就限制空闲状态，可在内核命令行中使用 “nohlt” 或 “cpuidle.off=1”。

在运行时，可通过以下方法禁用空闲状态：

可以通过 PM QoS 子系统禁用 CPU 空闲状态，更具体地说是使用 “/dev/cpu_dma_latency”
接口（详见 Documentation/power/pm_qos_interface.rst）。如 PM QoS 文档所述，所
请求的参数将一直生效，直到文件描述符被释放。

```

  # exec 3<> /dev/cpu_dma_latency; echo 0 >&3
  ...
  Do some work...
  ...
  # exec 3<>-

```
同样的操作也可从应用程序中完成。

通过 cpuidle sysfs 禁用特定 CPU 的特定空闲状态（参见

```

  # echo 1 > /sys/devices/system/cpu/cpu$cpu/cpuidle/state$state/disable

```

### 输出格式


```

  ARM external debug module:
  coresight-cpu-debug 850000.debug: CPU[0]:
  coresight-cpu-debug 850000.debug:  EDPRSR:  00000001 (Power:On DLK:Unlock)
  coresight-cpu-debug 850000.debug:  EDPCSR:  handle_IPI+0x174/0x1d8
  coresight-cpu-debug 850000.debug:  EDCIDSR: 00000000
  coresight-cpu-debug 850000.debug:  EDVIDSR: 90000000 (State:Non-secure Mode:EL1/0 Width:64bits VMID:0)
  coresight-cpu-debug 852000.debug: CPU[1]:
  coresight-cpu-debug 852000.debug:  EDPRSR:  00000001 (Power:On DLK:Unlock)
  coresight-cpu-debug 852000.debug:  EDPCSR:  debug_notifier_call+0x23c/0x358
  coresight-cpu-debug 852000.debug:  EDCIDSR: 00000000
  coresight-cpu-debug 852000.debug:  EDVIDSR: 90000000 (State:Non-secure Mode:EL1/0 Width:64bits VMID:0)

```
