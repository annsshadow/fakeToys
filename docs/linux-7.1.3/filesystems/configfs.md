## Configfs —由用户空间驱动的內核对象配置


Joel Becker <joel.becker@oracle.com>

更新005 3 31 

Copyright (c) 2005 Oracle Corporation,
	Joel Becker <joel.becker@oracle.com>


## 什么是 configfs


configfs 是一个基于内存的文件系统，提供与 sysfs 功能相反的服务。sysfs
是基于文件系统的内核对象视图，configfs 是基于文件系统的内核对象（即
config_items）管理器

使用 sysfs 时，对象在内核中创建（例如，在发现设备时），并向 sysfs 注册
随后它的属性便会出现在 sysfs 中，允许用户空间通过 readdir(3)/read(2) 读取
这些属性。它可能允许通过 write(2) 修改某些属性。要点在于，对象在内核中
创建和销毁，内核控制着 sysfs 表示的生存周期，sysfs 仅仅是这一切的一
窗口

一configfs config_item 通过显式的用户空间操mkdir(2) 创建，并通过
rmdir(2) 销毁。属性在 mkdir(2) 时即出现，并可通过 read(2) write(2) 读取
或修改。与 sysfs 一样，readdir(3) 查询项和/或属性的列表。symlink(2) 可用
将项分组在一起。与 sysfs 不同的是，表示的生存周期完全由用户空间驱动。支
这些项的內核模块必须对此作出响应

sysfs configfs 可以并且应该共存于同一系统中。二者互为补充，并非替代关系

## 使用 configfs


configfs 可以编译为模块或编入内核。你可以通过以下方式访问它：

```
	mount -t configfs none /config
```

除非同时加载了客户端模块，否configfs 树将是空的。这些模块作为子系统
configfs 注册了它们的项类型。一旦客户端子系统被加载，它就会作为 /config
下的一个（或多个）子目录出现。与 sysfs 一样，无论是否挂载/config
configfs 树始终存在

一项通过 mkdir(2) 创建。该项的属性也会在此时出现。readdir(3) 可以确定有哪
属性，read(2) 可以查询它们的默认值，write(2) 可以存储新值。不要在一个属
文件中混入多个属性

configfs 有两种类型的属性：

- 普通属性，sysfs 属性类似，是小ASCII 文本文件，最大大小为单页
  （PAGE_SIZE，在 i386 上为 4096）。最好每个文件只使用一个值，并且适用
  sysfs 相同的注意事项。configfs 期望 write(2) 一次性存储整个缓冲区。在
  普configfs 属性写入时，用户空间进程应先读取整个文件，修改其希望更改的
  部分，然后将整个缓冲区写回

- 二进制属性，sysfs 二进制属性有些类似，但语义上有一些细微变化。PAGE_SIZE
  的限制不适用，但整个二进制项必须能放入单个内vmalloc 缓冲区中。来自用
  空间write(2) 调用会被缓冲，属性的 write_bin_attribute 方法将在最后一
  关闭时被调用，因此用户空间必须检close(2) 的返回码，以确认操作已成
  完成。为了避免恶意用户使内核 OOM，每个二进制属性都有一个最大缓冲区值

当某项需要被销毁时，使rmdir(2) 将其移除。如果有任何其他项通过 symlink(2)
链接到它，则该项不能被销毁。链接可以通过 unlink(2) 移除

## 配置 FakeNBD：一个示


设想有一个网络块设备（NBD）驱动，允许你访问远程块设备。称之为 FakeNBD
FakeNBD 使用 configfs 进行配置。显然，会有一个很好的程序供系统管理员用来
配置 FakeNBD，但该程序总得以某种方式将配置告知驱动。这就是 configfs 的用
之地

FakeNBD 驱动被加载时，它会向 configfs 注册自己

```
	# ls /config
	fakenbd
```

一fakenbd 连接可以通过 mkdir(2) 创建。名称是任意的，但工具可能会利用
该名称。也许：

```
	# mkdir /config/fakenbd/disk1
	# ls /config/fakenbd/disk1
	target device rw
```

target 属性包FakeNBD 将要连接的服务器 IP 地址。device 属性是服务器上
设备。可以预见，rw 属性决定该连接是否

```
	# echo 10.0.0.1 > /config/fakenbd/disk1/target
	# echo /dev/sda1 > /config/fakenbd/disk1/device
	# echo 1 > /config/fakenbd/disk1/rw
```

