
## Linux I2C 从机 testunit 后端


by Wolfram Sang <wsa@sang-engineering.com> in 2020

此后端可用于触发针对 I2C 总线主控制器的测试用例，这些测试需要具有特能力（且通常不易获得）的远程设备。例子包括多主控制器测试SMBus Host
Notify 测试。对于某些测试，I2C 从机控制器必须能够在主模式与从模式之切换，因为它也需要发送数据
请注意，这是一个用于测试和调试的设备，不应在生产构建中启用。虽然我做了一定的版本管理并尽力保持向后兼容，*不保*稳定ABI
```

  # echo "slave-testunit 0x1030" > /sys/bus/i2c/devices/i2c-0/new_device

```
或者使用固件节点。下面是一devicetree 示例（注意这只是一```

  &i2c0	{
        ...

	testunit@30 {
		compatible = "slave-testunit";
		reg = <(0x30 | I2C_OWN_SLAVE_ADDRESS)>;
	};
  };

```
之后，该设备会开始监听。读取将返回单个字节。其值为 0 表示 testunit
空闲，否则为当前正在运行的命令的编号
写入时，该设备由 4 8 位寄存器组成，除某些“部分”命令外，要启动一测试用例必须写入所有寄存器，也就是说你通常会向设备写入 4 个字节寄存器如下：

  :header: "Offset", "Name", "Description"

  0x00, CMD, 要触发的测试
  0x01, DATAL, 该测试的配置字节 1
  0x02, DATAH, 该测试的配置字节 2
  0x03, DELAY, 启动测试前的延迟，单位为 n * 10ms

```

  # i2cset -y <bus_num> <testunit_address> <CMD> <DATAL> <DATAH> <DELAY> i

```
DELAY 是一个通用参数，会延迟 CMD 中测试的执行的执行。当某个命令正在运行
（包括延迟期间）时，新的命令不会被应答。你需要等待旧命令完成
命令在下一节中描述。无效的命令会导致传输不被应答
### 命令


#### 0x00 NOOP


保留供将来使用
#### 0x01 READ_BYTES


  :header-rows: 1

  - - CMD
    - DATAL
    - DATAH
    - DELAY

  - - 0x01
    - 要读取数据的地址（低 7 位，最高位当前未用    - 要读取的字节    - n * 10ms

还需要主模式。这对于测试你的总线主控制器是否正确地处理多主控制器很有用你可以触testunit 从总线上的另一个设备读取字节。如果被测的总线主控制器
同时也想访问总线，总线将处于忙状态。读128 字节的示例：
```

  # i2cset -y 0 0x30 1 0x50 0x80 5 i

```
#### 0x02 SMBUS_HOST_NOTIFY


  :header-rows: 1

  - - CMD
    - DATAL
    - DATAH
    - DELAY

  - - 0x02
    - 要发送的状态字低字    - 要发送的状态字高字    - n * 10ms

还需要主模式。该测试将向主机发送一SMBUS_HOST_NOTIFY 消息。请注意，状字目前在 Linux 内核中被忽略```

  # i2cset -y 0 0x30 2 0x42 0x64 1 i

```
如果主机控制器支HostNotify，这条调试级别的消息```

  Detected HostNotify from address 0x30

```
#### 0x03 SMBUS_BLOCK_PROC_CALL


  :header-rows: 1

  - - CMD
    - DATAL
    - DATAH
    - DELAY

  - - 0x03
    - 0x01（即还会再写入一个字节）
    - 要回送的字节    - 省略，部分命令！

部分命令。该测试会按SMBus 规范的定义响应一次块处理调用（block process
call）。写入的那一个数据字节指定了在随后的读传输中将回送多少字节。请注意在此读传输中，testunit 会先放置后续字节的长度前缀。因此，如果你的主机
总线驱动像大多数驱动那样模拟 SMBus 调用，它就需要支i2c_msg I2C_M_RECV_LEN 标志。这是一个很好的测试用例。返回的数据先是长度，然后是
一个从 length-1 0 的字节数组。下面是一个使i2ctransfer 模拟
i2c_smbus_block_process_call() 的示例（你需i2c-tools v4.2 ```

  # i2ctransfer -y 0 w3@0x30 3 1 0x10 r?
  0x10 0x0f 0x0e 0x0d 0x0c 0x0b 0x0a 0x09 0x08 0x07 0x06 0x05 0x04 0x03 0x02 0x01 0x00

```
#### 0x04 GET_VERSION_WITH_REP_START


  :header-rows: 1

  - - CMD
    - DATAL
    - DATAH
    - DELAY

  - - 0x04
    - 当前未用
    - 当前未用
    - 省略，部分命令！

部分命令。发送此命令后，testunit 会以一个基UTS_RELEASE、以 NUL 结尾版本字符串来回应读消息。第一个字符始终是 'v'，版本字符串长度最大为 128
字节。不过，它仅在读消息通过 repeated start 与写消息相连时才会回应。如你的控制器驱动处```

  # i2ctransfer -y 0 w3@0x30 4 0 0 r128
  0x76 0x36 0x2e 0x31 0x31 0x2e 0x30 0x2d 0x72 0x63 0x31 0x2d 0x30 0x30 0x30 0x30 ...

```
```

  # i2ctransfer -y -b 0 w3@0x30 4 0 0 r128
  v6.11.0-rc1-00009-gd37a1b4d3fd0

```
两条消息之间STOP/START 组合***会生效，因为它们不等同于一REPEATED START。例如，这只会返```

  # i2cset -y 0 0x30 4 0 0 i; i2cget -y 0 0x30
  0x00

```
#### 0x05 SMBUS_ALERT_REQUEST


  :header-rows: 1

  - - CMD
    - DATAL
    - DATAH
    - DELAY

  - - 0x05
    - 响应值（7 位解释为 I2C 地址    - 当前未用
    - n * 10ms

该测试通过 SMBAlert 引脚引发一个中断，主机控制器必须处理它。该引脚必须
作为 GPIO 连接testunit。对 GPIO 的访问不允许睡眠。目前，这只能使固件节点来描述。因此，对于 devicetree，你会在 testunit 中添加类似如下的
```

  gpios = <&gpio1 24 GPIO_ACTIVE_LOW>;

```
以下命令会在 1 秒后触发一个响应值为 0xc9 的告```

  # i2cset -y 0 0x30 5 0xc9 0x00 100 i

```
如果主机控制器支SMBusAlert，这条调试级别的消息```

  smbus_alert 0-000c: SMBALERT# from dev 0x64, flag 1

```
这条消息可能出现不止一次，因为 testunit 是软件而非硬件，因此可能无法快响应主机的回```

  # cat /proc/interrupts | grep smbus_alert
   93:          1  gpio-rcar  26 Edge      smbus_alert

```
如果主机1 秒内没有响应告警，测试将被中止，testunit 会报告一个错误
对于此测试，testunit 会短暂地放弃其被分配的地址，并SMBus Alert
Response Addressx0c）上监听。之后它会重新分配其原始地址