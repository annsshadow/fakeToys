## UHID - 面向 HID 子系统的用户空间 I/O 驱动支持

UHID 允许用户在用户空间实HID 传输驱动。关HID 传输驱动的介绍，请参hid-transport.rst。本文档大量依赖其中所定义的术语
借助 UHID，用户空间的传输驱动可以为每个连接到用户空间所控制总线的设备创内核 hid 设备。UHID API 定义了由内核提供给用户空间、以及反向传递的 I/O 事件
示例用户空间应用程序位于 ./samples/uhid/uhid-example.c

### UHID API

UHID 通过一个字符杂项设备（misc-device）进行访问。次设备号是动态分配的，因你需要依udev（或类似机制）来创建设备节点。默认情况下该节点为 /dev/uhid
如果你的 HID I/O 驱动检测到一个新设备，并希望HID 子系统注册该设备，那么你需为你想注册的每个设备打开一/dev/uhid。之后所有的通信都通过 read() / write()
"struct uhid_event" 对象来完成。支持非阻塞操作```
  struct uhid_event {
        __u32 type;
        union {
                struct uhid_create2_req create2;
                struct uhid_output_req output;
                struct uhid_input2_req input2;
                ...
        } u;
  };
```
"type" 字段包含事件ID。根据该 ID 的不同，发送不同的负载。你不得将单个事拆分到多read() 或多write() 中。一个事件必须始终作为一个整体发送。此外，
每次 read() write() 只能发送一个事件。挂起的数据会被忽略。如果你想在单次
系统调用中处理多个事件，可以使用带向量的 I/O，即 readv()/writev()
"type" 字段定义了负载。对于每种类型，在联合体 "u" 中都有一个对应的负载结构（空负载除外）。该负载包含管理数据或设备数据
你应做的第一件事是发送一UHID_CREATE2 事件。这将注册设备。UHID 会以一UHID_START 事件作为响应。此时你就可以开始向 UHID 发送数据以及从 UHID 读取数据但是，除UHID 发送了 UHID_OPEN 事件，否则内部挂载的 HID 设备驱动并没有用连接。也就是说，除非你收UHID_OPEN 事件，否则你可能会让设备进入休眠状态如果你收UHID_OPEN 事件，就应该开I/O。如果最后一个用户关闭了 HID 设备你会收到一UHID_CLOSE 事件。之后可能会再次跟着一UHID_OPEN 事件，依此类推用户空间无需进行引用计数。也就是说，在没UHID_CLOSE 事件的情况下，你永远不会
收到多个 UHID_OPEN 事件。HID 子系统会替你完成引用计数
不过，你也可以选择忽略 UHID_OPEN/UHID_CLOSE。即使设备可能没有用户，也允许进I/O
如果你想在中断通道上向 HID 子系统发送数据，就发送一个带有原始数据负载的
HID_INPUT2 事件。如果内核想要在中断通道上向设备发送数据，你会读到一UHID_OUTPUT
事件。控制通道上的数据请求目前仅限GET_REPORT SET_REPORT（到目前为止尚未
定义控制通道上的其他数据报告）。这些请求始终是同步的。也就是说，内核发UHID_GET_REPORT UHID_SET_REPORT 事件，并要求你将它们转发到控制通道上的设备一旦设备作出响应，你必须通过 UHID_GET_REPORT_REPLY UHID_SET_REPORT_REPLY 响应转发给内核。内核会在这种往返过程中阻塞内部驱动的执行（超过一段硬编码的时间后
会超时）
如果你的设备断开连接，你应该发送一UHID_DESTROY 事件。这将注销该设备。之后你
可以再次发UHID_CREATE2 来注册一个新设备
如果close() fd，设备会在内部被自动注销和销毁
### write()

write() 允许你修改设备的状态，并向内核送入输入数据。内核会立即解析该事件，如果
事件 ID 不被支持，会返回 -EOPNOTSUPP。如果负载无效，则返-EINVAL，否则会返回
已读取的数据量，且请求被成功处理。O_NONBLOCK 不会影响 write()，因为写入总是非阻塞方式立即处理。不过，未来的请求可能会使用 O_NONBLOCK
UHID_CREATE2:
  这会创建内部 HID 设备。在你将该事件发送给内核之前，无法进行任I/O。负载的
  类型struct uhid_create2_req，其中包含有关你的设备的信息。你现在就可以开  I/O 了
