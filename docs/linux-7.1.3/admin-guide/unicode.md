## Unicode 支持


		 最后更新：2005-01-17，版1.4

注意：本文档的原始版本由 lanana.org 作为 Linux 已分配名称与编号管理局
（LANANA）项目的一部分维护，现已不复存在。因此，主线 Linux 内核中的
这个版本现在成为受维护的主文档
### 简

Linux 内核代码已被重写，使Unicode 将字符映射到字体。通过下载一Unicode 到字体的表格，八位字符集UTF-8 模式都会被改为使用所指示的字体
这微妙地改变了八位字符表的语义。现在四个字符表如下
=============== =============================== ================
映射符号	映射名称		转义(G0)
=============== =============================== ================
LAT1_MAP	Latin-1 (ISO 8859-1)		ESC ( B
GRAF_MAP	DEC VT100 伪图	ESC ( 0
IBMPC_MAP	IBM 代码437		ESC ( U
USER_MAP	用户定义			ESC ( K
=============== =============================== ================

尤其是，ESC ( U 不再直接送字，因为字体可能与 IBM 字符集完全不同这样例如即便加载Latin-1 字体也能使用块图形
注意，尽管这些代码与 ISO 2022 类似，但无论是代码本身还是其使用方式都与
ISO 2022 不匹配；Linux 有两8 位代码（G0 G1），ISO 2022 有四7 位代码（G0-G3）
根据 Unicode 标准/ISO 10646，范U+F000 U+F8FF 被保留用于操作系范围的分配（Unicode 标准称之企业，由于这Linux 不准确，我们称之"Linux ）。选择 U+F000 作为起点，是因为它可以让直接映射区从一个大2 的幂开始（以防日后确实需1024 2048 字符的字体）。这就将 U+E000 U+EFFF 留作最终用户区
[v1.2]：从 U+F000 U+F7FF Unicode 范围已被硬编码为直接映射到所加载字体，绕过翻译表。用户自定义映射现在默认指向 U+F000 U+F0FF，模拟了之前
的行为。实际上该范围可能更短；例如 vgacon 只能处理 256 字符
（U+F000..U+F0FF）或 512 字符（U+F000..U+F1FF）的字体

### Linux 区中实际分配的字

此外，还定义了以Unicode 1.1.4 中不存在的字符；这些DEC VT 图形映射
使用。[v1.2] 此用法已过时，不应再使用；请参阅下文
====== ======================================
U+F800 DEC VT 图形 水平扫描 1
U+F801 DEC VT 图形 水平扫描 3
U+F803 DEC VT 图形 水平扫描 7
U+F804 DEC VT 图形 水平扫描 9
====== ======================================

DEC VT220 使用 6x10 字符矩阵，这些字符在 DEC VT 图形字符集中形成平滑递进。我略去了扫5 的线，因为它也被用作块图字符，因此被编码U+2500 细水平线
[v1.3]：这些字符已被正式加Unicode 3.2.0；它们被添加U+23BA、U+23BBU+23BC、U+23BD。Linux 现在使用新的值
[v1.2]：添加了以下字符以表示常见的键盘符号，这些符号不太可能被正式加入
Unicode，因为它们是极强的厂商专有内容。这当然是一个糟糕设计的绝佳范例
====== ======================================
U+F810 键盘符号 飞行旗帜
U+F811 键盘符号 下拉菜单
U+F812 键盘符号 开苹果
U+F813 键盘符号 实心苹果
====== ======================================

### 克林贡语支持


1996 年，Linux 成为世界上第一个添加对人造语言克林贡语支持的操作系统，该语言
Marc Okrand 为《星际迷航》电视剧创造。这种编码后来被 ConScript Unicode
注册表采纳，并被提议（但最终被拒绝）纳Unicode 平面 1。因此，它作Linux/CSUR 的私有分配保留在 Linux 区中
此编码已得到克林贡语言研究所的认可。更多信息请联系他们
	http://www.kli.org/

由于 Linux CZ 开头的字符多为装饰符号/符号/表单类，而这是一种语言，我将其
放在末尾，位于一16 单元的边界上，以符合标准 Unicode 惯例
  此范围现在由 ConScript Unicode 注册表正式管理。规范性参考位于：

	https://www.evertype.com/standards/csur/klingon.html

克林贡语26 个字符的字母表、一个带 10 个数字的位置数字书写系统，书写方向为
从左到右、从上到下
已经提出了几种克林贡字母的字形形式。然而，由于符号集似乎整体一致，只有实际
形状不同，按照标Unicode 惯例，这些差异被视为字体变体
======	=======================================================
U+F8D0	KLINGON LETTER A
U+F8D1	KLINGON LETTER B
U+F8D2	KLINGON LETTER CH
U+F8D3	KLINGON LETTER D
U+F8D4	KLINGON LETTER E
U+F8D5	KLINGON LETTER GH
U+F8D6	KLINGON LETTER H
U+F8D7	KLINGON LETTER I
U+F8D8	KLINGON LETTER J
U+F8D9	KLINGON LETTER L
U+F8DA	KLINGON LETTER M
U+F8DB	KLINGON LETTER N
U+F8DC	KLINGON LETTER NG
U+F8DD	KLINGON LETTER O
U+F8DE	KLINGON LETTER P
U+F8DF	KLINGON LETTER Q
 - 在标Okrand 拉丁转写中写<q>
U+F8E0	KLINGON LETTER QH
 - 在标Okrand 拉丁转写中写<Q>
U+F8E1	KLINGON LETTER R
U+F8E2	KLINGON LETTER S
U+F8E3	KLINGON LETTER T
U+F8E4	KLINGON LETTER TLH
U+F8E5	KLINGON LETTER U
U+F8E6	KLINGON LETTER V
U+F8E7	KLINGON LETTER W
U+F8E8	KLINGON LETTER Y
U+F8E9	KLINGON LETTER 澹伴棬鍋滈】绗。
U+F8F0	KLINGON DIGIT ZERO
U+F8F1	KLINGON DIGIT ONE
U+F8F2	KLINGON DIGIT TWO
U+F8F3	KLINGON DIGIT THREE
U+F8F4	KLINGON DIGIT FOUR
U+F8F5	KLINGON DIGIT FIVE
U+F8F6	KLINGON DIGIT SIX
U+F8F7	KLINGON DIGIT SEVEN
U+F8F8	KLINGON DIGIT EIGHT
U+F8F9	KLINGON DIGIT NINE

U+F8FD	KLINGON COMMA
U+F8FE	KLINGON FULL STOP
U+F8FF	KLINGON SYMBOL FOR EMPIRE
======	=======================================================

### 其它虚构与人工文

自从分配了克林贡 Linux Unicode 块以来，一个虚构与人工文字的注册表已由
John Cowan <jcowan@reutershealth.com> 鍜?Michael Everson <everson@evertype.com>
建立。ConScript Unicode 注册表位于：

	  https://www.evertype.com/standards/csur/

所使用的范围位于最终用户区的低端，因此不能被规范性地分配，但建议希望编码
虚构文字的人出于互操作性的考虑使用这些代码。对于克林贡语，CSUR 已采Linux 编码。CSUR 方面正在推动Tengwar Cirth 加入 Unicode 平面 1；将克林贡语
加入 Unicode 平面 1 已被拒绝，因此上述编码仍然为官方编码