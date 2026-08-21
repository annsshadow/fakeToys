## cafe_ccic 驱动


Author: Jonathan Corbet <corbet@lwn.net>

### 简

“cafe_ccic是面Marvell 88ALP01 “cafeCMOS 摄像头控制器的驱动。这是初OLPC 系统所用的控制器，该驱动在 OLPC 项目的支持下编写
当前状态：核心驱动可以工作。它可以生成 YUV422、RGB565 RGB444 格式的数据。（看到代码的人
也会发现 RGB32，但那是一个调试辅助，将很快被移除）。VGA QVGA 模式可用；CIF 也在，但颜色
仍有问题。目前已知只OV7670 传感器可与该控制器配合工作
试用的话，以下任一命令都可用：


     $ mplayer tv:// -tv driver=v4l2:width=640:height=480 -nosound
     $ mplayer tv:// -tv driver=v4l2:width=640:height=480:outfmt=bgr16 -nosound

“xawtv工具也可用；gqcam 由于未知原因不可用
### 加载时选项


有少量加载时选项，大多数也可以在加载后通过 sysfs 更改
 - alloc_bufs_at_load：通常，驱动在数据传输到来之前不会分配任何 DMA 缓冲区。如果设置了   选项，则会在模块加载时分配最坏情况下大小的缓冲区。该选项在整个模块生命周期内固定
   内存，但也许会降低以后分配失败的几率
 - dma_buf_size：要分配DMA 缓冲区大小。注意该选项仅在加载时分配时生效；当缓冲区在运行
   时分配时，它们会按当前摄像头设置适当调整大小
 - n_dma_bufs：控制器可以在两个或三个 DMA 缓冲区之间循环。通常，驱动尝试使用三个缓冲区   但在较快的系统上，只用两个也能良好工作
 - min_buffers：驱动愿意配合工作的最小流I/O 缓冲区数。默认为 1，但在较慢的系统上，   其值设为更高（6）可获得更好mplayer 行为
 - max_buffers：流I/O 缓冲区的最大数量；默认10。该数字是从帽子里精心挑出来的，不应
   假定它实际上有多大意义
 - flip：如果设置了该布尔参数，将指示传感器反转视频图像。这是否有意义取决于你的摄像   具体如何安装