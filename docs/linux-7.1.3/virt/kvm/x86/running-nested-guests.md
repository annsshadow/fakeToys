
## 使用 KVM 运行嵌套客户

嵌套客户机（nested guest）是在另一个客户机内部运行客户机的能力（它可以是基KVM 的，也可以是不同hypervisor）。最直接的例子是一KVM 客户机进而运行在一KVM 客户机之上（其余部分
```
              .----------------.  .----------------.
              |                |  |                |
              |      L2        |  |      L2        |
              | (Nested Guest) |  | (Nested Guest) |
              |                |  |                |
              |----------------'--'----------------|
              |                                    |
              |       L1 (Guest Hypervisor)        |
              |          KVM (/dev/kvm)            |
              |                                    |
      .------------------------------------------------------.
      |                 L0 (Host Hypervisor)                 |
      |                    KVM (/dev/kvm)                    |
      |------------------------------------------------------|
      |        Hardware (with virtualization extensions)     |
      '------------------------------------------------------'
```

术语
- L0 0 层；裸金属主机，运行 KVM

- L1 1 层客户机；运行在 L0 上的 VM；也称为“客户机 hypervisor”，因为它自身能够运KVM
- L2 2 层客户机；运行在 L1 上的 VM，这就是“嵌套客户机
          s390x、ppc64 以及其他架构的嵌套设计可能有所不同
          例如，s390x 在裸金属上始终运行一LPAR（LogicalPARtition）hypervisor，这增加了另一层，从而在嵌套设置中至少产生四个层级——L0（裸金属，运LPAR hypervisor）、L1（主hypervisor）、L2（客户机 hypervisor）、L3（嵌套客户机）
          本文档将坚持对所有架构使用三层术语（L0、L1 L2）；并且将主要关x86
### 使用场景


嵌套 KVM 在多种场景下很有用，仅举几例
- 作为开发者，你想在不同的操作系统（OS）上测试你的软件。与其从云服务提供商租用多个 VM，使用嵌KVM 可以让你租用足够大的“客户机 hypervisor”（1 层客户机）。这反过来让你能够创建运行不OS 的多个嵌套客户机（第 2 层客户机），在其上开发和测试你的软件
- “客户机 hypervisor”及其嵌套客户机的实时迁移，用于负载均衡、灾难恢复等
- VM 镜像创建工具（例`virt-install` 等）经常运行自己VM，用户期望这些工具能VM 内部正常工作
- 某些 OS 在内部使用虚拟化来提升安全性（例如让应用程序在隔离中安全运行）
### 启用“nested”（x86

Linux 内核 v4.20 起，对于 Intel AMD，`nested` KVM 参数默认启用。（不过你的 Linux 发行版可能会覆盖此默认值。）

如果你运行的是早v4.19 Linux 内核，要启用嵌套，请`nested` KVM 模块参数设为 `Y` `1`。要让此设置在重启后仍然生效，你可以将其添加到配置文件中，如下所示：

1. 在裸金属主机（L0）上，列出内核模块并确保

```
    $ lsmod | grep -i kvm
    kvm_intel             133627  0
    kvm                   435079  1 kvm_intel
```

```
    $ modinfo kvm_intel | grep -i nested
    parm:           nested:bool
```

3. 为使嵌套 KVM 配置在重启后仍然生效，将以下内容放入 `/etc/modprobed/kvm_intel.conf`（如果文件不存在则创建它
```
    $ cat /etc/modprobe.d/kvm_intel.conf
    options kvm-intel nested=y
```

```
    $ sudo rmmod kvm-intel
    $ sudo modprobe kvm-intel
```

```
    $ cat /sys/module/kvm_intel/parameters/nested
    Y
```

对于 AMD 主机，过程与上述相同，只是模块名`kvm-amd`
### 额外的嵌套相关内核参数（x86

如果你的硬件足够先进（Intel Haswell 处理器或更高，具有较新的硬件虚拟化扩展），以下附加特性也会默认启用：“Shadow VMCS（Virtual Machine Control Structure）”、裸金属上的 APIC 虚拟
```
    $ cat /sys/module/kvm_intel/parameters/enable_shadow_vmcs
    Y

    $ cat /sys/module/kvm_intel/parameters/enable_apicv
    Y

    $ cat /sys/module/kvm_intel/parameters/ept
    Y
```

          确保上述特性已启用（尤其是 `enable_shadow_vmcs` `ept`）
