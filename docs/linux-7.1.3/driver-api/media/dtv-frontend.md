
### 数字电视前端 kABI


#### 数字电视前端


数字电视前端 kABI 定义了将底层、与硬件相关的驱动注册到一个与硬件无关的前端层时所需的驱动内部接口。它仅对数字电视设备驱动开发者有意义。此 API 的头文件名为 `dvb_frontend.h`，位于 `include/media/`。

##### 解调器驱动


解调器驱动负责与硬件的解码部分通信。此类驱动应实现 `dvb_frontend_ops`，它说明了支持哪些类型的数字电视标准，并指向一系列函数，使 DVB 核心能够通过 `include/media/dvb_frontend.c` 下的代码控制硬件。

```

	static struct dvb_frontend_ops foo_ops = {
		.delsys = { SYS_DVBT, SYS_DVBT2, SYS_DVBC_ANNEX_A },
		.info = {
			.name	= "foo DVB-T/T2/C driver",
			.caps = FE_CAN_FEC_1_2 |
				FE_CAN_FEC_2_3 |
				FE_CAN_FEC_3_4 |
				FE_CAN_FEC_5_6 |
				FE_CAN_FEC_7_8 |
				FE_CAN_FEC_AUTO |
				FE_CAN_QPSK |
				FE_CAN_QAM_16 |
				FE_CAN_QAM_32 |
				FE_CAN_QAM_64 |
				FE_CAN_QAM_128 |
				FE_CAN_QAM_256 |
				FE_CAN_QAM_AUTO |
				FE_CAN_TRANSMISSION_MODE_AUTO |
				FE_CAN_GUARD_INTERVAL_AUTO |
				FE_CAN_HIERARCHY_AUTO |
				FE_CAN_MUTE_TS |
				FE_CAN_2G_MODULATION,
			.frequency_min = 42000000, /* Hz */
			.frequency_max = 1002000000, /* Hz */
			.symbol_rate_min = 870000,
			.symbol_rate_max = 11700000
		},
		.init = foo_init,
		.sleep = foo_sleep,
		.release = foo_release,
		.set_frontend = foo_set_frontend,
		.get_frontend = foo_get_frontend,
		.read_status = foo_get_status_and_stats,
		.tune = foo_tune,
		.i2c_gate_ctrl = foo_i2c_gate_ctrl,
		.get_frontend_algo = foo_get_algo,
	};

```
下面是名为 `bar` 的驱动中此类结构体的典型示例，它用于
```

	static const struct dvb_frontend_ops bar_ops = {
		.delsys = { SYS_DVBS, SYS_DVBS2 },
		.info = {
			.name		= "Bar DVB-S/S2 demodulator",
			.frequency_min	= 500000, /* KHz */
			.frequency_max	= 2500000, /* KHz */
			.frequency_stepsize	= 0,
			.symbol_rate_min = 1000000,
			.symbol_rate_max = 45000000,
			.symbol_rate_tolerance = 500,
			.caps = FE_CAN_INVERSION_AUTO |
				FE_CAN_FEC_AUTO |
				FE_CAN_QPSK,
		},
		.init = bar_init,
		.sleep = bar_sleep,
		.release = bar_release,
		.set_frontend = bar_set_frontend,
		.get_frontend = bar_get_frontend,
		.read_status = bar_get_status_and_stats,
		.i2c_gate_ctrl = bar_i2c_gate_ctrl,
		.get_frontend_algo = bar_get_algo,
		.tune = bar_tune,

		/* Satellite-specific */
		.diseqc_send_master_cmd = bar_send_diseqc_msg,
		.diseqc_send_burst = bar_send_burst,
		.set_tone = bar_set_tone,
		.set_voltage = bar_set_voltage,
	};

```

   #) 对于卫星数字电视标准（DVB-S、DVB-S2、ISDB-S），频率以 kHz 为单位指定；而对于地面和有线标准，则以 Hz 为单位。因此，如果同一个前端同时支持两类标准，就需要有两套独立的 `dvb_frontend_ops` 结构体，每种标准各一套。
   #) `.i2c_gate_ctrl` 字段仅当硬件支持控制 I2C 门控（直接控制或通过某个 GPIO 引脚）时才存在，以便在某个频道调谐完成后将调谐器从 I2C 总线上移除。
   #) 所有新驱动都应通过 `.read_status` 实现 DVBv5 统计信息 <dvbv5_stats>。不过，仍存在一些用于获取信号强度、S/N 和 UCB 统计信息的回调函数。它们是为了向后兼容那些不支持 DVBv5 API 的旧应用程序而保留的。实现这些回调是可选的。当所有现有驱动都支持 DVBv5 统计信息后，这些回调将来可能会被移除。
   #) 对于卫星电视标准，还需要其他回调来控制 LNBf 和 DiSEqC：`.diseqc_send_master_cmd`、`.diseqc_send_burst`、`.set_tone`、`.set_voltage`。


