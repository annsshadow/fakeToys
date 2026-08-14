
## IAA 压缩加速器加密驱动


Tom Zanussi <tom.zanussi@linux.intel.com>

IAA 加密驱动支持符合 RFC 1951 所描述 DEFLATE 压缩标准的压缩/解压缩，
这也是本模块导出的压缩/解压缩算法。

IAA 硬件规格可在此处找到：

  https://cdrdv2.intel.com/v1/dl/getContent/721858

iaa_crypto 驱动被设计为高阶层压缩设备（如 zswap）之下的一层。

用户可以通过在允许选择压缩算法的任何设施中指定受支持的 IAA 压缩算法之一，
来选择 IAA 压缩/解压缩加速。

例如，zswap 设备可以通过选择 'deflate-iaa' 加密压缩算法来选择 IAA 的
'fixed' 模式：
```
  # echo deflate-iaa > /sys/module/zswap/parameters/compressor

```
这将告知 zswap 在所有压缩和解压缩中使用 IAA 的 'fixed' 压缩模式。

目前只有一种压缩模式可用，即 'fixed' 模式。

'fixed' 压缩模式实现了 RFC 1951 所指定的压缩方案，并被赋予加密算法名称
'deflate-iaa'。（由于 IAA 硬件具有 4k 历史窗口限制，只有 <= 4k 的缓冲区，
或采用 <= 4k 历史窗口压缩的缓冲区，才在技术上符合 deflate 规范，而该规范
允许最多 32k 的窗口。由于此限制，IAA fixed 模式 deflate 算法被赋予了自己的
算法名称，而非简单的 'deflate'）。


## 配置选项与其他设置


IAA 加密驱动可通过 menuconfig 使用如下选项获得：
```
  Cryptographic API -> Hardware crypto devices -> Support for Intel(R) IAA Compression Accelerator

```
在配置文件中，该选项名为 CONFIG_CRYPTO_DEV_IAA_CRYPTO。

IAA 加密驱动还支持统计功能，可通过以下选项获得：
```
  Cryptographic API -> Hardware crypto devices -> Support for Intel(R) IAA Compression -> Enable Intel(R) IAA Compression Accelerator Statistics

```
在配置文件中，该选项名为 CONFIG_CRYPTO_DEV_IAA_CRYPTO_STATS。

```
  CONFIG_IRQ_REMAP=y
  CONFIG_INTEL_IOMMU=y
  CONFIG_INTEL_IOMMU_SVM=y
  CONFIG_PCI_ATS=y
  CONFIG_PCI_PRI=y
  CONFIG_PCI_PASID=y
  CONFIG_INTEL_IDXD=m
  CONFIG_INTEL_IDXD_SVM=y

```
IAA 是可与 Intel IOMMU 协同工作的首批 Intel 加速器 IP 之一。存在多种模式：
```
  - Scalable
  - Legacy
  - No IOMMU


```
### 可扩展模式（Scalable mode）


可扩展模式支持共享虚拟内存（SVM 或 SVA）。它通过以下方式启用：
```
  intel_iommu=on,sm_on

```
且 BIOS 中开启了 VT-d。

在可扩展模式下，共享和专用工作队列均可使用。

```
  Socket Configuration > IIO Configuration > Intel VT for Directed I/O (VT-d) > Intel VT for Directed I/O

  Socket Configuration > IIO Configuration > PCIe ENQCMD > ENQCMDS


```
### 传统模式（Legacy mode）


```
  intel_iommu=off

```
或 BIOS 中未开启 VT-d。

如果你已启动进入 Linux 但不确定 VT-d 是否开启，可执行 "dmesg | grep -i dmar"。
如果没有看到若干 DMAR 设备被枚举，则很可能 VT-d 未开启。

在传统模式下，只有专用工作队列可供使用。


### 无 IOMMU 模式（No IOMMU mode）


```
  iommu=off.

```
在无 IOMMU 模式下，只有专用工作队列可供使用。


## 使用方法


### accel-config


加载时，iaa_crypto 驱动会自动创建一个默认配置并启用它，同时分配默认驱动属性。
如果需要不同的配置或驱动属性集合，用户必须先禁用 IAA 设备和工作队列、重置配置，
然后通过移除并重新插入 iaa_crypto 模块，向加密子系统重新注册 deflate-iaa 算法。

