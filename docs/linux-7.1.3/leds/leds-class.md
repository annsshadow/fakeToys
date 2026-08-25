## Linux 下的 LED 处理

在其最简单的形式中，LED 类（LED class）只是允许从用户空间控制 LED。LED 出现/sys/class/leds/ 中。LED 的最大亮度在 max_brightness 文件中定义。brightness 文件用于设置 LED 的亮度（取0 max_brightness）。大多数 LED 没有硬件亮度支持，因此只要亮度设置为非零就会被点亮
该类还引入了 LED 触发器（LED trigger）这一可选概念。触发器是一种基于内核的 LED 事件来源。触发器可以是简单的，也可以是复杂的。简单触发器不可配置，设计目标是用最少的额外代码插入到现有子系统中。例disk-activity、nand-disk sharpsl-charge 触发器。在禁用 LED 触发器的情况下，相关代码会被优化掉
复杂触发器虽然对所LED 都可用，但具LED 特定的参数，并且是针对单LED 工作的。timer 触发器就是一个例子。timer 触发器会周期性地LED_OFF 与当前亮度设置之间改LED 亮度on" "off" 的时间可以通过 /sys/class/leds/<device>/delay_{on,off} 以毫秒为单位指定。你可以独立timer 触发器来改变 LED 的亮度值。不过，如果你把亮度值设置为 LED_OFF，它也会禁用 timer 触发器
你可以像选择 IO 调度器那样改变触发器（通过 /sys/class/leds/<device>/trigger）。一旦选定某个触发器，其特定参数就会出现在 /sys/class/leds/<device> 中
## 设计理念

底层的设计理念是简单。LED 是简单的设备，目标是用尽量少的代码提供尽可能多的功能。在提出增强建议时请谨记这一点
## LED 设备命名

目前采用如下形式
	"devicename:color:function"

- devicename:
        它应当引用由内核创建的唯一标识符，例如网络设备phyN 或输入设备的 inputN，而不是引用硬件本身；与产品以及该设备所挂载的总线相关的信息可sysfs 中获取，并可使用 tools/leds 下的 get_led_device_info.sh 脚本来检索；一般而言，这部分主要面向那些与某些其他设备相关联LED
- color:
        取头文件 include/dt-bindings/leds/common.h 中某LED_COLOR_ID_* 定义
- function:
        取头文件 include/dt-bindings/leds/common.h 中某LED_FUNCTION_* 定义
如果缺少所需color function，请linux-leds@vger.kernel.org 提交补丁
对于给定的平台，可能会需要多个具有相color function LED，它们仅以序号区分。在这种情况下，最好在驱动中将预定义的 LED_FUNCTION_* 名称与所需"-N" 后缀拼接。基fwnode 的驱动可以使function-enumerator 属性，随后该拼接将LED 类设备注册时LED 核心自动完成
LED 子系统还提供防止名称冲突的保护机制，这种情况可能发生在由可热插拔设备的驱动创LED 类设备、而又没有提供唯一 devicename 部分时。此时，会向所请求LED 类设备名称添加数字后缀（例"_1"_2"_3" 等）
可能仍有一LED 类驱动使用厂商或产品名称作为 devicename，但这种方式现在已废弃，因为它没有提供任何附加价值。产品信息可sysfs 的其他位置找到（参见 tools/leds/get_led_device_info.sh）
正确LED 名称示例
  - "red:disk"
  - "white:flash"
  - "red:indicator"
  - "phy1:green:wlan"
  - "phy3::wlan"
  - ":kbd_backlight"
  - "input5::kbd_backlight"
  - "input3::numlock"
  - "input3::scrolllock"
  - "input3::capslock"
  - "mmc1::status"
  - "white:status"

get_led_device_info.sh 脚本可用于验LED 名称是否满足此处提出的要求。它会对 LED 类设备的 devicename 部分执行校验，并在某部分校验失败时给出该部分期望值的提示。到目前为止，该脚本支持校验 LED 与以下类型设备之间的关联
        - 输入设备
        - 符合 ieee80211 规范USB 设备

该脚本可随时扩展
曾经有人呼吁color LED 属性作为独立的 led 类属性导出。作为不会带来过多开销的解决方案，我建议将这些属性纳入设备名称之中。上述命名方案为今后可能需要的更多属性留有空间。如果名称中的某些部分不适用，只需将该部分留空即可
## 亮度设置 API

LED 子系统核心提供以下用于设置亮度的 API
    - led_set_brightness:
		保证不会休眠，传LED_OFF 会停止闪烁，

    - led_set_brightness_sync:
		用于需要立即生效的场景——它可能会阻塞调用者，阻塞时间等于访问设备寄存器所需的时间，并且可能休眠，传LED_OFF 会停止硬件闪烁，如果启用了软件闪烁回退则返-EBUSY
