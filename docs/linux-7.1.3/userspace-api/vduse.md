## VDUSE - “vDPA Device in Userspace”（用户空间中的 vDPA 设备）


vDPA（virtio 数据路径加速）设备是一种使用符合 virtio 规范的数据路径、并配合厂商特定控制路径的设备。vDPA 设备可以物理上位于硬件上，也可以由软件模拟。VDUSE 是一个框架，使得在用户空间中实现软件模拟的 vDPA 设备成为可能。为了使设备模拟更安全，被模拟的 vDPA 设备的控制路径在内核中处理，只有数据路径在用户空间实现。

注意，目前 VDUSE 框架仅支持 virtio 块设备，这可以在实现数据路径的用户空间进程由非特权用户运行时降低安全风险。对其他设备类型的支持可以在相应设备驱动的安全问题在未来被澄清或修复后添加。

### 创建/销毁 VDUSE 设备


VDUSE 设备按如下方式创建：

1. 在 /dev/vduse/control 上用 ioctl(VDUSE_CREATE_DEV) 创建一个新的 VDUSE 实例。

2. 在 /dev/vduse/$NAME 上用 ioctl(VDUSE_VQ_SETUP) 设置每个 virtqueue。

3. 开始处理来自 /dev/vduse/$NAME 的 VDUSE 消息。前几条消息会在将 VDUSE 实例挂接到 vDPA 总线时到达。

4. 发送 VDPA_CMD_DEV_NEW netlink 消息，将 VDUSE 实例挂接到 vDPA 总线。

VDUSE 设备按如下方式销毁：

1. 发送 VDPA_CMD_DEV_DEL netlink 消息，将 VDUSE 实例从 vDPA 总线分离。

2. 关闭指向 /dev/vduse/$NAME 的文件描述符。

3. 在 /dev/vduse/control 上用 ioctl(VDUSE_DESTROY_DEV) 销毁 VDUSE 实例。

netlink 消息可以通过 iproute2 中的 vdpa 工具发送，也可以使用以下示例代码：


	static int netlink_add_vduse(const char *name, enum vdpa_command cmd)
	{
		struct nl_sock *nlsock;
		struct nl_msg *msg;
		int famid;

		nlsock = nl_socket_alloc();
		if (!nlsock)
			return -ENOMEM;

		if (genl_connect(nlsock))
			goto free_sock;

		famid = genl_ctrl_resolve(nlsock, VDPA_GENL_NAME);
		if (famid < 0)
			goto close_sock;

		msg = nlmsg_alloc();
		if (!msg)
			goto close_sock;

		if (!genlmsg_put(msg, NL_AUTO_PORT, NL_AUTO_SEQ, famid, 0, 0, cmd, 0))
			goto nla_put_failure;

		NLA_PUT_STRING(msg, VDPA_ATTR_DEV_NAME, name);
		if (cmd == VDPA_CMD_DEV_NEW)
			NLA_PUT_STRING(msg, VDPA_ATTR_MGMTDEV_DEV_NAME, "vduse");

		if (nl_send_sync(nlsock, msg))
			goto close_sock;

		nl_close(nlsock);
		nl_socket_free(nlsock);

		return 0;
	nla_put_failure:
		nlmsg_free(msg);
	close_sock:
		nl_close(nlsock);
	free_sock:
		nl_socket_free(nlsock);
		return -1;
	}

### VDUSE 如何工作


如上所述，VDUSE 设备由在 /dev/vduse/control 上的 ioctl(VDUSE_CREATE_DEV) 创建。通过该 ioctl，用户空间可以指定一些基本配置，例如设备名称（唯一标识一个 VDUSE 设备）、virtio 特性、virtio 配置空间、virtqueue 的数量等，用于这个被模拟的设备。然后会向用户空间导出一个字符设备接口（/dev/vduse/$NAME）用于设备模拟。用户空间可以使用 /dev/vduse/$NAME 上的 VDUSE_VQ_SETUP ioctl 向设备添加每个 virtqueue 的配置，例如 virtqueue 的最大大小。

初始化之后，VDUSE 设备可以通过 VDPA_CMD_DEV_NEW netlink 消息挂接到 vDPA 总线。用户空间需要在 /dev/vduse/$NAME 上 read()/write()，以从 VDUSE 内核模块接收/回复一些控制消息，如下所示：


	static int vduse_message_handler(int dev_fd)
	{
		int len;
		struct vduse_dev_request req;
		struct vduse_dev_response resp;

		len = read(dev_fd, &req, sizeof(req));
		if (len != sizeof(req))
			return -1;

		resp.request_id = req.request_id;

		switch (req.type) {

		/** handle different types of messages **/

		}

		len = write(dev_fd, &resp, sizeof(resp));
		if (len != sizeof(resp))
			return -1;

		return 0;
	}

VDUSE 框架目前引入了三种类型的消息：

- VDUSE_GET_VQ_STATE：获取 virtqueue 的状态，用户空间应返回 split virtqueue 的 avail 索引，或 packed virtqueue 的设备/驱动环回绕计数以及 avail 和 used 索引。

