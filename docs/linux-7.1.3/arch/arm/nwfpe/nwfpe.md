## 简

本目录包NetWinder 浮点模拟器（Floating Point Emulator.92 版本的测试发布
大部分代码由我（Scott Bambrough）编写，采用 C 语言，仅在必要处以少量内联汇编例程实现。编写过程较为仓促，首要目标是为编译器发出的所有浮点指令实现一个可工作的版本。我已在力所能及的范围内尽量优化，但仍有很大的改进空间
我尝试让该模拟器尽可能具备可移植性。其中一个问题在于内核符号的前导下划线：ELF 内核没有前导下划线，a.out 编译的内核则有。我在可能重要的地方都尽量使用了 `C_SYMBOL_NAME` 宏
我的另一处选择体现在文件结构上。我尝试将所有与操作系统相关的代码放在同一个模块（`fpmodule.*`）中，其余文件则包含模拟器特有的代码。这样一来，其他人就能相对容易地将该模拟器移植到 NetBSD 等系统上
浮点运算基于 John Hauser SoftFloat Release 2。SoftFloat 是符IEC/IEEE 二进制浮点算术标准的浮点软件实现。最多支持四种格式：单精度、双精度、扩展双精度以及四倍精度。标准要求的全部运算均已实现，仅十进制之间的转换除外。我们只使用单精度、双精度与扩展双精度格式。SoftFloat ARM 的移植由 Phil Blundell 完成，其基础Neil Carson NetBSD/arm32 所做的 SoftFloat 1 版本早期移植
文件 README.FPE 描述了模拟器目前已实现的内容。文TODO 包含尚未完成的工作以及模拟器的其他设想
缺陷报告、评论与建议请发给我，邮<scottb@netwinder.org>。诸如“安装你的模拟器后这个程序不能正常工作”之类的笼统报告，对于确认缺陷仍然存在是有用的，但在定位问题时几乎毫无帮助。请提交这些报告，但不要指望很快得到处理。缺陷仍然存在，问题主要在于定位究竟哪条指令包含该缺陷。能够说明某个具体问题的短小程序则极其珍贵
### 法律声明


NetWinder 浮点模拟器是自由软件。Rebel.com 编写的所有内容均依据 GNU GPL 提供。复制条件请参见 COPYING 文件。上述范围不包含 SoftFloat 代码，John Hauser SoftFloat 法律声明如下
-------------------------------------------------------------------------------

SoftFloat 法律声明

SoftFloat John R. Hauser 编写。此项工作部分得益于国际计算机科学研究所（International Computer Science Institute，地址：Suite 600, 1947 Center Street, Berkeley, California 94704）的支持。资金部分由美国国家科学基金会（National Science Foundation）通过 MIP-9311980 号资助提供。该代码的原始版本是与加州大学伯克利分校合作构建定点向量处理器项目的一部分，由 Nelson Morgan John Wawrzynek 教授监督
本软件按“原样”免费分发。尽管已尽合理努力避免，本软件仍可能包含缺陷，并在某些情况下导致不正确行为。本软件的使用仅限于能够并且愿意对其使用所引起的任何及所有损失、成本或其他问题承担全部责任的个人和组织