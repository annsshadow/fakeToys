Dynamic debug
+++++++++++++

## 简介（Introduction
Dynamic debug（动态调试）允许你动态地启用/禁用内核的调试打印代码，以获取额外的内核信息
如果 `/proc/dynamic_debug/control` 存在，说明你的内核支dynamic debug。你需root 权限
（sudo su）来使用它
Dynamic debug 提供
 - 一份你内核中所**prdbg** 的目录（Catalog）   `cat /proc/dynamic_debug/control` 即可查看它们
 - 一种简单的查询/命令语言，通过从以下属性中任0 个或 1 个的组合来选取并修**prdbg**
   - 源文件名（source filename   - 函数名（function name   - 行号（包括行号范围）
   - 模块名（module name   - 格式字符串（format string   - 类名（class name，由每个模块所所声明
注意：要真正在控制台上看到调试打印输出，你可能需要调整内核的 `loglevel=`，或使用
`ignore_loglevel`。关于这些内核参数，请参Documentation/admin-guide/kernel-parameters.rst
## 查看 Dynamic Debug 行为（Viewing Dynamic Debug Behaviour
```

  :#> head -n7 /proc/dynamic_debug/control
  # filename:lineno [module]function flags format
  init/main.c:1179 [main]initcall_blacklist =_ "blacklisting initcall %s\012
  init/main.c:1218 [main]initcall_blacklisted =_ "initcall %s blacklisted\012"
  init/main.c:1424 [main]run_init_process =_ "  with arguments:\012"
  init/main.c:1426 [main]run_init_process =_ "    %s\012"
  init/main.c:1427 [main]run_init_process =_ "  with environment:\012"
  init/main.c:1429 [main]run_init_process =_ "    %s\012"

```
3 个以空格分隔的列显示当前flags（标志），前面带`=` 以方便与 grep/cut 配合使用`=p` 表示已启用的调用点（callsite）
## 控制 dynamic debug 行为（Controlling dynamic debug Behaviour
**prdbg** 调用点的行为是通过写入以下内容来控制的```

  # grease the interface
  :#> alias ddcmd='echo $* > /proc/dynamic_debug/control'

  :#> ddcmd '-p; module main func run* +p'
  :#> grep =p /proc/dynamic_debug/control
  init/main.c:1424 [main]run_init_process =p "  with arguments:\012"
  init/main.c:1426 [main]run_init_process =p "    %s\012"
  init/main.c:1427 [main]run_init_process =p "  with environment:\012"
  init/main.c:1429 [main]run_init_process =p "    %s\012"

```
```

  :#> ddcmd mode foo +p
  dyndbg: unknown keyword "mode"
  dyndbg: query parse failed
  bash: echo: write error: Invalid argument

```
如果 debugfs 也已启用并挂载，那么 `dynamic_debug/control` 也会位于挂载目录下，通常`/sys/kernel/debug/`
## 命令语言参考（Command Language Reference
在基本的词法层面，一条命令是一系列由空格分隔的单词
```

  :#> ddcmd file svcsock.c line 1603 +p
  :#> ddcmd "file svcsock.c line 1603 +p"
  :#> ddcmd '  file   svcsock.c     line  1603 +p  '

```
命令提交以一write() 系统调用为界```

  :#> ddcmd "func pnpacpi_get_resources +p; func pnp_assign_mem +p"
  :#> ddcmd <<"EOC"
  func pnpacpi_get_resources +p
  func pnp_assign_mem +p
  EOC
  :#> cat query-batch-file > /proc/dynamic_debug/control

```
你还可以在每个查询项中使用通配符。匹配规则支`*` （匹配零个或多个字符）和 `` （精确匹一个字符）```

  :#> ddcmd file "drivers/usb/*" +p	# "" 用于抑制 shell 展开

```
从语法上讲，一条命令是成对的关键字-值，后跟一```

  command ::= match-spec* flags-spec

```
match-spec 从目录中选择 **prdbg**，然后在其上应用 flags-spec，所有约束之间是与（AND）的关系省略的关键字等同于关键字 "*"
match 规范由一个关键字（用于选择要比较的调用点的属性）和一个用于比较的值组成。可能的
关键字如下：
```

  match-spec ::= 'func' string |
		 'file' string |
		 'module' string |
		 'format' string |
		 'class' string |
		 'line' line-range

  line-range ::= lineno |
		 '-'lineno |
		 lineno'-' |
		 lineno'-'lineno

  lineno ::= unsigned-int

```
  `line-range` 不能包含空格，例"1-30" 是有效的范围，但 "1 - 30" 不是
