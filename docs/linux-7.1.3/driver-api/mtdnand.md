## MTD NAND 驱动编程接口


:Author: Thomas Gleixner

## 简


通用 NAND 驱动支持几乎所有基NAND AG-AND 的芯片，并将它们连接Linux 内核
存储技术设备（MTD）子系统

本文档面向希望实现适用NAND 设备的板级驱动或文件系统驱动的开发者

## 已知缺陷与假


无

## 文档说明

函数与结构体文档是自动生成的。每个函数和结构体成员都有一段简短描述，[XXX]
标识符标记。后续章节将解释这些标识符的含义

### 函数标识[XXX]


函数在简短注释中[XXX] 标识符标记。这些标识符说明了函数的用途与作用范围。使
如下标识符：

- [MTD Interface]

   这些函数提供MTD 内核 API 的接口。它们不可替换，提供完全独立于硬件的功能

- [NAND Interface]

   这些函数被导出，提供NAND 内核 API 的接口

- [GENERIC]

   通用函数不可替换，提供完全独立于硬件的功能

- [DEFAULT]

   默认函数提供适用于大多数实现的硬件相关功能。这些函数如有必要可由板级驱
   替换。这些函数通过 NAND 芯片描述结构中的指针调用。板级驱动可在调nand_scan()
   之前，将应被板级相关函数替换的函数设置好。如果在进入 nand_scan() 时函数指针为
   NULL，则该指针会被设为适用于所检测芯片类型的默认函数

### 结构体成员标识符 [XXX]


结构体成员在注释中以 [XXX] 标识符标记。这些标识符说明了成员的用途与作用范围。使
如下标识符：

- [INTERN]

   这些成员仅供 NAND 驱动内部使用，不得修改。这些值大多由 nand_scan() 期间评估
   芯片几何信息计算得出

- [REPLACEABLE]

   可替换成员保存可由板级驱动提供的硬件相关函数。板级驱动可在调nand_scan() 之前
   将应被板级相关函数替换的函数设置好。如果在进入 nand_scan() 时函数指针为 NULL，则
   该指针会被设为适用于所检测芯片类型的默认函数

- [BOARDSPECIFIC]

   板级相关成员保存必须由板级驱动提供的硬件相关信息。板级驱动必须在调用 nand_scan()
   之前设置好函数指针和数据字段

- [OPTIONAL]

   可选成员可保存与板级驱动相关的信息。通用 NAND 驱动代码不使用这些信息

## 基本板级驱动


对于大多数板子，只需提供基本函数并填nand_chip 描述结构中一些真正与板级相关
成员即可

### 基本定义

至少你需要提供一nand_chip 结构以及用于保存 ioremap 后的芯片地址的存储区。你
可以kmalloc 分配 nand_chip 结构，也可以静态分配。NAND 芯片结构内嵌了一mtd
结构，该结构会被注册MTD 子系统。你可以使用 nand_to_mtd() 辅助函数nand_chip
指针中提取出 mtd 结构的指针

基于 Kmalloc 的示

```

    static struct mtd_info *board_mtd;
    static void __iomem *baseaddr;


```
静态示
```

    static struct nand_chip board_chip;
    static void __iomem *baseaddr;


```
### 分区定义


如果你想将设备划分为多个分区，则定义一套适合你板子的分区方案
```

    #define NUM_PARTITIONS 2
    static struct mtd_partition partition_info[] = {
        { .name = "Flash partition 1",
          .offset =  0,
          .size =    8 * 1024 * 1024 },
        { .name = "Flash partition 2",
          .offset =  MTDPART_OFS_NEXT,
          .size =    MTDPART_SIZ_FULL },
    };


```
### 硬件控制函数


硬件控制函数提供NAND 芯片控制引脚的访问。访问可以通过 GPIO 引脚或地址线完成。如
使用地址线，请确保满足时序要求

