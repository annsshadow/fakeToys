
## DebugFS


Copyright |copy| 2009 Jonathan Corbet <corbet@lwn.net>

Debugfs 作为一种简单的方式存在，让内核开发者能够向用户空间提供信息。与仅用于提供
进程相关信息的 /proc，或者有着严格"每个文件一个值"规则的 sysfs 不同，debugfs 根本
没有任何规则。开发者可以在其中放入他们想要的任何信息。debugfs 文件系统也不打算作为
向用户空间提供的稳定 ABI；理论上，对其导出的文件没有任何稳定性约束。然而，现实并非
总是如此简单 [^1^]_；即使是 debugfs 接口，最好也以"需要永久维护"的理念来设计。

```

    mount -t debugfs none /sys/kernel/debug

```
（或一条等价的 /etc/fstab 行）。
debugfs 根目录默认只有 root 用户可访问。要改变整棵树的访问权限，可以使用 "uid"、
"gid" 和 "mode" 挂载选项。

注意，debugfs API 仅以 GPL 方式导出给模块。

使用 debugfs 的代码应包含 <linux/debugfs.h>。然后，第一件事将是至少创建一个目录来
容纳一组
```

    struct dentry *debugfs_create_dir(const char *name, struct dentry *parent);

```
该调用如果成功，将在指定的父目录之下创建一个名为 name 的目录。如果 parent 为 NULL，
目录将被创建在 debugfs 根目录下。成功时，返回值是一个 struct dentry 指针，可用于
在该目录中创建文件（以及最后清理它）。返回 ERR_PTR(-ERROR) 表示出现了问题。如果返回
ERR_PTR(-ENODEV)，则表明内核是在未启用 debugfs 支持的情况下构建的，下面描述的函数
都不会工作。

```

    struct dentry *debugfs_create_file(const char *name, umode_t mode,
				       struct dentry *parent, void *data,
				       const struct file_operations *fops);

```
这里，name 是要创建的文件的名称，mode 描述文件应具有的访问权限，parent 指明持有该
文件的目录，data 将被存储在结果 inode 结构的 i_private 字段中，而 fops 是一组实现
文件行为的文件操作。至少应提供 read() 和/或 write() 操作；其他操作可按需要加入。同样，
返回值是所创建文件的 dentry 指针，出错时为 ERR_PTR(-ERROR)，或者若缺少 debugfs 支持
则为 ERR_PTR(-ENODEV)。

要创建一个具有初始大小的文件，可以使用以下函数
```

    void debugfs_create_file_size(const char *name, umode_t mode,
				  struct dentry *parent, void *data,
				  const struct file_operations *fops,
				  loff_t file_size);

```
file_size 是文件的初始大小。其余参数与函数 debugfs_create_file 相同。

在许多情况下，创建一组文件操作实际上并无必要；debugfs 代码为简单场景提供了若干
辅助函数。包含单个整数值的文件可以用
```

    void debugfs_create_u8(const char *name, umode_t mode,
			   struct dentry *parent, u8 *value);
    void debugfs_create_u16(const char *name, umode_t mode,
			    struct dentry *parent, u16 *value);
    void debugfs_create_u32(const char *name, umode_t mode,
			    struct dentry *parent, u32 *value);
    void debugfs_create_u64(const char *name, umode_t mode,
			    struct dentry *parent, u64 *value);

```
这些文件支持读写给定的值；如果某个特定文件不应被写入，只需相应地设置 mode 位即可。
这些文件中的值以十进制表示；如果十六进制更合适，则
```

    void debugfs_create_x8(const char *name, umode_t mode,
			   struct dentry *parent, u8 *value);
    void debugfs_create_x16(const char *name, umode_t mode,
			    struct dentry *parent, u16 *value);
    void debugfs_create_x32(const char *name, umode_t mode,
			    struct dentry *parent, u32 *value);
    void debugfs_create_x64(const char *name, umode_t mode,
			    struct dentry *parent, u64 *value);

```
只要开发者知道要导出的数值大小，这些函数就很有用。不过，某些类型在不同的体系结构上
可能具有不同的位宽，这使得情况稍微复杂了一些。还有
```

    void debugfs_create_size_t(const char *name, umode_t mode,
			       struct dentry *parent, size_t *value);

```
正如所料，该函数会创建一个 debugfs 文件来表示一个 size_t 类型的变量。

