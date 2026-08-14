
######## V4L2_META_FMT_RK_ISP1_PARAMS ('rk1p')、V4L2_META_FMT_RK_ISP1_STAT_3A ('rk1s')、V4L2_META_FMT_RK_ISP1_EXT_PARAMS ('rk1e')


## 配置参数


RkISP1 ISP 的配置由用户空间通过 `v4l2_meta_format` 接口向驱动提供 ISP 参数
来完成。

有两种方法可以配置 ISP，分别是 `fixed parameters`（固定参数）配置格式与
`extensible parameters`（可扩展参数）配置格式。


## 固定参数配置格式


使用固定配置格式时，参数通过 `V4L2_META_FMT_RK_ISP1_PARAMS` 元格式传递给
rkisp1_params <rkisp1_params> 元数据输出视频节点。

缓冲区包含 `rkisp1-config.h` 中定义的 C 结构体 `rkisp1_params_cfg` 的单个
实例。因此可以从缓冲区中这样获取该结构体：


	struct rkisp1_params_cfg **params = (struct rkisp1_params_cfg**) buffer;

该方法仅支持 ISP 特性的一部分，新的应用程序应当使用可扩展参数方法。


## 可扩展参数配置格式


使用可扩展配置格式时，参数通过 `V4L2_META_FMT_RK_ISP1_EXT_PARAMS` 元格式
传递给 rkisp1_params <rkisp1_params> 元数据输出视频节点。

缓冲区包含 `rkisp1-config.h` 中定义的 C 结构体 `rkisp1_ext_params_cfg` 的
单个实例。`rkisp1_ext_params_cfg` 结构体被设计为允许用户空间仅用其打算配置的
ISP 模块的配置数据来填充数据缓冲区。可扩展参数格式的设计允许开发者定义新的
模块类型以支持新的配置参数，并定义了一套版本机制，从而可以在不破坏与现有
应用程序兼容性的情况下进行扩展与版本管理。

基于这些原因，该配置方法优先于 `fixed parameters`（固定参数）格式方案。


## 3A 与直方图统计


ISP1 设备会针对输入的 Bayer 帧收集不同的统计数据。这些统计数据通过
`v4l2_meta_format` 接口从 rkisp1_stats <rkisp1_stats> 元数据捕获视频节点
获取，缓冲区包含 `rkisp1-config.h` 中定义的 C 结构体 `rkisp1_stat_buffer` 的
单个实例。因此可以从缓冲区中这样获取该结构体：


	struct rkisp1_stat_buffer **stats = (struct rkisp1_stat_buffer**) buffer;

收集的统计信息包括曝光、AWB（自动白平衡）、直方图与 AF（自动对焦）。统计信息
的详情请参见 `rkisp1_stat_buffer`。

此处描述的 3A 统计信息与配置参数通常由专用的用户空间库消费和产生，这些库
包含了使用软件控制环的重要调优工具。


## rkisp1 uAPI 数据类型


