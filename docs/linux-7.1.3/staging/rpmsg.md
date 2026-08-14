## 远程处理器消息传递（Remote Processor Messaging，rpmsg）框架


  本文档描述了 rpmsg 总线以及如何编写 rpmsg 驱动。要了解如何为新平台添加 rpmsg 支持，
  请查看 remoteproc.txt（同样位于 Documentation/ 中）。

## 简介

现代 SoC 通常采用异构远程处理器设备，以非对称多处理（AMP）配置运行，这些处理器可能
运行不同实例的操作系统，无论是 Linux 还是任何其他实时操作系统。

例如，OMAP4 拥有双核 Cortex-A9、双核 Cortex-M3 以及一个 C64x+ DSP。通常，双核 Cortex-A9
以 SMP 配置运行 Linux，而其他三个核心（两个 M3 核心和一个 DSP）各自以 AMP 配置运行自己的
RTOS 实例。

通常 AMP 远程处理器会使用专用的 DSP 编解码器和多媒体硬件加速器，因此常被用来将 CPU 密集的
多媒体任务从主应用处理器上卸载下来。

这些远程处理器也可用于控制对延迟敏感的传感器、驱动各种硬件模块，或者仅仅在主 CPU 空闲时
执行后台任务。

这些远程处理器的使用者既可以是用户态应用（例如与远程 OMX 组件通信的多媒体框架），也可以
是内核驱动（控制只有远程处理器才能访问的硬件、代表远程处理器保留内核控制的资源等）。

Rpmsg 是一个基于 virtio 的消息传递总线，允许内核驱动与系统上可用的远程处理器通信。进而，
驱动可以在需要时暴露合适的用户空间接口。

在编写向用户态暴露 rpmsg 通信的驱动时，请记住远程处理器可能直接访问系统的物理内存和其他
敏感硬件资源（例如，在 OMAP4 上，远程核心和硬件加速器可能直接访问物理内存、gpio bank、
dma controller、i2c 总线、gptimer、mailbox 设备、hwspinlock 等）。此外，这些远程处理器
可能运行 RTOS，其中每个任务都可以访问暴露给该处理器的整个内存/设备。为了将恶意（或有缺陷）
的用户态代码利用远程漏洞并借此接管系统的风险降到最低，通常希望将用户态限制在它可以发送
消息的特定 rpmsg 通道上（见下定义），并在可能的情况下尽量减少它对消息内容的控制。

每个 rpmsg 设备都是与远程处理器的一条通信通道（因此 rpmsg 设备被称为通道）。通道由文本名称
标识，并具有一个本地（“源”）rpmsg 地址和一个远程（“目的”）rpmsg 地址。

当驱动开始在某个通道上监听时，其 rx 回调函数会绑定到一个唯一的 rpmsg 本地地址（一个 32 位
整数）。这样，当入站消息到达时，rpmsg 核心会根据其目的地址将它们分派给合适的驱动（这是通过
用入站消息的负载调用驱动的 rx 处理程序来完成的）。

## 用户 API

```
  int rpmsg_send(struct rpmsg_endpoint *ept, void *data, int len);

```
从给定端点向远程处理器发送一条消息。调用者应指定端点、想要发送的数据及其长度（以字节为单位）。
消息将在指定端点的通道上发送，即其源地址和目的地址字段将分别被设为端点的 src 地址及其父通道的
dst 地址。

如果没有可用的 TX 缓冲区，该函数会阻塞，直到有一个变得可用（即直到远程处理器消费了一个 tx
缓冲区并将其放回 virtio 的 used descriptor ring），或者直到 15 秒超时。当后者发生时，返回
-ERESTARTSYS。

该函数目前只能从进程上下文中调用。成功时返回 0，失败时返回相应的错误值。

```
  int rpmsg_sendto(struct rpmsg_endpoint *ept, void *data, int len, u32 dst);

```
从给定端点向远程处理器发送一条消息，目的地址由调用者提供。

