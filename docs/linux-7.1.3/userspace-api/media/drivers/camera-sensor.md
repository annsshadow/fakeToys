


## 使用 camera sensor 驱动

本节描述如何通过 V4L2 子设备（sub-device）接口控camera sensor 驱动的常见做法
你或许也会觉media_writing_camera_sensor_drivers 有用
### 传感器内部流水线配置


摄像头传感器具有包含裁剪（cropping）与合并（binning）功能的内部处理流水线。传感器驱动根据驱动配置该功能的方式分为两类：自由可配置驱动与基于寄存器列表（register list-based）的驱动
#### 自由可配置的摄像头传感器驱动


自由可配置的摄像头传感器驱动将设备的内部处理流水线以一个或多个具有不同裁剪与缩放配置的子设备形式暴露出来。设备的输出尺寸是对设备像素阵列（pixel array）尺寸进行一系列裁剪与缩放操作的结果
此类驱动的一个例子是 CCS 驱动
#### 基于寄存器列表的驱动


基于寄存器列表的驱动通常无法根据用户的请求来配置其所控制的设备，而是受限于若干预设配置，这些预设配置把硬件层面相互独立的多个不同参数组合在一起。驱动如何选择这样的配置，取决于设备内部流水线末端pad（source pad）上所设置的格式
大多数传感器驱动以这种方式实现
### 帧间隔（frame interval）配

获取不同帧间隔的可能性以及配置帧间隔有两种不同的方法。实现哪一种取决于设备类型
#### 原始（raw）摄像头传感

原始摄像头传感器并非以帧间隔这样的高层参数来表示，帧间隔是配置多个摄像头传感器实现特有的参数的结果。幸运的是，这些参数对几乎所有现raw 摄像头传感器来说或多或少是相同的
```

	frame interval = (analogue crop width + horizontal blanking) *
			 (analogue crop height + vertical blanking) / pixel rate

```
该公式与总线无关，适用于摄像头传感器之外大量设备的 raw 时序参数。没有模拟裁剪的设备使用完整的源图像尺寸，即像素阵列尺寸
水平消隐（horizontal blanking）与垂直消隐（vertical blanking）分别由 `V4L2_CID_HBLANK` `V4L2_CID_VBLANK` 指定。`V4L2_CID_HBLANK` 控制的单位是像素，`V4L2_CID_VBLANK` 的单位是行。传感器**像素阵列**中的像素速率（pixel rate）由同一子设备的 `V4L2_CID_PIXEL_RATE` 指定，该控制的单位是像素每秒
基于寄存器列表的驱动需要为此目的实现只读子设备节点。非基于寄存器列表的设备需要这些节点来配置设备的内部处理流水线
线性流水线中的第一个实体是像素阵列。像素阵列之后可能跟随其它实体，用于配置 binning、跳行（skipping）、缩放或数字裁剪，参:ref:`VIDIOC_SUBDEV_G_SELECTION
<VIDIOC_SUBDEV_G_SELECTION>`銆。
#### USB 摄像头等设备


USB 视频类（USB video class）硬件，以及许多原生提供类似高层接口的摄像头，通常在固件或硬件层面使用设备级的帧间隔（或帧率）概念。这意味着 raw 摄像头实现的低层控制可能无法在这些设备的 uAPI（甚kAPI）上用于控制帧间隔
### 旋转、朝向与翻转


某些系统的摄像头传感器相对于其自然安装方向被倒装（upside down）安装。在此类情况下，驱动应通过 :ref:`V4L2_CID_CAMERA_SENSOR_ROTATION
<v4l2-camera-sensor-rotation>` 控制向用户空间暴露该信息
传感器驱动还应通过 V4L2_CID_CAMERA_SENSOR_ORIENTATION <v4l2-camera-sensor-orientation> 报告传感器的安装朝向（orientation）
在其寄存器编程序列中嵌入了任何垂直或水平翻转的传感器驱动，应将这些序列所编程的值初始化:ref:`V4L2_CID_HFLIP
<v4l2-cid-hflip>` V4L2_CID_VFLIP <v4l2-cid-vflip> 控制中。这些控制的默认值应0（禁用）。尤其这些控制不应被反转，与传感器的安装旋转方向无关