类似地，对于 unsigned long 类型的变量也有辅助函数，以十进制表示
```

    struct dentry *debugfs_create_ulong(const char *name, umode_t mode,
					struct dentry *parent,
					unsigned long *value);
    void debugfs_create_xul(const char *name, umode_t mode,
			    struct dentry *parent, unsigned long *value);

```
```

    void debugfs_create_bool(const char *name, umode_t mode,
                             struct dentry *parent, bool *value);

```
对结果文件的一次读取将产生 Y（对于非零值）或 N，后跟一个换行符。如果对其写入，它将
接受大写或小写的值，或者 1 或 0。任何其他输入都会被静默忽略。

```

    void debugfs_create_atomic_t(const char *name, umode_t mode,
				 struct dentry *parent, atomic_t *value)

```
对该文件的读取将获得 atomic_t 值，对该文件的写入将设置 atomic_t 值。

另一个选项是导出一个任意二进制数据块，使用
```

    struct debugfs_blob_wrapper {
	void *data;
	unsigned long size;
    };

    struct dentry *debugfs_create_blob(const char *name, umode_t mode,
				       struct dentry *parent,
				       struct debugfs_blob_wrapper *blob);

```
对该文件的读取将返回 debugfs_blob_wrapper 结构所指向的数据。一些驱动使用 "blob" 作为
返回多行（静态）格式化文本输出的简单方式。该函数可用于导出二进制信息，但主线中似乎
没有这样做的代码。注意，所有用 debugfs_create_blob() 创建的文件都是只读的。

如果你想转储一块寄存器（这在开发过程中经常发生，尽管很少有这样的代码进入主线），
debugfs 提供两个函数：一个用于创建仅含寄存器的文件，另一个用于在另一个顺序文件的
中间插入一个寄存器块
```

    struct debugfs_reg32 {
	char *name;
	unsigned long offset;
    };

    struct debugfs_regset32 {
	const struct debugfs_reg32 *regs;
	int nregs;
	void __iomem *base;
	struct device *dev;     /* Optional device for Runtime PM */
    };

    debugfs_create_regset32(const char *name, umode_t mode,
			    struct dentry *parent,
			    struct debugfs_regset32 *regset);

    void debugfs_print_regs32(struct seq_file *s, const struct debugfs_reg32 *regs,
			 int nregs, void __iomem *base, char *prefix);

```
"base" 参数可以为 0，但你可能想用 __stringify 来构建 reg32 数组，并且许多寄存器名
（宏）实际上是相对于寄存器块基址的字节偏移。

```

    struct debugfs_u32_array {
	u32 *array;
	u32 n_elements;
    };

    void debugfs_create_u32_array(const char *name, umode_t mode,
			struct dentry *parent,
			struct debugfs_u32_array *array);

```
"array" 参数封装了指向数组数据的指针及其元素个数。注意：一旦数组被创建，其大小就
无法更改。

```

   void debugfs_create_devm_seqfile(struct device *dev,
				const char *name,
				struct dentry *parent,
				int (*read_fn)(struct seq_file *s,
					void *data));

```
"dev" 参数是与此 debugfs 文件相关的设备，"read_fn" 是一个函数指针，将被调用以打印
seq_file 的内容。

```

    struct dentry *debugfs_change_name(struct dentry *dentry,
					  const char *fmt, ...);

    struct dentry *debugfs_create_symlink(const char *name,
                                          struct dentry *parent,
				      	  const char *target);

```
对 debugfs_change_name() 的调用会为一个已存在的 debugfs 文件赋予一个新名称，且始终
在同一目录中。new_name 在调用前必须不存在；成功时返回值为 0，失败时返回 -E...。符号
链接可以用 debugfs_create_symlink() 创建。

所有 debugfs 用户都必须考虑一个要点：在 debugfs 中创建的任何目录都不会被自动清理。
如果一个模块在卸载时没有显式移除 debugfs 条目，结果将是大量陈旧指针，以及无穷无尽的、
极其不友好的行为。因此，所有 debugfs 用户——至少是那些可以被构建为模块的——必须准备好
移除它们在那里创建的所有文件和目录。一个文件
```

    void debugfs_remove(struct dentry *dentry);

```
dentry 值可以为 NULL 或错误值，此时不会移除任何内容。注意，该函数会递归移除其下方的
所有文件和目录。以前，debugfs_remove_recursive() 用于执行该任务，但现在该函数只是
debugfs_remove() 的一个别名。debugfs_remove_recursive() 应被视为已废弃。