就这样。仅此而已。现在设备已经配置好了，而且还是通过 shell 完成的

## configfs 编码


configfs 中的每个对象都是一config_item。一config_item 反映了子系统
中的一个对象。它具有与对象上值相对应的属性。configfs 处理该对象及其属性的
文件系统表示，使得子系统只需关注基本show/store 交互

项在 config_group 内部创建和销毁。一个组是一组共享相同属性和操作的项的集合
项通过 mkdir(2) 创建、通过 rmdir(2) 移除，但这由 configfs 处理。该组有一
执行这些操作的方法

子系统是客户端模块的顶层。在初始化期间，客户端模块向 configfs 注册子系统，
该子系统作为 configfs 文件系统顶层的目录出现。子系统同时也是一config_group
并且可以完成 config_group 能做的所有事情

## struct config_item


```
	struct config_item {
		char                    *ci_name;
		char                    ci_namebuf[UOBJ_NAME_LEN];
		struct kref             ci_kref;
		struct list_head        ci_entry;
		struct config_item      *ci_parent;
		struct config_group     *ci_group;
		struct config_item_type *ci_type;
		struct dentry           *ci_dentry;
	};

	void config_item_init(struct config_item *);
	void config_item_init_type_name(struct config_item *,
					const char *name,
					struct config_item_type *type);
	struct config_item *config_item_get(struct config_item *);
	void config_item_put(struct config_item *);
```

通常，struct config_item 嵌入在一个容器结构中，该结构实际代表了子系统正在
做的事情。该结构config_item 部分就是对象configfs 交互的方式

无论是静态定义在源文件中，还是由config_group 创建，一config_item 都必
调用其中一_init() 函数。这会初始化引用计数并设置相应的字段

所有使config_item 的地方都应该通过 config_item_get() 持有它的一个引用，
并在用完后通过 config_item_put() 释放引用

单凭自身，一config_item 除了出现configfs 中之外做不了太多事情。通常子系
希望该项显示或存储属性等。为此，它需要一个类型

## struct config_item_type


```
	struct configfs_item_operations {
		void (*release)(struct config_item *);
		int (*allow_link)(struct config_item *src,
				  struct config_item *target);
		void (*drop_link)(struct config_item *src,
				 struct config_item *target);
	};

	struct config_item_type {
		struct module                           *ct_owner;
		struct configfs_item_operations         *ct_item_ops;
		struct configfs_group_operations        *ct_group_ops;
		struct configfs_attribute               **ct_attrs;
		struct configfs_bin_attribute		**ct_bin_attrs;
	};
```

config_item_type 最基本的功能是定义可以config_item 上执行哪些操作。所
动态分配的项的都需要提ct_item_ops->release() 方法。当 config_item 的引用计
达到零时会调用该方法

## struct configfs_attribute


```
	struct configfs_attribute {
		char                    *ca_name;
		struct module           *ca_owner;
		umode_t                  ca_mode;
		ssize_t (*show)(struct config_item *, char *);
		ssize_t (*store)(struct config_item *, const char *, size_t);
	};
```

当一config_item 希望某个属性作为文件出现在configfs 目录中时，它必须
定义一个描述它configfs_attribute。然后它将属性添加到NULL 结尾的数
config_item_type->ct_attrs 中。当该项出现configfs 中时，属性文件将
configfs_attribute->ca_name 作为文件名出现。configfs_attribute->ca_mode 指定
文件权限

如果一个属性是可读的并且提供了 ->show 方法，那么每当用户空间对该属性请
read(2) 时都会调用该方法。如果一个属性是可写的并且提供了 ->store 方法，那
每当用户空间对该属性请write(2) 时都会调用该方法

## struct configfs_bin_attribute


```
	struct configfs_bin_attribute {
		struct configfs_attribute	cb_attr;
		void				*cb_private;
		size_t				cb_max_size;
	};
```

当需要使用二进制 blob 作为项在configfs 目录中文件的内容时，就会用到二进
属性。为此，将二进制属性添加到NULL 结尾的数config_item_type->ct_bin_attrs
中，当该项出现在 configfs 中时，属性文件将configfs_bin_attribute->cb_attr.ca_name
作为文件名出现。configfs_bin_attribute->cb_attr.ca_mode 指定了文件权限。cb_private
成员供驱动使用，cb_max_size 成员指定了要使用vmalloc 缓冲区的最大大小