`include/media/dvb_frontend.c` 中有一个内核线程，负责调谐设备。它支持多种用于检测频道的算法，定义于枚举 `dvbfe_algo`。

所使用的算法通过 `.get_frontend_algo` 获取。如果驱动没有在 struct dvb_frontend_ops 中填写该字段，则默认使用 `DVBFE_ALGO_SW`，意味着 dvb-core 在调谐时会执行“之字形”搜索，例如它先尝试使用指定的中心频率 `f`，然后依次尝试 `f` + |delta|、`f` - |delta|、`f` + 2×|delta|、`f` - 2×|delta|，依此类推。

如果硬件内部自带某种之字形算法，则应定义一个返回 `DVBFE_ALGO_HW` 的 `.get_frontend_algo` 函数。


   核心前端支持还提供了第三种类型（`DVBFE_ALGO_CUSTOM`），以允许驱动定义自己的硬件辅助算法。如今几乎不需要使用它。使用 `DVBFE_ALGO_CUSTOM` 需要在 struct dvb_frontend_ops 中提供其他函数回调。

##### 将前端驱动挂接到桥接驱动


在使用数字电视前端核心之前，桥接驱动应先挂接前端解调器、调谐器和 SEC 设备，并调用
`dvb_register_frontend()`，
以便向子系统注册新的前端。在设备分离/移除时，桥接驱动应调用
`dvb_unregister_frontend()` 将前端从核心中移除，然后再调用 `dvb_frontend_detach()`
释放前端驱动分配的内存。

驱动还应将 `dvb_frontend_suspend()` 作为其 `device_driver` 的 `suspend()` 处理函数的一部分来调用，并将 `dvb_frontend_resume()` 作为其 `device_driver` 的 `resume()` 处理函数的一部分来调用。

还提供了一些其他的可选函数，用于处理某些特殊情况。


#### 数字电视前端统计信息


##### 简介


数字电视前端提供一系列统计信息 <frontend-stat-properties>，用于辅助调谐设备并衡量服务质量。

