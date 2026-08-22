## SoundWire 中的音频
音频流是在以下对象之间建立的逻辑或虚拟连接：

  (1) 系统内存缓冲区与 Codec

  (2) DSP 内存缓冲区与 Codec

  (3) FIFO 涓?Codec

  (4) Codec 涓?Codec

通常DMA 通道通过数据链路驱动。一个音频流包含一个或多个数据通道。流中的所有通道必须具有相同的采样率和相同的采样大小
假设通过 SoundWire 接口打开一个具有两个通道（左声道与右声道）的流。以下是流在 SoundWire 中可表示的若干方式
```

	-------------------------
	| L | R | L | R | L | R |
	-------------------------

```
示例 1：由 Master 渲染、包L R 通道的立体声流，渲染方向Master ```

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|               |                                  |       1       |
	|               |                     Data Signal  |               |
	|    L  +  R    +----------------------------------+    L  +  R    |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+


```
示例 2：由 Slave 捕获、包L R 通道的立体声流，捕获方向Slave ```



	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|               |                                  |       1       |
	|               |                     Data Signal  |               |
	|    L  +  R    +----------------------------------+    L  +  R    |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  <-----------------------+       +---------------+



```
示例 3：由 Master 渲染的、包L R 通道的立体声流。L R 通道分别由两个不同的 Slave 接收。Master 与两Slave 之间的关系如
```

	+---------------+                    Clock Signal  +---------------+
	|    Master     +---------+------------------------+     Slave     |
	|   Interface   |         |                        |   Interface   |
	|               |         |                        |       1       |
	|               |         |           Data Signal  |               |
	|    L  +  R    +---+------------------------------+       L       |
	|     (Data)    |   |     |    Data Direction      |     (Data)    |
	+---------------+   |     |   +------------->      +---------------+
	                    |     |
	                    |     |
	                    |     |                        +---------------+
	                    |     +----------------------> |     Slave     |
	                    |                              |   Interface   |
	                    |                              |       2       |
	                    |                              |               |
	                    +----------------------------> |       R       |
	                                                   |     (Data)    |
	                                                   +---------------+

```
示例 4：由 Master 渲染的、包L R 通道的立体声流。L R 通道均由两个不同Slave 接收。Master 与两Slave 均采用单一端口处理
L+R。每Slave 设备通常在本地处L + R 数据，一般基于静态配置或动态方向，并可能驱```

	+---------------+                    Clock Signal  +---------------+
	|    Master     +---------+------------------------+     Slave     |
	|   Interface   |         |                        |   Interface   |
	|               |         |                        |       1       |
	|               |         |           Data Signal  |               |
	|    L  +  R    +---+------------------------------+     L + R     |
	|     (Data)    |   |     |    Data Direction      |     (Data)    |
	+---------------+   |     |   +------------->      +---------------+
	                    |     |
	                    |     |
	                    |     |                        +---------------+
	                    |     +----------------------> |     Slave     |
	                    |                              |   Interface   |
	                    |                              |       2       |
	                    |                              |               |
	                    +----------------------------> |     L + R     |
	                                                   |     (Data)    |
	                                                   +---------------+

```
示例 5：包L R 通道的立体声流由 Master 的两个不同端口渲染，并仅Slave 的单一端口接收
```

	+--------------------+
	|                    |
	|     +--------------+                             +----------------+
	|     |             ||                             |                |
	|     |  Data Port  ||  L Channel                  |                |
	|     |      1      |------------+                 |                |
	|     |  L Channel  ||           |                 +-----+----+     |
	|     |   (Data)    ||           |   L + R Channel ||    Data |     |
	| Master  +----------+           | +---+---------> ||    Port |     |
	| Interface          |           |                 ||     1   |     |
	|     +--------------+           |                 ||         |     |
	|     |             ||           |                 +----------+     |
	|     |  Data Port  |------------+                 |                |
	|     |      2      ||  R Channel                  |     Slave      |
	|     |  R Channel  ||                             |   Interface    |
	|     |   (Data)    ||                             |       1        |
	|     +--------------+         Clock Signal        |     L  +  R    |
	|                    +---------------------------> |      (Data)    |
	+--------------------+                             |                |
							   +----------------+

```
示例 6：包L R 通道的立体声流由 2 Master 渲染，每Master 渲染一个通道，并由两个不同的 Slave 接收，每Slave 接收
```

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|       1       |                                  |       1       |
	|               |                     Data Signal  |               |
	|       L       +----------------------------------+       L       |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|       2       |                                  |       2       |
	|               |                     Data Signal  |               |
	|       R       +----------------------------------+       R       |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+

```
示例 7：包L R 通道的立体声流由 2 Master 渲染，每Master 渲染两个通道。每Slave 接收 L + R。这与示4 的应用相同，只是 Slave 放置```

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|       1       |                                  |       1       |
	|               |                     Data Signal  |               |
	|     L + R     +----------------------------------+     L + R     |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|       2       |                                  |       2       |
	|               |                     Data Signal  |               |
	|     L + R     +----------------------------------+     L + R     |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+

```
示例 8 通道流由 2 Master 渲染，每Master 渲染一```

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|       1       |                                  |       1       |
	|               |                     Data Signal  |               |
	|    L1 + R1    +----------------------------------+    L1 + R1    |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+

	+---------------+                    Clock Signal  +---------------+
	|    Master     +----------------------------------+     Slave     |
	|   Interface   |                                  |   Interface   |
	|       2       |                                  |       2       |
	|               |                     Data Signal  |               |
	|     L2 + R2   +----------------------------------+    L2 + R2    |
	|     (Data)    |     Data Direction               |     (Data)    |
	+---------------+  +----------------------->       +---------------+

```
：在上述的多链路情况下，为了加锁，需要先获取一个全局锁，然后再依次锁定各个总线实例。但在这种情况下，调用方框架（ASoC DPCM）保证对一张声卡上的流操作始终是串行化的。因此不存在竞态条件，也就不需要全局锁
：一Slave 设备可被配置为接收在给定链路上为某个流传输的所有通道（示4），或者仅其中一部分数据（示3）。Slave 设备的配置不SoundWire 子系API 处理，而是snd_soc_dai_set_tdm_slot() API 处理。平台或机器驱动通常会配置使用哪些时隙（slot）。对于示4，所有设备将使用相同的时隙；而对于示3，Slave Device1 将使用例Slot 0，Slave device2 使用 Slot 1
：多Sink 端口可以SoundWire 帧中相同bitSlot 提取相同的信息，但多Source 端口必须配置为不同的 bitSlot。这I2S/PCM TDM 的使用限制相同
## SoundWire 流管理流
### 流定
  (1) 当前流（Current stream）：被归类为需要执prepare、enable、disable、de-prepare 等操作的流
  (2) 活动流（Active stream）：被归类为除当前流之外、已经在总线上处于活动状态的流。总线上可以存在多个活动流