**基于 GPIO 的示*
```

    static void board_hwcontrol(struct mtd_info *mtd, int cmd)
    {
        switch(cmd){
            case NAND_CTL_SETCLE: /* CLE 引脚置高 */ break;
            case NAND_CTL_CLRCLE: /* CLE 引脚置低 */ break;
            case NAND_CTL_SETALE: /* ALE 引脚置高 */ break;
            case NAND_CTL_CLRALE: /* ALE 引脚置低 */ break;
            case NAND_CTL_SETNCE: /* nCE 引脚置低 */ break;
            case NAND_CTL_CLRNCE: /* nCE 引脚置高 */ break;
        }
    }


```
**基于地址线的示例* 假定 nCE 引脚由片选译码器驱动
```

    static void board_hwcontrol(struct mtd_info *mtd, int cmd)
    {
        struct nand_chip *this = mtd_to_nand(mtd);
        switch(cmd){
            case NAND_CTL_SETCLE: this->legacy.IO_ADDR_W |= CLE_ADRR_BIT;  break;
            case NAND_CTL_CLRCLE: this->legacy.IO_ADDR_W |= ~CLE_ADRR_BIT; break;
            case NAND_CTL_SETALE: this->legacy.IO_ADDR_W |= ALE_ADRR_BIT;  break;
            case NAND_CTL_CLRALE: this->legacy.IO_ADDR_W |= ~ALE_ADRR_BIT; break;
        }
    }


```
### 设备就绪函数


如果硬件接口NAND 芯片 ready busy 引脚连接到了 GPIO 或其他可访问I/O 引脚，则使用
此函数来读回该引脚的状态。该函数没有参数，当设备忙时（R/B 引脚为低）应返回 0，当设备
就绪时（R/B 引脚为高）应返回 1。如果硬件接口不能访ready busy 引脚，则不得定义
函数，并且函数指this->legacy.dev_ready 应设NULL

### 初始化函


初始化函数分配内存并设置所有板级相关的参数和函数指针。当一切设置就绪后调用
nand_scan()。该函数尝试检测并识别芯片。如果找到芯片，则所有内部数据字段都会被相应
初始化。结构必须首先清零，然后再填入有关设备的必要信息
```

    static int __init board_init (void)
    {
        struct nand_chip *this;
        int err = 0;

        /* MTD 设备结构和私有数据分配内*/
        this = kzalloc(sizeof(struct nand_chip), GFP_KERNEL);
        if (!this) {
            printk ("Unable to allocate NAND MTD device structure.\n");
            err = -ENOMEM;
            goto out;
        }

        board_mtd = nand_to_mtd(this);

        /* 映射物理地址 */
        baseaddr = ioremap(CHIP_PHYSICAL_ADDRESS, 1024);
        if (!baseaddr) {
            printk("Ioremap to access NAND chip failed\n");
            err = -EIO;
            goto out_mtd;
        }

        /* 设置 NAND IO 线地址 */
        this->legacy.IO_ADDR_R = baseaddr;
        this->legacy.IO_ADDR_W = baseaddr;
        /* 引用硬件控制函数 */
        this->hwcontrol = board_hwcontrol;
        /* 设置命令延迟时间，正确取值参见数据手*/
        this->legacy.chip_delay = CHIP_DEPENDEND_COMMAND_DELAY;
        /* 若可用，分配设备就绪函数 */
        this->legacy.dev_ready = board_dev_ready;
        this->eccmode = NAND_ECC_SOFT;

        /* 扫描以查找设备是否存*/
        if (nand_scan (this, 1)) {
            err = -ENXIO;
            goto out_ior;
        }

        add_mtd_partitions(board_mtd, partition_info, NUM_PARTITIONS);
        goto out;

    out_ior:
        iounmap(baseaddr);
    out_mtd:
        kfree (this);
    out:
        return err;
    }
    module_init(board_init);


```
### 退出函


