## Samsung USB 2.0 PHY 閫傞厤灞?

### 1. 鎻忚堪


鍦ㄨ澶?Samsung SoC 涓紝USB 2.0 PHY 妯″潡鐨勬灦鏋勬槸鐩镐技鐨勩€傚敖绠℃湁杩欎簺鐩镐技涔嬪锛屼絾鍒涘缓涓€涓兘閫傞厤鎵€鏈夎繖浜?PHY 鎺у埗鍣ㄧ殑鍗曚竴椹卞姩琚瘉鏄庢槸鍥伴毦鐨勩€傚樊寮傚線寰€寰堝皬锛屽瓨鍦ㄤ簬 PHY 瀵勫瓨鍣ㄧ殑鐗瑰畾浣嶄腑銆傚湪灏戞暟缃曡鎯呭喌涓嬶紝蹇呴』
鏀瑰彉瀵勫瓨鍣ㄥ啓鍏ョ殑椤哄簭鎴?PHY 涓婄數杩囩▼銆傛閫傞厤灞傛槸鍦ㄦ嫢鏈夌嫭绔嬮┍鍔ㄥ拰鎷ユ湁涓鸿澶氱壒娈婃儏鍐靛鍔犳敮鎸佺殑鍗曚竴椹卞姩涔嬮棿鐨?涓€绉嶆姌琛枫€?
### 2. 鏂囦欢鎻忚堪


- phy-samsung-usb2.c
   杩欐槸閫傞厤灞傜殑涓绘枃浠躲€傝鏂囦欢鍖呭惈 probe 鍑芥暟锛屽苟鍚戦€氱敤 PHY 妗嗘灦鎻愪緵涓や釜鍥炶皟銆傝繖涓や釜鍥炶皟鐢ㄤ簬缁?phy 涓婄數
   鍜屼笅鐢点€傚畠浠墽琛屾墍鏈夌増鏈?PHY 妯″潡閮藉繀椤诲畬鎴愮殑鍏叡宸ヤ綔銆傛牴鎹墍閫夋嫨鐨?SoC锛屽畠浠墽琛?SoC 鐗瑰畾鐨勫洖璋冦€?   鐗瑰畾鐨?SoC 鐗堟湰閫氳繃閫夋嫨閫傚綋鐨?compatible 瀛楃涓叉潵纭畾銆傛澶栵紝璇ユ枃浠跺寘鍚拡瀵圭壒瀹?SoC 鐨?   struct of_device_id 瀹氫箟銆?
- phy-samsung-usb2.h
   杩欐槸澶存枃浠躲€傚畠澹版槑姝ら┍鍔ㄤ娇鐢ㄧ殑缁撴瀯浣撱€傛澶栵紝瀹冨簲鍖呭惈鎻忚堪鐗瑰畾 SoC 鐨勭粨鏋勪綋鐨?extern 澹版槑銆?
### 3. 鏀寔鐨?SoC


瑕佹敮鎸佷竴涓柊鐨?SoC锛屽簲鍚?drivers/phy 鐩綍娣诲姞涓€涓柊鏂囦欢銆傛瘡涓?SoC 鐨勯厤缃瓨鍌ㄥ湪涓€涓?```

  struct samsung_usb2_phy_config {
	const struct samsung_usb2_common_phy *phys;
	int (*rate_to_clk)(unsigned long, u32 *);
	unsigned int num_phys;
	bool has_mode_switch;
  };

```
num_phys 鏄┍鍔ㄥ鐞嗙殑 phy 鏁伴噺銆俙*phys` 鏄竴涓暟缁勶紝鍖呭惈姣忎釜 phy 鐨勯厤缃€俬as_mode_switch 灞炴€ф槸涓€涓?甯冨皵鏍囧織锛屽喅瀹?SoC 鏄惁鍦ㄤ竴瀵瑰紩鑴氫笂鍚屾椂鍏锋湁 USB 涓绘満鍜岃澶囥€傚鏋滄槸锛屽垯蹇呴』淇敼涓€涓壒娈婂瘎瀛樺櫒鏉ュ湪杩欎簺寮曡剼鐨?鍐呴儴璺敱涔嬮棿鍒囨崲锛屼互杩炲埌 USB 璁惧鎴栦富鏈烘ā鍧椼€?
```

  const struct samsung_usb2_phy_config exynos4210_usb2_phy_config = {
	.has_mode_switch        = 0,
	.num_phys		= EXYNOS4210_NUM_PHYS,
	.phys			= exynos4210_phys,
	.rate_to_clk		= exynos4210_rate_to_clk,
  }

