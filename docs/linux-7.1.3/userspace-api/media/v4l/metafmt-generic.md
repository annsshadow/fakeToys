
######## V4L2_META_FMT_GENERIC_8 ('MET8'), V4L2_META_FMT_GENERIC_CSI2_10 ('MC10'), V4L2_META_FMT_GENERIC_CSI2_12 ('MC1C'), V4L2_META_FMT_GENERIC_CSI2_14 ('MC1E'), V4L2_META_FMT_GENERIC_CSI2_16 ('MC1G'), V4L2_META_FMT_GENERIC_CSI2_20 ('MC1K'), V4L2_META_FMT_GENERIC_CSI2_24 ('MC1O')


通用基于行的元数据格式（Generic line-based metadata formats

## 描述

这些通用的基于行的元数据格式仅定义数据的内存布局，而不定义元数据本身的格式或含义

### V4L2_META_FMT_GENERIC_8


V4L2_META_FMT_GENERIC_8 格式是一种普通的 8 位元数据格式。该格式用于 CSI-2 *数据单元**（Data Unit 位的情况。此外，当两个字节的元数据被打包进一16 位数据单元时，它也用于每数据单元 16 位的情况。否则，每像16 位的数据格式
应为 V4L2_META_FMT_GENERIC_CSI2_16 <v4l2-meta-fmt-generic-csi2-16>
**V4L2_META_FMT_GENERIC_8 的字节序*
每个单元格为一个字节M" 表示一个字节的元数据

    :header-rows:  0
    :stub-columns: 0
    :widths: 12 8 8 8 8

    - - 偏移 + 0:
      - M\ `00`
      - M\ `10`
      - M\ `20`
      - M\ `30`
    - - 偏移 + 4:
      - M\ `01`
      - M\ `11`
      - M\ `21`
      - M\ `31`


### V4L2_META_FMT_GENERIC_CSI2_10


V4L2_META_FMT_GENERIC_CSI2_10 包含打包10 位数据单元中8 位通用元数据，
每四个字节的元数据之后跟一个填充字节。该格式通常CSI-2 接收端使用，其数据源
会发MEDIA_BUS_FMT_META_10，CSI-2 接收端将接收到的数据按原样写入内存数据的打包方式遵MIPI CSI-2 规范，填充方式定义于 MIPI CCS 规范
该格式也用于每数据单20 位的格式，后者将两个字节的元数据打包进一个数据单元否则，每像素 20 位的数据格式:ref:`V4L2_META_FMT_GENERIC_CSI2_20
<v4l2-meta-fmt-generic-csi2-20>`銆。
该格式为小端序（little endian）
**V4L2_META_FMT_GENERIC_CSI2_10 的字节序*
每个单元格为一个字节M" 表示一个字节的元数据，"x" 表示一个字节的填充

    :header-rows:  0
    :stub-columns: 0
    :widths: 12 8 8 8 8 8

    - - 偏移 + 0:
      - M\ `00`
      - M\ `10`
      - M\ `20`
      - M\ `30`
      - x
    - - 偏移 + 5:
      - M\ `01`
      - M\ `11`
      - M\ `21`
      - M\ `31`
      - x


### V4L2_META_FMT_GENERIC_CSI2_12


V4L2_META_FMT_GENERIC_CSI2_12 包含打包12 位数据单元中8 位通用元数据，
每两个字节的元数据之后跟一个填充字节。该格式通常CSI-2 接收端使用，其数据源
会发MEDIA_BUS_FMT_META_12，CSI-2 接收端将接收到的数据按原样写入内存数据的打包方式遵MIPI CSI-2 规范，填充方式定义于 MIPI CCS 规范
该格式也用于每数据单24 位的格式，后者将两个字节的元数据打包进一个数据单元否则，每像素 24 位的数据格式:ref:`V4L2_META_FMT_GENERIC_CSI2_24
<v4l2-meta-fmt-generic-csi2-24>`銆。
该格式为小端序（little endian）
**V4L2_META_FMT_GENERIC_CSI2_12 的字节序*
每个单元格为一个字节M" 表示一个字节的元数据，"x" 表示一个字节的填充

    :header-rows:  0
    :stub-columns: 0
    :widths: 12 8 8 8 8 8 8

    - - 偏移 + 0:
      - M\ `00`
      - M\ `10`
      - x
      - M\ `20`
      - M\ `30`
      - x
    - - 偏移 + 6:
      - M\ `01`
      - M\ `11`
      - x
      - M\ `21`
      - M\ `31`
      - x


### V4L2_META_FMT_GENERIC_CSI2_14


