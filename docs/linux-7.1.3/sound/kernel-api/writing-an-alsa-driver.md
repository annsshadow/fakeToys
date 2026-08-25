## Writing an ALSA Driver


:Author: Takashi Iwai <tiwai@suse.de>

## Preface


本文档描述了如何编写 `ALSA（Advanced Linux Sound
Architecture，Linux 高级声音架构http://www.alsa-project.org/>`__ 驱动。本文档
主要关注 PCI 声卡。对于其他设备类型，API 也可能有所不同。不过，至少 ALSA
内核 API 是一致的，因此它对编写这些驱动仍会有一些帮助

本文档面向已经具备足C 语言技能、并拥有基本 Linux 内核编程知识的人群
本文档不解释 Linux 内核编码的通用主题，也不涵盖底层驱动的实现细节。它只描
ALSA 上编PCI 声音驱动的标准方法

## File Tree Structure


### General


```
            sound
                    /core
                            /oss
                            /seq
                                    /oss
                    /include
                    /drivers
                            /mpu401
                            /opl3
                    /i2c
                    /synth
                            /emux
                    /pci
                            /(cards)
                    /isa
                            /(cards)
                    /arm
                    /ppc
                    /sparc
                    /usb
                    /pcmcia /(cards)
                    /soc
                    /oss
```

### core 目录


该目录包含了作为 ALSA 驱动核心的中间层。该目录中存放着原生ALSA 模块。其
子目录中包含不同的模块，并且依赖于内核配置

#### core/oss


OSS PCM 与混音器（mixer）模拟模块的代码存放在该目录中。OSS rawmidi 模拟因为
相当小，被包含在 ALSA rawmidi 代码中。音序器（sequencer）代码存放在
`core/seq/oss` 目录中（`下方 <core/seq/oss_>`__）

#### core/seq


该目录及其子目录用于 ALSA 音序器。该目录包含了音序器核心以及主要的音序器模块
例如 snd-seq-midi、snd-seq-virmidi 等。只有当内核配置中设置了
`CONFIG_SND_SEQUENCER` 时，它们才会被编译

#### core/seq/oss


该目录包含了 OSS 音序器模拟代码

### include 目录


这里ALSA 驱动公共头文件的所在位置，这些头文件将被导出到用户空间，或被不
目录中的多个文件包含。基本上，私有头文件不应放在该目录中，但由于历史原因
你仍可能在那里发现一些文:)

### drivers 目录


该目录包含了在不同架构上、不同驱动之间共享的代码。因此它们应当是架构无关的
例如，虚PCM 驱动和串MIDI 驱动就位于此目录中。在其子目录中，放着与总线
CPU 架构无关的组件代码

#### drivers/mpu401


MPU401 MPU401-UART 模块存放在此处

#### drivers/opl3 涓?opl4


OPL3 OPL4 FM 合成（FM-synth）相关的东西可以在这里找到

### i2c 目录


该目录包含了 ALSA i2c 组件

虽然 Linux 上有一个标准的 i2c 层，ALSA 对某些声卡拥有自己的 i2c 代码
因为声卡只需要简单的操作，而标i2c API 对于此类用途来说过于复杂

### synth 目录


该目录包含了合成器（synth）中间层模块

到目前为止，`synth/emux` 子目录下只有 Emu8000/Emu10k1 合成器驱动

### pci 目录


该目录及其子目录保存着 PCI 声卡的顶层声卡模块，以及PCI 总线相关的代码

由单个文件编译而来的驱动直接存放在 pci 目录中，而由多个源文件组成的驱动
存放在它们各自的子目录中（例emu10k1、ice1712）

### isa 目录


该目录及其子目录保存着 ISA 声卡的顶层声卡模块

### arm、ppc sparc 目录


它们用于特定于上述某一种架构的顶层声卡模块

### usb 目录


该目录包含了 USB 音频驱动。USB MIDI 驱动已经被集成进 usb-audio 驱动中

### pcmcia 目录


PCMCIA，尤其是 PCCard 驱动将放在这里。CardBus 驱动将位pci 目录中，
因为它们API 与标PCI 卡相同

### soc 目录


该目录包含了 ASoC（ALSA System on Chip，ALSA 片上系统）层的代码，包括
ASoC 核心、编解码器（codec）以及机器（machine）驱动

### oss 目录


该目录包含了 OSS/Lite 代码。在撰写本文档时，除m68k 上的 dmasound 之外
所有代码都已被移除


## PCI 驱动的基本流


### 概述


PCI 声卡的最小流程如下：

- 定义 PCI ID 表（`PCI Entries`_ 一节）

- 创建 `probe` 回调函数

- 创建 `remove` 回调函数

- 创建一struct pci_driver 结构体，
   其中包含上述三个指针

- 创建一`init` 函数，仅调用
   `pci_register_driver()` 来注册上面定义的 pci_driver
   表

- 创建一`exit` 函数来调
   `pci_unregister_driver()` 函数

### 完整代码示例


下面的代码例子展示了上述流程。某些部分目前尚未实现，但会在后续小节中补全
`snd_mychip_probe()` 函数注释行中的数字对应于下一节中解释的详细说明

```

      #include <linux/init.h>
      #include <linux/pci.h>
      #include <linux/slab.h>
      #include <sound/core.h>
      #include <sound/initval.h>

      /* module parameters (see "Module Parameters") */
      /* SNDRV_CARDS: maximum number of cards supported by this module */
      static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
      static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;
      static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;

      /* definition of the chip-specific record */
      struct mychip {
              struct snd_card *card;
              /* the rest of the implementation will be in section
               * "PCI Resource Management"
               */
      };

      /* chip-specific destructor
       * (see "PCI Resource Management")
       */
      static int snd_mychip_free(struct mychip *chip)
      {
              .... /* will be implemented later... */
      }

      /* component-destructor
       * (see "Management of Cards and Components")
       */
      static int snd_mychip_dev_free(struct snd_device *device)
      {
              return snd_mychip_free(device->device_data);
      }

      /* chip-specific constructor
       * (see "Management of Cards and Components")
       */
      static int snd_mychip_create(struct snd_card *card,
                                   struct pci_dev *pci,
                                   struct mychip **rchip)
      {
              struct mychip *chip;
              int err;
              static const struct snd_device_ops ops = {
                     .dev_free = snd_mychip_dev_free,
              };

              *rchip = NULL;

              /* check PCI availability here
               * (see "PCI Resource Management")
               */
              ....

              /* allocate a chip-specific data with zero filled */
              chip = kzalloc(sizeof(*chip), GFP_KERNEL);
              if (chip == NULL)
                      return -ENOMEM;

              chip->card = card;

              /* rest of initialization here; will be implemented
               * later, see "PCI Resource Management"
               */
              ....

              err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip, &ops);
              if (err < 0) {
                      snd_mychip_free(chip);
                      return err;
              }

              *rchip = chip;
              return 0;
      }

      /* constructor -- see "Driver Constructor" sub-section */
      static int snd_mychip_probe(struct pci_dev *pci,
                                  const struct pci_device_id *pci_id)
      {
              static int dev;
              struct snd_card *card;
              struct mychip *chip;
              int err;

              /* (1) */
              if (dev >= SNDRV_CARDS)
                      return -ENODEV;
              if (!enable[dev]) {
                      dev++;
                      return -ENOENT;
              }

              /* (2) */
              err = snd_card_new(&pci->dev, index[dev], id[dev], THIS_MODULE,
                                 0, &card);
              if (err < 0)
                      return err;

              /* (3) */
              err = snd_mychip_create(card, pci, &chip);
              if (err < 0)
                      goto error;

              /* (4) */
              strcpy(card->driver, "My Chip");
              strcpy(card->shortname, "My Own Chip 123");
              sprintf(card->longname, "%s at 0x%lx irq %i",
                      card->shortname, chip->port, chip->irq);

              /* (5) */
              .... /* implemented later */

              /* (6) */
              err = snd_card_register(card);
              if (err < 0)
                      goto error;

              /* (7) */
              pci_set_drvdata(pci, card);
              dev++;
              return 0;

      error:
              snd_card_free(card);
              return err;
      }

      /* destructor -- see the "Destructor" sub-section */
      static void snd_mychip_remove(struct pci_dev *pci)
      {
              snd_card_free(pci_get_drvdata(pci));
      }



```

### 驱动构造函


PCI 驱动真正的构造函数是 `probe` 回调函数。`probe` 回调函数以及
`probe` 回调函数调用的其他组件构造函数不能使`__init` 前缀
因为任何 PCI 设备都可能是热插拔（hotplug）设备

`probe` 回调函数中，通常会使用如下方案

#### 1) 检查并递增设备索引


```

  static int dev;
  ....
  if (dev >= SNDRV_CARDS)
          return -ENODEV;
  if (!enable[dev]) {
          dev++;
          return -ENOENT;
  }


```

其中 `enable[dev]` 是模块选项

每次调用 `probe` 回调函数时，检查设备的可用性。如果不可用，就简单地递增
设备索引并返回。dev 稍后还会被递增（`步骤 7
<7) Set the PCI driver data and return zero._>`__）

#### 2) 创建声卡实例


```

  struct snd_card *card;
  int err;
  ....
  err = snd_card_new(&pci->dev, index[dev], id[dev], THIS_MODULE,
                     0, &card);


```

相关细节将在 `Management of Cards and
Components`_ 一节中解释

#### 3) 创建主组


```

  struct mychip *chip;
  ....
  err = snd_mychip_create(card, pci, &chip);
  if (err < 0)
          goto error;

```

相关细节将在 `PCI Resource
Management`_ 一节中解释

当发生错误时，probe 函数需要处理该错误。在本例中，我们有一条统一的错误处理路径，
放在

```

  error:
          snd_card_free(card);
          return err;

```

由于每个组件都可以被正确地释放，在大多数情况下，单独一
`snd_card_free()` 调用就足够了


#### 4) 设置驱动 ID 与名称字符串


```

  strcpy(card->driver, "My Chip");
  strcpy(card->shortname, "My Own Chip 123");
  sprintf(card->longname, "%s at 0x%lx irq %i",
          card->shortname, chip->port, chip->irq);

```

driver 字段保存着芯片的最ID 字符串。它alsa-lib 的配置器所使用，因
要保持简单而唯一。即便是同一个驱动，也可以拥有不同的驱动 ID，以区分每种芯片类型
的功能

shortname 字段是作为更详细名称显示的字符串。longname 字段包含
信息显示`/proc/asound/cards` 中

#### 5) 创建其他组件，例如混音器、MIDI 等


在这里你定义基本的组件，例如 `PCM <PCM Interface_>`__、混音器（例
`AC97 <API for AC97 Codec_>`__）、MIDI（例
`MPU-401 <MIDI (MPU401-UART) Interface_>`__）以及其他接口。此外，如果
想要一`proc 文件 <Proc Interface_>`__，也要在这里定义它

#### 6) 注册声卡实例


```

  err = snd_card_register(card);
  if (err < 0)
          goto error;

```

这部分也会在 `Management of Cards and
Components`_ 一节中解释

#### 7) 设置 PCI 驱动数据并返回零


```

  pci_set_drvdata(pci, card);
  dev++;
  return 0;

```

在上面，声卡记录被保存下来。这个指针在 remove 回调函数以及电源管理回调函数
也会被使用

### 析构函数


