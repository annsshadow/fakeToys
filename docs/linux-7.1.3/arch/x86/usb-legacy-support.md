
## USB 传统支持


:Author: Vojtech Pavlik <vojtech@suse.cz>, 2004 骞?1 鏈。


BIOS Setup 中也称为“USB Keyboard”或“USB Mouse support”的功能，允许将 USB 鼠标和键盘当作经典的 PS/2 对应设备来使用。这意味着可以使用 USB 键盘LILO 中输入

不过它有几个缺点

1) 在某些机器上，即使没USB 鼠标而存在真实的 PS/2 鼠标，模拟的 PS/2 鼠标也会接管。在这种情况下，真实 PS/2 鼠标的额外特性（滚轮、额外按键、触摸板模式）可能不可用

2) 如果启用AMD64 64 位模式，往往会频繁发生系统崩溃，因为 SMM BIOS 没有预料CPU 会处64 位模式。BIOS 制造商只使Windows 测试，Windows 尚未支持 64 位

解决方案

问题 1)
  可以通过在加PS/2 鼠标驱动之前加载 USB 驱动来解决。由PS/2 鼠标驱动2.6 内核中无条件编译进内核，这意味着 USB 驱动也需要编译进内核

问题 2)
  通常通过 BIOS 更新修复。请查看主板制造商的网站。如果没有可用更新，请在 BIOS 中禁USB 传统支持。如果仅此还不够，还可以尝试在内核命令行上添idle=poll。BIOS 也可能在 HLT 指令上进SMM