如果二进制属性是可读的，并且 config_item 提供ct_item_ops->read_bin_attribute()
方法，那么每当用户空间对该属性请read(2) 时都会调用该方法。write(2) 的情
相反。读/写是缓冲的，因此只会发生单次写；属性无需关心这一点

## struct config_group


一config_item 不能孤立存在。创建它的唯一方式是在 config_group 上执
mkdir(2)。这会触发创建一个：

```
	struct config_group {
		struct config_item		cg_item;
		struct list_head		cg_children;
		struct configfs_subsystem 	*cg_subsys;
		struct list_head		default_groups;
		struct list_head		group_entry;
	};

	void config_group_init(struct config_group *group);
	void config_group_init_type_name(struct config_group *group,
					 const char *name,
					 struct config_item_type *type);
```

config_group 结构包含一config_item。正确配置该项意味着该组本身可以像一
项一样工作。不过，它能做的更多：它可以创建子项或子组。这是通过在该组的
group 操作中指定的方法来完成的

```
	struct configfs_group_operations {
		struct config_item *(*make_item)(struct config_group *group,
						 const char *name);
		struct config_group *(*make_group)(struct config_group *group,
						   const char *name);
		void (*disconnect_notify)(struct config_group *group,
					  struct config_item *item);
		void (*drop_item)(struct config_group *group,
				  struct config_item *item);
	};
```

一个组通过提供 ct_group_ops->make_item() 方法来创建子项。如果提供了该方法，
它会在该组目录中mkdir(2) 时被调用。子系统分配一个新config_item（或更可
是其容器结构），初始化它，并将其返回configfs。configfs 随后会填充文件系统树
以反映这个新项

如果子系统希望子项本身是一个组，则子系统提ct_group_ops->make_group()。其
一切都表现相同，在组上使用组的 _init() 函数

最后，当用户空间对该项或组调用 rmdir(2) 时，会调ct_group_ops->drop_item()
由于 config_group 也是一config_item，因此不需要单独的 drop_group() 方法。子系统
必须对项分配时初始化的引用执config_item_put()。如果子系统无事可做，它可以省略
ct_group_ops->drop_item() 方法，configfs 将代表子系统对该项调config_item_put()

重要
   drop_item() void 类型，因此无法失败。当调用 rmdir(2) 时，configfs 将把该项
   从文件系统树中移除（前提是它没有需要保持忙碌的子项）。子系统负责对此作出
   响应。如果子系统在其他线程中持有对该项的引用，内存是安全的。该项实际从子系统的
   使用中消失可能需要一些时间。但它已经从 configfs 中消失了

当调drop_item() 时，项的链接关系已经被拆除。它不再持有其父项的引用，也
不在项的层次结构中占有一席之地。如果客户端需要在拆除发生之前做一些清理工作，
子系统可以实ct_group_ops->disconnect_notify() 方法。该方法configfs 已将
从文件系统视图中移除之后、但在该项从其父组中移除之前被调用。与 drop_item() 一样，
disconnect_notify() void 类型且不能失败。客户端子系统不应在此处释放任何引用
因为它们仍然必须drop_item() 中执行

只要 config_group 仍然拥有子项，就不能被移除。这是在 configfs rmdir(2) 代码
实现的>drop_item() 不会被调用，因为项尚未被丢弃。rmdir(2) 会失败，因为目录
非空

## struct configfs_subsystem


一个子系统必须注册自己，通常module_init 时。这是通过

```
	struct configfs_subsystem {
		struct config_group	su_group;
		struct mutex		su_mutex;
	};

	int configfs_register_subsystem(struct configfs_subsystem *subsys);
	void configfs_unregister_subsystem(struct configfs_subsystem *subsys);
```

一个子系统由一个顶config_group 和一mutex 组成。该组是创建config_item
的地方。对于一个子系统，这个组通常是静态定义的。在调用
configfs_register_subsystem() 之前，子系统必须通过通常的组 _init() 函数初始化该
组，并且还必须初始化 mutex

当注册调用返回时，子系统就处于活动状态，并且可以通过 configfs 看到。此时，可以
调用 mkdir(2)，子系统必须为此做好准备

## 一个示


这些基本概念的最佳示例是 samples/configfs/configfs_sample.c 中的 simple_children
子系组和 simple_child 项。它展示了一个显示和存储属性的平凡对象，以及一
创建和销毁这些子项的简单组

## 层次导航与子系统 Mutex


configfs 还提供了一个额外的好处。config_group config_item 由于出现在文件系统中
而排列成层次结构。子系统绝不触碰文件系统的部分，但子系统可能对这个层次感兴趣
因此，该层次通过 config_group->cg_children config_item->ci_parent 结构成员
镜像出来

