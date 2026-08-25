## 内核驱动 i2c-sis96x


取代 2.4.x i2c-sis645

支持的适配器：

  - Silicon Integrated Systems Corp (SiS)

    这些主桥的任意组合：
	645, 645DX (aka 646), 648, 650, 651, 655, 735, 745, 746

    以及这些南桥	961, 962, 963(L)

Author: Mark M. Hoffman <mhoffman@lightlink.com>

### 描述


这个SMBus 的驱动已知可在带有上述芯片组组合的的主板上工作。该驱动是在没有 SiS
正规数据手册的情况下开发的。SMBus 寄存器被假定SiS630 的兼容，尽管它们位于完全
不同的位置。感Alexander Malysh <amalysh@web.de> 提供SiS630 数据手册（及驱动）
```

  00:00.0 Host bridge: Silicon Integrated Systems [SiS]: Unknown device 0645
  00:02.0 ISA bridge: Silicon Integrated Systems [SiS] 85C503/5513
  00:02.1 SMBus: Silicon Integrated Systems [SiS]: Unknown device 0016

```
```

  00:00.0 Host bridge: Silicon Integrated Systems [SiS]: Unknown device 0645
  00:02.0 ISA bridge: Silicon Integrated Systems [SiS]: Unknown device 0961
  00:02.1 SMBus: Silicon Integrated Systems [SiS]: Unknown device 0016

```
.4.18 之后版本的内核可能会填上那些“Unknown”）

如果你看不到它，请查quirk_sis_96x_smbus（drivers/pci/quirks.c）（南桥检测失时也适用
我怀疑这个驱动也可以被改造为支持以下 SiS 芯片组：635 635T。如果有人拥有带这些
芯片的主板，并且愿意为了进步而冒险让一个原本行为良好的内核崩溃……请通过
<mhoffman@lightlink.com> 或通过 linux-i2c 邮件列表linux-i2c@vger.kernel.org>
联系我。也请发bug 报告或成功案例

### 待办（TO DOs

- 该驱动不支持 SMBus 块读/写；如果发现需要它们的场景，我可能会添加

### 致谢（Thank You

Mark D. Studebaker <mdsxyz123@yahoo.com>
 - 设计提示bug 修复

Alexander Maylsh <amalysh@web.de>
 - 同上，外加一份重要的数据手册……几乎就是我真正想要的那
Hans-G眉nter L眉tke Uphues <hg_lu@t-online.de>
 - SiS735 的补
Robert Zwerus <arzie@dds.nl>
 - SiS645DX 的测
Kianusch Sayah Karadji <kianusch@sk-tech.net>
 - SiS645DX/962 的补
Ken Healy
 - SiS655 的补
也感谢其它任何提供反馈来函的人！
