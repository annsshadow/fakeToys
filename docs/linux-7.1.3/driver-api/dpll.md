
## Linux 鍐呮牳 dpll 瀛愮郴缁。

## DPLL


PLL - Phase Locked Loop（锁相环）是一种电子电路，用于将设备的时钟信号外部时钟信号同步。它有效地使设备能够按照 PLL 输入所提供的相同时钟节拍运行
DPLL - Digital Phase Locked Loop（数字锁相环）是一种集成电路，除了普PLL
的行为外，还包含一个数字鉴相器，并且可能在环路中带有数字分频器。因此，DPLL
输入和输出上的频率可能是可配置的
## 瀛愮郴缁。

dpll 子系统的主要目的是提供一个通用接口，用于配置那些使用任何一类数PLL、并
可以使用不同来源的输入信号进行同步、以及具有不同类型输出的设备其主要接口是建立NETLINK_GENERIC 之上的协议，并定义了一个事件监视多播组
## 设备对象


单个 dpll 设备对象表示单个数字 PLL 电路以及一组相连的引脚它会响应用户netlink 命令 `DPLL_CMD_DEVICE_GET` `do` 请求，报告所支持操作模式和当前状态；并通过同一命令`dump` netlink 请求，列出子系统中已注册dpll 列表更改 dpll 设备的配置是通过 netlink `DPLL_CMD_DEVICE_SET` 命令`do` 请求完成的设备句柄`DPLL_A_ID`，在获取或设置系统中特定设备的配置时必须提供它。它可以通过
`DPLL_CMD_DEVICE_GET` `dump` 请求，或`DPLL_CMD_DEVICE_ID_GET` `do` 请求
获得，在后者中必须提供能够唯一匹配单个设备的属性
## 引脚对象


引脚（pin）是一个形态不固定的对象，表示输入或输出，它可以是设备的内部组件，
也可以是外部连接的每个 dpll 的引脚数量各不相同，但单dpll 设备通常应提供多个引脚引脚的属性、能力和状态会在响netlink `DPLL_CMD_PIN_GET` 命令`do` 请求时提供给
用户也可以通过 `DPLL_CMD_PIN_GET` 命令`dump` 请求列出系统中注册的所有引脚引脚的配置可以通过 netlink `DPLL_CMD_PIN_SET` 命令`do` 请求更改引脚句柄`DPLL_A_PIN_ID`，在获取或设置系统中特定引脚的配置时必须提供它。它可以通过
`DPLL_CMD_PIN_GET` `dump` 请求`DPLL_CMD_PIN_ID_GET` `do` 请求获得，在这两种方式中
用户需提供能够唯一匹配单个引脚的属性
## 引脚选择


一般而言，被选中的引脚（即其信号驱动 dpll 设备的引脚）可以`DPLL_A_PIN_STATE`
属性获得，并且对于任何 dpll 设备，只能有一个引脚处`DPLL_PIN_STATE_CONNECTED`
状态
引脚选择可以手动或自动完成，取决于硬件能力和当前 dpll 设备的工作模（`DPLL_A_MODE` 属性）。其结果是，每种模式在可用引脚状态方面，以及用户可以请求dpll 设备状态方面，都存在差异
在手动模式（`DPLL_MODE_MANUAL`）下，用户可以请求或接收以下引脚状态之一
- `DPLL_PIN_STATE_CONNECTED` - 该引脚用于驱dpll 设备
- `DPLL_PIN_STATE_DISCONNECTED` - 该引脚不用于驱动 dpll 设备

在自动模式（`DPLL_MODE_AUTOMATIC`）下，用户可以请求或接收以下引脚状态之一
- `DPLL_PIN_STATE_SELECTABLE` - 该引脚应被视为自动选择算法的有效输- `DPLL_PIN_STATE_DISCONNECTED` - 该引脚不应被视为自动选择算法的有效输
在自动模式（`DPLL_MODE_AUTOMATIC`）下，用户只能在自动选择算法将某个输入锁定到
dpll 设备之后，接收到引脚状`DPLL_PIN_STATE_CONNECTED`
## 共享引脚


单个引脚对象可以附加到多dpll 设备这时有两组配置旋钮：

