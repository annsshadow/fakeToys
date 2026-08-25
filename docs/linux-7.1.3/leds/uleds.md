## 用户空间 LED


uleds 驱动程序支持用户空间 LED。这对于测试很有
触发器，也可用于实现虚拟 LED


## 用法


加载驱动程序时，会在 /dev/uleds 处创建一个字符设备。到
创建一个新LED 类设备，打开 /dev/uleds 并写uleds_user_dev
```

    #define LED_MAX_NAME_SIZE 64

    struct uleds_user_dev {
	char name[LED_MAX_NAME_SIZE];
    };

```
将使用给定的名称创建一个新LED 类设备。名字可以是
任何有效sysfs 设备节点名称，但考虑使用 LED 类命
“设备名称：颜色：功能”的约定

通过读取字符中的单个字节来找到当前亮
设备。值是无符号的 255。读取将阻塞，直到亮
变化。还可以轮询设备节点以通知亮度值何
变化

当打开文件句柄/dev/uleds 时，LED 类设备将被删
已关闭

通过打开附加文件句柄来创建多LED 类设
/dev/uleds銆。

有关示例用户空间程序，请参阅tools/leds/uledmon.c
