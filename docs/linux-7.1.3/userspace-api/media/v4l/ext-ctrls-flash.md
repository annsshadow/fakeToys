

######## 闪光灯控制参

V4L2 闪光灯控制旨在提供对闪光灯控制器设备的通用访问。闪光灯控制器设备通常用于
数码相机
该接口可以同时支LED 和氙气闪光灯设备。在撰写本文时，还没有使用此接口的氙闪光灯驱动

## 支持的用

### 非同LED 闪光灯（软件频闪

非同LED 闪光灯由主机像传感器一样直接控制。闪光灯必须在图像曝光开始前由主启用，并在曝光结束后禁用。主机完全负责闪光灯的时序
此类设备的例子：Nokia N900

### 同步 LED 闪光灯（硬件频闪

同步 LED 闪光灯由主机预先编程（功率和超时），但由传感器通过从传感器到闪光灯频闪信号控制
传感器控制闪光灯的持续时间和时序。该信息通常必须提供给传感器

### 作为手电筒的 LED 闪光

LED 闪光灯可以作为手电筒使用，既可以结合涉及相机的其它用例，也可以单独使用

### 闪光灯控ID


`V4L2_CID_FLASH_CLASS (class)`
    FLASH 类描述符

`V4L2_CID_FLASH_LED_MODE (menu)`
    定义闪光LED 的模式，即连接到闪光灯控制器的高功率白色 LED。在存在某些故障    可能无法设置此控制。参V4L2_CID_FLASH_FAULT

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_FLASH_LED_MODE_NONE`
      - 关闭    - - `V4L2_FLASH_LED_MODE_FLASH`
      - 闪光模式    - - `V4L2_FLASH_LED_MODE_TORCH`
      - 手电筒模式
        参见 V4L2_CID_FLASH_TORCH_INTENSITY


`V4L2_CID_FLASH_STROBE_SOURCE (menu)`
    定义闪光LED 频闪的源

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_FLASH_STROBE_SOURCE_SOFTWARE`
      - 闪光灯频闪通过使用
	V4L2_CID_FLASH_STROBE 控制触发    - - `V4L2_FLASH_STROBE_SOURCE_EXTERNAL`
      - 闪光灯频闪由外部源触发。通常
	这是一个传感器，这使得可以同步闪光灯频闪的起始与曝光的起始        这种控制闪光LED 频闪的方法有两个额外的先决条件：频闪源的 :ref:`频闪
        输出 <v4l2-cid-flash-strobe-oe>` 必须启用（如果可用），并且闪光灯控制器的
        :ref:`闪光LED 模式 <v4l2-cid-flash-led-mode>` 必须设置        `V4L2_FLASH_LED_MODE_FLASH`


`V4L2_CID_FLASH_STROBE (button)`
    频闪闪光。当 V4L2_CID_FLASH_LED_MODE 设置V4L2_FLASH_LED_MODE_FLASH     V4L2_CID_FLASH_STROBE_SOURCE 设置V4L2_FLASH_STROBE_SOURCE_SOFTWARE     有效。在存在某些故障时可能无法设置此控制。参V4L2_CID_FLASH_FAULT
`V4L2_CID_FLASH_STROBE_STOP (button)`
    立即停止闪光灯频闪
`V4L2_CID_FLASH_STROBE_STATUS (boolean)`
    频闪状态：闪光灯此刻是否正在频闪。这是一个只读控制
`V4L2_CID_FLASH_TIMEOUT (integer)`
    闪光灯的硬件超时。从频闪开始起经过这段时长后，闪光灯频闪会停止
`V4L2_CID_FLASH_INTENSITY (integer)`
    闪光LED 处于闪光模式（V4L2_FLASH_LED_MODE_FLASH）时闪光灯频闪的强度。单    应尽可能为毫安（mA）
`V4L2_CID_FLASH_TORCH_INTENSITY (integer)`
    手电筒模式下闪光LED 的强度（V4L2_FLASH_LED_MODE_TORCH）。单位应尽可能为毫安
    （mA）。在存在某些故障时可能无法设置此控制。参V4L2_CID_FLASH_FAULT