1) 在引脚上设置 - 该配置影响引脚注册到的所dpll 设备（即 `DPLL_A_PIN_FREQUENCY`），
2) 在引dpll 元组上设- 该配置只影响被选中dpll 设备（即 `DPLL_A_PIN_PRIO`   `DPLL_A_PIN_STATE`、`DPLL_A_PIN_DIRECTION`）
## MUX 鍨嬪紩鑴。

一个引脚可以是 MUX 型（多路复用）的，它聚合子引脚并充当引脚多路复用器。一个或多个
引脚MUX 型注册，而不是直接注册到某个 dpll 设备MUX 型引脚注册的引脚，对它们注册到的每个父引脚，会向用户提供额外的嵌套属`DPLL_A_PIN_PARENT_PIN`如果一个引脚注册了多个父引脚，它们的行为就像一个多输出多路复用器。这种情况下
`DPLL_CMD_PIN_GET` 的输出中将包含多pin-parent 嵌套
```

        'pin': [{{
          'clock-id': 282574471561216,
          'module-name': 'ice',
          'capabilities': 4,
          'id': 13,
          'parent-pin': [
          {'parent-id': 2, 'state': 'connected'},
          {'parent-id': 3, 'state': 'disconnected'}
          ],
          'type': 'synce-eth-port'
          }}]

```
同一时刻只有一个子引脚能将其信号提供给MUX 型引脚，选择是通过在期望的父引脚上
请求更改某个子引脚状态来完成的，使用 `DPLL_A_PIN_PARENT` 嵌套属性。`set state on
parent pin`（在父引脚上设置状态）消息格式的示例：

  ========================== =============================================
  `DPLL_A_PIN_ID`          child pin id（子引脚 id  `DPLL_A_PIN_PARENT_PIN`  用于请求与父引脚相关配置的嵌套属                             related to parent pin
    `DPLL_A_PIN_PARENT_ID` parent pin id（父引脚 id    `DPLL_A_PIN_STATE`     在父引脚上请求的引脚状  ========================== =============================================

## 引脚优先

某些设备可能提供自动引脚选择模式的能力（`DPLL_A_MODE` 属性的枚举`DPLL_MODE_AUTOMATIC`）。通常，自动选择是在硬件层面执行的，这意味着只有直接连接dpll 的引脚才能用于自动输入引脚选择在自动选择模式下，用户不能手动选择设备的输入引脚，而是应当为所有直接连接的引脚
提供优先`DPLL_A_PIN_PRIO`，设备会挑选优先级最高的有效信号并用它来控制 DPLL
设备。`set priority on parent pin`（在父引脚上设置优先级）消息格式的示例：

  ============================ =============================================
  `DPLL_A_PIN_ID`            配置的引id
  `DPLL_A_PIN_PARENT_DEVICE` 用于请求与父 dpll 设备相关配置的嵌套属                               related to parent dpll device
    `DPLL_A_PIN_PARENT_ID`   dpll 设备 id
    `DPLL_A_PIN_PRIO`        在父 dpll 上请求的引脚优先  ============================ =============================================

MUX 型引脚的子引脚不具备自动输入引脚选择能力，为了配MUX 型引脚的活动输入用户需要像 `MUX 型引脚` 一章所描述的那样，在父引脚上请求子引脚的期望状态
## 相位偏移测量与调

设备可能提供测量引脚与其dpll 设备之间信号相位差的能力。如果支持引dpll 相位偏移
测量，应为每个父 dpll 设备提供 `DPLL_A_PIN_PHASE_OFFSET` 属性。报告的相位偏移可以先前值和当前测量的平均值来计算，公式如下：

   curr\_avg = prev\_avg ** \frac{2^N-1}{2^N} + new\_val ** \frac{1}{2^N}

其中 `curr_avg` 是当前报告的相位偏移，`prev_avg` 是先前报告的值，`new_val` 是当测量值，`N` 是平均因子。配置的平均因子值通过设备`DPLL_A_PHASE_OFFSET_AVG_FACTOR`
属性提供，可以使用相同的属性配`DPLL_CMD_DEVICE_SET` 命令请求更改其值
  ================================== ======================================
  `DPLL_A_PHASE_OFFSET_AVG_FACTOR` 配置的相位偏移平均因子                                     attr configured value of phase offset
                                     averaging factor
  ================================== ======================================

