## i2c-stub


## 描述


本模块是一个非常简单假I2C/SMBus 驱动。它实现了六种类型的 SMBus
命令：write quick、（写）byte、（写）byte data、（写）
word data、（写）I2C block data，以及（写）SMBus block data
加载本驱动时，你需要以模块参数的形式提供芯片地址，此后它只会响应发往
这些地址SMBus 命令
本模块不需要也不关联任何硬件。它会接受发往指定地址write quick 命令对于其它命令（同样仅针对指定地址），它会通过读写内存中的数组来响应它还会为处理的每条命令向内核日志刷屏
所有字节操作都实现了一个带自动递增的指针寄存器。这允许进行连续的字读取，例EEPROM 所支持的那些
SMBus block 命令支持默认是禁用的，必须通过设置 functionality 模块参数的相应位x03000000）来显式启用
必须写入 SMBus block 命令，才能为 SMBus block 操作配置一SMBus 命令写入可以是部分的。block 读命令总是返回迄今为止最大一次写入所选择的字节数
典型的使用场景如下：

 1. 加载本模 2. 使用 i2cset（来i2c-tools 项目）预加载一些数 3. 加载目标芯片驱动模块
 4. 在内核日志中观察其行
i2c-tools 软件包中有一个名i2c-stub-from-dump 的脚本，可以自动从芯转储中加载寄存器值
## 参数


int chip_addr[^10^]	要模拟芯片的 SMBus 地址
unsigned long functionality	功能覆盖，用于禁用某些命令。合适的值请参阅 <linux/i2c.h> 中的
	I2C_FUNC_* 常量。例如，0x1f0000 只会启用 quick、byte 	byte data 命令
u8 bank_reg[^10^]、u8 bank_mask[^10^]、u8 bank_start[^10^]、u8 bank_end[^10^]	可选的 bank 设置。它们指示哪个寄存器中的哪些位选择当前 bank	以及 banked 寄存器的范围
## 注意事项


如果你的目标驱动轮询某个字节或字以等待其改变，stub 可能会将其锁死使用 i2cset 来解锁
如果你对它刷屏足够严重，printk 可能会丢消息。本模块确实希望有类relayfs 这样的机制