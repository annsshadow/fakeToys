
## iosm devlink 支持


本文档描述由 `iosm` 设备驱动实现的 devlink 特性。

## 参数


`iosm` 驱动实现了以下驱动特定的参数。

   :widths: 5 5 5 85

   - - 名称
     - 类型
     - 模式
     - 描述
   - - `erase_full_flash`
     - u8
     - runtime
     - erase_full_flash 参数用于检查在固件刷写期间设备是否需要完全擦除。
       如果设置，将向设备发送完整的 nand 擦除命令。默认情况下，
       仅启用条件擦除支持。

## 闪存更新（Flash Update）


`iosm` 驱动实现了使用 `devlink-flash` 接口进行闪存更新的支持。

它支持使用包含 Bootloader 镜像和其他调制解调器软件镜像的组合闪存镜像
来更新设备闪存。

驱动使用 DEVLINK_SUPPORT_FLASH_UPDATE_COMPONENT 来识别需要由用户空间应用程序
请求的闪存刷写类型。支持的固件镜像类型：

    :widths: 15 85

    - - 名称
      - 描述
    - - `PSI RAM`
      - Primary Signed Image（主签名镜像）
    - - `EBL`
      - External Bootloader（外部引导加载程序）
    - - `FLS`
      - Modem Software Image（调制解调器软件镜像）

PSI RAM 和 EBL 是 RAM 镜像，当设备处于 BOOT ROM 阶段时被注入到设备。一旦成功，
实际的调制解调器固件镜像将被刷写到设备。调制解调器软件镜像包含多个文件，
每个文件有一个安全 bin 文件以及至少一个 Loadmap/Region 文件。为了刷写这些
文件，需要向调制解调器设备发送适当的命令以及刷写所需的数据。诸如区域计数和
每个区域的地址这样的数据必须使用 devlink param 命令传递给驱动。

如果设备需要在固件刷写前被完全擦除，用户应用程序需要使用 devlink param 命令
设置 erase_full_flash 参数。默认情况下，支持条件擦除特性。

## 闪存命令：


1) 当调制解调器处于 Boot ROM 阶段时，用户可以使用以下命令通过 devlink flash
   命令注入 PSI RAM 镜像。

$ devlink dev flash pci/0000:02:00.0 file <PSI_RAM_File_name>

2) 如果用户想要进行完全擦除，需要发出以下命令来设置 erase full flash 参数
   （仅在需要完全擦除时设置）。

$ devlink dev param set pci/0000:02:00.0 name erase_full_flash value true cmode runtime

3) 在调制解调器进入 PSI 阶段后注入 EBL。

$ devlink dev flash pci/0000:02:00.0 file <EBL_File_name>

4) 一旦 EBL 注入成功，就会进行实际的固件刷写。以下是用于每个固件镜像的命令
   序列。

a) 刷写安全 bin 文件。

$ devlink dev flash pci/0000:02:00.0 file <Secure_bin_file_name>

b) 刷写 Loadmap/Region 文件。

$ devlink dev flash pci/0000:02:00.0 file <Load_map_file_name>

## 区域（Regions）


`iosm` 驱动支持转储（dump）coredump 日志。

如果固件遇到异常，驱动将获取一个快照。以下区域用于访问设备内部数据。

    :widths: 15 85

    - - 名称
      - 描述
    - - `report.json`
      - 作为该区域一部分记录的异常详情摘要。
    - - `coredump.fcd`
      - 该区域包含与设备中发生的异常相关的详情（RAM 转储）。
    - - `cdd.log`
      - 该区域包含与调制解调器 CDD 驱动相关的日志。
    - - `eeprom.bin`
      - 该区域包含 eeprom 日志。
    - - `bootcore_trace.bin`
      - 该区域包含当前实例的 bootloader 日志。
    - - `bootcore_prev_trace.bin`
      - 该区域包含上一个实例的 bootloader 日志。

## 区域命令


$ devlink region show

$ devlink region new pci/0000:02:00.0/report.json

$ devlink region dump pci/0000:02:00.0/report.json snapshot 0

$ devlink region del pci/0000:02:00.0/report.json snapshot 0

$ devlink region new pci/0000:02:00.0/coredump.fcd

$ devlink region dump pci/0000:02:00.0/coredump.fcd snapshot 1

$ devlink region del pci/0000:02:00.0/coredump.fcd snapshot 1

$ devlink region new pci/0000:02:00.0/cdd.log

$ devlink region dump pci/0000:02:00.0/cdd.log snapshot 2

$ devlink region del pci/0000:02:00.0/cdd.log snapshot 2

$ devlink region new pci/0000:02:00.0/eeprom.bin

$ devlink region dump pci/0000:02:00.0/eeprom.bin snapshot 3

$ devlink region del pci/0000:02:00.0/eeprom.bin snapshot 3

$ devlink region new pci/0000:02:00.0/bootcore_trace.bin

$ devlink region dump pci/0000:02:00.0/bootcore_trace.bin snapshot 4

$ devlink region del pci/0000:02:00.0/bootcore_trace.bin snapshot 4

$ devlink region new pci/0000:02:00.0/bootcore_prev_trace.bin

$ devlink region dump pci/0000:02:00.0/bootcore_prev_trace.bin snapshot 5

$ devlink region del pci/0000:02:00.0/bootcore_prev_trace.bin snapshot 5