设备也可能提供调整引脚上信号相位的能力。如果支持引脚相位调整，则引脚句柄的最小、最值以及粒度应`DPLL_CMD_PIN_GET` 的响应中通过 `DPLL_A_PIN_PHASE_ADJUST_MIN``DPLL_A_PIN_PHASE_ADJUST_MAX` `DPLL_A_PIN_PHASE_ADJUST_GRAN` 属性提供给用户配置的相位调整值通过引脚`DPLL_A_PIN_PHASE_ADJUST` 属性提供，可以使用相同的属配合 `DPLL_CMD_PIN_SET` 命令请求更改其值
  ================================ ==========================================
  `DPLL_A_PIN_ID`                配置的引id
  `DPLL_A_PIN_PHASE_ADJUST_GRAN` 相位调整值的粒度属  `DPLL_A_PIN_PHASE_ADJUST_MIN`  相位调整的最小值属  `DPLL_A_PIN_PHASE_ADJUST_MAX`  相位调整的最大值属  `DPLL_A_PIN_PHASE_ADJUST`      在父 dpll 设备上配置的相位调整值属                                   adjustment on parent dpll device
  `DPLL_A_PIN_PARENT_DEVICE`     用于请求给定dpll 设备配置的嵌套属                                   configuration on given parent dpll
                                   device
    `DPLL_A_PIN_PARENT_ID`       dpll 设备 id
    `DPLL_A_PIN_PHASE_OFFSET`    测量的引脚与dpll 设备之间的相位差属                                   between a pin and parent dpll device
  ================================ ==========================================

所有与相位相关的值都以皮秒（pico seconds）为单位，表示信号相位之间的时间差。负值表引脚上信号的相位早于 dpll 的信号。正值表示引脚上信号的相位晚dpll 的信号
相位调整（以及最小和最大值）是整数，但测量的相位偏移值是3 位小数的小数，应除以
`DPLL_PIN_PHASE_OFFSET_DIVIDER` 得到整数部分，并用取模除法得到小数部分
## 相位偏移监视

相位偏移测量通常针对当前活动源执行。然而，某些 DPLL（Digital Phase-Locked Loop，数锁相环）设备可能提供监视所有可用输入相位偏移的能力。对于支持的 DPLL 设备，该属性和
当前功能状态应包含`DPLL_CMD_DEVICE_GET` 命令的响应消息中。在这种情况下，用户也可通过 `DPLL_CMD_DEVICE_SET` 命令为该属性设`enum dpll_feature_state` 值来控制该功能一旦启用，输入的相位偏移测量值应`DPLL_A_PIN_PHASE_OFFSET` 属性中返回
  =============================== ========================
  `DPLL_A_PHASE_OFFSET_MONITOR` 功能的状态属  =============================== ========================

## 频率监视

某些 DPLL 设备可能提供测量所有可用输入引脚实际频率的能力。对于支持的 DPLL 设备，该属和当前功能状态应包含`DPLL_CMD_DEVICE_GET` 命令的响应消息中。在这种情况下，用户也可通过 `DPLL_CMD_DEVICE_SET` 命令为该属性设`enum dpll_feature_state` 值来控制该功能一旦启用，每个输入引脚的测量输入频率应`DPLL_A_PIN_MEASURED_FREQUENCY` 属性中返回。该以毫赫兹（mHz）为单位，使`DPLL_PIN_MEASURED_FREQUENCY_DIVIDER` 作为除数
  =============================== ========================
  `DPLL_A_FREQUENCY_MONITOR`    功能的状态属  =============================== ========================

## 嵌入SYNC


设备可能提供使用 Embedded SYNC（嵌入式同步）特性的能力。它允许将额外的 SYNC 信号嵌入引脚的基本频率中——每SYNC 信号脉冲发生时，嵌入一个基本频率信号的特殊脉冲。用户可配置 Embedded SYNC 的频率。Embedded SYNC 能力始终与给定的基本频率和硬件能力相关。根当前为引脚配置的基本频率，会向用户提供一组受支持Embedded SYNC 频率
  ========================================= =================================
  `DPLL_A_PIN_ESYNC_FREQUENCY`            当前 Embedded SYNC 频率
  `DPLL_A_PIN_ESYNC_FREQUENCY_SUPPORTED`  嵌套的可Embedded SYNC 频率范围
                                            frequency ranges
    `DPLL_A_PIN_FREQUENCY_MIN`            频率的最小值属    `DPLL_A_PIN_FREQUENCY_MAX`            频率的最大值属  `DPLL_A_PIN_ESYNC_PULSE`                Embedded SYNC 的脉冲类  ========================================= =================================

