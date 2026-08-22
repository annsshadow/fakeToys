## 如何正确使用 printk 格式说明

:Author: Randy Dunlap <rdunlap@infradead.org>
:Author: Andrew Murray <amurray@mpc-data.co.uk>


## 整数类型


```
	If variable is of Type,		use printk format specifier:
	------------------------------------------------------------
		signed char		%d or %hhx
		unsigned char		%u or %x
		char			%u or %x
		short int		%d or %hx
		unsigned short int	%u or %x
		int			%d or %x
		unsigned int		%u or %x
		long			%ld or %lx
		unsigned long		%lu or %lx
		long long		%lld or %llx
		unsigned long long	%llu or %llx
		size_t			%zu or %zx
		ssize_t			%zd or %zx
		s8			%d or %hhx
		u8			%u or %x
		s16			%d or %hx
		u16			%u or %x
		s32			%d or %x
		u32			%u or %x
		s64			%lld or %llx
		u64			%llu or %llx
```

如果 <type> 的大小依赖于体系结构（例cycles_t、tcflag_t），或者依赖于某个配置选项来决定大小（例如 blk_status_t），则应使用其可能的最大类型对应的格式说明符，并显式转换为该类型
```
	printk("test: latency: %llu cycles\n", (unsigned long long)time);
```

提醒：`sizeof()` 返回的类型是 size_t
内核printf 不支%n。浮点格式（%efga）出于显而易见的原因也不被识别。使用任何不受支持的说明符或长度限定符都会导致一WARN 并提早从 vsnprintf() 返回
## 指针类型

原始的指针值可以用 %p 打印，它会在打印前对地址进行哈希。内核还支持用于打印不同类型指针的扩展说明符
某些扩展说明符会打印给定地址上的数据，而不是打印地址本身。在这种情况下，会出现以下错误消息：

```
	(null)	 data on plain NULL address
	(efault) data on invalid address
	(einval) invalid data on a valid address
```

### 普通指

```
	%p	abcdef12 or 00000000abcdef12
```

不带扩展说明符（即裸%p）打印的指针会被哈希，以防止泄露内核内存布局信息。这还有一个额外的好处，就是提供了一个唯一标识符。在 64 位机器上，高 32 位被清零。内核会打印 `(ptrval)`，直到它收集到足够的熵为止
尽可能使用专门的修饰符（%pS %pB，见下文），以避免需要提供事后才能解读的未哈希地址。如果做不到，且打印地址的目的是为调试提供更多信息的，那么在调试期间使用 %p 并在内核启动时加`no_hash_pointers` 参数，它会打印所有未经修改的 %p 地址。如果你**确实**始终想要未修改的地址，请参阅下面%px
如果（且仅当）你打印地址是作为虚拟文件（例如 procfs sysfs 中，使用 seq_printf() 而非 printk() 读出）的内容、供用户空间进程读取，请使用下文描述%pK 修饰符，而不%p %px
### 错误指针


```
	%pe	-ENOSPC
```

用于将错误指针（IS_ERR() 为真的指针）作为符号化的错误名打印。没有已知符号名的错误值以十进制打印，而传%pe 的非 ERR_PTR 会被当作普通的 %p 处理
### 符号/函数指针


```
	%pS	versatile_init+0x0/0x110
	%ps	versatile_init
	%pSR	versatile_init+0x9/0x110
		(with __builtin_extract_return_addr() translation)
	%pB	prev_fn_of_versatile_init+0x88/0x88
```

`S` `s` 说明符用于以符号格式打印指针。它们产生带（S）或不带（s）偏移量的符号名。如果禁用了 KALLSYMS，则改为打印符号地址
`B` 说明符产生带偏移量的符号名，应在打印栈回溯时使用。该说明符会考虑在使用尾调用并以 GCC noreturn 属性标记时可能发生的编译器优化影响
如果指针位于某个模块内部，则模块名以及可选的 build ID 会紧接在符号名之后打印，并在说明符末尾附加一`b`
```
	%pS	versatile_init+0x0/0x110 [module_name]
	%pSb	versatile_init+0x0/0x110 [module_name ed5019fdf5e53be37cb1ba7899292d7e143b259e]
	%pSRb	versatile_init+0x9/0x110 [module_name ed5019fdf5e53be37cb1ba7899292d7e143b259e]
		(with __builtin_extract_return_addr() translation)
	%pBb	prev_fn_of_versatile_init+0x88/0x88 [module_name ed5019fdf5e53be37cb1ba7899292d7e143b259e]
```

