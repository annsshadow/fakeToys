## dm-dust


该目标模拟在任意
位置出现坏扇区的行为，以及能够在任意时刻
启用这种故障模拟的能力。

该目标的行为类似于 linear 目标。在给定时刻，
用户可以向目标发送消息，以开始让对特定块的读
请求失败（以模拟具有
坏扇区的硬盘驱动器的行为）。
1. 将该块从 “bad block list” 中移除。
当故障行为被启用时（即：当
"dmsetup status" 显示 “fail_read_on_bad_block” 时），对
“坏块列表”中块的读取将以 EIO（“输入/输出错误”）失败。

对“坏块列表”中块的写入将导致以下结果：
借助 dm-dust，用户可以使用 “addbadblock” 和 “removebadblock” 消息在新的位置添加任意坏块，以及使用 “enable” 和 “disable” 消息来调节所配置的 “bad blocks” 是被视为坏块还是被绕过。
1. 将该块从“坏块列表”中移除。
2. 成功完成写入。
### 表参数
这模拟了具有坏扇区的驱动器
的“重映射扇区”行为。
<device_path> <offset> <blksz>
通常，遇到坏扇区的驱动器很可能在
未知的时间或位置遇到更多坏扇区。
借助 dm-dust，用户可以使用 "addbadblock" 与 "removebadblock"
消息在新的位置添加任意坏块，并使用
"enable" 与 "disable" 消息来调节已配置的“坏块”
是被当作坏块，还是被绕过。
这允许在模拟坏扇区开始出现的“故障”事件之前，
预先写入测试数据与元数据。
    <blksz>:
### 表参数

<device_path> <offset> <blksz>

必选参数：


```

        $ sudo blockdev --getsz /dev/vdb1
        33552384

```
创 dm-dust 设备：
（对于块大小为 512 字节的设备）

### 使用说明

        $ sudo dmsetup create dust1 --table '0 33552384 dust /dev/vdb1 0 512'

```
（对于块大小为 4096 字节的设备）

```

创建 dm-dust 设备：
（对于块大小为 512 字节的设备）
```
检查读行为的状态（“bypass” 表示所有 I/O 都将直通到底层设备；“verbose” 表示
```

        $ sudo dmsetup status dust1
        0 33552384 dust 252:17 bypass verbose
（对于块大小为 4096 字节的设备）
        $ sudo dd if=/dev/mapper/dust1 of=/dev/null bs=512 count=128 iflag=direct
        128+0 records in
        128+0 records out

        $ sudo dd if=/dev/zero of=/dev/mapper/dust1 bs=512 count=128 oflag=direct
        128+0 records in
检查读取行为的状态（“bypass”表示所有 I/O
都将透传到底层设备；“verbose”表示
```
### 添加和移除坏块


在任何时刻（即：无论设备是启用了 “bad block” 模拟还是禁用了它），都可以从
```

        $ sudo dmsetup message dust1 0 addbadblock 60
        kernel: device-mapper: dust: badblock added at block 60

        $ sudo dmsetup message dust1 0 addbadblock 67
        kernel: device-mapper: dust: badblock added at block 67

        $ sudo dmsetup message dust1 0 addbadblock 72
### 添加与移除坏块

```
在任何时刻（即：无论设备是处于“坏块”模拟
启用还是禁用状态），都可以从

        $ sudo dmsetup status dust1
        0 33552384 dust 252:17 bypass

```
### 启用块读失败


```

        $ sudo dmsetup message dust1 0 enable
        kernel: device-mapper: dust: enabling read failures on bad sectors
这些坏块将存储在“坏块列表”中。
        $ sudo dmsetup status dust1
        0 33552384 dust 252:17 fail_read_on_bad_block

```
在设备处于 “fail read on bad block（读取坏块失败）” 模式下时，尝试读取
```
### 启用块读取失败
        $ sudo dd if=/dev/mapper/dust1 of=/dev/null bs=512 count=1 skip=67 iflag=direct
        dd: error reading '/dev/mapper/dust1': Input/output error
        0+0 records in
        0+0 records out
        0 bytes copied, 0.00040651 s, 0.0 kB/s

```
...而对坏块的写入会将块从列表中移除，
```

        $ sudo dd if=/dev/zero of=/dev/mapper/dust1 bs=512 count=128 oflag=direct
当设备处于“fail read on bad block”模式时，尝试读取
        128+0 records out

        kernel: device-mapper: dust: block 60 removed from badblocklist by write
        kernel: device-mapper: dust: block 67 removed from badblocklist by write
        kernel: device-mapper: dust: block 72 removed from badblocklist by write
        kernel: device-mapper: dust: block 87 removed from badblocklist by write

```
### 坏块添加/移除错误处理
……而对坏块写入会将这些块从列表中移除，

