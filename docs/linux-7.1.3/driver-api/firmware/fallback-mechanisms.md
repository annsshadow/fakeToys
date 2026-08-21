## 回退机制


内核支持一种回退机制，用于克服在根文件系统上直接进行文件系统查找失败的情况，
或在实际原因导致固件根本无法安装到根文件系统上的情况。与固件回退机制支持
相关的内核配置选项有：

  - CONFIG_FW_LOADER_USER_HELPER：启用构建固件回退机制。如今大多数发行    都启用了此选项。如果启用了它但禁用    CONFIG_FW_LOADER_USER_HELPER_FALLBACK，则仅有自定义回退机制可用    且仅针对 request_firmware_nowait() 调用  - CONFIG_FW_LOADER_USER_HELPER_FALLBACK：强制为每个请求在所有固API 调用
    上启kobject uevent 回退机制，request_firmware_direct() 除外。如    大多数发行版禁用了此选项。调request_firmware_nowait() 允许一种替代的
    回退机制：如果启用了kconfig 选项，并且您传给 request_firmware_nowait()
    的第二个参数 uevent 设为 false，则您是在告知内核您拥有自定义的回退机制    将由您手动加载固件。详见下文
注意，这意味着当配置为
CONFIG_FW_LOADER_USER_HELPER=y
CONFIG_FW_LOADER_USER_HELPER_FALLBACK=n

时，即使对于 uevent 设为 true request_firmware_nowait()，kobject uevent
回退机制也永远不会生效
## 固件回退机制的合理

直接文件系统查找可能因多种原因失败。值得列举并记录这些已知原因，因为它们
证明了回退机制的必要性：

- 与启动过程中访问根文件系统发生竞争
- 从挂起恢复时发生竞争。这由固件缓存解决，但固件缓存仅在您使用 uevent   受支持，request_firmware_into_buf() 不支持它
- 固件无法通过常规手段访问
        - 无法安装到根文件系统        - 固件提供使用本地信息收集的、为设备量身定制的非常独特的设备特定
          数据。例如移动设WiFi 芯片组的校准数据。该校准数据并非所          设备通用，而是针对每台设备定制。此类信息可能安装在除根文件系统
          所在分区之外的单独闪存分区上
## 回退机制的类

实际有两种回退机制可用，它们共用同一sysfs 接口作为加载设施
- Kobject uevent 回退机制
- 自定义回退机制

首先来记录共用的 sysfs 加载设施
## 固件 sysfs 加载设施


为了帮助设备驱动使用回退机制上传固件，固件基础设施会创建一sysfs 接口以便用户空间在固件就绪时加载并通知。该 sysfs 目录通过 fw_create_instance()
创建。此调用创建一个以所请求固件命名的新 struct device，并通过将发出请求的
设备关联为该设备的父设备，将其建立到设备层级中。该 sysfs 目录的文件属性由
新设备的类（firmware_class）和组（fw_dev_attr_groups）定义和控制。这其实
就是最firmware_class 模块名称的由来，因为最初唯一可用的固件加载机制就我们现在用作回退机制的机制，它注册了一struct class firmware_class。由所暴露的属性是模块名称的一部分，模块名 firmware_class 将来不能被重命名，以
确保与旧用户空间的向后兼容性
要使sysfs 接口加载固件，我们暴露一loading 指示符，以及一个文用来上传固件到：

  - /sys/$DEVPATH/loading
  - /sys/$DEVPATH/data

要上传固件，您将 1 写入 loading 文件，以指示您正在加载固件。然后将固件写入
data 文件，并通过0 写入 loading 文件来通知内核固件已就绪
用于帮助通过 sysfs 加载固件的固件设备，仅在直接固件加载失败、且为您的固请求启用了回退机制时才会创建，这由 `firmware_fallback_sysfs` 设置。需重申的是，如果直接文件系统查找成功，则不会创建任何设备
```

        echo 1 > /sys/$DEVPATH/loading

```
会立即清除任何先前的部分加载，并使固API 返回错误。在加载固件时，
firmware_class PAGE_SIZE 为增量增长一个用于固件数据的缓冲区，以容传入的镜像
firmware_data_read() firmware_loading_show() 仅为 test_firmware 驱动
提供用于测试，它们不会在正常使用中被调用，也不期望被用户空间常规使用
### firmware_fallback_sysfs

   :functions: firmware_fallback_sysfs

