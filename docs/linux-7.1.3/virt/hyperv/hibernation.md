
## Hibernating Guest VMs

客户机虚拟机休眠

### Background

背景

Linux 支持让自己休眠以节省电力。休眠有时也被称为挂起到磁盘（suspend-to-disk），
因为它会把内存镜像写入磁盘，并使硬件进入最低可能的电源状态。从休眠恢复时，硬件
重新启动，内存镜像从磁盘恢复，从而能够从离开的地方继续执行。详见
Documentation/admin-guide/pm/sleep-states.rst 中的"Hibernation"小节。

休眠通常在单用户设备上使用，例如个人笔记本电脑。例如，合上盖子时笔记本进入
休眠，再次打开盖子时恢复。休眠与恢复发生在同一硬件上，而编排休眠步骤的 Linux
内核代码假定在休眠状态下硬件配置不会发生变化。

在 Linux 中，可以通过向 /sys/power/state 写入 "disk"，或带上相应参数调用 reboot
系统调用来发起休眠。此功能可能由用户空间命令（如直接在命令行运行的，或响应
笔记本合盖等事件而运行的 "systemctl hibernate"）所封装。

### Considerations for Guest VM Hibernation

客户机虚拟机休眠的注意事项

Hyper-V 上的 Linux 客户机也可以被休眠，此时硬件就是 Hyper-V 提供给客户机虚拟机的
虚拟硬件。只有目标客户机虚拟机被休眠，而其他客户机虚拟机以及底层的 Hyper-V 主机
继续正常运行。虽然底层的 Windows Hyper-V 及其所运行的物理硬件也可以使用 Windows
主机上的休眠功能被休眠，但主机休眠及其对客户机虚拟机的影响不在本文档范畴之内。

恢复一个休眠的客户机虚拟机可能比在物理硬件上更具挑战性，因为虚拟机让在休眠与
恢复之间改变硬件配置变得非常容易。即使恢复是在执行休眠的那个同一台虚拟机上进行，
内存大小也可能被改变，或者虚拟 NIC 或 SCSI 控制器可能被添加或移除。分配给虚拟机的
虚拟 PCI 设备也可能被添加或移除。大多数此类更改会导致恢复步骤失败，不过添加一个新的
虚拟 NIC、SCSI 控制器或 vPCI 设备应当可以正常工作。

由于休眠的虚拟机其磁盘可以被移动到另一个新创建的、虚拟硬件配置其他方面相同的
虚拟机上，还可能产生额外的复杂性。虽然在这样的迁移之后能够从休眠恢复是可取的，
但仍存在挑战。有关此场景及其限制的详情，请参阅下面的"Resuming on a Different VM"
小节。

Hyper-V 还提供了将虚拟机从一台 Hyper-V 主机迁移到另一台主机的方法。Hyper-V 尝试
通过 VM Configuration Versions 来确保处理器型号与 Hyper-V 版本的兼容性，并阻止
向不兼容的主机迁移。Linux 通过在启动时检测它们来适应主机和处理器的差异，但在从
休眠镜像恢复执行时并不会进行此类检测。如果虚拟机在一台主机上休眠，然后在具有
不同处理器型号或 Hyper-V 版本的主机上恢复，休眠镜像中记录的设置可能与新主机
不匹配。由于 Linux 在恢复休眠镜像时不检测此类不匹配，可能导致未定义行为和故障。

### Enabling Guest VM Hibernation

启用客户机虚拟机休眠

Hyper-V 客户机虚拟机的休眠默认是禁用的，因为休眠与由 Hyper-V balloon 驱动提供的
内存热添加（hot-add）不兼容。如果使用了热添加且虚拟机休眠，它会以比启动时所拥有的
更多内存休眠。但当虚拟机从休眠恢复时，Hyper-V 只给虚拟机最初分配的内存，内存大小
的不匹配会导致恢复失败。

要为客户机虚拟机启用休眠，Hyper-V 管理员必须在 Hyper-V 提供给客户机虚拟机的 ACPI
配置中启用 ACPI 虚拟 S4 睡眠状态。这种启用是通过修改虚拟机的某个 WMI 属性来完成的，
具体步骤不在本文档范畴之内，但可在网上找到。启用被视为管理员优先在虚拟机中支持
Linux 休眠、而非热添加的标志，因此 Linux 中的 Hyper-V balloon 驱动会禁用热添加。
如果 /sys/power/disk 的内容包含 "platform" 作为选项，即表明已启用。该启用状态也可在
/sys/bus/vmbus/hibernation 中看到。参见函数 hv_is_hibernation_supported()。

