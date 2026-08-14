
## Linux 微码加载器


:Authors: - Fenghua Yu <fenghua.yu@intel.com>
          - Borislav Petkov <bp@suse.de>
   - Ashok Raj <ashok.raj@intel.com>

内核提供了一个 x86 微码加载机制，用于在操作系统中提供微码加载方法。潜在的
使用场景包括：在 OEM 停止支持之后更新平台上的微码，以及在不重启的情况下
更新长期运行系统的微码。

该加载器支持三种加载方式：

## 早期加载微码


内核可以在启动过程的非常早期更新微码。在早期加载微码可以在内核启动期间
被观察到问题之前修复 CPU 缺陷。

微码存储在 initrd 文件中。在启动过程中，内核从中读取并加载到 CPU 核心。

组合 initrd 镜像的格式为：以（未压缩的）cpio 格式存放的微码，后接
（可能被压缩的）initrd 镜像。加载器在启动期间解析该组合 initrd 镜像。

cpio 命名空间中的微码文件为：

on Intel:
  kernel/x86/microcode/GenuineIntel.bin
on AMD  :
  kernel/x86/microcode/AuthenticAMD.bin

在 BSP（BootStrapping Processor，引导处理器）启动（SMP 之前）期间，内核
扫描 initrd 中的微码文件。如果找到与 CPU 匹配的微码，则会在 BSP 上应用，
随后在所有 AP（Application Processors，应用处理器）上应用。

加载器还会在内存中保存与 CPU 匹配的微码。因此，当 CPU 从睡眠状态恢复时，
会应用缓存的微码补丁。

下面是一个准备带微码的 initrd 的粗略示例（这通常会在重新生成 initrd 时由
发行版自动完成，所以你实际上不必自己操作。此处仅为留作日后参考而记录）。
```

  #!/bin/bash

  if [ -z "$1" ]; then
      echo "You need to supply an initrd file"
      exit 1
  fi

  INITRD="$1"

  DSTDIR=kernel/x86/microcode
  TMPDIR=/tmp/initrd

  rm -rf $TMPDIR

  mkdir $TMPDIR
  cd $TMPDIR
  mkdir -p $DSTDIR

  if [ -d /lib/firmware/amd-ucode ]; then
          cat /lib/firmware/amd-ucode/microcode_amd*.bin > $DSTDIR/AuthenticAMD.bin
  fi

  if [ -d /lib/firmware/intel-ucode ]; then
          cat /lib/firmware/intel-ucode/* > $DSTDIR/GenuineIntel.bin
  fi

  find . | cpio -o -H newc >../ucode.cpio
  cd ..
  mv $INITRD $INITRD.orig
  cat ucode.cpio $INITRD.orig > $INITRD

  rm -rf $TMPDIR


```
系统需要将微码软件包安装到 /lib/firmware，否则如果你的路径在别处，和/或
你直接从处理器厂商网站下载了微码，就需要修改上面的路径。

## 延迟加载


你只需安装发行版提供的微码软件包，然后以 root 身份执行
```
  # echo 1 > /sys/devices/system/cpu/microcode/reload
```
即可。

加载机制会在 /lib/firmware/{intel-ucode,amd-ucode} 中查找微码 blob。
发行版的默认安装包已经将它们放在那里。

自内核 5.19 起，延迟加载默认不启用。

/dev/cpu/microcode 方式已在 5.19 中被移除。

## 为什么延迟加载是危险的？


### 同步所有 CPU


接收微码更新的微码引擎在 SMT 系统的两个逻辑线程之间共享。因此，当
在核心的一个 SMT 线程上执行更新时，其兄弟线程会“自动”获得更新。

由于微码也可以“模拟”MSR，在微码更新进行期间，这些被模拟的 MSR 会
暂时消失。如果 SMT 兄弟线程恰好正在访问这样一个 MSR 的过程中，这就会
导致不可预测的结果。通常观察到的情况是，此类 MSR 访问会引发 #GP，
以表明前者已不存在。