析构函数，即 remove 回调函数，只是简单地释放声卡实例。随ALSA 中间层会自动
释放所有已挂载的组件

```

  static void snd_mychip_remove(struct pci_dev *pci)
  {
          snd_card_free(pci_get_drvdata(pci));
  }


```

上面的代码假定声卡指针已被设置为 PCI 驱动数据

### 头文


对于上面的例子，至少需要包含以下头文件

```

  #include <linux/init.h>
  #include <linux/pci.h>
  #include <linux/slab.h>
  #include <sound/core.h>
  #include <sound/initval.h>

```

其中最后一个只有在源文件中定义了模块选项时才需要。如果代码被拆分成多个文件，
那么没有模块选项的文件就不需要它们

除了这些头文件之外，中断处理需`<linux/interrupt.h>`，I/O 访问需
`<linux/io.h>`。如果你使用`mdelay()` `udelay()` 函数
还需要包`<linux/delay.h>`

PCM 和控制（control）API 这样ALSA 接口定义在其
`<sound/xxx.h>` 头文件中。它们必须在 `<sound/core.h>` 之后被包含

## 声卡与组件的管理


### 声卡实例


对于每张声卡，都必须分配一个“声卡（card）”记录

声卡记录是声卡的总指挥部。它管理着声卡上整个设备（组件）列表，例如 PCM、混音器
MIDI、合成器等。此外，声卡记录保存着声卡ID 与名称字符串，管理着 proc 文件
的根目录，并控制着电源管理状态与热插拔断开。声卡记录上的组件列表用于在销毁时
管理资源的正确释放

如上所述，要创建声卡实例，调用

```

  struct snd_card *card;
  int err;
  err = snd_card_new(&pci->dev, index, id, module, extra_size, &card);


```

该函数接受六个参数：父设备指针、声卡索引号、id 字符串、模块指针（通常
`THIS_MODULE`）、额外数据空间的大小，以及用于返回声卡实例的指针。extra_size
参数用于为芯片专有数据分card->private_data。注意这些数据是
`snd_card_new()` 分配的

第一个参数，struct device 的指针，指定了父设备。对PCI 设备，通常
传入 `&pci->`

### 组件


在声卡创建之后，你可以将组件（设备）挂载到声卡实例上。在 ALSA 驱动中，一个组
struct snd_device 对象表示。一个组件可以是一PCM 实例、一个控制接口
一raw MIDI 接口等。每一个这样的实例都有一个组件条目

可以通过 `snd_device_new()` 创建一个组件：

```

  snd_device_new(card, SNDRV_DEV_XXX, chip, &ops);

```

它接受声卡指针、设备级别（`SNDRV_DEV_XXX`）、数据指针以及回调指针（`&ops`）
设备级别定义了组件的类型以及注册和反注册的顺序。对于大多数组件，设备级别已
定义好了。对于用户自定义的组件，可以使用 `SNDRV_DEV_LOWLEVEL`

该函数本身并不分配数据空间。数据必须事先手动分配，其指针作为参数传入。这个指
（上面例子中`chip`）被用作该实例的标识符

每个预定义的 ALSA 组件（如 AC97 PCM）都会在其构造函数内部调
`snd_device_new()`。每个组件的析构函数定义在回调指针中。因此，你不需
关心为这样的组件调用析构函数

如果你希望创建自己的组件，则需要将析构函数设置`ops` dev_free 回调中，
以便它能通过 `snd_card_free()` 自动释放。下一个例子将展示芯片专有数据的实现

### 芯片专有数据


芯片专有信息，例I/O 端口地址、其资源等：

```

  struct mychip {
          ....
  };


```

一般来说，分配芯片记录有两种方式

#### 1. 通过 :c:func:`snd_card_new()` 分配


如上所述，你可以将额外数据长度传给5 个参数：

```

  err = snd_card_new(&pci->dev, index[dev], id[dev], THIS_MODULE,
                     sizeof(struct mychip), &card);

```

struct mychip 是芯片记录的类型

作为回报，已分配的记录可以如下方式访问：

```

  struct mychip *chip = card->private_data;

```

使用这种方法，你不必分配两次。该记录会随声卡实例一起被释放

#### 2. 分配一个额外的设备


在通过 `snd_card_new()` 分配声卡实例之后

```

  struct snd_card *card;
  struct mychip *chip;
  err = snd_card_new(&pci->dev, index[dev], id[dev], THIS_MODULE,
                     0, &card);
  .....
  chip = kzalloc(sizeof(*chip), GFP_KERNEL);

```

芯片记录至少应当包含用于保存声卡指针的字段：

```

  struct mychip {
          struct snd_card *card;
          ....
  };


```

```

  chip->card = card;

```

接下来，初始化各字段，并将这个芯片记录注册为一个组件：

```

  static const struct snd_device_ops ops = {
          .dev_free =        snd_mychip_dev_free,
  };
  ....
  snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip, &ops);

```

`snd_mychip_dev_free()` 是设备析构函数：

```

  static int snd_mychip_dev_free(struct snd_device *device)
  {
          return snd_mychip_free(device->device_data);
  }

```

其中 `snd_mychip_free()` 是真正的析构函数

这种方法的缺点显然是代码量更大。但其优点是，你可以通过 snd_device_ops 中的
设置，在注册与断开声卡时触发你自己的回调。关于注册和断开声卡，请参阅下面的小节


### 注册与释


在所有组件都被分配之后，通过调用 `snd_card_register()` 注册声卡实例
此时设备的文件访问被启用。也就是说，在调`snd_card_register()` 之前
组件从外部是无法安全访问的。如果该调用失败，则在通过 `snd_card_free()`
释放声卡之后退probe 函数

要释放声卡实例，你可以简单地调用 `snd_card_free()`。如前所述，所有组件都
通过该调用被自动释放

对于允许热插拔的设备，你可以使用 `snd_card_free_when_closed()`。这个函数会
将销毁推迟到所有设备都关闭之后

## PCI 资源管理


### 完整代码示例


在本节中，我们将补全芯片专有构造函数：

```

      struct mychip {
              struct snd_card *card;
              struct pci_dev *pci;

              unsigned long port;
              int irq;
      };

      static int snd_mychip_free(struct mychip *chip)
      {
              /* disable hardware here if any */
              .... /* (not implemented in this document) */

              /* release the irq */
              if (chip->irq >= 0)
                      free_irq(chip->irq, chip);
              /* release the I/O ports & memory */
              pci_release_regions(chip->pci);
              /* disable the PCI entry */
              pci_disable_device(chip->pci);
              /* release the data */
              kfree(chip);
              return 0;
      }

      /* chip-specific constructor */
      static int snd_mychip_create(struct snd_card *card,
                                   struct pci_dev *pci,
                                   struct mychip **rchip)
      {
              struct mychip *chip;
              int err;
              static const struct snd_device_ops ops = {
                     .dev_free = snd_mychip_dev_free,
              };

              *rchip = NULL;

              /* initialize the PCI entry */
              err = pci_enable_device(pci);
              if (err < 0)
                      return err;
              /* check PCI availability (28bit DMA) */
              if (pci_set_dma_mask(pci, DMA_BIT_MASK(28)) < 0 ||
                  pci_set_consistent_dma_mask(pci, DMA_BIT_MASK(28)) < 0) {
                      printk(KERN_ERR "error to set 28bit mask DMA\n");
                      pci_disable_device(pci);
                      return -ENXIO;
              }

              chip = kzalloc(sizeof(*chip), GFP_KERNEL);
              if (chip == NULL) {
                      pci_disable_device(pci);
                      return -ENOMEM;
              }

              /* initialize the stuff */
              chip->card = card;
              chip->pci = pci;
              chip->irq = -1;

              /* (1) PCI resource allocation */
              err = pci_request_regions(pci, "My Chip");
              if (err < 0) {
                      kfree(chip);
                      pci_disable_device(pci);
                      return err;
              }
              chip->port = pci_resource_start(pci, 0);
              if (request_irq(pci->irq, snd_mychip_interrupt,
                              IRQF_SHARED, KBUILD_MODNAME, chip)) {
                      printk(KERN_ERR "cannot grab irq %d\n", pci->irq);
                      snd_mychip_free(chip);
                      return -EBUSY;
              }
              chip->irq = pci->irq;
              card->sync_irq = chip->irq;

              /* (2) initialization of the chip hardware */
              .... /*   (not implemented in this document) */

              err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip, &ops);
              if (err < 0) {
                      snd_mychip_free(chip);
                      return err;
              }

              *rchip = chip;
              return 0;
      }

      /* PCI IDs */
      static struct pci_device_id snd_mychip_ids[] = {
              { PCI_VENDOR_ID_FOO, PCI_DEVICE_ID_BAR,
                PCI_ANY_ID, PCI_ANY_ID, 0, 0, 0, },
              ....
              { 0, }
      };
      MODULE_DEVICE_TABLE(pci, snd_mychip_ids);

      /* pci_driver definition */
      static struct pci_driver driver = {
              .name = KBUILD_MODNAME,
              .id_table = snd_mychip_ids,
              .probe = snd_mychip_probe,
              .remove = snd_mychip_remove,
      };

      /* module initialization */
      static int __init alsa_card_mychip_init(void)
      {
              return pci_register_driver(&driver);
      }

      /* module clean up */
      static void __exit alsa_card_mychip_exit(void)
      {
              pci_unregister_driver(&driver);
      }

      module_init(alsa_card_mychip_init)
      module_exit(alsa_card_mychip_exit)

      EXPORT_NO_SYMBOLS; /* for old kernels only */

```

### 一些要


PCI 资源的分配在 `probe` 函数中完成，通常为此会专门编写一个额外的
`xxx_create()` 函数

对于 PCI 设备，在分配资源之前，你必须首先调用 `pci_enable_device()` 函数
此外，你还需要设置合适的 PCI DMA 掩码（mask）来限制可访问的 I/O 范围。在某些
情况下，你可能还需要调`pci_set_master()` 函数

```

  err = pci_enable_device(pci);
  if (err < 0)
          return err;
  if (pci_set_dma_mask(pci, DMA_BIT_MASK(28)) < 0 ||
      pci_set_consistent_dma_mask(pci, DMA_BIT_MASK(28)) < 0) {
          printk(KERN_ERR "error to set 28bit mask DMA\n");
          pci_disable_device(pci);
          return -ENXIO;
  }


```

### 资源分配


I/O 端口与中断的分配是通过标准内核函数完成的。这些资源必须在析构函数中被释放
（见下文）

现在假设PCI 设备有一8 字节I/O 端口和一个中断。那struct mychip
将包含：

```

  struct mychip {
          struct snd_card *card;

          unsigned long port;
          int irq;
  };


```

对于 I/O 端口（以及内存区域），你需要为标准资源管理保存资源指针。对于中断，
只需保存中断号（整数）即可。但需要在实际分配之前将其初始化为 -1，因为中0
也是有效的。端口地址及其资源指针会被 `kzalloc()` 自动初始化为 null，因
你不必关心重置它们

```

  err = pci_request_regions(pci, "My Chip");
  if (err < 0) { 
          kfree(chip);
          pci_disable_device(pci);
          return err;
  }
  chip->port = pci_resource_start(pci, 0);

```

它会保留该给PCI 设备 8 字节I/O 端口区域。返回`chip->res_port`
是由 `request_region()` 通过 `kmalloc()` 分配的。该指针必须通过
`kfree()` 释放，但这里存在一个问题。这个问题将在后面解释

