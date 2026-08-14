
## 固件上传 API


向固件加载器注册的设备驱动会暴露持久的 sysfs 节点，使用户能够发起针对该设备的
固件更新。对收到的数据进行任何校验，是设备驱动和/或设备本身的责任。固件上传
使用了固件回退（firmware fallback）文档中所描述的 **loading** 和 **data** 这两个
sysfs 文件，此外还新增了若干 sysfs 文件，用于提供固件镜像传输到设备过程中的状态信息。

## 注册固件上传


设备驱动通过调用 firmware_upload_register() 来注册固件上传。在参数列表中包含
一个用于在 /sys/class/firmware 下标识该设备的名称。用户可以向目标设备的
**loading** sysfs 文件写入 1 来发起一次固件上传。接着，用户将固件镜像写入
**data** sysfs 文件。写完固件数据后，用户向 **loading** sysfs 文件写入 0 表示
传输完成。向 **loading** 写入 0 还会触发在内存内核工作线程（worker thread）上下文
中将固件传输给下层设备驱动。

要使用固件上传 API，需编写一个实现了若干 ops 的驱动。probe 函数调用
firmware_upload_register()，remove 函数调用
```
firmware_upload_unregister()
```
。

```
	static const struct fw_upload_ops m10bmc_ops = {
		.prepare = m10bmc_sec_prepare,
		.write = m10bmc_sec_write,
		.poll_complete = m10bmc_sec_poll_complete,
		.cancel = m10bmc_sec_cancel,
		.cleanup = m10bmc_sec_cleanup,
	};

	static int m10bmc_sec_probe(struct platform_device *pdev)
	{
		const char *fw_name, *truncate;
		struct m10bmc_sec *sec;
		struct fw_upload *fwl;
		unsigned int len;

		sec = devm_kzalloc(&pdev->dev, sizeof(*sec), GFP_KERNEL);
		if (!sec)
			return -ENOMEM;

		sec->dev = &pdev->dev;
		sec->m10bmc = dev_get_drvdata(pdev->dev.parent);
		dev_set_drvdata(&pdev->dev, sec);

		fw_name = dev_name(sec->dev);
		truncate = strstr(fw_name, ".auto");
		len = (truncate) ? truncate - fw_name : strlen(fw_name);
		sec->fw_name = kmemdup_nul(fw_name, len, GFP_KERNEL);

		fwl = firmware_upload_register(THIS_MODULE, sec->dev, sec->fw_name,
					       &m10bmc_ops, sec);
		if (IS_ERR(fwl)) {
			dev_err(sec->dev, "Firmware Upload driver failed to start\n");
			kfree(sec->fw_name);
			return PTR_ERR(fwl);
		}

		sec->fwl = fwl;
		return 0;
	}

	static int m10bmc_sec_remove(struct platform_device *pdev)
	{
		struct m10bmc_sec *sec = dev_get_drvdata(&pdev->dev);

		firmware_upload_unregister(sec->fwl);
		kfree(sec->fw_name);
		return 0;
	}

```

### firmware_upload_register

   :identifiers: firmware_upload_register

### firmware_upload_unregister

   :identifiers: firmware_upload_unregister

### 固件上传 Ops

   :identifiers: fw_upload_ops

### 固件上传进度码

以下进度码由固件加载器在内部使用。对应的字符串会通过下文描述的 status sysfs
节点上报，并在 ABI 文档中有说明。

   :identifiers: fw_upload_prog

### 固件上传错误码

在失败时，驱动 ops 可能返回以下错误码：

   :identifiers: fw_upload_err

## Sysfs 属性


除了 **loading** 和 **data** 这两个 sysfs 文件外，还有额外的 sysfs 文件用于监视
数据传输到目标设备的状态，并确定传输最终的成功/失败状态。根据设备及固件镜像
大小的不同，一次固件更新可能耗时数毫秒到数分钟不等。

额外的 sysfs 文件如下：

- status - 提供固件更新进度的指示
- error - 提供失败固件更新的错误信息
- remaining_size - 跟踪一次更新中数据传输的部分
- cancel - 向该文件写入 1 以取消更新
