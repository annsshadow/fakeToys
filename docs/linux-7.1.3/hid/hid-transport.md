## HID I/O 传输驱动

HID 子系统独立于底层的传输驱动。最初仅支持 USB，但其他规范也采纳了 HID 设计并提供了新的传输驱动。内核至少包含对 USB、Bluetooth、I2C 以及用户空间 I/O 驱动的支持
## 1) HID 总线

HID 子系统被设计为一个总线。任I/O 子系统都可以提供 HID 设备并向 HID 总线注册。随HID core 在其上加载通用的设备驱动。传输驱动负责原始数据的传输以及设备的建立与管理。HID core 负责报告解析、报告解释以及用户空API。设备的特定细节和怪异行为（quirks）由各层根据具体情况处理
```

 +-----------+  +-----------+            +-----------+  +-----------+
 | Device #1 |  | Device #i |            | Device #j |  | Device #k |
 +-----------+  +-----------+            +-----------+  +-----------+
          \      //                              \      //
        +------------+                          +------------+
        | I/O Driver |                          | I/O Driver |
        +------------+                          +------------+
              ||                                      ||
     +------------------+                    +------------------+
     | Transport Driver |                    | Transport Driver |
     +------------------+                    +------------------+
                       \___                ___/
                           \              /
                          +----------------+
                          |    HID Core    |
                          +----------------+
                           /  |        |  \
                          /   |        |   \
             ____________/    |        |    \_________________
            /                 |        |                      \
           /                  |        |                       \
 +----------------+  +-----------+  +------------------+  +------------------+
 | Generic Driver |  | MT Driver |  | Custom Driver #1 |  | Custom Driver #2 |
 +----------------+  +-----------+  +------------------+  +------------------+

```
示例驱动
  - I/O：USB、I2C、Bluetooth-l2cap
  - Transport：USB-HID、I2C-HID、BT-HIDP

本图"HID Core" 以下的部分做了简化，因为那些部分仅与 HID 设备驱动相关。传输驱动无需了解这些细节
### 1.1) 设备建立

I/O 驱动通常向传输驱动提供热插拔检测或设备枚举 API。传输驱动利用这些来寻找合适的 HID 设备。它们分HID 设备对象并向 HID core 注册。传输驱动无需HID core 注册自身。HID core 永远不知道有哪些传输驱动可用，也不关心这一点，它只关心设备
传输驱动会为每个设备附加一个常驻的 "struct hid_ll_driver" 对象。一旦设备向 HID core 注册，HID core 就会通过该结构体提供的回调与设备通信
传输驱动负责检测设备故障与拔除。只要设备仍处于注册状态，无论是否有设备故障，HID core 都会继续操作该设备。一旦传输驱动检测到拔除或故障事件，就必须从 HID core 注销该设备，此后 HID core 将停止使用所提供的回调
### 1.2) 传输驱动要求

本文档中"asynchronous"（异步）"synchronous"（同步）描述的是与确认（acknowledgement）相关的传输行为。异步通道不得执行任何同步操作，例如等待确认或校验。通常，在异步通道上运行的 HID 调用必须能够atomic-context（原子上下文）中良好工作另一方面，同步通道可以由传输驱动以任意方式实现。它们可能与异步通道相同，但也可以以阻塞方式提供确认报告、失败自动重传等。如果异步通道上需要此类功能，传输驱动必须通过其自身的 worker 线程来实现
HID core 要求传输驱动遵循特定的设计。传输驱动必须为每个 HID 设备提供两个双向 I/O 通道。这些通道在硬件上本身未必是双向的。传输驱动也可能只提4 个单向通道，或者将全部四个通道多路复用到单一物理通道上。但在本文档中，我们将它们描述为两个双向通道，因为它们具有若干共同特性
 - 中断通道（intr）：intr 通道用于异步数据报告。本通道上不发送管理命令或数据确认。任何未经请求的传入或传出数据报告都必须通过本通道发送，且远端不会发送确认。设备通常在本通道上发送其输入事件。除非需要高吞吐量，否则传出事件一般不通过 intr 发送 - 控制通道（ctrl）：ctrl 通道用于同步请求与设备管理。未经请求的数据输入事件不得在本通道发送，通常会被忽略。相反，设备只在本通道上发送管理事件或是对主机请求的应答   control 通道用于对设备进行直接的阻塞查询，与 intr 通道上的任何事件无关   传出报告通常通过同步SET_REPORT 请求ctrl 通道上发送