```

  if (request_irq(pci->irq, snd_mychip_interrupt,
                  IRQF_SHARED, KBUILD_MODNAME, chip)) {
          printk(KERN_ERR "cannot grab irq %d\n", pci->irq);
          snd_mychip_free(chip);
          return -EBUSY;
  }
  chip->irq = pci->irq;

```

其中 `snd_mychip_interrupt()` 是中断处理函数，定义
`后文 <PCM Interrupt Handler_>`__。注`chip->irq` 应仅
`request_irq()` 成功时才被定义

PCI 总线上，中断是可以共享的。因此，`IRQF_SHARED` 被用
`request_irq()` 的中断标志

`request_irq()` 的最后一个参数是传递给中断处理函数的数据指针。通常，芯
专有记录被用作该指针，但你也可以使用任何你喜欢的东西

我现在不打算给出中断处理函数的细节，但至少它的样子现在可以说明。中断处理函
看起来像这样

```

  static irqreturn_t snd_mychip_interrupt(int irq, void *dev_id)
  {
          struct mychip *chip = dev_id;
          ....
          return IRQ_HANDLED;
  }

```

请求 IRQ 之后，你可以将其传递给 `card->sync_irq`

```

          card->irq = chip->irq;

```

这允PCM 核心在合适的时机（例`hw_free` 之前）自动调
`synchronize_irq()`。详见后文的 `sync_stop callback`_ 一节

现在让我们为上述资源编写相应的析构函数。析构函数的角色很简单：禁用硬件（如
已经激活）并释放资源。到目前为止我们还没有硬件部分，因此这里没有写出禁用代码

对于释放资源，“检查并释放”的方法是一种更安全的方式

```

  if (chip->irq >= 0)
          free_irq(chip->irq, chip);

```

由于中断号可以从 0 开始，你应该用一个负值（例如 -1）初始化 `chip->irq`
这样你就可以像上面那样检查中断号的有效性

当你像本例中一样通过 `pci_request_region()` 
`pci_request_regions()` 请求I/O 端口或内存区域时，使用相应的函数
`pci_release_region()` 鎴。

```

  pci_release_regions(chip->pci);

```

来释放资源

当你通过 `request_region()` `request_mem_region()` 手动请求时，
可以通过 `release_resource()` 释放它。假设你保存了由 `request_region()`
返回的指针：

```

  release_and_free_resource(chip->res_port);

```

在结束之前，别忘了调`pci_disable_device()`

```

  kfree(chip);

```

我们上面没有实现硬件禁用部分。如果你需要这样做，请注意，析构函数甚至在芯片初始
完成之前就可能被调用。最好有一个标志，以便在没有初始化硬件时跳过硬件禁用

当芯片数据通过 `snd_device_new()` 配合 `SNDRV_DEV_LOWLELVEL` 被分配给
声卡时，它的析构函数是最后被调用的。也就是说，可以保证所有其他组件（PCM 
控制）都已经被释放。你不必显式地停PCM 等，只需调用底层硬件停止即可

内存映射（memory-mapped）区域的管理几乎与上面相同：

```

  struct mychip {
          ....
          unsigned long iobase_phys;
          void __iomem *iobase_virt;
  };


```

```

  err = pci_request_regions(pci, "My Chip");
  if (err < 0) {
          kfree(chip);
          return err;
  }
  chip->iobase_phys = pci_resource_start(pci, 0);
  chip->iobase_virt = ioremap(chip->iobase_phys,
                                      pci_resource_len(pci, 0));

```

```

  static int snd_mychip_free(struct mychip *chip)
  {
          ....
          if (chip->iobase_virt)
                  iounmap(chip->iobase_virt);
          ....
          pci_release_regions(chip->pci);
          ....
  }

```

当然，使`pci_iomap()` 的现代方式会让事情变

```

  err = pci_request_regions(pci, "My Chip");
  if (err < 0) {
          kfree(chip);
          return err;
  }
  chip->iobase_virt = pci_iomap(pci, 0, 0);

```

这在析构函数中与 `pci_iounmap()` 配对使用


### PCI 条目


到目前为止，一切顺利。让我们完成缺失PCI 部分。首先，我们需要一
struct pci_device_id 表，用于这个芯片组。它是一PCI 厂商/设备 ID 
以及某些掩码的表

```

  static struct pci_device_id snd_mychip_ids[] = {
          { PCI_VENDOR_ID_FOO, PCI_DEVICE_ID_BAR,
            PCI_ANY_ID, PCI_ANY_ID, 0, 0, 0, },
          ....
          { 0, }
  };
  MODULE_DEVICE_TABLE(pci, snd_mychip_ids);

```

struct pci_device_id 的第一个和第二个字段是厂商和设ID。如果你没有理由过滤
匹配的设备，可以将其余字段保持如上。struct pci_device_id 的最后一个字段包
该条目的私有数据。你可以在这里指定任意值，例如为受支持的设ID 定义特定
操作。这样的例子可以intel8x0 驱动中找到

该列表的最后一个条目是终止符。你必须指定这个全零条目

然后，准struct pci_driver

```

  static struct pci_driver driver = {
          .name = KBUILD_MODNAME,
          .id_table = snd_mychip_ids,
          .probe = snd_mychip_probe,
          .remove = snd_mychip_remove,
  };

```

`probe` `remove` 函数已经在前面几节中定义过了。`name` 字段是这
设备的名称字符串。注意，你不能在该字符串中使用斜杠（”）

```

  static int __init alsa_card_mychip_init(void)
  {
          return pci_register_driver(&driver);
  }

  static void __exit alsa_card_mychip_exit(void)
  {
          pci_unregister_driver(&driver);
  }

  module_init(alsa_card_mychip_init)
  module_exit(alsa_card_mychip_exit)

```

注意，这些模块条目都带有 `__init` `__exit` 前缀

就这些了

## PCM 接口


### 概述


ALSA PCM 中间层相当强大，每个驱动只需实现访问其硬件的低层函数即可

要访PCM 层，你需要先包含 `<sound/pcm.h>`。此外，如果你访问一些与
hw_param 相关的函数，可能还需`<sound/pcm_params.h>`

每张声卡设备最多可以有四个 PCM 实例。一PCM 实例对应一PCM 设备文件。实
数量的限制仅来自 Linux 设备号可用的位大小。一旦使64 位设备号，我们就会有
更多可用PCM 实例

一PCM 实例PCM 播放（playback）和捕获（capture）流组成，而每PCM 流由
一个或多个 PCM 子流（substream）组成。某些声卡支持多种播放功能。例如，emu10k1
拥有 32 个立体声子流PCM 播放。在这种情况下，每次打开时，（通常）会自动选择
并打开一个空闲的子流。同时，当只存在一个子流且它已经被打开时，随后的打开将根
文件打开模式阻塞或以 `EAGAIN` 错误返回。但你不必在驱动中关心这些细节。PCM
中间层会处理这类工作

### 完整代码示例


下面的示例代码不包含任何硬件访问例程，但

```

      #include <sound/pcm.h>
      ....

      /* hardware definition */
      static struct snd_pcm_hardware snd_mychip_playback_hw = {
              .info = (SNDRV_PCM_INFO_MMAP |
                       SNDRV_PCM_INFO_INTERLEAVED |
                       SNDRV_PCM_INFO_BLOCK_TRANSFER |
                       SNDRV_PCM_INFO_MMAP_VALID),
              .formats =          SNDRV_PCM_FMTBIT_S16_LE,
              .rates =            SNDRV_PCM_RATE_8000_48000,
              .rate_min =         8000,
              .rate_max =         48000,
              .channels_min =     2,
              .channels_max =     2,
              .buffer_bytes_max = 32768,
              .period_bytes_min = 4096,
              .period_bytes_max = 32768,
              .periods_min =      1,
              .periods_max =      1024,
      };

      /* hardware definition */
      static struct snd_pcm_hardware snd_mychip_capture_hw = {
              .info = (SNDRV_PCM_INFO_MMAP |
                       SNDRV_PCM_INFO_INTERLEAVED |
                       SNDRV_PCM_INFO_BLOCK_TRANSFER |
                       SNDRV_PCM_INFO_MMAP_VALID),
              .formats =          SNDRV_PCM_FMTBIT_S16_LE,
              .rates =            SNDRV_PCM_RATE_8000_48000,
              .rate_min =         8000,
              .rate_max =         48000,
              .channels_min =     2,
              .channels_max =     2,
              .buffer_bytes_max = 32768,
              .period_bytes_min = 4096,
              .period_bytes_max = 32768,
              .periods_min =      1,
              .periods_max =      1024,
      };

      /* open callback */
      static int snd_mychip_playback_open(struct snd_pcm_substream *substream)
      {
              struct mychip *chip = snd_pcm_substream_chip(substream);
              struct snd_pcm_runtime *runtime = substream->runtime;

              runtime->hw = snd_mychip_playback_hw;
              /* more hardware-initialization will be done here */
              ....
              return 0;
      }

      /* close callback */
      static int snd_mychip_playback_close(struct snd_pcm_substream *substream)
      {
              struct mychip *chip = snd_pcm_substream_chip(substream);
              /* the hardware-specific codes will be here */
              ....
              return 0;

      }

      /* open callback */
      static int snd_mychip_capture_open(struct snd_pcm_substream *substream)
      {
              struct mychip *chip = snd_pcm_substream_chip(substream);
              struct snd_pcm_runtime *runtime = substream->runtime;

              runtime->hw = snd_mychip_capture_hw;
              /* more hardware-initialization will be done here */
              ....
              return 0;
      }

      /* close callback */
      static int snd_mychip_capture_close(struct snd_pcm_substream *substream)
      {
              struct mychip *chip = snd_pcm_substream_chip(substream);
              /* the hardware-specific codes will be here */
              ....
              return 0;
      }

      /* hw_params callback */
      static int snd_mychip_pcm_hw_params(struct snd_pcm_substream *substream,
                                   struct snd_pcm_hw_params *hw_params)
      {
              /* the hardware-specific codes will be here */
              ....
              return 0;
      }

      /* hw_free callback */
      static int snd_mychip_pcm_hw_free(struct snd_pcm_substream *substream)
      {
              /* the hardware-specific codes will be here */
              ....
              return 0;
      }

      /* prepare callback */
      static int snd_mychip_pcm_prepare(struct snd_pcm_substream *substream)
      {
              struct mychip *chip = snd_pcm_substream_chip(substream);
              struct snd_pcm_runtime *runtime = substream->runtime;

              /* set up the hardware with the current configuration
               * for example...
               */
              mychip_set_sample_format(chip, runtime->format);
              mychip_set_sample_rate(chip, runtime->rate);
              mychip_set_channels(chip, runtime->channels);
              mychip_set_dma_setup(chip, runtime->dma_addr,
                                   chip->buffer_size,
                                   chip->period_size);
              return 0;
      }

      /* trigger callback */
      static int snd_mychip_pcm_trigger(struct snd_pcm_substream *substream,
                                        int cmd)
      {
              switch (cmd) {
              case SNDRV_PCM_TRIGGER_START:
                      /* do something to start the PCM engine */
                      ....
                      break;
              case SNDRV_PCM_TRIGGER_STOP:
                      /* do something to stop the PCM engine */
                      ....
                      break;
              default:
                      return -EINVAL;
              }
      }

      /* pointer callback */
      static snd_pcm_uframes_t
      snd_mychip_pcm_pointer(struct snd_pcm_substream *substream)
      {
              struct mychip *chip = snd_pcm_substream_chip(substream);
              unsigned int current_ptr;

              /* get the current hardware pointer */
              current_ptr = mychip_get_hw_pointer(chip);
              return current_ptr;
      }

      /* operators */
      static struct snd_pcm_ops snd_mychip_playback_ops = {
              .open =        snd_mychip_playback_open,
              .close =       snd_mychip_playback_close,
              .hw_params =   snd_mychip_pcm_hw_params,

              .hw_free =     snd_mychip_pcm_hw_free,
              .prepare =     snd_mychip_pcm_prepare,
              .trigger =     snd_mychip_pcm_trigger,
              .pointer =     snd_mychip_pcm_pointer,
      };

      /* operators */
      static struct snd_pcm_ops snd_mychip_capture_ops = {
              .open =        snd_mychip_capture_open,
              .close =       snd_mychip_capture_close,
              .hw_params =   snd_mychip_pcm_hw_params,
              .hw_free =     snd_mychip_pcm_hw_free,
              .prepare =     snd_mychip_pcm_prepare,
              .trigger =     snd_mychip_pcm_trigger,
              .pointer =     snd_mychip_pcm_pointer,
      };

      /*
       *  definitions of capture are omitted here...
       */

      /* create a pcm device */
      static int snd_mychip_new_pcm(struct mychip *chip)
      {
              struct snd_pcm *pcm;
              int err;

              err = snd_pcm_new(chip->card, "My Chip", 0, 1, 1, &pcm);
              if (err < 0)
                      return err;
              pcm->private_data = chip;
              strcpy(pcm->name, "My Chip");
              chip->pcm = pcm;
              /* set operators */
              snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK,
                              &snd_mychip_playback_ops);
              snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE,
                              &snd_mychip_capture_ops);
              /* pre-allocation of buffers */
              /* NOTE: this may fail */
              snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
                                             &chip->pci->dev,
                                             64*1024, 64*1024);
              return 0;
      }


```