Linux 在 x86 上支持 ACPI 睡眠状态，但在 arm64 上不支持。因此 Hyper-V 上的 Linux
客户机虚拟机休眠在 arm64 上不可用。

### Initiating Guest VM Hibernation

发起客户机虚拟机休眠

客户机虚拟机可以使用标准的 Linux 方法（向 /sys/power/state 写入 "disk" 或调用
reboot 系统调用）自行发起休眠。作为额外的一层，Hyper-V 上的 Linux 客户机支持
"Shutdown" 集成服务，Hyper-V 管理员可以通过虚拟机外部的命令告知 Linux 虚拟机
进行休眠。该命令会向 Linux 中的 Hyper-V shutdown 驱动生成请求，后者发送 uevent
"EVENT=hibernate"。参见内核函数 shutdown_onchannelcallback() 和
send_hibernate_uevent()。必须在虚拟机中提供一个处理此事件并发起休眠的 udev 规则。

### Handling VMBus Devices During Hibernation & Resume

休眠与恢复期间对 VMBus 设备的处理

VMBus 总线驱动以及各个 VMBus 设备驱动，实现了在 Linux 编排休眠以及从休眠恢复时
被调用的 suspend 和 resume 函数。整体思路是保留主 VMBus 通道及其关联的 Linux
设备（如 SCSI 控制器等）的数据结构原地不动，以便它们被捕获到休眠镜像中。这种方法
允许与设备相关的任何状态在休眠/恢复之间持久化。当虚拟机恢复时，设备由 Hyper-V
重新提供，并连接到休眠镜像中已经存在的数据结构。

VMBus 设备通过类（class）和实例（instance）GUID 来标识。（参见
Documentation/virt/hyperv/vmbus.rst 中的"VMBus device creation/deletion"小节。）
从休眠恢复时，resume 函数期望 Hyper-V 提供的设备具有与休眠时存在的设备相同的
类/实例 GUID。具有相同类/实例 GUID 使得被提供的设备能够匹配到此刻已恢复的休眠镜像
内存中的主 VMBus 通道数据结构。如果有任何被提供的设备与已经存在的主 VMBus 通道
数据结构不匹配，它们会作为新添加的设备被正常处理。如果休眠镜像中存在的某个主
VMBus 通道没有被恢复后的虚拟机中提供的某个设备匹配上，恢复序列会等待 10 秒，
然后继续。但那个未被匹配的设备很可能会在恢复后的虚拟机中导致错误。

当恢复已有的主 VMBus 通道时，新提供的 relid 可能不同，因为 relid 在每次虚拟机
启动时都可能会改变，即便虚拟机配置没有变化。VMBus 总线驱动的 resume 函数会匹配
类/实例 GUID，并在 relid 发生变化时更新它们。

VMBus 子通道不会被持久化到休眠镜像中。每个 VMBus 设备驱动的 suspend 函数必须在
休眠之前关闭任何子通道。关闭子通道会导致 Hyper-V 发送 RESCIND_CHANNELOFFER 消息，
Linux 通过释放通道数据结构来处理它，从而移除子通道的所有痕迹。相比之下，主通道
被标记为已关闭且其环形缓冲区被释放，但 Hyper-V 不会发送 rescind 消息，因此通道
数据结构继续存在。恢复时，设备驱动的 resume 函数重新分配环形缓冲区并重新打开已有的
通道。然后它与 Hyper-V 通信，从头开始重新打开子通道。

Hyper-V 套接字的 Linux 端在休眠时被强制关闭。客户机无法强制关闭套接字的主机端，
但主机端在主机端的任何操作都会产生一个错误。

VMBus 设备对 "freeze" 和 "poweroff" 阶段使用相同的 suspend 函数，对 "thaw" 和
"restore" 阶段使用相同的 resume 函数。各阶段的顺序参见
Documentation/driver-api/pm/devices.rst 中的"Entering Hibernation"小节。

### Detailed Hibernation Sequence

详细休眠序列

1. Linux 电源管理（PM）子系统通过冻结用户空间进程并分配用于保存休眠镜像的内存来
   为休眠做准备。
2. 作为 "freeze" 阶段的一部分，Linux PM 依次调用每个 VMBus 设备的 "suspend"
   函数。如上所述，该函数移除子通道，并使主通道处于已关闭状态。
3. Linux PM 调用 VMBus 总线的 "suspend" 函数，该函数关闭任何 Hyper-V 套接字通道，
   并卸载与 Hyper-V 主机的顶层 VMBus 连接。
