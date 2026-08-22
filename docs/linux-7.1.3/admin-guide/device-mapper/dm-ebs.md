## dm-ebs


该目标与 linear 目标类似，区别在于它在逻辑块大
较大的设备上模拟较小的逻辑块大小。其主要用途是
不提供此模拟的设备（4K 原生磁盘）上提供 512 字节
扇区的模拟

支持的模拟逻辑块大小为 512024048 4096

底层块大小可设置> 4K，以测试更大单元的缓冲


### 表参

  <dev path> <offset> <emulated sectors> [<underlying sectors>]

必选参数：

    <dev path>:
        底层块设备的完整路径名，
        "major:minor" 设备号
    <offset>:
        设备在内的起始扇区；
        必须<emulated sectors> 的整数倍
    <emulated sectors>:
        定义要模拟的逻辑块大小的扇区数；
        支持 1 512 字节扇区

可选参数：

    <underlying sectors>:
        定义 <dev path> 逻辑块大小的扇区数
        支持 2^N，例8 = 模拟 8 512 字节扇区 = 4KiB
        若未提供，则使用 <dev path> 的逻辑块大小


示例

/dev/sda 上从偏移 1024 扇区开始模1 扇区 = 512 字节
逻辑块大小，底层设备块大小自动设置：

ebs /dev/sda 1024 1

/dev/sda 上从偏移 128 扇区开始模2 扇区 = 1KiB 
逻辑块大小，强制 2KiB 底层设备块大小
这要/dev/sda 上的逻辑块大小为 2KiB 或更小才能工作：

ebs /dev/sda 128 2 4
