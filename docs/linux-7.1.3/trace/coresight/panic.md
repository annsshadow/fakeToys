## 使用 Coresight 应对内核 panic 与看门狗复位


### 简
本文档介绍如何使Linux coresight 跟踪支持来调试内panic 与看门狗复位场景
### 内核 panic 期间Coresight 跟踪

coresight 驱动的角度来看，处理内核 panic 情形有四个主要需求
a. 支持从保留内存区域分配跟踪缓冲区页。平台可通过在相coresight 节点上新增的 device tree 属性来声明这一点
b. 支持在内panic 时停coresight 模块

c. 以指定格式保存所需的元数据

d. 支持读取内核 panic 时捕获的跟踪数据

#### 从保RAM 分配跟踪缓冲区页

一个新的可device tree 属"memory-region" 被加入到 Coresight TMC 设备节点中，用于给出跟踪缓冲区的基地址与大小
跟踪缓冲区的静态分配可确保 IOMMU 启用与禁用两种情况都被处理。此外，支持持久 RAM 的平台允许用户在后续启动中读取跟踪数据，而无需启动 crashdump 内核
注意对于 ETR sink 设备，该保留区域将同时用于跟踪捕获与跟踪数据读取对于 ETF sink 设备，将使用内部 SRAM 进行跟踪捕获，并同步到保留区域以供读取

#### 在内panic 时禁coresight 模块

为了避免内核 panic 后丢失相关跟踪数据，最好在内核 panic 时停coresight 模块
这可以通过配置 comparator、CTI sink 来实现：

```
           Trigger on panic
    Comparator --->External out --->CTI -->External In---->ETR/ETF stop

```
#### 在内panic 时保存元数据

Coresight 元数据包含除跟踪数据外，成功进行跟踪解码所需的所有附加数据。这包括 ETR/ETF/ETB 寄存器快照等
为此，一个新的可选设备属"memory-region" 被加入到 ETR/ETF/ETB 设备节点中
#### 读取内核 panic 时捕获的跟踪数据

内核 panic 时捕获的跟踪数据，可通过特殊的设备文/dev/crash_tmc_xxx 从重启后的内核或 crashdump 内核中读取。该设备文件仅在存在有效 crashdata 时才会被创建
#### 内核 panic 情况下的跟踪捕获与解码一般流
1. 通过 sysfs 接口在所有核上使能源sink。ETR sink 应通过sysfs 选择 "resrv" 缓冲区模式，从保留内存分配跟踪缓冲区
2. 运行相关测试
3. 发生内核 panic 时，所coresight 模块被禁用，必要的元数据由内panic 处理函数同步
   系统最终将重启或启crashdump 内核
4. 对于支持 crashdump 内核的平台，可使coresight sysfs 接口直接crashdump 内核转储原始跟踪数据。此种情况下无需持久 RAM
5. 对于支持持久 RAM 的平台，可在随后Linux 启动中通过 coresight sysfs 接口转储跟踪数据。此种情况下无需 crashdump 内核。持RAM 可确保跟踪数据在重启后保持完整
### 看门狗复位期间的 Coresight 跟踪

处理看门狗复位与内核 panic 情况的主要区别如下：

a. 保存 coresight 元数据需SCP（系统控制处理器）固件按指定格式负责，而非内核
b. 固件为跟踪缓冲区与元数据提供的保留内存区域必须位于持RAM 中   注意：这是看门狗复位情况下的要求，但在内panic 情况下为可选项
看门狗复位仅能在满足上述两项要求的平台上得到支持
### 使用 ETR sink 测试内核 panic 情况的示例命