4. Linux PM 禁用非启动 CPU，在先前分配的内存中创建休眠镜像，然后重新启用非启动
   CPU。休眠镜像包含已关闭的主通道的内存数据结构，但不包含子通道。
5. 作为 "thaw" 阶段的一部分，Linux PM 调用 VMBus 总线的 "resume" 函数，该函数
   重新建立顶层 VMBus 连接，并请求 Hyper-V 重新提供 VMBus 设备。随着为主通道提供
   offer，relid 会按前述方式更新。
6. Linux PM 调用每个 VMBus 设备的 "resume" 函数。每个设备重新打开它的主通道，
   并在适当时与 Hyper-V 通信以重新建立子通道。由于子通道在步骤 2 中已被完全移除，
   它们作为新通道被重新创建。
7. VMBus 设备现在再次工作，Linux PM 将休眠镜像从内存写入磁盘。
8. Linux PM 作为 "poweroff" 阶段的一部分重复上述步骤 2 和 3。VMBus 通道被关闭，
   顶层 VMBus 连接被卸载。
9. Linux PM 禁用非启动 CPU，然后进入 ACPI 睡眠状态 S4。休眠至此完成。

### Detailed Resume Sequence

详细恢复序列

1. 客户机虚拟机启动进入一个全新的 Linux OS 实例。在启动期间，建立顶层 VMBus 连接，
   并启用合成设备。这通过不涉及休眠的正常路径发生。
2. Linux PM 休眠代码读取交换空间以查找并将休眠镜像读入内存。如果没有休眠镜像，
   则此次启动成为一次正常启动。
3. 如果这是从休眠恢复，则使用 "freeze" 阶段来关闭 VMBus 设备并卸载正在运行的全新
   OS 实例中的顶层 VMBus 连接，就像休眠序列中的步骤 2 和 3 一样。
4. Linux PM 禁用非启动 CPU，并将控制权转移给读入的休眠镜像。在当下正在运行的
   休眠镜像中，非启动 CPU 被重新启动。
5. 作为 "resume" 阶段的一部分，Linux PM 重复休眠序列中的步骤 5 和 6。重新建立
   顶层 VMBus 连接，并接收 offer 并将其匹配到镜像中的主通道。更新 relid。VMBus
   设备 resume 函数重新打开主通道并重新创建子通道。
6. Linux PM 退出休眠恢复序列，虚拟机现在从休眠镜像正常运行。

### Key-Value Pair (KVP) Pseudo-Device Anomalies

键值对（KVP）伪设备异常

VMBus KVP 设备的行为与 Hyper-V 提供的其他伪设备不同。当 KVP 主通道关闭时，
Hyper-V 会发送一条 rescind 消息，导致该设备的所有痕迹被移除。但 Hyper-V 随后
重新提供该设备，导致它被新建。这种移除和重新创建发生在休眠的 "freeze" 阶段，因此
休眠镜像包含重新创建后的 KVP 设备。类似的行为也发生在仍处于全新 OS 实例中、恢复
序列的 "freeze" 阶段。但在两种情况下，顶层 VMBus 连接随后都会被卸载，从而导致
该设备在 Hyper-V 一侧被丢弃。因此不会造成危害，一切仍然正常工作。

### Virtual PCI devices

虚拟 PCI 设备

虚拟 PCI 设备是物理 PCI 设备，被直接映射到虚拟机的物理地址空间，以便虚拟机能够
直接与硬件交互。vPCI 设备包括通过 Hyper-V 所谓的 "Discrete Device Assignment"（DDA）
访问的设备，以及 SR-IOV NIC 虚拟功能（VF）设备。参见
Documentation/virt/hyperv/vpci.rst。

Hyper-V DDA 设备在建立顶层 VMBus 连接之后才提供给客户机虚拟机，就像 VMBus 合成
设备一样。它们被静态分配给虚拟机，其实例 GUID 除非 Hyper-V 管理员更改配置，否则
不会改变。DDA 设备在 Linux 中表现为既具有 VMBus 标识又具有 PCI 标识的虚拟 PCI
设备。因此，Linux 客户机休眠首先把 DDA 设备作为 VMBus 设备来处理，以管理 VMBus
通道。但随后它们也作为 PCI 设备，使用其原生 PCI 驱动实现的休眠函数来处理。