只有当驱动被编译为模块时才需要退出函数。它释放芯片驱动占用的所有资源，并在 MTD 
注销分区
```

    #ifdef MODULE
    static void __exit board_cleanup (void)
    {
        /* 注销设备 */
        WARN_ON(mtd_device_unregister(board_mtd));
        /* 释放资源 */
        nand_cleanup(mtd_to_nand(board_mtd));

        /* 取消映射物理地址 */
        iounmap(baseaddr);

        /* 释放 MTD 设备结构 */
        kfree (mtd_to_nand(board_mtd));
    }
    module_exit(board_cleanup);
    #endif


```
## 高级板级驱动函数


本章描述 NAND 驱动的高级功能。有关可被板级驱动覆盖的函数列表，请参见 nand_chip 结构
的文档

### 多芯片控


nand 驱动可以控制芯片阵列。因此板级驱动必须提供自己的 select_chip 函数。该函数必须
（去）选择所请求的芯片。函数指针必须在调用 nand_scan() 之前设置好。nand_scan() 
maxchip 参数定义了要扫描的最大芯片数。请确保 select_chip 函数能够处理所请求的芯片数

nand 驱动将芯片拼接为一个虚拟芯片，并将该虚拟芯片提供给 MTD 层

*注意：驱动只能处理由等大小芯片组成的线性芯片阵列。不支持扩展总线宽度的并行阵列

**基于 GPIO 的示*
```

    static void board_select_chip (struct mtd_info *mtd, int chip)
    {
        /* 取消选择所有芯片，将所nCE 引脚置高 */
        GPIO(BOARD_NAND_NCE) |= 0xff;
        if (chip >= 0)
            GPIO(BOARD_NAND_NCE) &= ~ (1 << chip);
    }


```
**基于地址线的示例* 假定 nCE 引脚连接到地址译码器
```

    static void board_select_chip (struct mtd_info *mtd, int chip)
    {
        struct nand_chip *this = mtd_to_nand(mtd);

        /* 取消选择所有芯*/
        this->legacy.IO_ADDR_R &= ~BOARD_NAND_ADDR_MASK;
        this->legacy.IO_ADDR_W &= ~BOARD_NAND_ADDR_MASK;
        switch (chip) {
        case 0:
            this->legacy.IO_ADDR_R |= BOARD_NAND_ADDR_CHIP0;
            this->legacy.IO_ADDR_W |= BOARD_NAND_ADDR_CHIP0;
            break;
        ....
        case n:
            this->legacy.IO_ADDR_R |= BOARD_NAND_ADDR_CHIPn;
            this->legacy.IO_ADDR_W |= BOARD_NAND_ADDR_CHIPn;
            break;
        }
    }


```
### 硬件 ECC 支持


#### 函数与常


nand 驱动支持三种不同类型的硬ECC

- NAND_ECC_HW3_256

   256 字节提供 3 字节 ECC 的硬ECC 生成器

- NAND_ECC_HW3_512

   512 字节提供 3 字节 ECC 的硬ECC 生成器

- NAND_ECC_HW6_512

   512 字节提供 6 字节 ECC 的硬ECC 生成器

- NAND_ECC_HW8_512

   512 字节提供 8 字节 ECC 的硬ECC 生成器

如果你的硬件生成器功能不同，请在 nand_base.c 中合适的位置添加

板级驱动必须提供以下函数

- enable_hwecc

   该函数在对芯片读/写之前调用。在此函数中复位或初始化硬件生成器。调用时会传入一
   参数，用于区分读操作和写操作

- calculate_ecc

   该函数在对芯片读/写之后调用。将 ECC 从硬件传送到缓冲区。如果设置了 NAND_HWECC_SYNDROME
   选项，则该函数只在写操作时调用。见下文

- correct_data

   发生 ECC 错误时调用此函数进行错误检测与纠正。如果错误可以纠正，则分别返1 2
   如果错误不可纠正则返-1。如果你的硬件生成器nand_ecc 软件生成器的默认算法匹配
   则使nand_ecc 提供的纠正函数，而不要实现重复的代码

