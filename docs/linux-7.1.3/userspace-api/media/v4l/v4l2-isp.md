


######## 通用 V4L2 ISP 格式


通用 ISP 格式是一组元数据格式，定义了在用户空间与驱动之间、通过 V4L2 缓冲区传递
ISP 参数与统计信息的机制。它们的设计允许以向后兼容的方式对其进行扩展。

## ISP 参数


通用 ISP 配置参数格式通过定义一个单一的 C 结构体来实现，该结构体包含一个头部，
随后是一个二进制缓冲区，用户空间在其中编程可变数量的 ISP 配置数据块，每个受支持的
ISP 特性对应一个。

`v4l2_isp_params_buffer` 结构体定义了缓冲区头部，其后跟着 ISP 配置数据的二进制
缓冲区。用户空间应当正确地用通用参数格式版本以及它将用来存储 ISP 块配置的二进制
数据缓冲区的大小（字节数）来填充缓冲区头部。

每个 **ISP 配置块** 前面都有一个由 `v4l2_isp_params_block_header` 结构体实现的
头部，其后跟着该特定块的配置参数，由 ISP 驱动特定的数据类型定义。

用户空间应用程序负责正确地填充每个块的头部字段（type、flags 与 size）以及块特定的
参数。

### ISP 块的启用、禁用与配置


当用户空间想要配置并启用一个 ISP 块时，它应当完整地填充该块配置，并在块头部的
`flags` 字段中设置 V4L2_ISP_PARAMS_FL_BLOCK_ENABLE 位。

当用户空间仅仅想要禁用一个 ISP 块时，应当在块头部的 `flags` 字段中设置
V4L2_ISP_PARAMS_FL_BLOCK_DISABLE 位。在这种情况下，驱动接受在头部之后没有额外数据
的配置参数块。

如果必须更新一个已经激活的 ISP 块的配置，用户空间应当完整地填充该 ISP 块的参数，
并省略在头部的 `flags` 字段中设置 V4L2_ISP_PARAMS_FL_BLOCK_ENABLE 与
V4L2_ISP_PARAMS_FL_BLOCK_DISABLE 位。

在 flags 字段中同时设置 V4L2_ISP_PARAMS_FL_BLOCK_ENABLE 与
V4L2_ISP_PARAMS_FL_BLOCK_DISABLE 位是不允许的，会返回错误。

可以通过添加新块定义来扩展参数格式，而不会使现有的块失效。

## ISP 统计信息


Video4Linux2 中尚未实现对通用统计信息格式的支持。

## V4L2 ISP uAPI 数据类型
