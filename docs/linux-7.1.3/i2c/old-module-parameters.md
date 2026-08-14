## 旧内核中从用户空间控制 I2C 设备驱动绑定


   注意：本节仅在你处理在内核 2.6 中发现的一些旧代码时相关。如果你使用的是较新的内核，可以安全地跳过本节。

在内核 2.6.32 之前，许多 I2C 驱动使用 <linux/i2c.h> 提供的辅助宏，这些宏创建了标准的模块参数，让用户可以控制驱动如何探测 I2C 总线并附加到设备。这些参数被称为 `probe`（让驱动探测一个额外的地址）、`force`（强制将驱动附加到给定设备）和 `ignore`（阻止驱动探测给定地址）。

随着 I2C 子系统向标准设备驱动绑定模型转换，这些每模块参数变得不再需要，并且集中式实现成为可能。新的、基于 sysfs 的接口在 Documentation/i2c/instantiating-devices.rst 的"Method 4: Instantiate from user-space"一节中描述。

下面是旧模块参数到新接口的映射。

### 将驱动附加到 I2C 设备


```

  # modprobe <driver> probe=1,0x2d
  # modprobe <driver> force=1,0x2d
  # modprobe <driver> force_<device>=1,0x2d

```

```
  # echo <device> 0x2d > /sys/bus/i2c/devices/i2c-1/new_device

```
### 阻止驱动附加到 I2C 设备


```

  # modprobe <driver> ignore=1,0x2f

```

```
  # echo dummy 0x2f > /sys/bus/i2c/devices/i2c-1/new_device
  # modprobe <driver>

```
当然，重要的是在加载驱动之前实例化 `dummy` 设备。dummy 设备将由 i2c-core 自身处理，从而阻止其他驱动稍后绑定到它。如果问题地址处有一个真实设备，并且你希望另一个驱动绑定到它，那么只需传入相关设备的名称而不是 `dummy`。