SoundWire 总线管理SoundWire 总线上渲捕获的每个流的操作。本节说明总线对在总线上分释放的每个流所执行的操作。以下是总线为每个音频流维护的流状态
### SoundWire 流状
```

	+-----------+     +------------+     +----------+     +----------+
	| ALLOCATED +---->| CONFIGURED +---->| PREPARED +---->| ENABLED  |
	|   STATE   |     |    STATE   |     |  STATE   |     |  STATE   |
	+-----------+     +------------+     +---+--+---+     +----+-----+
	                                         ^  ^              ^
				                 |  |              |
				               __|  |___________   |
				              |                 |  |
	                                      v                 |  v
	         +----------+           +-----+------+        +-+--+-----+
	         | RELEASED |<----------+ DEPREPARED |<-------+ DISABLED |
	         |  STATE   |           |   STATE    |        |  STATE   |
	         +----------+           +------------+        +----------+

```
注意：仅ALSA/ASoC 层面支持 INFO_PAUSE 标志时，`SDW_STREAM_ENABLED` `SDW_STREAM_DISABLED` 之间的状态转换才相关。同样，`SDW_DISABLED_STATE` `SDW_PREPARED_STATE` 之间的转换取决于 INFO_RESUME 标志
：该框架实现了基本的状态转换检查，但并不会（例如）检查从 DISABLED ENABLED 的转换在特定平台上是否有效。此类测试需要在 ALSA/ASoC 层面添加
### 流状态操
以下小节说明作为流状态转换的一部分，总线Master Slave 上所执行的操作
#### SDW_STREAM_ALLOCATED

