## Devlink 自检

`devlink-selftests` API 允许在设备上执行自检。

## 测试掩码

`devlink-selftests` 命令应配合一个掩码运行，以指示要执行的测试。

## 测试说明

以下是驱动可能执行的测试列表。

   :widths: 5 90

   - - 名称
     - 说明
   - - `DEVLINK_SELFTEST_FLASH`
     - 设备可能在板载非易失性存储器（例如 flash）上存放固件。该测试用于在设备上执行 flash 自检。
       测试的具体实现由驱动/固件负责。

### 使用示例


    # 查询 devlink 设备支持的自检
    $ devlink dev selftests show DEV
    # 查询所有 devlink 设备支持的自检
    $ devlink dev selftests show
    # 在设备上执行自检
    $ devlink dev selftests run DEV id flash