下面『用例』小节中的 iaa_disable_script 可用于禁用默认配置。

有关默认配置的详细信息，请参见下文的 iaa_default_config。

不过，由于加速器设备的复杂性和可配置性，用户更可能需要配置设备并手动启用所需的
设备和工作队列。

帮助用户完成此操作的用户空间工具名为 accel-config。强烈建议使用 accel-config
来配置设备或加载先前保存的配置。也可以通过 sysfs 直接控制设备，但需特别警告：
只有在你确切知道自己在做什么时才应这样做。后续章节不会涵盖 sysfs 接口，而是假定
你将使用 accel-config。

如有兴趣，可查阅附录中的 iaa_sysfs_config 小节以了解 sysfs 接口详情。

accel-config 工具及其构建说明可在此处找到：

  https://github.com/intel/idxd-config/#readme

### 典型用法


为了让 iaa_crypto 模块真正代表某个设施执行压缩/解压缩工作，需要将一个或多个
IAA 工作队列绑定到 iaa_crypto 驱动。

例如，下面是一个配置 IAA 工作队列并将其绑定到 iaa_crypto 驱动的示例（注意设备名
以 'iax' 而非 'iaa' 指定——这是因为上游仍然
```
  # configure wq1.0

  accel-config config-wq --group-id=0 --mode=dedicated --type=kernel --priority=10 --name="iaa_crypto" --driver-name="crypto" iax1/wq1.0

  accel-config config-engine iax1/engine1.0 --group-id=0

  # enable IAA device iax1

  accel-config enable-device iax1

  # enable wq1.0 on IAX device iax1

  accel-config enable-wq iax1/wq1.0

```
每当有新的工作队列绑定到或解绑自 iaa_crypto 驱动时，可用的工作队列会被『重新平衡』，
使得从特定 CPU 提交的工作被分配给最合适的可用工作队列。当前的最佳实践是为每个 IAA
设备配置并绑定至少一个工作队列，但只要系统中存在至少一个配置并绑定到任意 IAA 设备的
工作队列，iaa_crypto 驱动就能工作，尽管效率很可能不如前者。

在第一个 IAA 工作队列成功绑定到 iaa_crypto 驱动后，IAA 加密算法即进入可运行状态，
压缩和解压缩操作被完全启用。

类似地，在最后一个 IAA 工作队列从 iaa_crypto 驱动解绑后，IAA 加密算法将不再可运行，
压缩和解压缩操作被禁用。

因此，只有当一个或多个工作队列绑定到 iaa_crypto 驱动时，IAA 加密算法以及 IAA 硬件
才可用。

当没有 IAA 工作队列绑定到驱动时，可以通过移除模块来注销 IAA 加密算法。


### 驱动属性


有若干用户可配置的驱动属性可用于配置各种操作模式。它们及其默认值如下所列。要设置
其中任一属性，请将相应值 echo 到位于 /sys/bus/dsa/drivers/crypto/ 下的属性文件中。

在 IAA 算法注册时捕获的属性设置会被保存在各算法的 crypto_ctx 中，并在使用该算法时
应用于所有压缩和解压缩。

可用属性如下：

  - verify_compress

    切换压缩校验。若设置，每次压缩将在内部进行解压缩并校验内容，返回错误：
```
      echo 0 > /sys/bus/dsa/drivers/crypto/verify_compress

```
    默认设置为 '1'——校验所有压缩。

  - sync_mode

    选择用于等待每次压缩和解压缩操作完成的模式。

    iaa_crypto 实现的加密异步接口支持提供了一个满足该接口的实现，但采用的是同步方式——
    它填充并提交 IDXD 描述符，然后循环等待其完成再返回。目前这不是问题，因为所有现有
    调用者（例如 zswap）都会将任何异步被调用者包装在同步包装器中。

    iaa_crypto 驱动确实为能够利用它的调用者提供了真正的异步支持。在此模式下，它填充并
    提交 IDXD 描述符，然后立即以 -EINPROGRESS 返回。调用者随后可以自行轮询完成（这需要在
    调用者中包含特定代码，目前上游内核中没有任何实现），或者进入睡眠并等待发出完成信号的
    中断。后一种模式受到内核中现有用户（如通过同步包装器的 zswap）的支持。尽管受支持，但
    此模式比前述在 iaa_crypto 驱动中进行轮询的同步模式明显慢得多。

    可以通过将 'async_irq' 写入 sync_mode iaa_crypto 驱动属性来启用此模式：

      echo async_irq > /sys/bus/dsa/drivers/crypto/sync_mode

    无中断的异步模式（调用者必须轮询）可通过向其写入 'async' 来启用（请参阅注意事项）：

      echo async > /sys/bus/dsa/drivers/crypto/sync_mode

    在 iaa_crypto 驱动中进行轮询的模式可通过向其写入 'sync' 来启用：

      echo sync > /sys/bus/dsa/drivers/crypto/sync_mode

    默认模式为 'sync'。

    注意事项：由于 iaa_crypto 当前实现的唯一无中断异步轮询机制是通过前述的 'sync' 模式，
    向 '/sys/bus/dsa/drivers/crypto/sync_mode' 写入 'async' 会在内部启用 'sync' 模式。
    这是为了确保 iaa_crypto 的正确行为，直到 iaa_crypto 中启用真正的无中断异步轮询为止。

