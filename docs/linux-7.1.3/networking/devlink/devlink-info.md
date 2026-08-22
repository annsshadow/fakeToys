
## Devlink 信息


`devlink-info` 机制使设备驱动能够以标准、可扩展的方式上报设备（硬件与固件）
信息
`devlink-info` API 的最初动机有两点
 - 使以与厂商无关的方式对一大堆机器中的设备与固件管理实现自动化成为可能
   （另Documentation/networking/devlink/devlink-flash.rst <devlink_flash>）；
 - 为每个组件命FW 版本（相对于拥挤ethtool 版本字符串）
`devlink-info` 支持上报多种类型的对象。上报驱动版本通常是不被鼓励的——无是在此处，还是通过其他任何 Linux API
   :widths: 5 95

   - - Name
     - Description
   - - `driver`
     - 当前所用设备驱动的名称，也可通过 sysfs 获取
   - - `serial_number`
     - 设备的序列号
       这通常就是 ASIC 的序列号，也常在设备PCI 配置空间中以 **Device Serial
       Number**（设备序列号）能力提供
       序列号对于每个物理设备应当是唯一的。有时设备的序列号只48 位长（即
       以太MAC 地址的长度），而由PCI DSN 64 位，设备会在序列号中填充
       或编码额外信息。一个例子是在额外的两个字节中加入端ID PCI 接口 ID       驱动应确保剥离或归一化任何此类填充或接口 ID，仅上报能唯一标识硬件的那
       部分序列号。换言之，同一设备的两个端口、或同一多主机设备的两个主机所
       上报的序列号应当相同
   - - `board.serial_number`
     - 设备的主板序列号
       这通常就是主板的序列号，常可在 PCI **Vital Product Data**（重要产       数据）中获取
   - - `fixed`
     - 用于硬件标识符，以及不可现场更新的组件版本的组
       本节中的版本标识设备的设计。例PCI VPD 中上报的组件标识符或主板版本       `devlink-info` 中的数据应拆分为最小的逻辑组件，例PCI VPD 可能将各       信息拼接成部件号字符串，而在 `devlink-info` 中，所有部分都应作为独立项
       上报
       该组不得包含任何频繁变化的标识符（例如序列号）。其原因请参       Documentation/networking/devlink/devlink-flash.rst <devlink_flash>
   - - `running`
     - 用于当前运行的软固件信息的组。这些版本通常仅在重启后更新，有时       设备复位后更新
   - - `stored`
     - 用于设备闪存中软固件版本的组
       即使尚未重启，存储的值也必须更新以反映闪存中的变化。如果设备在新软       刷入时无法更`stored` 版本，则不得上报它们
每个版本在每个版本组中最多只能上报一次。若设备能够上报 `stored` 版本，则
存储在闪存中的固件组件应同时出现`running` `stored` 两节中（Documentation/networking/devlink/devlink-flash.rst <devlink_flash>）。如软件/固件组件是从磁盘（例`/lib/firmware`）加载的，则只应通过内核 API 上报
running 版本
请注意，通过 devlink 上报的任何安全版本都仅供信息参考。Devlink 不使用安通道与设备通信
## 通用版本


期望驱动使用以下通用名称来导出版本信息。如果某个给定组件尚无通用名称，驱作者应参考现有的驱动特定版本并尝试复用。万不得已时，若某组件确实独一无二允许使用驱动特定的名称，但应在驱动特定的文档中加以说明
所有版本都应尽量使用以下术语：

   :widths: 10 90

   - - Name
     - Description
   - - `id`, `revision`
     - 设计与修订的标识符，主要用于硬件版本
   - - `api`
     - 组件之间 API 的版本。API 项对用户通常价值有限，且可被厂商从其他版本
       推断出来，因此一般不建议添加 API 版本，以免产生噪声
   - - `bundle_id`
     - 刷入设备的发行包标识符。这是固件包的属性，该固件包涵盖多个版本以便       管理固件镜像（见
       Documentation/networking/devlink/devlink-flash.rst <devlink_flash>）
       `bundle_id` 可以同时出现`running` `stored` 版本中，但如       `bundle_id` 涵盖的任何组件被更改、且不再与包中的版本匹配，则不得
       上报它
### board.id


主板设计的唯一标识符
### board.rev


主板设计修订版本
### asic.id


ASIC 设计标识符
### asic.rev


ASIC 设计修订/步进
### board.manufacture


生产该部件的公司或工厂的标识符
### board.part_number


主板及其组件的部件号
### fw


整体固件版本，通常代表 fw.mgmt、fw.app 等的集合
### fw.mgmt


控制单元固件版本。该固件负责日常事务处理、PHY 控制等，但不负责逐包的数路径操作
### fw.mgmt.api


驱动与固件之间软件接口的固件接口规范版本
### fw.app


控制高速数据包处理的数据路径微码
### fw.undi


UNDI 软件，可能包UEFI 驱动、固件或两者
### fw.ncsi


负责支持/处理网络控制器边带接口（Network Controller Sideband Interface的软件版本
### fw.psid


固件参数集的唯一标识符。这些通常是特定主板在制造时定义的参数
### fw.roce


负责处理 RoCE 管理RoCE 固件版本
### fw.bundle_id


整个固件包的唯一标识符
### fw.bootloader


引导加载程序的版本
## 未来工作


以下扩展可能会有用处
 - 磁盘上的固件文件名——驱动通过 `MODULE_FIRMWARE()` 宏列出它们可能需要加   到设备上的固件文件名。但这些是按模块而非按设备列出的。若能按优先级列   驱动将为给定设备尝试加载的固件文件名，会很有用