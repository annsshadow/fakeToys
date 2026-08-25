## 内核驱动 i2c-via


支持的适配器：
  - VIA Technologies, InC. VT82C586B
    Datasheet: VIA 网站上公开提供

Author: Ky枚sti M盲lkki <kmalkki@cc.hut.fi>

### 描述


i2c-via 是面向采VIA 芯片组主板的 i2c 总线驱动

支持以下 VIA pci 芯片组：
 - MVP3, VP3, VP2/97, VPX/97
 - 其他采用南桥 VT82C586B 的芯片组

```

 Bridge: VIA Technologies, Inc. VT82C586B ACPI (rev 10)

```
### 有问题？


 Q:
    你的主板上装VT82C586B，但未出现在列表中

 A:
    进入 BIOS 设置，找PCI 设备（或类似）部分
    开USB 支持，然后重试

 Q:
    没有错误消息，但 i2c 似乎仍不工作

 A:
    这种情况可能发生。本驱动使用 VIA 在其数据手册
    推荐的引脚，但主板制造商实际的接线方式可能有很多种