```


### IAA 默认配置


当加载 iaa_crypto 驱动时，每个 IAA 设备都有一个单独的
```
          mode              "dedicated"
          threshold         0
          size              Total WQ Size from WQCAP
          priority          10
          type              IDXD_WQT_KERNEL
          group             0
          name              "iaa_crypto"
          driver_name       "crypto"

```
这些设备及工作队列也已启用，因此该驱动无需任何额外配置即可使用。

```
          sync_mode         "sync"
          verify_compress   1

```
要更改设备/工作队列或驱动属性，必须先禁用已启用的设备和工作队列。为了让新配置应用到
deflate-iaa 加密算法，需要通过移除并重新插入 iaa_crypto 模块来重新注册。下面『用例』
小节中的 iaa_disable_script 可用于禁用默认配置。

## 统计信息


如果启用了可选的 debugfs 统计支持，IAA 加密
```
  # ls -al /sys/kernel/debug/iaa-crypto/
  total 0
  drwxr-xr-x  2 root root 0 Mar  3 07:55 .
  drwx------ 53 root root 0 Mar  3 07:55 ..
  -rw-r--r--  1 root root 0 Mar  3 07:55 global_stats
  -rw-r--r--  1 root root 0 Mar  3 07:55 stats_reset
  -rw-r--r--  1 root root 0 Mar  3 07:55 wq_stats

```
global_stats 文件显示在以下时间以来收集的一组全局统计信息：
```
  # cat global_stats
  global stats:
    total_comp_calls: 4300
    total_decomp_calls: 4164
    total_sw_decomp_calls: 0
    total_comp_bytes_out: 5993989
    total_decomp_bytes_in: 5993989
    total_completion_einval_errors: 0
    total_completion_timeout_errors: 0
    total_completion_comp_buf_overflow_errors: 136

```
wq_stats 文件显示每个工作队列的统计信息，为每个 iaa 设备及工作队列各提供一组：
```
  # cat wq_stats
  iaa device:
    id: 1
    n_wqs: 1
    comp_calls: 0
    comp_bytes: 0
    decomp_calls: 0
    decomp_bytes: 0
    wqs:
      name: iaa_crypto
      comp_calls: 0
      comp_bytes: 0
      decomp_calls: 0
      decomp_bytes: 0

  iaa device:
    id: 3
    n_wqs: 1
    comp_calls: 0
    comp_bytes: 0
    decomp_calls: 0
    decomp_bytes: 0
    wqs:
      name: iaa_crypto
      comp_calls: 0
      comp_bytes: 0
      decomp_calls: 0
      decomp_bytes: 0

  iaa device:
    id: 5
    n_wqs: 1
    comp_calls: 1360
    comp_bytes: 1999776
    decomp_calls: 0
    decomp_bytes: 0
    wqs:
      name: iaa_crypto
      comp_calls: 1360
      comp_bytes: 1999776
      decomp_calls: 0
      decomp_bytes: 0

  iaa device:
    id: 7
    n_wqs: 1
    comp_calls: 2940
    comp_bytes: 3994213
    decomp_calls: 4164
    decomp_bytes: 5993989
    wqs:
      name: iaa_crypto
      comp_calls: 2940
      comp_bytes: 3994213
      decomp_calls: 4164
      decomp_bytes: 5993989
    ...