调用者应指定端点、想要发送的数据、其长度（以字节为单位），以及一个显式的目的地址。

消息随后将使用端点的 src 地址和用户提供的 dst 地址，发送到端点所属通道所对应的远程处理器
（因此通道的 dst 地址会被忽略）。

如果没有可用的 TX 缓冲区，该函数会阻塞，直到有一个变得可用（即直到远程处理器消费了一个 tx
缓冲区并将其放回 virtio 的 used descriptor ring），或者直到 15 秒超时。当后者发生时，返回
-ERESTARTSYS。

该函数目前只能从进程上下文中调用。成功时返回 0，失败时返回相应的错误值。

```
  int rpmsg_trysend(struct rpmsg_endpoint *ept, void *data, int len);

```
从给定端点向远程处理器发送一条消息。调用者应指定端点、想要发送的数据及其长度（以字节为单位）。
消息将在指定端点的通道上发送，即其源地址和目的地址字段将分别被设为端点的 src 地址及其父通道的
dst 地址。

如果没有可用的 TX 缓冲区，该函数会立即返回 -ENOMEM，而不等待有缓冲区变得可用。

该函数目前只能从进程上下文中调用。成功时返回 0，失败时返回相应的错误值。

```
  int rpmsg_trysendto(struct rpmsg_endpoint *ept, void *data, int len, u32 dst)


```
从给定端点向远程处理器发送一条消息，目的地址由用户提供。

用户应指定通道、想要发送的数据、其长度（以字节为单位），以及一个显式的目的地址。

消息随后将使用通道的 src 地址和用户提供的 dst 地址发送到通道所属的远程处理器（因此通道的
dst 地址会被忽略）。

如果没有可用的 TX 缓冲区，该函数会立即返回 -ENOMEM，而不等待有缓冲区变得可用。

该函数目前只能从进程上下文中调用。成功时返回 0，失败时返回相应的错误值。

```
  struct rpmsg_endpoint *rpmsg_create_ept(struct rpmsg_device *rpdev,
					  rpmsg_rx_cb_t cb, void *priv,
					  struct rpmsg_channel_info chinfo);

```
系统中每一个 rpmsg 地址都通过一个 rpmsg_endpoint 结构绑定到一个 rx 回调函数（因此当入站消息
到达时，它们由 rpmsg 总线使用相应的回调处理程序来分派）。

该函数允许驱动创建这样一个端点，并借此将一个回调（可能还有某些私有数据）绑定到一个 rpmsg 地址
（既可以是预先已知的地址，也可以是为它们动态分配的地址）。

简单的 rpmsg 驱动无需调用 rpmsg_create_ept，因为当它们被 rpmsg 总线探测（probe）时，已经为它们
创建了一个端点（使用它们向 rpmsg 总线注册时提供的 rx 回调）。

因此对于简单驱动来说一切应当开箱即用：它们已经拥有端点，其 rx 回调绑定到了它们的 rpmsg 地址，
当相关的入站消息到达时（即目的地址等于其 rpmsg 通道 src 地址的消息），驱动的 handler 会被
调用来处理它。

也就是说，更复杂的驱动可能确实需要有额外分配的 rpmsg 地址，并将它们绑定到不同的 rx 回调。
为此，这些驱动需要调用该函数。驱动应提供它们的通道（这样新端点会绑定到其通道所属的同一远程
处理器）、一个 rx 回调函数、可选的私有数据（在 rx 回调被调用时会传回），以及它们想要绑定回调的
地址。如果 addr 为 RPMSG_ADDR_ANY，那么 rpmsg_create_ept 会为它们动态分配一个可用的 rpmsg 地址
（驱动应当有非常充分的理由才不在这里始终使用 RPMSG_ADDR_ANY）。

成功时返回指向端点的指针，出错时返回 NULL。