#### 使用校验子计算的硬件 ECC


许多硬件 ECC 实现提供 Reed-Solomon 码，并在读取时计算错误校验子（syndrome）。该校验
在调用通用 Reed-Solomon 库中的纠错代码之前，必须转换为标准的 Reed-Solomon 校验子

为了使校验子生成器工作，ECC 字节必须紧接在数据字节之后放置。这与软ECC 使用的常
布局相反。数据区与带外（out of band）区的分离不再可能。nand 驱动代码会处理这种布局
oob 区中剩余的空闲字节由自动放置（autoplacement）代码管理。这种情况下请提供匹配的 oob
布局。实现参考见 rts_from4.c diskonchip.c。在这些情况下，我们还必须在 FLASH 上使
坏块表，因为 ECC 布局与坏块标记位置相互冲突。详见坏块表支持一节

### 坏块表支


大多NAND 芯片spare 区的固定位置标记坏块。这些块在任何情况下都不得被擦除，否
坏块信息会丢失。可以在每次访问块时通过读取块中第一页的 spare 区来检查坏块标记。这
耗时，因此使用坏块表

nand 驱动支持多种类型的坏块表

- 每设

   坏块表包含设备的所有坏块信息，设备可由多个芯片组成

- 姣忚姱鐗。

   每个芯片使用一个坏块表，包含该特定芯片的坏块信息

- 固定偏移

   坏块表位于芯片（设备）中的固定偏移处。这适用于各DiskOnChip 设备

- 自动放置

   坏块表自动放置并检测，位于芯片（设备）的末尾或开头

- 镜像

   坏块表在芯片（设备）上做镜像，以便在不丢失数据的情况下更新坏块表

nand_scan() 调用 nand_default_bbt() 函数。nand_default_bbt() 根据 nand_scan() 获取
芯片信息选择适当的默认坏块表描述符

标准策略是扫描设备以查找坏块，并构建一个基RAM 的坏块表，这比始终检查闪存芯片本
上的坏块信息访问更快

#### 基于 FLASH 的表


可能期望或必须将坏块表保存在 FLASH 中。对AG-AND 芯片这是强制性的，因为它们没
出厂标记的坏块，而是有出厂标记的好块。当块被擦除以重新使用时，标记模式会被清除。因此，
如果在将模式写回芯片之前发生断电，该块就会丢失并被加入坏块。因此，当我们第一次检测到
芯片时，会扫描其中的好块，并在擦除任何块之前将此信息存入坏块表

存储这些表所在的块通过在内存坏块表中标记为坏来进行保护，以防止意外访问。坏块表管理
函数被允许绕过这种保护

激活基FLASH 的坏块表支持最简单的方法，是在调nand_scan() 之前，于 nand 芯片结构
bbt_option 字段中设NAND_BBT_USE_FLASH 选项。对AG-AND 芯片，这是默认完成的。这
会激NAND 驱动默认的基FLASH 的坏块表功能。默认的坏块表选项为：

- 每个芯片存储一个坏块表

- 每块使用 2 

- 自动放置在芯片末

- 使用带版本号的镜像表

- 在芯片末尾保4 个块

#### 用户自定义表


用户自定义表通过填写 nand_bbt_descr 结构，并在调nand_scan() 之前将该指针存入 nand_chip
结构bbt_td 成员来创建。如果还需要镜像表，则必须创建第二个结构，并将指向该结构的指针
存入 nand_chip 结构内的 bbt_md。如bbt_md 成员设为 NULL，则只使用主表，不会扫描镜像表

nand_bbt_descr 结构中最重要的字段是 options 字段。options 定义了表的大部分属性。使
rawnand.h 中预定义的常量来定义选项

- 每块位数

   支持的位数为 1

- 每芯片表

   设置常量 NAND_BBT_PERCHIP 表示对芯片阵列中的每个芯片管理一个坏块表。如果未设置此选项
   则使用每设备坏块表