```
写入 'stats_reset' 会重置所有统计信息，包括
```
  # echo 1 > stats_reset
  # cat wq_stats
    global stats:
    total_comp_calls: 0
    total_decomp_calls: 0
    total_comp_bytes_out: 0
    total_decomp_bytes_in: 0
    total_completion_einval_errors: 0
    total_completion_timeout_errors: 0
    total_completion_comp_buf_overflow_errors: 0
    ...


```


## 用例


### 简单的 zswap 测试


在本示例中，内核应按照上文所述专用模式选项进行配置，并且 zswap 应通过以下方式启用：
```
  CONFIG_ZSWAP=y

```
这是一个简单的测试，使用 iaa_compress 作为交换（zswap）设备的压缩器。它设置 zswap
设备，然后使用下面列出的 memory_memadvise 程序强制换出和换入指定数量的页，演示压缩
和解压缩。

zswap 测试期望系统上每个 IAA 设备的工作队列都被正确配置为内核工作队列，且工作队列
driver_name 为 "crypto"。

```
  modprobe iaa_crypto

```
如果 IAA 设备和工作队列之前未被禁用和重新配置，则应当处于默认配置状态，无需进一步的
IAA 配置。有关默认配置的详细信息，请参见下文的 iaa_default_config。

如果默认配置已就绪，你应当看到 iaa
```
  # cat /sys/bus/dsa/devices/iax1/state
  enabled
  # cat /sys/bus/dsa/devices/iax1/wq1.0/state
  enabled

```
为了演示后续步骤按预期工作，这些
```
  # echo -n 'module iaa_crypto +p' > /sys/kernel/debug/dynamic_debug/control
  # echo -n 'module idxd +p' > /sys/kernel/debug/dynamic_debug/control

```
```
  # echo 0 > /sys/module/zswap/parameters/enabled
  # echo 50 > /sys/module/zswap/parameters/max_pool_percent
  # echo deflate-iaa > /sys/module/zswap/parameters/compressor
  # echo 1 > /sys/module/zswap/parameters/enabled
  # echo 100 > /proc/sys/vm/swappiness
  # echo never > /sys/kernel/mm/transparent_hugepage/enabled
  # echo 1 > /proc/sys/vm/overcommit_memory

```
现在你可以运行想要测量的 zswap 工作负载了。例如，使用下面的 memory_memadvise 代码，
以下命令
```
  ./memory_madvise 100

  Allocating 100 pages to swap in/out
  Swapping out 100 pages
  Swapping in 100 pages
  Swapped out and in 100 pages

```
```
  [  404.202972] idxd 0000:e7:02.0: iaa_comp_acompress: dma_map_sg, src_addr 223925c000, nr_sgs 1, req->src 00000000ee7cb5e6, req->slen 4096, sg_dma_len(sg) 4096
  [  404.202973] idxd 0000:e7:02.0: iaa_comp_acompress: dma_map_sg, dst_addr 21dadf8000, nr_sgs 1, req->dst 000000008d6acea8, req->dlen 4096, sg_dma_len(sg) 8192
  [  404.202975] idxd 0000:e7:02.0: iaa_compress: desc->src1_addr 223925c000, desc->src1_size 4096, desc->dst_addr 21dadf8000, desc->max_dst_size 4096, desc->src2_addr 2203543000, desc->src2_size 1568
  [  404.202981] idxd 0000:e7:02.0: iaa_compress_verify: (verify) desc->src1_addr 21dadf8000, desc->src1_size 228, desc->dst_addr 223925c000, desc->max_dst_size 4096, desc->src2_addr 0, desc->src2_size 0
  ...

```
既然基本功能已演示完毕，可以清除默认值并替换为不同的配置。为此，
```
  # echo lzo > /sys/module/zswap/parameters/compressor
  # swapoff -a
  # echo 0 > /sys/module/zswap/parameters/accept_threshold_percent
  # echo 0 > /sys/module/zswap/parameters/max_pool_percent
  # echo 0 > /sys/module/zswap/parameters/enabled
  # echo 0 > /sys/module/zswap/parameters/enabled

```
然后运行下面『用例』小节中的 iaa_disable_script 来禁用默认配置。

```
  # swapon -a

