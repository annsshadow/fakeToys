

######## 相机控制参


Camera 类包含用于设备机械（或等效的数字）特性的控制，例如可控镜头或传感器


## 相机控制 ID


`V4L2_CID_CAMERA_CLASS (class)`
    相机类描述符。对该控制调
    VIDIOC_QUERYCTRL 将返回该控制类的描述


`V4L2_CID_EXPOSURE_AUTO`
    (enum)

enum v4l2_exposure_auto_type -
    启用对曝光时间和/或光圈孔径的自动调整。在启用这些特性时手动修改曝光时间或光圈孔径的效果是未定义的，驱动应忽略此类请求。可能的值有


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_EXPOSURE_AUTO`
      - 自动曝光时间，自动光圈孔径
    - - `V4L2_EXPOSURE_MANUAL`
      - 手动曝光时间，手动光圈
    - - `V4L2_EXPOSURE_SHUTTER_PRIORITY`
      - 手动曝光时间，自动光圈
    - - `V4L2_EXPOSURE_APERTURE_PRIORITY`
      - 自动曝光时间，手动光圈



`V4L2_CID_EXPOSURE_ABSOLUTE (integer)`
    决定相机传感器的曝光时间。曝光时间受帧间隔限制。驱动应将数值解释为 100 µs 单位，其中1 表示 1/10000 秒，10000 表示 1 秒，100000 表示 10 秒

`V4L2_CID_EXPOSURE_AUTO_PRIORITY (boolean)`
    `V4L2_CID_EXPOSURE_AUTO` 设为 `AUTO` 
    `APERTURE_PRIORITY` 时，该控制决定设备是否可以动态调整帧率。默认情况下该功能被禁用
    (0)，帧率必须保持恒定

`V4L2_CID_AUTO_EXPOSURE_BIAS (integer menu)`
    决定自动曝光补偿，仅`V4L2_CID_EXPOSURE_AUTO` 控制设为 `AUTO`
    `SHUTTER_PRIORITY` `APERTURE_PRIORITY` 时才生效。它
    EV 表示，驱动应将数值解释为 0.001 EV 单位，其中1000 表示 +1 EV

    增大曝光补偿值相当于降低曝光值（EV），并会增加图像传感器处的光量。相机通过调整绝对曝光时间或光圈孔径来执行曝光补偿


`V4L2_CID_EXPOSURE_METERING`
    (enum)

enum v4l2_exposure_metering -
    决定相机如何测量可用于帧曝光的光量。可能的值有


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_EXPOSURE_METERING_AVERAGE`
      - 使用来自整个帧的光信息并取平均，对测光区域的任何特定部分都不加权
    - - `V4L2_EXPOSURE_METERING_CENTER_WEIGHTED`
      - 对来自整个帧的光信息取平均，但优先对待测光区域的中心
    - - `V4L2_EXPOSURE_METERING_SPOT`
      - 仅测量帧中心非常小的区域
    - - `V4L2_EXPOSURE_METERING_MATRIX`
      - 多区域测光。在帧的若干个点测量光强并合并结果。区域选择及其在计算最终值中的重要性的算法取决于具体设备



`V4L2_CID_PAN_RELATIVE (integer)`
    该控制将相机水平转动指定量。单位未定义。正值使相机向右移动（从上方看为顺时针），负值向左。零值不引起运动。这是一个只写控制

`V4L2_CID_TILT_RELATIVE (integer)`
    该控制将相机垂直转动指定量。单位未定义。正值使相机向上移动，负值向下。零值不引起运动。这是一个只写控制

`V4L2_CID_PAN_RESET (button)`
    设置该控制时，相机水平移动到默认位置

`V4L2_CID_TILT_RESET (button)`
    设置该控制时，相机垂直移动到默认位置

`V4L2_CID_PAN_ABSOLUTE (integer)`
    该控制将相机水平转动到指定位置。正值使相机向右移动（从上方看为顺时针），负值向左。驱动应将数值解释为角秒，有效值在 -180
    * 3600 +180 * 3600（含边界）之间

`V4L2_CID_TILT_ABSOLUTE (integer)`
    该控制将相机垂直转动到指定位置。正值使相机向上移动，负值向下。驱动应将数值解
    为角秒，有效值在 -180 ** 3600 +180 ** 3600（含边界）之间

`V4L2_CID_FOCUS_ABSOLUTE (integer)`
    该控制将相机的焦点设置到指定位置。单位未定义。正值将焦点移近相机，负值移向无穷远