1. 在内bootargs 中加"crash_kexec_post_notifiers" 启动 Linux 内核。若用户希望crashdump 内核读取跟踪数据，这是必需的
```

    #echo 1 > /sys/kernel/config/cs-syscfg/configurations/panicstop/enable

```
```

    #./cti_setup.sh

    #cat cti_setup.sh


    cd /sys/bus/coresight/devices/

    ap_cti_config () {
      #ETM trig out[0] trigger to Channel 0
      echo 0 4 > channels/trigin_attach
    }

    etf_cti_config () {
      #ETF Flush in trigger from Channel 0
      echo 0 1 > channels/trigout_attach
      echo 1 > channels/trig_filter_enable
    }

    etr_cti_config () {
      #ETR Flush in from Channel 0
      echo 0 1 > channels/trigout_attach
      echo 1 > channels/trig_filter_enable
    }

    ctidevs=`find . -name "cti*"`

    for i in $ctidevs
    do
            cd $i

            connection=`find . -name "ete*"`
            if [ ! -z "$connection" ]
            then
                    echo "AP CTI config for $i"
                    ap_cti_config
            fi

            connection=`find . -name "tmc_etf*"`
            if [ ! -z "$connection" ]
            then
                    echo "ETF CTI config for $i"
                    etf_cti_config
            fi

            connection=`find . -name "tmc_etr*"`
            if [ ! -z "$connection" ]
            then
                    echo "ETR CTI config for $i"
                    etr_cti_config
            fi

            cd ..
    done

```
注：CTI 连接SoC 相关的，因此上面的脚本仅供参考
```

    #echo "resrv" > /sys/bus/coresight/devices/tmc_etr0/buf_mode_preferred

```
```

    #echo 1 > /sys/bus/coresight/devices/tmc_etr0/stop_on_flush

```
6. 使用 sysfs 接口在核 1 与核 2 上启Coresight 跟踪

```

    #taskset -c 1 dd if=/dev/urandom of=/dev/null &

```
```

    #echo 1 > /proc/sys/kernel/panic
    #taskset -c 2 echo c > /proc/sysrq-trigger

```
```

    #dd if=/dev/crash_tmc_etr0 of=/trace/cstrace.bin

```
10. 运行 opencsd 解码器工脚本来生成指令跟踪
#### 指令跟踪转储示例


