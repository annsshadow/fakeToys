## stmmac（synopsys dwmac）devlink 支持


本文档描述了`stmmac` 设备驱动实现devlink 功能
## 参数（Parameters

`stmmac` 驱动实现了以下驱动特定的参数
   :widths: 5 5 5 85

   - - Name
     - Type
     - Mode
     - Description
   - - `phc_coarse_adj`
     - Boolean
     - runtime
     - 启用粗粒度（Coarse）时间戳模式，如 DWMAC TRM 中所定义      有关该时间戳模式的详细说明，请参       Socfpga 功能描述 [^1^]
       Coarse 模式下，ptp 时钟预期由一个高精度、外部调整的时钟驱动       用于时间戳的子秒增量（subsecond increment）设置为 1/ptp_clock_rate
       Fine 模式（即 Coarse 模式 == false）下，ptp 时钟频率会被连续调整       但子秒增量设置为 2/ptp_clock_rate
       Coarse 模式适用PTP 主时钟（Grand Master）操作。如果不确定，请       该参数保持为 False
       [^1^] https://www.intel.com/content/www/us/en/docs/programmable/683126/21-2/functional-description-of-the-emac.html