- 表位置是绝对

   使用选项常量 NAND_BBT_ABSPAGE，并pages 字段中定义坏块表起始的绝对页号。如果你选择
   每芯片坏块表并且拥有多芯片阵列，则必须为芯片阵列中的每个芯片给定起始页。注意：不会
   扫描表标识模式，因此 pattern、veroffs、offs、len 字段可以不初始化

- 表位置自动检

   表可以位于芯片（设备）的第一个或最后一个好块中。设NAND_BBT_LASTBLOCK 将坏块表放置
   芯片（设备）末尾。坏块表通过存储在持有坏块表的块的第一spare 区中的模式来标记和识别
   pattern 字段中存入指向该模式的指针。此外，模式的长度必须存len，spare 区中的偏
   必须nand_bbt_descr 结构offs 成员中给定。对于镜像坏块表，必须使用不同的模式

- 表创

   设置选项 NAND_BBT_CREATE 以在扫描期间找不到表时启用表的创建。通常仅当发现新芯片时执行一次

- 表写支持

   设置选项 NAND_BBT_WRITE 以启用表的写支持。这允许在由于磨损必须将某块标记为坏时更新坏块表
   MTD 接口函数 block_markbad 会调用坏块表的更新函数。如果启用了写支持，则表会在 FLASH 上更新

   注意：写支持应仅对带版本控制的镜像表启用

- 表版本控

   设置选项 NAND_BBT_VERSION 以启用表的版本控制。强烈建议对带写支持的镜像表启用此项。它确保
   丢失坏块表信息的风险降低到仅丢失关于那一个应标记为坏的磨损块的信息。版本号存储在设spare
   区中连续4 个字节。版本号的位置由坏块表描述符veroffs 成员定义

- 鍐欐椂淇濆瓨鍧楀唴瀹。

   如果持有坏块表的块中确实包含其他有用信息，则设置选项 NAND_BBT_SAVECONTENT。写入坏块表时，
   会读取整个块、更新坏块表、擦除该块，然后将所有内容写回。如果未设置此选项，则只会写入坏块
   表，块中的其他所有内容都被忽略并擦除

- 保留块数

   对于自动放置，必须保留一些块用于坏块表存储。保留块数在坏块表描述结构的 maxblocks 成员
   定义。为镜像表保4 个块应该是一个合理的数字。这也限制了为坏块表标识模式而扫描的块数

### Spare 区（自动）放


nand 驱动实现了在 spare 区中放置文件系统数据的不同方式：

- fs 驱动定义的放

- 自动放置

默认的放置函数是自动放置。nand 驱动内建了针对各种芯片类型的默认放置方案。如果由于硬ECC
功能导致默认放置不合适，则板级驱动可以提供自己的放置方案

文件系统驱动可以提供自己的放置方案以取代默认放置方案

放置方案nand_oobinfo 结构定义
```

    struct nand_oobinfo {
        int useecc;
        int eccbytes;
        int eccpos[24];
        int oobfree[8][2];
    };


```
- useecc

   useecc 成员控制 ecc 和放置函数。头文件 include/mtd/mtd-abi.h 包含用于选择 ecc 和放置的
   常量。MTD_NANDECC_OFF 完全关闭 ecc。这不推荐使用，仅用于测试和诊断。MTD_NANDECC_PLACE
   选择调用者定义的放置，MTD_NANDECC_AUTOPLACE 选择自动放置

- eccbytes

   eccbytes 成员定义每页ecc 字节数

- eccpos

   eccpos 数组保存 ecc 码在 spare 区中放置的字节偏移

- oobfree

   oobfree 数组定义 spare 区中可用于自动放置的区域。信息以 {offset, size} 格式给出。offset
   定义可用区域的起始，size 为长度（字节）。可以定义多个区域。列表以 {0, 0} 条目终止

#### fs 驱动定义的放