### 来自 BPF / tracing 的探测指

```
	%pks	kernel string
	%pus	user string
```

`k` `u` 说明符用于打印先前探测到的、来自内核内存（k）或用户内存（u）的内存。后续的 `s` 说明符会打印一个字符串。在常规 vsnprintf() 中直接使用时，（k）和（u）注解会被忽略；但在 BPF bpf_trace_printk() 中使用时，例如，它会在不触发缺页的情况下读取其所指向的内存
### 内核指针


```
	%pK	01234567 or 0123456789abcdef
```

用于打印应对非特权用户隐藏的内核指针pK 的行为取决于 kptr_restrict sysctl——更多细节请参阅 Documentation/admin-guide/sysctl/kernel.rst
此修饰符***用于产生由用户空间从 procfs sysfs 读取的文件内容，而非用于 dmesg。有关如何在 printk() 中管理哈希指针的讨论，请参阅上面关于 %p 的章节
### 未修改的地址


```
	%px	01234567 or 0123456789abcdef
```

当你**确实**想要打印地址时使用。在打印之前，请考虑是否正在泄露有关内核内存布局的敏感信息px 在功能上等价%lx（或 %lu）。之所以优先选择 %px，是因为它更便于grep 搜索。如果将来我们需要修改内核处理指针打印的方式，我们将更容易找到调用点
在使%px 之前，请考虑是否仅使%p 并在调试会话期间启用 `no_hash_pointers` 内核参数（见上文 %p 的描述）就足够了。使%px 的一个合理场景是panic 之前立即打印信息，由panic 会阻止任何敏感信息被利用，而且使用 %px 就无需no_hash_pointers 来复现该 panic
### 指针差

```
	%td	2560
	%tx	a00
```

用于打印指针差值，ptrdiff_t 使用 %t 修饰符
```
	printk("test: difference between pointers: %td\n", ptr2 - ptr1);
```

### 结构体资源（struct resources

```
	%pr	[mem 0x60000000-0x6fffffff flags 0x2200] or
		[mem 0x60000000 flags 0x2200] or
		[mem 0x0000000060000000-0x000000006fffffff flags 0x2200]
		[mem 0x0000000060000000 flags 0x2200]
	%pR	[mem 0x60000000-0x6fffffff pref] or
		[mem 0x60000000 pref] or
		[mem 0x0000000060000000-0x000000006fffffff pref]
		[mem 0x0000000060000000 pref]
```

用于打印结构体资源。`R` `r` 说明符会产生带（R）或不带（r）已解码 flags 成员的打印资源。如start 等于 end，则只打start 值
通过引用传递
### 物理地址类型 phys_addr_t


```
	%pa[p]	0x01234567 or 0x0123456789abcdef
```

用于打印 phys_addr_t 类型（及其衍生类型，resource_size_t），它可以随构建选项变化，而与 CPU 数据通路的宽度无关
通过引用传递
### 结构体范围（struct range

```
	%pra    [range 0x0000000060000000-0x000000006fffffff] or
		[range 0x0000000060000000]
```

用于打印结构体范围。struct range 保存任意范围u64 值。如start 等于 end，则只打start 值
通过引用传递
### DMA 地址类型 dma_addr_t


```
	%pad	0x01234567 or 0x0123456789abcdef
```

用于打印 dma_addr_t 类型，它可以随构建选项变化，而与 CPU 数据通路的宽度无关
通过引用传递
### 作为转义字符串的原始缓冲

```
	%*pE[achnops]
```

```
		1b 62 20 5c 43 07 22 90 0d 5d
```