每个关键字的含义如下
func
    给定的字符串与函数名进行比较
```

	func svc_tcp_accept
	func *recv*		# in rfcomm, bluetooth, ping, tcp

```
file
    给定的字符串与每个调用点的、相对于 src-root 的路径名或源文件基名进行比较
```

	file svcsock.c
	file kernel/freezer.c	# 即控制文件第 1 	file drivers/usb/*	# 其下的所有调用点
	file inode.c:start_*	# :tail 解析func（见上）
	file inode.c:1-100	# :tail 解析line-range（见上）

```
module
    给定的字符串与每个调用点的模块名进行比较。模块名`lsmod` 中看到的字符串，即不    目录也不`.ko` 后缀
```

	module sunrpc
	module nfsd
	module drm*	# 同时匹配 drm drm_kms_helper

```
format
    给定的字符串会在 dynamic debug format 字符串中搜索。注意，字符串不需要匹配整    format，只需匹配其中一部分即可。空白字符和其它特殊字符可以使用 C 的八进制转义
    `\ooo` 表示法来转义，例如空格字符是 `\040`。另外，字符串也可以用双引号（`"`）或
    单引号（`'`）括起来```

	format svcrdma:         // 许多 NFS/RDMA 服务pr_debug
	format readahead        // readahead 缓存中的部分 pr_debug
	format nfsd:\040SETATTR // 匹配带空白的 format 的一种方	format "nfsd: SETATTR"  // 匹配带空白的 format 的一种更整洁的方	format 'nfsd: SETATTR'  // 又一种匹配带空白format 的方
```
class
    给定class_name 会针对每个模块进行校验，模块可能已经声明了一个已知的 class_name 列表    如果某个模块找到了该 class_name，则调用点与类的匹配和调```

	class DRM_UT_KMS	# 一DRM.debug 类别
	class JUNK		# 静默不匹	// class TLD_*		# 注意：class 名中不支持通配
```
line
    给定的单个行号或行号范围会与每个 `pr_debug()` 调用点的行号进行比较。单个行号会精确匹配
    调用点的行号。行号范围会匹配从首行号到末行号（含）之间的任何调用点。首行号为空表示文件
    中的第一行，末行号为空表```

	line 1603           // 精确匹配1603 	line 1600-1605      // 从第 1600 行到1605 行的六行
	line -1605          // 从第 1 行到1605 行的 1605 	line 1600-          // 从第 1600 行到文件末尾的所有行

```
flags 规范由一个修改操作，后跟一个或多个标志字符组成。修改操作是下列之一```

  -    remove the given flags
  +    add the given flags
  =    set the flags to the given flags

```
```

  p    enables the pr_debug() callsite.
  _    enables no flags.

  Decorator flags add to the message-prefix, in order:
  t    Include thread ID, or <intr>
  m    Include module name
  f    Include the function name
  s    Include the source file name
  l    Include line number
  d    Include call trace

```
对于 `print_hex_dump_debug()` `print_hex_dump_bytes()`，只`p` 标志有意义，其它标志
会被忽略
注意，正则表达式 `^[-+=][fslmptd_]+$` 匹配一flags 规范。要一次性清除所有标志，可以使用
`=_` 鎴?`-fslmptd`銆。
## 启动过程中的调试消息（Debug messages during Boot Process
要在启动过程中（甚至早于用户空间debugfs 出现之前）激活核心代码与内建模块的调试消息，
可使`dyndbg="QUERY"` `module.dyndbg="QUERY"`。QUERY 遵循上述语法，但不得超过 1023
个字符。你bootloader 可能会施加更低的限制
这些 `dyndbg` 参数会在 ddebug 表被处理之后、作early_initcall 的一部分被处理。因此，可以通过这个启动参数，启用在early_initcall 之后运行的所有代码中的调试消息```

   dyndbg="file ec.c +p"

```
如果你的机器（通常是笔记本）带有嵌入式控制器（Embedded Controller），上述命令会在 ACPI 设置
期间显示早期的嵌入式控制器事务。PCI（或其它设备）初始化也是使用该启动参数进行调试的热门
候选场景
如果 `foo` 模块不是内建的，`foo.dyndbg` 仍会在启动时处理，但不会有任何效果，不过它会在模稍后被加载时重新被处理。单独的 `dyndbg=` 只在启动时处理
## 模块初始化时的调试消息（Debug Messages at Module Initialization Time
当调`modprobe foo` 时，modprobe 会扫`/proc/cmdline` 中的 `foo.params`，去`foo.`并与 modprobe 参数`/etc/modprobe.d/*.conf` 文件中给定的参数一起传给内核，顺序如下```

	options foo dyndbg=+pt
	options foo dyndbg # defaults to +p

```
```

	foo.dyndbg=" func bar +p; func buz +mp"

```
```

	modprobe foo dyndbg==pmf # override previous settings

```
这些 `dyndbg` 查询按顺序应用，最后一条具有最终决定权。这样，启动参数可以覆盖或修改来`/etc/modprobe.d` 的设置（这很合理，因1 是系统范围的 是内核或启动特定的），modprobe
参数则可以覆盖这两者
`foo.dyndbg="QUERY"` 形式中，查询必须排除 `module foo`。`foo` 会从参数名中提取出来，并
应用`QUERY` 中的每个查询，并且每种类型只允许一match-spec
`dyndbg` 选项是一模块参数，这意味着
- 模块不需要显式定义它
- 每个模块都会隐式获得它，无论是否使用pr_debug
- 它不会出现在 `/sys/module/$module/parameters/`   要查看它，可grep 控制文件，或检`/proc/cmdline.`