## 参SYNC


设备可能支持 Reference SYNC（参考同步）特性，它允许将两个输入组合成一个输入对。在这种
配置中，来自两个输入的时钟信号都用于同步 DPLL 设备。频率较高的信号用于 DPLL 的环带宽，而频率较低的信号用于DPLL 设备的输出信号同步。该特性使得能够从外部源提供高
质量的环路带宽信号
具备能力的输入会提供一份可与之绑定以创Reference SYNC 的输入列表。要控制此特性，用户
必须为目标引脚请求期望的状态：使用 `DPLL_PIN_STATE_CONNECTED` 启用，或使用
`DPLL_PIN_STATE_DISCONNECTED` 禁用该特性。一个输入引脚在任何给定时刻只能绑定到另一引脚
  ============================== ==========================================
  `DPLL_A_PIN_REFERENCE_SYNC`  用于提供信息或请求配Reference SYNC 特性的
                                 requesting configuration of the Reference
                                 SYNC feature
    `DPLL_A_PIN_ID`             Reference SYNC 特性的目标引脚 id
    `DPLL_A_PIN_STATE`          Reference SYNC 连接的状  ============================== ==========================================

## 閰嶇疆鍛戒护缁。

配置命令用于获取有关已注dpll 设备（和引脚）的信息，以及设置设备或引脚的配置由于 dpll 设备必须被抽象并反映真实硬件，因此无法从用户空间通过 netlink 添加新的 dpll
设备，每个设备都应由其驱动注册
所netlink 命令都需`GENL_ADMIN_PERM`。这是为了防止来自未授权用户空间应用的任垃圾信息/DoS 攻击
## 带有可能属性的 netlink 命令列表


标识 dpll 设备命令类型的常量使`DPLL_CMD_` 前缀，并根据命令用途使用后缀dpll 设备相关属性使`DPLL_A_` 前缀，并根据属性用途使用后缀
  ==================================== =================================
  `DPLL_CMD_DEVICE_ID_GET`           获取设备 ID 的命    `DPLL_A_MODULE_NAME`             注册者的模块名属    `DPLL_A_CLOCK_ID`                唯一时钟标识符属                                       (EUI-64), as defined by the
                                       IEEE 1588 standard
    `DPLL_A_TYPE`                    dpll 设备类型属  ==================================== =================================

  ==================================== =================================
  `DPLL_CMD_DEVICE_GET`              获取设备信息或转储可用设备列表的命令
                                       dump list of available devices
    `DPLL_A_ID`                      唯一 dpll 设备 ID 属    `DPLL_A_MODULE_NAME`             注册者的模块名属    `DPLL_A_CLOCK_ID`                唯一时钟标识符属                                       (EUI-64), as defined by the
                                       IEEE 1588 standard
    `DPLL_A_MODE`                    选择模式属    `DPLL_A_MODE_SUPPORTED`          可用选择模式属    `DPLL_A_LOCK_STATUS`             dpll 设备锁状态属    `DPLL_A_TEMP`                    设备温度信息属    `DPLL_A_TYPE`                    dpll 设备类型属  ==================================== =================================

  ==================================== =================================
  `DPLL_CMD_DEVICE_SET`              设置 dpll 设备配置的命    `DPLL_A_ID`                      内部 dpll 设备索引属    `DPLL_A_MODE`                    要配置的选择模式属  ==================================== =================================

