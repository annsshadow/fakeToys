## TEE（可信执行环境）驱动 API


内核提供 TEE 总线基础设施，其中可信应用程序被表示为通过通用唯一标识（UUID）标识的设备，客户端驱动注册一张受支持设备 UUID 的表
TEE 总线基础设施注册以下 API
match()  遍历客户端驱UUID 表，为设UUID 查找对应的匹配。如果找到匹配，则通过
  客户端驱动注册的相应 probe API 探测该特定设备。每当设备或客户端驱动在 TEE
  总线上注册时，都会发生此过程
uevent()  每当 TEE 总线上注册新设备时通知用户空间（udev），以自动加载模块化的客户端
  驱动
TEE 总线设备枚举特定于底TEE 实现，因此留TEE 驱动提供相应的实现
然后 TEE 客户端驱动可以使include/linux/tee_drv.h 中列出的 API 与匹配的
Trusted Application 通信
### TEE 客户端驱动示

假设某个 TEE 客户端驱动需要与一个具有以UUID Trusted Application 通信`ac6a4085-0e82-4c33-bf98-8eb8e118b6c2`，则驱动注册如下
```

	static const struct tee_client_device_id client_id_table[] = {
		{UUID_INIT(0xac6a4085, 0x0e82, 0x4c33,
			   0xbf, 0x98, 0x8e, 0xb8, 0xe1, 0x18, 0xb6, 0xc2)},
		{}
	};

	MODULE_DEVICE_TABLE(tee, client_id_table);

	static struct tee_client_driver client_driver = {
		.probe		= client_probe,
		.remove		= client_remove,
		.id_table	= client_id_table,
		.driver		= {
			.name		= DRIVER_NAME,
		},
	};

	module_tee_client_driver(client_driver);

```