## 固件 kobject uevent 回退机制


由于sysfs 接口创建了一个设备以辅助加载固件作为回退机制，用户空间可依靠 kobject uevent 获知该设备的添加。将设备加入设备层级意味着固件加载回退机制已被启动。实现细节请参阅 fw_load_sysfs_fallback()，特别是关于
dev_set_uevent_suppress() kobject_uevent() 的使用
内核kobject uevent 机制实现lib/kobject_uevent.c，它向用户空间发uevent。作为对 kobject uevent 的补充，Linux 发行版也可以启用
CONFIG_UEVENT_HELPER_PATH，它利用核心内核usermode helper（UMH）功能来
调用一个用户空间辅助程序处kobject uevent。不过在实践中，没有任何标准
发行版曾经使用过 CONFIG_UEVENT_HELPER_PATH。如果启用了
CONFIG_UEVENT_HELPER_PATH，则每次内核kobject_uevent_env() 被调用以触发
kobject uevent 时，都会调用此二进制程序
用户空间曾支持不同的实现来利用此回退机制。当固件加载只能使用 sysfs 机制时，
用户空间组件 “hotplug提供了监kobject 事件的功能。历史上它后来被 systemd
udev 取代，不过自 2014 8 月的 v217 起，udev 固件加载支持已从 udev 移除（systemd commit be2ea723b1d0，“udev: remove userspace firmware loading
support”）。这意味着如今大多Linux 发行版并未使用或利用 kobject uevent
提供的固件回退机制。由于如今大多数发行版禁用了
CONFIG_FW_LOADER_USER_HELPER_FALLBACK，这一情况尤为严重
有关 kobject 事件变量设置的细节，请参do_firmware_uevent()。目前随
“kobject add”事件传递给用户空间的变量有
- FIRMWARE=固件名称
- TIMEOUT=超时- ASYNC=API 请求是否为异
默认情况DEVPATH 由内核内kobject 基础设施设置```

        # $DEVPATH $FIRMWARE 均已由环境提供        MY_FW_DIR=/lib/firmware/
        echo 1 > /sys/$DEVPATH/loading
        cat $MY_FW_DIR/$FIRMWARE > /sys/$DEVPATH/data
        echo 0 > /sys/$DEVPATH/loading

```
## 固件自定义回退机制


request_firmware_nowait() 的调用者有另一种可用的选择：依sysfs 回退机制但请求不要向用户空间发出 kobject uevent。这背后的原始逻辑是，udev 之外工具可能需要到非传统路径——即 “Direct filesystem lookup”（直接文件系统查找一节所记录列表之外的路径——去查找固件。此选项对其他任API 调用都不可用因为它们总是被强制发uevent
由于 uevent 只有在回退机制在内核中启用时才有意义，似乎在一些没有在其内核中
启用回退机制的内核上启用 uevent 会很奇怪。遗憾的是，我们还依赖可request_firmware_nowait() 禁用uevent 标志来为固件请求设置固件缓存。如所述，固件缓存仅在API 调用uevent 启用时才被设置。尽管这会为
request_firmware_nowait() 调用禁用固件缓存，但API 的使用者不应将其用禁用缓存，因为那并非该标志的原始用途。不设置 uevent 标志意味着您希望选择加入
固件回退机制，但您希望抑kobject uevent，因为您拥有自定义的解决方案，它以某种方式监控您的设备被加入设备层级，并通过自定义路径为您加载固件
## 固件回退超时


固件回退机制有一个超时。如果在超时值之前固件未被加载到 sysfs 接口上，则会驱动发送一个错误。默认情况下，如uevent 是可取的，超时设60 秒，否则使用
MAX_JIFFY_OFFSET（尽可能大的超时）。在uevent 情况下使MAX_JIFFY_OFFSET
的逻辑是，自定义解决方案将有它需要的时间来加载固件
您可以通过将期望的超时写入以下文件来自定义固件超时
- /sys/class/firmware/timeout

如果您写0，意味着将使MAX_JIFFY_OFFSET。超时的数据类型int
## EFI 内嵌固件回退机制


