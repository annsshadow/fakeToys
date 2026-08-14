


######## GPIO_V2_GET_LINE_IOCTL


## Name


GPIO_V2_GET_LINE_IOCTL - 从内核请求一条或多条线（line）。

## Synopsis


`int ioctl(int chip_fd, GPIO_V2_GET_LINE_IOCTL, struct gpio_v2_line_request *request)`

## Arguments


`chip_fd`
    `open()` 返回的 GPIO 字符设备的文件描述符。

`request`
    指定要请求的线及其配置的 `line_request<gpio_v2_line_request>`。

## Description


成功时，请求进程被授予对该线值的独占访问、对线配置的写入访问，并且可以在线上检测到边沿时接收事件，所有这些在 gpio-v2-line-request 中有更详细的描述。

可以在一个线请求中请求多条线，内核会尽可能原子地对这些请求的线执行请求操作。例如，gpio-v2-line-get-values-ioctl.rst 会一次性读取所有请求的线。

一条线的状态（包括输出线的值）保证保持为所请求的状态，直到返回的文件描述符被关闭。一旦文件描述符被关闭，从该用户空间的角度来看，线的状态就变得不受控制，并可能恢复到其默认状态。

请求一条已被使用的线是一个错误（**EBUSY**）。

关闭 `chip_fd` 对现有的线请求没有影响。


### Configuration Rules


对于任何给定的请求线，适用以下配置规则：

方向标志 `GPIO_V2_LINE_FLAG_INPUT` 和 `GPIO_V2_LINE_FLAG_OUTPUT` 不能组合。如果两者都未设置，那么唯一可以设置的另一个标志是 `GPIO_V2_LINE_FLAG_ACTIVE_LOW`，并且该线以"原样"（as-is）请求，以便在不改变电气配置的情况下读取线值。

驱动标志 `GPIO_V2_LINE_FLAG_OPEN_xxx` 需要设置 `GPIO_V2_LINE_FLAG_OUTPUT`。只能设置一个驱动标志。如果都未设置，则该线假定为推挽（push-pull）。

只能设置一个偏置（bias）标志 `GPIO_V2_LINE_FLAG_BIAS_xxx`，并且它也需要设置一个方向标志。如果没有设置偏置标志，则偏置配置不会改变。

边沿标志 `GPIO_V2_LINE_FLAG_EDGE_xxx` 需要设置 `GPIO_V2_LINE_FLAG_INPUT`，并且可以组合以检测上升沿和下降沿。向不支持边沿检测的线请求边沿检测是一个错误（**ENXIO**）。

只能设置一个事件时钟标志 `GPIO_V2_LINE_FLAG_EVENT_CLOCK_xxx`。如果都未设置，事件时钟默认为 `CLOCK_MONOTONIC`。`GPIO_V2_LINE_FLAG_EVENT_CLOCK_HTE` 标志需要支持的硬件以及设置了 `CONFIG_HTE` 的内核。向不支持它的设备请求 HTE 是一个错误（**EOPNOTSUPP**）。

`debounce_period_us<gpio_v2_line_attribute>` 属性只能应用于设置了 `GPIO_V2_LINE_FLAG_INPUT` 的线。设置后，去抖（debounce）同时应用于 gpio-v2-line-get-values-ioctl.rst 返回的值以及 gpio-v2-line-event-read.rst 返回的边沿。如果硬件不直接支持，内核会在软件中模拟去抖。向既不支持硬件去抖也不支持中断（软件模拟所需）的线请求去抖是一个错误（**ENXIO**）。

请求无效配置是一个错误（**EINVAL**）。


### Configuration Support


当请求的底层硬件和驱动不直接支持该配置时，内核会采用以下方法之一：

 - 拒绝请求
 - 在软件中模拟该特性
 - 将该特性视为尽力而为（best effort）

采用哪种方法取决于该特性是否可以在软件中合理地模拟，以及如果不支持该特性对硬件和用户空间的影响。每个特性所采用的方法如下：

==============   ===========
Feature          Approach
==============   ===========
Bias             尽力而为
Debounce         模拟
Direction        拒绝
Drive            模拟
Edge Detection   拒绝
==============   ===========

Bias 被视为尽力而为，以允许用户空间对支持内部偏置的平台和需要外部偏置的平台应用相同的配置。最坏情况下，该线会浮空，而不是按预期被偏置。

Debounce 是通过对线上的硬件中断应用一个过滤器来模拟的。在检测到边沿且线在去抖周期内保持稳定后，会产生一个边沿事件。事件时间戳对应于去抖周期的结束。

Drive 是通过在该线不应被主动驱动时将其切换为输入来模拟的。

边沿检测需要中断支持，如果不支持则被拒绝。用户空间仍然可以通过轮询进行模拟。

在所有情况下，gpio-v2-get-lineinfo-ioctl.rst 报告的配置是所请求的配置，而不是最终的硬件配置。用户空间无法判断某个特性是由硬件支持、被模拟，还是尽力而为。

## Return Value


成功时返回 0，并且 `request.fd<gpio_v2_line_request>` 包含该请求的文件描述符。

出错时返回 -1，并且 `errno` 变量会被相应地设置。常见的错误码在 error-codes.rst 中描述。