- VDUSE_SET_STATUS：设置设备状态，用户空间应遵循 virtio 规范：https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.html 来处理此消息。例如，如果设备无法接受从 VDUSE_DEV_GET_FEATURES ioctl 获得的已协商 virtio 特性，则设置 FEATURES_OK 设备状态位失败。

- VDUSE_UPDATE_IOTLB：通知用户空间更新指定 IOVA 范围的内存映射，用户空间应首先移除旧映射，然后通过 VDUSE_IOTLB_GET_FD ioctl 建立新映射。

在通过 VDUSE_SET_STATUS 消息设置 DRIVER_OK 状态位之后，用户空间就可以开始数据面处理，如下所示：

1. 用 VDUSE_VQ_GET_INFO ioctl 获取指定 virtqueue 的信息，包括大小、描述符表/可用环/已用环的 IOVA、状态以及就绪状态。

2. 将上述 IOVA 传给 VDUSE_IOTLB_GET_FD ioctl，以便将这些 IOVA 区域映射到用户空间。一些示例代码如下：


	static int perm_to_prot(uint8_t perm)
	{
		int prot = 0;

		switch (perm) {
		case VDUSE_ACCESS_WO:
			prot |= PROT_WRITE;
			break;
		case VDUSE_ACCESS_RO:
			prot |= PROT_READ;
			break;
		case VDUSE_ACCESS_RW:
			prot |= PROT_READ | PROT_WRITE;
			break;
		}

		return prot;
	}

	static void **iova_to_va(int dev_fd, uint64_t iova, uint64_t **len)
	{
		int fd;
		void *addr;
		size_t size;
		struct vduse_iotlb_entry entry;

		entry.start = iova;
		entry.last = iova;

		/*
   - Find the first IOVA region that overlaps with the specified
   - range [start, last] and return the corresponding file descriptor.
		 */
		fd = ioctl(dev_fd, VDUSE_IOTLB_GET_FD, &entry);
		if (fd < 0)
			return NULL;

		size = entry.last - entry.start + 1;
		*len = entry.last - iova + 1;
		addr = mmap(0, size, perm_to_prot(entry.perm), MAP_SHARED,
			    fd, entry.offset);
		close(fd);
		if (addr == MAP_FAILED)
			return NULL;

		/*
   - Using some data structures such as linked list to store
   - the iotlb mapping. The munmap(2) should be called for the
   - cached mapping when the corresponding VDUSE_UPDATE_IOTLB
   - message is received or the device is reset.
		 */

		return addr + iova - entry.start;
	}

3. 用 VDUSE_VQ_SETUP_KICKFD ioctl 为指定 virtqueue 设置 kick eventfd。kick eventfd 由 VDUSE 内核模块用于通知用户空间消费可用环。这是可选的，因为用户空间也可以选择轮询可用环。

4. 监听 kick eventfd（可选）并消费可用环。描述符表中所描述的描述符所指向的缓冲区在访问之前也应通过 VDUSE_IOTLB_GET_FD ioctl 映射到用户空间。

5. 在已用环被填充之后，用 VDUSE_INJECT_VQ_IRQ ioctl 为特定 virtqueue 注入一个中断。

### 启用 ASID（API 版本 1）


VDUSE 从 API 版本 1 开始支持每地址空间标识符（ASID）。在通过 ioctl(VDUSE_CREATE_DEV) 创建新的 VDUSE 实例之前，在 `/dev/vduse/control` 上用 ioctl(VDUSE_SET_API_VERSION) 并设置 `VDUSE_API_VERSION_1` 来进行设置。

之后，你可以使用 ioctl(VDUSE_VQ_SETUP) 参数的 asid 成员来选择所查询 IOTLB 的地址空间。驱动可以通过使用 VDUSE_SET_VQ_GROUP_ASID VDUSE 消息类型更改任何 virtqueue 组的地址空间，如果可以更改，VDUSE 实例需要以 VDUSE_REQ_RESULT_OK 回复。

类似地，你可以使用 ioctl(VDUSE_IOTLB_GET_FD2) 获取描述特定 ASID 的 IOVA 区域的文件描述符。使用示例：


	static void *iova_to_va(int dev_fd, uint32_t asid, uint64_t iova,
	                        uint64_t *len)
	{
		int fd;
		void *addr;
		size_t size;
		struct vduse_iotlb_entry_v2 entry = { 0 };

		entry.v1.start = iova;
		entry.v1.last = iova;
		entry.asid = asid;

		fd = ioctl(dev_fd, VDUSE_IOTLB_GET_FD2, &entry);
		if (fd < 0)
			return NULL;

		size = entry.v1.last - entry.v1.start + 1;
		*len = entry.v1.last - iova + 1;
		addr = mmap(0, size, perm_to_prot(entry.v1.perm), MAP_SHARED,
			    fd, entry.v1.offset);
		close(fd);
		if (addr == MAP_FAILED)
			return NULL;

		/*
   - Using some data structures such as linked list to store
   - the iotlb mapping. The munmap(2) should be called for the
   - cached mapping when the corresponding VDUSE_UPDATE_IOTLB
   - message is received or the device is reset.
		 */

		return addr + iova - entry.v1.start;
	}

关于 uAPI 的更多细节，请参见 include/uapi/linux/vduse.h。
