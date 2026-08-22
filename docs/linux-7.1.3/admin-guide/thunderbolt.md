
##  USB4 and Thunderbolt（USB4 Thunderbolt

USB4 是基Thunderbolt 3 协议的公开规范，但在寄存器级别等方面存在一些差异连接管理器（connection manager）是一个运行在主机路由器（主机控制器）上的实体负责枚举路由器并建立隧道（tunnel）。连接管理器既可以用固件实现，也可以用软实现。通常，PC 配有用于 Thunderbolt 3 与早USB4 系统的固件连接管理器。Apple 系统则使用软件连接管理器，后来的 USB4 合规设备也沿用此方式
Linux Thunderbolt 驱动同时支持两者，并能在运行时检测应当使用哪种连接管理器
实现。为了安全起见，Linux 中的软件连接管理器还会通告安全级别 `user`，这意味着
默认禁用 PCIe 隧道。下面的文档适用于这两种实现，唯一的例外是软件连接管理器只
支持 `user` 安全级别，并且应当配合基IOMMU DMA 保护一起使用
### Security levels and how to use them（安全级别及其使用方法）


此处呈现的接口并非面向最终用户。相反，应当有一个用户空间工具来处理所有底细节、维护已授权设备的数据库，并在有新连接时提示用户
关于 Thunderbolt 设备 sysfs 接口的更多细节，可以Documentation/ABI/testing/sysfs-bus-thunderbolt 中找到
那些只想连接任何设备而无需任何手动操作的用户，可以将下面这一行添加到
```
（udev 规则中）
  ACTION=="add", SUBSYSTEM=="thunderbolt", ATTR{authorized}=="0", ATTR{authorized}="1"

```
这会在设备出现时自动授权所有设备。然而，请记住这样做会绕过安全级别，并使系统
容易受到 DMA 攻击
Intel Falcon Ridge Thunderbolt 控制器起，有 4 个可用的安全级别。Intel Titan
Ridge 又增加了一个安全级别（usbonly）。之所以需要这些，是因为所连接的设备可成为 DMA 主设备，从而在没有 CPU 和操作系统知晓的情况下读取主机内存的内容。虽可以通过设置 IOMMU 来防止这种情况，但由于各种原因它并不总是可用
一USB4 系统有一个用于禁PCIe 隧道BIOS 设置。这被视为另一个安全级（nopcie）
安全级别如下
  none（无    所有设备由固件自动连接。无需用户批准。在 BIOS 设置中这通常称为
    **Legacy mode（传统模式）**
  user（用户）
    会询问用户是否允许连接该设备。基于通过 `/sys/bus/thunderbolt/devices`
    可用的设备标识信息，用户随后可以做出决定。在 BIOS 设置中这通常称为
    **Unique ID（唯一 ID*
  secure（安全）
    会询问用户是否允许连接该设备。除UUID 之外，设备（如果支持安全连接）还    收到一个挑战值，该值应与基于写`key` sysfs 属性的随机密钥所期望的值匹配    BIOS 设置中这通常称为 **One time saved key（一次性保存密钥）**
  dponly（仅显示端口    固件自动Display Port USB 创建隧道。不进行 PCIe 隧道。在 BIOS 设置    这通常称为 **Display Port Only（仅显示端口*
  usbonly（仅 USB    固件自动为扩展坞中的 USB 控制器与 Display Port 创建隧道。扩展坞下游的所    PCIe 链路被移除
  nopcie（无 PCIe    PCIe 隧道BIOS 禁用/禁止。在一USB4 系统中可用
当前的安全级别可以从 `/sys/bus/thunderbolt/devices/domainX/security` 读取，其`domainX` 是主机控制器管理Thunderbolt 域。通常每个 Thunderbolt 主机控制对应一个域
如果安全级别`user` `secure`，则在创PCIe 隧道（例PCIe 设备出现之前，必须由用户授权所连接的设备
每个插入Thunderbolt 设备都会出现sysfs `/sys/bus/thunderbolt/devices`
下。该设备目录携带可用于识别特定设备的信息，包括其名称UUID
### Authorizing devices when security level is ``user`` or ``secure``（在安全级别``user`` ``secure`` 时授权设备）