SR-IOV NIC VF 同样既具有 VMBus 标识又具有 PCI 标识，整体上与 DDA 设备的处理类似。
一个区别在于，VF 在虚拟机初始启动期间不会被提供给虚拟机。相反，VMBus 合成 NIC
驱动先开始运行，并告知 Hyper-V 它已准备好接受一个 VF，然后才发出 VF offer。然而，
VMBus 连接随后可能被卸载，然后在不重启虚拟机的情况下重新建立，如上面"详细休眠
序列"中的步骤 3 和 5 以及"详细恢复序列"中那样。在这种情况下，VF 很可能是在初始
启动期间成为虚拟机一部分的，因此当 VMBus 连接重新建立时，VF 是在重新建立的连接上
提供的，无需合成 NIC 驱动介入。

### UIO Devices

UIO 设备

一个 VMBus 设备可以通过 Hyper-V UIO 驱动（uio_hv_generic.c）暴露给用户空间，以便
用户空间驱动能够控制并操作该设备。然而，Hyper-V UIO 驱动不支持休眠所需的 suspend
和 resume 操作。如果一个 VMBus 设备被配置为使用 UIO 驱动，休眠虚拟机会失败，而
Linux 继续正常运行。Hyper-V UIO 驱动最常见的用途是 DPDK 网络，但也存在其他用途。

### Resuming on a Different VM

在不同虚拟机上恢复

此场景出现在 Azure 公有云中：一个休眠的客户虚拟机仅作为已保存的配置和磁盘存在——
该虚拟机不再存在于任何 Hyper-V 主机上。当客户虚拟机恢复时，会创建一个具有完全相同
配置的新 Hyper-V 虚拟机，很可能位于不同的 Hyper-V 主机上。那个新的 Hyper-V 虚拟机
成为恢复后的客户虚拟机，而 Linux 内核为从休眠镜像恢复所采取的步骤必须能在这个新
虚拟机中正常工作。

虽然磁盘及其内容从原始虚拟机保留下来，但磁盘控制器和其他合成设备的、由 Hyper-V
提供的 VMBus 实例 GUID 通常会不同。这种差异会导致从休眠恢复失败，因此采取了若干
措施来解决这个问题：

- 对于只支持单个实例的 VMBus 合成设备，Hyper-V 总是分配相同的实例 GUID。例如，
  Hyper-V 鼠标、shutdown 伪设备、时间同步伪设备等，无论本地 Hyper-V 安装还是
  Azure 云中，总是具有相同的实例 GUID。

- VMBus 合成 SCSI 控制器在一个虚拟机中可能有多个实例，一般情况下实例 GUID 因虚拟机
  而异。然而，Azure 虚拟机总是恰好有两个合成 SCSI 控制器，而 Azure 代码会覆盖正常的
  Hyper-V 行为，使这些控制器总是被分配相同的两个实例 GUID。因此，当客户虚拟机在一个
  新建的虚拟机上恢复时，实例 GUID 是匹配的。但这一保证对本地 Hyper-V 安装不成立。

- 类似地，VMBus 合成 NIC 在一个虚拟机中可能有多个实例，实例 GUID 因虚拟机而异。
  同样，Azure 代码会覆盖正常的 Hyper-V 行为，使得客户虚拟机中合成 NIC 的实例 GUID
  不会改变，即使客户虚拟机被解除分配或休眠，然后在新建的虚拟机上重新构成。与 SCSI
  控制器一样，此行为对本地 Hyper-V 安装不成立。

- vPCI 设备在新创建的虚拟机上从休眠恢复时，不具有相同的实例 GUID。因此，Azure 不支持
  带有 DDA 设备（如 NVMe 控制器或 GPU）的虚拟机休眠。对于 SR-IOV NIC VF，Azure 在
  虚拟机休眠之前从虚拟机中移除该 VF，使休眠镜像不包含 VF 设备。当虚拟机恢复时，它会
  实例化一个新的 VF，而不是尝试与休眠镜像中存在的某个 VF 匹配。由于 Azure 必须在
  发起休眠之前移除任何 VF，Azure 虚拟机的休眠必须从 Azure Portal 或 Azure CLI 外部
  发起，后者进而使用 Shutdown 集成服务告知 Linux 进行休眠。如果休眠是在 Azure 虚拟机
  内部自行发起的，VF 会保留在休眠镜像中，无法被正确恢复。

总之，Azure 采取特殊措施来移除 VF，并确保 VMBus 设备实例 GUID 在新的/不同的虚拟机上
匹配，从而使休眠对大多数通用 Azure 虚拟机规格可用。虽然在本地 Hyper-V 安装上、在不同
虚拟机上恢复时也可以采取类似的特殊措施，但编排这些措施并非本地 Hyper-V 开箱即用提供
的，因此需要自定义脚本。
