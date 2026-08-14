


######## Digital TV property parameters


有若干不同的数字电视（Digital TV）参数可供 `FE_SET_PROPERTY` 与
`FE_GET_PROPERTY` ioctl<FE_GET_PROPERTY> 使用。本节将逐一描述它们。但请注意，
设置前端（frontend）时只需用到其中的一个子集。



## DTV_UNDEFINED


内部使用。对其执行 GET/SET 操作不会改变或返回任何内容。



## DTV_TUNE


解析数据缓存（cache），构建一条传统的 frontend 调谐请求（tunerequest），以便
能够通过 `FE_SET_FRONTEND` ioctl 的校验。



## DTV_CLEAR


在此处重置专属于该 frontend 的数据缓存（cache）。这不会影响硬件。



## DTV_FREQUENCY


数字电视转发器（transponder）/频道（channel）的频率。


  #. 对于卫星传输系统，频率单位为 kHz。

  #. 对于有线电视（cable）和地面（terrestrial）传输系统，频率单位为 Hz。

  #. 在大多数传输系统中，频率为转发器/频道的中心频率。ISDB-T 例外，其主载波相对
     中心有 1/7 的偏移。

  #. 对于 ISDB-T，频道通常带有约 143kHz 的偏移进行传输。例如，一个有效的频率可以
     是 474,143 kHz。步进（stepping）与频道带宽相关，通常为 6MHz。

  #. 在 ISDB-Tsb 中，频道仅由一段或三段组成，频率步进分别为 429kHz、3*429。



## DTV_MODULATION


为支持多种调制（modulation）的传输系统指定 frontend 的调制类型。

调制（modulation）可以是枚举 `fe_modulation` 所定义的类型之一。

大多数数字电视标准都提供多于一种可能的调制类型。

下表汇总了当前规范中定义的各传输系统所支持的调制类型。

======================= =======================================================
Standard		Modulation types
======================= =======================================================
ATSC (version 1)	8-VSB and 16-VSB.
DMTB			4-QAM, 16-QAM, 32-QAM, 64-QAM and 4-QAM-NR.
DVB-C Annex A/C		16-QAM, 32-QAM, 64-QAM and 256-QAM.
DVB-C Annex B		64-QAM.
DVB-C2			QPSK, 16-QAM, 64-QAM, 256-QAM, 1024-QAM and 4096-QAM.
DVB-T			QPSK, 16-QAM and 64-QAM.
DVB-T2			QPSK, 16-QAM, 64-QAM and 256-QAM.
DVB-S			No need to set. It supports only QPSK.
DVB-S2			QPSK, 8-PSK, 16-APSK and 32-APSK.
DVB-S2X			8-APSK-L, 16-APSK-L, 32-APSK-L, 64-APSK and 64-APSK-L.
ISDB-T			QPSK, DQPSK, 16-QAM and 64-QAM.
ISDB-S			8-PSK, QPSK and BPSK.
======================= =======================================================


   由于 DVB-S2X 是对 DVB-S2 标准的扩展，使用了相同的传输系统枚举值（SYS_DVBS2）。

   请注意，上述某些调制类型当前可能尚未在内核（Kernel）中定义。原因很简单：尚没有
   驱动需要这样的定义。



## DTV_BANDWIDTH_HZ


频道的带宽，单位为 Hz。

仅应在地面传输系统中设置。

可能的值：`1712000`、`5000000`、`6000000`、`7000000`、
`8000000`、`10000000`。

======================= =======================================================
Terrestrial Standard	Possible values for bandwidth
======================= =======================================================
ATSC (version 1)	No need to set. It is always 6MHz.
DMTB			No need to set. It is always 8MHz.
DVB-T			6MHz, 7MHz and 8MHz.
DVB-T2			1.172 MHz, 5MHz, 6MHz, 7MHz, 8MHz and 10MHz
ISDB-T			5MHz, 6MHz, 7MHz and 8MHz, although most places
			use 6MHz.
======================= =======================================================




  #. 对于 ISDB-Tsb，带宽会随所连接段（segment）的数量而变化。

     它可由其他参数（DTV_ISDBT_SB_SEGMENT_IDX、DTV_ISDBT_SB_SEGMENT_COUNT）
     轻易推导得出。

  #. 在卫星和有线电视传输系统中，带宽取决于符号率（symbol rate）。内核会静默忽略任何
     DTV-BANDWIDTH-HZ 设置，并用带宽估算值覆盖它。

     该带宽估算会考虑由 DTV-SYMBOL-RATE 设置的符号率，以及滚降（rolloff）因子
     （对于 DVB-C 和 DVB-S 为固定值）。

     对于 DVB-S2，滚降还应通过 DTV-ROLLOFF 设置。