调用函数提供一个指nand_oobinfo 结构的指针，该结构定ecc 放置。对于写操作，调用者必
提供 spare 区缓冲区以及数据缓冲区。spare 区缓冲区大小为（页数（spare 区大小）。对于读
操作，缓冲区大小为（页数（（spare 区大小）+（每ecc 步数 sizeof (int)）。驱动存
ecc 检查的结果
```

	<spare data page 0><ecc result 0>...<ecc result n>

	...

	<spare data page n><ecc result 0>...<ecc result n>

```
这是 YAFFS1 使用的遗留模式

如果 spare 区缓冲区NULL，则仅根nand_oobinfo 结构中给定的方案进行 ECC 放置

#### 自动放置


自动放置使用内建默认值将 ecc 字节放置spare 区中。如果需要将文件系统数据存储/读取spare
区中，则调用函数必须提供一个缓冲区。每页的缓冲区大小由 nand_oobinfo 结构中的 oobfree 数组
决定

如果 spare 区缓冲区NULL，则仅根据默认内建方案进ECC 放置

### Spare 区自动放置默认方


#### 256 瀛楄妭椤靛ぇ灏。


======== ================== ===================================================
偏移     内容               注释
======== ================== ===================================================
0x00     ECC byte 0         错误纠正码字0
0x01     ECC byte 1         错误纠正码字1
0x02     ECC byte 2         错误纠正码字2
0x03     Autoplace 0
0x04     Autoplace 1
0x05     Bad block marker   如果该字节中任一位为零，则该块为坏块。这只适用于块
			    的第一页。在其余页中该字节被保留
0x06     Autoplace 2
0x07     Autoplace 3
======== ================== ===================================================

#### 512 瀛楄妭椤靛ぇ灏。


============= ================== ==============================================
偏移          内容               注释
============= ================== ==============================================
0x00          ECC byte 0         本页中低 256 字节数据的错误纠正码字节 0
0x01          ECC byte 1         本页中低 256 字节数据的错误纠正码字节 1
0x02          ECC byte 2         本页中低 256 字节数据的错误纠正码字节 2
0x03          ECC byte 3         本页中高 256 字节数据的错误纠正码字节 0
0x04          reserved           保留
0x05          Bad block marker   如果该字节中任一位为零，则该块为坏块。这只适用于块
				 的第一页。在其余页中该字节被保留
0x06          ECC byte 4         本页中高 256 字节数据的错误纠正码字节 1
0x07          ECC byte 5         本页中高 256 字节数据的错误纠正码字节 2
0x08 - 0x0F   Autoplace 0 - 7
============= ================== ==============================================

#### 2048 瀛楄妭椤靛ぇ灏。


=========== ================== ================================================
偏移        内容               注释
=========== ================== ================================================
0x00        Bad block marker   如果该字节中任一位为零，则该块为坏块。这只适用于块
			       的第一页。在其余页中该字节被保留
