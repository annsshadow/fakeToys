
# 通用 API 元素


编写 V4L2 设备程序包含以下步骤：

- 打开设备

- 更改设备属性，选择视频与音频输入、视频标准、图像亮度等

- 协商数据格式

- 协商输入/输出方法

- 实际的输入/输出循环

- 关闭设备

实际上，大多数步骤都是可选的，并且可以乱序执行。这取决于 V4L2 设备类型，你可以在 devices 中阅读相关细节。本章将讨论适用于所有设备的基本概念。


- [open](open)
- [querycap](querycap)
- [app-pri](app-pri)
- [video](video)
- [audio](audio)
- [tuner](tuner)
- [standard](standard)
- [dv-timings](dv-timings)
- [control](control)
- [extended-controls](extended-controls)
- [ext-ctrls-camera](ext-ctrls-camera)
- [ext-ctrls-flash](ext-ctrls-flash)
- [ext-ctrls-image-source](ext-ctrls-image-source)
- [ext-ctrls-image-process](ext-ctrls-image-process)
- [ext-ctrls-codec](ext-ctrls-codec)
- [ext-ctrls-codec-stateless](ext-ctrls-codec-stateless)
- [ext-ctrls-jpeg](ext-ctrls-jpeg)
- [ext-ctrls-dv](ext-ctrls-dv)
- [ext-ctrls-rf-tuner](ext-ctrls-rf-tuner)
- [ext-ctrls-fm-tx](ext-ctrls-fm-tx)
- [ext-ctrls-fm-rx](ext-ctrls-fm-rx)
- [ext-ctrls-detect](ext-ctrls-detect)
- [ext-ctrls-colorimetry](ext-ctrls-colorimetry)
- [fourcc](fourcc)
- [format](format)
- [planar-apis](planar-apis)
- [selection-api](selection-api)
- [crop](crop)
- [streaming-par](streaming-par)
