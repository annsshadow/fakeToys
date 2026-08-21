
## Sysfs 标记


（几乎逐字摘自 Eric Biederman netns 标记补丁提交信息
问题所在。网络设备在 sysfs 中显示，当网络命名空间处于活动状态时，相同名称的多个
设备可能出现在同一个目录中，这很麻烦！

为了避免该问题，并允许网络命名空间中的现有应用程序看到当前在 sysfs 中呈现的同一
接口，sysfs 现在支持标记目录
通过使用网络命名空间指针作为标记来分sysfs 目录项，我们确保目录中不会发生冲突，
并且应用程序只能看到有限的网络设备集合
每个 sysfs 目录项都可以通过`kernfs_node` `void *ns` 成员用命名空间进行标记如果一个目录项被标记，那么 `kernfs_node->flags` 中将存在一个介KOBJ_NS_TYPE_NONE
KOBJ_NS_TYPES 之间的标志，ns 将指向它所属的命名空间
每个 sysfs 超级块的 kernfs_super_info 包含一个数`void *ns[KOBJ_NS_TYPES]`当处于标记命名空kobj_nstype 中的一个任务首次挂sysfs 时，会创建一个新的超级块它将通过将其 `s_fs_info->ns[kobj_nstype]` 设置为新的命名空间而与其他 sysfs 挂载区分开请注意，通过绑定挂载和挂载传播，一个任务可以轻易查看其他命名空间的 sysfs 挂载内容因此，当一个命名空间退出时，它会调kobj_ns_exit() 来使任何指向它的 kernfs_node->ns
指针失效
该接口的使用者：

- `kobj_ns_type` 枚举中定义一个类型- 调用 kobj_ns_type_register()，并传入`kobj_ns_type_operations`，其中包
  - current_ns()，返回当前任务的命名空间
  - netlink_ns()，返回某个套接字的命名空  - initial_ns()，返回初始命名空
- 当某个标记不再有效时，调kobj_ns_exit()