标识引脚命令类型的常量使`DPLL_CMD_PIN_` 前缀，并根据命令用途使用后缀引脚相关属性使`DPLL_A_PIN_` 前缀，并根据属性用途使用后缀
  ==================================== =================================
  `DPLL_CMD_PIN_ID_GET`              获取引脚 ID 的命    `DPLL_A_PIN_MODULE_NAME`         注册者的模块名属    `DPLL_A_PIN_CLOCK_ID`            唯一时钟标识符属                                       (EUI-64), as defined by the
                                       IEEE 1588 standard
    `DPLL_A_PIN_BOARD_LABEL`         注册者提供的引脚板标签属                                       by registerer
    `DPLL_A_PIN_PANEL_LABEL`         注册者提供的引脚面板标签属                                       by registerer
    `DPLL_A_PIN_PACKAGE_LABEL`       注册者提供的引脚封装标签属                                       by registerer
    `DPLL_A_PIN_TYPE`                引脚类型属  ==================================== =================================

  ==================================== ==================================
  `DPLL_CMD_PIN_GET`                 获取引脚信息或转储可用引脚列表的命令
                                       list of available pins
    `DPLL_A_PIN_ID`                  唯一引脚 ID 属    `DPLL_A_PIN_MODULE_NAME`         注册者的模块名属    `DPLL_A_PIN_CLOCK_ID`            唯一时钟标识符属                                       (EUI-64), as defined by the
                                       IEEE 1588 standard
    `DPLL_A_PIN_BOARD_LABEL`         注册者提供的引脚板标签属                                       by registerer
    `DPLL_A_PIN_PANEL_LABEL`         注册者提供的引脚面板标签属                                       by registerer
    `DPLL_A_PIN_PACKAGE_LABEL`       注册者提供的引脚封装标签属                                       by registerer
    `DPLL_A_PIN_TYPE`                引脚类型属    `DPLL_A_PIN_FREQUENCY`           引脚的当前频率属    `DPLL_A_PIN_FREQUENCY_SUPPORTED` 提供受支持频率的嵌套属                                       frequencies
      `DPLL_A_PIN_ANY_FREQUENCY_MIN` 频率的最小值属      `DPLL_A_PIN_ANY_FREQUENCY_MAX` 频率的最大值属    `DPLL_A_PIN_PHASE_ADJUST_GRAN`   相位调整值的粒度属                                       adjustment value
    `DPLL_A_PIN_PHASE_ADJUST_MIN`    相位调整的最小值属                                       adjustment
    `DPLL_A_PIN_PHASE_ADJUST_MAX`    相位调整的最大值属                                       adjustment
    `DPLL_A_PIN_PHASE_ADJUST`        在父设备上配置的相位调整值属                                       adjustment on parent device
    `DPLL_A_PIN_PARENT_DEVICE`       引脚所连接的每个父设备的嵌套属                                       the pin is connected with
      `DPLL_A_PIN_PARENT_ID`         dpll 设备 id 属      `DPLL_A_PIN_PRIO`              引脚dpll 设备上的优先级属                                       dpll device
      `DPLL_A_PIN_STATE`             引脚在父 dpll 设备上的状态属                                       dpll device
      `DPLL_A_PIN_DIRECTION`         引脚在父 dpll 设备上的方向属                                       dpll device
      `DPLL_A_PIN_PHASE_OFFSET`      引脚与父 dpll 之间测量的相位差属                                       between a pin and parent dpll
    `DPLL_A_PIN_PARENT_PIN`          引脚所连接的每个父引脚的嵌套属                                       the pin is connected with
      `DPLL_A_PIN_PARENT_ID`         父引id 属      `DPLL_A_PIN_STATE`             引脚在父引脚上的状态属                                       pin
    `DPLL_A_PIN_CAPABILITIES`        引脚能力位掩码属    `DPLL_A_PIN_MEASURED_FREQUENCY`  mHz 为单位的输入引脚测量频率属                                       an input pin in mHz
  ==================================== ==================================

  ==================================== =================================
  `DPLL_CMD_PIN_SET`                 设置引脚配置的命    `DPLL_A_PIN_ID`                  唯一引脚 ID 属    `DPLL_A_PIN_FREQUENCY`           请求的引脚频率属    `DPLL_A_PIN_PHASE_ADJUST`        在父设备上请求的相位调整值属                                       adjustment on parent device
    `DPLL_A_PIN_PARENT_DEVICE`       每个dpll 设备配置请求的嵌套属                                       device configuration request
      `DPLL_A_PIN_PARENT_ID`         dpll 设备 id 属      `DPLL_A_PIN_DIRECTION`         请求的引脚方向属      `DPLL_A_PIN_PRIO`              dpll 设备上请求的引脚优先级属                                       the dpll device
      `DPLL_A_PIN_STATE`             dpll 设备上请求的引脚状态属                                       the dpll device
    `DPLL_A_PIN_PARENT_PIN`          每个父引脚配置请求的嵌套属                                       configuration request
      `DPLL_A_PIN_PARENT_ID`         父引id 属      `DPLL_A_PIN_STATE`             在父引脚上请求的引脚状态属                                       parent pin
  ==================================== =================================

