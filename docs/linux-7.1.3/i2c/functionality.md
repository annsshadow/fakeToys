## I2C/SMBus Functionality


### 简介


由于并非每个 I2C 或 SMBus 适配器都实现了 I2C 规范中的全部内容，因此当一个客户端获得
挂载到某个适配器的选项时，它不能信任自身所需的全部功能都已被实现：客户端需要某种方式来
检查适配器是否具备所需的功能。


### 功能常量


有关最新版的功能常量列表，请查看 <uapi/linux/i2c.h>！

  =============================== ==============================================
  I2C_FUNC_I2C                    纯 i2c 级命令（纯 SMBus
                                  适配器通常无法执行这些命令）
  I2C_FUNC_10BIT_ADDR             处理 10 位地址扩展
  I2C_FUNC_PROTOCOL_MANGLING      了解 I2C_M_IGNORE_NAK、
                                  I2C_M_REV_DIR_ADDR 和 I2C_M_NO_RD_ACK
                                  标志（这些会修改 I2C 协议！）
  I2C_FUNC_NOSTART                可以跳过 repeated start 序列
  I2C_FUNC_SMBUS_QUICK            处理 SMBus write_quick 命令
  I2C_FUNC_SMBUS_READ_BYTE        处理 SMBus read_byte 命令
  I2C_FUNC_SMBUS_WRITE_BYTE       处理 SMBus write_byte 命令
  I2C_FUNC_SMBUS_READ_BYTE_DATA   处理 SMBus read_byte_data 命令
  I2C_FUNC_SMBUS_WRITE_BYTE_DATA  处理 SMBus write_byte_data 命令
  I2C_FUNC_SMBUS_READ_WORD_DATA   处理 SMBus read_word_data 命令
  I2C_FUNC_SMBUS_WRITE_WORD_DATA  处理 SMBus write_byte_data 命令
  I2C_FUNC_SMBUS_PROC_CALL        处理 SMBus process_call 命令
  I2C_FUNC_SMBUS_READ_BLOCK_DATA  处理 SMBus read_block_data 命令
  I2C_FUNC_SMBUS_WRITE_BLOCK_DATA 处理 SMBus write_block_data 命令
  I2C_FUNC_SMBUS_READ_I2C_BLOCK   处理 SMBus read_i2c_block_data 命令
  I2C_FUNC_SMBUS_WRITE_I2C_BLOCK  处理 SMBus write_i2c_block_data 命令
  =============================== ==============================================

上面这些标志的一些组合也为了你的方便而定义：

  =========================       ======================================
  I2C_FUNC_SMBUS_BYTE             处理 SMBus read_byte
                                  与 write_byte 命令
  I2C_FUNC_SMBUS_BYTE_DATA        处理 SMBus read_byte_data
                                  与 write_byte_data 命令
  I2C_FUNC_SMBUS_WORD_DATA        处理 SMBus read_word_data
                                  与 write_word_data 命令
  I2C_FUNC_SMBUS_BLOCK_DATA       处理 SMBus read_block_data
                                  与 write_block_data 命令
  I2C_FUNC_SMBUS_I2C_BLOCK        处理 SMBus read_i2c_block_data
                                  与 write_i2c_block_data 命令
  I2C_FUNC_SMBUS_EMUL             处理所有可由真实 I2C 适配器模拟的
                                  SMBus 命令（使用透明的
                                  模拟层）
  =========================       ======================================

在 3.5 之前的內核版本中，I2C_FUNC_NOSTART 是作为
I2C_FUNC_PROTOCOL_MANGLING 的一部分实现的。


### 适配器实现


当你编写一个新的适配器驱动时，你将不得不实现一个名为 `functionality` 的函数回调。
典型的实现如下所示。

一个典型的仅支持 SMBus 的适配器会列出它支持的所有 SMBus 事务
```

  static u32 piix4_func(struct i2c_adapter *adapter)
  {
	return I2C_FUNC_SMBUS_QUICK | I2C_FUNC_SMBUS_BYTE |
	       I2C_FUNC_SMBUS_BYTE_DATA | I2C_FUNC_SMBUS_WORD_DATA |
	       I2C_FUNC_SMBUS_BLOCK_DATA;
  }

```
一个典型的完整 I2C 适配器会使用以下内容（来自 i2c-pxa
```

  static u32 i2c_pxa_functionality(struct i2c_adapter *adap)
  {
	return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
  }

```
I2C_FUNC_SMBUS_EMUL 包含了 i2c-core 可以在无需适配器驱动帮助的情况下、使用
I2C_FUNC_I2C 模拟的所有 SMBus 事务（外加 I2C 块事务）。其思想是让客户端驱动检查
对 SMBus 功能的支持，而无需关心这些功能是由适配器在硬件中实现，还是由 i2c-core
在 I2C 适配器之上以软件模拟。


### 客户端检查


在客户端尝试挂载到某个适配器之前，甚至在执行测试以检查它所支持的某个设备是否出现在
适配器上之前，它应该检查所需的功能是否存在。典型的方式是
```

  static int lm75_detect(...)
  {
	(...)
	if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA |
				     I2C_FUNC_SMBUS_WORD_DATA))
		goto exit;
	(...)
  }

```
这里，lm75 驱动检查适配器是否能够同时执行 SMBus byte data 和 SMBus word data 事务。
如果不能，那么该驱动将无法在此适配器上工作，继续下去也没有意义。如果上述检查成功，
驱动便知道它可以调用以下函数：i2c_smbus_read_byte_data()、i2c_smbus_write_byte_data()、
i2c_smbus_read_word_data() 和 i2c_smbus_write_word_data()。作为经验法则，你通过
i2c_check_functionality() 测试的功能常量，应当与你驱动所调用的 i2c_smbus_* 函数
精确匹配。

注意，上述检查并不能说明这些功能是由底层适配器在硬件中实现，还是由 i2c-core 在
软件中模拟。客户端驱动无需关心这一点，因为 i2c-core 会透明地在 I2C 适配器之上
实现 SMBus 事务。


### 通过 /DEV 检查


如果你尝试从用户空间程序访问某个适配器，你将不得不使用 /dev 接口。当然，你仍然需要
检查所需的功能是否受支持。这通过 I2C_FUNCS ioctl 完成。下面一个改编自 i2cdetect
程序的示例：
```

  int file;
  if (file = open("/dev/i2c-0", O_RDWR) < 0) {
	/* Some kind of error handling */
	exit(1);
  }
  if (ioctl(file, I2C_FUNCS, &funcs) < 0) {
	/* Some kind of error handling */
	exit(1);
  }
  if (!(funcs & I2C_FUNC_SMBUS_QUICK)) {
	/* Oops, the needed functionality (SMBus write_quick function) is
           not available! */
	exit(1);
  }
  /* Now it is safe to use the SMBus write_quick command */

```