对于 `CONFIG_DYNAMIC_DEBUG` 内核，启动时给定的任何设置（或在编译期间`-DDEBUG` 标志启用
的）之后都可以通过如下方式禁用```

   echo "module module_name -p" > /proc/dynamic_debug/control

```
## 示例（Examples
```

  // enable the message at line 1603 of file svcsock.c
  :#> ddcmd 'file svcsock.c line 1603 +p'

  // enable all the messages in file svcsock.c
  :#> ddcmd 'file svcsock.c +p'

  // enable all the messages in the NFS server module
  :#> ddcmd 'module nfsd +p'

  // enable all 12 messages in the function svc_process()
  :#> ddcmd 'func svc_process +p'

  // disable all 12 messages in the function svc_process()
  :#> ddcmd 'func svc_process -p'

  // enable messages for NFS calls READ, READLINK, READDIR and READDIR+.
  :#> ddcmd 'format "nfsd: READ" +p'

  // enable messages in files of which the paths include string "usb"
  :#> ddcmd 'file *usb* +p'

  // enable all messages
  :#> ddcmd '+p'

  // add module, function to all enabled messages
  :#> ddcmd '+mf'

  // boot-args example, with newlines and comments for readability
  Kernel command line: ...
    // see what's going on in dyndbg=value processing
    dynamic_debug.verbose=3
    // enable pr_debugs in the btrfs module (can be builtin or loadable)
    btrfs.dyndbg="+p"
    // enable pr_debugs in all files under init/
    // and the function parse_one, #cmt is stripped
    dyndbg="file init/* +p #cmt ; func parse_one +p"
    // enable pr_debugs in 2 functions in a module loaded later
    pc87360.dyndbg="func pc87360_init_device +p; func pc87360_find +p"

```
## 内核配置（Kernel Configuration
```

  CONFIG_DYNAMIC_DEBUG=y	# build catalog, enables CORE
  CONFIG_DYNAMIC_DEBUG_CORE=y	# enable mechanics only, skip catalog

```
如果你不想全局启用 dynamic debug（例如在某些嵌入式系统中），你可以把 `CONFIG_DYNAMIC_DEBUG_CORE`
设置dynamic debug 的基础支持，并在你希望稍后进行动态调试的任何模块Makefile 中加`ccflags := -DDYNAMIC_DEBUG_MODULE`
## 内核 *prdbg* API

以下函数在启dynamic debug 时会被编入目录并可被控制```

  pr_debug()
  dev_dbg()
  print_hex_dump_debug()
  print_hex_dump_bytes()

```
否则，它们默认是关闭的；在源文件中使`ccflags += -DDEBUG` `#define DEBUG` 会适当启用它们
如果未设`CONFIG_DYNAMIC_DEBUG`，则 `print_hex_dump_debug()` 只是 `print_hex_dump(KERN_DEBUG)`
的快捷方式
对于 `print_hex_dump_debug()`/`print_hex_dump_bytes()`，其 format 字符串是 `prefix_str`
参数（如果它是常量字符串），或者是`prefix_str` 被动态构造时`hexdump`