流的分配状态。这是流的入口状态。在进入此状态之前执行的操作
  (1) 为流分配一个流运行时（stream runtime）。此流运行时用作对该流执行的所有操作的引用
  (2) 分配并初始化用于保存流运行时信息的资源。其保存所有与流相关的信息，例如流类型（PCM/PDM）及参数、与流关联的 Master Slave 接口、流状态等
上述所有操作成功后，流状态被设置`SDW_STREAM_ALLOCATED`
总线实现了以下用于分配流API，每个流需调用一次。在 ASoC DPCM 框架中，此流状态可能与 .startup() 操作相关联

  int sdw_alloc_stream(char * stream_name, enum sdw_stream_type type);

SoundWire 核心提供了一sdw_startup_stream() 辅助函数，通常dailink .startup() 回调期间调用，用于执行流分配并为连接到某个流的所DAI 设置流指针
#### SDW_STREAM_CONFIGURED

流的配置状态。在进入此状态之前执行的操作
  (1) SDW_STREAM_ALLOCATED 状态中为流信息分配的资源在此处被更新。这包括流参数、与当前流关联的 Master Slave 运行时信息
  (2) 与当前流关联的所Master Slave 向总线提供端口信息，包括由 Master Slave 为当前流分配的端口号及其通道掩码
上述所有操作成功后，流状态被设置`SDW_STREAM_CONFIGURED`
总线实现了以下用CONFIG 状态的 API，需要由与流关联的相Master Slave 调用。这API 只能由相应的 Master Slave 各调用一次。在 ASoC DPCM 框架中，此流状态与 .hw_params() 操作相关联

  int sdw_stream_add_master(struct sdw_bus * bus,
		struct sdw_stream_config * stream_config,
		const struct sdw_ports_config * ports_config,
		struct sdw_stream_runtime * stream);

  int sdw_stream_add_slave(struct sdw_slave * slave,
		struct sdw_stream_config * stream_config,
		const struct sdw_ports_config * ports_config,
		struct sdw_stream_runtime * stream);


#### SDW_STREAM_PREPARED

流的准备状态。在进入此状态之前执行的操作
  (0) 在恢复（resume）操作的情况下省略步1 2，此时总线带宽已知
  (1) 总线参数（如带宽、帧形状、时钟频率）根据当前流以及总线上已有的活动流进行计算。需要重新计算以容纳总线上的当前流
  (2) 所Master Slave 端口的传输（transport）与端口参数，根据步1 计算出的帧形状与时钟频率，针对当前流以及已有活动流进行计算
  (3) 计算出的总线与传输参数被编程Master Slave 的寄存器中。影子寄存器（banked registers）的编程在备bank（当前未使用bank）上进行。已有的活动流的端口在备bank（当前未使用bank）上被启用。这样做是为了不打断已有的活动流
  (4) 一旦所有值被编程，总线发起切换到备bank，所有新编程的值即生效
  (5) 当前流的 Master Slave 端口通过编程 PrepareCtrl 寄存器进行准备
上述所有操作成功后，流状态被设置`SDW_STREAM_PREPARED`
总线实现了以下用PREPARE 状态的 API，每个流需调用一次。在 ASoC DPCM 框架中，此流状态与 .prepare() 操作相关联。由.trigger() 操作可能并不跟随 .prepare()，因此允许从
`SDW_STREAM_PREPARED` 直接转换`SDW_STREAM_DEPREPARED`

  int sdw_prepare_stream(struct sdw_stream_runtime * stream);


#### SDW_STREAM_ENABLED

流的使能状态。数据端口在进入此状态时启用。在进入此状态之前执行的操作
  (1) SDW_STREAM_PREPARED 状态计算出的所有值被编程到备bank（当前未使用bank）。这同样包括已有活动流的编程
  (2) 当前流的所Master Slave 端口通过编程 ChannelEn 寄存器在备用 bank（当前未使用bank）上启用
  (3) 一旦所有值被编程，总线发起切换到备bank，所有新编程的值即生效，并与当前流关联的端口被启用