对于每次统计测量，驱动应设置所使用的刻度类型；如果在某个时刻统计信息不可用，则设置为 `FE_SCALE_NOT_AVAILABLE`。驱动还应提供每种类型的统计量个数，对于大多数视频标准而言通常为 1 [#f2]_。

驱动应在其初始化代码中，以长度和刻度初始化每个统计计数器。例如，如果前端提供信号
```

	struct dtv_frontend_properties *c = &state->fe.dtv_property_cache;

	c->strength.len = 1;
	c->strength.stat[0].scale = FE_SCALE_NOT_AVAILABLE;

```
```

	c->strength.stat[0].scale = FE_SCALE_DECIBEL;
	c->strength.stat[0].uvalue = strength;

```
   （统计集合）。在这种情况下，len 应等于 4。第一个值对应全局统计；其余的对应各个层，例如：

   - c->cnr.stat[^0^] 对应全局信噪比（S/N）载噪比，
   - c->cnr.stat[^1^] 对应层 A 的 S/N 载噪比，
   - c->cnr.stat[^2^] 对应层 B 的 S/N 载噪比，
   - c->cnr.stat[^3^] 对应层 C 的 S/N 载噪比。

   对于信号强度和 CNR 测量，使用 `FE_SCALE_RELATIVE`。

##### 统计信息分组


当前支持以下几组统计信息：

信号强度（DTV-STAT-SIGNAL-STRENGTH）
  - 测量调谐器或解调器模拟部分的信号强度电平。

  - 通常来自为检测载波而施加到调谐器和/或前端的增益。当未检测到载波时，增益处于最大值（因此强度处于最小值）。

  - 由于增益可通过调整增益的寄存器组观察到，通常该统计信息始终可用 [#f3]_。

  - 驱动应尽量使其始终可用，因为这些统计信息可用于调整天线方位以及检查线缆连接问题。

  .. [#f3] 在少数设备上，若无载波，增益会持续浮动。在此类设备上，强度报告应先检查调谐器是否检测到载波（`FE_HAS_CARRIER`，参见 `fe_status`），否则返回尽可能最低的值。

载波信噪比（DTV-STAT-CNR）
  - 主载波的信噪比。

  - 信噪比测量取决于设备。在某些硬件上，主载波被检测到时即可获得。在此类硬件上，CNR 测量通常来自调谐器（例如 `FE_HAS_CARRIER` 之后，参见 `fe_status`）。

    在其他设备上，它需要内部 FEC 解码，因为前端是从其他参数间接测量的（例如 `FE_HAS_VITERBI` 之后，参见 `fe_status`）。

    在内层 FEC 之后即可获得更为常见。

FEC 之后的比特计数（DTV-STAT-POST-ERROR-BIT-COUNT 和 DTV-STAT-POST-TOTAL-BIT-COUNT）
  - 这些计数器测量内层编码块上前向纠错（FEC）之后的比特数与比特错误数（在 Viterbi、LDPC 或其他内层编码之后）。

  - 由于其特性，这些统计信息依赖于完整的编码锁定（例如 `FE_HAS_SYNC` 之后或 `FE_HAS_LOCK` 之后，参见 `fe_status`）。

FEC 之前的比特计数（DTV-STAT-PRE-ERROR-BIT-COUNT 和 DTV-STAT-PRE-TOTAL-BIT-COUNT）
  - 这些计数器测量内层编码块上前向纠错（FEC）之前的比特数与比特错误数（在 Viterbi、LDPC 或其他内层编码之前）。

  - 并非所有前端都提供此类统计信息。

  - 由于其特性，这些统计信息依赖于内层编码锁定（例如 `FE_HAS_VITERBI` 之后，参见 `fe_status`）。

块计数（DTV-STAT-ERROR-BLOCK-COUNT 和 DTV-STAT-TOTAL-BLOCK-COUNT）
  - 这些计数器测量内层编码块上前向纠错（FEC）之后的块数与块错误数（在 Viterbi、LDPC 或其他内层编码之前）。

  - 由于其特性，这些统计信息依赖于完整的编码锁定（例如 `FE_HAS_SYNC` 之后或
    `FE_HAS_LOCK`，参见 `fe_status`）。

   - 从硬件采集而来。

```

	static int foo_get_status_and_stats(struct dvb_frontend *fe)
	{
		struct foo_state *state = fe->demodulator_priv;
		struct dtv_frontend_properties *c = &fe->dtv_property_cache;

		int rc;
		enum fe_status *status;

		/* Both status and strength are always available */
		rc = foo_read_status(fe, &status);
		if (rc < 0)
			return rc;

		rc = foo_read_strength(fe);
		if (rc < 0)
			return rc;

		/* Check if CNR is available */
		if (!(fe->status & FE_HAS_CARRIER))
			return 0;

		rc = foo_read_cnr(fe);
		if (rc < 0)
			return rc;

		/* Check if pre-BER stats are available */
		if (!(fe->status & FE_HAS_VITERBI))
			return 0;

		rc = foo_get_pre_ber(fe);
		if (rc < 0)
			return rc;

		/* Check if post-BER stats are available */
		if (!(fe->status & FE_HAS_SYNC))
			return 0;

		rc = foo_get_post_ber(fe);
		if (rc < 0)
			return rc;
	}

	static const struct dvb_frontend_ops ops = {
		/* ... */
		.read_status = foo_get_status_and_stats,
	};

```
##### 统计信息采集


在几乎所有前端硬件上，比特和字节计数会由硬件在一段特定时间之后、或总比特/块计数器达到某个值（通常可编程）之后进行存储，例如每 1000 ms 一次，或在接收到 1,000,000 比特之后。

因此，如果读取寄存器过早，最终会读到与上一次相同的值，导致单调值被过于频繁地累加。

驱动应负责避免过于频繁的读取。这可以通过以下两种方式实现：

如果驱动有一个指示采集数据何时就绪的位
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

驱动应在统计信息可用之前检查该位。

此类行为的示例可在以下代码片段（改编自
```

	static int foo_get_pre_ber(struct dvb_frontend *fe)
	{
		struct foo_state *state = fe->demodulator_priv;
		struct dtv_frontend_properties *c = &fe->dtv_property_cache;
		int rc, bit_error;

		/* Check if the BER measures are already available */
		rc = foo_read_u8(state, 0x54);
		if (rc < 0)
			return rc;

		if (!rc)
			return 0;

		/* Read Bit Error Count */
		bit_error = foo_read_u32(state, 0x55);
		if (bit_error < 0)
			return bit_error;

		/* Read Total Bit Count */
		rc = foo_read_u32(state, 0x51);
		if (rc < 0)
			return rc;

		c->pre_bit_error.stat[0].scale = FE_SCALE_COUNTER;
		c->pre_bit_error.stat[0].uvalue += bit_error;
		c->pre_bit_count.stat[0].scale = FE_SCALE_COUNTER;
		c->pre_bit_count.stat[0].uvalue += rc;

		return 0;
	}

```
如果驱动没有提供“统计可用”检查位
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

然而，少数设备可能不提供检查统计是否可用的方式（或检查方式未知）。它们甚至可能不提供直接读取总比特数或总块数的方式。

在此类设备上，驱动需要确保不会过于频繁地从寄存器读取，和/或估算总比特数/块数。

在此类驱动上，获取统计信息的典型例程类似于
```

	struct foo_state {
		/* ... */

		unsigned long per_jiffies_stats;
	}

	static int foo_get_pre_ber(struct dvb_frontend *fe)
	{
		struct foo_state *state = fe->demodulator_priv;
		struct dtv_frontend_properties *c = &fe->dtv_property_cache;
		int rc, bit_error;
		u64 bits;

		/* Check if time for stats was elapsed */
		if (!time_after(jiffies, state->per_jiffies_stats))
			return 0;

		/* Next stat should be collected in 1000 ms */
		state->per_jiffies_stats = jiffies + msecs_to_jiffies(1000);

		/* Read Bit Error Count */
		bit_error = foo_read_u32(state, 0x55);
		if (bit_error < 0)
			return bit_error;

		/*
		 * On this particular frontend, there's no register that
		 * would provide the number of bits per 1000ms sample. So,
		 * some function would calculate it based on DTV properties
		 */
		bits = get_number_of_bits_per_1000ms(fe);

		c->pre_bit_error.stat[0].scale = FE_SCALE_COUNTER;
		c->pre_bit_error.stat[0].uvalue += bit_error;
		c->pre_bit_count.stat[0].scale = FE_SCALE_COUNTER;
		c->pre_bit_count.stat[0].uvalue += bits;

		return 0;
	}

```
请注意，在这两种情况下，我们都是使用 `dvb_frontend_ops` 的 `.read_status` 回调来获取统计信息的。其原因是，前端核心会自动周期性地调用该函数（通常当前端锁定时每秒 3 次）。

这保证了我们不会错过采集某个计数器并在正确时间累加单调统计值。

#### 数字电视前端函数与类型


