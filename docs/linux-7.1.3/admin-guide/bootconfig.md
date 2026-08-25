

## Boot Configuration


:Author: Masami Hiramatsu <mhiramat@kernel.org>

## Overview


引导配置（boot configuration）扩展了当前的内核命令行，以便在内核引导时以高效的方式支持额外的键值数据。这允许管理员传递一个结构化的键值配置文件

## Config File Syntax


引导配置的语法是一种简单的结构化键值形式。每个键由以句点连接的单词组成，键与值之间用 `=` 连接。值字符串必须由下文描述的以下分隔符之一终止

每个键单词只能包含字母、数字、连字符（`-`）或下划线（`_`）。而每个值只能包含可打印字符或空格，分隔符除外，例如分号（`;`）、换行（`\n`）、逗号（`,`）、井号（`#`）与右花括号（`}`）

如果 `=` 之后到其中一个分隔符之间仅为空白字符，则该键被赋予一个空值

对于数组，数组值以逗号（`,`）分隔，并且为了可读性，允许在数组值之间使用注释以及换行（`\n`）。因此，数组的第一个元素必须与（键）位于同一行上
```

  KEY[.WORD[...]] = VALUE[, VALUE2[...]][;]

```
与内核命令行语法不同，逗号`=` 周围的空白字符（包括制表符）会被忽略

如果要在值中使用这些分隔符，可以使用双引号（`"VALUE"`）或单引号（`'VALUE'`）将其括起来。请注意，这些引号无法被转义

可以存在一个没有值或具有空值的键。这类键用于检查该键是否存在（类似于布尔值）

### Key-Value Syntax


引导配置文件语法允许用户合并部分相同的单词键
```

 foo.bar.baz = value1
 foo.bar.qux.quux = value2

```
```

 foo.bar {
    baz = value1
    qux.quux = value2
 }

```
```

 foo.bar { baz = value1; qux.quux = value2 }

```
在这两种风格中，相同的键单词在引导时解析时会自动合并。因此你可以追加相似的树或键值

### Same-key Values


禁止两个或更多的值或数组共享同一个键
```

 foo = bar, baz
 foo = qux  # !ERROR! we can not re-define same key

```
如果要更新值，必须使用覆盖操作
```

 foo = bar, baz
 foo := qux

```
然后，`qux` 会被赋给 `foo` 键。这对于通过添加（部分）自定义引导配置来覆盖默认值非常有用，而无需解析默认引导配置

如果要将值作为数组成员追加到现有键上
```

 foo = bar, baz
 foo += qux

```
在此情况下，`foo` 拥有 `bar`、`baz` `qux`

此外，子键与值可以在一个父键下共存
```

 foo = value1
 foo.bar = value2
 foo := value3 # This will update foo's value.

```
注意，由于没有语法可将原始值直接放在（一个父键）
```

 foo {
     bar = value1
     bar {
         baz = value2
         qux = value3
     }
 }

```
另外，键下值节点的顺序是固定的。如果既存在值又存在子键，则值始终是第一个子节点
```

 foo.bar = value1
 foo = value2

```
```

 foo = value2
 foo.bar = value1

```
### Comments


该配置语法接shell 脚本风格的注释。以井号#"）开始直到换行（"\n"）的注释将被忽略

```

 # comment line
 foo = value # value is set to foo.
 bar = 1, # 1st element
       2, # 2nd element
       3  # 3rd element

```
```

 foo = value
 bar = 1, 2, 3

```
注意，不能在值与分隔符之间放置注释或换行
```

 key = 1 # comment
       ,2


```
## /proc/bootconfig


/proc/bootconfig 是引导配置的用户空间接口。与 /proc/cmdline 不同，该文件显示键值风格的列表
```

 KEY[.WORDS...] = "[VALUE]"[,"VALUE2"...]


```
## Boot Kernel With a Boot Config


使用引导配置引导内核有两种方式：将引导配置附加到 initrd 镜像，或将其内嵌到内核本身中

### Attaching a Boot Config to Initrd


由于引导配置文件默认initrd 一起加载，它会被以填充、大小、校验和以及 12 字节魔数（magic word）的形式追加initrd（initramfs）镜像文件的末尾，如下所示

