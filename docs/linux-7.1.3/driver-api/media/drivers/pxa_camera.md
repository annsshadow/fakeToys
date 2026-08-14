## PXA 摄像头主机驱动


作者: Robert Jarzmik <robert.jarzmik@free.fr>

### 约束


a) YUV422P 格式的图像尺寸
   所有 YUV422P 图像都被强制要求 width x height % 16 = 0。
   这是由于 DMA 约束，它只传输 8 字节倍数的平面。

### 全局视频工作流


a) QCI 已停止
   最初，QCI 接口是停止的。
   当一个缓冲区被排队时，调用 start_streaming，QCI 启动。

b) QCI 已启动
   在 QCI 已启动的情况下，可以排队更多缓冲区而不会停止捕获。新缓冲区被“追加”到 DMA 链的尾部，并
   平滑地一帧接一帧捕获。

   一旦一个缓冲区在 QCI 接口中被填满，它会被标记为“DONE”并从活动缓冲区列表中移除。然后它可以由用户空间应用程序重新排队或出队。

   一旦最后一个缓冲区被填满，QCI 接口停止。

c) 捕获全局有限状态机示意


	+----+                             +---+  +----+
	| DQ |                             | Q |  | DQ |
	|    v                             |   v  |    v
	+-----------+                     +------------------------+
	|   STOP    |                     | Wait for capture start |
	+-----------+         Q           +------------------------+
	+-> | QCI: stop | ------------------> | QCI: run               | <------------+
	|   | DMA: stop |                     | DMA: stop              |              |
	|   +-----------+             +-----> +------------------------+              |
	|                            /                            |                   |
	|                           /             +---+  +----+   |                   |
	|capture list empty        /              | Q |  | DQ |   | QCI Irq EOF       |
	|                         /               |   v  |    v   v                   |
	|   +--------------------+             +----------------------+               |
	|   | DMA hotlink missed |             |    Capture running   |               |
	|   +--------------------+             +----------------------+               |
	|   | QCI: run           |     +-----> | QCI: run             | <-+           |
	|   | DMA: stop          |    /        | DMA: run             |   |           |
	|   +--------------------+   /         +----------------------+   | Other     |
	|     ^                     /DMA still            |               | channels  |
	|     | capture list       /  running             | DMA Irq End   | not       |
	|     | not empty         /                       |               | finished  |
	|     |                  /                        v               | yet       |
	|   +----------------------+           +----------------------+   |           |
	|   |  Videobuf released   |           |  Channel completed   |   |           |
	|   +----------------------+           +----------------------+   |           |
	+-- | QCI: run             |           | QCI: run             | --+           |
	| DMA: run             |           | DMA: run             |               |
	+----------------------+           +----------------------+               |
		^                      /           |                           |
		|          no overrun /            | overrun                   |
		|                    /             v                           |
	+--------------------+         /   +----------------------+               |
	|  Frame completed   |        /    |     Frame overran    |               |
	+--------------------+ <-----+     +----------------------+ restart frame |
	| QCI: run           |             | QCI: stop            | --------------+
	| DMA: run           |             | DMA: stop            |
	+--------------------+             +----------------------+

	Legend（图例）: - 每个方框是一个 FSM 状态
  - 每个箭头是转换到另一个状态的条件
  - 带注释的箭头是强制转换（无条件）
  - 箭头 "Q" 表示：一个缓冲区已被入队
  - 箭头 "DQ" 表示：一个缓冲区已被出队
  - "QCI: stop" 表示 QCI 接口未使能
  - "DMA: stop" 表示所有 3 个 DMA 通道都停止
  - "DMA: run" 表示至少有一个 DMA 通道仍在运行

### DMA 使用


a) DMA 流
     - 第一个排队的捕获缓冲区
       一旦第一个缓冲区被排队用于捕获，QCI 启动，但数据传输未启动。在“帧结束（End Of Frame）”中断时，irq 处理程序
       启动 DMA 链。
     - 一个 videobuffer 的捕获
       DMA 链开始将数据传输到 videobuffer 的 RAM 页中。
       当所有页都传输完毕时，在 “ENDINTR” 状态下引发 DMA irq
     - 完成一个 videobuffer
       DMA irq 处理程序将 videobuffer 标记为“done”，并将其从活动运行队列中移除
       同时，下一个 videobuffer（如果有）由 DMA 传输
     - 完成最后一个 videobuffer
       在最后一个 videobuffer 的 DMA irq 上，QCI 停止。

