######## 前端统计指标

各数值通过 `dtv_property.stat` 返回。若该属性受支持，则 `dtv_property.stat.len` 大于零

对于大多数传输系统，若支持该统计，`dtv_property.stat.len` 1，且各参数只返回一个单一数值

但需注意，诸ISDB 等新OFDM 传输系统可以为每一组载波使用不同的调制类型。在此类标准下，最多可提供 3 组统计，并且 `dtv_property.stat.len` 会更新以反映“全局”指标，再加上每组载波的一个指标（ISDB 中称为“层”）

因此，为与其他传输系统保持一致，`dtv_property.stat.dtv_stats <dtv_stats>` 数组的第一个元素指代全局指标；数组的其余元素表示各个层，从层 A（索1）、层 B（索2）依次类推

已填充的元素个数保存`dtv_property.stat.len` 中

`dtv_property.stat.dtv_stats` 数组的每个元素由两个部分组成

- `svalue` `uvalue`，其`svalue` 用于带符号的测量值（dB 度量），`uvalue` 用于无符号值（计数器、相对比例）

- `scale` —数值的比例尺度。其取值可为：

  - `FE_SCALE_NOT_AVAILABLE` —前端支持该参数，但无法采集到它（可能是暂时性或永久性的情况）

  - `FE_SCALE_DECIBEL` —参数为带符号值，1/1000 dB 为单位度量

  - `FE_SCALE_RELATIVE` —参数为无符号值，其中 0 表示 0%5535 表示 100%

  - `FE_SCALE_COUNTER` —参数为无符号值，用于统计某事件发生的次数，例如误码、误块或流逝的时间


## DTV_STAT_SIGNAL_STRENGTH

表示调谐器或解调器模拟部分的信号强度水平

此指标可能的比例尺度有：

- `FE_SCALE_NOT_AVAILABLE` —测量失败，或测量尚未完成

- `FE_SCALE_DECIBEL` —信号强度0.001 dBm 为单位，功率以毫瓦度量。该值通常为负数

- `FE_SCALE_RELATIVE` —前端提供 0% 100% 的功率测量（实际0 65535）


## DTV_STAT_CNR

表示主载波的信噪比

此指标可能的比例尺度有：

- `FE_SCALE_NOT_AVAILABLE` —测量失败，或测量尚未完成

- `FE_SCALE_DECIBEL` —信噪比以 0.001 dB 为单位

- `FE_SCALE_RELATIVE` —前端提供 0% 100% 的信噪比测量（实际为 0 65535）


## DTV_STAT_PRE_ERROR_BIT_COUNT

度量前向纠错（FEC）之前、内编码块（Viterbi、LDPC 或其他内码之前）的误码数

该度量在 `DTV_STAT_PRE_TOTAL_BIT_COUNT` 所覆盖的同一时间间隔内获取

为得BER（误码率）测量值，应将其除`DTV_STAT_PRE_TOTAL_BIT_COUNT <DTV-STAT-PRE-TOTAL-BIT-COUNT>`

随着前端获取更多的位计数测量，该测量值单调递增。当调谐到某个频转发器时，前端可能会将其重置

此指标可能的比例尺度有：

- `FE_SCALE_NOT_AVAILABLE` —测量失败，或测量尚未完成

- `FE_SCALE_COUNTER` —内编码之前统计到的误码个数


## DTV_STAT_PRE_TOTAL_BIT_COUNT

度量内编码块之前、在同一周期内接收到的位数，该周期与 `DTV_STAT_PRE_ERROR_BIT_COUNT <DTV-STAT-PRE-ERROR-BIT-COUNT>` 测量所采用的周期相同

需注意，由于前端可能需要手动重启测量，从而在每个测量间隔之间丢失部分数据，因此该测量值可能小于传输流的总位数

随着前端获取更多的位计数测量，该测量值单调递增。当调谐到某个频转发器时，前端可能会将其重置

此指标可能的比例尺度有：

- `FE_SCALE_NOT_AVAILABLE` —测量失败，或测量尚未完成

- `FE_SCALE_COUNTER` —在测`DTV_STAT_PRE_ERROR_BIT_COUNT <DTV-STAT-PRE-ERROR-BIT-COUNT>` 时统计到的位数


## DTV_STAT_POST_ERROR_BIT_COUNT

度量前向纠错（FEC）之后、由内编码块（即 Viterbi、LDPC 或其他内码之后）产生的误码数

该度量在 `DTV_STAT_POST_TOTAL_BIT_COUNT` 所覆盖的同一时间间隔内获取

为得BER（误码率）测量值，应将其除`DTV_STAT_POST_TOTAL_BIT_COUNT <DTV-STAT-POST-TOTAL-BIT-COUNT>`

随着前端获取更多的位计数测量，该测量值单调递增。当调谐到某个频转发器时，前端可能会将其重置

此指标可能的比例尺度有：

- `FE_SCALE_NOT_AVAILABLE` —测量失败，或测量尚未完成

- `FE_SCALE_COUNTER` —内编码之后统计到的误码个数


## DTV_STAT_POST_TOTAL_BIT_COUNT

度量内编码之后、在同一周期内接收到的位数，该周期与 `DTV_STAT_POST_ERROR_BIT_COUNT <DTV-STAT-POST-ERROR-BIT-COUNT>` 测量所采用的周期相同

需注意，由于前端可能需要手动重启测量，从而在每个测量间隔之间丢失部分数据，因此该测量值可能小于传输流的总位数

随着前端获取更多的位计数测量，该测量值单调递增。当调谐到某个频转发器时，前端可能会将其重置

此指标可能的比例尺度有：

- `FE_SCALE_NOT_AVAILABLE` —测量失败，或测量尚未完成

- `FE_SCALE_COUNTER` —在测`DTV_STAT_POST_ERROR_BIT_COUNT <DTV-STAT-POST-ERROR-BIT-COUNT>` 时统计到的位数


## DTV_STAT_ERROR_BLOCK_COUNT

度量外前向纠错编码（Reed-Solomon 或其他外码之后）之后的误块数

随着前端获取更多的位计数测量，该测量值单调递增。当调谐到某个频转发器时，前端可能会将其重置

此指标可能的比例尺度有：

- `FE_SCALE_NOT_AVAILABLE` —测量失败，或测量尚未完成

- `FE_SCALE_COUNTER` —外编码之后统计到的误块个数


## DTV-STAT_TOTAL_BLOCK_COUNT

度量在与 `DTV_STAT_ERROR_BLOCK_COUNT <DTV-STAT-ERROR-BLOCK-COUNT>` 测量相同的周期内所接收到的块总数

可用于计PER 指标，方法是`DTV_STAT_ERROR_BLOCK_COUNT <DTV-STAT-ERROR-BLOCK-COUNT>` 除以 `DTV-STAT-TOTAL-BLOCK-COUNT`

此指标可能的比例尺度有：

- `FE_SCALE_NOT_AVAILABLE` —测量失败，或测量尚未完成

- `FE_SCALE_COUNTER` —在测`DTV_STAT_ERROR_BLOCK_COUNT <DTV-STAT-ERROR-BLOCK-COUNT>` 时统计到的块数
