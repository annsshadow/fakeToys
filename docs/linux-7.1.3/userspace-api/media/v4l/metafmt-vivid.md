######## V4L2_META_FMT_VIVID ('VIVD')


VIVID 元数据格


## 描述


本格式描vivid 驱动所使用的元数据格式

它设置亮度（Brightness）、饱和度（Saturation）、对比度（Contrast）和色相（Hue），每一项都映射
vivid 驱动的对应控件，范围与默认值均与之对应

包含以下字段

    :widths: 1 4
    :header-rows:  1
    :stub-columns: 0

    - - 字段
      - Description
    - - u16 brightness;
      - 图像亮度，取值范0 255，默认值为 128
    - - u16 contrast;
      - 图像对比度，取值范0 255，默认值为 128
    - - u16 saturation;
      - 图像色彩饱和度，取值范0 255，默认值为 128
    - - s16 hue;
      - 图像色彩平衡，取值范-128 128，默认值为 0