```
  /sys/bus/thunderbolt/devices/0-1/authorized	- 0
  /sys/bus/thunderbolt/devices/0-1/device	- 0x8004
  /sys/bus/thunderbolt/devices/0-1/device_name	- Thunderbolt to FireWire Adapter
  /sys/bus/thunderbolt/devices/0-1/vendor	- 0x1
  /sys/bus/thunderbolt/devices/0-1/vendor_name	- Apple, Inc.
  /sys/bus/thunderbolt/devices/0-1/unique_id	- e0376f00-0300-0100-ffff-ffffffffffff

```
`authorized` 属性读取为 0，意味着尚未创建 PCIe 隧道。授权该设备```
（向 authorized 写入 1：）

  # echo 1 > /sys/bus/thunderbolt/devices/0-1/authorized

```
这将创建 PCIe 隧道，设备现已连接
如果设备支持安全连接，且域安全级别设`secure`，它会有一个额外的 `key` 属性，
可保存一个随机的 32 字节值，用于授权与挑战该设备```
（例如：
  /sys/bus/thunderbolt/devices/0-3/authorized	- 0
  /sys/bus/thunderbolt/devices/0-3/device	- 0x305
  /sys/bus/thunderbolt/devices/0-3/device_name	- AKiTiO Thunder3 PCIe Box
  /sys/bus/thunderbolt/devices/0-3/key		-
  /sys/bus/thunderbolt/devices/0-3/vendor	- 0x41
  /sys/bus/thunderbolt/devices/0-3/vendor_name	- inXtron
  /sys/bus/thunderbolt/devices/0-3/unique_id	- dc010000-0000-8508-a22d-32ca6421cb16

```
注意，默认情况下 key 为空
如果用户不想使用安全连接，他们只需 `echo 1` `authorized` 属性，PCIe 隧道就会
以与 `user` 安全级别相同的方式被创建
如果用户想使用安全连接，在设备首次插入时```
（生成并写入密钥，然后授权：
  # key=$(openssl rand -hex 32)
  # echo $key > /sys/bus/thunderbolt/devices/0-3/key
  # echo 1 > /sys/bus/thunderbolt/devices/0-3/authorized

```
现在设备已连接（PCIe 隧道被创建），并且密钥被存储在设备的 NVM 上
下一次插入设备时，用户可以对设备进行验证（挑战）```
（写入密钥并以挑战模式授权：
  # echo $key > /sys/bus/thunderbolt/devices/0-3/key
  # echo 2 > /sys/bus/thunderbolt/devices/0-3/authorized

```
如果设备返回的挑战值与基于密钥所期望的值匹配，设备就被连接并且 PCIe 隧道被创建然而，如果挑战失败，则不会创建任何隧道，并向用户返回错误
如果用户仍想连接该设备，他们可以不用密钥直接批准该设备，或者写入一个新密钥并向
`authorized` 文件写入 1，从而将新密钥存储在设备NVM 上
### De-authorizing devices（取消授权设备）


可以通过`0` 写入`authorized` 属性来取消对设备的授权。这需要连接管理器
实现的支持，可以通过读取域的 `deauthorization` 属性来检查。如果它读为 `1`，则
该功能受支持
当一个设备被取消授权时，从父设备PCIe 下游（或根）端口到设PCIe 上游端口PCIe 隧道会被拆除。这本质上与 PCIe 热移除相同，所涉及PCIe 拓扑将不再可访问直到设备被再次授权。如果涉NVMe 或类似的存储设备，若其上的文件系统未正确
关闭，就有数据丢失的风险。特此警告！

### DMA protection utilizing IOMMU（利IOMMU DMA 保护

