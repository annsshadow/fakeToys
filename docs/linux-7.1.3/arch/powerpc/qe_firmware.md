## Freescale QUICC Engine 固件上传


(c) 2007 Timur Tabi <timur at freescale.com>,
    Freescale Semiconductor


   I - 固件的软件许可证

   II - 微码可用性

   III - 描述与术语

   IV - 微码编程细节

   V - 固件结构布局

   VI - 用于创建固件文件的示例代码

## 修订信息


2007年11月30日：Rev 1.0 - 初始版本

## 一、固件的软件许可证


每个固件文件都带有其自己的软件许可证。有关特定许可证的信息，请查看随固件分发的许可证文本。

## 二、微码可用性


固件文件通过多种渠道分发。部分可在 http://opensource.freescale.com 获取。其他固件文件请联系你的 Freescale 代表或操作系统供应商。

## 三、描述与术语


在本文档中，术语“microcode（微码）”指的是构成实际 QE 微码的一串 32 位整数。

术语“firmware（固件）”指的是一个二进制 blob，它包含微码以及其他数据，这些数据：

 1) 描述微码的目的
 2) 描述如何以及在哪里上传微码
 3) 指定各种寄存器的值
 4) 包含供特定设备驱动使用的额外数据

固件文件是只包含一份固件的二进制文件。

## 四、微码编程细节


QE 架构允许每个 RISC 处理器在 I-RAM 中只存在一份微码。要替换任何当前的微码，必须先执行一次完整的 QE 复位（它会禁用微码）。

QE 微码按以下步骤上传：

1) 使用 IRAM.IADD 和 IRAM.IDATA 寄存器，将微码放置到 I-RAM 中的特定位置。

2) 根据固件是否需要分离式 I-RAM，将 CERCR.CIR 位设为 0 或 1。分离式 I-RAM 仅对拥有多 RISC 处理器 QE 的 SOC（如 8360）有意义。分离 I-RAM 允许每个处理器运行不同的微码，从而有效地构成一个非对称多处理（AMP）系统。

3) 将 TIBCR 陷阱寄存器加载为微码中陷阱处理程序的地址。

4) 将 RSP.ECCR 寄存器编程为给定的值。

5) 如有必要，需要虚拟陷阱和扩展模式数据的设备驱动会使用它们。

虚拟微码陷阱

这些虚拟陷阱是微码中的条件分支。这些是 ROM 码中引入的“软”临时机制，用以提供更高的灵活性并节省硬件陷阱。如果激活了新特性，或在 RAM 包中修复了某个问题，应当激活它们。该数据结构向微码发出信号，告知哪些虚拟陷阱处于激活状态。

该结构包含 6 个字，应用程序应当将其复制到某处

```

	---------------------------------------------------------------
	| Offset in |                  | Destination Offset | Size of |
	|   array   |     Protocol     |   within PRAM      | Operand |
	--------------------------------------------------------------|
	|     0     | Ethernet         |      0xF8          | 4 bytes |
	|           | interworking     |                    |         |
	---------------------------------------------------------------
	|     4     | ATM              |      0xF8          | 4 bytes |
	|           | interworking     |                    |         |
	---------------------------------------------------------------
	|     8     | PPP              |      0xF8          | 4 bytes |
	|           | interworking     |                    |         |
	---------------------------------------------------------------
	|     12    | Ethernet RX      |      0x22          | 1 byte  |
	|           | Distributor Page |                    |         |
	---------------------------------------------------------------
	|     16    | ATM Globtal      |      0x28          | 1 byte  |
	|           | Params Table     |                    |         |
	---------------------------------------------------------------
	|     20    | Insert Frame     |      0xF8          | 4 bytes |
	---------------------------------------------------------------


```
扩展模式

这是一个双字位数组（64 位），定义了对软件驱动有特殊影响的功能。每一位都有自身的影响，并带有与之相关的软件专用指令。该结构为

```

	-----------------------------------------------------------------------
	| Bit #  |     Name     |   Description                               |
	-----------------------------------------------------------------------
	|   0    | General      | Indicates that prior to each host command   |
	|        | push command | given by the application, the software must |
	|        |              | assert a special host command (push command)|
	|        |              | CECDR = 0x00800000.                         |
	|        |              | CECR = 0x01c1000f.                          |
	-----------------------------------------------------------------------
	|   1    | UCC ATM      | Indicates that after issuing ATM RX INIT    |
	|        | RX INIT      | command, the host must issue another special|
	|        | push command | command (push command) and immediately      |
	|        |              | following that re-issue the ATM RX INIT     |
	|        |              | command. (This makes the sequence of        |
	|        |              | initializing the ATM receiver a sequence of |
	|        |              | three host commands)                        |
	|        |              | CECDR = 0x00800000.                         |
	|        |              | CECR = 0x01c1000f.                          |
	-----------------------------------------------------------------------
	|   2    | Add/remove   | Indicates that following the specific host  |
	|        | command      | command: "Add/Remove entry in Hash Lookup   |
	|        | validation   | Table" used in Interworking setup, the user |
	|        |              | must issue another command.                 |
	|        |              | CECDR = 0xce000003.                         |
	|        |              | CECR = 0x01c10f58.                          |
	-----------------------------------------------------------------------
	|   3    | General push | Indicates that the s/w has to initialize    |
	|        | command      | some pointers in the Ethernet thread pages  |
	|        |              | which are used when Header Compression is   |
	|        |              | activated.  The full details of these       |
	|        |              | pointers is located in the software drivers.|
	-----------------------------------------------------------------------
	|   4    | General push | Indicates that after issuing Ethernet TX    |
	|        | command      | INIT command, user must issue this command  |
	|        |              | for each SNUM of Ethernet TX thread.        |
	|        |              | CECDR = 0x00800003.                         |
	|        |              | CECR = 0x7'b{0}, 8'b{Enet TX thread SNUM},  |
	|        |              |        1'b{1}, 12'b{0}, 4'b{1}              |
	-----------------------------------------------------------------------
	| 5 - 31 |     N/A      | Reserved, set to zero.                      |
	-----------------------------------------------------------------------

```
## 五、固件结构布局