子系统可以遍cg_children 列表ci_parent 指针来查看由子系统创建的树。这可能
configfs 对该层次的管理产生竞争，因此 configfs 使用子系mutex 来保护修改。每
子系统想要遍历该层次时，它必须在子系mutex 的保护下进行

newly allocated item 尚未链接进该层次时，子系统将被阻止获mutex。类似地
dropping item 尚未被解除链接时，它也无法获mutex。这意味着只要项在 configfs
中，ci_parent 指针就永远不会是 NULL，并且项仅在同一时间段内存在于其父项
cg_children 列表中。这使得子系统在持有 mutex 时可以信ci_parent cg_children

## 通过 symlink(2) 进行项聚


configfs 通过 group->item 的父/子关系提供一个简单的组。然而，更大的环境常常需
在父/子连接之外进行聚合。这是通过 symlink(2) 实现的

一config_item 可以提供 ct_item_ops->allow_link() ct_item_ops->drop_link() 方法
如果 ->allow_link() 方法存在，就可以以该 config_item 作为链接源来调用 symlink(2)
这些链接只允许在 configfs config_item 之间建立。任何在 configfs 文件系统之外
symlink(2) 尝试都将被拒绝

当调symlink(2) 时，config_item ->allow_link() 方法会被调用，传入它自己
目标项。如果源项允许链接到目标项，则返0。如果源项只希望链接到某种特定类型的
对象（例如它自己子系统内的对象），它可能希望拒绝一个链接

当对符号链接调用 unlink(2) 时，源项会通过 ->drop_link() 方法得到通知。与 ->drop_item()
方法一样，这是一void 函数，不能返回失败。子系统负责响应这一变化

一config_item 在链接到任何其他项时不能被移除，在有其他项链接到它时也不能被移除
configfs 中不允许悬空符号链接

## 自动创建的子


一个新config_group 可能希望拥有两种类型的子 config_item。虽然这可以通过
->make_item() 中的魔法名称来编码，但有一种方法让用户空间看到这种差异会更加明确

configfs 不是让一个组中的某些项表现得与其他项不同，而是提供了一种方法，即在父组
创建时自动在其内部创建一个或多个子组。因此，mkdir("parent") 会生"parent"
"parent/subgroup1"，一直到 "parent/subgroupN"。类型为 1 的项现在可以
"parent/subgroup1" 中创建，类型N 的项可以"parent/subgroupN" 中创建

这些自动创建的子组（或称默认组）并不排除父组的其他子项。如ct_group_ops->make_group()
存在，可以直接在父组上创建其他子组

一configfs 子系统通过使用 configfs_add_default_group() 函数将默认组添加到父
config_group 结构来指定它们。每个被添加的组与父组在同一时间被填充进 configfs 树中
类似地，它们与父组在同一时间被移除。不提供额外的通知。当 ->drop_item() 方法调用
通知子系统父组即将消失时，这也意味着与该父组关联的每一个默认子组

因此，默认组不能通过 rmdir(2) 直接移除。在父组上执rmdir(2) 检查子项时，它
也不被考虑

## 渚濊禆瀛愮郴缁。


有时其他驱动依赖于特定的 configfs 项。例如，ocfs2 挂载依赖于一个心跳区域项。如果该
区域项通过 rmdir(2) 被移除，ocfs2 挂载就必BUG 或进入只读。这并不理想

configfs 提供了两个额外的 API 调用：configfs_depend_item() configfs_undepend_item()
一个客户端驱动可以在一个已存在的项上调configfs_depend_item() 来告configfs 
依赖于该项。configfs 随后会针对该项的 rmdir(2) 返回 -EBUSY。当该项不再被依赖时
客户端驱动对其调configfs_undepend_item()

这些 API 不能configfs 的任何回调之下调用，因为它们会发生冲突。它们可能阻塞和分配
客户端驱动大概不应该凭自己的意愿调用它们，而应该提供一个供外部子系统调用的 API

这是如何工作的？设想 ocfs2 挂载过程。当它挂载时，它请求一个心跳区域项。这是通过调用
心跳代码完成的。在心跳代码内部，会查找该区域项。在这里，心跳代码调
configfs_depend_item()。如果成功，那么心跳就知道该区域可以安全地交ocfs2。如
失败，说明它反正正在被拆除，心跳可以优雅地上传一个错误