[initrd][bootconfig][padding][size(le32)][checksum(le32)][#BOOTCONFIG\n]

大小和校验和字段均为无符32 位小端值

当引导配置被添加initrd 镜像时，整个文件大小会对齐到 4 字节。为填补空隙，会添加空字符（`\0`）。因`size` 为引导配置文件的长度加上填充字节

Linux 内核会解码内存中 initrd 镜像的最后一部分以获取引导配置数据。由于这piggyback"（背负式）方法，只要引导加载程序传递正确的 initrd 文件大小，就无需更改或更新引导加载程序及内核镜像本身。万一引导加载程序传递了更大的大小，内核将无法找到引导配置数据

为此操作，Linux 内核tools/bootconfig 下提供了 `bootconfig` 命令，允许管理员应用或删除配置文
```

 # make -C tools/bootconfig

```
要将你的引导配置文件添加initrd 镜像，按如下方式运行 bootconfig
```

 # tools/bootconfig/bootconfig -a your-config /boot/initrd.img-X.Y.Z

```
```

 # tools/bootconfig/bootconfig -d /boot/initrd.img-X.Y.Z

```
然后在正常的内核命令行上添加 "bootconfig"，以告知内核initrd 文件末尾查找引导配置。或者，在编译内核时选中 `CONFIG_BOOT_CONFIG_FORCE` Kconfig 选项

### Embedding a Boot Config into Kernel


如果无法使用 initrd，你也可以通过 Kconfig 选项将引导配置文件内嵌到内核中。在这种情况下，你需要重新编译内
```

 CONFIG_BOOT_CONFIG_EMBED=y
 CONFIG_BOOT_CONFIG_EMBED_FILE="/PATH/TO/BOOTCONFIG/FILE"

```
`CONFIG_BOOT_CONFIG_EMBED_FILE` 需要一个指向引导配置文件的绝对路径，或相对于源代码对象树的相对路径。内核会将其作为默认引导配置内嵌

与将引导配置附加initrd 时一样，需要在内核命令行上使用 `bootconfig` 选项来启用内嵌的引导配置，或者也可以在编译内核时选中 `CONFIG_BOOT_CONFIG_FORCE` Kconfig 选项

请注意，即使设置了该选项，你也可以用附加initrd 的另一个引导配置来覆盖内嵌的引导配置

## Kernel parameters via Boot Config


除了内核命令行之外，引导配置还可用于传递内核参数。位`kernel` 键下的所有键值对将直接传递给内核命令行。此外，位于 `init` 键下的键值对将通过命令行传递给 init 进程。这些参数与用户给定的内核命令行字符串按下述顺序拼接，因此命令行参数可以覆盖引导配置参数（这取决于各子系统如何处理参
```

 [bootconfig params][cmdline params] -- [bootconfig init params][cmdline init params]

```
```

 kernel {
   root = 01234567-89ab-cdef-0123-456789abcd
 }
 init {
  splash
 }

```
```

 root="01234567-89ab-cdef-0123-456789abcd" -- splash

```
```

 ro bootconfig -- quiet

```
```

 root="01234567-89ab-cdef-0123-456789abcd" ro bootconfig -- splash quiet


```
## Config File Limitation


目前最大配置大小为 32KB，且总的键单词数（而非键值条目数）必须少1024 个节点。注意：这里指的是节点数而非条目数，一个条目至少要消2 个节点（一个键单词和一个值）。因此理论上最多可512 个键值对。如果键平均包含 3 个单词，则可包含 256 个键值对。在大多数情况下，配置项数量会少100 条且小于 8KB，因此已经足够。如果节点数超过 1024，即使文件大小小32KB，解析器也会返回错误。（注意，此最大大小不包含填充用的空字符。）无论如何，由bootconfig 命令在将引导配置追加initrd 镜像时会进行校验，用户可以在引导前就注意到这一点


## Bootconfig APIs


用户可以查询或遍历键值对，也可以通过查找根（前缀）键节点来找到该节点下的键值

如果拥有一个键字符串，你可以使xbc_find_value() 通过该键直接查询值。如果想了解引导配置中存在哪些键，可以使xbc_for_each_key_value() 来遍历键值对。注意，访问（数组值）时需要使xbc_array_for_each_value()
```

 vnode = NULL;
 xbc_find_value("key.word", &vnode);
 if (vnode && xbc_node_is_array(vnode))
    xbc_array_for_each_value(vnode, value) {
      printk("%s ", value);
    }

```
如果想聚焦于带有前缀字符串的键，可以使用 xbc_find_node() 通过该前缀字符串查找节点，并使xbc_node_for_each_key_value() 遍历该前缀节点下的键

但最典型的用法是获取前缀下的具名
```

 root = xbc_find_node("key.prefix");
 value = xbc_node_find_value(root, "option", &vnode);
 ...
 xbc_node_for_each_array_value(root, "array-option", value, anode) {
    ...
 }

```
这会访问 "key.prefix.option" 的值以"key.prefix.array-option" 的数组

不需要加锁，因为在初始化之后，配置变为只读。如果需要修改，必须复制全部数据与键


## Functions and structures