2018 年及之后带有 Thunderbolt 端口的新系统可能原生支持 IOMMU。这意味着 Thunderbolt
安全性由 IOMMU 处理，因此所连接的设备无法访问驱动为其分配之外的内存区域。当 Linux
运行在这样的系统上时，如果用户尚未启用，它会自动启用 IOMMU。这些系统可以通过`/sys/bus/thunderbolt/devices/domainX/iommu_dma_protection` 属性读`1` 来识别
在这种情况下，驱动并未做任何特殊操作，但由于 DMA 保护IOMMU 处理，安全级（如果设置了）就变得多余。出于这个原因，一些系统出厂时将安全级别设`none`其他系统将安全级别设`user` 以支持降级到较旧的操作系统，因此希望IOMMU DMA
保护启用时自动授权设备的用户可以使用```
（以udev 规则：）

  ACTION=="add", SUBSYSTEM=="thunderbolt", ATTRS{iommu_dma_protection}=="1", ATTR{authorized}=="0", ATTR{authorized}="1"

```
### Upgrading NVM on Thunderbolt device, host or retimer（升Thunderbolt 设备、主机或重定时器NVM

由于大部分功能由运行在主机控制器或设备上的固件处理，因此固件能够被升级到最版本（其中可能的缺陷已被修复）是很重要的。通常 OEM 会从其支持站点提供该固件
目前，推荐通过 “fwupd工具更新固件。默认情况下它使LVFS（Linux Vendor Firmware
Service，Linux 供应商固件服务）门户从硬件供应商获取最新固件，并在发现兼容时更所连接的设备。详情参见：https://github.com/fwupd/fwupd
在为设备、主机或重定时器升级固件之前，请确保这是一次合适的升级。如果未能做到，
可能会使设备进入一种没有特殊工具就无法正常使用的状态！

Apple Mac 上的主机 NVM 升级不受支持
fwupd 默认已安装。如果你的系统上没有它，只需使用你的发行版包管理器来获取它
要通过 fwupd 查看可能的更新，你需要插入一Thunderbolt 设备，以便主机控制器出现连接哪个设备并不重要（除非你是在升级某个设备NVM——此时你需要连接那个特定的
设备）
注意，你的系统可能提OEM 特定的方法来为上电控制器（“强制上电”，force power），
在这种情况下就无需插入 Thunderbolt 设备
使用 fwupd 更新固件很简单——请参阅 fwupd github 上的官方 readme
如果固件映像写入成功，设备会短暂消失。一旦它重新出现，驱动会注意到它并发起一完整的加电循环。过了一会儿设备会再次出现，此时它应当完全可用
目标设备应在 fwupd 界面中显“Current version（当前版本）下的新版本，以及
“Update State: Success（更新状态：成功）”
### Upgrading firmware manually（手动升级固件）


如果可能，请使用 fwupd 来更新固件。但是，如果你的设备 OEM 尚未将固件上传到 LVFS而它可从他们一侧下载，你可以使用下面的方法直接升级固件
手动固件更新可以使用 'dd' 工具完成。要使用该方法更新固件，你需要将其写入主机或
设备 NVM 的非活跃部分。以下是Intel NUC6i7KYK 上更新的示例```
（将固件映像写入非活NVM：）

  # dd if=KYK_TBT_FW_0018.bin of=/sys/bus/thunderbolt/devices/0-0/nvm_non_active0/nvmem

```
一旦操作完成，我们可以触发 NVM 认证```
（写1 触发认证：）

  # echo 1 > /sys/bus/thunderbolt/devices/0-0/nvm_authenticate

```
如果没有返回错误，设备的行为应与上一节所述一致
我们可以通过运行以下命令来验证新NVM 固件已激活：
```
（检查认证状态与版本：）

  # cat /sys/bus/thunderbolt/devices/0-0/nvm_authenticate
  0x0
  # cat /sys/bus/thunderbolt/devices/0-0/nvm_version
  18.0

```
如果 `nvm_authenticate` 包含0x0 之外的任何值，它就是上一次认证周期的错误码，
这意味着 NVM 映像的认证失败
注意，NVMem 设备的名`nvm_activeN` `nvm_non_activeN` 取决于它们在 NVMem
子系统中注册的顺序。名称中N NVMem 子系统添加的标识符
### Upgrading on-board retimer NVM when there is no cable connected（在没有线缆连接时升级板载重定时器的 NVM

