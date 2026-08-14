## Infinity Usb Unlimited 自述


大家好，


本模块提供一个串行接口，用于以 phoenix 模式使用你的 IUU 单元。加载此模块会带来
一个 ttyUSB[0-x] 接口。你的常用应用程序必须使用此驱动来操控 IUU。

本驱动仍处于测试（beta）阶段，因此可能会出现 bug，并且你的系统可能会冻结。据我所
知，我从未遇到过任何问题，但我并不是真正的专家，所以如果您的系统不稳定，请不要
怪我。

你可以插入多个 IUU。每个单元都会有自己的设备文件（/dev/ttyUSB0、/dev/ttyUSB1、...）。


## 如何调整读卡器速度？


 可以在加载时使用几个参数。要使用参数，只需在模块已加载时先卸载它，然后使用
 modprobe iuu_phoenix param=value。对于预构建的模块，使用命令
 insmod iuu_phoenix param=value。

```

	modprobe iuu_phoenix clockmode=3

 这些参数是：

```
clockmode:
	1=3Mhz579,2=3Mhz680,3=6Mhz (int)
boost:
	超频提升百分比 100 到 500 (int)
cdmode:
	卡检测模式
	0=none, 1=CD, 2=!CD, 3=DSR, 4=!DSR, 5=CTS, 6=!CTS, 7=RING, 8=!RING (int)
xmas:
	xmas 颜色是否启用 (bool)
debug:
	调试是否启用 (bool)

- clockmode 将提供 3 种不同的、被不同软件普遍采用的基准设置：

 1. 3Mhz579
 2. 3Mhz680
 3. 6Mhz

- boost 提供了一种超频读卡器的方式（我的最爱 :-) ）
```

      modprobe boost=195

   这会将读卡器置于 3Mhz579 的基准，但提升了 195%！实际时钟现在将为：6979050 Hz
   （6Mhz979），并将速度提升到比简单的 clockmode=3 好 10 到 20% 的分数！！！


```
- cdmode 允许设置用于通知用户空间（ioctl 应答）卡是否存在与否的信号。
   共有八种可能的信号。

- xmas 除了养眼之外完全没用。这是我的一位朋友，他对拥有像 iuu 这样漂亮的设备却
   看不到所有可用颜色范围感到很遗憾。所以我添加了这个选项，让他能看到大量颜色
   （每次活动变化都会随机改变颜色和频率）。

- debug 会产生大量调试信息……


## 最后说明


 不必担心串行设置，串行仿真是一个抽象层，因此使用任何速度或奇偶校验设置都可以
 工作。（这不会改变任何东西）。以后我也许会用这些设置来推导 boost，但那个功能
 真的有必要吗？所使用的自动检测特性是串行 CD。如果它对你的软件不起作用，请在
 软件中禁用检测机制。


 玩得开心！

 Alain Degreffe

 eczema(at)ecze.com
