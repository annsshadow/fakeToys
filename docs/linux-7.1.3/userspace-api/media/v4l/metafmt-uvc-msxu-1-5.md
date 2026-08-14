
######## V4L2_META_FMT_UVC_MSXU_1_5 ('UVCM')


Microsoft(R) 的 UVC 负载元数据。


## 描述


V4L2_META_FMT_UVC_MSXU_1_5 缓冲区遵循 V4L2_META_FMT_UVC 的元数据缓冲区布局，唯一
区别在于它在 `buffer[]` 字段中包含了所有 UVC 元数据，而不仅是前 2-12 字节。

元数据格式遵循 Microsoft(R) [^1^] 的规范。


[^1^] https://docs.microsoft.com/en-us/windows-hardware/drivers/stream/uvc-extensions-1-5