b) 准备好的 DMA 缓冲区将具有如下结构


     +------------+-----+---------------+-----------------+
     | desc-sg[^0^] | ... | desc-sg[last] | finisher/linker |
     +------------+-----+---------------+-----------------+

该结构由 dma->sg_cpu 指向。
描述符的用法如下：

- desc-sg[i]: 第 i 个描述符，将第 i 个 sg 元素传输到视频缓冲区的分散/聚集
- finisher: 具有 ddadr=DADDR_STOP, dcmd=ENDIRQEN
- linker: 具有 ddadr= 下一个视频缓冲区的 desc-sg[^0^]，dcmd=0

对于下一个示意图，假设 d0=desc-sg[^0^] .. dN=desc-sg[N]，
“f” 代表 finisher，“l” 代表 linker。
一个典型的运行链是：


         Videobuffer 1         Videobuffer 2
     +---------+----+---+  +----+----+----+---+
     | d0 | .. | dN | l |  | d0 | .. | dN | f |
     +---------+----+-|-+  ^----+----+----+---+
                      |    |
                      +----+

链接完成后，该链看起来像：


         Videobuffer 1         Videobuffer 2         Videobuffer 3
     +---------+----+---+  +----+----+----+---+  +----+----+----+---+
     | d0 | .. | dN | l |  | d0 | .. | dN | l |  | d0 | .. | dN | f |
     +---------+----+-|-+  ^----+----+----+-|-+  ^----+----+----+---+
                      |    |                |    |
                      +----+                +----+
                                           new_link

c) DMA 热链接（hot chaining）时间片问题

由于 DMA 链接是在 DMA 运行期间完成的，链接可能发生在 DMA 从一个 Videobuffer 跳到另一个时。在示意图上，如果
遇到以下序列，那将是个问题：

- DMA 链是 Videobuffer1 + Videobuffer2
- 调用 pxa_videobuf_queue() 排队 Videobuffer3
- DMA 控制器完成 Videobuffer2，DMA 停止


      =>
         Videobuffer 1         Videobuffer 2
     +---------+----+---+  +----+----+----+---+
     | d0 | .. | dN | l |  | d0 | .. | dN | f |
     +---------+----+-|-+  ^----+----+----+-^-+
                      |    |                |
                      +----+                +-- DMA DDADR 加载 DDADR_STOP

- 调用 pxa_dma_add_tail_buf()，Videobuffer2 的 “finisher” 被
  替换为指向 Videobuffer3 的 “linker”（创建 new_link）
- pxa_videobuf_queue() 结束
- 调用 DMA irq 处理程序，它终止 Videobuffer2
- Videobuffer3 捕获未被安排在 DMA 链上（因为它停止了！！！）


         Videobuffer 1         Videobuffer 2         Videobuffer 3
     +---------+----+---+  +----+----+----+---+  +----+----+----+---+
     | d0 | .. | dN | l |  | d0 | .. | dN | l |  | d0 | .. | dN | f |
     +---------+----+-|-+  ^----+----+----+-|-+  ^----+----+----+---+
                      |    |                |    |
                      +----+                +----+
                                           new_link
                                          DMA DDADR 仍然是 DDADR_STOP

- 调用 pxa_camera_check_link_miss()
  这会检查 DMA 是否已完成且缓冲区仍在 pcdev->capture 列表上。如果是这样，捕获将被重启，
  并且 Videobuffer3 被安排在 DMA 链上。
- DMA irq 处理程序结束


     如果在 pxa_camera_check_link_miss() 读取 DDADR() 值后 DMA 刚好停止，我们就有保证：当 DMA 完成该缓冲区时，
     DMA irq 处理程序会被回调，并且 pxa_camera_check_link_miss() 将被再次调用，以重新安排 Videobuffer3。
