## Extcon 设备子系统


## 概述


Extcon（External Connector，外部连接器）子系统提供了一个统一的框架，用于管理
Linux 系统中的外部连接器。它允许驱动报告外部连接器的状态，并为用户空间提供
标准化的接口来查询和监控这些状态。

Extcon 在具有多种连接选项的现代设备（例如智能手机、平板电脑和笔记本电脑）中
尤其有用。它有助于管理各种类型的连接器，包括：

1. USB 连接器（例如 USB-C、micro-USB）
2. 充电端口（例如快充、无线充电）
3. 音频插孔（例如 3.5mm 耳机插孔）
4. 视频输出（例如 HDMI、DisplayPort）
5. 扩展坞

真实世界的例子：

1. 智能手机 USB-C 端口：
   智能手机上单个 USB-C 端口可以提供多种功能。Extcon 可以管理该端口的不同状态，
   例如：
   - USB 数据连接
   - 充电（各种类型，如快充、USB Power Delivery）
   - 音频输出（USB-C 耳机）
   - 视频输出（USB-C 转 HDMI 适配器）

2. 笔记本电脑扩展坞：
   当笔记本电脑连接到扩展坞时，会同时进行多个连接。Extcon 可以处理以下状态的
   变化：
   - 电力传输
   - 外部显示器
   - USB 集线器连接
   - 以太网连接

3. 无线充电板：
   Extcon 可以管理无线充电连接的状态，使系统能够在设备放上或移开充电板时做出
   恰当的响应。

4. 智能电视 HDMI 端口：
   在智能电视中，Extcon 可以管理多个 HDMI 端口，检测设备何时连接或断开，并可能
   识别设备的类型（例如游戏机、机顶盒、蓝光播放器）。

Extcon 框架通过提供标准化的方式来报告与查询连接器状态、处理互斥连接以及管理
连接器属性，简化了这些复杂场景的驱动开发。这使得现代设备中对外部连接的处理更加
稳健和灵活。

## 关键组件


### extcon_dev


```

    struct extcon_dev {
        const char *name;
        const unsigned int *supported_cable;
        const u32 *mutually_exclusive;

        /* Internal data */
        struct device dev;
        unsigned int id;
        struct raw_notifier_head nh_all;
        struct raw_notifier_head *nh;
        struct list_head entry;
        int max_supported;
        spinlock_t lock;
        u32 state;

        /* Sysfs related */
        struct device_type extcon_dev_type;
        struct extcon_cable *cables;
        struct attribute_group attr_g_muex;
        struct attribute **attrs_muex;
        struct device_attribute *d_attrs_muex;
    };

```
关键字段：

- `name`：Extcon 设备的名称
- `supported_cable`：支持的线缆类型数组
- `mutually_exclusive`：定义互斥线缆类型的数组
  该字段对于强制实施硬件约束至关重要。它是一个 32 位无符号整数数组，其中每个
  元素代表一组互斥的线缆类型。该数组应以 0 结尾。

  例如：

```

      static const u32 mutually_exclusive[] = {
          BIT(0) | BIT(1),  /* Cable 0 and 1 are mutually exclusive */
          BIT(2) | BIT(3) | BIT(4),  /* Cables 2, 3, and 4 are mutually exclusive */
          0  /* Terminator */
      };

  在此示例中，线缆 0 和 1 不能同时连接，线缆 2、3 和 4 也是互斥的。这对于诸如
  单个端口既可以是 USB 也可以是 HDMI、但不能同时是两者这样的场景很有用。

  Extcon 核心利用这些信息来防止无效的线缆状态组合，确保报告的状态始终与硬件
  能力一致。

```
- `state`：设备的当前状态（已连接线缆的位图）


### extcon_cable


```

    struct extcon_cable {
        struct extcon_dev *edev;
        int cable_index;
        struct attribute_group attr_g;
        struct device_attribute attr_name;
        struct device_attribute attr_state;
        struct attribute *attrs[3];
        union extcon_property_value usb_propval[EXTCON_PROP_USB_CNT];
        union extcon_property_value chg_propval[EXTCON_PROP_CHG_CNT];
        union extcon_property_value jack_propval[EXTCON_PROP_JACK_CNT];
        union extcon_property_value disp_propval[EXTCON_PROP_DISP_CNT];
        DECLARE_BITMAP(usb_bits, EXTCON_PROP_USB_CNT);
        DECLARE_BITMAP(chg_bits, EXTCON_PROP_CHG_CNT);
        DECLARE_BITMAP(jack_bits, EXTCON_PROP_JACK_CNT);
        DECLARE_BITMAP(disp_bits, EXTCON_PROP_DISP_CNT);
    };

```
## 核心函数


   :identifiers: extcon_get_state

   :identifiers: extcon_set_state

   :identifiers: extcon_set_state_sync

   :identifiers: extcon_get_property


## Sysfs 接口


Extcon 设备暴露以下 sysfs 属性：

- `name`：Extcon 设备的名称
- `state`：所有受支持线缆的当前状态
- `cable.N/name`：第 N 个受支持线缆的名称
- `cable.N/state`：第 N 个受支持线缆的状态

### 使用示例


    #include <linux/module.h>
    #include <linux/platform_device.h>
    #include <linux/extcon.h>

    struct my_extcon_data {
        struct extcon_dev *edev;
        struct device *dev;
    };

    static const unsigned int my_extcon_cable[] = {
        EXTCON_USB,
        EXTCON_USB_HOST,
        EXTCON_NONE,
    };

    static int my_extcon_probe(struct platform_device *pdev)
    {
        struct my_extcon_data *data;
        int ret;

        data = devm_kzalloc(&pdev->dev, sizeof(*data), GFP_KERNEL);
        if (!data)
            return -ENOMEM;

        data->dev = &pdev->dev;

        /** Initialize extcon device **/
        data->edev = devm_extcon_dev_allocate(data->dev, my_extcon_cable);
        if (IS_ERR(data->edev)) {
            dev_err(data->dev, "Failed to allocate extcon device\n");
            return PTR_ERR(data->edev);
        }

        /** Register extcon device **/
        ret = devm_extcon_dev_register(data->dev, data->edev);
        if (ret < 0) {
            dev_err(data->dev, "Failed to register extcon device\n");
            return ret;
        }

        platform_set_drvdata(pdev, data);

        /** Example: Set initial state **/
        extcon_set_state_sync(data->edev, EXTCON_USB, true);

        dev_info(data->dev, "My extcon driver probed successfully\n");
        return 0;
    }

    static int my_extcon_remove(struct platform_device *pdev)
    {
        struct my_extcon_data *data = platform_get_drvdata(pdev);

        /** Example: Clear state before removal **/
        extcon_set_state_sync(data->edev, EXTCON_USB, false);

        dev_info(data->dev, "My extcon driver removed\n");
        return 0;
    }

    static const struct of_device_id my_extcon_of_match[] = {
        { .compatible = "my,extcon-device", },
        { },
    };
    MODULE_DEVICE_TABLE(of, my_extcon_of_match);

    static struct platform_driver my_extcon_driver = {
        .driver = {
            .name = "my-extcon-driver",
            .of_match_table = my_extcon_of_match,
        },
        .probe = my_extcon_probe,
        .remove = my_extcon_remove,
    };

    module_platform_driver(my_extcon_driver);

### 此示例演示了：


- 定义受支持的线缆类型（本例中为 USB 和 USB Host）。
- 分配并注册一个 extcon 设备。
- 为线缆设置初始状态（本例中为 USB 已连接）。
- 在驱动被移除时清除状态。
