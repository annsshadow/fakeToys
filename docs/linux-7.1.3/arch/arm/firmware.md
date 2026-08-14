## 鐢ㄤ簬娉ㄥ唽鍜岃皟鐢?ARM 鍥轰欢鐗瑰畾鎿嶄綔鐨勬帴鍙?

Written by Tomasz Figa <t.figa@samsung.com>

涓€浜涙澘鍗¤繍琛屽湪 TrustZone 瀹夊叏涓栫晫锛坰ecure world锛変腑鐨勫畨鍏ㄥ浐浠朵笂锛岃繖鏀瑰彉浜嗘煇浜涗簨椤圭殑鍒濆鍖栨柟寮忋€傝繖灏变骇鐢熶簡涓鸿繖绫诲钩鍙版彁渚涙帴鍙ｇ殑闇€姹傦紝浠ユ寚瀹氬彲鐢ㄧ殑鍥轰欢鎿嶄綔骞跺湪闇€瑕佹椂璋冪敤瀹冧滑銆?
鍥轰欢鎿嶄綔鍙互閫氳繃濉厖涓€涓甫鏈夐€傚綋鍥炶皟鐨?struct firmware_ops 缁撴瀯锛岀劧鍚庝娇鐢?register_firmware_ops() 娉ㄥ唽瀹冩潵鎸囧畾
```

	void register_firmware_ops(const struct firmware_ops *ops)

```
ops 鎸囬拡蹇呴』闈炵┖銆傚叧浜?struct firmware_ops 鍙婂叾鎴愬憳鐨勬洿澶氫俊鎭彲鍦?arch/arm/include/asm/firmware.h 澶存枃浠朵腑鎵惧埌銆?
鎻愪緵浜嗕竴涓粯璁ょ殑銆佺┖鐨勬搷闆嗗悎锛屽洜姝ゅ鏋滃钩鍙颁笉闇€瑕佸浐浠舵搷浣滐紝灏辨棤闇€璁剧疆浠讳綍涓滆タ銆?
```

	#define call_firmware_op(op, ...)				\
		((firmware_ops->op) ? firmware_ops->op(__VA_ARGS__) : (-ENOSYS))

```
璇ュ畯妫€鏌ユ槸鍚︽彁渚涗簡璇ユ搷浣滐紝鑻ユ彁渚涗簡鍒欒皟鐢ㄥ畠锛屽惁鍒欒繑鍥?-ENOSYS 浠ヨ〃绀虹粰瀹氭搷浣滀笉鍙敤锛堜緥濡傦紝浠ヤ究鍥為€€鍒颁紶缁熸搷浣滐級銆?
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
