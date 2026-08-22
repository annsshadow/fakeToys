## 调节器机器驱动接口（Regulator Machine Driver Interface

调节器机器驱动接口用于板机器特定的初始化代码来配置调节器（regulator）子系统
```

  Regulator-1 -+-> Regulator-2 --> [Consumer A @ 1.8 - 2.0V]
               |
               +-> [Consumer B @ 3.3V]

```

消费A B 的驱动必须映射到正确的调节器，以便控制它们的电源。这种映射可以在机器初始化代码中通过为每个消费者创struct regulator_consumer_supply 来实
```

  struct regulator_consumer_supply {
	const char *dev_name;	/* consumer dev_name() */
	const char *supply;	/* consumer supply - e.g. "vcc" */
  };

```

```

  static struct regulator_consumer_supply regulator1_consumers[] = {
	REGULATOR_SUPPLY("Vcc", "consumer B"),
  };

  static struct regulator_consumer_supply regulator2_consumers[] = {
	REGULATOR_SUPPLY("Vcc", "consumer A"),
  };

```

这将 Regulator-1 映射Consumer B 'Vcc' 供电，并Regulator-2 映射Consumer A 'Vcc' 供电
现在可以通过为每个调节器电源域定struct regulator_init_data 来注册约束。该结构还将消费者映射进
```

  static struct regulator_init_data regulator1_data = {
	.constraints = {
		.name = "Regulator-1",
		.min_uV = 3300000,
		.max_uV = 3300000,
		.valid_modes_mask = REGULATOR_MODE_NORMAL,
	},
	.num_consumer_supplies = ARRAY_SIZE(regulator1_consumers),
	.consumer_supplies = regulator1_consumers,
  };

```

name 字段应设置为对板级配置其他调节器的供电、以及用于日志和其他诊断输出时有用描述性的内容。通常原理图中用于供电轨（supply rail）的名称是一个不错的选择。如果没有提供名称，子系统会自动选择一个
Regulator-1 Regulator-2 供电。这种关系必须向核心注册，以便当 Consumer A 启用其供电（Regulator-2）时，Regulator-1 也会随之启用。供电调节器supply_regulator 字段设置

```

  static struct regulator_init_data regulator2_data = {
	.supply_regulator = "Regulator-1",
	.constraints = {
		.min_uV = 1800000,
		.max_uV = 2000000,
		.valid_ops_mask = REGULATOR_CHANGE_VOLTAGE,
		.valid_modes_mask = REGULATOR_MODE_NORMAL,
	},
	.num_consumer_supplies = ARRAY_SIZE(regulator2_consumers),
	.consumer_supplies = regulator2_consumers,
  };

```

```

  static struct platform_device regulator_devices[] = {
	{
		.name = "regulator",
		.id = DCDC_1,
		.dev = {
			.platform_data = &regulator1_data,
		},
	},
	{
		.name = "regulator",
		.id = DCDC_2,
		.dev = {
			.platform_data = &regulator2_data,
		},
	},
  };
  /* register regulator 1 device */
  platform_device_register(&regulator_devices[0]);

  /* register regulator 2 device */
  platform_device_register(&regulator_devices[1]);

```