```
  void rpmsg_destroy_ept(struct rpmsg_endpoint *ept);


```
销毁一个已存在的 rpmsg 端点。用户应提供一个之前由 rpmsg_create_ept() 创建的 rpmsg 端点指针。

```
  int register_rpmsg_driver(struct rpmsg_driver *rpdrv);


```
向 rpmsg 总线注册一个 rpmsg 驱动。用户应提供一个指向 rpmsg_driver 结构的指针，其中包含驱动的
->probe() 和 ->remove() 函数、一个 rx 回调，以及一个 id_table，指定该驱动希望被探测到的通道
名称。

```
  void unregister_rpmsg_driver(struct rpmsg_driver *rpdrv);


```
从 rpmsg 总线注销一个 rpmsg 驱动。用户应提供一个之前注册的 rpmsg_driver 结构指针。
成功时返回 0，失败时返回相应的错误值。

## 典型用法

下面是一个简单的 rpmsg 驱动，它在 probe() 时发送一条 "hello!" 消息，并在每次收到入站消息时
将其内容转储到控制台。

```
  #include <linux/dev_printk.h>
  #include <linux/mod_devicetable.h>
  #include <linux/module.h>
  #include <linux/printk.h>
  #include <linux/rpmsg.h>
  #include <linux/types.h>

  static void rpmsg_sample_cb(struct rpmsg_channel *rpdev, void *data, int len,
						void *priv, u32 src)
  {
	print_hex_dump(KERN_INFO, "incoming message:", DUMP_PREFIX_NONE,
						16, 1, data, len, true);
  }

  static int rpmsg_sample_probe(struct rpmsg_channel *rpdev)
  {
	int err;

	dev_info(&rpdev->dev, "chnl: 0x%x -> 0x%x\n", rpdev->src, rpdev->dst);

	/* send a message on our channel */
	err = rpmsg_send(rpdev->ept, "hello!", 6);
	if (err) {
		dev_err(&rpdev->dev, "rpmsg_send failed: %d\n", err);
		return err;
	}

	return 0;
  }

  static void rpmsg_sample_remove(struct rpmsg_channel *rpdev)
  {
	dev_info(&rpdev->dev, "rpmsg sample client driver is removed\n");
  }

  static struct rpmsg_device_id rpmsg_driver_sample_id_table[] = {
	{ .name	= "rpmsg-client-sample" },
	{ },
  };
  MODULE_DEVICE_TABLE(rpmsg, rpmsg_driver_sample_id_table);

  static struct rpmsg_driver rpmsg_sample_client = {
	.drv.name	= KBUILD_MODNAME,
	.id_table	= rpmsg_driver_sample_id_table,
	.probe		= rpmsg_sample_probe,
	.callback	= rpmsg_sample_cb,
	.remove		= rpmsg_sample_remove,
  };
  module_rpmsg_driver(rpmsg_sample_client);

```

   一个可以构建并加载的类似示例可在 samples/rpmsg/ 中找到。

## rpmsg 通道的分配

目前我们只支持动态分配 rpmsg 通道。

这只有在具备 VIRTIO_RPMSG_F_NS virtio 设备特性集的远程处理器上才可能。该特性位意味着远程
处理器支持动态名称服务宣告消息。

当启用该特性时，rpmsg 设备（即通道）的创建是完全动态的：远程处理器通过发送一条名称服务消息
（其中包含远程服务的名称和 rpmsg 地址，参见 struct rpmsg_ns_msg）来宣告一个远程 rpmsg 服务的
存在。

这条消息随后由 rpmsg 总线处理，并由此动态创建并注册一个 rpmsg 通道（代表该远程服务）。当
（如果）一个相关的 rpmsg 驱动被注册时，它会立即被总线探测，然后就可以开始向远程服务发送消息。

我们也计划通过 virtio 配置空间添加 rpmsg 通道的静态创建，但这尚未实现。
