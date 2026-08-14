## HW consumer


IIO 设备可以在硬件中直接连接到另一个设备。在这种情况下，IIO 提供者与 IIO 消费者
之间的缓冲区由硬件处理。Industrial I/O 硬件消费者提供了一种无需软件缓冲区来
绑定这些 IIO 设备的方式。其实现可在 `drivers/iio/buffer/hw-consumer.c` 找到。


- struct iio_hw_consumer — 硬件消费者结构
- `iio_hw_consumer_alloc` — 分配 IIO 硬件消费者
- `iio_hw_consumer_free` — 释放 IIO 硬件消费者
- `iio_hw_consumer_enable` — 启用 IIO 硬件消费者
- `iio_hw_consumer_disable` — 禁用 IIO 硬件消费者


## HW consumer 设置


作为标准 IIO 设备，该实现基于 IIO 提供者/消费者模型。
```

	static struct iio_hw_consumer *hwc;

	static const struct iio_info adc_info = {
		.read_raw = adc_read_raw,
	};

	static int adc_read_raw(struct iio_dev *indio_dev,
				struct iio_chan_spec const *chan, int *val,
				int *val2, long mask)
	{
		ret = iio_hw_consumer_enable(hwc);

		/* 获取数据 */

		ret = iio_hw_consumer_disable(hwc);
	}

	static int adc_probe(struct platform_device *pdev)
	{
		hwc = devm_iio_hw_consumer_alloc(&iio->dev);
	}

```
## 更多细节


   :export:
