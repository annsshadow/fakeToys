## HW consumer


IIO 璁惧鍙互鍦ㄧ‖浠朵腑鐩存帴杩炴帴鍒板彟涓€涓澶囥€傚湪杩欑鎯呭喌涓嬶紝IIO 鎻愪緵鑰呬笌 IIO 娑堣垂鑰?涔嬮棿鐨勭紦鍐插尯鐢辩‖浠跺鐞嗐€侷ndustrial I/O 纭欢娑堣垂鑰呮彁渚涗簡涓€绉嶆棤闇€杞欢缂撳啿鍖烘潵
缁戝畾杩欎簺 IIO 璁惧鐨勬柟寮忋€傚叾瀹炵幇鍙湪 `drivers/iio/buffer/hw-consumer.c` 鎵惧埌銆?

- struct iio_hw_consumer 鈥?纭欢娑堣垂鑰呯粨鏋?- `iio_hw_consumer_alloc` 鈥?鍒嗛厤 IIO 纭欢娑堣垂鑰?- `iio_hw_consumer_free` 鈥?閲婃斁 IIO 纭欢娑堣垂鑰?- `iio_hw_consumer_enable` 鈥?鍚敤 IIO 纭欢娑堣垂鑰?- `iio_hw_consumer_disable` 鈥?绂佺敤 IIO 纭欢娑堣垂鑰?

## HW consumer 璁剧疆


浣滀负鏍囧噯 IIO 璁惧锛岃瀹炵幇鍩轰簬 IIO 鎻愪緵鑰?娑堣垂鑰呮ā鍨嬨€?```

	static struct iio_hw_consumer *hwc;

	static const struct iio_info adc_info = {
		.read_raw = adc_read_raw,
	};

	static int adc_read_raw(struct iio_dev *indio_dev,
				struct iio_chan_spec const *chan, int *val,
				int *val2, long mask)
	{
		ret = iio_hw_consumer_enable(hwc);

		/* 鑾峰彇鏁版嵁 */

		ret = iio_hw_consumer_disable(hwc);
	}

	static int adc_probe(struct platform_device *pdev)
	{
		hwc = devm_iio_hw_consumer_alloc(&iio->dev);
	}

```
## 鏇村缁嗚妭


   :export:
