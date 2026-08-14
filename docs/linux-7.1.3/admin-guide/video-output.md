#### 视频输出切换器控制


2006年luming.yu@intel.com

输出 sysfs 类驱动程序提供了一个抽象视频输出层，
可用于挂钩特定于平台的方法以启用/禁用视频输出
通过通用 sysfs 接口访问设备。例如，在我的 IBM ThinkPad T42 上
笔记本电脑，ACPI 视频驱动程序注册其输出设备并读/写
```

  linux:/sys/class/video_output # tree .
  .
  |-- CRT0
  |   |-- device -> ../../../devices/pci0000:00/0000:00:01.0
  |   |-- state
  |   |-- subsystem -> ../../../class/video_output
  |   `-- uevent
  |-- DVI0
  |   |-- device -> ../../../devices/pci0000:00/0000:00:01.0
  |   |-- state
  |   |-- subsystem -> ../../../class/video_output
  |   `-- uevent
  |-- LCD0
  |   |-- device -> ../../../devices/pci0000:00/0000:00:01.0
  |   |-- state
  |   |-- subsystem -> ../../../class/video_output
  |   `-- uevent
  `-- TV0
     |-- device -> ../../../devices/pci0000:00/0000:00:01.0
     |-- state
     |-- subsystem -> ../../../class/video_output
     `-- uevent


```