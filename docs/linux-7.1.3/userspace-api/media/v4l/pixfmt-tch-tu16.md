######## V4L2_TCH_FMT_TU16 ('TU16')

本文定义 V4L2 触摸数据格式 V4L2_TCH_FMT_TU16（'TU16'），描述其作为来自触摸控制器的无符号 16 位小端原始数据的字节布局与取值范围，供用户空间程序解析触摸设备的输出。



**man V4L2_TCH_FMT_TU16(2)**

16 位无符号小端（little endian）原始触摸数据


## Description


该格式表示来自触摸控制器的无符号 16 位数据。

这可用于原始数据与参考数据的输出。取值范围为 0 到 65535。

**字节序。**
每个单元为一个字节。

   :header-rows:  0
   :stub-columns: 0
   :widths:       2 1 1 1 1 1 1 1 1

   - - start + 0:
     - R'\ `00low`
     - R'\ `00high`
     - R'\ `01low`
     - R'\ `01high`
     - R'\ `02low`
     - R'\ `02high`
     - R'\ `03low`
     - R'\ `03high`
   - - start + 8:
     - R'\ `10low`
     - R'\ `10high`
     - R'\ `11low`
     - R'\ `11high`
     - R'\ `12low`
     - R'\ `12high`
     - R'\ `13low`
     - R'\ `13high`
   - - start + 16:
     - R'\ `20low`
     - R'\ `20high`
     - R'\ `21low`
     - R'\ `21high`
     - R'\ `22low`
     - R'\ `22high`
     - R'\ `23low`
     - R'\ `23high`
   - - start + 24:
     - R'\ `30low`
     - R'\ `30high`
     - R'\ `31low`
     - R'\ `31high`
     - R'\ `32low`
     - R'\ `32high`
     - R'\ `33low`
     - R'\ `33high`