## Netlink dump 请求


`DPLL_CMD_DEVICE_GET` `DPLL_CMD_PIN_GET` 命令能够进行 dump 类型netlink 请求这种情况下响应的格式与它们的 `do` 请求相同，但会返回系统中注册的每个设备或引脚
## SET 命令格式


`DPLL_CMD_DEVICE_SET` - 为了定位一dpll 设备，用户提`DPLL_A_ID`，它是系统中
dpll 设备的唯一标识符，以及正在配置的参数（`DPLL_A_MODE`）
`DPLL_CMD_PIN_SET` - 为了定位一个引脚，用户必须提供 `DPLL_A_PIN_ID`，它是系统中引脚唯一标识符。同时必须添加已配置的引脚参数如果配置`DPLL_A_PIN_FREQUENCY`，这会影响与该引脚相连的所dpll 设备，因此频属性不应被包含`DPLL_A_PIN_PARENT_DEVICE` 中其它属性：`DPLL_A_PIN_PRIO`、`DPLL_A_PIN_STATE` `DPLL_A_PIN_DIRECTION` 必须被包含在
`DPLL_A_PIN_PARENT_DEVICE` 中，因为它们的配置只与由 `DPLL_A_PIN_PARENT_ID` 属性定位的
某一个父 dpll 相关，而该属性也是该嵌套中所必需的对于 MUX 型引脚，`DPLL_A_PIN_STATE` 属性的配置方式类似，即将所需状态包含在
`DPLL_A_PIN_PARENT_PIN` 嵌套属性中，并将目标父引脚 id 放在 `DPLL_A_PIN_PARENT_ID` 中
一般而言，可以一次性配置多个参数，但在内部每个参数更改都会单独调用，配置顺序无法以
任何方式保证
## 配置预定义枚


## 通知


dpll 设备可以提供有关设备状态变化的通知，即锁状态变化、输输出变化或其它告警有一个多播组用于通过 netlink 套接字通知用户空间应用：`DPLL_MCGRP_MONITOR`

通知消息
  ============================== =====================================
  `DPLL_CMD_DEVICE_CREATE_NTF` dpll 设备已创  `DPLL_CMD_DEVICE_DELETE_NTF` dpll 设备已删  `DPLL_CMD_DEVICE_CHANGE_NTF` dpll 设备已改  `DPLL_CMD_PIN_CREATE_NTF`    dpll 引脚已创  `DPLL_CMD_PIN_DELETE_NTF`    dpll 引脚已删  `DPLL_CMD_PIN_CHANGE_NTF`    dpll 引脚已改  ============================== =====================================

事件格式与相应的 get 命令相同`DPLL_CMD_DEVICE_` 事件的格式与 `DPLL_CMD_DEVICE_GET` 的响应相同`DPLL_CMD_PIN_` 事件的格式与 `DPLL_CMD_PIN_GET` 的响应相同
## 设备驱动实现


设备通过 dpll_device_get() 调用分配。使用相同参数的第二次调用不会创建新对象，而是提供
指向给定参数先前所创设备的指针，同时增加该对象的引用计数设备通过 dpll_device_put() 调用释放，它首先减少引用计数，一旦引用计数清零，该对象即销毁
设备应实现一组操作，并通过 dpll_device_register() 注册设备，此时它对用户可用。多驱动实例可以通过 dpll_device_get() 获取对它的引用，也可以用它们自己ops priv 注册
dpll 设备
引脚通过 dpll_pin_get() 单独分配，其工作方式类似dpll_device_get()。该函数首先创建
对象，然后对于每次使用相同参数的调用，只增加对象的引用计数。dpll_pin_put() 的工作方也类似于 dpll_device_put()
一个引脚可以根据硬件需要，注册到父 dpll 设备或父引脚。每次注册都要求注册者提供一引脚回调，以及用于调用它们的私有数据指针
- dpll_pin_register() - 将引脚注册到一dpll 设备- dpll_pin_on_pin_register() - 将引脚注册到另一MUX 型引脚
添加或移dpll 设备的通知是在子系统内部创建的注册/注销引脚的通知也由子系统调用有关 dpll 设备或引脚状态变化的通知以两种方式调用：