下面几个例子展示了转换是如何进行的（不包含外围的
```
		%*pE		"\eb \C\a"\220\r]"
		%*pEhp		"\x1bb \C\x07"\x90\x0d]"
		%*pEa		"\e\142\040\\\103\a\042\220\r\135"
```

转换规则根据可选的标志组合来应用（详情请参`string_escape_mem` 内核文档）：

 - a - ESCAPE_ANY
 - c - ESCAPE_SPECIAL
 - h - ESCAPE_HEX
 - n - ESCAPE_NULL
 - o - ESCAPE_OCTAL
 - p - ESCAPE_NP
 - s - ESCAPE_SPACE

默认使用 ESCAPE_ANY_NP
ESCAPE_ANY_NP 对许多情况都是合理的选择，尤其是在打SSID 时
如果省略了字段宽度，则只会转1 个字节
### 作为十六进制字符串的原始缓冲

```
	%*ph	00 01 02  ...  3f
	%*phC	00:01:02: ... :3f
	%*phD	00-01-02- ... -3f
	%*phN	000102 ... 3f
```

用于以小写十六进制字符串形式打印小型缓冲区（最64 字节），并带有某种分隔符。对于更大的缓冲区，考虑使用 `print_hex_dump`
### MAC/FDDI 地址


```
	%pM	00:01:02:03:04:05
	%pMR	05:04:03:02:01:00
	%pMF	00-01-02-03-04-05
	%pm	000102030405
	%pmR	050403020100
```

用于以十六进制表示法打印 6 字节MAC/FDDI 地址。`M` `m` 说明符会产生带（M）或不带（m）字节分隔符的打印地址。默认的字节分隔符是冒号）
对于 FDDI 地址，可`M` 说明符之后使`F` 说明符，以使用短横线）分隔符代替默认分隔符
对于蓝牙地址，应`M` 说明符之后使`R` 说明符，以使用反转的字节序，便于little endian 顺序排列的蓝牙地址进行直观解读
通过引用传递
### IPv4 地址


```
	%pI4	1.2.3.4
	%pi4	001.002.003.004
	%p[Ii]4[hnbl]
```

用于打印以点分隔的十进制 IPv4 地址。`I4` `i4` 说明符会产生带（i4）或不带（I4）前导零的打印地址
附加`h`、`n`、`b` `l` 说明符分别用于指定主机序、网络序、大端或小端序地址。若未提供说明符，则使用默认的网络序/大端序
通过引用传递
### IPv6 地址


```
	%pI6	0001:0002:0003:0004:0005:0006:0007:0008
	%pi6	00010002000300040005000600070008
	%pI6c	1:2:3:4:5:6:7:8
```

用于打印 IPv6 网络序的 16 位十六进制地址。`I6` `i6` 说明符会产生带（I6）或不带（i6）冒号分隔符的打印地址。始终使用前导零
附加`c` 说明符可`I` 说明符一起使用，以打印由 https://tools.ietf.org/html/rfc5952 描述的压IPv6 地址
通过引用传递
### IPv4/IPv6 地址（通用，带端口、flowinfo、作用域

```
	%pIS	1.2.3.4		or 0001:0002:0003:0004:0005:0006:0007:0008
	%piS	001.002.003.004	or 00010002000300040005000600070008
	%pISc	1.2.3.4		or 1:2:3:4:5:6:7:8
	%pISpc	1.2.3.4:12345	or [1:2:3:4:5:6:7:8]:12345
	%p[Ii]S[pfschnbl]
```

