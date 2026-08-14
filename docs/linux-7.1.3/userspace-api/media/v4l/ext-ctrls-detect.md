


######## 检测控制参考（Detect Control Reference）


Detect 类包含用于各种具备运动或物体检测能力的设备的通用特性控制。



## 检测控制 IDs


`V4L2_CID_DETECT_CLASS (class)`
    Detect 类描述符。对该控制调用 VIDIOC_QUERYCTRL 将返回该控制类的描述。

`V4L2_CID_DETECT_MD_MODE (menu)`
    设置运动检测模式。


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_DETECT_MD_MODE_DISABLED`
      - 禁用运动检测。
    - - `V4L2_DETECT_MD_MODE_GLOBAL`
      - 使用单一的运动检测阈值。
    - - `V4L2_DETECT_MD_MODE_THRESHOLD_GRID`
      - 将图像划分为网格，每个单元格有自己的运动检测阈值。这些阈值通过
	`V4L2_CID_DETECT_MD_THRESHOLD_GRID` 矩阵控制设置。
    - - `V4L2_DETECT_MD_MODE_REGION_GRID`
      - 将图像划分为网格，每个单元格有自己的区域值，用于指定应当使用哪个
	每区域的运动检测阈值。每个区域都有自己的阈值。这些每区域阈值的设置方式
	是驱动相关的。网格的区域值通过 `V4L2_CID_DETECT_MD_REGION_GRID` 矩阵控制设置。



`V4L2_CID_DETECT_MD_GLOBAL_THRESHOLD (integer)`
    设置与 `V4L2_DETECT_MD_MODE_GLOBAL` 运动检测模式一起使用的全局运动检测阈值。

`V4L2_CID_DETECT_MD_THRESHOLD_GRID (__u16 matrix)`
    设置网格中每个单元格的运动检测阈值。需与 `V4L2_DETECT_MD_MODE_THRESHOLD_GRID`
    运动检测模式一起使用。矩阵元素 (0, 0) 表示网格左上角的单元格。

`V4L2_CID_DETECT_MD_REGION_GRID (__u8 matrix)`
    设置网格中每个单元格的运动检测区域值。需与 `V4L2_DETECT_MD_MODE_REGION_GRID`
    运动检测模式一起使用。矩阵元素 (0, 0) 表示网格左上角的单元格。
