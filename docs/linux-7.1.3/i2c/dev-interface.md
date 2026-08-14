## 在用户空间实现 I2C 设备驱动


通常，I2C 设备由内核驱动控制。但也可以通过 /dev 接口，从用户空间访问适配器上的所有设备。为此你需要加载 i2c-dev 模块。

每个注册的 I2C 适配器都会获得一个从 0 开始计数的编号。你可以查看 /sys/class/i2c-dev/ 来了解哪个编号对应哪个适配器。或者，你可以运行 “i2cdetect -l” 来获取系统在某一时刻存在的所有 I2C 适配器的格式化列表。i2cdetect 是 i2c-tools 软件包的一部分。

I2C 设备文件是主设备号为 89、次设备号对应于上述分配编号的字符设备文件。它们应被称为 “i2c-%d”（i2c-0、i2c-1、……、i2c-10、……）。所有 256 个次设备号都保留给 I2C 使用。

## C 示例


那么假设你希望从一个 C 程序访问某个 I2C 适配器。
```

  #include <linux/i2c-dev.h>
  #include <i2c/smbus.h>

```
现在，你必须决定要访问哪个适配器。你应检查 /sys/class/i2c-dev/ 或运行 “i2cdetect -l” 来决定。适配器编号的分配有些动态，因此你不能对其做太多假设。它们甚至可能在两次启动之间发生变化。

```

  int file;
  int adapter_nr = 2; /* 可能动态确定 */
  char filename[20];

  snprintf(filename, 19, "/dev/i2c-%d", adapter_nr);
  file = open(filename, O_RDWR);
  if (file < 0) {
    /* 错误处理；你可以检查 errno 了解出错原因 */
    exit(1);
  }

```
当你打开设备后，必须指定要与之通信的设备
```

  int addr = 0x40; /* I2C 地址 */

  if (ioctl(file, I2C_SLAVE, addr) < 0) {
    /* 错误处理；你可以检查 errno 了解出错原因 */
    exit(1);
  }

```
好了，现在一切都准备就绪。你可以使用 SMBus 命令或纯 I2C 与你的设备通信。如果可能，优先使用 SMBus 命令
```

  __u8 reg = 0x10; /* 要访问的设备寄存器 */
  __s32 res;
  char buf[10];

  /* 使用 SMBus 命令 */
  res = i2c_smbus_read_word_data(file, reg);
  if (res < 0) {
    /* 错误处理：I2C 事务失败 */
  } else {
    /* res 包含读取到的字 */
  }

  /*
   * 使用 I2C 写，等价于
   * i2c_smbus_write_word_data(file, reg, 0x6543)
   */
  buf[0] = reg;
  buf[1] = 0x43;
  buf[2] = 0x65;
  if (write(file, buf, 3) != 3) {
    /* 错误处理：I2C 事务失败 */
  }

  /* 使用 I2C 读，等价于 i2c_smbus_read_byte(file) */
  if (read(file, buf, 1) != 1) {
    /* 错误处理：I2C 事务失败 */
  } else {
    /* buf[0] 包含读取到的字节 */
  }

```
注意，只有通过 read() 和 write() 调用才能实现 I2C 和 SMBus 协议的一个子集。特别是，所谓的组合事务（在同一事务中混合读写消息）不被支持。因此，这个接口几乎从不被用户空间程序使用。

重要：由于使用了内联函数，编译你的程序时**必须**使用 ‘-O’ 或其某种变体！

## 完整接口描述


定义了以下 IOCTL：

`ioctl(file, I2C_SLAVE, long addr)`
  更改从设备地址。地址通过参数的低 7 位传入（10 位地址除外，此时通过低 10 位传入）。

`ioctl(file, I2C_TENBIT, long select)`
  如果 select 不等于 0，则选择 10 位地址；如果 select 等于 0，则选择普通 7 位地址。默认 0。此请求仅在适配器具有 I2C_FUNC_10BIT_ADDR 时才有效。

`ioctl(file, I2C_PEC, long select)`
  如果 select 不等于 0，则选择生成并校验 SMBus PEC（包错误检查）；如果 select 等于 0，则禁用。默认 0。仅用于 SMBus 事务。此请求仅在适配器具有 I2C_FUNC_SMBUS_PEC 时才起作用；即便没有也仍然安全，只是没有任何效果。

`ioctl(file, I2C_FUNCS, unsigned long *funcs)`
  获取适配器功能并放入 `*funcs`。

`ioctl(file, I2C_RDWR, struct i2c_rdwr_ioctl_data *msgset)`
  执行组合读/写事务，中间不发送停止（stop）。仅当适配器具有 I2C_FUNC_I2C 时才有效。参数是
