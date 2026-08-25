
## Printk 索引（Printk Index

有许多方式可以监控系统状态。一个重要的信息来源是系统日志。它提供了大量信息，
包括或多或少的警告与错误消息
有一些监控工具会根据记录的消息进行过滤并采取行动
内核消息是随代码一起演化的。因此，特定的内核消息不KABI，也永远不会是！

维护系统日志监视器是一个巨大的挑战。它要求知道在某个特定内核版本中哪些消息更新了、以及为什么。在源码中找到这些变化需要相当复杂的解析器。而且它还需要将
源码与二进制内核匹配，这并非总是易事。各种更改可能被反向移植（backport）。不同的
被监控系统上可能使用不同的内核版本
这正printk 索引特性可能有用的地方。它提供了一份运行中系统上内核及模块所用源printk 格式的转储。它可以通过 debugfs 在运行时访问
printk 索引有助于发现消息格式中的变化。同时它也有助于将字符串回溯到内核源码及
相关的提交

## 用户接口


printk 格式的索引被拆分到独立的文件中。这些文件根据该 printk 格式内置（built-in所在的二进制文件来命名。有
```

   /sys/kernel/debug/printk/index/vmlinux
   /sys/kernel/debug/printk/index/ext4
   /sys/kernel/debug/printk/index/scsi_mod

```
注意只显示已加载的模块。当某个模块被内置时，它printk 格式也可能出现在
"vmlinux" 中
```

   $> head -1 /sys/kernel/debug/printk/index/vmlinux; shuf -n 5 vmlinux
   # <level[,flags]> filename:line function "format"
   <5> block/blk-settings.c:661 disk_stack_limits "%s: Warning: Device %s is misaligned\n"
   <4> kernel/trace/trace.c:8296 trace_create_file "Could not create tracefs '%s' entry\n"
   <6> arch/x86/kernel/hpet.c:144 _hpet_print_config "hpet: %s(%d):\n"
   <6> init/do_mounts.c:605 prepare_namespace "Waiting for root device %s...\n"
   <6> drivers/acpi/osl.c:1410 acpi_no_auto_serialize_setup "ACPI: auto-serialization disabled\n"

```
，其含义为：

   - :level: 日志级别值：特定严重程度0-71 为默认，'c' 为没有明确日志级别的
	连续   - :flags: 可选标志：目前只有 'c' 表示 KERN_CONT
   - :filename\:line: 相关 printk() 调用的源文件名和行号。注意有许多包装函数	例如 pr_warn()、pr_warn_once()、dev_warn()   - :function: 使用 printk() 调用的函数名   - :format: 格式字符
这些额外信息使得在不同内核之间查找差异稍微困难一些。尤其是行号可能经常变化另一方面，它非常有助于确认是同一个字符串，或者找到负责最终变化的提交

## printk() 不是稳定KABI


一些开发者担心，将这些实现细节全部导出到用户空间会把特定printk() 调用变成
KABI銆。
但事实恰恰相反。printk() 调用**绝不**应该KABI。printk 索引帮助用户空间
工具应对这一点

## 子系统特定的 printk 包装函数


printk 索引是使用存储在专用 .elf ".printk_index" 中的额外元数据生成的。这通过宏包装函数与真正printk() 调用一起执__printk_index_emit() 来实现的动态调试（dynamic debug）特性所使用的元数据也采用了相同的技术
这些元数据只有在使用这些特殊包装函数打印特定消息时才会被存储。它针对常用printk() 调用实现，包括例pr_warn() pr_once()
对于通过各种子系统特定的包装函数（它们通过公共辅助函数调用原始printk()）需做额外的更改。这些需要它们自己的包装函数来添__printk_index_emit()
到目前为止只有少数子系统特定的包装函数被更新，例dev_printk()。因此，某些系统printk 格式可能会缺失于 printk 索引中

## 子系统特定的前缀


pr_fmt() 允许定义一个前缀，它会被打印在相printk() 调用生成的字符串之前
子系统特定的包装函数通常会添加更复杂的后缀
这些前缀可以通过 __printk_index_emit() 的一个可选参数存储到 printk 索引元数据中debugfs 接口随后可能会显示包含这些前缀printk 格式```

  #define pr_fmt(fmt) "ACPI: OSL: " fmt

  static int __init acpi_no_auto_serialize_setup(char *str)
  {
	acpi_gbl_auto_serialize_methods = FALSE;
	pr_info("Auto-serialization disabled\n");

	return 1;
  }

```
```

  <6> drivers/acpi/osl.c:1410 acpi_no_auto_serialize_setup "ACPI: auto-serialization disabled\n"

```
它有助于将真实日志中的消息与 printk 索引匹配。然后源文件名、行号和函数名可用来将字符串与源代码匹配