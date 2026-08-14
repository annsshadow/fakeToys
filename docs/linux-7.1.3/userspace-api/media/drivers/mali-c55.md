## Arm Mali-C55 ISP 驱动


Arm Mali-C55 ISP 驱动实现了一个驱动专用控制：

`V4L2_CID_MALI_C55_CAPABILITIES (bitmask)`
    通过给出已装配模块的细节，详细说明 ISP 的能力。

    .. flat-table:: 位掩码含义定义
	:header-rows: 1
	:widths: 2 4 8

 - - 位
   - 宏
   - 含义
        - - 0
          - MALI_C55_PONG
          - ISP 中装配了 Pong 配置空间
        - - 1
          - MALI_C55_WDR
          - ISP 中装配了 WDR Framestitch、偏移与增益
        - - 2
          - MALI_C55_COMPRESSION
          - ISP 中装配了 Temper 压缩
        - - 3
          - MALI_C55_TEMPER
          - ISP 中装配了 Temper
        - - 4
          - MALI_C55_SINTER_LITE
          - ISP 中装配的是 Sinter Lite 而非完整版 Sinter
        - - 5
          - MALI_C55_SINTER
          - Sinter 已装配于 ISP
        - - 6
          - MALI_C55_IRIDIX_LTM
          - ISP 中装配了 Iridix 局部色调映射
        - - 7
          - MALI_C55_IRIDIX_GTM
          - ISP 中装配了 Iridix 全局色调映射
        - - 8
          - MALI_C55_CNR
          - ISP 中装配了彩色降噪
        - - 9
          - MALI_C55_FRSCALER
          - ISP 中装配了全分辨率管线缩放器
        - - 10
          - MALI_C55_DS_PIPE
          - ISP 中装配了下缩放管线

    Mali-C55 ISP 可以通过多种方式进行配置，以包含或排除可能不必要的模块。
    该控制为驱动提供了一种向用户空间通报设计中装配了哪些模块的方式。
