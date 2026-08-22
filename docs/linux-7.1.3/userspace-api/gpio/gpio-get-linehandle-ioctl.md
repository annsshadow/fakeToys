
######## GPIO_GET_LINEHANDLE_IOCTL


    ioctl 属于 chardev_v1.rst，并已被 gpio-v2-get-line-ioctl.rst 取代
## 名称


GPIO_GET_LINEHANDLE_IOCTL - 从内核请求一条或多条 GPIO 线
## 摘要


`int ioctl(int chip_fd, GPIO_GET_LINEHANDLE_IOCTL, struct gpiohandle_request *request)`

## 参数


`chip_fd`
    `open()` 返回GPIO 字符设备的文件描述符
`request`
    指定要请求的线路及其配置`handle_request<gpiohandle_request>`
## 描述


从内核请求一条或多条 GPIO 线
虽然可以一次请求多条线，但相同的配置会应用于请求中的所有线路
成功后，请求进程被授予对该线值的独占访问权限，以及对线路配置的写访问权限
线的状态（包括输出线的值）保证保持为所请求的状态，直到返回的文件描述符被关闭一旦文件描述符被关闭，从用户空间的角度看，线的状态便不再受控，并可能恢复为其默认状态
请求一条已在使用中的线路会出错*EBUSY**）
关闭 `chip_fd` 对已有的线路句柄没有影响
### 配置规则


以下配置规则适用
方向标志 `GPIOHANDLE_REQUEST_INPUT` `GPIOHANDLE_REQUEST_OUTPUT` 不能组合如果两者都未设置，则唯一可以设置的另一个标志是 `GPIOHANDLE_REQUEST_ACTIVE_LOW`此时线路以“原样”请求，以便在不改变电气配置的情况下读取线路值
驱动标志 `GPIOHANDLE_REQUEST_OPEN_xxx` 需要设`GPIOHANDLE_REQUEST_OUTPUT`只能设置一个驱动标志。如果都未设置，则假定线路为推挽（push-pull）
只能设置一个偏置标`GPIOHANDLE_REQUEST_BIAS_xxx`，并且它还需要同时设置一个方向标志如果未设置任何偏置标志，则偏置配置不会改变
请求无效的配置会出错*EINVAL**）
### 配置支持


当底层硬件与驱动不直接支持所请求的配置时，内核采用以下方法之一
 - 拒绝该请 - 在软件中模拟该特 - 将特性作为尽力而为（best effort）处
所采用的方法取决于该特性是否能合理地在软件中模拟，以及若该特性不受支持时对硬件和
用户空间的影响。每种特性所采用的方法如下：

==============   ===========
特            方法
==============   ===========
Bias             尽力而为
Direction        拒绝
Drive            模拟
==============   ===========

Bias 被视为尽力而为，以便用户空间对支持内部偏置的平台与需要外部偏置的平台应用相同配置。最坏情况下线路会悬空，而不是如预期那样被偏置
Drive 通过在线路不应被驱动时将其切换为输入来模拟
在所有情况下，gpio-get-lineinfo-ioctl.rst 报告的配置都是所请求的配置，而不是最的硬件配置。用户空间无法判断某个特性是由硬件支持、被模拟，还是尽力而为
## 杩斿洖鍊。

成功时返0，且 `request.fd<gpiohandle_request>` 包含该请求的文件描述符
出错时返-1，并设置 `errno` 变量。常见错误码error-codes.rst 中描述