## DTV_INVERSION


指定 frontend 是否应进行频谱反转（spectral inversion）。

可接受的值由 `fe_spectral_inversion` 定义。



## DTV_DISEQC_MASTER


当前未实现。



## DTV_SYMBOL_RATE


用于有线电视和卫星传输系统。

数字电视符号率（symbol rate），单位为波特（bauds，即符号/秒）。



## DTV_INNER_FEC


用于有线电视和卫星传输系统。

可接受的值由 `fe_code_rate` 定义。



## DTV_VOLTAGE


用于卫星传输系统。

电压通常用于不具备 DiSEqC 能力的 LNB，以切换极化方式（水平/垂直）。使用 DiSEqC
设备时，该电压必须随 DiSEqC 命令一致地切换，如 DiSEqC 规范中所述。

可接受的值由 `fe_sec_voltage` 定义。



## DTV_TONE


当前未使用。



## DTV_PILOT


用于 DVB-S2。

设置 DVB-S2 的导频（pilot）。

可接受的值由 `fe_pilot` 定义。



## DTV_ROLLOFF


用于 DVB-S2。

设置 DVB-S2 的滚降（rolloff）。

可接受的值由 `fe_rolloff` 定义。



## DTV_DISEQC_SLAVE_REPLY


当前未实现。



## DTV_FE_CAPABILITY_COUNT


当前未实现。



## DTV_FE_CAPABILITY


当前未实现。



## DTV_DELIVERY_SYSTEM


指定传输系统（delivery system）的类型。

可接受的值由 `fe_delivery_system` 定义。



## DTV_ISDBT_PARTIAL_RECEPTION


仅用于 ISDB。

如果 `DTV_ISDBT_SOUND_BROADCASTING` 为 '0'，该位字段表示频道是否处于部分接收
（partial reception）模式。

如果为 '1'，则 `DTV_ISDBT_LAYERA_*` 的值被分配给中心段（center segment），且
`DTV_ISDBT_LAYERA_SEGMENT_COUNT` 必须为 '1'。

如果 `DTV_ISDBT_SOUND_BROADCASTING` 还为 '1'，则 `DTV_ISDBT_PARTIAL_RECEPTION`
表示该 ISDB-Tsb 频道是由一段一层还是三段两层组成。

可能的值：0、1、-1（AUTO）



## DTV_ISDBT_SOUND_BROADCASTING


仅用于 ISDB。

该字段表示其他 DTV_ISDBT_*-参数所指的是一个 ISDB-T 频道还是一个 ISDB-Tsb 频道。
（另见 `DTV_ISDBT_PARTIAL_RECEPTION`）。

可能的值：0、1、-1（AUTO）



## DTV_ISDBT_SB_SUBCHANNEL_ID


仅用于 ISDB。

该字段仅当 `DTV_ISDBT_SOUND_BROADCASTING` 为 '1' 时适用。

（作者注：这可能并非对 `SUBCHANNEL-ID` 全部细节的准确描述，但它是我对编程设备
所需技术背景的理解）

一个 ISDB-Tsb 频道（1 段或 3 段）可以单独广播，也可以成组（set）地与其他相连的
ISDB-Tsb 频道一起广播。在这组频道中，每个频道都可以独立接收。相连的 ISDB-Tsb
段的数量可以变化，例如取决于可用的频谱带宽。

示例：假设广播了 8 个相连的 ISDB-Tsb 段。广播方有多种方式将这些频道送上空中：
假设一个普通的 13 段 ISDB-T 频谱，他可以将这 8 段从位置 1-8 对齐到 5-13，或介于
两者之间的任何位置。

段（segment）的下层是子信道（sub-channel）：每个段由若干具有预定义 ID 的子信道
组成。子信道用于帮助解调器（demodulator）与频道同步。

一个 ISDB-T 频道总是以所有子信道为中心对齐。如上面示例所述，在 ISDB-Tsb 中就不再
如此简单了。

`DTV_ISDBT_SB_SUBCHANNEL_ID` 参数用于给出待解调的段的子信道 ID。

可能的值：0 .. 41、-1（AUTO）



## DTV_ISDBT_SB_SEGMENT_IDX


仅用于 ISDB。

该字段仅当 `DTV_ISDBT_SOUND_BROADCASTING` 为 '1' 时适用。

`DTV_ISDBT_SB_SEGMENT_IDX` 给出待解调段的索引，用于多个 ISDB-Tsb 频道以相连方式
传输的情况。

可能的值：0 .. `DTV_ISDBT_SB_SEGMENT_COUNT` - 1