### PCM 构造函


一PCM 实例是通过 `snd_pcm_new()` 分配的：

```

  static int snd_mychip_new_pcm(struct mychip *chip)
  {
          struct snd_pcm *pcm;
          int err;

          err = snd_pcm_new(chip->card, "My Chip", 0, 1, 1, &pcm);
          if (err < 0) 
                  return err;
          pcm->private_data = chip;
          strcpy(pcm->name, "My Chip");
          chip->pcm = pcm;
          ...
          return 0;
  }

```

`snd_pcm_new()` 函数接受六个参数。第一个参数是PCM 所分配到的声卡指针
第二个是 ID 字符串

第三个参数（`index`，上面为 0）是这个PCM 的索引。它从零开始。如果你创建
多个 PCM 实例，请在该参数中指定不同的数字。例如，第二PCM 设备使用 ``index =
1``銆。

第四个和第五个参数分别是播放和捕获的子流数量。这里两个参数都使用 1。当没有
播放或捕获子流可用时，向相应参数传入 0

如果一个芯片支持多个播放或捕获，你可以指定更大的数字，但它们必须在 open/close
等回调中被正确处理。当你需要知道你引用的是哪个子流时，可以从传递给每个回调
struct snd_pcm_substream 数据中获取：

```

  struct snd_pcm_substream *substream;
  int index = substream->number;


```

```

  snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK,
                  &snd_mychip_playback_ops);
  snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE,
                  &snd_mychip_capture_ops);

```

```

  static struct snd_pcm_ops snd_mychip_playback_ops = {
          .open =        snd_mychip_pcm_open,
          .close =       snd_mychip_pcm_close,
          .hw_params =   snd_mychip_pcm_hw_params,
          .hw_free =     snd_mychip_pcm_hw_free,
          .prepare =     snd_mychip_pcm_prepare,
          .trigger =     snd_mychip_pcm_trigger,
          .pointer =     snd_mychip_pcm_pointer,
  };

```

所有回调都Operators_ 小节中描述

设置好运算符之后，你可能想要预分配缓冲区并设置托管分配模式

```

  snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
                                 &chip->pci->dev,
                                 64*1024, 64*1024);

```

默认情况下它会分配一个最64kB 的缓冲区。缓冲区管理细节将在后文
`Buffer and Memory Management`_ 一节中描述

此外，你可以`pcm->info_flags` 中为这个 PCM 设置一些额外信息。可用值在
`<sound/asound.h>` 中定义为 `SNDRV_PCM_INFO_XXX`，它用于硬件定义
（后文描述）。当你的声音芯片只支

```

  pcm->info_flags = SNDRV_PCM_INFO_HALF_DUPLEX;


```

### ……那么析构函数呢


PCM 实例的析构函数并不总是必要的。由PCM 设备会被中间层代码自动释放，你不
显式地调用析构函数

如果你在内部创建了特殊的记录并且需要释放它们，则析构函数是必要的。在这种情况下，
设置

```

      static void mychip_pcm_free(struct snd_pcm *pcm)
      {
              struct mychip *chip = snd_pcm_chip(pcm);
              /* free your own data */
              kfree(chip->my_private_pcm_data);
              /* do what you like else */
              ....
      }

      static int snd_mychip_new_pcm(struct mychip *chip)
      {
              struct snd_pcm *pcm;
              ....
              /* allocate your own data */
              chip->my_private_pcm_data = kmalloc(...);
              /* set the destructor */
              pcm->private_data = chip;
              pcm->private_free = mychip_pcm_free;
              ....
      }



```

### 运行时指针——PCM 信息的宝


PCM 子流被打开时，会分配一PCM 运行时（runtime）实例并赋值给该子流。这
指针可以通过 `substream->runtime` 访问。这个运行时指针保存了你控制 PCM
所需的大部分信息：hw_params sw_params 配置的副本、缓冲区指针、mmap 记录
自旋锁等

运行时实例的定义位于 `<sound/pcm.h>` 中。这

```

  struct _snd_pcm_runtime {
          /* -- Status -- */
          struct snd_pcm_substream *trigger_master;
          snd_timestamp_t trigger_tstamp;	/* trigger timestamp */
          int overrange;
          snd_pcm_uframes_t avail_max;
          snd_pcm_uframes_t hw_ptr_base;	/* Position at buffer restart */
          snd_pcm_uframes_t hw_ptr_interrupt; /* Position at interrupt time*/

          /* -- HW params -- */
          snd_pcm_access_t access;	/* access mode */
          snd_pcm_format_t format;	/* SNDRV_PCM_FORMAT_* */
          snd_pcm_subformat_t subformat;	/* subformat */
          unsigned int rate;		/* rate in Hz */
          unsigned int channels;		/* channels */
          snd_pcm_uframes_t period_size;	/* period size */
          unsigned int periods;		/* periods */
          snd_pcm_uframes_t buffer_size;	/* buffer size */
          unsigned int tick_time;		/* tick time */
          snd_pcm_uframes_t min_align;	/* Min alignment for the format */
          size_t byte_align;
          unsigned int frame_bits;
          unsigned int sample_bits;
          unsigned int info;
          unsigned int rate_num;
          unsigned int rate_den;

          /* -- SW params -- */
          struct timespec tstamp_mode;	/* mmap timestamp is updated */
          unsigned int period_step;
          unsigned int sleep_min;		/* min ticks to sleep */
          snd_pcm_uframes_t start_threshold;
          /*
           * The following two thresholds alleviate playback buffer underruns; when
           * hw_avail drops below the threshold, the respective action is triggered:
           */
          snd_pcm_uframes_t stop_threshold;	/* - stop playback */
          snd_pcm_uframes_t silence_threshold;	/* - pre-fill buffer with silence */
          snd_pcm_uframes_t silence_size;       /* max size of silence pre-fill; when >= boundary,
                                                 * fill played area with silence immediately */
          snd_pcm_uframes_t boundary;	/* pointers wrap point */

          /* internal data of auto-silencer */
          snd_pcm_uframes_t silence_start; /* starting pointer to silence area */
          snd_pcm_uframes_t silence_filled; /* size filled with silence */

          snd_pcm_sync_id_t sync;		/* hardware synchronization ID */

          /* -- mmap -- */
          volatile struct snd_pcm_mmap_status *status;
          volatile struct snd_pcm_mmap_control *control;
          atomic_t mmap_count;

          /* -- locking / scheduling -- */
          spinlock_t lock;
          wait_queue_head_t sleep;
          struct timer_list tick_timer;
          struct fasync_struct *fasync;

          /* -- private section -- */
          void *private_data;
          void (*private_free)(struct snd_pcm_runtime *runtime);

          /* -- hardware description -- */
          struct snd_pcm_hardware hw;
          struct snd_pcm_hw_constraints hw_constraints;

          /* -- timer -- */
          unsigned int timer_resolution;	/* timer resolution */

          /* -- DMA -- */           
          unsigned char *dma_area;	/* DMA area */
          dma_addr_t dma_addr;		/* physical bus address (not accessible from main CPU) */
          size_t dma_bytes;		/* size of DMA area */

          struct snd_dma_buffer *dma_buffer_p;	/* allocated buffer */

  #if defined(CONFIG_SND_PCM_OSS) || defined(CONFIG_SND_PCM_OSS_MODULE)
          /* -- OSS things -- */
          struct snd_pcm_oss_runtime oss;
  #endif
  };


```

对于每个声音驱动的运算符（回调），这些记录大多应是只读的。只PCM 中间层会改变
/ 更新它们。例外是硬件描述（hw）、DMA 缓冲区信息以及私有数据。此外，如果你使
标准的托管缓冲区分配模式，你不需要自己设DMA 缓冲区信息

在下面的小节中，将解释重要的记录

#### 硬件描述


硬件描述符（struct snd_pcm_hardware）包含了基本硬件配置的定义。最重要的是
你需要在 `PCM open callback`_ 中定义它。注意，运行时实例保存的是该描述符的
副本，而不是指向现有描述符的指针。也就是说，open 回调中，你可以根据需要修
被复制的描述符（`runtime->hw`）。例如，如果某些芯片型号的最大通道数只1
你仍然可以使用相同的

```

          struct snd_pcm_runtime *runtime = substream->runtime;
          ...
          runtime->hw = snd_mychip_playback_hw; /* common definition */
          if (chip->model == VERY_OLD_ONE)
                  runtime->hw.channels_max = 1;

```

```

  static struct snd_pcm_hardware snd_mychip_playback_hw = {
          .info = (SNDRV_PCM_INFO_MMAP |
                   SNDRV_PCM_INFO_INTERLEAVED |
                   SNDRV_PCM_INFO_BLOCK_TRANSFER |
                   SNDRV_PCM_INFO_MMAP_VALID),
          .formats =          SNDRV_PCM_FMTBIT_S16_LE,
          .rates =            SNDRV_PCM_RATE_8000_48000,
          .rate_min =         8000,
          .rate_max =         48000,
          .channels_min =     2,
          .channels_max =     2,
          .buffer_bytes_max = 32768,
          .period_bytes_min = 4096,
          .period_bytes_max = 32768,
          .periods_min =      1,
          .periods_max =      1024,
  };

```