用于打印 IP 地址，而无需区分它是 AF_INET 还是 AF_INET6 类型。可以将指向有效 struct sockaddr 的指针（通过 `IS` `iS` 指定）传给此格式说明符
附加`p`、`f` `s` 说明符分别用于指定端口（IPv4、IPv6）、flowinfo（IPv6）和作用域（IPv6）。端口带`:` 前缀，flowinfo 带有 `/` 前缀，作用域带有 `%` 前缀，各自后跟实际值
对于 IPv6 地址，如果给出了附加说明`c`，则使用 https://tools.ietf.org/html/rfc5952 描述的压IPv6 地址。在带有附加说明`p`、`f` `s` 的情况下，IPv6 地址会被 `[`、`]` 包围，正https://tools.ietf.org/html/draft-ietf-6man-text-addr-representation-07 所建议的
对于 IPv4 地址，也可以同样使用附加`h`、`n`、`b` `l` 说明符，而在 IPv6 地址情况下它们会被忽略
通过引用传递
```
	%pISfc		1.2.3.4		or [1:2:3:4:5:6:7:8]/123456789
	%pISsc		1.2.3.4		or [1:2:3:4:5:6:7:8]%1234567890
	%pISpfc		1.2.3.4:12345	or [1:2:3:4:5:6:7:8]:12345/123456789
```

### UUID/GUID 地址


```
	%pUb	00010203-0405-0607-0809-0a0b0c0d0e0f
	%pUB	00010203-0405-0607-0809-0A0B0C0D0E0F
	%pUl	03020100-0504-0706-0809-0a0b0c0e0e0f
	%pUL	03020100-0504-0706-0809-0A0B0C0E0E0F
```

用于打印 16 字节UUID/GUID 地址。附加的 `l`、`L`、`b` `B` 说明符用于以小端序（l 小写L 大写十六进制）或大端序（b 小写B 大写十六进制）指定地址
未使用附加说明符时，将打印默认的大端序小写十六进制表示
通过引用传递
### dentry 名称


```
	%pd{,2,3,4}
	%pD{,2,3,4}
```

用于打印 dentry 名称；如果我们与 `d_move` 发生竞争，名称可能是新旧名称的混合，但不会发oopspd dentry 是我们过去使用的 %s dentry->d_name.name 的一个更安全的等价形式，%pd<n> 打印最`n` 个分量pD struct file 做同样的事情
通过引用传递
### block_device 名称


```
	%pg	sda, sda1 or loop0p1
```

用于打印 block_device 指针的名称
### struct va_format


```
	%pV
```

用于打印 struct va_format 结构体。它们包含一个格式字符串
```
	struct va_format {
		const char *fmt;
		va_list *va;
	};
```

实现了一递归 vsnprintf"
不要在没有某种机制来验证格式字符串和 va_list 参数正确性的情况下使用此功能
通过引用传递
### 设备树节

```
	%pOF[fnpPcCF]
```

用于打印设备树节点结构体。默认行为等价于 %pOFf
 - f - 设备节点 full_name
 - n - 设备节点 name
 - p - 设备节点 phandle
 - P - 设备节点路径规格（name + @unit - F - 设备节点 flags
 - c - 主要 compatible 字符 - C - 完整 compatible 字符
使用多个参数时，分隔符为 ':'
```
	%pOF	/foo/bar@0			- 节点全名
	%pOFf	/foo/bar@0			- 同上
	%pOFfp	/foo/bar@0:10			- 节点全名 + phandle
	%pOFfcF	/foo/bar@0:foo,device:--P-	- 节点全名 +
	                                          major compatible 字符+
						  节点 flags
							D - 动态（dynamic							d - 已分离（detached							P - 已填充（Populated							B - 已填充总线（Populated bus```

通过引用传递
### Fwnode 句柄


```
	%pfw[fP]
```

用于打印有关 fwnode_handle 的信息。默认是打印完整的节点名，包括路径。这些修饰符在功能上等价于上面的 %pOF
 - f - 节点的全名，包括路径
 - P - 节点的名称，包括地址（如果有
```
	%pfwf	\_SB.PCI0.CIO2.port@1.endpoint@0	- 完整节点	%pfwP	endpoint@0				- 节点```

```
	%pfwf	/ocp@68000000/i2c@48072000/camera@10/port/endpoint - 全名
	%pfwP	endpoint				- 节点```

### 时间与日

```
	%pt[RT]			YYYY-mm-ddTHH:MM:SS
	%pt[RT]s		YYYY-mm-dd HH:MM:SS
	%pt[RT]d		YYYY-mm-dd
	%pt[RT]t		HH:MM:SS
	%ptSp			<seconds>.<nanoseconds>
	%pt[RST][dt][r][s]
```