```
完成以上所有步骤后，可以根据需要重新配置并启用 IAA 设备以进行进一步测试。下面是一个
示例。

zswap 测试期望系统上每个 IAA 设备的工作队列都被正确配置为内核工作队列，且工作队列
driver_name 为 "crypto"。

```
  #!/bin/bash

  echo "IAA devices:"
  lspci -d:0cfe
  echo "# IAA devices:"
  lspci -d:0cfe | wc -l

  #
  # count iaa instances
  #
  iaa_dev_id="0cfe"
  num_iaa=$(lspci -d:${iaa_dev_id} | wc -l)
  echo "Found ${num_iaa} IAA instances"

  #
  # disable iaa wqs and devices
  #
  echo "Disable IAA"

  for ((i = 1; i < ${num_iaa} * 2; i += 2)); do
      echo disable wq iax${i}/wq${i}.0
      accel-config disable-wq iax${i}/wq${i}.0
      echo disable iaa iax${i}
      accel-config disable-device iax${i}
  done

  echo "End Disable IAA"

  echo "Reload iaa_crypto module"

  rmmod iaa_crypto
  modprobe iaa_crypto

  echo "End Reload iaa_crypto module"

  #
  # configure iaa wqs and devices
  #
  echo "Configure IAA"
  for ((i = 1; i < ${num_iaa} * 2; i += 2)); do
      accel-config config-wq --group-id=0 --mode=dedicated --wq-size=128 --priority=10 --type=kernel --name="iaa_crypto" --driver-name="crypto" iax${i}/wq${i}.0
      accel-config config-engine iax${i}/engine${i}.0 --group-id=0
  done

  echo "End Configure IAA"

  #
  # enable iaa wqs and devices
  #
  echo "Enable IAA"

  for ((i = 1; i < ${num_iaa} * 2; i += 2)); do
      echo enable iaa iax${i}
      accel-config enable-device iax${i}
      echo enable wq iax${i}/wq${i}.0
      accel-config enable-wq iax${i}/wq${i}.0
  done

  echo "End Enable IAA"

```
当工作队列绑定到 iaa_crypto 驱动时，如果你已启用调试输出（echo -n 'module iaa_crypto +p' >），
你应当在 dmesg 输出中看到类似以下内容：
```
  [   60.752344] idxd 0000:f6:02.0: add_iaa_wq: added wq 000000004068d14d to iaa 00000000c9585ba2, n_wq 1
  [   60.752346] iaa_crypto: rebalance_wq_table: nr_nodes=2, nr_cpus 160, nr_iaa 8, cpus_per_iaa 20
  [   60.752347] iaa_crypto: rebalance_wq_table: iaa=0
  [   60.752349] idxd 0000:6a:02.0: request_iaa_wq: getting wq from iaa_device 0000000042d7bc52 (0)
  [   60.752350] idxd 0000:6a:02.0: request_iaa_wq: returning unused wq 00000000c8bb4452 (0) from iaa device 0000000042d7bc52 (0)
  [   60.752352] iaa_crypto: rebalance_wq_table: assigned wq for cpu=0, node=0 = wq 00000000c8bb4452
  [   60.752354] iaa_crypto: rebalance_wq_table: iaa=0
  [   60.752355] idxd 0000:6a:02.0: request_iaa_wq: getting wq from iaa_device 0000000042d7bc52 (0)
  [   60.752356] idxd 0000:6a:02.0: request_iaa_wq: returning unused wq 00000000c8bb4452 (0) from iaa device 0000000042d7bc52 (0)
  [   60.752358] iaa_crypto: rebalance_wq_table: assigned wq for cpu=1, node=0 = wq 00000000c8bb4452
  [   60.752359] iaa_crypto: rebalance_wq_table: iaa=0
  [   60.752360] idxd 0000:6a:02.0: request_iaa_wq: getting wq from iaa_device 0000000042d7bc52 (0)
  [   60.752361] idxd 0000:6a:02.0: request_iaa_wq: returning unused wq 00000000c8bb4452 (0) from iaa device 0000000042d7bc52 (0)
  [   60.752362] iaa_crypto: rebalance_wq_table: assigned wq for cpu=2, node=0 = wq 00000000c8bb4452
  [   60.752364] iaa_crypto: rebalance_wq_table: iaa=0
  .
  .
  .

```
一旦工作队列和设备已启用，IAA 加密算法即被启用并可用。当 IAA 加密算法成功启用后，
你应当看到如下 dmesg
```
  [   64.893759] iaa_crypto: iaa_crypto_enable: iaa_crypto now ENABLED