`V4L2_CID_FOCUS_RELATIVE (integer)`
    该控制将相机的焦点移动指定量。单位未定义。正值将焦点移近相机，负值移向无穷远。这是一个只写控制

`V4L2_CID_FOCUS_AUTO (boolean)`
    启用连续自动对焦调整。在启用该特性时手动对焦调整的效果是未定义的，驱动应忽略此类请求

`V4L2_CID_AUTO_FOCUS_START (button)`
    启动单次自动对焦过程。当 `V4L2_CID_FOCUS_AUTO` 设为 `TRUE` (1) 时设置该控制的效果是未定义的，驱动应忽略此类请求

`V4L2_CID_AUTO_FOCUS_STOP (button)`
    中止`V4L2_CID_AUTO_FOCUS_START` 控制启动的自动对焦。仅当连续自动对焦被禁用（即 `V4L2_CID_FOCUS_AUTO` 控制设为 `FALSE` (0)）时才生效


`V4L2_CID_AUTO_FOCUS_STATUS (bitmask)`
    自动对焦状态。这是一个只读控制

    设置 `V4L2_CID_3A_LOCK` 控制`V4L2_LOCK_FOCUS` 锁位可能会停止对
    `V4L2_CID_AUTO_FOCUS_STATUS` 控制值的更新


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_AUTO_FOCUS_STATUS_IDLE`
      - 自动对焦未激活
    - - `V4L2_AUTO_FOCUS_STATUS_BUSY`
      - 自动对焦进行中
    - - `V4L2_AUTO_FOCUS_STATUS_REACHED`
      - 已达到焦点
    - - `V4L2_AUTO_FOCUS_STATUS_FAILED`
      - 自动对焦失败，在应用程序执行另一个动作之前，驱动不会从该状态转换



`V4L2_CID_AUTO_FOCUS_RANGE`
    (enum)

enum v4l2_auto_focus_range -
    决定镜头可调的自动对焦距离范围


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_AUTO_FOCUS_RANGE_AUTO`
      - 相机自动选择对焦范围
    - - `V4L2_AUTO_FOCUS_RANGE_NORMAL`
      - 正常距离范围，为获得最佳自动对焦性能而受限
    - - `V4L2_AUTO_FOCUS_RANGE_MACRO`
      - 微距（特写）自动对焦。相机将使用其最小可能距离进行自动对焦
    - - `V4L2_AUTO_FOCUS_RANGE_INFINITY`
      - 镜头设为对焦无穷远处的物体



`V4L2_CID_ZOOM_ABSOLUTE (integer)`
    以绝对值指定物镜焦距。变焦单位由驱动特定，其值应为正整数

`V4L2_CID_ZOOM_RELATIVE (integer)`
    相对于当前值指定物镜焦距。正值使变焦镜头组移向长焦方向，负值移向广角方向。变焦单位由驱动特定。这是一个只写控制

`V4L2_CID_ZOOM_CONTINUOUS (integer)`
    以指定速度移动变焦镜头组，直到达到物理设备极限或收到明确的停止移动请求。正值使变焦镜头组移向长焦方向。零值停止变焦镜头组的运动。负值使变焦镜头组移向广角方向。变焦速度单位由驱动特定

`V4L2_CID_IRIS_ABSOLUTE (integer)`
    该控制将相机的光圈设置到指定值。单位未定义。较大的值使光圈开得更大，较小的值使其关闭

`V4L2_CID_IRIS_RELATIVE (integer)`
    该控制按指定量修改相机的光圈。单位未定义。正值使光圈再开大一步，负值再关闭一步。这是一个只写控制

`V4L2_CID_PRIVACY (boolean)`
    阻止相机获取视频。当该控制设`TRUE` (1) 时，相机无法捕获任何图像。强制隐私的常见手段是传感器的机械遮光以及固件图像处理，但设备不限于这些方法。实privacy 控制的设备必须支持读访问，并可以支持写访问

`V4L2_CID_BAND_STOP_FILTER (integer)`
    开启或关闭相机传感器的带阻滤波器，或指定其强度。此类带阻滤波器可用于例如滤除荧光灯成分


`V4L2_CID_AUTO_N_PRESET_WHITE_BALANCE`
    (enum)

