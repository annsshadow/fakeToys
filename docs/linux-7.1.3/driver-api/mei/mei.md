
## 简


Intel Management Engine（Intel ME，英特尔管理引擎）是驻留在某Intel 芯片组内部的一个隔离且受保护的计算资源（协处理器）。Intel ME 为计算机/IT 管理与安全特性提供支持。实际的功能集合取决Intel 芯片组的 SKU

Intel Management Engine Interface（Intel MEI，此前称HECI）是主机Intel ME 之间的接口。该接口作为 PCI 设备暴露给主机，实际上可能会暴露出多PCI 设备。Intel MEI 驱动负责主机应用程序Intel ME 特性之间的通信通道

每个 Intel ME 特性（Intel ME 客户端）都由一个唯一GUID 寻址，并且每个客户端都有自己的协议。该协议是基于消息的，带有一个头部和负载，负载最大字节数由客户端在连接时通告

## Intel MEI 驱动


该驱动暴露一个字符设备，其设备节点为 /dev/meiX

应用程序/dev/meiX 处于打开状态时与某Intel ME 特性保持通信。与特定特性的绑定通过调用 `MEI_CONNECT_CLIENT_IOCTL` 完成，该调用传入期望GUID。可以同时打开的某Intel ME 特性的实例数量取决于该 Intel ME 特性，但大多数特性只允许单个实例

该驱动对固件特性与主机应用程序之间传递的数据是透明的

由于某些 Intel ME 特性可以改变系统配置，默认情况下该驱动只允许特权用户访问它

会话通过调用 :c`close(fd)` 终止

一个与 Intel AMTHI 客户端通信的应用程序的代码片段

为了支持虚拟化或沙箱化，受信任的监管程序可以使用 `MEI_CONNECT_CLIENT_IOCTL_VTAG` 来与某个 Intel ME 特性创建虚拟通道。并非所有特性都支持虚拟通道，这样的客户端会回答 EOPNOTSUPP


	struct mei_connect_client_data data;
	fd = open(MEI_DEVICE);

	data.d.in_client_uuid = AMTHI_GUID;

	ioctl(fd, IOCTL_MEI_CONNECT_CLIENT, &data);

	printf("Ver=%d, MaxLen=%ld\n",
	       data.d.in_client_uuid.protocol_version,
	       data.d.in_client_uuid.max_msg_length);

	[...]

	write(fd, amthi_req_data, amthi_req_data_len);

	[...]

	read(fd, &amthi_res_data, amthi_res_data_len);

	[...]
	close(fd);


用户空间 API

## IOCTL锛。


Intel MEI 驱动支持以下 IOCTL 命令

### IOCTL_MEI_CONNECT_CLIENT


连接到固件特客户端


	Usage:

        struct mei_connect_client_data client_data;

        ioctl(fd, IOCTL_MEI_CONNECT_CLIENT, &client_data);

	Inputs:

        struct mei_connect_client_data - 包含以下内容
	Input field:

		in_client_uuid -	需要连接到FW 特性的 GUID
         Outputs:
		out_client_properties - 客户端属性：MTU 与协议版本

         Error returns:

                ENOTTY  没有这样的客户端（即错误GUID）或连接不被允许
		EINVAL	错误IOCTL 编号
		ENODEV	设备或连接未初始化或尚未就绪
		ENOMEM	无法为客户端内部数据分配内存
		EFAULT	致命错误（例如无法访问用户输入数据）
		EBUSY	连接已经打开

:Note:
        max_msg_length（MTU）在客户端属性中描述了可以发送或接收的最大数据。（例如，如MTU=2K，则可以发送最2k 字节的请求，并接收最2k 字节的响应）

### IOCTL_MEI_CONNECT_CLIENT_VTAG锛。



        Usage:

        struct mei_connect_client_data_vtag client_data_vtag;

        ioctl(fd, IOCTL_MEI_CONNECT_CLIENT_VTAG, &client_data_vtag);

        Inputs:

        struct mei_connect_client_data_vtag - 包含以下内容
        Input field:

                in_client_uuid -  需要连接到FW 特性的 GUID
                vtag - 虚拟标签 [1, 255]

         Outputs:
                out_client_properties - 客户端属性：MTU 与协议版本

         Error returns:

                ENOTTY 没有这样的客户端（即错误GUID）或连接不被允许
                EINVAL 错误IOCTL 编号tag == 0
                ENODEV 设备或连接未初始化或尚未就绪
                ENOMEM 无法为客户端内部数据分配内存
                EFAULT 致命错误（例如无法访问用户输入数据）
                EBUSY  连接已经打开
                EOPNOTSUPP 不支Vtag

### IOCTL_MEI_NOTIFY_SET


启用或禁用事件通知



	Usage:

		uint32_t enable;

		ioctl(fd, IOCTL_MEI_NOTIFY_SET, &enable);


		uint32_t enable = 1;
		or
		uint32_t enable[disable] = 0;

	Error returns:


		EINVAL	错误IOCTL 编号
		ENODEV	设备未初始化或客户端未连
		ENOMEM	无法为客户端内部数据分配内存
		EFAULT	致命错误（例如无法访问用户输入数据）
		EOPNOTSUPP 如果设备不支持该特

:Note:
	客户端必须已连接才能启用通知事件


### IOCTL_MEI_NOTIFY_GET


检索事



	Usage:
		uint32_t event;
		ioctl(fd, IOCTL_MEI_NOTIFY_GET, &event);

	Outputs:
		1 - 如果有事件待处理
		0 - 如果没有事件待处

	Error returns:
		EINVAL	错误IOCTL 编号
		ENODEV	设备未初始化或客户端未连
		ENOMEM	无法为客户端内部数据分配内存
		EFAULT	致命错误（例如无法访问用户输入数据）
		EOPNOTSUPP 如果设备不支持该特

:Note:
	客户端必须已连接，并且必须已启用事件通知，才能接收事



## 支持的芯片组


82X38/X48 Express 及更新的型号

linux-mei@linux.intel.com