注意：该值无法由自动频道搜索确定。



## DTV_ISDBT_SB_SEGMENT_COUNT


仅用于 ISDB。

该字段仅当 `DTV_ISDBT_SOUND_BROADCASTING` 为 '1' 时适用。

`DTV_ISDBT_SB_SEGMENT_COUNT` 给出相连的 ISDB-Tsb 频道的总数。

可能的值：1 .. 13

注意：该值无法由自动频道搜索确定。



## DTV-ISDBT-LAYER[A-C] parameters


仅用于 ISDB。

ISDB-T 频道可以采用分层（hierarchical）编码。与 DVB-T 不同，ISDB-T 中的分层可以
同时解码。因此一个 ISDB-T 解调器拥有 3 个 Viterbi 和 3 个 Reed-Solomon 解码器。

ISDB-T 有 3 个分层，每一层可以使用可用段的一部分。所有层的总段数在 ISDB-T 中
必须为 13。

共有 3 组参数，分别用于层 A、B 和 C。



### DTV_ISDBT_LAYER_ENABLED


仅用于 ISDB。

ISDB-T 中的分层接收（hierarchical reception）通过解码过程中启用或禁用各层来实现。
将 `DTV_ISDBT_LAYER_ENABLED` 的所有位设为 '1' 会强制解调所有层（如适用）。这是
默认行为。

如果频道处于部分接收模式（`DTV_ISDBT_PARTIAL_RECEPTION` = 1），中心段可以独立于
其他 12 段被解码。在该模式下，层 A 的 `SEGMENT_COUNT` 必须为 1。

在 ISDB-Tsb 中仅使用层 A，根据 `DTV_ISDBT_PARTIAL_RECEPTION`，它可以是 1 或 3。
`SEGMENT_COUNT` 必须相应地填写。

仅使用前 3 位的值。其他位将被静默忽略：

`DTV_ISDBT_LAYER_ENABLED` 位 0：启用层 A

`DTV_ISDBT_LAYER_ENABLED` 位 1：启用层 B

`DTV_ISDBT_LAYER_ENABLED` 位 2：启用层 C

`DTV_ISDBT_LAYER_ENABLED` 位 3-31：未使用



### DTV_ISDBT_LAYER[A-C]_FEC


仅用于 ISDB。

给定 ISDB 层所使用的前向纠错（Forward Error Correction）机制，由 `fe_code_rate`
定义。

可能的值为：`FEC_AUTO`、`FEC_1_2`、`FEC_2_3`、`FEC_3_4`、
`FEC_5_6`、`FEC_7_8`



### DTV_ISDBT_LAYER[A-C]_MODULATION


仅用于 ISDB。

给定 ISDB 层所使用的调制（modulation），由 `fe_modulation` 定义。

可能的值为：`QAM_AUTO`、`QPSK`、`QAM_16`、`QAM_64`、`DQPSK`


   #. 如果层 C 为 `DQPSK`，则层 B 必须为 `DQPSK`。

   #. 如果层 B 为 `DQPSK` 且 `DTV_ISDBT_PARTIAL_RECEPTION` = 0，则层必须为
      `DQPSK`。



### DTV_ISDBT_LAYER[A-C]_SEGMENT_COUNT


仅用于 ISDB。

可能的值：0、1、2、3、4、5、6、7、8、9、10、11、12、13、-1（AUTO）

注意：`DTV_ISDBT_SOUND_BROADCASTING`、`DTV_ISDBT_PARTIAL_RECEPTION` 与
`LAYER[A-C]_SEGMENT_COUNT` 的真值表（truth table）


    :header-rows:  1
    :stub-columns: 0


    - .. row 1

       - Partial Reception

       - Sound Broadcasting

       - Layer A width

       - Layer B width

       - Layer C width

       - total width

    - .. row 2

       - 0

       - 0

       - 1 .. 13

       - 1 .. 13

       - 1 .. 13

       - 13

    - .. row 3

       - 1

       - 0

       - 1

       - 1 .. 13

       - 1 .. 13

       - 13

    - .. row 4

       - 0

       - 1

       - 1

       - 0

       - 0

       - 1

    - .. row 5

       - 1

       - 1

       - 1

       - 2

       - 0

       - 13



### DTV_ISDBT_LAYER[A-C]_TIME_INTERLEAVING


仅用于 ISDB。

有效值：0、1、2、4、-1（AUTO）

当 DTV_ISDBT_SOUND_BROADCASTING 处于激活状态时，值 8 也是有效的。

