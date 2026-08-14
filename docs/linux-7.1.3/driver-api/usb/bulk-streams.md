#### USB bulk 流


## 背景


批量端点流（bulk endpoint streams）在 USB 3.0 规范中引入。流允许设备驱动对一个批量端点进行复用，从而可以一次性排队多个传输。

流在 https://www.usb.org/developers/docs/ 的 Universal Serial Bus 3.0 规范的第 4.4.6.4 节与第 8.12.1.4 节中定义。使用流来排队多个 SCSI 命令的 USB Attached SCSI Protocol 可在 T10 网站（https://t10.org/）上找到。


## 设备侧影响


一旦缓冲区被排队到某个流环（stream ring），设备就会（通过另一个端点上的带外机制）收到通知，表明该 stream ID 的数据已就绪。随后设备告诉主机它想启动哪个“流”。主机也可以在没有设备请求的情况下主动在某个流上发起传输，但设备可以拒绝该传输。设备可以随时在流之间切换。


## 驱动影响


```

  int usb_alloc_streams(struct usb_interface *interface,
		struct usb_host_endpoint **eps, unsigned int num_eps,
		unsigned int num_streams, gfp_t mem_flags);

```

设备驱动将调用此 API，请求主机控制器驱动分配内存，以便该驱动能够使用多达 num_streams 个 stream ID。它们必须传入一个需要以相似 stream ID 进行设置的 usb_host_endpoints 数组。这是为了确保 UASP 驱动能够在双向命令序列所用的批量 IN 与 OUT 端点上使用相同的 stream ID。

返回值是一个错误状态（如果某个端点不支持流，或 xHCI 驱动内存耗尽），或者是主机控制器为该端点分配的流数量。xHCI 主机控制器硬件声明了它能支持多少个 stream ID，而 SuperSpeed 设备上的每个批量端点也会声明它能处理多少个 stream ID。因此，驱动应当能够处理被分配的 stream ID 少于其请求数量的情况。

如果对作为参数传入的某个端点有 URB 已排队，请勿调用此函数。不要调用此函数请求少于两个流。

在没有调用 usb_free_streams() 的情况下，驱动只允许对同一端点调用此 API 一次。这是对 xHCI 主机控制器驱动的简化，未来可能会改变。


## 选择要使用的新的 Stream ID


Stream ID 0 是保留的，不应被用于与设备通信。如果 usb_alloc_streams() 返回值为 N，则你可以使用 1 到 N 的流。要为一个特定的流排队 URB，请设置 urb->stream_id 的值。如果该端点不支持流，将返回错误。

注意，如果 xHCI 驱动支持次级 stream ID，则需要新增用于选择下一个 stream ID 的 API。


## 清理


如果驱动希望停止使用流来与设备通信，它

```

  void usb_free_streams(struct usb_interface *interface,
		struct usb_host_endpoint **eps, unsigned int num_eps,
		gfp_t mem_flags);

```

当驱动释放接口时，所有 stream ID 都会被释放，以确保不支持流的驱动也能使用该端点。