- dpll 子系统上成功请求更改后，子系统调用相应的通知- 由设备驱动通过 dpll_device_change_ntf() dpll_pin_change_ntf() 请求，当驱动报告状  变化时
使用 dpll 接口的设备驱动不要求实现所有的回调操作。不过，有少数几个是必须实现的dpll 设备级别必需的回调操作：

- `.mode_get`锛?- `.lock_status_get`銆。
引脚级别必需的回调操作：

- `.state_on_dpll_get`（注册到 dpll 设备的引脚）- `.state_on_pin_get`（注册到父引脚的引脚），
- `.direction_get`銆。
每个其它操作处理程序都会检查是否存在，若特定处理程序缺失则返回 `-EOPNOTSUPP`
最简单的实现OCP TimeCard 驱动中。ops 结构定义如下

	static const struct dpll_device_ops dpll_ops = {
		.lock_status_get = ptp_ocp_dpll_lock_status_get,
		.mode_get = ptp_ocp_dpll_mode_get,
		.mode_supported = ptp_ocp_dpll_mode_supported,
	};

	static const struct dpll_pin_ops dpll_pins_ops = {
		.frequency_get = ptp_ocp_dpll_frequency_get,
		.frequency_set = ptp_ocp_dpll_frequency_set,
		.direction_get = ptp_ocp_dpll_direction_get,
		.direction_set = ptp_ocp_dpll_direction_set,
		.state_on_dpll_get = ptp_ocp_dpll_state_get,
	};

注册部分看起来像这样

        clkid = pci_get_dsn(pdev);
        bp->dpll = dpll_device_get(clkid, 0, THIS_MODULE);
        if (IS_ERR(bp->dpll)) {
                err = PTR_ERR(bp->dpll);
                dev_err(&pdev->dev, "dpll_device_alloc failed\n");
                goto out;
        }

        err = dpll_device_register(bp->dpll, DPLL_TYPE_PPS, &dpll_ops, bp);
        if (err)
                goto out;

        for (i = 0; i < OCP_SMA_NUM; i++) {
                bp->sma[i].dpll_pin = dpll_pin_get(clkid, i, THIS_MODULE, &bp->sma[i].dpll_prop);
                if (IS_ERR(bp->sma[i].dpll_pin)) {
                        err = PTR_ERR(bp->dpll);
                        goto out_dpll;
                }

                err = dpll_pin_register(bp->dpll, bp->sma[i].dpll_pin, &dpll_pins_ops,
                                        &bp->sma[i]);
                if (err) {
                        dpll_pin_put(bp->sma[i].dpll_pin);
                        goto out_dpll;
                }
        }

在错误路径中，我们必须以相反的顺序回退每一次分配：


        while (i) {
                --i;
                dpll_pin_unregister(bp->dpll, bp->sma[i].dpll_pin, &dpll_pins_ops, &bp->sma[i]);
                dpll_pin_put(bp->sma[i].dpll_pin);
        }
        dpll_device_put(bp->dpll);

更复杂的示例可以Intel ICE 驱动nVidia mlx5 驱动中找到
## SyncE 启用


为了启用 SyncE，需要允许一个软件应用控dpll 设备，该应用监视并配dpll 设备的输入，
以响dpll 设备及其输入的当前状态在这种场景下，dpll 设备的输入信号也应当是可配置的，以便用从 PHY netdevice 恢复出的信号
驱动 dpll。这是通过将一个引脚暴露给 netdevice——把引脚附加netdevice 本身来实现，使用
`dpll_netdev_pin_set(struct net_device **dev, struct dpll_pin **dpll_pin)`暴露的引id 句柄 `DPLL_A_PIN_ID` 之后可由用户识别，因为它附加rtnetlink `RTM_NEWLINK` 命令的响应中的嵌套属`IFLA_DPLL_PIN` 上