上述所有操作成功后，流状态被设置`SDW_STREAM_ENABLED`
总线实现了以下用ENABLE 状态的 API，每个流需调用一次。在 ASoC DPCM 框架中，此流状态与 .trigger() start 操作相关联

  int sdw_enable_stream(struct sdw_stream_runtime * stream);

#### SDW_STREAM_DISABLED

流的禁用状态。数据端口在退出此状态时禁用。在进入此状态之前执行的操作
  (1) 当前流的所Master Slave 端口通过编程 ChannelEn 寄存器在备用 bank（当前未使用bank）上禁用
  (2) 总线的所有当前配置以及活动流被编程到备用 bank（当前未使用bank）
  (3) 一旦所有值被编程，总线发起切换到备bank，所有新编程的值即生效，并与当前流关联的端口被禁用
上述所有操作成功后，流状态被设置`SDW_STREAM_DISABLED`
总线实现了以下用DISABLED 状态的 API，每个流需调用一次。在 ASoC DPCM 框架中，此流状态与 .trigger() stop 操作相关联
当支INFO_PAUSE 标志时，允许直接转换`SDW_STREAM_ENABLED`
对于 ASoC 将使.prepare() 回调的恢复操作，流可以从 `SDW_STREAM_DISABLED` 转换`SDW_STREAM_PREPARED`，恢复所有必需设置，但不更新带宽与比特分配

  int sdw_disable_stream(struct sdw_stream_runtime * stream);


#### SDW_STREAM_DEPREPARED

流的去准备状态。在进入此状态之前执行的操作
  (1) 当前流的所Master Slave 端口通过编程 PrepareCtrl 寄存器进行去准备
  (2) 当前流的载荷带宽从总线总带宽需求中扣减，并通过执行 bank 切换等方式计算并应用新参数
上述所有操作成功后，流状态被设置`SDW_STREAM_DEPREPARED`
总线实现了以下用DEPREPARED 状态的 API，每个流需调用一次。ALSA/ASoC 没有“去准备（deprepare）”的概念，因此从此流状态到 ALSA/ASoC 操作的映射可能是实现相关的
当支INFO_PAUSE 标志时，流状态与 .hw_free() 操作相关联——在 TRIGGER_STOP 时不会去准备该流
其他实现可能会在 TRIGGER_STOP 时转换到 `SDW_STREAM_DEPREPARED` 状态，前提是它们需要经`SDW_STREAM_PREPARED` 状态进行转换

  int sdw_deprepare_stream(struct sdw_stream_runtime * stream);


#### SDW_STREAM_RELEASED

流的释放状态。在进入此状态之前执行的操作
  (1) 释放与当前流关联的所Master Slave 端口的端口资源
  (2) 释放与当前流关联Master Slave 运行时资源
  (3) 释放与当前流关联的流运行时资源
上述所有操作成功后，流状态被设置`SDW_STREAM_RELEASED`
总线实现了以下用RELEASE 状态的 API，需要由与流关联的所Master Slave 调用。在 ASoC DPCM 框架中，此流状态与 .hw_free() 操作相关联

  int sdw_stream_remove_master(struct sdw_bus * bus,
		struct sdw_stream_runtime * stream);
  int sdw_stream_remove_slave(struct sdw_slave * slave,
		struct sdw_stream_runtime * stream);


.shutdown() ASoC DPCM 操作调用以下总线 API 来释放作ALLOCATED 状态一部分分配的流
.shutdown() 中，维护流状态的数据结构被释放

  void sdw_release_stream(struct sdw_stream_runtime * stream);

SoundWire 核心提供了一sdw_shutdown_stream() 辅助函数，通常dailink .shutdown() 回调期间调用，用于清除连接到某个流的所DAI 的流指针，并释放为该流分配的内存
## 不支持的情况

1. 具有多个受支持通道的单一端口不能用于两个流之间或跨流使用。例如，一个具4 个通道的端口不能用于处2 个独立的立体声流，即使在理论SoundWire 是可行的