- `info` 字段包含这个 PCM 的类型与能力。位标志`<sound/asound.h>` 
   定义`SNDRV_PCM_INFO_XXX`。这里你至少必须指定是否支持 mmap 以及支持
   哪种交错（interleaving）格式。当硬件支持 mmap 时，在这里添
   `SNDRV_PCM_INFO_MMAP` 标志。当硬件支持交错或非交错格式时，必须分别设置
   `SNDRV_PCM_INFO_INTERLEAVED` 鎴?`SNDRV_PCM_INFO_NONINTERLEAVED`
   标志。如果两者都支持，你也可以同时设置两者

   在上面的例子中，OSS mmap 模式指定`MMAP_VALID` `BLOCK_TRANSFER`
   通常两者都会设置。当然，`MMAP_VALID` 只有mmap 真正受支持时才设置

   其他可能的标志是 `SNDRV_PCM_INFO_PAUSE` `SNDRV_PCM_INFO_RESUME`
   `PAUSE` 位表PCM 支持“暂停”操作，`RESUME` 位表PCM 支持完整
   “挂恢复（suspend/resume）”操作。如果设置了 `PAUSE` 标志，则下面
   `trigger` 回调必须处理相应的（暂停推入/释放）命令。即使没`RESUME`
   标志，也可以定义挂起/恢复触发命令。详`Power Management`_ 一节

   PCM 子流可以同步时（典型情况是播放流和捕获流的同步启停止），你也可以
   给出 `SNDRV_PCM_INFO_SYNC_START`。在这种情况下，你需要在 trigger 回调
   检PCM 子流的链表。这将在后面的一节中描述

- `formats` 字段包含受支持格式的位标志（`SNDRV_PCM_FMTBIT_XXX`）。如果硬
   支持多种格式，请给出所有按位或（or）后的位。在上面的例子中，指定了有符
   16 位小端（little-endian）格式

- `rates` 字段包含受支持速率的位标志（`SNDRV_PCM_RATE_XXX`）。当芯片支持
   连续速率时，额外传入 `CONTINUOUS` 位。预定义的速率位仅针对典型速率提供
   如果你的芯片支持非标准的速率，你需要添`KNOT` 位并手动设置硬件约束
   （后文解释）

- `rate_min` `rate_max` 定义最小和最大采样率。它应当在某种程度对应于
   `rates` 位

- `channels_min` `channels_max` 定义了你可能已经预料到的、通道的最小和
   最大数量

- `buffer_bytes_max` 定义缓冲区的最大大小（以字节计）。没
   `buffer_bytes_min` 字段，因为它可以从最小周期大小和最小时期数计算出来
   同时，`period_bytes_min` `period_bytes_max` 定义了周期（period）的
   最小和最大大小（以字节计）。`periods_max` `periods_min` 定义了缓冲区
   周期的最大和最小数量

   “周期（period）”这个词对应OSS 世界中的碎片（fragment）。周期定义了生成
   PCM 中断的点。这个点强烈依赖于硬件。一般来说，较小的周期大小会给你更多
   中断，从而能够及时地填充/排空缓冲区。在捕获的情况下，这个大小定义了输入
   延迟。另一方面，整个缓冲区大小定义了播放方向的输出延迟

- 还有一`fifo_size` 字段。它指定硬件 FIFO 的大小，但目前它既不被驱
   使用，也不在 alsa-lib 中使用。因此，你可以忽略这个字段

#### PCM 配置


好，让我们再次回PCM 运行时记录。运行时实例中最常被引用的记录是 PCM 配置
PCM 配置是在应用程序通过 alsa-lib 发`hw_params` 数据之后，存储在运行
实例中的。有许多字段是从 hw_params sw_params 结构复制过来的。例如，
`format` 保存着应用程序选择的格式类型。该字段包含枚举
`SNDRV_PCM_FORMAT_XXX`銆。

需要注意的一点是，配置好的缓冲区和周期大小在运行时中以“帧（frames）”存储。在
ALSA 世界中，``1 = 通道× 样本大小``。为了在帧和字节之间转换，你可以使用
`frames_to_bytes()` 以及

```

  period_bytes = frames_to_bytes(runtime, runtime->period_size);

```

此外，许多软件参数（sw_params）也以帧存储。请检查字段的类型
`snd_pcm_uframes_t` 用于无符号整数形式的帧，`snd_pcm_sframes_t` 用于
有符号整数形式的帧

#### DMA 缓冲区信


DMA 缓冲区由以下四个字段定义：`dma_area`、`dma_addr`、`dma_bytes` 
`dma_private`。`dma_area` 保存缓冲区指针（逻辑地址）。你可以对这个指针调
`memcpy()`。同时，`dma_addr` 保存缓冲区的物理地址。该字段仅在缓冲区是线
缓冲区时才指定。`dma_bytes` 保存缓冲区的大小（以字节计）。`dma_private` 用于
ALSA DMA 分配器

如果你使用托管缓冲区分配模式或标API 函数 `snd_pcm_lib_malloc_pages()`
来分配缓冲区，这些字段由 ALSA 中间层设置，*不应**自己修改它们。你可以读取
它们但不能写入它们。另一方面，如果你想自己分配缓冲区，你需要在 hw_params 回调
管理它。至少，`dma_bytes` 是必需的。`dma_area` 在缓冲区mmap 时是必需的
如果你的驱动不支mmap，这个字段就不是必需的。`dma_addr` 也是可选的。你也可
随意使用 dma_private

#### 运行状


运行状态可以通过 `runtime->status` 引用。这是一个指struct
snd_pcm_mmap_status 记录的指针。例如，你可以通过 `runtime->status->hw_ptr`
获取当前DMA 硬件指针

DMA 应用指针可以通过 `runtime->control` 引用，它指向一struct
snd_pcm_mmap_control 记录。但是，不建议直接访问这个值

#### 私有数据


你可以为子流分配一个记录并将其存储`runtime->private_data` 中。通常，这
是在 `PCM open callback`_ 中完成的。不要将它与 `pcm->private_data` 混淆
`pcm->private_data` 通常指向PCM 设备创建时静态分配的芯片实例，
`runtime->private_data` 指向PCM open 回调中创建的动态数据结构：

```

  static int snd_xxx_open(struct snd_pcm_substream *substream)
  {
          struct my_pcm_data *data;
          ....
          data = kmalloc(sizeof(*data), GFP_KERNEL);
          substream->runtime->private_data = data;
          ....
  }


```

所分配的对象必须在 `close callback`_ 中被释放

### 杩愮畻绗。


好，现在让我给出每个 PCM 回调（`ops`）的细节。一般来说，每个回调在成功时必须
返回 0，或者返回一个负的错误号，例`-EINVAL`。要选择合适的错误号，建议检
当同一类请求失败时内核其他部分返回什么值

每个回调函数至少接受一个包struct snd_pcm_substream 指针的参数。要从给定的
子流实例中取回芯片记录，你可以使用以下方法：

```

  int xxx(...) {
          struct mychip *chip = snd_pcm_substream_chip(substream);
          ....
  }

```

这个宏读`substream->private_data`，它`pcm->private_data` 的副本
如果需要为每个 PCM 子流分配不同的数据记录，你可以覆盖前者。例如，cmi8330 驱动
为播放和捕获方向分配了不同的 `private_data`，因为它对不同的方向使用两个不同
编解码器（SB 兼容AD 兼容）

#### PCM open 回调


```

  static int snd_xxx_open(struct snd_pcm_substream *substream);

```

PCM 子流被打开时调用

至少，在这里你必须初始化 `runtime->hw`

```

  static int snd_xxx_open(struct snd_pcm_substream *substream)
  {
          struct mychip *chip = snd_pcm_substream_chip(substream);
          struct snd_pcm_runtime *runtime = substream->runtime;

          runtime->hw = snd_mychip_playback_hw;
          return 0;
  }

```

其中 `snd_mychip_playback_hw` 是预定义的硬件描述

你可以在这个回调中分配私有数据，`Private Data`_ 一节所述

如果硬件配置需要更多约束，也要在这里设置硬件约束。详Constraints_

#### close 回调


```

  static int snd_xxx_close(struct snd_pcm_substream *substream);


```

显然，当 PCM 子流被关闭时调用

任何`open` 回调中为 PCM 子流分配的私有实例必须在这里释放

```

  static int snd_xxx_close(struct snd_pcm_substream *substream)
  {
          ....
          kfree(substream->runtime->private_data);
          ....
  }

```

#### ioctl 回调


这用于任何对 PCM ioctl 的特殊调用。但通常你可以将其保留为 NULL，然PCM 核心
会调用通用 ioctl 回调函数 `snd_pcm_lib_ioctl()`。如果你需要处理通道信息
重置过程的独特设置，可以在这里传入你自己的回调函数

#### hw_params 回调


```

  static int snd_xxx_hw_params(struct snd_pcm_substream *substream,
                               struct snd_pcm_hw_params *hw_params);

```

当应用程序设置了硬件参数（`hw_params`）时调用，也就是当缓冲区大小、周期大小
格式等被PCM 子流定义好时调用一次

许多硬件设置应该在这个回调中完成，包括缓冲区的分配

要初始化的参数通过 `params_xxx()` 宏获取

当你为子流选择托管缓冲区分配模式时，在该回调被调用之前缓冲区就已经被分配好了
或者，你可以调用下面的辅助函数

```

  snd_pcm_lib_malloc_pages(substream, params_buffer_bytes(hw_params));

```

`snd_pcm_lib_malloc_pages()` 只有DMA 缓冲区已被预分配时才可用。详
`Buffer Types`_ 一节

注意，这个回调和 `prepare` 回调可能在每次初始化时被多次调用。例如，OSS 模拟
可能在其 ioctl 的每次变更中调用这些回调

因此，你需要注意不要多次分配相同的缓冲区，那会导致内存泄漏！多次调用上面的辅助
函数是没问题的。如果缓冲区之前已经分配过，它会自动释放之前的缓冲区

另一个注意点是，默认情况下这个回调是非原子的（可调度），即当没有设置 `nonatomic`
标志时。这很重要，因为 `trigger` 回调是原子的（不可调度）。也就是说，
`trigger` 回调中不能使用互斥体或任何与调度相关的函数。详Atomicity_ 小节

#### hw_free 回调


```

  static int snd_xxx_hw_free(struct snd_pcm_substream *substream);

```

这用于释放通过 `hw_params` 分配的资源

这个函数总是close 回调函数被调用之前被调用。此外，该回调也可能被多次调用
请跟踪每个资源是否已经被释放

当你PCM 子流选择了托管缓冲区分配模式时，分配PCM 缓冲区将在该回调被调用后
自动释放。否则你将不得不手动释放缓冲区。典型的做法是，当缓冲区是从预分配池
分配时，你可以使用标API 函数

```

  snd_pcm_lib_free_pages(substream);

```

#### prepare 回调


```

  static int snd_xxx_prepare(struct snd_pcm_substream *substream);

```

PCM “准备好（prepared）”时调用这个回调。你可以在这里设置格式类型、采样率等
`hw_params` 的区别在于，`prepare` 回调会在每次调用 `snd_pcm_prepare()`
时被调用，即在欠载（underrun）等恢复之后

注意这个回调是非原子的。你可以在这个回调中安全地使用与调度相关的函数

在这个以及后续回调中，你可以通过运行时记`substream->runtime` 引用值
例如，要获取当前的速率、格式或通道，分别访`runtime->rate`
`runtime->format` `runtime->channels`。已分配缓冲区的物理地址被设置为
`runtime->dma_area`。缓冲区和周期大小分别在 `runtime->buffer_size` 
`runtime->period_size` 中

