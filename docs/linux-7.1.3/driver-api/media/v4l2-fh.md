
### V4L2 文件句柄


struct v4l2_fh 提供了一种简便的方式来保存 V4L2 框架所使用的、与文件句柄相关的特定数据。
在所有驱动中都必须使用它。

struct v4l2_fh 在驱动的 `open()` 文件操作处理函数中分配。它通常内嵌于一个更大的、
驱动特定的结构中。`v4l2_fh` 必须通过调用 `v4l2_fh_init` 进行初始化，
并通过 `v4l2_fh_add` 添加到 video 设备。这通过将 `file->private_data`
设为指向 `v4l2_fh` 的指针，从而将 `v4l2_fh` 与 `file` 关联起来。

类似地，struct v4l2_fh 在驱动的 `release()` 文件操作处理函数中释放。在释放之前，
必须先用 `v4l2_fh_del` 从 video 设备移除，并用 `v4l2_fh_exit` 清理。

驱动不得直接访问 `file->private_data`。它们可以通过调用 `file_to_v4l2_fh`
获取与 `file` 关联的 `v4l2_fh`。驱动可以使用 container_of 宏提取它们自己的文件句柄结构。

示例：


	struct my_fh {
		int blah;
		struct v4l2_fh fh;
	};

	...

	int my_open(struct file *file)
	{
		struct my_fh *my_fh;
		struct video_device *vfd;
		int ret;

		...

		my_fh = kzalloc(sizeof(*my_fh), GFP_KERNEL);

		...

		v4l2_fh_init(&my_fh->fh, vfd);

		...

		v4l2_fh_add(&my_fh->fh, file);
		return 0;
	}

	int my_release(struct file *file)
	{
		struct v4l2_fh *fh = file_to_v4l2_fh(file);
		struct my_fh *my_fh = container_of(fh, struct my_fh, fh);

		...
		v4l2_fh_del(&my_fh->fh, file);
		v4l2_fh_exit(&my_fh->fh);
		kfree(my_fh);
		return 0;
	}

下面简要描述所使用的 `v4l2_fh` 函数：

`v4l2_fh_init <v4l2_fh_init>`
（`fh <v4l2_fh>`, `vdev <video_device>`）

- 初始化文件句柄。这**必须**在驱动的 `v4l2_file_operations`->open() 处理函数中执行。

`v4l2_fh_add <v4l2_fh_add>`
（`fh <v4l2_fh>`, struct file \*filp）

- 将一个 `v4l2_fh` 添加到 `video_device` 的文件句柄列表。
  必须在文件句柄完全初始化后调用。

`v4l2_fh_del <v4l2_fh_del>`
（`fh <v4l2_fh>`, struct file \*filp）

- 解除文件句柄与 `video_device` 的关联。现在可以调用文件句柄的退出函数。

`v4l2_fh_exit <v4l2_fh_exit>`
（`fh <v4l2_fh>`）

- 反初始化文件句柄。反初始化之后，`v4l2_fh` 的内存可被释放。

`file_to_v4l2_fh <file_to_v4l2_fh>`
（struct file \*filp）

- 获取与 `file` 关联的 `v4l2_fh` 实例。

如果 struct v4l2_fh 未被内嵌，则可以使用以下辅助函数：

`v4l2_fh_open <v4l2_fh_open>`
（struct file \*filp）

- 该函数分配一个 struct v4l2_fh，初始化它，并将其添加到与该文件结构关联的
  struct video_device。

`v4l2_fh_release <v4l2_fh_release>`
（struct file \*filp）

- 该函数将其从与文件结构关联的 struct video_device 中删除，反初始化 `v4l2_fh`
  并释放它。

这两个函数可以插入到 v4l2_file_operation 的 `open()` 与 `release()` 操作中。

若干驱动需要在第一个文件句柄被打开以及最后一个文件句柄被关闭时执行某些操作。为此添加了
两个辅助函数，用于检查 `v4l2_fh` 结构是否是关联设备节点唯一打开的文件句柄：

`v4l2_fh_is_singular <v4l2_fh_is_singular>`
（`fh <v4l2_fh>`）

- 如果文件句柄是唯一的打开文件句柄则返回 1，否则返回 0。

`v4l2_fh_is_singular_file <v4l2_fh_is_singular_file>`
（struct file \*filp）

- 同上，但它以 filp->private_data 调用 v4l2_fh_is_singular。

##### V4L2 fh 函数与数据结构
