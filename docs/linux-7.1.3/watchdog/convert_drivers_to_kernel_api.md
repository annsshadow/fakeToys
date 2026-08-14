## 将旧的看门狗（watchdog）驱动转换到看门狗框架

作者：Wolfram Sang <wsa@kernel.org>

随着看门狗框架（watchdog framework）进入内核，过去每个驱动都自行实现 API 的局面已经改变。如今，框架已将公共组件抽取出来，驱动得以精简，用户可以直接使用框架。本文档将指导你完成这一转换工作，描述必要的步骤以及需要留意的地方。

### 移除 file_operations 结构体

旧的驱动会定义自己的 file_operations 操作，例如 open()、write() 等，而现在这些大多由框架处理，框架只在需要时调用驱动。因此，一般而言，'file_operations' 结构体及其相关函数可以移除，只有极少数驱动特定的细节需要移到相应函数中。下面概述各函数可能需要进行的操作：

- open：所有涉及资源管理（文件打开检查、magic close 的准备工作）的内容直接删除即可。设备特定的部分需要移到驱动的 start 函数中。对于某些驱动，start 函数同时也充当 ping 函数。如果需要 start/stop 保持平衡（尤其是涉及时钟时），最好将其重构为独立的 start 函数。
- release：与 open 相同的提示适用。
- write：直接删除即可，框架会负责定义好的行为，即处理写入 magic 字符（'V'）的 ping 操作。
- ioctl：驱动仍然允许扩展 IOCTL 接口，但最常见的那些已由框架处理，只需驱动提供协助：

  WDIOC_GETSUPPORT：返回驱动必须提供的 watchdog_info 结构体。
  WDIOC_GETSTATUS：需要定义 status 回调，否则返回 0。
  WDIOC_GETBOOTSTATUS：需要正确设置 bootstatus 成员。请确保不要因“不再支持”而错误地将 0 写入其中！
  WDIOC_SETOPTIONS：需要做一些准备工作。
  WDIOC_KEEPALIVE：如需要，watchdog_info 必须设置 WDIOF_KEEPALIVEPING 标志。
  WDIOC_SETTIMEOUT：watchdog_info 需要设置 WDIOF_SETTIMEOUT 标志，并定义 set_timeout 回调。核心层会进行范围检查，并要求设置 min_timeout 与 max_timeout。该回调是可选的。
  WDIOC_GETTIMEOUT：需要做一些准备工作。
  WDIOC_GETTIMELEFT：需要定义 get_timeleft() 回调，否则返回 EOPNOTSUPP。

通过 ioctl 回调来处理那些框架未提供的 IOCTL。需要注意的是，该机制主要面向移植旧驱动；新的驱动不应发明私有的 IOCTL。私有 IOCTL 会被优先处理。如果回调返回 -ENOIOCTLCMD，框架也会尝试处理该 IOCTL。出现错误时直接返回给用户即可。

```
  -static const struct file_operations s3c2410wdt_fops = {
  -       .owner          = THIS_MODULE,
  -       .write          = s3c2410wdt_write,
  -       .unlocked_ioctl = s3c2410wdt_ioctl,
  -       .open           = s3c2410wdt_open,
  -       .release        = s3c2410wdt_release,
  -};
```

检查各函数，将设备特定的内容保留下来供后续重构，其余部分删除。

### 移除 miscdevice

由于 file_operations 已移除，现在也应移除 'miscdevice' 结构体。框架会在 watchdog_dev_register() 被调用时自动创建设备：

```
  -static struct miscdevice s3c2410wdt_miscdev = {
  -       .minor          = WATCHDOG_MINOR,
  -       .name           = "watchdog",
  -       .fops           = &s3c2410wdt_fops,
  -};
```

### 移除过时的 include 与定义

经过上述简化后，少数定义现在可能已不再使用，可以移除：

```
  - #include <linux/fs.h>
  - #include <linux/miscdevice.h> (if MODULE_ALIAS_MISCDEV is not used)
  - #include <linux/uaccess.h> (if no custom IOCTLs are used)
```

### 添加 watchdog 操作

可以在 'watchdog_ops' 结构体中定义可用的回调，其详细说明见 'watchdog-内核-接口.txt'。除 start() 与 owner 必须设置外，其余均为可选。你可以很容易地在旧驱动中找到对应的函数。请注意，现在函数会收到指向 watchdog_device 的指针作为参数，因此可能需要修改函数签名。大多数情况下，这类改动只是因为直接进行了硬件访问。设备特定的代码留在各步骤中，被重构为回调。

```
  +static struct watchdog_ops s3c2410wdt_ops = {
  +       .owner = THIS_MODULE,
  +       .start = s3c2410wdt_start,
  +       .stop = s3c2410wdt_stop,
  +       .ping = s3c2410wdt_keepalive,
  +       .set_timeout = s3c2410wdt_set_heartbeat,
  +};
```

```
  -static void s3c2410wdt_keepalive(void)
  +static int s3c2410wdt_keepalive(struct watchdog_device *wdd)
   {
  ...
  +
  +       return 0;
   }

  ...

  -       s3c2410wdt_keepalive();
  +       s3c2410wdt_keepalive(&s3c2410_wdd);
```

### 添加 watchdog 设备

现在需要创建 'watchdog_device' 结构体，并填充框架所需的必要信息。该结构体在 'watchdog-内核-接口.txt' 中有详细说明。必须传入新建的 watchdog_ops 以及 watchdog_info 结构体。通常，旧驱动会使用静态变量来记录 bootstatus、timeout 等信息，现在应改用 watchdog_device 的对应成员。请注意，timeout 值为 unsigned int 类型；如果驱动原来使用 signed int，也需要一并转换。

```
  +static struct watchdog_device s3c2410_wdd = {
  +       .info = &s3c2410_wdt_ident,
  +       .ops = &s3c2410wdt_ops,
  +};
```

### 处理 'nowayout' 特性

少数驱动静态地使用 nowayout，即由模块参数 CONFIG_WATCHDOG_NOWAYOUT 决定该特性是否启用。需要将其转换为对状态变量的初始化：

```
        .status = WATCHDOG_NOWAYOUT_INIT_STATUS,
```

不过，大多数驱动允许在运行时配置 nowayout，通常如下：

```
	watchdog_set_nowayout(&s3c2410_wdd, nowayout);
```

模块参数本身需要保留，但与 nowayout 相关的其余代码（很可能位于 open()、release()、write() 中）都可以删除。

### 注册 watchdog 设备

将 misc_register(&miscdev) 替换为 watchdog_register_device(&watchdog_dev)。请确保检查返回值并给出错误消息（如有）：

```
  -       ret = misc_register(&s3c2410wdt_miscdev);
  +       ret = watchdog_register_device(&s3c2410_wdd);

  ...

  -       misc_deregister(&s3c2410wdt_miscdev);
  +       watchdog_unregister_device(&s3c2410_wdd);
```

### 更新 Kconfig 项

该驱动现在需要选择 WATCHDOG_CORE：

- 选择 WATCHDOG_CORE

### 创建补丁并发送上游

在发送补丁前，请务必阅读 文档/进程/submitting-patches.rst，并发送到 linux-watchdog@vger.kernel.org。期待你的贡献 :)
