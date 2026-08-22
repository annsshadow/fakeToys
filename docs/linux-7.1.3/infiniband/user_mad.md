## 用户MAD 访问


## 设备文件


  每个 InfiniBand 设备的每个端口都附有一"umad" 设备和一"issm" 设备。例如，
  一个双端口HCA 将有两个 umad 设备和两issm 设备，而一个交换机（switch）将  每种类型各一个设备（对应交换机端0）
## 创建 MAD 代理（agents

  可以通过填充一struct ib_user_mad_reg_req，然后在对相应设备文件的文件描述  上调IB_USER_MAD_REGISTER_AGENT ioctl 来创建一MAD 代理。如果注册请求成功，
  一32 位的 id 将被返回到该结构中```

	struct ib_user_mad_reg_req req = { /* ... */ };
	ret = ioctl(fd, IB_USER_MAD_REGISTER_AGENT, (char *) &req);
        if (!ret)
		my_agent = req.id;
	else
		perror("agent register");

  代理可以通过 IB_USER_MAD_UNREGISTER_AGENT ioctl 注销。此外，通过一个文件描述符
  注册的所有代理将在这该描述符被关闭时注销
  2014
       现在提供了一个新的注ioctl，允许在注册期间提供额外的字段。此注册调用       使用者隐式地设置pkey_index 的使用（见下文）
```
## 接收 MAD


  MAD 通过 read() 接收。接收端现在支持 RMPP。传read() 的缓冲区必须至少  一struct ib_user_mad + 256 字节。例如：

  如果传入的缓冲区不够大以容纳接收到的 MAD（RMPP），errno 会被设为 ENOSPC，并  所需缓冲区的长度被设置到 mad.length 中
```

	struct ib_user_mad *mad;
	mad = malloc(sizeof *mad + 256);
	ret = read(fd, mad, sizeof *mad + 256);
	if (ret != sizeof mad + 256) {
		perror("read");
		free(mad);
	}

  RMPP 读取的示:

	struct ib_user_mad *mad;
	mad = malloc(sizeof *mad + 256);
	ret = read(fd, mad, sizeof *mad + 256);
	if (ret == -ENOSPC)) {
		length = mad.length;
		free(mad);
		mad = malloc(sizeof *mad + length);
		ret = read(fd, mad, sizeof *mad + length);
	}
	if (ret < 0) {
		perror("read");
		free(mad);
	}

  除了实际MAD 内容外，struct ib_user_mad 的其它字段也会被填充上关于接收到  MAD 的信息。例如，远端 LID 将在 mad.lid 中
  如果发送超时，将生成一个接收，mad.status 被设ETIMEDOUT。否则，当一MAD
  被成功接收时，mad.status 将为 0
  poll()/select() 可用于等待直到一MAD 可以被读取
```
## 发MAD


  MAD 通过 write() 发送。用于发送的代理 ID 应被填入 MAD id 字段，目LID 应被
  填入 lid 字段，依此类推。发送端确实支持
```

	struct ib_user_mad *mad;

	mad = malloc(sizeof *mad + mad_length);

	/* 填充 mad->data */

	mad->hdr.id  = my_agent;	/* 来自代理注册req.id */
	mad->hdr.lid = my_dest;		/* 缃戠粶瀛楄妭搴?.. */
	/* 等等 */

	ret = write(fd, &mad, sizeof *mad + mad_length);
	if (ret != sizeof *mad + mad_length)
		perror("write");

```
## 事务 ID（Transaction IDs

  用户umad 设备的使用者可以使用事ID 字段的低 32 位（即在网络字节序中该字  的较低有效一半）来匹配正在发送的 MAD 中的请求/响应对。高 32 位保留给内核使用  并将MAD 被发送之前被覆盖
## P_Key 索引处理


  旧的 ib_umad 接口不允许为发送的 MAD 设置 P_Key 索引，也不提供获取接收到MAD   P_Key 索引的方法。已经定义了一个带pkey_index 成员struct ib_user_mad_hdr   新布局；然而，为了与旧应用程序保持二进制兼容性，除非在文件描述符被用于其它任  操作之前调用IB_USER_MAD_ENABLE_PKEY IB_USER_MAD_REGISTER_AGENT2 ioctl 之一  否则不会使用这个新布局
  2008 9 月，IB_USER_MAD_ABI_VERSION 将递增6，struct ib_user_mad_hdr   新布局将默认使用，并且 IB_USER_MAD_ENABLE_PKEY ioctl 将被移除
## 设置 IsSM 能力

  要为某个端口设置 IsSM 能力位，只需打开相应issm 设备文件。如IsSM 位已经设置，
  open 调用将阻塞直到该位被清除（或者，如果open() 传入O_NONBLOCK 标志，则
  立即返回并将 errno 设为 EAGAIN）。当 issm 文件被关闭时，IsSM 位将被清除。不能对
  issm 文件执行 read、write 或其它操作
## /dev 文件


  要使用以下规则自动创建相应的字符设备文件
```

    KERNEL=="umad*", NAME="infiniband/%k"
    KERNEL=="issm*", NAME="infiniband/%k"

  这可以用来。这将创建设备节点，命名:

    /dev/infiniband/umad0
    /dev/infiniband/issm0

  对应第一个端口，依此类推。与这些设备关联InfiniBand 设备和端口可以从以下文件
  确定::

    /sys/class/infiniband_mad/umad0/ibdev
    /sys/class/infiniband_mad/umad0/port

  以及::

    /sys/class/infiniband_mad/issm0/ibdev
    /sys/class/infiniband_mad/issm0/port

```