MSR 消失只是被观察到的其中一个常见问题。任何其它正在被打补丁、并被
另一个 SMT 兄弟线程并发执行的指令，也可能导致类似的、不可预测的行为。

为了消除这种情况，引入了基于 stop_machine() 的 CPU 同步机制，以
保证所有逻辑 CPU 不会执行任何代码，而只是在一个自旋循环中等待，
轮询一个原子变量。

虽然这解决了设备中断或外部中断、包括 LVT（如 CMCI 等）在内的 IPI，
但它无法处理其它无法关闭的特殊中断。这些中断包括机器检查（#MC）、
系统管理中断（#SMI）和不可屏蔽中断（#NMI）。

### 机器检查


机器检查（#MC）是不可屏蔽的。MCE 有两种类型：致命且不可恢复的 MCE，
以及可恢复的 MCE。虽然不可恢复的错误是致命的，可恢复的错误如果发生在
内核上下文中，内核也会将其视为致命的。

在某些 Intel 机器上，MCE 还会被广播到系统中所有线程。如果一个线程正在
执行 WRMSR 的过程中，MCE 会在此流程结束时被触发。无论哪种情况，它们都会
等待执行 wrmsr(0x79) 的线程在 MCE 处理程序中会合；如果系统中任何线程未能
签到 MCE 会合点，最终将导致关机。

为了万无一失并获得可预测的行为，OS 可以选择设置 MCG_STATUS.MCIP。由于
系统中最多只能有一个 MCE，如果触发了 MCE，上述条件会自动升级为系统复位。
OS 可以在该核心更新结束时关闭 MCIP。

### 系统管理中断


SMI 也会被广播到平台上的所有 CPU。微码更新会在写入 MSR 0x79 之前
请求对核心的独占访问。因此，如果真的发生一个线程处于 WRMSR 流程中，
而第二个线程收到 SMI 的情况，该线程会停在 SMI 处理程序的第一条指令处。

由于辅助线程停在 SMI 的第一条指令处，它处在正在被打补丁的指令
执行过程中的可能性很小。此外，OS 无法阻止 SMI 的发生。

### 不可屏蔽中断


当核心的 thread0 正在进行微码更新时，如果 thread1 被拉入 NMI，由于上述
原因，这可能会导致不可预测的行为。

OS 可以选择多种方法来避免陷入这种情况。


### 该微码是否适合延迟加载？


延迟加载是在系统完全正常运行并运行实际工作负载时进行的。延迟加载的行为
取决于升级到新补丁之前 CPU 上的基础补丁是什么。

对于 Intel CPU 而言情况确实如此。

例如，假设某 CPU 的补丁级别为 1，而更新目标是补丁级别 3。

在 patch1 与 patch3 之间，patch2 可能已经废弃了某个软件可见的特性。

如果软件甚至可能正在使用该特性，这是不可接受的。例如，假设 MSR_X 在
更新后不再可用，访问该 MSR 将导致 #GP 错误。

基本上无法声明某个新的微码更新适合延迟加载。这是导致延迟加载默认不启用的
另一个原因。

## 内置微码


该加载器还支持通过常规的内置固件方法 CONFIG_EXTRA_FIRMWARE 加载所提供的
内置微码。目前仅支持 64 位。

```
  CONFIG_EXTRA_FIRMWARE="intel-ucode/06-3a-09 amd-ucode/microcode_amd_fam15h.bin"
  CONFIG_EXTRA_FIRMWARE_DIR="/lib/firmware"

```
```
  /lib/firmware/
  |-- amd-ucode
  ...
  |   |-- microcode_amd_fam15h.bin
  ...
  |-- intel-ucode
  ...
  |   |-- 06-3a-09
  ...

```
这样构建系统就能找到这些文件并将其集成到最终的内核镜像中。早期加载器会
找到它们并应用。

毋庸讳言，这种方法并非最灵活的一种，因为每次 CPU 厂商发布更新后的微码时
都需要重新构建内核。