enum v4l2_auto_n_preset_white_balance -
    将白平衡设为自动、手动或预设。预设决定光的色温，作为相机进行白平衡调整的提示，从而获得最准确的色彩表现。以下白平衡预设按色温递增顺序排列


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_WHITE_BALANCE_MANUAL`
      - 手动白平衡
    - - `V4L2_WHITE_BALANCE_AUTO`
      - 自动白平衡调整
    - - `V4L2_WHITE_BALANCE_INCANDESCENT`
      - 白炽灯（钨丝灯）照明的白平衡设置。它通常会使颜色偏冷，大约对应于
        2500...3500 K 色温范围
    - - `V4L2_WHITE_BALANCE_FLUORESCENT`
      - 荧光灯照明的白平衡预设。大约对应于 4000...5000 K 色温
    - - `V4L2_WHITE_BALANCE_FLUORESCENT_H`
      - 使用该设置时，相机将补偿荧光H 照明
    - - `V4L2_WHITE_BALANCE_HORIZON`
      - 地平线日光的白平衡设置。大约对应于 5000 K 色温
    - - `V4L2_WHITE_BALANCE_DAYLIGHT`
      - 日光（晴朗天空）的白平衡预设。大约对应于 5000...6500 K 色温
    - - `V4L2_WHITE_BALANCE_FLASH`
      - 使用该设置时，相机将补偿闪光灯照明。它使颜色略微偏暖，大约对应
        5000...5500 K 色温
    - - `V4L2_WHITE_BALANCE_CLOUDY`
      - 中度阴天的白平衡预设。该选项大约对应6500...8000 K 色温范围
    - - `V4L2_WHITE_BALANCE_SHADE`
      - 阴影或浓密阴天的白平衡预设。大约对应于 9000...10000 K 色温



`V4L2_CID_WIDE_DYNAMIC_RANGE (boolean)`
    启用或禁用相机的宽动态范围特性。该特性允许在场景内光照强度变化显著（即同时存在非常暗和非常亮的区域）的情况下获得清晰的图像。它最常见的是通过合并两帧曝光时间不同的后续帧来实现[#f1]_


`V4L2_CID_IMAGE_STABILIZATION (boolean)`
    启用或禁用图像稳定

`V4L2_CID_ISO_SENSITIVITY (integer menu)`
    决定图像传感器的 ISO 等效值，表示传感器对光的灵敏度。这些数字按算术标度表示，遵iso12232 标准，其中传感器灵敏度加倍由数ISO 值加倍表示。应用程序应将数值解释为标准 ISO 值乘1000，例如控制800 表示 ISO 0.8。驱动通常只支持标ISO 值的一个子集。在
    `V4L2_CID_ISO_SENSITIVITY_AUTO` 控制设为 `V4L2_CID_ISO_SENSITIVITY_MANUAL`
    以外的值时设置该控制的效果是未定义的，驱动应忽略此类请求


`V4L2_CID_ISO_SENSITIVITY_AUTO`
    (enum)

enum v4l2_iso_sensitivity_type -
    启用或禁用自ISO 灵敏度调整



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_CID_ISO_SENSITIVITY_MANUAL`
      - 手动 ISO 灵敏度
    - - `V4L2_CID_ISO_SENSITIVITY_AUTO`
      - 自动 ISO 灵敏度调整



`V4L2_CID_SCENE_MODE`
    (enum)

