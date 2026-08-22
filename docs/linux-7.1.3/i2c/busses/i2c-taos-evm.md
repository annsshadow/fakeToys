## 内核驱动 i2c-taos-evm


Author: Jean Delvare <jdelvare@suse.de>

这是 TAOS I2C/SMBus 芯片评估模块的驱动。这些模块包含一个功能受限的 SMBus 主设备，可通过串口控制。几乎支持所有评估模块，但需要为每种新模块添加几行代码以在总线上实例化正确I2C 芯片。显然，还需要该芯片对应的驱动

当前支持的设备有

- TAOS TSL2550 EVM

有关 TAOS 产品的更多信息，请参
  http://www.taosinc.com/


### 使用该驱


为了使用该驱动，你需serport 驱动以及 inputattach 工具，后者是 input-utils 软件包的一部分。以下命令将告诉内核你在第一个串口上有一TAOS EVM
```
  # modprobe serport
  # inputattach --taos-evm /dev/ttyS0


```
### 技术细


TAOS 评估模块仅支4 SMBus 事务类型
- Receive Byte（接收字节）
- Send Byte（发送字节）
- Read Byte（读字节
- Write Byte（写字节

通信协议是基于文本的且相当简单。它在评估模块随CD 上的 PDF 文档中有所描述。通信相当慢，因为串口必须1200 bps 运行。不过，我认为在实践中这并不是什么大问题，因为这些模块仅用于评估和测试