设备HID core 之间的通信主要通过 HID 报告完成。报告可以是以下三种类型之一
 - INPUT 报告（INPUT Report）：INPUT 报告提供从设备到主机的数据。这些数据可能包含按键事件、轴事件、电池状态等。这些数据由设备生成，并可在有或没有显式请求的情况下发送给主机。设备可以选择持续发送数据，或仅在状态改变时发送 - OUTPUT 报告（OUTPUT Report）：OUTPUT 报告用于改变设备状态。它们从主机发往设备，可能包LED 请求、震动请求等。OUTPUT 报告永远不会从设备发往主机，但主机可以获取它们的当前状态   主机可以选择持续发OUTPUT 报告，或仅在状态改变时发送 - FEATURE 报告（FEATURE Report）：FEATURE 报告用于特定的静态设备特性，从不自发上报。主机可以读取和/或写入它们以访问诸如电池状态或设备设置之类的数据   FEATURE 报告绝不会在无请求的情况下发送。主机必须显式地设置或获FEATURE 报告。这也意味着 FEATURE 报告永远不会intr 通道上发送，因为该通道是异步的
INPUT OUTPUT 报告可以作为纯数据报告在 intr 通道上发送。对 INPUT 报告而言这是常规的运行模式。但OUTPUT 报告而言很少这样做，因为 OUTPUT 报告通常相当稀少。不过设备可自由地大量使用异OUTPUT 报告（例如，定制HID 音频扬声器就大量使用该机制）
不过，纯报告不得ctrl 通道上发送。相反，ctrl 通道提供同步GET/SET_REPORT 请求。纯报告只允许在 intr 通道上发送，并且是通道上唯一的数据传输方式
 - GET_REPORT：GET_REPORT 请求以报ID 作为载荷，由主机发往设备。设备必须以针对所请求报告 ID 的数据报告作为同步确认，ctrl 通道上应答。每个设备只能有一GET_REPORT 请求处于挂起状态。由于部分传输驱动不允许同时发起多个 GET_REPORT 请求，HID core 强制实施了这一限制   注意，作GET_REPORT 请求应答而被发送的数据报告，不会被当作通用设备事件处理。也就是说，如果设备不运行在持续数据上报模式，对 GET_REPORT 的应答不会替intr 通道上状态改变时的原始数据报告   GET_REPORT 仅由定制HID 设备驱动用于查询设备状态。通常 HID core 会缓存任意设备状态，因此除了在设备初始化期间为获取当前状态外，遵HID 规范的设备并不需要此请求   GET_REPORT 请求可针对三种报告类型中的任意一种发送，并应返回设备的当前报告状态。但是，若规范不允许，底层传输驱动可能会阻止OUTPUT 报告作为载荷 - SET_REPORT：SET_REPORT 请求以报ID 加数据作为载荷。它由主机发往设备，设备必须根据所给数据更新其当前报告状态。可使用三种报告类型中的任意一种。但是，若规范不允许，底层传输驱动可能会阻止INPUT 报告作为载荷   设备必须以同步确认应答。但是，HID core 并不要求传输驱动将该确认转发HID core   GET_REPORT 相同，同一时刻只能有一SET_REPORT 处于挂起状态。由于部分传输驱动不支持多个同步 SET_REPORT 请求，HID core 强制实施了这一限制
其他 ctrl 通道请求USB-HID 支持，但在大多数其他传输层规范中不可用（或已被弃用）
 - GET/SET_IDLE：仅USB-HID I2C-HID 使用 - GET/SET_PROTOCOL：HID core 不使用 - RESET：由 I2C-HID 使用，未HID core 中挂接 - SET_POWER：由 I2C-HID 使用，未HID core 中挂接
## 2) HID API

### 2.1) 初始
传输驱动通常使用以下流程来注册一个新设备

```

	struct hid_device *hid;
	int ret;

	hid = hid_allocate_device();
	if (IS_ERR(hid)) {
		ret = PTR_ERR(hid);
		goto err_<...>;
	}

	strscpy(hid->name, <device-name-src>, sizeof(hid->name));
	strscpy(hid->phys, <device-phys-src>, sizeof(hid->phys));
	strscpy(hid->uniq, <device-uniq-src>, sizeof(hid->uniq));

	hid->ll_driver = &custom_ll_driver;
	hid->bus = <device-bus>;
	hid->vendor = <device-vendor>;
	hid->product = <device-product>;
	hid->version = <device-version>;
	hid->country = <device-country>;
	hid->dev.parent = <pointer-to-parent-device>;
	hid->driver_data = <transport-driver-data-field>;

	ret = hid_add_device(hid);
	if (ret)
		goto err_<...>;

```