```

    A                                  etm4_enable_hw: ffff800008ae1dd4
    CONTEXT EL2                        etm4_enable_hw: ffff800008ae1dd4
    I                                  etm4_enable_hw: ffff800008ae1dd4:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1dd8:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1ddc:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1de0:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1de4:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1de8:
    d503233f   paciasp
    I                                  etm4_enable_hw: ffff800008ae1dec:
    a9be7bfd   stp     x29, x30, [sp, #-32]!
    I                                  etm4_enable_hw: ffff800008ae1df0:
    910003fd   mov     x29, sp
    I                                  etm4_enable_hw: ffff800008ae1df4:
    a90153f3   stp     x19, x20, [sp, #16]
    I                                  etm4_enable_hw: ffff800008ae1df8:
    2a0003f4   mov     w20, w0
    I                                  etm4_enable_hw: ffff800008ae1dfc:
    900085b3   adrp    x19, ffff800009b95000 <reserved_mem+0xc48>
    I                                  etm4_enable_hw: ffff800008ae1e00:
    910f4273   add     x19, x19, #0x3d0
    I                                  etm4_enable_hw: ffff800008ae1e04:
    f8747a60   ldr     x0, [x19, x20, lsl #3]
    E                                  etm4_enable_hw: ffff800008ae1e08:
    b4000140   cbz     x0, ffff800008ae1e30 <etm4_starting_cpu+0x50>
    I    149.039572921                 etm4_enable_hw: ffff800008ae1e30:
    a94153f3   ldp     x19, x20, [sp, #16]
    I    149.039572921                 etm4_enable_hw: ffff800008ae1e34:
    52800000   mov     w0, #0x0                        // #0
    I    149.039572921                 etm4_enable_hw: ffff800008ae1e38:
    a8c27bfd   ldp     x29, x30, [sp], #32

    ..snip

        149.052324811           chacha_block_generic: ffff800008642d80:
    9100a3e0   add     x0,
    I    149.052324811           chacha_block_generic: ffff800008642d84:
    b86178a2   ldr     w2, [x5, x1, lsl #2]
    I    149.052324811           chacha_block_generic: ffff800008642d88:
    8b010803   add     x3, x0, x1, lsl #2
    I    149.052324811           chacha_block_generic: ffff800008642d8c:
    b85fc063   ldur    w3, [x3, #-4]
    I    149.052324811           chacha_block_generic: ffff800008642d90:
    0b030042   add     w2, w2, w3
    I    149.052324811           chacha_block_generic: ffff800008642d94:
    b8217882   str     w2, [x4, x1, lsl #2]
    I    149.052324811           chacha_block_generic: ffff800008642d98:
    91000421   add     x1, x1, #0x1
    I    149.052324811           chacha_block_generic: ffff800008642d9c:
    f100443f   cmp     x1, #0x11



```
```

    A                                  etm4_enable_hw: ffff800008ae1dd4
    CONTEXT EL2                        etm4_enable_hw: ffff800008ae1dd4
    I                                  etm4_enable_hw: ffff800008ae1dd4:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1dd8:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1ddc:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1de0:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1de4:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1de8:
    d503233f   paciasp
    I                                  etm4_enable_hw: ffff800008ae1dec:
    a9be7bfd   stp     x29, x30, [sp, #-32]!
    I                                  etm4_enable_hw: ffff800008ae1df0:
    910003fd   mov     x29, sp
    I                                  etm4_enable_hw: ffff800008ae1df4:
    a90153f3   stp     x19, x20, [sp, #16]
    I                                  etm4_enable_hw: ffff800008ae1df8:
    2a0003f4   mov     w20, w0
    I                                  etm4_enable_hw: ffff800008ae1dfc:
    900085b3   adrp    x19, ffff800009b95000 <reserved_mem+0xc48>
    I                                  etm4_enable_hw: ffff800008ae1e00:
    910f4273   add     x19, x19, #0x3d0
    I                                  etm4_enable_hw: ffff800008ae1e04:
    f8747a60   ldr     x0, [x19, x20, lsl #3]
    E                                  etm4_enable_hw: ffff800008ae1e08:
    b4000140   cbz     x0, ffff800008ae1e30 <etm4_starting_cpu+0x50>
    I    149.046243445                 etm4_enable_hw: ffff800008ae1e30:
    a94153f3   ldp     x19, x20, [sp, #16]
    I    149.046243445                 etm4_enable_hw: ffff800008ae1e34:
    52800000   mov     w0, #0x0                        // #0
    I    149.046243445                 etm4_enable_hw: ffff800008ae1e38:
    a8c27bfd   ldp     x29, x30, [sp], #32
    I    149.046243445                 etm4_enable_hw: ffff800008ae1e3c:
    d50323bf   autiasp
    E    149.046243445                 etm4_enable_hw: ffff800008ae1e40:
    d65f03c0   ret
    A                                ete_sysreg_write: ffff800008adfa18

    ..snip

    I     149.05422547                          panic: ffff800008096300:
    a90363f7   stp     x23, x24, [sp, #48]
    I     149.05422547                          panic: ffff800008096304:
    6b00003f   cmp     w1, w0
    I     149.05422547                          panic: ffff800008096308:
    3a411804   ccmn    w0, #0x1, #0x4, ne  // ne = any
    N     149.05422547                          panic: ffff80000809630c:
    540001e0   b.eq    ffff800008096348 <panic+0xe0>  // b.none
    I     149.05422547                          panic: ffff800008096310:
    f90023f9   str     x25, [sp, #64]
    E     149.05422547                          panic: ffff800008096314:
    97fe44ef   bl      ffff8000080276d0 <panic_smp_self_stop>
    A                                           panic: ffff80000809634c
    I     149.05422547                          panic: ffff80000809634c:
    910102d5   add     x21, x22, #0x40
    I     149.05422547                          panic: ffff800008096350:
    52800020   mov     w0, #0x1                        // #1
    E     149.05422547                          panic: ffff800008096354:
    94166b8b   bl      ffff800008631180 <bust_spinlocks>
    N    149.054225518                 bust_spinlocks: ffff800008631180:
    340000c0   cbz     w0, ffff800008631198 <bust_spinlocks+0x18>
    I    149.054225518                 bust_spinlocks: ffff800008631184:
    f000a321   adrp    x1, ffff800009a98000 <pbufs.0+0xbb8>
    I    149.054225518                 bust_spinlocks: ffff800008631188:
    b9405c20   ldr     w0, [x1, #92]
    I    149.054225518                 bust_spinlocks: ffff80000863118c:
    11000400   add     w0, w0, #0x1
    I    149.054225518                 bust_spinlocks: ffff800008631190:
    b9005c20   str     w0, [x1, #92]
    E    149.054225518                 bust_spinlocks: ffff800008631194:
    d65f03c0   ret
    A                                           panic: ffff800008096358

```
### 基于 Perf 的测

#### 启动 perf 会话

```

    perf record -e cs_etm/panicstop,@tmc_etf1/ -C 1
    perf record -e cs_etm/panicstop,@tmc_etf2/ -C 2

```
```

    perf record -e cs_etm/panicstop,@tmc_etr0/ -C 1,2

```
#### panic 后读取跟踪数
上文介绍的相同的基于 sysfs 的方法，可用于在内核 panic 重启后获取并解码跟踪数据