0x01        Reserved           保留
0x02-0x27   Autoplace 0 - 37
0x28        ECC byte 0         本页中第一256 字节数据的错误纠正码字节 0
0x29        ECC byte 1         本页中第一256 字节数据的错误纠正码字节 1
0x2A        ECC byte 2         本页中第一256 字节数据的错误纠正码字节 2
0x2B        ECC byte 3         本页中第二份 256 字节数据的错误纠正码字节 0
0x2C        ECC byte 4         本页中第二份 256 字节数据的错误纠正码字节 1
0x2D        ECC byte 5         本页中第二份 256 字节数据的错误纠正码字节 2
0x2E        ECC byte 6         本页中第三份 256 字节数据的错误纠正码字节 0
0x2F        ECC byte 7         本页中第三份 256 字节数据的错误纠正码字节 1
0x30        ECC byte 8         本页中第三份 256 字节数据的错误纠正码字节 2
0x31        ECC byte 9         本页中第四份 256 字节数据的错误纠正码字节 0
0x32        ECC byte 10        本页中第四份 256 字节数据的错误纠正码字节 1
0x33        ECC byte 11        本页中第四份 256 字节数据的错误纠正码字节 2
0x34        ECC byte 12        本页中第五份 256 字节数据的错误纠正码字节 0
0x35        ECC byte 13        本页中第五份 256 字节数据的错误纠正码字节 1
0x36        ECC byte 14        本页中第五份 256 字节数据的错误纠正码字节 2
0x37        ECC byte 15        本页中第六份 256 字节数据的错误纠正码字节 0
0x38        ECC byte 16        本页中第六份 256 字节数据的错误纠正码字节 1
0x39        ECC byte 17        本页中第六份 256 字节数据的错误纠正码字节 2
0x3A        ECC byte 18        本页中第七份 256 字节数据的错误纠正码字节 0
0x3B        ECC byte 19        本页中第七份 256 字节数据的错误纠正码字节 1
0x3C        ECC byte 20        本页中第七份 256 字节数据的错误纠正码字节 2
0x3D        ECC byte 21        本页中第八份 256 字节数据的错误纠正码字节 0
0x3E        ECC byte 22        本页中第八份 256 字节数据的错误纠正码字节 1
0x3F        ECC byte 23        本页中第八份 256 字节数据的错误纠正码字节 2
=========== ================== ================================================

## 文件系统支持


NAND 驱动通过 MTD 接口为文件系统提供所有必要的函数

文件系统必须了解 NAND 的特性和限制。NAND Flash 的一个主要限制是，你不能想写一页就写一
地频繁写入。在再次擦除之前，对一页的连续写入被限制为 1-3 次，具体取决于厂商规格。这同样
适用spare 区

因此，支NAND 的文件系统必须以页大小块写入，或者持有一个写缓冲区，以收集较小的写入
直到它们累加到页大小。可用的支持 NAND 的文件系统有：JFFS2、YAFFS

用于存储文件系统数据spare 区使用，由前面某章描述的 spare 区放置功能控制

## 工具


MTD 项目提供了一组有用的工具来处NAND Flash

- flasherase, flasheraseall：擦除并格式FLASH 分区

- nandwrite：将文件系统镜像写入 NAND FLASH

- nanddump：转NAND FLASH 分区的内

这些工具了解 NAND 的限制。请使用这些工具，而不是抱怨由不支NAND 的访问方法导致的错误

## 常量


本章描述可能与驱动开发者相关的常量

### 芯片选项常量


#### 芯片 ID 表常


这些常量定义rawnand.h 中。它们通过按位或组合在一
```

    /* 总线宽度16 */
    #define NAND_BUSWIDTH_16    0x00000002
    /* 设备支持无需填充的部分编*/
    #define NAND_NO_PADDING     0x00000004
    /* 芯片具有缓存编程功能 */
    #define NAND_CACHEPRG       0x00000008
    /* 芯片具有回拷功能 */
    #define NAND_COPYBACK       0x00000010
    /* AND 芯片，具4 bank 以及令人困惑的页/
     * 分配。更多信息参Renesas 数据手册 */
    #define NAND_IS_AND     0x00000020
    /* 芯片具有 4 页的阵列，可在无
     * 额外 ready /busy 等待的情况下读取 */
    #define NAND_4PAGE_ARRAY    0x00000040


```
#### 运行时选项常量


这些常量定义rawnand.h 中。它们通过按位或组合在一
```

    /* 硬件 ecc 生成器在读取时提供校验子而非 ecc 
     * 这只有在 ecc 字节紧接数据字节之后时才能工作。适用DOC AG-AND Renesas 硬件 Reed Solomon 生成*/
    #define NAND_HWECC_SYNDROME 0x00020000


```
### ECC 选择常量