注意这个回调在每次设置时也会被调用多次

#### trigger 回调


```

  static int snd_xxx_trigger(struct snd_pcm_substream *substream, int cmd);

```

PCM 被启动、停止或暂停时调用

动作在第二个参数中指定，`<sound/pcm.h>` 中定义的
`SNDRV_PCM_TRIGGER_XXX`。至少，`START`

```

  switch (cmd) {
  case SNDRV_PCM_TRIGGER_START:
          /* do something to start the PCM engine */
          break;
  case SNDRV_PCM_TRIGGER_STOP:
          /* do something to stop the PCM engine */
          break;
  default:
          return -EINVAL;
  }

```

如果 PCM 支持暂停操作（在硬件表的 info 字段中给出），则 `PAUSE_PUSH` 
`PAUSE_RELEASE` 命令也必须在这里处理。前者是暂停 PCM 的命令，后者是重新启动
PCM 的命令

PCM 支持挂起/恢复操作时，无论是否支持完整或部分的挂起/恢复，都必须处理
`SUSPEND` `RESUME` 命令。这些命令在电源管理状态改变时发出。显然，
`SUSPEND` `RESUME` 命令分别挂起和恢PCM 子流，通常它们分别等同
`STOP` `START` 命令。详`Power Management`_ 一节

如前所述，除非设置`nonatomic` 标志，否则这个回调默认是原子的，你不能调
可能休眠的函数。`trigger` 回调应当尽可能精简，仅仅真正触DMA。其他部分应
预先`hw_params` `prepare` 回调中正确初始化

#### sync_stop 回调


```

  static int snd_xxx_sync_stop(struct snd_pcm_substream *substream);

```

这个回调是可选的，可以传NULL。它PCM 核心停止流之后、在它通过 `prepare`
`hw_params` `hw_free` 改变流状态之前被调用。由IRQ 处理函数可能仍在
挂起，我们需要等待挂起的任务完成，然后再进入下一步；否则可能会由于资源冲突或
访问已释放资源而导致崩溃。典型的行为是在这里调用`synchronize_irq()` 这样
的同步函数

对于只需要调`synchronize_irq()` 的大多数驱动，也有一个更简单的设置。在保持
`sync_stop` PCM 回调NULL 的同时，驱动可以在请IRQ 之后`card->sync_irq`
字段设置为返回的中断号。然PCM 核心会用给定IRQ 适当地调
`synchronize_irq()`銆。

如果 IRQ 处理函数由声卡析构函数释放，你不需要清`card->sync_irq`，因为声
本身正在被释放。所以，通常你只需要在驱动代码中添加一行来赋`card->sync_irq`
除非驱动重新获取 IRQ。当驱动动态释放并重新获取 IRQ（例如在挂起/恢复时），它需
再次适当地清除并重新设置 `card->sync_irq`

#### pointer 回调


```

  static snd_pcm_uframes_t snd_xxx_pointer(struct snd_pcm_substream *substream)

```

PCM 中间层查询缓冲区中的当前硬件位置时调用这个回调。位置必须以帧的形式返回
范围0 `buffer_size - 1`

这通常PCM 中间层的缓冲区更新例程中调用，该例程在中断例程调
`snd_pcm_period_elapsed()` 时被调用。然PCM 中间层更新位置并计算可用空间
并唤醒睡眠的 poll 线程等

默认情况下这个回调也是原子的

#### copy fill_silence 运算


这些回调不是强制的，在大多数情况下可以省略。当硬件缓冲区不在正常的存储空间时，
使用这些回调。某些芯片在硬件中有自己的缓冲区，该缓冲区不可映射。在这种情况下，
必须手动将数据从内存缓冲区传输到硬件缓冲区。或者，如果缓冲区在物理和虚拟内存空
上都是非连续的，也必须定义这些回调

如果定义了这两个回调，复制和填充静音（set-silence）操作由它们完成。细节将在后
`Buffer and Memory Management`_ 一节中描述

#### ack 回调


这个回调也不是强制的。当在读取或写入操作`appl_ptr` 被更新时调用这个回调
某些驱动emu10k1-fx cs46xx 需要为内部缓冲区跟踪当前的 `appl_ptr`，这
回调仅对此类用途有用

回调函数可以返回 0 或负的错误。当返回值为 `-EPIPE` 时，PCM 核心将其视为缓冲
XRUN，并自动将状态更改为 `SNDRV_PCM_STATE_XRUN`

默认情况下这个回调是原子的

#### page 回调


这个回调也是可选的。mmap 调用这个回调来获取缺页地址

对于标准SG 缓冲区或 vmalloc 缓冲区，你不需要特殊的回调。因此这个回调应该很
使用

#### mmap 回调


这是另一个用于控mmap 行为的可选回调。当定义了它时，PCM 核心在内存被映射时会
调用这个回调，而不是使用标准辅助函数。如果你需要特殊处理（由于某些架构或设
特定的问题），可以像你喜欢的那样在这里实现所有内容


### PCM 中断处理函数


PCM 剩余的部分是 PCM 中断处理函数。声音驱动中 PCM 中断处理函数的角色是更新
缓冲区位置，并在缓冲区位置越过指定的周期边界时通知 PCM 中间层。为此，调用
`snd_pcm_period_elapsed()` 函数

声音芯片生成中断的方式有几种

#### 在周期（碎片）边界处的中


这是最常见的类型：硬件在每个周期边界处生成一个中断。在这种情况下，你可以在每次
中断时调`snd_pcm_period_elapsed()`

`snd_pcm_period_elapsed()` 以子流指针作为其参数。因此你需要保持子流指针可
芯片实例访问。例如，在芯片记录中定义 `substream` 字段来保存当前运行的子流指针
并在 `open` 回调中设置该指针值（`close` 回调中重置）

如果你在中断处理函数中获取了自旋锁，并且该锁也在其他 PCM 回调中使用，那么你必须在
调用 `snd_pcm_period_elapsed()` 之前释放该锁，因`snd_pcm_period_elapsed()`
会在内部调用其他 PCM 回调

```


      static irqreturn_t snd_mychip_interrupt(int irq, void *dev_id)
      {
              struct mychip *chip = dev_id;
              spin_lock(&chip->lock);
              ....
              if (pcm_irq_invoked(chip)) {
                      /* call updater, unlock before it */
                      spin_unlock(&chip->lock);
                      snd_pcm_period_elapsed(chip->substream);
                      spin_lock(&chip->lock);
                      /* acknowledge the interrupt if necessary */
              }
              ....
              spin_unlock(&chip->lock);
              return IRQ_HANDLED;
      }

```

此外，当设备可以检测到缓冲区欠溢出（underrun/overrun）时，驱动可以通过调用
`snd_pcm_stop_xrun()` XRUN 状态通知PCM 核心。这个函数停止流并将 PCM 状
设置`SNDRV_PCM_STATE_XRUN`。注意它必须PCM 流锁之外调用，因此无法从原子
回调中调用


#### 高频定时器中


当硬件不在周期边界处生成中断，而是以固定的定时器速率发出定时器中断时（例es1968
ymfpci 驱动），会发生这种情况。在这种情况下，你需要检查当前的硬件位置并在每次
中断时累加已处理的样本长度。当累加的大小超过周期大小时，调
`snd_pcm_period_elapsed()` 并重置累加器

```


      static irqreturn_t snd_mychip_interrupt(int irq, void *dev_id)
      {
              struct mychip *chip = dev_id;
              spin_lock(&chip->lock);
              ....
              if (pcm_irq_invoked(chip)) {
                      unsigned int last_ptr, size;
                      /* get the current hardware pointer (in frames) */
                      last_ptr = get_hw_ptr(chip);
                      /* calculate the processed frames since the
                       * last update
                       */
                      if (last_ptr < chip->last_ptr)
                              size = runtime->buffer_size + last_ptr
                                       - chip->last_ptr;
                      else
                              size = last_ptr - chip->last_ptr;
                      /* remember the last updated point */
                      chip->last_ptr = last_ptr;
                      /* accumulate the size */
                      chip->size += size;
                      /* over the period boundary? */
                      if (chip->size >= runtime->period_size) {
                              /* reset the accumulator */
                              chip->size %= runtime->period_size;
                              /* call updater */
                              spin_unlock(&chip->lock);
                              snd_pcm_period_elapsed(substream);
                              spin_lock(&chip->lock);
                      }
                      /* acknowledge the interrupt if necessary */
              }
              ....
              spin_unlock(&chip->lock);
              return IRQ_HANDLED;
      }



```

#### 关于调用 :c:func:`snd_pcm_period_elapsed()`


在两种情况下，即使已经过了不止一个周期，你也不必多次调用
`snd_pcm_period_elapsed()`。只调用一次。PCM 层会检查当前的硬件指针并更新到
最新状态

### 鍘熷瓙鎬。


内核编程中最重要（也因此最难调试）的问题之一是竞态条件（race conditions）。在
Linux 内核中，它们通常通过自旋锁、互斥体或信号量来避免。一般来说，如果竞态条
可能发生在中断处理函数中，它必须以原子方式处理，你必须使用自旋锁来保护临界区
如果临界区不在中断处理函数代码中，并且执行较长时间是可以接受的，你应该使用互斥体
或信号量

如已经看到的，某PCM 回调是原子的，某些则不是。例如，`hw_params` 回调
非原子的，`trigger` 回调是原子的。这意味着，后者已经在PCM 中间层持有的
自旋锁（PCM 流锁）中被调用。在为回调选择加锁方案时，请考虑这种原子性

在原子回调中，你不能使用可能调用 `schedule()` 或进`sleep()` 的函数
信号量和互斥体可能会休眠，因此它们不能用于原子回调内部（例如 `trigger` 回调）
要在此类回调中实现某种延迟，请使`udelay()` `mdelay()`

所有三个原子回调（trigger、pointer ack）都在本地中断被禁用的状态下调用

然而，可以请求所PCM 操作为非原子的。这假设所有调用点都处于非原子上下文。例如，
`snd_pcm_period_elapsed()` 通常从中断处理函数调用。但是，如果你将驱动设置为使
线程化中断处理函数，这个调用也可以处于非原子上下文中。在这种情况下，你可以在创建
struct snd_pcm 对象之后设置它的 `nonatomic` 字段。当设置了这个标志时，PCM 核心
内部使用互斥体和 rwsem 代替自旋锁和 rwlocks，这样你就可以在非原子上下文中安全地
调用所PCM 函数

此外，在某些情况下，你可能需要在原子上下文中调用 `snd_pcm_period_elapsed()`
（例如，周期`ack` 或其他回调期间过去）。为此也有一个可以在 PCM 流锁内部调用
的变`snd_pcm_period_elapsed_under_stream_lock()`

### 约束


由于物理限制，硬件不是无限可配置的。这些限制通过设置的约束来表达

例如，为了将采样率限制为某些受支持的值，使用 `snd_pcm_hw_constraint_list()`
你需

```

      static unsigned int rates[] =
              {4000, 10000, 22050, 44100};
      static struct snd_pcm_hw_constraint_list constraints_rates = {
              .count = ARRAY_SIZE(rates),
              .list = rates,
              .mask = 0,
      };

      static int snd_mychip_pcm_open(struct snd_pcm_substream *substream)
      {
              int err;
              ....
              err = snd_pcm_hw_constraint_list(substream->runtime, 0,
                                               SNDRV_PCM_HW_PARAM_RATE,
                                               &constraints_rates);
              if (err < 0)
                      return err;
              ....
      }

```