```
现在运行以下 zswap 专用设置命令，使 zswap 使用
```
  echo 0 > /sys/module/zswap/parameters/enabled
  echo 50 > /sys/module/zswap/parameters/max_pool_percent
  echo deflate-iaa > /sys/module/zswap/parameters/compressor
  echo 1 > /sys/module/zswap/parameters/enabled

  echo 100 > /proc/sys/vm/swappiness
  echo never > /sys/kernel/mm/transparent_hugepage/enabled
  echo 1 > /proc/sys/vm/overcommit_memory

```
最后，现在你可以运行想要测量的 zswap 工作负载了。例如，使用下面的代码，以下命令将
换入和
```
  ./memory_madvise 100

  Allocating 100 pages to swap in/out
  Swapping out 100 pages
  Swapping in 100 pages
  Swapped out and in 100 pages

```
如果你已启用调试输出（echo -n 'module iaa_crypto +p' >），你应当在 dmesg 输出中看到
类似以下内容：
```
  [  404.202972] idxd 0000:e7:02.0: iaa_comp_acompress: dma_map_sg, src_addr 223925c000, nr_sgs 1, req->src 00000000ee7cb5e6, req->slen 4096, sg_dma_len(sg) 4096
  [  404.202973] idxd 0000:e7:02.0: iaa_comp_acompress: dma_map_sg, dst_addr 21dadf8000, nr_sgs 1, req->dst 000000008d6acea8, req->dlen 4096, sg_dma_len(sg) 8192
  [  404.202975] idxd 0000:e7:02.0: iaa_compress: desc->src1_addr 223925c000, desc->src1_size 4096, desc->dst_addr 21dadf8000, desc->max_dst_size 4096, desc->src2_addr 2203543000, desc->src2_size 1568
  [  404.202981] idxd 0000:e7:02.0: iaa_compress_verify: (verify) desc->src1_addr 21dadf8000, desc->src1_size 228, desc->dst_addr 223925c000, desc->max_dst_size 4096, desc->src2_addr 0, desc->src2_size 0
  [  409.203227] idxd 0000:e7:02.0: iaa_comp_adecompress: dma_map_sg, src_addr 21ddd8b100, nr_sgs 1, req->src 0000000084adab64, req->slen 228, sg_dma_len(sg) 228
  [  409.203235] idxd 0000:e7:02.0: iaa_comp_adecompress: dma_map_sg, dst_addr 21ee3dc000, nr_sgs 1, req->dst 000000004e2990d0, req->dlen 4096, sg_dma_len(sg) 4096
  [  409.203239] idxd 0000:e7:02.0: iaa_decompress: desc->src1_addr 21ddd8b100, desc->src1_size 228, desc->dst_addr 21ee3dc000, desc->max_dst_size 4096, desc->src2_addr 0, desc->src2_size 0
  [  409.203254] idxd 0000:e7:02.0: iaa_comp_adecompress: dma_map_sg, src_addr 21ddd8b100, nr_sgs 1, req->src 0000000084adab64, req->slen 228, sg_dma_len(sg) 228
  [  409.203256] idxd 0000:e7:02.0: iaa_comp_adecompress: dma_map_sg, dst_addr 21f1551000, nr_sgs 1, req->dst 000000004e2990d0, req->dlen 4096, sg_dma_len(sg) 4096
  [  409.203257] idxd 0000:e7:02.0: iaa_decompress: desc->src1_addr 21ddd8b100, desc->src1_size 228, desc->dst_addr 21f1551000, desc->max_dst_size 4096, desc->src2_addr 0, desc->src2_size 0

```
为了注销 IAA 加密算法并使用不同参数注册新算法，应当停止当前算法的任何使用者，并禁用
IAA 工作队列和设备。

对于 zswap，需要将 IAA 加密算法移出压缩器并关闭交换（以移除对
```
  echo lzo > /sys/module/zswap/parameters/compressor
  swapoff -a

  echo 0 > /sys/module/zswap/parameters/accept_threshold_percent
  echo 0 > /sys/module/zswap/parameters/max_pool_percent
  echo 0 > /sys/module/zswap/parameters/enabled