enum v4l2_scene_mode -
    该控制允许选择场景程序，即相机针对常见拍摄场景优化的自动模式。在这些模式下，相机决定最佳曝光、光圈、对焦、测光、白平衡和等效灵敏度。这些参数的控制受场景模式控制影响。每种模式下的确切行为取决于相机规格

    当不使用场景模式特性时，应将此控制设为 `V4L2_SCENE_MODE_NONE`，以确保其他可能相关的控制可访问。定义了以下场景程序


    \small



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_SCENE_MODE_NONE`
      - 场景模式特性被禁用
    - - `V4L2_SCENE_MODE_BACKLIGHT`
      - 背光。当光来自主体背后时补偿暗部阴影，也会自动开启闪光灯
    - - `V4L2_SCENE_MODE_BEACH_SNOW`
      - 海滩和雪地。该模式补偿全白或明亮的场景，当相机自动曝光基于平均场景亮度时，这类场景往往显得灰暗且对比度低。为补偿，该模式自动略微过曝帧。白平衡也可能被调整，以补偿反射的雪看起来偏蓝而非白色这一事实
    - - `V4L2_SCENE_MODE_CANDLELIGHT`
      - 烛光。相机通常会提ISO 灵敏度并降低快门速度。该模式补偿场景中相对靠近的主体。为保留光线氛围，闪光灯被禁用
    - - `V4L2_SCENE_MODE_DAWN_DUSK`
      - 黎明和黄昏。保留黄昏前和黎明后低自然光下看到的颜色。相机可能会关闭闪光灯，并自动对焦到无穷远。它通常会提高饱和度并降低快门速度
    - - `V4L2_SCENE_MODE_FALL_COLORS`
      - 秋色。提高饱和度并调整白平衡以增强色彩。秋叶照片会得到饱和的红色和黄色
    - - `V4L2_SCENE_MODE_FIREWORKS`
      - 烟花。使用长曝光时间来捕捉烟花向外扩散的光爆发。相机可能会调用图像稳定
    - - `V4L2_SCENE_MODE_LANDSCAPE`
      - 风景。相机会选择小光圈以提供深景深，并使用长曝光时长以帮助在昏暗光线下捕捉细节。对焦固定在无穷远。适合远景和广阔风景
    - - `V4L2_SCENE_MODE_NIGHT`
      - 夜间，也称夜间风景。为低光条件设计，它在保留暗部细节的同时不会使明亮物体过曝。相机通常将自身设为中到高 ISO 灵敏度，配合相对较长的曝光时间，并关闭闪光灯。因此，图像噪点会增加，并可能出现图像模糊
    - - `V4L2_SCENE_MODE_PARTY_INDOOR`
      - 聚会和室内。为捕捉由室内背景照明以及闪光灯共同照明的室内场景而设计。相机通常会提ISO 灵敏度，并为低光条件调整曝光
    - - `V4L2_SCENE_MODE_PORTRAIT`
      - 人像。相机调整光圈以减小景深，有助于将主体从平滑的背景中分离出来。大多数相机会识别场景中人脸并对其对焦。色调被调整以增强肤色。闪光灯强度通常降低
    - - `V4L2_SCENE_MODE_SPORTS`
      - 运动。显著提ISO 并使用快速快门速度以冻结快速移动主体的动作。该模式下可能会看到增加的图像噪点
    - - `V4L2_SCENE_MODE_SUNSET`
      - 日落。保留在日落和日出中看到的深沉色调。它提高饱和度
    - - `V4L2_SCENE_MODE_TEXT`
      - 文本。它应用额外的对比度和锐度，通常是一种为可读性优化的黑白模式。自动对焦可能切换到特写模式，该设置也可能涉及一些镜头畸变校正

    \normalsize


`V4L2_CID_3A_LOCK (bitmask)`
    该控制锁定或解锁自动对焦、曝光和白平衡。通过将相应的锁位设为 1，可以独立地暂停自动调整。然后相机保留这些设置，直到锁位被清除。定义了以下锁位

    当某个给定算法未启用时，驱动应忽略锁定它的请求，并且不应返回错误。例如，
    `V4L2_CID_AUTO_WHITE_BALANCE` 控制设为 `FALSE` 时，应用程序设置
    `V4L2_LOCK_WHITE_BALANCE` 位。该控制的值可能被曝光、白平衡或对焦控制改变



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_LOCK_EXPOSURE`
      - 自动曝光调整锁
    - - `V4L2_LOCK_WHITE_BALANCE`
      - 自动白平衡调整锁
    - - `V4L2_LOCK_FOCUS`
      - 自动对焦锁



`V4L2_CID_PAN_SPEED (integer)`
    该控制以特定速度将相机水平转动。单位未定义。正值使相机向右移动（从上方看为顺时针），负值向左。零值停止正在进行的运动（如果有的话），否则无效果

`V4L2_CID_TILT_SPEED (integer)`
    该控制以指定速度将相机垂直转动。单位未定义。正值使相机向上移动，负值向下。零值停止正在进行的运动（如果有的话），否则无效果