## LED 注册 API

想要注册一个供其他驱动/用户空间使用LED classdev 的驱动，需要分配并填充一led_classdev 结构体，然后调用 `[devm_]led_classdev_register`。如果使用非 devm 版本，驱动必须在remove 函数中先调用 led_classdev_unregister，然后再释放 led_classdev 结构体
如果驱动能够检测到硬件发起的亮度变化，从而希望拥brightness_hw_changed 属性，则必须在注册前于 flags 中设LED_BRIGHT_HW_CHANGED 标志。对未使LED_BRIGHT_HW_CHANGED 标志注册classdev 调用 led_classdev_notify_brightness_hw_changed 属于 bug，会触发 WARN_ON
## LED 的硬件加速闪
某些 LED 可以被编程为在没有任CPU 参与的情况下闪烁。为支持此特性，LED 驱动可以选择实现 blink_set() 函数（参<linux/leds.h>）。不过，要将 LED 设置为闪烁，更好的做法是使用 API 函数 led_blink_set()，因为它会在必要时检查并实现软件回退
要关闭闪烁，可使API 函数 led_brightness_set()，传入亮度LED_OFF，这应当会停止任何可能因闪烁而需要的软件定时器
如果 blink_set() 函数`**delay_on==0` && `**delay_off==0` 参数被调用，它应当选择一个对用户友好的闪烁值。在这种情况下，驱动应当通过 delay_on delay_off 参数将所选值返回给 LEDs 子系统
通过 brightness_set() 回调将亮度设置为零，应当完全关闭 LED，并取消之前已编程的硬件闪烁函数（如果有的话）
## 硬件驱动LED

某些 LED 可以被编程为受硬件驱动。这并不仅限于闪烁，也包括自主地关闭或点亮。为支持此特性，LED 需要实现若干额外的操作（ops），并声明对所支持触发器的特定支持
所hw control（硬件控制），我们指的是由硬件驱动的 LED
LED 驱动必须定义以下值以支持硬件控制
    - hw_control_trigger:
                LED 在硬件控制模式下所支持的、唯一的触发器名称
LED 驱动必须实现以下 API 以支持硬件控制：
    - hw_control_is_supported:
                检查受支持触发器传入的 flags 是否可以被解析并激活该 LED 上的硬件控制
                如果传入flags 掩码受支持、且能用 hw_control_set() 设置，则返回 0
                如果传入flags 掩码不受支持，则必须返回 -EOPNOTSUPP，这种情况下 LED 触发器将使用软件回退
                如果因任何其他错误（如设备未就绪或超时）而返回负错误值
     - hw_control_set:
                激活硬件控制。LED 驱动将使用受支持触发器传入的 flags，将其解析为一组模式，并将 LED 配置为按照所请求的模式由硬件驱动
                通过 brightness_set 设置 LED_OFF 来停用硬件控制
                成功时返0，应flags 失败时返回负的错误号
    - hw_control_get:
                从已经处于硬件控制中LED 获取活动模式，进行解析，并在 flags 中设置受支持触发器当前的活动 flags
                成功时返0，解析初始模式失败时返回负的错误号                该函数的错误并非致命的，因为设备可能处于附加LED 触发器所不支持的初始状态
    - hw_control_get_device:
                返回硬件控制中与 LED 驱动相关联的设备。触发器可能用它来将该函数返回的设备与触发器配置的、作为闪烁事件来源的设备进行匹配，从而正确启用硬件控制                （例如，配置为针对某个特定网络设备闪烁的 netdev 触发器，用从 get_device 返回的设备来匹配，以设置硬件控制
                返回指向 struct device 的指针，如果当前没有附加任何设备则返NULL
LED 驱动可以默认启用额外的模式，以规避无法在受支持触发器上支持每种不同模式的问题。例如，将闪烁速度硬编码为固定间隔，或者在未满足某些要求时启用绕过闪烁等特殊功能
触发器应当首先检LED 驱动是否支持硬件控制 API，并检查该触发器是否受支持以确认能否进行硬件控制，使用 hw_control_is_supported 检flags 是否受支持，最后才使用 hw_control_set 激活硬件控制
触发器可以使hw_control_get 检查某LED 是否已经处于硬件控制中，并初始化flags
LED 处于硬件控制中时，无法进行软件闪烁，这样做会实际停用硬件控制
## 已知问题

LED 触发器核心不能编译为模块，因为简单触发器函数会引发棘手的依赖问题。与简单触发器功能所带来的好处相比，我认为这是一个小问题。LED 子系统的其余部分可以编译为模块