有许多不同的约束。查`sound/pcm.h` 获取完整列表。你甚至可以定义自己的约束规则
例如，假my_chip 当且仅当格式`S16_LE` 时才能管1 通道的子流，否则它支
struct snd_pcm_hardware（或任何其他）中指定的任何格式：

```

      static int hw_rule_channels_by_format(struct snd_pcm_hw_params *params,
                                            struct snd_pcm_hw_rule *rule)
      {
              struct snd_interval *c = hw_param_interval(params,
                            SNDRV_PCM_HW_PARAM_CHANNELS);
              struct snd_mask *f = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
              struct snd_interval ch;

              snd_interval_any(&ch);
              if (f->bits[0] == SNDRV_PCM_FMTBIT_S16_LE) {
                      ch.min = ch.max = 1;
                      ch.integer = 1;
                      return snd_interval_refine(c, &ch);
              }
              return 0;
      }


```

```

  snd_pcm_hw_rule_add(substream->runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS,
                      hw_rule_channels_by_format, NULL,
                      SNDRV_PCM_HW_PARAM_FORMAT, -1);

```

当应用程序设PCM 格式时调用规则函数，并相应地细化通道数量。但应用程序可能在设
格式之前设置通道数量。因此你还需

```

      static int hw_rule_format_by_channels(struct snd_pcm_hw_params *params,
                                            struct snd_pcm_hw_rule *rule)
      {
              struct snd_interval *c = hw_param_interval(params,
                    SNDRV_PCM_HW_PARAM_CHANNELS);
              struct snd_mask *f = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
              struct snd_mask fmt;

              snd_mask_any(&fmt);    /* Init the struct */
              if (c->min < 2) {
                      fmt.bits[0] &= SNDRV_PCM_FMTBIT_S16_LE;
                      return snd_mask_refine(f, &fmt);
              }
              return 0;
      }


```

```

  snd_pcm_hw_rule_add(substream->runtime, 0, SNDRV_PCM_HW_PARAM_FORMAT,
                      hw_rule_format_by_channels, NULL,
                      SNDRV_PCM_HW_PARAM_CHANNELS, -1);

```

hw 约束的一个典型用途是将缓冲区大小与周期大小对齐。默认情况下，ALSA PCM 核心
不强制缓冲区大小为周期大小的整数倍。例如，可能会出256 周期字节搭配 999 缓冲
字节这样的组合

然而，许多设备芯片要求缓冲区是周期数的整数倍。在这种情况下，调用
`snd_pcm_hw_constraint_integer()` 用于

```

  snd_pcm_hw_constraint_integer(substream->runtime,
                                SNDRV_PCM_HW_PARAM_PERIODS);

```

这确保了周期的数量是整数，因此缓冲区大小与周期大小对齐

hw 约束是定义首PCM 配置的一个非常强大的机制，并且有相关的辅助函数。我在这
不给出更多细节，而是想说，“Luke，使用源码（use the source）。

## 控制接口


### 概述


控制接口被广泛用于许多开关、滑块等，这些可从用户空间访问。它最重要的用途是混音
（mixer）接口。换句话说，ALSA 0.9.x 起，所有混音器相关的内容都实现在控制内
API 上

ALSA 有一个定义良好的 AC97 控制模块。如果你的芯片只支持 AC97 而没有其他东西，
你可以跳过本节

控制 API 定义`<sound/control.h>` 中。如果你想添加自己的控制，请包含这个文件

### 控件的定


要创建一个新的控件，你需要定义以下三个回调：`info`、`get` `put`。然后，
定义一

```


      static struct snd_kcontrol_new my_control = {
              .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
              .name = "PCM Playback Switch",
              .index = 0,
              .access = SNDRV_CTL_ELEM_ACCESS_READWRITE,
              .private_value = 0xffff,
              .info = my_control_info,
              .get = my_control_get,
              .put = my_control_put
      };


```

`iface` 字段指定控件类型 `SNDRV_CTL_ELEM_IFACE_XXX`，通常`MIXER`
对不属于混音器逻辑部分的全局控件使用 `CARD`。如果控件与声卡上某个特定设备密切相关，
使用 `HWDEP`、`PCM`、`RAWMIDI`、`TIMER` `SEQUENCER`，并`device` 
`subdevice` 字段指定设备号

`name` 是名称标识符字符串。自 ALSA 0.9.x 起，控件名称非常重要，因为它的角色是
从名称中分类出来的。有预定义的标准控件名称。细节在 `Control Names`_ 小节中描述

`index` 字段保存这个控件的索引号。如果有几个名称不同的控件，可以通过索引号来区分
当声卡上存在多个编解码器时就是这种情况。如果索引为零，你可以省略上面的定义

`access` 字段包含这个控件的访问类型。在这里给出位掩码的组合
`SNDRV_CTL_ELEM_ACCESS_XXX`。细节将`Access Flags`_ 小节中解释

`private_value` 字段包含这个记录的任意长整型值。当使用通用`info`、`get`
`put` 回调时，你可以通过这个字段传值。如果需要几个小数字，你可以将它们按
组合。或者，也可以在这个字段中存储某个记录的指针（转换为 unsigned long）

`tlv` 字段可用于提供关于控件的元数据；`Metadata`_ 小节

其他三个`Control Callbacks`_

### 控件名称


定义控件名称有一些标准。一个控件通常由三部分定义为“源 方向 功能（SOURCE
DIRECTION FUNCTION）”

第一，`SOURCE`，指定控件的源，是一个如“Master”、“PCM”、“CD”和“Line”这样的字符串
有许多预定义的源

第二，`DIRECTION`，根据控件的方向，是以下字符串之一：“Playback”、“Capture”
“Bypass Playback”和“Bypass Capture”。或者，可以省略，意味着播放和捕获两个方向

第三，`FUNCTION`，根据控件的功能，是以下字符串之一：“Switch”、“Volume”和
鈥淩oute鈥濄€?

因此，控件名称的例子有“Master Capture Switch”或“PCM Playback Volume”

有一些例外：

#### 全局捕获与播


“Capture Source”、“Capture Switch”和“Capture Volume”用于全局捕获（输入）源
开关和音量。类似地，“Playback Switch”和“Playback Volume”用于全局输出增益开关和
音量

#### 音调控制


音调控制开关和音量指定为“Tone Control - XXX”，例如“Tone Control - Switch”
“Tone Control - Bass”、“Tone Control - Center”

#### 3D 控制


3D 控制开关和音量指定为D Control - XXX”，例如D Control - Switch”、D
Control - Center”、D Control - Space”

#### Mic 增益


Mic-boost 开关设置为“Mic Boost”或“Mic Boost (6dB)”

更精确的信息可以`Documentation/sound/designs/control-names.rst` 中找到

### 访问标志


访问标志是指定位掩码，它指定给定控件的访问类型。默认访问类型是
`SNDRV_CTL_ELEM_ACCESS_READWRITE`，这意味着允许对该控件进行读和写。当访问标志
被省略（= 0）时，默认被视为 `READWRITE` 访问

当控件是只读时，改为传入 `SNDRV_CTL_ELEM_ACCESS_READ`。在这种情况下，你不必定
`put` 回调。类似地，当控件是只写时（尽管这种情况很少见），你可以使`WRITE`
标志，并且不需`get` 回调

如果控件值频繁变化（例如 VU 表），应给出 `VOLATILE` 标志。这意味着该控件可能会
在没`Change notification`_ 的情况下被改变。应用程序应当持续轮询这样的控件

当控件可能被更新，但当前对任何东西都没有影响时，设置 `INACTIVE` 标志可能是合适的
例如，当没有 PCM 设备打开时，PCM 控件应当是不活跃的

`LOCK` `OWNER` 标志可以改变写权限

### 控件回调


#### info 回调


`info` 回调用于获取关于这个控件的详细信息。它必须存储给定struct
snd_ctl_elem_info 对象的值。例如，

```


      static int snd_myctl_mono_info(struct snd_kcontrol *kcontrol,
                              struct snd_ctl_elem_info *uinfo)
      {
              uinfo->type = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
              uinfo->count = 1;
              uinfo->value.integer.min = 0;
              uinfo->value.integer.max = 1;
              return 0;
      }



```

`type` 字段指定控件的类型。有 `BOOLEAN`、`INTEGER`、`ENUMERATED`、`BYTES`
`IEC958` `INTEGER64`。`count` 字段指定这个控件中元素的数量。例如，立体
音量会有 count = 2。`value` 字段是一个联合体，存储的值取决于类型。布尔和整数
类型是相同的

枚举类型与其他类型略有不同。你需

```

  static int snd_myctl_enum_info(struct snd_kcontrol *kcontrol,
                          struct snd_ctl_elem_info *uinfo)
  {
          static char *texts[4] = {
                  "First", "Second", "Third", "Fourth"
          };
          uinfo->type = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
          uinfo->count = 1;
          uinfo->value.enumerated.items = 4;
          if (uinfo->value.enumerated.item > 3)
                  uinfo->value.enumerated.item = 3;
          strcpy(uinfo->value.enumerated.name,
                 texts[uinfo->value.enumerated.item]);
          return 0;
  }

```

上面的回调可以用辅助函数 `snd_ctl_enum_info()` 简化。最终代码如下所示
（你可以在第三个参数中传`ARRAY_SIZE(texts)` 而不4；这看个人喜好。）

```

  static int snd_myctl_enum_info(struct snd_kcontrol *kcontrol,
                          struct snd_ctl_elem_info *uinfo)
  {
          static char *texts[4] = {
                  "First", "Second", "Third", "Fourth"
          };
          return snd_ctl_enum_info(uinfo, 1, 4, texts);
  }


```

一些常见的 info 回调可供你方便使用：`snd_ctl_boolean_mono_info()` 
`snd_ctl_boolean_stereo_info()`。显然，前者是单声道布尔项info 回调，就像上面的
`snd_myctl_mono_info()`，后者是立体声布尔项info 回调

#### get 回调


这个回调用于读取控件的当前值，以便它可以返回给用户空间

```

      static int snd_myctl_get(struct snd_kcontrol *kcontrol,
                               struct snd_ctl_elem_value *ucontrol)
      {
              struct mychip *chip = snd_kcontrol_chip(kcontrol);
              ucontrol->value.integer.value[0] = get_some_value(chip);
              return 0;
      }



```

`value` 字段取决于控件类型以info 回调。例如，sb 驱动使用这个字段来存储寄存器
偏移、位移和位掩码。`private_value`

```

  .private_value = reg | (shift << 16) | (mask << 24)

```

```

  static int snd_sbmixer_get_single(struct snd_kcontrol *kcontrol,
                                    struct snd_ctl_elem_value *ucontrol)
  {
          int reg = kcontrol->private_value & 0xff;
          int shift = (kcontrol->private_value >> 16) & 0xff;
          int mask = (kcontrol->private_value >> 24) & 0xff;
          ....
  }

```

`get` 回调中，如果控件有多个元素（`count > 1`），你必须填充所有元素
在上面的例子中，由于假设 `count = 1`，我们只填充了一个元
（`value.integer.value[^0^]`）

#### put 回调


这个回调用于写入来自用户空间的值