### 启动一个嵌套客户机（x86

一旦你的裸金属主机（L0）配置为支持嵌套，你应该

```
    $ qemu-kvm -cpu host [...]
```

上述将主CPU 的能力原样透传给客户机，或者为了更好的实时迁移兼容性，使用命名CPU

```
    $ qemu-kvm -cpu Haswell-noTSX-IBRS,vmx=on
```

那么客户hypervisor 随后就能够运行带加速的 KVM 嵌套客户机
### 启用“nested”（s390x

1. 在主hypervisor（L0）上，使`nested` 参数启用

```
    $ rmmod kvm
    $ modprobe kvm nested=1
```

          由于 `nested` 参数——即为了能够启用 `nested`，“hpage”参*必须**被禁用
2. 客户hypervisor（L1）必须提`sie` CPU 特性——在 QEMU 中，这可以通过“host passthrough”（通过命令`-cpu host`）完成
```
    $ modprobe kvm
```

### 嵌套 KVM 的实时迁

将一个内部带*运行*嵌套客户机的 L1 客户机迁移到另一台裸金属主机，自 Linux 内核 5.3 QEMU 4.2.0 起对 Intel x86 系统可用，对 s390x 甚至在更早版本也可用
AMD 系统上，一L1 客户机启动了 L2 客户机，L2 客户机关闭之前，不应再迁移或保存（指 QEMU 文档中的“savevm“loadvm”）L1 客户机。在 L2 客户机运行时尝试迁移或保存再加载 L1 客户机将导致未定义行为。你可能会在 `dmesg` 中看`kernel BUG!` 条目、内核“oops”，或者直接的内核 panic。这样迁移或加载L1 客户机不能再被视为稳定或安全，必须重启。仅配置为支持嵌套但并未实际运行 L2 客户机的 L1 客户机，即使AMD 系统上也预期能正常工作，但一旦启动客户机就可能会失败
迁移 L2 客户机总是预期会成功，因此以下所有场景即使在 AMD 系统上也应可行：

- 将嵌套客户机（L2）迁移到**同一**台裸金属主机上的另一L1 客户机
- 将嵌套客户机（L2）迁移到**不同**台裸金属主机上的另一L1 客户机
- 将嵌套客户机（L2）迁移到裸金属主机
### 从嵌套设置中报告缺陷


调试“嵌套”问题可能涉及在 L0、L1 L2 之间筛选日志文件；这可能导致缺陷报告者与修复者之间繁琐的来回沟通
- 说明你处于“嵌套”设置中。如果你在运行任何形式的“嵌套”，请明说。遗憾的是，这一点需要明确指出，因为在报告缺陷时，人们往往忘记甚至**提及**他们正在使用嵌套虚拟化
- 确保你确实是KVM 上运KVM。有时人们没有为他们的客户机 hypervisor（L1）启KVM，导致他们以纯模拟或 QEMU 所谓的“TCG”运行，但他们以为自己在运行嵌套 KVM。从而将“嵌Virt”（也可能意味着 KVM 上的 QEMU）与“嵌KVM”（KVM 上的 KVM）混淆
#### 需要收集的信息（通用

以下并非详尽清单，但是一个非常好的起点：

  - L0 的内核、libvirt QEMU 版本

  - L1 的内核、libvirt QEMU 版本

  - L1 QEMU 命令行——使libvirt 时，你会在这里找到它：`/var/log/libvirt/qemu/instance.log`

  - L2 QEMU 命令行——同上，使用 libvirt 时，获取完整的由 libvirt 生成QEMU 命令
  - 来自 L0 `cat /sys/cpuinfo`

  - 来自 L1 `cat /sys/cpuinfo`

  - 来自 L0 `lscpu`

  - 来自 L1 `lscpu`

  - 来自 L0 的完`dmesg` 输出

  - 来自 L1 的完`dmesg` 输出

#### x86 特定的待收集信息


下面的两条命`x86info` `dmidecode` 在大多数 Linux 发行版中都应可用，名称相同：

  - 来自 L0 的：`x86info -a` 的输
  - 来自 L1 的：`x86info -a` 的输
  - 来自 L0 的：`dmidecode` 的输
  - 来自 L1 的：`dmidecode` 的输
#### s390x 特定的待收集信息


除了前面提到的通用细节外，还建议收集以下内容：

  - 来自 L1 `/proc/sysinfo`；这也将包含来自 L0 的信