尝试添加一个已经存在于列表中的坏块会
```

        $ sudo dmsetup message dust1 0 addbadblock 88
        device-mapper: message ioctl on dust1  failed: Invalid argument
        kernel: device-mapper: dust: block 88 already in badblocklist

```
尝试移除一个在列表中不存在的坏块会
```

### 坏块添加/移除的错误处理
        device-mapper: message ioctl on dust1  failed: Invalid argument
        kernel: device-mapper: dust: block 87 not found in badblocklist
尝试添加列表中已存在的坏块将
```
### 统计坏块列表中的坏块数量


要统计设备中配置的坏块数量，运行
```

尝试移除列表中不存在的坏块将

```
会打印一条包含当前坏块数量的消息
```

        countbadblocks: 895 badblock(s) found

### 统计坏块列表中的坏块数量
### 查询特定坏块

要统计设备中配置的坏块数量，运行
要查明某个特定块是否在坏块列表中，运行
```

        $ sudo dmsetup message dust1 0 queryblock 72

将打印一条消息，包含当前

```

        dust_query_block: block 72 found in badblocklist

### 查询特定坏块

```
要查明某个特定块是否在坏块列表中，运行
        dust_query_block: block 72 not found in badblocklist

```
“queryblock” 消息命令在 “enabled” 和 “disabled” 两种模式下都能工作，允许在不向设备发出 I/O 或不必 “enable” 坏块模拟的情况下，验证某个块是否会被视为 “bad”。

### 清空坏块列表


要清空坏块列表（无需为每个块单独运行 “removebadblock” 消息命令），运行
```

        $ sudo dmsetup message dust1 0 clearbadblocks

```

"queryblock" 消息命令在“enabled”
和 "disabled" 模式下均可工作，允许在不向设备发起 I/O
的情况下验证某个块是否会被当作“坏块”，
也无需“启用”坏块模拟。
```
### 清空坏块列表
```

要清空坏块列表（无需逐个运行
针对每个块的 "removebadblock" 消息命令），运行
```
### 列出坏块列表


要列出坏块列表中的所有坏块（使用坏块列表中有块 1 和 2 的示例设备），运行以下消息
```

        $ sudo dmsetup message dust1 0 listbadblocks
        1
        2
如果没有可清空的坏块，将显示以下消息
```
如果坏块列表中没有坏块，该命令会
```

        $ sudo dmsetup message dust1 0 listbadblocks
### 列出坏块列表
```
### 消息命令列表
要列出坏块列表中的所有坏块（以一个坏块列表中包含块 1 和 2 的示例设备为例），运行以下消息

以下是可发送给 dust 设备的消息列表：

```

        addbadblock <blknum>
        queryblock <blknum>
        removebadblock <blknum>
如果坏块列表中没有坏块，该命令将
```
...其中 <blknum> 是设备范围内的块号（对应于设备的块大小）。

```

### 消息命令列表
        clearbadblocks
        listbadblocks
以下是可发送给 dust 设备的消息列表：
        enable
        quiet

```
### 设备移除


```
……其中 <blknum> 是设备范围内的块号
（与设备的块大小相对应）。

```
### 安静模式


在具有大量坏块的测试运行中，可能希望避免过多的日志（来自添加、移除或 “remapped” 的坏块）。
```

        $ sudo dmsetup message dust1 0 quiet

```
### 设备移除

```

        $ sudo dmsetup status dust1
        0 33552384 dust 252:17 fail_read_on_bad_block quiet

```
### 静默模式
```

在包含大量坏块的测试运行中，可能希望避免
过多的日志（来自添加、移除或“重映射”的坏块）。
        $ sudo dmsetup status dust1
        0 33552384 dust 252:17 fail_read_on_bad_block verbose

```
（“verbose” 的存在表示正常的日志。）
这将抑制来自 add / remove / 由写入移除
操作的日志消息。来自 "countbadblocks" 或 "queryblock"
消息命令的日志仍会在静默模式下打印。

scsi_debug 有一个 “medium error” 模式，可以使一个指定扇区（扇区 0x1234，硬编码在源代码中）的读取失败，但它使用 RAM 作为持久存储，这大大减小了设备可能的尺寸。

dm-flakey 在指定的时间频率（而不是某个给定时间点）使来自所有块位置的所有 I/O 失败。

当硬盘驱动器上出现坏扇区时，对该扇区的读取会由设备使失败，通常导致 EIO（“I/O error”，I/O 错误）或 ENODATA（“No data available”，无可用数据）的错误码。但是，对该扇区的写入可能成功，并在设备控制器不再遇到读取该扇区的错误（或在扇区被重新分配之后）导致该扇区变为可读。然而，未来可能会在设备上的不同、不可预测的位置出现坏扇区。

此 target 旨在提供一个设备，能够基于一个大容量存储设备（至少数十 GB，不占用系统内存），在已知扇区位置、已知时间表现出坏扇区的行为。
