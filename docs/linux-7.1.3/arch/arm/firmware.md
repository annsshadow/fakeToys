## 用于注册和调用 ARM 固件特定操作的接口


Written by Tomasz Figa <t.figa@samsung.com>

一些板卡运行在 TrustZone 安全世界（secure world）中的安全固件上，这改变了某些事项的初始化方式。这就产生了为这类平台提供接口的需求，以指定可用的固件操作并在需要时调用它们。

固件操作可以通过填充一个带有适当回调的 struct firmware_ops 结构，然后使用 register_firmware_ops() 注册它来指定
```

	void register_firmware_ops(const struct firmware_ops *ops)

```
ops 指针必须非空。关于 struct firmware_ops 及其成员的更多信息可在 arch/arm/include/asm/firmware.h 头文件中找到。

提供了一个默认的、空的操集合，因此如果平台不需要固件操作，就无需设置任何东西。

```

	#define call_firmware_op(op, ...)				\
		((firmware_ops->op) ? firmware_ops->op(__VA_ARGS__) : (-ENOSYS))

```
该宏检查是否提供了该操作，若提供了则调用它，否则返回 -ENOSYS 以表示给定操作不可用（例如，以便回退到传统操作）。

```

	/* board file */

	static int platformX_do_idle(void)
	{
		/* tell platformX firmware to enter idle */
		return 0;
	}

	static int platformX_cpu_boot(int i)
	{
		/* tell platformX firmware to boot CPU i */
		return 0;
	}

	static const struct firmware_ops platformX_firmware_ops = {
		.do_idle        = exynos_do_idle,
		.cpu_boot       = exynos_cpu_boot,
		/* other operations not available on platformX */
	};

	/* init_early callback of machine descriptor */
	static void __init board_init_early(void)
	{
		register_firmware_ops(&platformX_firmware_ops);
	}

```

```

	/* some platform code, e.g. SMP initialization */

	__raw_writel(__pa_symbol(exynos4_secondary_startup),
		CPU1_BOOT_REG);

	/* Call Exynos specific smc call */
	if (call_firmware_op(cpu_boot, cpu) == -ENOSYS)
		cpu_boot_legacy(...); /* Try legacy way */

	gic_raise_softirq(cpumask_of(cpu), 1);

```
