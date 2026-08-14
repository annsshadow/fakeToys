
## 虚拟无状态解码器驱动（visl）


用于无状态 uAPI 开发的虚拟无状态解码器设备。

该工具的目标是为使用 V4L2 无状态 API 解码媒体的用户空间应用程序的开发与测试
提供帮助。即使没有可用硬件，或者编解码器的内核 uAPI 尚未合入主线，用户空间
实现也可以使用 visl 来运行解码循环。这有助于在早期阶段发现缺陷。

该驱动还可以跟踪提交给它的 V4L2 控件的内容。它还可以通过 debugfs 接口转储 vb2
缓冲区的内容。这在许多方面类似于其他流行编解码 API 所提供的跟踪基础设施，并且
可以通过以另一个（可用的）实现作为参考来帮助开发用户空间应用程序。


        visl 实际上并不会对视频帧进行任何解码。它改用 V4L2 测试图案生成器，
        向捕获缓冲区写入各种调试信息。

### 模块参数


- visl_debug：激活调试信息，通过 dprintk 打印各种调试消息。同时控制是否显示
  逐帧调试信息。默认关闭。注意，启用此功能会通过串口导致性能下降。

- visl_transtime_ms：模拟的处理时间（毫秒）。降低解码速度有助于调试。

- visl_dprintk_frame_start、visl_dprintk_frame_nframes：指定启用 dprintk 的
  帧范围。这仅按帧控制 dprintk 跟踪。注意，通过串口打印大量数据可能较慢。

- keep_bitstream_buffers：控制解码会话结束后是否保留码流（即 OUTPUT）缓冲区。
  默认为 false 以减少杂乱。当使用 GDB 实时调试客户端程序时，
  keep_bitstream_buffers == false 表现良好。

- bitstream_trace_frame_start、bitstream_trace_nframes：与 visl_dprintk_frame_start、
  visl_dprintk_nframes 类似，但改为控制通过 debugfs 转储缓冲区数据。

- tpg_verbose：在每个输出帧上写入额外信息以便于调试 API。当设为 true 时，给定
  输入的输出帧不再稳定，因为会向其中添加指针或队列状态等信息。

### 该驱动的默认用例是什么？


该驱动可用于比较不同的用户空间实现。这假设先运行一个可用的客户端对接 visl，
然后利用 ftrace 和 OUTPUT 缓冲区数据来调试一个正在开发中的实现。

尽管实际上并未进行视频解码，但输出帧仍可作为给定输入的参考，除非 tpg_verbose
被设为 true。

根据 tpg_verbose 参数的值，关于参考帧、其时间戳、OUTPUT 与 CAPTURE 队列的状态等
更多信息，可以直接从 CAPTURE 缓冲区中读取。

### 支持的编解码器


- FWHT
- MPEG2
- VP8
- VP9
- H.264
- HEVC
- AV1

### visl 跟踪事件


跟踪事件是按编解码器定义的，例如：


        $ ls /sys/kernel/tracing/events/ | grep visl
        visl_av1_controls
        visl_fwht_controls
        visl_h264_controls
        visl_hevc_controls
        visl_mpeg2_controls
        visl_vp8_controls
        visl_vp9_controls

例如，要转储 HEVC SPS 数据：


        $ echo 1 >  /sys/kernel/tracing/events/visl_hevc_controls/v4l2_ctrl_hevc_sps/enable

