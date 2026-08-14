
## 内核驱动 i2c-sis630


支持适配器：
  - Silicon Integrated Systems Corp (SiS)
	630 芯片组（Datasheet: available at http://www.sfr-fresh.com/linux）
	730 芯片组
	964 芯片组
  - 可能还有其他 SiS 芯片组？

Author:
        - Alexander Malysh <amalysh@web.de>
 - Amaury Decrême <amaury.decreme@gmail.com> - SiS964 支持

### 模块参数


==================      =====================================================
force = [1|0]           强制启用 SIS630。危险！
                        这对于上述未列出的芯片组可能有用，以检查它是否适用于你的
                        芯片组，但很危险！

high_clock = [1|0]      强制将主机主时钟设为 56KHz（默认即你的 BIOS 所用值）。危险！
			这应该会稍快一些，但会使某些系统（如我的笔记本）死机。
			SIS630/730 芯片专用。
==================      =====================================================


### 描述


已知该仅支持 SMBus 的驱动可在使用上述芯片组的主板上工作。

```

  00:00.0 Host bridge: Silicon Integrated Systems [SiS] 630 Host (rev 31)
  00:01.0 ISA bridge: Silicon Integrated Systems [SiS] 85C503/5513

```
```
  00:00.0 Host bridge: Silicon Integrated Systems [SiS] 730 Host (rev 02)
  00:01.0 ISA bridge: Silicon Integrated Systems [SiS] 85C503/5513

```
```
  00:00.0 Host bridge: Silicon Integrated Systems [SiS] 760/M760 Host (rev 02)
  00:02.0 ISA bridge: Silicon Integrated Systems [SiS] SiS964 [MuTIOL Media IO]
							LPC Controller (rev 36)

```
若上述输出出现在你的 `lspci` 输出中，则本驱动适用于你的芯片组。

### 致谢


Philip Edelbrock <phil@netroedge.com>
- 测试 SiS730 支持
Mark M. Hoffman <mhoffman@lightlink.com>
- bug 修复

也感谢这里被我遗漏的任何人 ;)
