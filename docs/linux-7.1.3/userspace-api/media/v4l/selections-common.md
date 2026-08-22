## 通用选择定义


虽然 V4L2 selection API <selection-api> V4L2 subdev selection APIs <v4l2-subdev-selections> 非常相似，但两者之间有一个根本区别。在子设API 上，选择矩形指的是媒体总线格式，并绑定到子设备pad。在 V4L2 接口上，选择矩形指的是内存中的像素格式

本节定义了这两个 API 上选择接口的通用定义


- [v4l2-selection-targets](v4l2-selection-targets)
- [v4l2-selection-flags](v4l2-selection-flags)