`V4L2_CID_FLASH_INDICATOR_INTENSITY (integer)`
    指示LED 的强度。指示灯 LED 可以完全独立于闪光灯 LED。单位应尽可能为微安（uA）
`V4L2_CID_FLASH_FAULT (bitmask)`
    与闪光灯相关的故障。故障说明闪光灯芯片本身或其附带LED 中的特定问题。故障可    阻止进一步使用某些闪光灯控制。特别是，如果故障影响到闪光LED，则
    V4L2_CID_FLASH_LED_MODE 会被设置V4L2_FLASH_LED_MODE_NONE。究竟哪些故障有此类
    效果取决于芯片。读取故障会重置该控制，并尽可能使芯片回到可用状态

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_FLASH_FAULT_OVER_VOLTAGE`
      - 闪光灯控制器到闪光灯 LED 的电压已超过闪光灯控制器特定的限制    - - `V4L2_FLASH_FAULT_TIMEOUT`
      - 当用户设置的超时（V4L2_CID_FLASH_TIMEOUT 控制）已过期时，闪光灯频闪仍	处于开启状态。并非所有闪光灯控制器都会在所有的此类条件下设置此标志    - - `V4L2_FLASH_FAULT_OVER_TEMPERATURE`
      - 闪光灯控制器过热    - - `V4L2_FLASH_FAULT_SHORT_CIRCUIT`
      - 闪光灯控制器的短路保护已被触发    - - `V4L2_FLASH_FAULT_OVER_CURRENT`
      - LED 电源中的电流已超过闪光灯控制器特定的限制    - - `V4L2_FLASH_FAULT_INDICATOR`
      - 闪光灯控制器在指示灯 LED 上检测到短路或开路状态    - - `V4L2_FLASH_FAULT_UNDER_VOLTAGE`
      - 闪光灯控制器到闪光灯 LED 的电压低于闪光灯控制器特定的最小限制    - - `V4L2_FLASH_FAULT_INPUT_VOLTAGE`
      - 闪光灯控制器的输入电压低于无法以全电流进行频闪的限制。该状态会一直持续到
	此标志不再被设置    - - `V4L2_FLASH_FAULT_LED_OVER_TEMPERATURE`
      - LED 的温度已超过其允许的上限

`V4L2_CID_FLASH_CHARGE (boolean)`
    启用或禁用氙气闪光灯电容的充电
`V4L2_CID_FLASH_READY (boolean)`
    闪光灯是否准备好频闪？氙气闪光灯需要在频闪前将其电容充电。LED 闪光灯通常需要在
    频闪后有一段冷却期，在此期间无法进行另一次频闪。这是一个只读控制

`V4L2_CID_FLASH_DURATION (integer)`
    使用外部频闪时，由频闪源生成的闪光灯频闪脉冲的持续时间。此控制应由生成硬件闪光    频闪信号的设备实现，通常是连接到闪光灯控制器的相机传感器
    闪光灯控制器的频闪源 <v4l2-cid-flash-strobe-source> 必须为这种操作模式配置为
    `V4L2_FLASH_STROBE_SOURCE_EXTERNAL`。更多细节也请查看那里的文档
    单位应尽可能为微秒（µs）

`V4L2_CID_FLASH_STROBE_OE (boolean)`
    使用外部频闪时，启用来自频闪源的硬件频闪信号的输出。此控制应由生成硬件闪光    频闪信号的设备实现，通常是连接到闪光灯控制器的相机传感器
    在信号生成设备驱动支持的前提下，频闪信号的长度可以通过调整    闪光灯持续时<v4l2-cid-flash-duration> 来配置
    闪光灯控制器的频闪源 <v4l2-cid-flash-strobe-source> 必须为这种操作模式配置为
    `V4L2_FLASH_STROBE_SOURCE_EXTERNAL`。更多细节也请查看那里的文档