注意：实际的时域交织（time interleaving）长度取决于模式（fft 大小）。此处的值
指的是可在 TMCC 结构中找到的内容，如下表所示。


    :header-rows:  1
    :stub-columns: 0


    - .. row 1

       - `DTV_ISDBT_LAYER[A-C]_TIME_INTERLEAVING`

       - Mode 1 (2K FFT)

       - Mode 2 (4K FFT)

       - Mode 3 (8K FFT)

    - .. row 2

       - 0

       - 0

       - 0

       - 0

    - .. row 3

       - 1

       - 4

       - 2

       - 1

    - .. row 4

       - 2

       - 8

       - 4

       - 2

    - .. row 5

       - 4

       - 16

       - 8

       - 4



### DTV_ATSCMH_FIC_VER


仅用于 ATSC-MH。

FIC（Fast Information Channel，快速信息信道）信令数据的版本号。

FIC 用于传递信息，以便接收端快速获取服务。

可能的值：0、1、2、3、...、30、31



### DTV_ATSCMH_PARADE_ID


仅用于 ATSC-MH。

Parade 标识号（parade identification number）。

一个 parade 是至多 8 个 MH 组（group）的集合，承载一个或两个 ensemble。

可能的值：0、1、2、3、...、126、127



### DTV_ATSCMH_NOG


仅用于 ATSC-MH。

指定 parade 中每个 MH 子帧（subframe）的 MH 组（group）数量。

可能的值：1、2、3、4、5、6、7、8



### DTV_ATSCMH_TNOG


仅用于 ATSC-MH。

MH 组的总数，包含属于一个 MH 子帧中所有 MH parade 的全部 MH 组。

可能的值：0、1、2、3、...、30、31



### DTV_ATSCMH_SGN


仅用于 ATSC-MH。

起始组号（start group number）。

可能的值：0、1、2、3、...、14、15



### DTV_ATSCMH_PRC


仅用于 ATSC-MH。

Parade 重复周期（parade repetition cycle）。

可能的值：1、2、3、4、5、6、7、8



### DTV_ATSCMH_RS_FRAME_MODE


仅用于 ATSC-MH。

Reed Solomon（RS）帧模式（frame mode）。

可接受的值由 `atscmh_rs_frame_mode` 定义。



### DTV_ATSCMH_RS_FRAME_ENSEMBLE


仅用于 ATSC-MH。

Reed Solomon（RS）帧 ensemble。

可接受的值由 `atscmh_rs_frame_ensemble` 定义。



### DTV_ATSCMH_RS_CODE_MODE_PRI


仅用于 ATSC-MH。

Reed Solomon（RS）编码模式（code mode，主）。

可接受的值由 `atscmh_rs_code_mode` 定义。



### DTV_ATSCMH_RS_CODE_MODE_SEC


仅用于 ATSC-MH。

Reed Solomon（RS）编码模式（code mode，次）。

可接受的值由 `atscmh_rs_code_mode` 定义。



### DTV_ATSCMH_SCCC_BLOCK_MODE


仅用于 ATSC-MH。

串接卷积码块模式（Series Concatenated Convolutional Code Block Mode）。

可接受的值由 `atscmh_sccc_block_mode` 定义。



### DTV_ATSCMH_SCCC_CODE_MODE_A


仅用于 ATSC-MH。

串接卷积码率（Series Concatenated Convolutional Code Rate）。

可接受的值由 `atscmh_sccc_code_mode` 定义。



### DTV_ATSCMH_SCCC_CODE_MODE_B


仅用于 ATSC-MH。

串接卷积码率（Series Concatenated Convolutional Code Rate）。

可能的值与枚举 `atscmh_sccc_code_mode` 中记录的值相同。



### DTV_ATSCMH_SCCC_CODE_MODE_C


仅用于 ATSC-MH。

串接卷积码率（Series Concatenated Convolutional Code Rate）。

可能的值与枚举 `atscmh_sccc_code_mode` 中记录的值相同。



### DTV_ATSCMH_SCCC_CODE_MODE_D


仅用于 ATSC-MH。

串接卷积码率（Series Concatenated Convolutional Code Rate）。

可能的值与枚举 `atscmh_sccc_code_mode` 中记录的值相同。



## DTV_API_VERSION


返回数字电视 API 的主/次版本号。



## DTV_CODE_RATE_HP


用于地面传输。

可接受的值由 `fe_transmit_mode` 定义。



## DTV_CODE_RATE_LP


用于地面传输。

可接受的值由 `fe_transmit_mode` 定义。



## DTV_GUARD_INTERVAL