在某些设备上，系统的 EFI 代码/ROM 可能包含系统部分集成外设设备的固件副本，
而该外设Linux 设备驱动需要访问此固件
需要此类固件的设备驱动可以使用 firmware_request_platform() 函数，注意这一个与其他回退机制分离的回退机制，且不使sysfs 接口
需要此固件的设备驱动可以使efi_embedded_fw_desc 结构体来描述其所需固件
   :functions: efi_embedded_fw_desc

EFI 内嵌固件代码的工作方式是扫描所EFI_BOOT_SERVICES_CODE 内存段，寻找
匹配前缀8 字节序列；如果找到了前缀，则length 字节sha256，若匹配复制 length 字节并将其加入已找到固件列表
为避免在所有系统上都进行这种代价较高的扫描，使用了 dmi 匹配。驱动应当导出一dmi_system_id 数组，其中每个条目的 driver_data 指向一efi_embedded_fw_desc
要向 efi-embedded-fw 代码注册此数组，驱动需要：

1. 始终内建到内核中，或dmi_system_id 数组存放在一个始终被内建的独   目标文件中
2. include/linux/efi_embedded_fw.h 中添加对dmi_system_id 数组   extern 声明
3. dmi_system_id 数组添加drivers/firmware/efi/embedded-firmware.c    embedded_fw_table，并#ifdef 测试该驱动是否正被内建来包裹
4. 在其 Kconfig 条目中添“select EFI_EMBEDDED_FIRMWARE if EFI_STUB”
firmware_request_platform() 函数将始终首先尝试以指定名称直接从磁盘加载固件，
因此通过将文件放/lib/firmware 下，EFI 内嵌固件总是可以被覆盖
注意
1. 扫描 EFI 内嵌固件的代码运行于 start_kernel() 接近末尾处，恰在调用
   rest_init() 之前。对于使subsys_initcall() 注册自身的普通驱动和子系   而言这无关紧要。这意味着运行得更早的代码无法使用 EFI 内嵌固件
2. 目前 EFI 内嵌固件代码假定固件总是起始于一8 字节整数倍的偏移，如果您   情况并非如此，请提交补丁来修复
3. 目前 EFI 内嵌固件代码仅在 x86 上工作，因为其他架构EFI 内嵌固件代码
   有机会扫描之前就释放EFI_BOOT_SERVICES_CODE
4. 当前EFI_BOOT_SERVICES_CODE 的暴力扫描是一种临时的暴力方案。曾有讨   使用 UEFI Platform Initialization（PI）规范的 Firmware Volume 协议。这一
   方案已被拒绝，因FV 协议依赖 PI 规范**internal** 接口，并且：
   1. PI 规范根本未定义外设固   2. PI 规范的内部接口不保证任何向后兼容性。FV 中的任何实现细节都可   发生变更，并可能因系统而异。支FV 协议将十分困难，因为它刻意具   模糊性
### 检查并提取内嵌固件的示

要检查（例如）Silead 触摸屏控制器的内嵌固件，请执行以下操作：

1. 在内核命令行中加efi=debug 启动系统

2. /sys/kernel/debug/efi/boot_services_code 复制到您的主目录

3. 在十六进制编辑器中打开 boot_services_code 文件，搜Silead 固件   魔术前缀：F0 00 00 00 02 00 00 00，这会给出固件在 boot_services_code 文件
   中的起始地址
4. 该固件有特定模式，它以一8 字节的页地址开头，第一页通常F0 00 00 00
   02 00 00 00，后32 位字地址 + 32 位值的配对。字地址每对递增 4 字节
    个字），直到一页完成。一页完整后跟随一个新的页地址，再跟更多字 +    配对。这形成一种非常独特的模式。向下滚动直到此模式停止，这给出固件   boot_services_code 文件中的结束地址
5. “dd if=boot_services_code of=firmware bs=1 skip=<begin-addr> count=<len>   将为您提取固件。在十六进制编辑器中检查固件文件，以确保您给出dd 参数
   正确
6. 将其以期望的名称复制/lib/firmware 下进行测试
7. 如果提取的固件可用，您可以使用找到的信息填充一efi_embedded_fw_desc
   结构体来描述它，运行 “sha256sum firmware以获取要填入 sha256 字段   sha256 校验和