UHID_DESTROY:
  这会销毁内HID 设备。不再接受进一步的 I/O。可能仍有待处理的消息可以通过
  read() 接收，但无法再向内核发送进一步的 UHID_INPUT 事件  你可以再次发UHID_CREATE2 来创建一个新设备。无需重新打开字符设备
UHID_INPUT2:
  在向内和发送输入之前，必须先发UHID_CREATE2！该事件包含一个数据负载。这  你在中断通道上从设备读取的原始数据。内核会解析 HID 报告
UHID_GET_REPORT_REPLY:
  如果你收到一UHID_GET_REPORT 请求，必须用此请求来应答。你必须将请求中"id"
  字段复制到应答中。如果没有错误发生，"err" 字段设为 0，如果发I/O 错误  设为 EIO。如"err" 0，则应将 GET_REPORT 请求的结果填入应答的缓冲区，并相  地设"size"
UHID_SET_REPORT_REPLY:
  这是UHID_GET_REPORT_REPLY 对应SET_REPORT 版本。与 GET_REPORT 不同  SET_REPORT 从不返回数据缓冲区，因此只需正确设置 "id" "err" 字段即可
### read()

read() 会返回一个排队的输出报告。对其中的任何一个都不必作出响应，但你应该根需要对其进行处理
UHID_START:
  HID 设备启动时发送。可将此视为UHID_CREATE2 的应答。这始终是发送的第一  事件。注意，该事件在 write(UHID_CREATE2) 返回之后可能不会立即可用。设备驱  可能需要延迟的初始化过程  该事件包含一个类型为 uhid_start_req 的负载dev_flags" 字段描述了设备的特殊
  行为。定义了以下标志
      - UHID_DEV_NUMBERED_FEATURE_REPORTS
      - UHID_DEV_NUMBERED_OUTPUT_REPORTS
      - UHID_DEV_NUMBERED_INPUT_REPORTS

          这些标志中的每一个都定义了给定报告类型是否使用编号报告。如果某类型使用
          编号报告，则内核发送的所有消息都已经以报告号为前缀。否则内核不会添          前缀。对于由用户空间发送给内核的消息，你必须根据这些标志来调整前缀
UHID_STOP:
  HID 设备停止时发送。可将此视为UHID_DESTROY 的应答
  如果你没有通过 UHID_DESTROY 销毁设备，但内核发送了 UHID_STOP 事件，通常应忽  它。这意味着内核重新加载/更改了加载在 HID 设备上的设备驱动（或发生了其他一  维护操作）
  通常你可以安全地忽略任何 UHID_STOP 事件
UHID_OPEN:
  HID 设备被打开时发送。也就是说，HID 设备提供的数据被其他某个进程读取。你
  可以忽略此事件，但它对电源管理很有用。只要你还没有收到此事件，实际上就没有其  进程读取你的数据，因此无需向内核发UHID_INPUT2 事件
UHID_CLOSE:
  当不再有进程读取 HID 数据时发送。它UHID_OPEN 的对应事件，你同样可以忽略它
UHID_OUTPUT:
  HID 设备驱动想要在中断通道上向 I/O 设备发送原始数据时发送。你应该读取负载
  并将其转发给设备。负载的类型"struct uhid_output_req"  即使你还没有收到 UHID_OPEN，也可能收到此事件
UHID_GET_REPORT:
  当内核驱动想要按HID 规范在控制通道上执GET_REPORT 请求时，会发送此事件  报告类型和报告号可在负载中获取  内核会对 GET_REPORT 请求进行串行化，因此永远不会有两个并行请求。但是，如果  未能UHID_GET_REPORT_REPLY 应答，该请求可能会静默超时  一旦你读取到一GET_REPORT 请求，就应将其转发给 HID 设备，并记住负载中的 "id"
  字段。一旦你HID 设备GET_REPORT 作出响应（或失败时），你必须用与请求  完全相同"id" 向内核发送一UHID_GET_REPORT_REPLY。如果请求已经超时，内核  静默忽略该响应id" 字段永远不会被重用，因此不会发生冲突
UHID_SET_REPORT:
  这是UHID_GET_REPORT 对应SET_REPORT 版本。收到后，你应该向你HID 设备
  发送一SET_REPORT 请求。一旦它作出应答，你必须通过 UHID_SET_REPORT_REPLY 通知
  内核  UHID_GET_REPORT 相同的限制同样适用
----------------------------------------------------

Written 2012, David Herrmann <dh.herrmann@gmail.com>
