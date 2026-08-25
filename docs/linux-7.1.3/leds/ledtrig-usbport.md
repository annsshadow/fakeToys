## USB 端口 LED 触发

LED 触发器可用于向用户指示给定端口上是否存在 USB 设备。它会在设备
出现时点LED，并在设备消失时熄灭
它需要选择要观察的 USB 端口。所有可用端口都作为独立条目列在 "ports"
子目录中。选择通过"1" 写入所选端口来完成
请注意，该触发器允许为单LED 选择多个 USB 端口
这在两种情况下有用：

## 1) 具有单个 USB LED 与少量物理端口的设备


在这种情况下，只要存在至少一个已连接USB 设备，LED 就会点亮
## 2) 由少量控制器处理的物理端口的设备


某些设备可能每个 PHY 标准有一个控制器。例USB 3.0 物理端口可能ohci-platform、ehci-platform xhci-hcd 处理。如果只有一LED，用很可能希望分配来自全3 个集线器的端口

该触发器可从用户空间led class 设备上激活，如下所```

  echo usbport > trigger

```
这会LED 添加在以下位置文档化sysfs 属性：
Documentation/ABI/testing/sysfs-class-led-trigger-usbport
```

  echo usbport > trigger
  echo 1 > ports/usb1-port1
  echo 1 > ports/usb2-port1
  cat ports/usb1-port1
  echo 0 > ports/usb1-port1

```
