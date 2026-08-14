## 面向卫星 MC 的 IPMB 驱动


智能平台管理总线（Intelligent Platform Management Bus，IPMB）是一条 I2C 总线，提供机箱内
不同板卡之间的标准化互连。这种互连位于基板管理（BMC）与机箱电子元件之间。IPMB 也与通过
IPMB 总线的消息协议相关联。

使用 IPMB 的设备通常是执行管理功能的管理控制器，例如维护前面板接口、监控基板、
在系统机箱中热插拔磁盘驱动器等。

当系统中实现了 IPMB 时，BMC 充当控制器，为系统软件提供对 IPMB 的访问。BMC 通过 IPMB 向设备
（通常是卫星管理控制器，Satellite Management Controller 或 Satellite MC）发送 IPMI 请求，
设备则将响应发回给 BMC。

有关 IPMB 与 IPMB 消息格式的更多信息，请参考 IPMB 与 IPMI 规范。

### 面向卫星 MC 的 IPMB 驱动


ipmb-dev-int - 这是卫星 MC 上需要的驱动，用于从 BMC 接收 IPMB 消息并发回响应。该驱动与 I2C 驱动
以及一个用户空间程序（如 OpenIPMI）配合工作：

1) 它是一个 I2C 从机后端驱动。因此，它定义了一个回调函数，将卫星 MC 设置为 I2C 从机。
   该回调函数处理接收到的 IPMI 请求。

2) 它定义了读写函数，使用户空间程序（如 OpenIPMI）能够与内核通信。

### 加载 IPMB 驱动


该驱动需要在启动时或手动首先加载。首先，确保你的配置文件中包含以下内容：
CONFIG_IPMB_DEVICE_INTERFACE=y

1) 如果你希望驱动在启动时加载：

```

     Device (SMB0) // 示例 SMBus 主机控制器
     {
     Name (_HID, "<Vendor-Specific HID>") // 厂商特定的 HID
     Name (_UID, 0) // 特定主机控制器的唯一 ID
     :
     :
       Device (IPMB)
       {
         Name (_HID, "IPMB0001") // IPMB 设备接口
         Name (_UID, 0) // 唯一设备标识符
       }
     }

```
```

     &i2c2 {
            status = "okay";

            ipmb@10 {
                    compatible = "ipmb-dev";
                    reg = <0x10>;
                    i2c-protocol;
            };
     };

```
如果要使用原始 i2c 块而非 smbus 来传输数据，则需要如上定义 "i2c-protocol"。

```

     modprobe ipmb-dev-int


```
### 实例化设备


加载驱动后，你可以按照 'Documentation/i2c/instantiating-devices.rst' 中所述实例化设备。如果你有多个
BMC，每个都通过不同的 I2C 总线连接到你的卫星 MC，你可以为每个 BMC 实例化一个设备。

实例化设备的名称包含 I2C 总线编号
```

  BMC1 ------ IPMB/I2C bus 1 ---------|   /dev/ipmb-1
				Satellite MC
  BMC1 ------ IPMB/I2C bus 2 ---------|   /dev/ipmb-2

```
例如，你可以从以下方式实例化 ipmb-dev-int 设备
```

  # echo ipmb-dev 0x1010 > /sys/bus/i2c/devices/i2c-2/new_device

```
这将创建设备文件 /dev/ipmb-2，用户空间程序可以访问它。该设备需要在运行用户空间程序之前实例化。