```
一旦 zswap 被禁用且不再使用 iaa_crypto，就可以禁用 IAA 工作队列和设备。


### IAA 禁用脚本


```
  #!/bin/bash

  echo "IAA devices:"
  lspci -d:0cfe
  echo "# IAA devices:"
  lspci -d:0cfe | wc -l

  #
  # count iaa instances
  #
  iaa_dev_id="0cfe"
  num_iaa=$(lspci -d:${iaa_dev_id} | wc -l)
  echo "Found ${num_iaa} IAA instances"

  #
  # disable iaa wqs and devices
  #
  echo "Disable IAA"

  for ((i = 1; i < ${num_iaa} * 2; i += 2)); do
      echo disable wq iax${i}/wq${i}.0
      accel-config disable-wq iax${i}/wq${i}.0
      echo disable iaa iax${i}
      accel-config disable-device iax${i}
  done

  echo "End Disable IAA"

```
最后，此时可以移除 iaa_crypto 模块，这
```
  rmmod iaa_crypto


```
```
  #include <stdio.h>
  #include <stdlib.h>
  #include <string.h>
  #include <unistd.h>
  #include <sys/mman.h>
  #include <linux/mman.h>

  #ifndef MADV_PAGEOUT
  #define MADV_PAGEOUT    21      /* force pages out immediately */
  #endif

  #define PG_SZ           4096

  int main(int argc, char **argv)
  {
        int i, nr_pages = 1;
        int64_t *dump_ptr;
        char *addr, *a;
        int loop = 1;

        if (argc > 1)
                nr_pages = atoi(argv[1]);

        printf("Allocating %d pages to swap in/out\n", nr_pages);

        /* allocate pages */
        addr = mmap(NULL, nr_pages * PG_SZ, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_ANONYMOUS, -1, 0);
        *addr = 1;

        /* initialize data in page to all '*' chars */
        memset(addr, '*', nr_pages * PG_SZ);

         printf("Swapping out %d pages\n", nr_pages);

        /* Tell kernel to swap it out */
        madvise(addr, nr_pages * PG_SZ, MADV_PAGEOUT);

        while (loop > 0) {
                /* Wait for swap out to finish */
                sleep(5);

                a = addr;

                printf("Swapping in %d pages\n", nr_pages);

                /* Access the page ... this will swap it back in again */
                for (i = 0; i < nr_pages; i++) {
                        if (a[0] != '*') {
                                printf("Bad data from decompress!!!!!\n");

                                dump_ptr = (int64_t *)a;
                                 for (int j = 0; j < 100; j++) {
                                        printf("  page %d data: %#llx\n", i, *dump_ptr);
                                        dump_ptr++;
                                }
                        }

                        a += PG_SZ;
                }

                loop --;
        }

       printf("Swapped out and in %d pages\n", nr_pages);

```
## 附录



### IAA sysfs 配置接口


以下是对 IAA sysfs 接口的描述，正如主文档中所提及，只有在你确切知道自己在做什么时才
应使用它。即便如此，也没有充分的理由直接使用它，因为 accel-config 能够完成 sysfs 接口
可以做的一切，事实上 accel-config 在底层正是基于它实现的。

『IAA 配置路径』为 /sys/bus/dsa/devices，其中包含代表每个 IAA 设备、工作队列、引擎和
组的子目录。注意在 sysfs 接口中，IAA 设备实际上以 iax 命名，例如 iax1、iax3 等。（注意
IAA 设备是奇数编号的设备；偶数编号的设备是 DSA 设备，对于 IAA 可以忽略。）

『IAA 设备绑定路径』为 /sys/bus/dsa/drivers/idxd/bind，是写入以启用 IAA 设备的文件。

『IAA 工作队列绑定路径』为 /sys/bus/dsa/drivers/crypto/bind，是写入以启用 IAA 工作队列
的文件。

类似地，/sys/bus/dsa/drivers/idxd/unbind 和 /sys/bus/dsa/drivers/crypto/unbind 用于
禁用 IAA 设备和工作队列。

设置 IAA 设备和工作队列所需的基本命令序列如下：

```
最后，此时可以移除 iaa_crypto 模块，这
```
  rmmod iaa_crypto


```
```
  #include <stdio.h>
  #include <stdlib.h>
  #include <string.h>
  #include <unistd.h>
  #include <sys/mman.h>
  #include <linux/mman.h>

  #ifndef MADV_PAGEOUT
  #define MADV_PAGEOUT    21      /* force pages out immediately */
  #endif

  #define PG_SZ           4096

  int main(int argc, char **argv)
  {
        int i, nr_pages = 1;
        int64_t *dump_ptr;
        char *addr, *a;
        int loop = 1;

        if (argc > 1)
                nr_pages = atoi(argv[1]);

        printf("Allocating %d pages to swap in/out
", nr_pages);

        /* allocate pages */
        addr = mmap(NULL, nr_pages * PG_SZ, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_ANONYMOUS, -1, 0);
        *addr = 1;

        /* initialize data in page to all '*' chars */
        memset(addr, '*', nr_pages * PG_SZ);

         printf("Swapping out %d pages
", nr_pages);

        /* Tell kernel to swap it out */
        madvise(addr, nr_pages * PG_SZ, MADV_PAGEOUT);

        while (loop > 0) {
                /* Wait for swap out to finish */
                sleep(5);

                a = addr;

                printf("Swapping in %d pages
", nr_pages);

                /* Access the page ... this will swap it back in again */
                for (i = 0; i < nr_pages; i++) {
                        if (a[0] != '*') {
                                printf("Bad data from decompress!!!!!
");

                                dump_ptr = (int64_t *)a;
                                 for (int j = 0; j < 100; j++) {
                                        printf("  page %d data: %#llx
", i, *dump_ptr);
                                        dump_ptr++;
                                }
                        }

                        a += PG_SZ;
                }

                loop --;
        }

       printf("Swapped out and in %d pages
", nr_pages);

```
## 附录



### IAA sysfs 配置接口


以下是对 IAA sysfs 接口的描述，正如主文档中所提及，只有在你确切知道自己在做什么时才
应使用它。即便如此，也没有充分的理由直接使用它，因为 accel-config 能够完成 sysfs 接口
可以做的一切，事实上 accel-config 在底层正是基于它实现的。

『IAA 配置路径』为 /sys/bus/dsa/devices，其中包含代表每个 IAA 设备、工作队列、引擎和
组的子目录。注意在 sysfs 接口中，IAA 设备实际上以 iax 命名，例如 iax1、iax3 等。（注意
IAA 设备是奇数编号的设备；偶数编号的设备是 DSA 设备，对于 IAA 可以忽略。）

『IAA 设备绑定路径』为 /sys/bus/dsa/drivers/idxd/bind，是写入以启用 IAA 设备的文件。

『IAA 工作队列绑定路径』为 /sys/bus/dsa/drivers/crypto/bind，是写入以启用 IAA 工作队列
的文件。

类似地，/sys/bus/dsa/drivers/idxd/unbind 和 /sys/bus/dsa/drivers/crypto/unbind 用于
禁用 IAA 设备和工作队列。

设置 IAA 设备和工作队列所需的基本命令序列如下：

```
  1) Disable any workqueues enabled on the device.  For example to
     disable workques 0 and 1 on IAA device 3::

       # echo wq3.0 > /sys/bus/dsa/drivers/crypto/unbind
       # echo wq3.1 > /sys/bus/dsa/drivers/crypto/unbind

  2) Disable the device. For example to disable IAA device 3::

       # echo iax3 > /sys/bus/dsa/drivers/idxd/unbind

  3) configure the desired workqueues.  For example, to configure
     workqueue 3 on IAA device 3::

       # echo dedicated > /sys/bus/dsa/devices/iax3/wq3.3/mode
       # echo 128 > /sys/bus/dsa/devices/iax3/wq3.3/size
       # echo 0 > /sys/bus/dsa/devices/iax3/wq3.3/group_id
       # echo 10 > /sys/bus/dsa/devices/iax3/wq3.3/priority
       # echo "kernel" > /sys/bus/dsa/devices/iax3/wq3.3/type
       # echo "iaa_crypto" > /sys/bus/dsa/devices/iax3/wq3.3/name
       # echo "crypto" > /sys/bus/dsa/devices/iax3/wq3.3/driver_name

  4) Enable the device. For example to enable IAA device 3::

       # echo iax3 > /sys/bus/dsa/drivers/idxd/bind

  5) Enable the desired workqueues on the device.  For example to
     enable workques 0 and 1 on IAA device 3::

       # echo wq3.0 > /sys/bus/dsa/drivers/crypto/bind
       # echo wq3.1 > /sys/bus/dsa/drivers/crypto/bind

```