可接受的值由 `fe_guard_interval` 定义。


   #. 如果 `DTV_GUARD_INTERVAL` 设置为 `GUARD_INTERVAL_AUTO`，硬件将尝试找到正确的
      保护间隔（guard interval）（若支持），并使用 TMCC 填充缺失的参数。
   #. 间隔 `GUARD_INTERVAL_1_64` 仅用于 DVB-C2。
   #. 间隔 `GUARD_INTERVAL_1_128` 同时用于 DVB-C2 和 DVB_T2。
   #. 间隔 `GUARD_INTERVAL_19_128` 与 `GUARD_INTERVAL_19_256` 仅用于 DVB-T2。
   #. 间隔 `GUARD_INTERVAL_PN420`、`GUARD_INTERVAL_PN595` 与
      `GUARD_INTERVAL_PN945` 当前仅用于 DMTB。在该标准下，仅这些间隔与
      `GUARD_INTERVAL_AUTO` 是有效的。


## DTV_TRANSMISSION_MODE



仅用于基于 OFDM 的标准，例如 DVB-T/T2、ISDB-T、DTMB。

指定该标准所使用的 FFT 大小（对应于载波的近似数量）。

可接受的值由 `fe_transmit_mode` 定义。


   #. ISDB-T 支持三种载波/符号大小：8K、4K、2K。在该标准中称为**模式（mode）**，
      并从 1 到 3 编号：

      ====	========	========================
      Mode	FFT size	Transmission mode
      ====	========	========================
      1		2K		`TRANSMISSION_MODE_2K`
      2		4K		`TRANSMISSION_MODE_4K`
      3		8K		`TRANSMISSION_MODE_8K`
      ====	========	========================

   #. 如果 `DTV_TRANSMISSION_MODE` 设置为 `TRANSMISSION_MODE_AUTO`，硬件将尝试找到
      正确的 FFT 大小（若支持），并使用 TMCC 填充缺失的参数。

   #. DVB-T 规定 2K 和 8K 为有效大小。

   #. DVB-T2 规定 1K、2K、4K、8K、16K 和 32K。

   #. DTMB 规定 C1 和 C3780。



## DTV_HIERARCHY


仅用于 DVB-T 和 DVB-T2。

Frontend 分层（hierarchy）。

可接受的值由 `fe_hierarchy` 定义。



## DTV_STREAM_ID


用于 DVB-C2、DVB-S2、DVB-T2 和 ISDB-S。

DVB-C2、DVB-S2、DVB-T2 和 ISDB-S 支持在单一传输流（transport stream）上传输多个
流（stream）。当硬件支持时，该属性使数字电视驱动能够处理子流过滤（substream
filtering）。默认情况下，子流过滤是禁用的。

对于 DVB-C2、DVB-S2 和 DVB-T2，有效的子流 id 范围为 0 到 255。

对于 ISDB，有效的子流 id 范围为 1 到 65535。

要禁用它，应使用特殊宏 NO_STREAM_ID_FILTER。

注意：任何超出 id 范围的值也会禁用过滤。



## DTV_DVBT2_PLP_ID_LEGACY


已废弃，由 DTV_STREAM_ID 取代。



## DTV_ENUM_DELSYS


一个多标准（multi standard）frontend 需要通告其所提供的传输系统。应用程序在使用
前端的任何其他操作之前，需要枚举所提供的传输系统。在引入该属性之前，使用
FE_GET_INFO 来确定前端类型。对于提供多个传输系统的前端，FE_GET_INFO 帮助不大。
打算使用多标准前端的应用程序必须枚举与其关联的传输系统，而不是尝试使用
FE_GET_INFO。对于遗留前端，结果与 FE_GET_INFO 相同，但格式更具结构化。

可接受的值由 `fe_delivery_system` 定义。



## DTV_INTERLEAVING


要使用的时域交织（time interleaving）。

可接受的值由 `fe_interleaving` 定义。



## DTV_LNA


低噪声放大器（Low-noise amplifier）。

硬件可能提供可控的 LNA，可通过该参数手动设置。通常 LNA 只在地面设备中存在（如果
有的话）。

可能的值：0、1、LNA_AUTO

0，LNA 关闭

1，LNA 开启

使用特殊宏 LNA_AUTO 设置 LNA 为自动模式



## DTV_SCRAMBLING_SEQUENCE_INDEX


用于 DVB-S2。

该 18 位字段（存在时）承载 DVB-S2 物理层加扰序列（scrambling sequence）的索引，
如 EN 302 307 第 5.5.4 节所定义。没有显式的信令方法将加扰序列索引发送给接收端。
如果可用的话，可以使用 S2 卫星传输系统描述符来读取加扰序列索引（EN 300 468
表 41）。

默认使用 gold 加扰序列索引 0。

有效的加扰序列索引范围为 0 到 262142。