```

    struct i2c_rdwr_ioctl_data {
      struct i2c_msg *msgs;  /* 指向简单消息数组的指针 */
      int nmsgs;             /* 要交换的消息数量 */
    }

  这些 msgs[] 自身含有指向数据缓冲区的进一步指针。函数会根据特定消息中是否设置了 I2C_M_RD 标志，向这些缓冲区写入或从其中读取数据。从设备地址以及是否使用 10 位地址模式必须在每条消息中设置，覆盖上述 ioctl 设置的值。

```
`ioctl(file, I2C_SMBUS, struct i2c_smbus_ioctl_data *args)`
  如果可能，请使用下面描述的 `i2c_smbus_*` 方法，而不是直接发出 ioctl。

你可以使用 read(2) 和 write(2) 调用执行纯 I2C 事务。你无需传递地址字节；相反，在尝试访问设备之前通过 ioctl I2C_SLAVE 设置它。

你可以执行 SMBus 级事务（参见文档文件 smbus-protocol.rst
```

  __s32 i2c_smbus_write_quick(int file, __u8 value);
  __s32 i2c_smbus_read_byte(int file);
  __s32 i2c_smbus_write_byte(int file, __u8 value);
  __s32 i2c_smbus_read_byte_data(int file, __u8 command);
  __s32 i2c_smbus_write_byte_data(int file, __u8 command, __u8 value);
  __s32 i2c_smbus_read_word_data(int file, __u8 command);
  __s32 i2c_smbus_write_word_data(int file, __u8 command, __u16 value);
  __s32 i2c_smbus_process_call(int file, __u8 command, __u16 value);
  __s32 i2c_smbus_block_process_call(int file, __u8 command, __u8 length,
                                     __u8 *values);
  __s32 i2c_smbus_read_block_data(int file, __u8 command, __u8 *values);
  __s32 i2c_smbus_write_block_data(int file, __u8 command, __u8 length,
                                   __u8 *values);

```
所有这些事务在失败时返回 -1；你可以读取 errno 了解发生了什么。‘写’事务在成功时返回 0；‘读’事务返回读取到的值，但 read_block 例外，它返回读取到的值的数量。块缓冲区不必长于 32 字节。

上述函数通过链接 libi2c 库提供，该库由 i2c-tools 项目提供。参见：
https://git.kernel.org/pub/scm/utils/i2c-tools/i2c-tools.git/。

## 实现细节


对于感兴趣的人，以下是当你使用 /dev 接口访问 I2C 时，内核内部发生的代码流程：

1) 你的程序打开 /dev/i2c-N 并对其调用 ioctl()，如上面“C 示例”一节所述。

2) 这些 open() 和 ioctl() 调用由 i2c-dev 内核驱动处理：分别参见 i2c-dev.c:i2cdev_open() 和 i2c-dev.c:i2cdev_ioctl()。你可以把 i2c-dev 看作一个可从用户空间编程的通用 I2C 芯片驱动。

3) 某些 ioctl() 调用用于管理任务，由 i2c-dev 直接处理。例子包括 I2C_SLAVE（设置你要访问的设备的地址）和 I2C_PEC（在未来事务上启用或禁用 SMBus 错误检查）。

4) 其他 ioctl() 调用由 i2c-dev 转换为内核内函数调用。例子包括 I2C_FUNCS，它使用 i2c.h:i2c_get_functionality() 查询 I2C 适配器功能；以及 I2C_SMBUS，它使用 i2c-core-smbus.c:i2c_smbus_xfer() 执行 SMBus 事务。

   i2c-dev 驱动负责检查来自用户空间的所有参数是否有效。在此之后，这些通过 i2c-dev 来自用户空间的调用，与由内核 I2C 芯片驱动直接执行的调用之间就没有区别了。这意味着 I2C 总线驱动无需实现任何特殊的东西来支持来自用户空间的访问。

5) 这些 i2c.h 函数是你的 I2C 总线驱动实际实现的封装。每个适配器都必须声明实现这些标准调用的回调函数。i2c.h:i2c_get_functionality() 调用 i2c_adapter.algo->functionality()，而 i2c-core-smbus.c:i2c_smbus_xfer() 要么调用 adapter.algo->smbus_xfer()（如果已实现），要么调用 i2c-core-smbus.c:i2c_smbus_xfer_emulated()，后者进而调用 i2c_adapter.algo->master_xfer()。

在你的 I2C 总线驱动处理完这些请求后，执行沿调用链向上返回，几乎不做任何处理，除了 i2c-dev 在需要时把返回的数据打包成适合 ioctl 的格式。