```

    /* ECC。不推荐使用*/
    #define NAND_ECC_NONE       0
    /* 软件 ECC，每 256 字节数据 3 字节 ECC */
    #define NAND_ECC_SOFT       1
    /* 硬件 ECC，每 256 字节数据 3 字节 ECC */
    #define NAND_ECC_HW3_256    2
    /* 硬件 ECC，每 512 字节数据 3 字节 ECC */
    #define NAND_ECC_HW3_512    3
    /* 硬件 ECC，每 512 字节数据 6 字节 ECC */
    #define NAND_ECC_HW6_512    4
    /* 硬件 ECC，每 512 字节数据 8 字节 ECC */
    #define NAND_ECC_HW8_512    6


```
### 硬件控制相关常量


这些常量描述了请求的硬件访问函数
```

    /* 通过nCE 置低来选择芯片 */
    #define NAND_CTL_SETNCE     1
    /* 通过nCE 置高来取消选择芯片 */
    #define NAND_CTL_CLRNCE     2
    /* 通过CLE 置高来选择命令锁存 */
    #define NAND_CTL_SETCLE     3
    /* 通过CLE 置低来取消选择命令锁存 */
    #define NAND_CTL_CLRCLE     4
    /* 通过ALE 置高来选择地址锁存 */
    #define NAND_CTL_SETALE     5
    /* 通过ALE 置低来取消选择地址锁存 */
    #define NAND_CTL_CLRALE     6
    /* 通过WP 置高来设置写保护。未使用*/
    #define NAND_CTL_SETWP      7
    /* 通过WP 置低来清除写保护。未使用*/
    #define NAND_CTL_CLRWP      8


```
### 坏块表相关常


这些常量描述了用于坏块表的选项
```

    /* 坏块表描述符的选项 */

    /* 设备bbt 中每块使用的位数 */
    #define NAND_BBT_NRBITS_MSK 0x0000000F
    #define NAND_BBT_1BIT       0x00000001
    #define NAND_BBT_2BIT       0x00000002
    #define NAND_BBT_4BIT       0x00000004
    #define NAND_BBT_8BIT       0x00000008
    /* 坏块表位于设备的最后一个好块中 */
    #define NAND_BBT_LASTBLOCK  0x00000010
    /* bbt 位于给定页，否则我们必须扫描 bbt */
    #define NAND_BBT_ABSPAGE    0x00000020
    /* 在多芯片设备上，bbt 按芯片存*/
    #define NAND_BBT_PERCHIP    0x00000080
    /* bbt veroffs 偏移处带有版本计数器 */
    #define NAND_BBT_VERSION    0x00000100
    /* 若不存在则创bbt */
    #define NAND_BBT_CREATE     0x00000200
    /* 必要时写bbt */
    #define NAND_BBT_WRITE      0x00001000
    /* 写入 bbt 时读回并写回块内*/
    #define NAND_BBT_SAVECONTENT    0x00002000


```
## 结构


本章包含 NAND 驱动中使用、并可能与驱动开发者相关的结构体的自动生成文档。每个结构体成员
都有一段以 [XXX] 标识符标记的简短描述。解释请参见“文档说明”一章

   :internal:

## 导出的公共函


本章包含导出NAND 内核 API 函数的自动生成文档。每个函数都有一段以 [XXX] 标识符标记的
简短描述。解释请参见“文档说明”一章

   :export:

## 提供的内部函


本章包含 NAND 驱动内部函数的自动生成文档。每个函数都有一段以 [XXX] 标识符标记的简短描述
解释请参见“文档说明”一章。标记为 [DEFAULT] 的函数可能与板级驱动开发者相关

   :internal:

   :internal:

## 致谢


以下人员NAND 驱动做出了贡献：

1. Steven J. Hill\ sjhill@realitydiluted.com

2. David Woodhouse\ dwmw2@infradead.org

3. Thomas Gleixner\ tglx@kernel.org

大量用户提供了缺陷修复、改进以及测试中的帮助。在此深表感谢

以下人员为本文档做出了贡献：

1. Thomas Gleixner\ tglx@kernel.org