```
	R  struct rtc_time 的内	S  struct timespec64 的内	T  time64_t 类型
```

以人类可读的格式
默认情况下年份会1900，月份会1。使%pt[RT]r（原始）来抑制此行为
%pt[RT]s（空格）会用 ' '（空格）代替日期与时间之间的 ISO 8601 分隔'T'（大T）。当日期或时间被省略时它不起作用
%ptSp 等价struct timespec64 内容%lld.%09ld。当给出其他说明符时，它就变%ptT[dt][r][s].%09ld 的相应等价形式。换句话说，秒以人类可读的格式打印，后跟一个点以及纳秒
通过引用传递
### struct clk


```
	%pC	pll1
```

用于打印 struct clk 结构体pC 打印时钟（通用时钟框架，Common Clock Framework）的名称，或一个唯一32 ID（遗留时钟框架）
通过引用传递
### 位图及其衍生类型（如 cpumask nodemask

```
	%*pb	0779
	%*pbl	0,3-6,8-10
```

用于打印位图及其衍生类型（如 cpumask nodemask），%*pb 以字段宽度为位数输出位图*pbl 以字段宽度为位数将位图作为范围列表输出
字段宽度按值传递，位图通过引用传递。辅助宏 cpumask_pr_args() nodemask_pr_args() 可用于简cpumask nodemask 的打印
### 标志位域（如页标志和 gfp_flags

```
	%pGp	0x17ffffc0002036(referenced|uptodate|lru|active|private|node=0|zone=2|lastcpupid=0x1fffff)
	%pGg	GFP_USER|GFP_DMA32|GFP_NOWARN
	%pGv	read|exec|mayread|maywrite|mayexec|denywrite
```

用于将标志位域作为一组构造该值的符号常量打印。标志的类型由第三个字符给出。目前支持的有：

        - p - [p]age 标志，期望类型为 (`unsigned long *`) 的        - v - [v]ma_flags，期望类型为 (`unsigned long *`) 的        - g - [g]fp_flags，期望类型为 (`gfp_t *`) 的
标志名和打印顺序取决于具体类型
注意，此格式不应直接tracepoint `TP_printk()` 部分中使用。而应改用 <trace/events/mmflags.h> 中的 show_*_flags() 函数
通过引用传递
### 网络设备特

```
	%pNF	0x000000000000c000
```

用于打印 netdev_features_t
通过引用传递
### V4L2 DRM FourCC 码（像素格式

```
	%p4cc
```

打印 V4L2 DRM 使用FourCC 码，包括格式字节序及其十六进制数值
通过引用传递
```
	%p4cc	BG12 little-endian (0x32314742)
	%p4cc	Y10  little-endian (0x20303159)
	%p4cc	NV12 big-endian (0xb231564e)
```

### 閫氱敤 FourCC 鐮?

```
	%p4c[h[R]lb]	gP00 (0x67503030)
```

打印通用FourCC 码，同时ASCII 字符及其十六进制数值形式输出
通用 FourCC 码总是以大端格式打印，即最高有效字节在前。这V4L/DRM FourCC 相反
附加`h`、`hR`、`l` `b` 说明符定义了用于加载所存储字节的字节序。数据可能被解释为主机序、反主机字节序、小端序或大端序
通过引用传递
```
	%p4ch	gP00 (0x67503030)
	%p4chR	00Pg (0x30305067)
	%p4cl	gP00 (0x67503030)
	%p4cb	00Pg (0x30305067)
```

```
	%p4ch	gP00 (0x67503030)
	%p4chR	00Pg (0x30305067)
	%p4cl	00Pg (0x30305067)
	%p4cb	gP00 (0x67503030)
```

### Rust


```
	%pA
```

**仅打算用于从 Rust 代码格式* ``core``
: fmt::Arguments``*不要**C 代码中使用它
## 致谢


如果你添加了其他 %p 扩展，请在可行的情况下用一个或多个测试用例扩展 <lib/tests/printf_kunit.c>
感谢你的配合与关注