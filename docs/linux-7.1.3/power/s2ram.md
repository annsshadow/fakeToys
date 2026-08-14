## 如何让 s2ram 工作


2006 Linus Torvalds
2006 Pavel Machek

1) 查看 suspend.sf.net，其中的 s2ram 程序拥有很长的“已知可用”机器白名单，
   以及每台机器上可用的技巧。

2) 若那没有帮助，尝试阅读 tricks.txt 与 video.txt。问题也许简单如某个有
   缺陷的模块，简单地卸载模块即可解决。

3) 你可以使用 Linus 的 TRACE_RESUME 基础设施，如下所述。

#### 使用 TRACE_RESUME


我一直在努力让我手头的机器能够 STR（挂起到内存），而几乎总是某个驱动存在
缺陷。感谢 suspend/resume 调试——也就是 Chuck 曾试图禁用的那个东西。那常常
是调试这些问题的_唯一_途径，而且它实际上相当强大（但很耗时——必须在无法
resume 的设备驱动中插入 TRACE_RESUME() 标记，然后重新编译并重启）。

无论如何，对于感兴趣的人（拥有一台无法启动的机器）调试方法如下：

 - 启用 PM_DEBUG 与 PM_TRACE

```
#!/bin/sh
sync
echo 1 > /sys/power/pm_trace
echo mem > /sys/power/state

   以挂起

 - 若它没有恢复（这通常是问题所在），按住电源按钮重启，并查看 dmesg 输出中
   类似如下的内容：

	Magic number: 4:156:725
	hash matches drivers/base/power/resume.c:28
	hash matches device 0000:01:00.0

   这意味着最后一个跟踪事件恰好发生在尝试恢复设备 0000:01:00.0 之前。然后
   弄清是哪个驱动在控制该设备（lspci 和 /sys/devices/pci* 是你的好帮手），
   看看能否修复它、禁用它，或跟踪进入它的 resume 函数。

   若没有设备匹配该哈希（或任何匹配看起来是误报），罪魁祸首可能是来自可加载
   内核模块的、在哈希检查之后才加载的设备。你可以在加载更多模块后，使用
   sysfs 再次将哈希与当前设备比对：

	cat /sys/power/pm_trace_dev_match

```
例如，上面的情况恰好是我 EVO 上的 VGA 设备，我曾用 “radeonfb” 运行它（它是
一块 ATI Radeon 移动显卡）。结果发现 “radeonfb” 根本无法恢复该设备——它尝试
设置 PLL，然后就_挂起_了。使用常规的 VGA 控制台，并改由 X 来恢复它，则工作
正常。

## 注意


pm_trace 使用系统的实时时钟（RTC）来保存魔数（magic number）。原因是 RTC 是
resume 操作期间唯一可靠可用的、能够设置一个可 surviving 重启的值的硬件。

pm_trace 与异步挂起（asynchronous suspend）不兼容，因此它会关闭异步挂起
（这可能绕过时序或顺序相关的缺陷）。

后果是，在 resume 之后（即使成功），你的系统时钟将具有对应于魔数而非正确
日期/时间的值！因此，在使用此跟踪选项时，建议使用 ntp-date 或 rdate 之类的
程序从外部时间源重置正确的日期/时间。

由于时钟持续走动，在 resume 失败后尽快重启也至关重要。该跟踪选项不使用 RTC
的秒或分钟的低位，但过长的延迟会破坏魔数值。
