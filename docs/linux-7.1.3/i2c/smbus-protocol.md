## The SMBus Protocol（SMBus 协议

以下是对 SMBus 协议的概要说明，适用于该协议的所有修订版本（1.0.1 2.0）。某些不被本软件包支持的协议特性，将在本文档末尾简要说明

部分适配器只能理SMBus（System Management Bus，系统管理总线）协议，它是 I2C 协议的一个子集。幸运的是，许多设备只使用了相同的这个子集，因而可以把它们挂在 SMBus 上

如果你为某个 I2C 设备编写驱动，请尽可能使SMBus 命令（前提是设备只使用了 I2C 协议的该子集）。这样就能让同一驱动既可用于 SMBus 适配器，也可用于 I2C 适配器（I2C 适配器上，SMBus 命令集会自动转换I2C；但I2C 命令在大多数SMBus 适配器上完全无法处理）

下面列出 SMBus 协议操作及其对应的执行函数。请注意，SMBus 协议规范中使用的名称通常与这些函数名并不一致；对于某些只传递单个数据字节的操作，使SMBus 协议操作名的函数实际上执行的是完全不同的协议操作

每种事务类型都对应一个功能标志（functionality flag）。在调用某个事务函数之前，设备驱动应当（只需一次）先检查相应的功能标志，以确认底层 I2C 适配器支持该事务。详Documentation/i2c/functionality.rst

## Key to symbols（符号说明）

=============== =============================================================
S               Start 条件（起始条件）
Sr              Repeated start 条件（重复起始条件），用于在写与读之间切
P               Stop 条件（停止条件）
Rd/Wr (1 bit)   Read/Write 位。Rd 等于 1，Wr 等于 0
A, NA (1 bit)   应答（ACK）与非应答（NACK）位
Addr  (7 bits)  I2C 7 位地址。注意该地址可扩展为 10 位
Comm  (8 bits)  命令字节，一个数据字节，通常用来选择设备上的某个寄存器
Data  (8 bits)  一个普通的数据字节。DataLow DataHigh 表示 16 位字中的低字节与高字节
Count (8 bits)  一个包含块操作长度的数据字节
[..]            I2C 设备发送的数据，与主机适配器发送的数据相对
=============== =============================================================

## SMBus Quick Command

  S Addr Rd/Wr [A] P

功能标志：I2C_FUNC_SMBUS_QUICK

该命令向设备写入一个比特（位于 Rd/Wr 位中）。部分设备会借此触发某个动作

## SMBus Receive Byte

  S Addr Rd [A] [Data] NA P

功能标志：I2C_FUNC_SMBUS_READ_BYTE

i2c_smbus_read_byte() 实现

此操作从设备读取一个字节，且不指定设备寄存器。有些设备非常简单，这个接口就足够了；对于其它设备，如果你希望读取与下文相同的寄存器，它只是一种简写形式

## SMBus Send Byte

  S Addr Wr [A] [Data] NA P

功能标志：I2C_FUNC_SMBUS_WRITE_BYTE

i2c_smbus_write_byte() 实现

这是 Receive Byte 的逆操作：它向设备发送一个字节。更多信息请参阅“Receive Byte”

## SMBus Read Byte

  S Addr Wr [A] Comm [A] Sr Addr Rd [A] [Data] NA P

功能标志：I2C_FUNC_SMBUS_READ_BYTE_DATA

i2c_smbus_read_byte_data() 实现

此操作从一个指定的设备寄存器（通过 Comm 指定）读取一个字节

## SMBus Read Word

  S Addr Wr [A] Comm [A] Sr Addr Rd [A] [DataLow] A [DataHigh] NA P

功能标志：I2C_FUNC_SMBUS_READ_WORD_DATA

i2c_smbus_read_word_data() 实现

该操作与 Read Byte 非常相似；同样是从设备、从一个通过 Comm 指定的寄存器读取数据。注意，对于两个数据字节顺序相反（不符合 SMBus，但非常流行）的读取，可以使用便捷函i2c_smbus_read_word_swapped()

## SMBus Write Byte

  S Addr Wr [A] Comm [A] [Data] NA P

功能标志：I2C_FUNC_SMBUS_WRITE_BYTE_DATA

i2c_smbus_write_byte_data() 实现

此操作向设备的一个指定寄存器写入一个字节。寄存器通过 Comm 字节指定。这Read Byte 操作的逆操作

## SMBus Write Word

  S Addr Wr [A] Comm [A] [DataLow] A [DataHigh] NA P