```

      static int snd_myctl_put(struct snd_kcontrol *kcontrol,
                               struct snd_ctl_elem_value *ucontrol)
      {
              struct mychip *chip = snd_kcontrol_chip(kcontrol);
              int changed = 0;
              if (chip->current_value !=
                   ucontrol->value.integer.value[0]) {
                      change_current_value(chip,
                                  ucontrol->value.integer.value[0]);
                      changed = 1;
              }
              return changed;
      }



```

如上所示，如果值改变了，你必须返回 1。如果值没有改变，则返0。如果发生任何致
错误，像往常一样返回负的错误码

`get` 回调一样，当控件有多个元素时，所有元素也必须在这个回调中被求值

#### 回调不是原子


这三个回调都不是原子的

### 控件构造函


当一切就绪，我们终于可以创建一个新的控件。要创建一个控件，需要调用两个函数，
`snd_ctl_new1()` 鍜?`snd_ctl_add()`銆。

```

  err = snd_ctl_add(card, snd_ctl_new1(&my_control, chip));
  if (err < 0)
          return err;

```

其中 `my_control` 是上面定义的 struct snd_kcontrol_new 对象，chip 是要传递给
kcontrol->private_data 的对象指针，可以在回调中引用

`snd_ctl_new1()` 分配一个新struct snd_kcontrol 实例，`snd_ctl_add()` 将给
的控件组件分配给声卡

### 变更通知


如果你需要在中断例程中变更和更新一个控件，

```

  snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, id_pointer);

```

这个函数接受声卡指针、事件掩码以及用于通知的控id 指针。事件掩码指定通知的类型，
例如，在上面的例子中，通知控件值的改变。id 指针是要通知struct snd_ctl_elem_id
的指针。你可以`es1938.c` `es1968.c` 中找到硬件音量中断的一些例子

### 元数


要提供关于混音器控件 dB 值的信息，使`<sound/tlv.h>` 中的某个
`DECLARE_TLV_xxx` 宏来定义一个包含此信息的变量，`tlv.p` 字段设置为指向这
变量，并包含 `SNDRV_CTL_ELEM_ACCESS_TLV_READ` 标志

```

  static DECLARE_TLV_DB_SCALE(db_scale_my_control, -4050, 150, 0);

  static struct snd_kcontrol_new my_control = {
          ...
          .access = SNDRV_CTL_ELEM_ACCESS_READWRITE |
                    SNDRV_CTL_ELEM_ACCESS_TLV_READ,
          ...
          .tlv.p = db_scale_my_control,
  };


```

`DECLARE_TLV_DB_SCALE()` 宏定义关于一个混音器控件的信息，其中控件值的每一步变
都会以恒定的 dB 量改dB 值。第一个参数是要定义的变量名。第二个参数是最小值，
单位0.01 dB。第三个参数是步长，单位0.01 dB。如果最小值实际上会使控件静音
则将第四个参数设置为 1

`DECLARE_TLV_DB_LINEAR()` 宏定义关于一个混音器控件的信息，其中控件的值线性地影响
输出。第一个参数是要定义的变量名。第二个参数是最小值，单位0.01 dB。第三个参数
最大值，单位0.01 dB。如果最小值使控件静音，则将第二个参数设置
`TLV_DB_GAIN_MUTE`銆。

## AC97 编解码器API


### 概述


ALSA AC97 编解码器层是一个定义良好的层，你不必编写太多代码来控制它。只需要低
控制例程。AC97 编解码器 API 定义`<sound/ac97_codec.h>` 中

### 完整代码示例


```

      struct mychip {
              ....
              struct snd_ac97 *ac97;
              ....
      };

      static unsigned short snd_mychip_ac97_read(struct snd_ac97 *ac97,
                                                 unsigned short reg)
      {
              struct mychip *chip = ac97->private_data;
              ....
              /* read a register value here from the codec */
              return the_register_value;
      }

      static void snd_mychip_ac97_write(struct snd_ac97 *ac97,
                                       unsigned short reg, unsigned short val)
      {
              struct mychip *chip = ac97->private_data;
              ....
              /* write the given register value to the codec */
      }

      static int snd_mychip_ac97(struct mychip *chip)
      {
              struct snd_ac97_bus *bus;
              struct snd_ac97_template ac97;
              int err;
              static struct snd_ac97_bus_ops ops = {
                      .write = snd_mychip_ac97_write,
                      .read = snd_mychip_ac97_read,
              };

              err = snd_ac97_bus(chip->card, 0, &ops, NULL, &bus);
              if (err < 0)
                      return err;
              memset(&ac97, 0, sizeof(ac97));
              ac97.private_data = chip;
              return snd_ac97_mixer(bus, &ac97, &chip->ac97);
      }


```

### AC97 构造函


要创建一ac97 实例，首先调`snd_ac97_bus()`

```

  struct snd_ac97_bus *bus;
  static struct snd_ac97_bus_ops ops = {
        .write = snd_mychip_ac97_write,
        .read = snd_mychip_ac97_read,
  };

  snd_ac97_bus(card, 0, &ops, NULL, &pbus);

```

总线记录在所有从属的 ac97 实例之间共享

然后用一struct snd_ac97_template 调用 `snd_ac97_mixer()`

```

  struct snd_ac97_template ac97;
  int err;

  memset(&ac97, 0, sizeof(ac97));
  ac97.private_data = chip;
  snd_ac97_mixer(bus, &ac97, &chip->ac97);

```

其中 chip->ac97 是指向新创建`ac97_t` 实例的指针。在这种情况下，芯片指针
设置为私有数据，以便写回调函数可以引用这个芯片实例。这个实例不一定保存在芯片
记录中。如果你需要从驱动改变寄存器值，或者需ac97 编解码器的挂恢复，请保留
这个指针以传给相应的函数

### AC97 回调


标准的回调是 `read` `write`。显然它们对应于硬件低层代码的读和写访问函数

`read` 回调返回 `read` 回调指定的寄存器值：

```

  static unsigned short snd_mychip_ac97_read(struct snd_ac97 *ac97,
                                             unsigned short reg)
      {
              struct mychip *chip = ac97->private_data;
              ....
              return the_register_value;
      }

```

这里，chip 可以`ac97->private_data` 转换得到

同时，`write` 回调用于设置寄存

```

  static void snd_mychip_ac97_write(struct snd_ac97 *ac97,
                       unsigned short reg, unsigned short val)


```

这些回调与控API 回调一样是非原子的

还有其他回调：`reset`、`wait` `init`

`reset` 回调用于重置编解码器。如果芯片需要一种特殊的重置，你可以定义这个回调

`wait` 回调用于在编解码器的标准初始化中添加一些等待时间。如果芯片需要额外的等待
时间，定义这个回调

`init` 回调用于编解码器的额外初始化

### 在驱动中更新寄存


如果你需要从驱动访问编解码器，你可以调用以下函数：`snd_ac97_write()`
`snd_ac97_read()`、`snd_ac97_update()` `snd_ac97_update_bits()`

`snd_ac97_write()` `snd_ac97_update()` 函数都用于给给定寄存
（`AC97_XXX`）设置一个值。它们之间的区别在于，`snd_ac97_update()` 在给定值已
设置时不写入，`snd_ac97_write()`

```

  snd_ac97_write(ac97, AC97_MASTER, 0x8080);
  snd_ac97_update(ac97, AC97_MASTER, 0x8080);

```

`snd_ac97_read()` 用于读取给定

```

  value = snd_ac97_read(ac97, AC97_MASTER);

```

`snd_ac97_update_bits()` 用于更新某些位：

```

  snd_ac97_update_bits(ac97, reg, mask, value);

```

此外，还有一个函数在支持 VRA DRA 时改变采样率（针对给定的寄存器，例如
`AC97_PCM_FRONT_DAC_RATE`）：

```

  snd_ac97_set_rate(ac97, AC97_PCM_FRONT_DAC_RATE, 44100);


```

以下寄存器可用于设置速率：`AC97_PCM_MIC_ADC_RATE`、`AC97_PCM_FRONT_DAC_RATE`
`AC97_PCM_LR_ADC_RATE`、`AC97_SPDIF`。当指定 `AC97_SPDIF` 时，寄存器实际上
并没有被改变，而是相应IEC958 状态位会被更新

### 时钟调整


在某些芯片中，编解码器的时钟不是 48000，而是使用 PCI 时钟（以节省一个石英晶振！）
在这种情况下，将 `bus->clock` 字段更改为相应的值。例如，intel8x0 es1968 驱动
有它们自己的函数从时钟读取

### Proc 文件


ALSA AC97 接口将创建一proc 文件，如 `/proc/asound/card0/codec97#0/ac97#0-0`
`ac97#0-0+regs`。你可以参考这些文件来查看编解码器的当前状态和寄存器

### 多个编解码器


当同一张声卡上有多个编解码器时，你需要多次调`snd_ac97_mixer()`，并
`ac97.num=1` 或更大。`num` 字段指定编解码器编号

如果你设置了多个编解码器，你要么需要为每个编解码器编写不同的回调，要么在回调例程中
检`ac97->num`

## MIDI（MPU401-UART）接


### 概述


许多声卡有内置的 MIDI（MPU401-UART）接口。当声卡支持标准MPU401-UART 接口时，
很可能你可以使用 ALSA MPU401-UART API。MPU401-UART API 定义`<sound/mpu401.h>`
中

某些声芯有类似但略有不同mpu401 实现。例如，emu10k1 有自己的 mpu401 例程

### MIDI 构造函


```

  struct snd_rawmidi *rmidi;
  snd_mpu401_uart_new(card, 0, MPU401_HW_MPU401, port, info_flags,
                      irq, &rmidi);


```

第一个参数是声卡指针，第二个是这个组件的索引。你最多可以创8 rawmidi 设备

第三个参数是硬件类型 `MPU401_HW_XXX`。如果不是特殊的，可以使
`MPU401_HW_MPU401`銆。

4 个参数是 I/O 端口地址。许多向后兼容的 MPU401 有一个像 0x330 这样I/O 端口
或者，它可能是其自PCI I/O 区域的一部分。这取决于芯片设计

5 个参数是用于额外信息的位标志。当上面I/O 端口地址PCI I/O 区域的一部分时，
MPU401 I/O 端口可能已经被驱动自身分配（保留）。在这种情况下，传入位标
`MPU401_INFO_INTEGRATED`，mpu401-uart 层将自行分配 I/O 端口

当控制器只支持输入或输出 MIDI 流时，分别传`MPU401_INFO_INPUT` 
`MPU401_INFO_OUTPUT` 位标志。然rawmidi 实例被创建为单流

`MPU401_INFO_MMIO` 位标志用于将访问方法更改MMIO（通过 readb writeb）而不
iob outb。在这种情况下，你必须将 iomapped 地址传给 `snd_mpu401_uart_new()`

当设置了 `MPU401_INFO_TX_IRQ` 时，输出流不在默认中断处理函数中检查。驱动需要自
调用 `snd_mpu401_uart_interrupt_tx()` 来在 irq 处理函数中启动输出流的处理

如果 MPU-401 接口与声卡上的其他逻辑设备共享其中断，设置 `MPU401_INFO_IRQ_HOOK`
（见 `下方 <MIDI Interrupt Handler_>`__）

通常，端口地址对应于命令端口，端口 + 1 对应于数据端口。如果不是，你可以稍后手
更改 struct snd_mpu401 `c