V4L2_META_FMT_GENERIC_CSI2_14 包含打包14 位数据单元中8 位通用元数据，
每四个字节的元数据之后跟三个填充字节。该格式通常CSI-2 接收端使用，其数据源
会发MEDIA_BUS_FMT_META_14，CSI-2 接收端将接收到的数据按原样写入内存数据的打包方式遵MIPI CSI-2 规范，填充方式定义于 MIPI CCS 规范
该格式为小端序（little endian）
**V4L2_META_FMT_GENERIC_CSI2_14 的字节序*
每个单元格为一个字节M" 表示一个字节的元数据，"x" 表示一个字节的填充

    :header-rows:  0
    :stub-columns: 0
    :widths: 12 8 8 8 8 8 8 8

    - - 偏移 + 0:
      - M\ `00`
      - M\ `10`
      - M\ `20`
      - M\ `30`
      - x
      - x
      - x
    - - 偏移 + 7:
      - M\ `01`
      - M\ `11`
      - M\ `21`
      - M\ `31`
      - x
      - x
      - x


### V4L2_META_FMT_GENERIC_CSI2_16


V4L2_META_FMT_GENERIC_CSI2_16 包含打包16 位数据单元中8 位通用元数据，
每字节元数据之后跟一个填充字节。该格式通常CSI-2 接收端使用，其数据源会发MEDIA_BUS_FMT_META_16，CSI-2 接收端将接收到的数据按原样写入内存。数据的
打包方式遵循 MIPI CSI-2 规范，填充方式定义于 MIPI CCS 规范
部分设备在与 16 位图像数据配合时支持更高效的元数据打包方式。在这种情况下，
应使用的数据格式V4L2_META_FMT_GENERIC_8 <v4l2-meta-fmt-generic-8>
该格式为小端序（little endian）
**V4L2_META_FMT_GENERIC_CSI2_16 的字节序*
每个单元格为一个字节M" 表示一个字节的元数据，"x" 表示一个字节的填充

    :header-rows:  0
    :stub-columns: 0
    :widths: 12 8 8 8 8 8 8 8 8

    - - 偏移 + 0:
      - M\ `00`
      - x
      - M\ `10`
      - x
      - M\ `20`
      - x
      - M\ `30`
      - x
    - - 偏移 + 8:
      - M\ `01`
      - x
      - M\ `11`
      - x
      - M\ `21`
      - x
      - M\ `31`
      - x


### V4L2_META_FMT_GENERIC_CSI2_20


V4L2_META_FMT_GENERIC_CSI2_20 包含打包20 位数据单元中8 位通用元数据，
每字节元数据之后交替跟一个或两个填充字节。该格式通常CSI-2 接收端使用，数据源会发MEDIA_BUS_FMT_META_20，CSI-2 接收端将接收到的数据按原样写内存。数据的打包方式遵循 MIPI CSI-2 规范，填充方式定义于 MIPI CCS 规范
部分设备在与 16 位图像数据配合时支持更高效的元数据打包方式。在这种情况下，
应使用的数据格式V4L2_META_FMT_GENERIC_CSI2_10 <v4l2-meta-fmt-generic-csi2-10>
该格式为小端序（little endian）
**V4L2_META_FMT_GENERIC_CSI2_20 的字节序*
每个单元格为一个字节M" 表示一个字节的元数据，"x" 表示一个字节的填充

    :header-rows:  0
    :stub-columns: 0
    :widths: 12 8 8 8 8 8 8 8 8 8 8

    - - 偏移 + 0:
      - M\ `00`
      - x
      - M\ `10`
      - x
      - x
      - M\ `20`
      - x
      - M\ `30`
      - x
      - x
    - - 偏移 + 10:
      - M\ `01`
      - x
      - M\ `11`
      - x
      - x
      - M\ `21`
      - x
      - M\ `31`
      - x
      - x


### V4L2_META_FMT_GENERIC_CSI2_24


V4L2_META_FMT_GENERIC_CSI2_24 包含打包24 位数据单元中8 位通用元数据，
每字节元数据之后跟两个填充字节。该格式通常CSI-2 接收端使用，其数据源会发MEDIA_BUS_FMT_META_24，CSI-2 接收端将接收到的数据按原样写入内存。数据的
打包方式遵循 MIPI CSI-2 规范，填充方式定义于 MIPI CCS 规范
部分设备在与 16 位图像数据配合时支持更高效的元数据打包方式。在这种情况下，
应使用的数据格式V4L2_META_FMT_GENERIC_CSI2_12 <v4l2-meta-fmt-generic-csi2-12>
该格式为小端序（little endian）
**V4L2_META_FMT_GENERIC_CSI2_24 的字节序*
每个单元格为一个字节M" 表示一个字节的元数据，"x" 表示一个字节的填充

    :header-rows:  0
    :stub-columns: 0
    :widths: 12 8 8 8 8 8 8 8 8 8 8 8 8

    - - 偏移 + 0:
      - M\ `00`
      - x
      - x
      - M\ `10`
      - x
      - x
      - M\ `20`
      - x
      - x
      - M\ `30`
      - x
      - x
    - - 偏移 + 12:
      - M\ `01`
      - x
      - x
      - M\ `11`
      - x
      - x
      - M\ `21`
      - x
      - x
      - M\ `31`
      - x
      - x