功能标志：I2C_FUNC_SMBUS_WRITE_WORD_DATA

i2c_smbus_write_word_data() 实现

这是 Read Word 操作的逆操作，向设备、向指定的寄存器写入 16 位数据。注意，对于两个数据字节顺序相反（不符合 SMBus，但非常流行）的写入，可以使用便捷函i2c_smbus_write_word_swapped()

## SMBus Process Call

  S Addr Wr [A] Comm [A] [DataLow] A [DataHigh] NA Sr Addr Rd [A] [DataLow] A [DataHigh] NA P

功能标志：I2C_FUNC_SMBUS_PROC_CALL

i2c_smbus_proc_call() 实现

该命令选择一个设备寄存器（通过 Comm 字节），发16 位数据，再读16 位数据

## SMBus Block Read

  S Addr Wr [A] Comm [A] Sr Addr Rd [A] [Count] A [Data] ... A P

功能标志：I2C_FUNC_SMBUS_READ_BLOCK_DATA

i2c_smbus_read_block_data() 实现

此命令从一个指定的设备寄存器（通过 Comm 字节指定）读取最32 字节的块。数据量由设备通过 Count 字节指定

## SMBus Block Write

  S Addr Wr [A] Comm [A] [Count] A [Data] ... A P

功能标志：I2C_FUNC_SMBUS_WRITE_BLOCK_DATA

i2c_smbus_write_block_data() 实现

这是 Block Read 命令的逆操作，向设备、向通过 Comm 字节指定的寄存器写入最32 字节。数据量Count 字节中指定

## SMBus Block Write - Block Read Process Call

  S Addr Wr [A] Comm [A] [Count] A [Data] ... A Sr Addr Rd [A] [Count] A [Data] ... A P

功能标志：I2C_FUNC_SMBUS_BLOCK_PROC_CALL

i2c_smbus_block_proc_call() 实现

SMBus Block Write - Block Read Process Call 在规范的 2.0 修订版中引入。它先写入一个数据块，再读回一个数据块

## SMBus Host Notify

  [S] [HostAddr] [Wr] A [DevAddr] A [DataLow] A [DataHigh] A [P]

功能标志：I2C_FUNC_SMBUS_HOST_NOTIFY

该命令由充当主设备的 SMBus 设备发送给充当从设备的 SMBus 主机。它的形式与 Write Word 相同，只是命令码被替换为报警设备的地址

Linux 内核中，它的实现方式如下

- 支持 SMBus Host Notify I2C 总线驱动应报I2C_FUNC_SMBUS_HOST_NOTIFY
- 对于能够触发 SMBus Host Notify 的设备，I2C 驱动如果没有被其他人指定其它中断，则 client->irq 会被分配为一Host Notify IRQ

## Packet Error Checking (PEC)

Packet Error Checking 在规范的 1.1 修订版中引入。PEC 在使用它的传输中、紧接在终止STOP 之前，添加一CRC-8 错误检查字节

## Address Resolution Protocol (ARP)

地址解析协议（Address Resolution Protocol）是在规范的 2.0 修订版中引入的。它是一个使用上述消息的更高层协议。ARP 为协议增加了设备枚举与动态地址分配功能。所ARP 通信都使用从机地址 0x61，并且需PEC 校验和

## SMBus Alert

SMBus 报警协议在规范的 1.0 修订版中引入。SMBus 报警协议允许多个 SMBus 从设备共SMBus 主设备上的一个中断引脚，同时仍允许主设备知道是哪个从设备触发了中断

这在 Linux 内核中按以下方式实现

- 支持 SMBus Alert I2C 总线驱动应调i2c_new_smbus_alert_device() 来安SMBus Alert 支持
- I2C 总线驱动通过调用相应接口来触SMBus Host Notify

## I2C 鍧椾簨鍔。

I2C 块事务不限制传输的字节数，但 SMBus 层施加了 32 字节的限制

  S Addr Wr [A] Comm [A]
            Sr Addr Rd [A] [Data] A [Data] A ... A [Data] NA P

功能标志：I2C_FUNC_SMBUS_READ_I2C_BLOCK

i2c_smbus_read_i2c_block_data() 实现

此命令从一个指定寄存器读取字节。注意，长度0 或更多字节的命令是受支持的，因为它们与数据无法区分

  S Addr Wr [A] Comm [A] Data [A] Data [A] ... [A] Data [A] P

i2c_smbus_write_i2c_block_data() 实现

这是块读取命令的逆操作，向设备、向通过 Comm 字节指定的寄存器写入字节