来自 Freescale 的 QE 微码通常作为头文件提供。该头文件包含定义微码二进制本身以及用于上传该微码的其他数据的宏。这些文件的格式不利于简单地包含到其他代码中。因此，需要一种更具可移植性的格式。本节定义该格式。

分发时不再使用头文件，而是将微码及相关数据嵌入到一个二进制 blob 中。该 blob 被传给 qe_upload_firmware() 函数，它解析该 blob 并执行上传微码所需的全部操作。

所有整数均为大端序。有关最新实现信息，请查看 qe_upload_firmware() 函数的注释。

该结构支持版本化，结构的版本嵌入在结构自身之中。为确保前向与后向兼容，所有版本的结构都必须在开头使用相同的 'qe_header' 结构。

'header'（类型：struct qe_header）：
	'length' 字段是整份结构的大小（以字节计），包含其中嵌入的所有微码以及 CRC（如果存在）。

	'magic' 字段是一个由三个字节组成的数组，包含字母 'Q'、'E' 和 'F'。这是一个标识符，表明该结构是一个 QE 固件结构。

	'version' 字段是一个单字节，表明该结构的版本。如果结构的布局需要更改以添加对其他类型微码的支持，则版本号也应相应更改。

'id' 字段是一个以空字符结尾的字符串（适合打印），用于标识固件。

'count' 字段表示 'microcode' 结构的数量。每个 RISC 处理器必须有且仅有一个 'microcode' 结构。因此，该字段也表示此 SOC 的 RISC 处理器数量。

'soc' 结构包含用于将微码与 SOC 本身匹配的 SOC 编号和修订号。通常，微码加载程序应当将该结构中的数据与 SOC 编号和修订号进行核对，仅当匹配时才上传微码。不过，并非所有平台都会做此检查。

尽管不推荐，但你可以在 soc.model 字段中指定 '0' 以完全跳过 SOC 匹配。

'model' 字段是一个 16 位数字，与实际 SOC 匹配。'major' 和 'minor' 字段分别是 SOC 的主修订号和次修订号。

```

     soc.model = 8323
     soc.major = 1
     soc.minor = 0

```
'padding'（填充）是为结构对齐所必需的。该字段确保 'extended_modes' 字段在 64 位边界上对齐。

'extended_modes' 是一个位域，定义了对设备驱动有影响的功能。每一位都有自身的影响并带有与之相关的驱动专用指令。该字段存储在 QE 库中，可供任何调用 qe_get_firmware_info() 的驱动使用。

'vtraps' 是一个包含 8 个字的数组，存放每个虚拟陷阱的虚拟陷阱值。与 'extended_modes' 相同，该字段存储在 QE 库中，可供任何调用 qe_get_firmware_info() 的驱动使用。

'microcode'（类型：struct qe_microcode）：
	每个 RISC 处理器对应一个 'microcode' 结构。第一个 'microcode' 结构对应第一个 RISC，依此类推。

	'id' 字段是一个适合打印的、以空字符结尾的字符串，用于标识此特定微码。

	'traps' 是一个包含 16 个字的数组，存放 16 个陷阱各自的硬件陷阱值。如果 trap[i] 为 0，则忽略此特定陷阱（即不写入 TIBCR[i]）。整个值按原样写入 TIBCR[i] 寄存器，因此如有必要，请确保设置 EN 和 T_IBP 位。

	'eccr' 是要编程到 ECCR 寄存器中的值。

	'iram_offset' 是开始写入微码时相对于 IRAM 的偏移。

	'count' 是微码中 32 位字的数量。

	'code_offset' 是从本结构开头到微码本身所在位置的偏移（以字节计）。第一个微码二进制应紧接在 'microcode' 数组之后。

	'major'、'minor' 和 'revision' 分别是微码的主版本号、次版本号和修订版本号。如果所有值都为 0，则忽略这些字段。

	'reserved' 是为结构对齐所必需的。由于 'microcode' 是一个数组，64 位的 'extended_modes' 字段需要在 64 位边界上对齐，而这只有在 'microcode' 的大小为 8 字节的整数倍时才能实现。为确保这一点，我们加入 'reserved'。

最后一份微码之后是一个 32 位 CRC。它可以使用

```

  u32 crc32(const u8 *p, unsigned int len)
  {
	unsigned int i;
	u32 crc = 0;

	while (len--) {
	   crc ^= *p++;
	   for (i = 0; i < 8; i++)
		   crc = (crc >> 1) ^ ((crc & 1) ? 0xedb88320 : 0);
	}
	return crc;
  }

```
## 六、用于创建固件文件的示例代码


一个从 Freescale 通常分发的头文件创建固件二进制的 Python 程序可在 http://opensource.freescale.com 找到。