`V4L2_CID_CAMERA_ORIENTATION (menu)`
    该只读控制通过报告相机所安装设备的安装位置来描述相机朝向。控制值是恒定的，不能被软件修改。该控制对于具有明确定义朝向的设备（例如手机、笔记本电脑和便携设备）特别有意义，因为该控制表示为相对于设备预期使用朝向的位置。例如，安装在手机、平板或笔记本电脑用户侧面的相机被称为具`V4L2_CAMERA_ORIENTATION_FRONT` 朝向，而安装在正面相反一侧的相机被称为具`V4L2_CAMERA_ORIENTATION_BACK` 朝向。未直接附接到设备、或以允许其自由移动的方式附接的相机传感器（例如网络摄像头和数码相机）被称为具有 `V4L2_CAMERA_ORIENTATION_EXTERNAL` 朝向



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_CAMERA_ORIENTATION_FRONT`
      - 相机朝向设备的用户侧面
    - - `V4L2_CAMERA_ORIENTATION_BACK`
      - 相机朝向设备的背面
    - - `V4L2_CAMERA_ORIENTATION_EXTERNAL`
      - 相机未直接附接到设备，且可自由移动



`V4L2_CID_CAMERA_SENSOR_ROTATION (integer)`
    该只读控制描述在图像捕获到内存后，为补偿相机传感器安装旋转而需以逆时针方向施加的旋转校正角度

    有关传感器安装旋转的精确定义，请参阅设备树绑定文'video-interfaces.txt' 中对 'rotation' 属性的详尽描述

    下面报告了几个示例，使用一条从左向右游动的鲨鱼
```

                 0               X-axis
               0 +------------------------------------->
                 !
                 !
                 !
                 !           |\____)\___
                 !           ) _____  __`<
                 !           |/     )/
                 !
                 !
                 !
                 V
               Y-axis

    Example one - Webcam

    Assuming you can bring your laptop with you while swimming with sharks,
    the camera module of the laptop is installed on the user facing part of a
    laptop screen casing, and is typically used for video calls. The captured
    images are meant to be displayed in landscape mode (width > height) on the
    laptop screen.

    The camera is typically mounted upside-down to compensate the lens optical
    inversion effect. In this case the value of the
    V4L2_CID_CAMERA_SENSOR_ROTATION control is 0, no rotation is required to
    display images correctly to the user.

    If the camera sensor is not mounted upside-down it is required to compensate
    the lens optical inversion effect and the value of the
    V4L2_CID_CAMERA_SENSOR_ROTATION control is 180 degrees, as images will
    result rotated when captured to memory. ::

                 +--------------------------------------+
                 !                                      !
                 !                                      !
                 !                                      !
                 !              __/(_____/|             !
                 !            >.___  ____ (             !
                 !                 \(    \|             !
                 !                                      !
                 !                                      !
                 !                                      !
                 +--------------------------------------+

    A software rotation correction of 180 degrees has to be applied to correctly
    display the image on the user screen. ::

                 +--------------------------------------+
                 !                                      !
                 !                                      !
                 !                                      !
                 !             |\____)\___              !
                 !             ) _____  __`<            !
                 !             |/     )/                !
                 !                                      !
                 !                                      !
                 !                                      !
                 +--------------------------------------+

    Example two - Phone camera

    It is more handy to go and swim with sharks with only your mobile phone
    with you and take pictures with the camera that is installed on the back
    side of the device, facing away from the user. The captured images are meant
    to be displayed in portrait mode (height > width) to match the device screen
    orientation and the device usage orientation used when taking the picture.

    The camera sensor is typically mounted with its pixel array longer side
    aligned to the device longer side, upside-down mounted to compensate for
    the lens optical inversion effect.

    The images once captured to memory will be rotated and the value of the
    V4L2_CID_CAMERA_SENSOR_ROTATION will report a 90 degree rotation. ::


                 +-------------------------------------+
                 |                 _ _                 |
                 |                \   /                |
                 |                 | |                 |
                 |                 | |                 |
                 |                 |  >                |
                 |                <  |                 |
                 |                 | |                 |
                 |                   .                 |
                 |                  V                  |
                 +-------------------------------------+

    A correction of 90 degrees in counter-clockwise direction has to be
    applied to correctly display the image in portrait mode on the device
    screen. ::

                          +--------------------+
                          |                    |
                          |                    |
                          |                    |
                          |                    |
                          |                    |
                          |                    |
                          |   |\____)\___      |
                          |   ) _____  __`<    |
                          |   |/     )/        |
                          |                    |
                          |                    |
                          |                    |
                          |                    |
                          |                    |
                          +--------------------+


```
   该控制未来可能会改为菜单控制，如果需要更多选项的话

`V4L2_CID_HDR_SENSOR_MODE (menu)`
    更改传感HDR 模式。HDR 图像是通过使用两个不同的曝光周期合并同一场景的两次捕获获得的。HDR 模式描述了这两个捕获在传感器中合并的方式

    由于每种传感器的模式不同，菜单项不由该控制标准化，而留给编程者决定