SPS 数据将被转储到跟踪缓冲区，即：


        $ cat /sys/kernel/tracing/trace
        video_parameter_set_id 0
        seq_parameter_set_id 0
        pic_width_in_luma_samples 1920
        pic_height_in_luma_samples 1080
        bit_depth_luma_minus8 0
        bit_depth_chroma_minus8 0
        log2_max_pic_order_cnt_lsb_minus4 4
        sps_max_dec_pic_buffering_minus1 6
        sps_max_num_reorder_pics 2
        sps_max_latency_increase_plus1 0
        log2_min_luma_coding_block_size_minus3 0
        log2_diff_max_min_luma_coding_block_size 3
        log2_min_luma_transform_block_size_minus2 0
        log2_diff_max_min_luma_transform_block_size 3
        max_transform_hierarchy_depth_inter 2
        max_transform_hierarchy_depth_intra 2
        pcm_sample_bit_depth_luma_minus1 0
        pcm_sample_bit_depth_chroma_minus1 0
        log2_min_pcm_luma_coding_block_size_minus3 0
        log2_diff_max_min_pcm_luma_coding_block_size 0
        num_short_term_ref_pic_sets 0
        num_long_term_ref_pics_sps 0
        chroma_format_idc 1
        sps_max_sub_layers_minus1 0
        flags AMP_ENABLED|SAMPLE_ADAPTIVE_OFFSET|TEMPORAL_MVP_ENABLED|STRONG_INTRA_SMOOTHING_ENABLED


### 通过 debugfs 转储 OUTPUT 缓冲区数据


如果启用了 **VISL_DEBUGFS** Kconfig，visl 会根据 bitstream_trace_frame_start 和
bitstream_trace_nframes 的值，将 OUTPUT 缓冲区数据填入
**/sys/kernel/debug/visl/bitstream**。这可以发现错误，因为存在缺陷的客户端可能
无法正确填充缓冲区。

为每个已处理的 OUTPUT 缓冲区创建一个单独的文件。其名称包含一个表示缓冲区序号
的整数，即：


	snprintf(name, 32, "bitstream%d", run->src->sequence);

转储这些值只需从文件读取即可，即：

对于 sequence == 0 的缓冲区：


        $ xxd /sys/kernel/debug/visl/bitstream/bitstream0
        00000000: 2601 af04 d088 bc25 a173 0e41 a4f2 3274  &......%.s.A..2t
        00000010: c668 cb28 e775 b4ac f53a ba60 f8fd 3aa1  .h.(.u...:.`..:.
        00000020: 46b4 bcfc 506c e227 2372 e5f5 d7ea 579f  F...Pl.'#r....W.
        00000030: 6371 5eb5 0eb8 23b5 ca6a 5de5 983a 19e4  cq^...#..j]..:..
        00000040: e8c3 4320 b4ba a226 cbc1 4138 3a12 32d6  ..C ...&..A8:.2.
        00000050: fef3 247b 3523 4e90 9682 ac8e eb0c a389  ..${5#N.........
        00000060: ddd0 6cfc 0187 0e20 7aae b15b 1812 3d33  ..l.... z..[..=3
        00000070: e1c5 f425 a83a 00b7 4f18 8127 3c4c aefb  ...%.:..O..'<L..

对于 sequence == 1 的缓冲区：


        $ xxd /sys/kernel/debug/visl/bitstream/bitstream1
        00000000: 0201 d021 49e1 0c40 aa11 1449 14a6 01dc  ...!I..@...I....
        00000010: 7023 889a c8cd 2cd0 13b4 dab0 e8ca 21fe  p#....,.......!.
        00000020: c4c8 ab4c 486e 4e2f b0df 96cc c74e 8dde  ...LHnN/.....N..
        00000030: 8ce7 ee36 d880 4095 4d64 30a0 ff4f 0c5e  ...6..@.Md0..O.^
        00000040: f16b a6a1 d806 ca2a 0ece a673 7bea 1f37  .k.....*...s{..7
        00000050: 370f 5bb9 1dc4 ba21 6434 bc53 0173 cba0  7.[....!d4.S.s..
        00000060: dfe6 bc99 01ea b6e0 346b 92b5 c8de 9f5d  ........4k.....]
        00000070: e7cc 3484 1769 fef2 a693 a945 2c8b 31da  ..4..i.....E,.1.

依此类推。

默认情况下，这些文件会在 STREAMOFF 期间被删除。这是为了减少杂乱。