如果平台支持，即USB4 端口上没有连接任何东西，也可能升级重定时NVM 固件在这种情况下，`usb4_portX` 设备有两个特殊属性：`offline`（离线）`rescan`
（重新扫描）。升级固件的方式是：
```
（先将端口置为离线：
  # echo 1 > /sys/bus/thunderbolt/devices/0-0/usb4_port1/offline

```
这一步确保端口不响应任何热插拔事件，同时也确保重定时器被上电。下一步是扫描```
（触发重新扫描以枚举板载重定时器：）

  # echo 1 > /sys/bus/thunderbolt/devices/0-0/usb4_port1/rescan

```
这会枚举并添加板载重定时器。现在可以像有线缆连接时一样升级重定时NVM（参上一节）。然而，由于处于离线模式，重定时器并未断开连接，因此在`nvm_authenticate`
写入 `1` 之后，应当等待：
```
（再次扫描使重定时器重新就绪：）

  # echo 1 > /sys/bus/thunderbolt/devices/0-0/usb4_port1/rescan

```
如果一切顺利，此时可以将端口恢复为```
（退出离线模式：
  # echo 0 > /sys/bus/thunderbolt/devices/0-0/usb4_port1/offline

```
### Upgrading NVM when host controller is in safe mode（在主机控制器处于安全模式时升级 NVM

如果现有 NVM 未被正确认证（或缺失），主机控制器会进入安全模式，这意味着唯一
可用的功能是刷写一个新NVM 映像。在此模式下，读`nvm_version` 会因
`ENODATA` 而失败，并且设备标识信息缺失
要从该模式恢复，需要以与上一章相同的方式向主机控制器刷写一个有效的 NVM 映像
### Tunneling events（隧道事件）


`thunderbolt_domain` 中发生隧道变化时，驱动会向用户空间发`KOBJ_CHANGE`
事件。该通知携带```
（以下环境变量：
  TUNNEL_EVENT=<EVENT>
  TUNNEL_DETAILS=0:12 <-> 1:20 (USB3)

```
`<EVENT>` 的可能取值为
  activated（已激活）
    隧道被激活（创建）
  changed（已改变    此隧道发生了变化。例如带宽分配被改变
  deactivated（已停用    隧道被拆除
  low bandwidth（低带宽    隧道未获得最佳带宽
  insufficient bandwidth（带宽不足）
    当前隧道需求没有足够的带宽
`TUNNEL_DETAILS` 仅在隧道已知时才提供。例如，在固件连接管理器的情况下，这缺失或不提供完整的隧道信息。在软件连接管理器的情况下，这会包含完整的隧道详情目前的格式与驱动记录日志时使用的格式一致。这可能会随时间改变
### Networking over Thunderbolt cable（通过 Thunderbolt 线缆联网

Thunderbolt 技术允许通过 Thunderbolt 线缆连接的两台主机之间进行软件通信
可以Thunderbolt 链路上隧道传输任何类型的流量，但目前我们只支Apple
ThunderboltIP 协议
如果另一台主机运行的Windows macOS，你唯一需要做的是在两台主机之间连一Thunderbolt 线缆；`thunderbolt-net` 驱动会自动加载。如果另一台主机也Linux，你应当在一台主机上手动加载 `thunderbolt-net`（它
```
会自动触发另一台主机上的模块加载：
  # modprobe thunderbolt-net

```
如果驱动内建到内核映像中，则无需做任何事情
驱动会为每个 Thunderbolt 端口创建一个虚拟以太网接口，其名称类似 `thunderbolt0`
等等。从这一点起，你可以使用 `ip` 等标准用户空间工具来配置接口，或让你GUI
自动处理它
### Forcing power（强制上电）


许多 OEM 包含一个方法，可用于将 Thunderbolt 控制器的电源强制置于“开”状态，即使
没有连接任何东西。如果你的机器支持，这会WMI 总线通过一个名“force_powersysfs 属性暴露出来，详见
Documentation/ABI/testing/sysfs-platform-intel-wmi-thunderbolt銆。
注意：目前无法查询平台的强制上电状态