一旦进hid_add_device()，HID core 就可能使"custom_ll_driver" 中提供的回调。注意，若底层传输驱动不支持，则"country" 这样的字段可被忽略
```

	hid_destroy_device(hid);

```

一hid_destroy_device() 返回，HID core 将不再使用任何驱动回调
### 2.2) hid_ll_driver 操作

可用HID 回调如下
```

      int (*start) (struct hid_device *hdev)

   Called from HID device drivers once they want to use the device. Transport
   drivers can choose to setup their device in this callback. However, normally
   devices are already set up before transport drivers register them to HID core
   so this is mostly only used by USB-HID.

   ::

      void (*stop) (struct hid_device *hdev)

   Called from HID device drivers once they are done with a device. Transport
   drivers can free any buffers and deinitialize the device. But note that
   ->start() might be called again if another HID device driver is loaded on the
   device.

   Transport drivers are free to ignore it and deinitialize devices after they
   destroyed them via hid_destroy_device().

   ::

      int (*open) (struct hid_device *hdev)

   Called from HID device drivers once they are interested in data reports.
   Usually, while user-space didn't open any input API/etc., device drivers are
   not interested in device data and transport drivers can put devices asleep.
   However, once ->open() is called, transport drivers must be ready for I/O.
   ->open() calls are nested for each client that opens the HID device.

   ::

      void (*close) (struct hid_device *hdev)

   Called from HID device drivers after ->open() was called but they are no
   longer interested in device reports. (Usually if user-space closed any input
   devices of the driver).

   Transport drivers can put devices asleep and terminate any I/O of all
   ->open() calls have been followed by a ->close() call. However, ->start() may
   be called again if the device driver is interested in input reports again.

   ::

      int (*parse) (struct hid_device *hdev)

   Called once during device setup after ->start() has been called. Transport
   drivers must read the HID report-descriptor from the device and tell HID core
   about it via hid_parse_report().

   ::

      int (*power) (struct hid_device *hdev, int level)

   Called by HID core to give PM hints to transport drivers. Usually this is
   analogical to the ->open() and ->close() hints and redundant.

   ::

      void (*request) (struct hid_device *hdev, struct hid_report *report,
		       int reqtype)

   Send a HID request on the ctrl channel. "report" contains the report that
   should be sent and "reqtype" the request type. Request-type can be
   HID_REQ_SET_REPORT or HID_REQ_GET_REPORT.

   This callback is optional. If not provided, HID core will assemble a raw
   report following the HID specs and send it via the ->raw_request() callback.
   The transport driver is free to implement this asynchronously.

   ::

      int (*wait) (struct hid_device *hdev)

   Used by HID core before calling ->request() again. A transport driver can use
   it to wait for any pending requests to complete if only one request is
   allowed at a time.

   ::

      int (*raw_request) (struct hid_device *hdev, unsigned char reportnum,
                          __u8 *buf, size_t count, unsigned char rtype,
                          int reqtype)

   Same as ->request() but provides the report as raw buffer. This request shall
   be synchronous. A transport driver must not use ->wait() to complete such
   requests. This request is mandatory and hid core will reject the device if
   it is missing.

   ::

      int (*output_report) (struct hid_device *hdev, __u8 *buf, size_t len)

   Send raw output report via intr channel. Used by some HID device drivers
   which require high throughput for outgoing requests on the intr channel. This
   must not cause SET_REPORT calls! This must be implemented as asynchronous
   output report on the intr channel!

   ::

      int (*idle) (struct hid_device *hdev, int report, int idle, int reqtype)

   Perform SET/GET_IDLE request. Only used by USB-HID, do not implement!

```

### 2.3) 数据路径

传输驱动负责I/O 设备读取数据。它们必须自行处理任何与 I/O 相关的状态跟踪。HID core 不实现协议握手或其他管理命令，而这类命令可能是给定 HID 传输规范所要求的
从设备读取到的每个原始数据包都必须通过 hid_input_report() 送入 HID core。你必须指定通道类型（intr ctrl）以及报告类型（input/output/feature）。正常情况下，通过API 提供的只input 报告
经由 ->request() 发出GET_REPORT 请求的应答也必须通过API 提供。->raw_request() 的应答是同步的，必须由传输驱动拦截，不得传递给 hid_input_report()SET_REPORT 请求的确认，HID core 并不关心
----------------------------------------------------

Written 2013, David Herrmann <dh.herrmann@gmail.com>
