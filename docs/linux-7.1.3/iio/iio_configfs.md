## 工业 IIO configfs 支持


## 1. 概述


Configfs 是一个基于文件系统的内核对象管理器。IIO 使用一些可以方便地通过 configfs 配置的对象（例如：设备、触发器）。

有关 configfs 的工作方式，请参阅 Documentation/filesystems/configfs.rst。

## 2. 用法


为了在 IIO 中使用 configfs 支持，我们需要在编译时通过 CONFIG_IIO_CONFIGFS 配置选项将其选中。

```

  $ mkdir /config
  $ mount -t configfs none /config

```
此时，所有默认的 IIO 组都将被创建，并可在 /config/iio 下访问。后续章节将描述可用的 IIO 配置对象。

## 3. 软件触发器


IIO 默认 configfs 组之一是 “triggers（触发器）”组。它在 configfs 挂载后自动可访问，可在 /config/iio/triggers 下找到。

IIO 软件触发器的实现支持创建多种触发器类型。一个新的触发器类型通常作为一个独立的
```

  /*
   * drivers/iio/trigger/iio-trig-sample.c
   * sample kernel module implementing a new trigger type
   */
  #include <linux/iio/sw_trigger.h>


  static struct iio_sw_trigger *iio_trig_sample_probe(const char *name)
  {
	/*
	 * This allocates and registers an IIO trigger plus other
	 * trigger type specific initialization.
	 */
  }

  static int iio_trig_sample_remove(struct iio_sw_trigger *swt)
  {
	/*
	 * This undoes the actions in iio_trig_sample_probe
	 */
  }

  static const struct iio_sw_trigger_ops iio_trig_sample_ops = {
	.probe		= iio_trig_sample_probe,
	.remove		= iio_trig_sample_remove,
  };

  static struct iio_sw_trigger_type iio_trig_sample = {
	.name = "trig-sample",
	.owner = THIS_MODULE,
	.ops = &iio_trig_sample_ops,
  };

  module_iio_sw_trigger_driver(iio_trig_sample);

```
每种触发器类型在 /config/iio/triggers 下都有自己的目录。加载 iio-trig-sample 模块将创建 'trig-sample' 触发器类型目录 /config/iio/triggers/trig-sample。

我们支持以下中断源（触发器类型）：

 - hrtimer，使用高分辨率定时器作为中断源

### 3.1 hrtimer 触发器的创建与销毁


加载 iio-trig-hrtimer 模块将注册 hrtimer 触发器类型，允许用户在 /config/iio/triggers/hrtimer 下创建 hrtimer 触发器。

```

  $ mkdir /config/iio/triggers/hrtimer/instance1
  $ rmdir /config/iio/triggers/hrtimer/instance1

```
每个触发器可以拥有一个或多个特定于该触发器类型的属性。

### 3.2 "hrtimer" 触发器类型的属性


"hrtimer" 触发器类型在 /config 目录下没有任何可配置属性。它会向触发器目录引入 sampling_frequency 属性。该属性以 Hz 为单位设置轮询频率，精度为 mHz。