```
- `int (**rate_to_clk)(unsigned long, u32 **)`

	rate_to_clk 鍥炶皟鐢ㄤ簬灏嗙敤浣?PHY 妯″潡鍙傝€冩椂閽熺殑鏃堕挓閫熺巼杞崲涓哄簲鍐欏叆纭欢瀵勫瓨鍣ㄧ殑鍊笺€?
```

  static const struct samsung_usb2_common_phy exynos4210_phys[] = {
	{
		.label		= "device",
		.id		= EXYNOS4210_DEVICE,
		.power_on	= exynos4210_power_on,
		.power_off	= exynos4210_power_off,
	},
	{
		.label		= "host",
		.id		= EXYNOS4210_HOST,
		.power_on	= exynos4210_power_on,
		.power_off	= exynos4210_power_off,
	},
	{
		.label		= "hsic0",
		.id		= EXYNOS4210_HSIC0,
		.power_on	= exynos4210_power_on,
		.power_off	= exynos4210_power_off,
	},
	{
		.label		= "hsic1",
		.id		= EXYNOS4210_HSIC1,
		.power_on	= exynos4210_power_on,
		.power_off	= exynos4210_power_off,
	},
	{},
  };

```
- `int (**power_on)(struct samsung_usb2_phy_instance **);`
  `int (**power_off)(struct samsung_usb2_phy_instance **);`

	杩欎袱涓洖璋冪敤浜庨€氳繃淇敼閫傚綋鐨勫瘎瀛樺櫒鏉ョ粰 phy 涓婄數鍜屼笅鐢点€?
瀵归┍鍔ㄧ殑鏈€鍚庢敼鍔ㄦ槸鍚?phy-samsung-usb2.c 鏂囦欢娣诲姞閫傚綋鐨?compatible 鍊笺€傚浜?Exynos 4210锛屼互涓嬭
```

  #ifdef CONFIG_PHY_EXYNOS4210_USB2
	{
		.compatible = "samsung,exynos4210-usb2-phy",
		.data = &exynos4210_usb2_phy_config,
	},
  #endif

```
涓轰簡缁欓┍鍔ㄥ鍔犺繘涓€姝ョ殑鐏垫椿鎬э紝Kconfig 鏂囦欢浣胯兘鍦ㄧ紪璇戠殑椹卞姩涓寘鍚鎵€閫?SoC 鐨勬敮鎸併€侹config
```

  config PHY_EXYNOS4210_USB2
	bool "Support for Exynos 4210"
	depends on PHY_SAMSUNG_USB2
	depends on CPU_EXYNOS4210
	help
	  Enable USB PHY support for Exynos 4210. This option requires that
	  Samsung USB 2.0 PHY driver is enabled and means that support for this
	  particular SoC is compiled in the driver. In case of Exynos 4210 four
	  phys are available - device, host, HSCI0 and HSCI1.

```
鏂板垱寤虹殑鏀寔鏂?SoC 鐨勬枃浠朵篃蹇呴』娣诲姞鍒?```

  obj-$(CONFIG_PHY_EXYNOS4210_USB2)       += phy-exynos4210-usb2.o

```
瀹屾垚杩欎簺姝ラ鍚庯紝瀵规柊 SoC 鐨勬敮鎸佸氨搴旇灏辩华浜嗐€?