## PHY 瀛愮郴缁?

:浣滆€? Kishon Vijay Abraham I <kishon@ti.com>

鏈枃妗ｈ鏄庝簡閫氱敤 PHY 妗嗘灦鍙婂叾鎻愪緵鐨?API锛屼互鍙婂浣曚娇鐢ㄣ€?
## 绠€浠?

**PHY** 鏄?physical layer锛堢墿鐞嗗眰锛夌殑缂╁啓銆傚畠鐢ㄤ簬灏嗚澶囪繛鎺ュ埌鐗╃悊浠嬭川锛屼緥濡?USB 鎺у埗鍣ㄦ湁涓€涓?PHY锛岀敤鏉ユ彁渚涗覆琛屽寲銆佸弽涓茶鍖栥€佺紪鐮併€佽В鐮佺瓑鍔熻兘锛屽苟璐熻矗
鑾峰彇鎵€闇€鐨勬暟鎹紶杈撻€熺巼銆傛敞鎰忥紝鏌愪簺 USB 鎺у埗鍣ㄥ皢 PHY 鍔熻兘鍐呭祵鍏朵腑锛岃€屽叾浠栧垯浣跨敤澶栭儴
PHY銆傚叾浠栦娇鐢?PHY 鐨勫璁惧寘鎷?Wireless LAN銆丒thernet銆丼ATA 绛夈€?
鍒涘缓姝ゆ鏋剁殑鎰忓浘鏄皢鏁ｅ竷鍦ㄦ暣涓?Linux 鍐呮牳涓殑 PHY 椹卞姩闆嗕腑鍒?drivers/phy锛屼互澧炲姞浠ｇ爜澶嶇敤骞?鏀瑰杽浠ｇ爜鐨勫彲缁存姢鎬с€?
姝ゆ鏋朵粎瀵逛娇鐢ㄥ閮?PHY锛圥HY 鍔熻兘鏈唴宓屼簬鎺у埗鍣ㄤ腑锛夌殑璁惧鏈夌敤銆?
## 娉ㄥ唽/娉ㄩ攢 PHY provider


PHY provider 鎸囧疄鐜颁簡涓€涓垨澶氫釜 PHY 瀹炰緥鐨勫疄浣撱€?瀵逛簬 PHY provider 浠呭疄鐜板崟涓?PHY 瀹炰緥鐨勭畝鍗曟儏鍐碉紝妗嗘灦鍦?of_phy_simple_xlate 涓彁渚涗簡瀹冭嚜宸辩殑 of_xlate 瀹炵幇銆傚鏋?PHY provider 瀹炵幇浜嗗涓疄渚嬶紝
瀹冨簲鎻愪緵鑷繁鐨?of_xlate 瀹炵幇銆俹f_xlate 浠呯敤浜?dt锛坉evice tree锛夊紩瀵肩殑鎯呭喌銆?
```
	#define of_phy_provider_register(dev, xlate)    \
		__of_phy_provider_register((dev), NULL, THIS_MODULE, (xlate))

	#define devm_of_phy_provider_register(dev, xlate)       \
		__devm_of_phy_provider_register((dev), NULL, THIS_MODULE,
						(xlate))
```
of_phy_provider_register 涓?devm_of_phy_provider_register 瀹忓彲鐢ㄤ簬
娉ㄥ唽 phy_provider锛屽畠浠?device 鍜?of_xlate 浣滀负鍙傛暟銆傚浜?dt 寮曞鎯呭喌锛屾墍鏈?PHY provider 閮藉簲浣跨敤涓婅堪
涓や釜瀹忎箣涓€鏉ユ敞鍐岃 PHY provider銆?
閫氬父锛屼笌 PHY provider 鍏宠仈鐨勮澶囨爲鑺傜偣浼氬寘鍚竴缁勫瓙鑺傜偣锛屾瘡涓瓙鑺傜偣浠ｈ〃涓€涓?PHY銆傛煇浜涚粦瀹氬彲鑳戒负浜?涓婁笅鏂囧拰鍙墿灞曟€ц€屽皢瀛愯妭鐐瑰祵濂楀湪棰濆鐨勫眰绾т腑锛屾鏃跺彲浣跨敤浣庡眰鐨?of_phy_provider_register_full() 涓?devm_of_phy_provider_register_full()
瀹忔潵瑕嗙洊鍖呭惈瀛愯妭鐐圭殑鑺傜偣銆?
```
	#define of_phy_provider_register_full(dev, children, xlate) \
		__of_phy_provider_register(dev, children, THIS_MODULE, xlate)

	#define devm_of_phy_provider_register_full(dev, children, xlate) \
		__devm_of_phy_provider_register_full(dev, children,
						     THIS_MODULE, xlate)

	void devm_of_phy_provider_unregister(struct device *dev,
		struct phy_provider *phy_provider);
	void of_phy_provider_unregister(struct phy_provider *phy_provider);
```
devm_of_phy_provider_unregister 涓?of_phy_provider_unregister 鍙敤浜?娉ㄩ攢璇?PHY銆?
## 鍒涘缓 PHY


PHY 椹卞姩搴斿垱寤?PHY锛屼互渚垮叾浠栧璁炬帶鍒跺櫒鑳藉浣跨敤瀹冦€侾HY 妗嗘灦鎻愪緵浜?2 涓?API 鏉ュ垱寤?PHY銆?
```
	struct phy *phy_create(struct device *dev, struct device_node *node,
			       const struct phy_ops *ops);
	struct phy *devm_phy_create(struct device *dev,
				    struct device_node *node,
				    const struct phy_ops *ops);
```
PHY 椹卞姩鍙互浣跨敤涓婅堪 2 涓?API 涔嬩竴锛岄€氳繃浼犲叆 device 鎸囬拡鍜?phy ops 鏉ュ垱寤?PHY銆?phy_ops 鏄竴缁勭敤浜庢墽琛?PHY 鎿嶄綔锛堝 init銆乪xit銆乸ower_on 鍜?power_off锛夌殑鍑芥暟鎸囬拡銆?
涓轰簡鍦?phy_ops 涓В寮曠敤绉佹湁鏁版嵁锛坧rivate data锛夛紝PHY provider 椹卞姩鍙互鍦ㄥ垱寤?PHY 鍚庝娇鐢?phy_set_drvdata()锛屽苟鍦?phy_ops 涓娇鐢?phy_get_drvdata() 鍙栧洖绉佹湁鏁版嵁銆?
## 鑾峰彇瀵?PHY 鐨勫紩鐢?

鍦ㄦ帶鍒跺櫒鑳藉浣跨敤璇?PHY 涔嬪墠锛屽畠蹇呴』鍏堣幏寰楀瀹冪殑寮曠敤銆傛妗嗘灦鎻愪緵浜嗕互涓?API 鏉ヨ幏鍙栧 PHY 鐨勫紩鐢ㄣ€?
```
	struct phy *phy_get(struct device *dev, const char *string);
	struct phy *devm_phy_get(struct device *dev, const char *string);
	struct phy *devm_phy_optional_get(struct device *dev,
					  const char *string);
	struct phy *devm_of_phy_get(struct device *dev, struct device_node *np,
				    const char *con_id);
	struct phy *devm_of_phy_optional_get(struct device *dev,
					     struct device_node *np,
					     const char *con_id);
	struct phy *devm_of_phy_get_by_index(struct device *dev,
					     struct device_node *np,
					     int index);
```
phy_get銆乨evm_phy_get 涓?devm_phy_optional_get 鍙敤浜庤幏鍙?PHY銆?鍦?dt 寮曞鎯呭喌涓嬶紝string 鍙傛暟搴斿寘鍚?dt 鏁版嵁涓粰鍑虹殑 phy 鍚嶇О锛涘湪
闈?dt 寮曞鎯呭喌涓嬶紝瀹冨簲鍖呭惈 PHY 鐨?label锛堟爣绛撅級銆備袱涓?devm_phy_get 鍦ㄦ垚鍔熻幏鍙?PHY 鍚庯紝浣跨敤 devres 灏嗚澶囦笌 PHY 鍏宠仈銆?鍦ㄩ┍鍔ㄥ垎绂伙紙detach锛夋椂锛屼細鍦?devres 鏁版嵁涓婅皟鐢ㄩ噴鏀惧嚱鏁板苟閲婃斁 devres 鏁版嵁銆?_optional_get 鍙樹綋搴斿湪 phy 涓哄彲閫夋椂浣跨敤銆傝繖浜涘嚱鏁版案杩滀笉浼氳繑鍥?-ENODEV锛岃€屾槸鍦?鎵句笉鍒?phy 鏃惰繑鍥?NULL銆?
鏌愪簺閫氱敤椹卞姩锛堝 ehci锛夊彲鑳戒娇鐢ㄥ涓?phys銆傚湪杩欑鎯呭喌涓嬶紝
devm_of_phy_get 鎴?devm_of_phy_get_by_index 鍙敤浜庡熀浜庡悕绉版垨绱㈠紩鑾峰彇 phy 寮曠敤銆?
搴旀敞鎰忥紝NULL 鏄竴涓悎娉曠殑 phy 寮曠敤銆傛墍鏈夊 NULL phy 鐨?phy 娑堣垂鑰呰皟鐢ㄩ兘浼氬彉鎴?NOP锛堢┖鎿嶄綔锛夈€?鍗抽噴鏀捐皟鐢ㄣ€乸hy_init() 涓?phy_exit() 璋冪敤锛屼互鍙?phy_power_on() 涓?phy_power_off() 璋冪敤锛屽湪搴旂敤浜?NULL phy 鏃堕兘鏄?NOP銆侼ULL phy 鍦ㄥ鐞嗗彲閫?phy 璁惧鐨勫満鏅腑寰堟湁鐢ㄣ€?
## API 璋冪敤椤哄簭


```
    [devm_][of_]phy_get()
    phy_init()
    phy_power_on()
    [phy_set_mode[_ext]()]
    ...
    phy_power_off()
    phy_exit()
    [[of_]phy_put()]
```
鏌愪簺 PHY 椹卞姩鍙兘鏈疄鐜?`phy_init` 鎴?`phy_power_on`锛?浣嗘帶鍒跺櫒搴斿缁堣皟鐢ㄨ繖浜涘嚱鏁颁互鍏煎鍏朵粬 PHY銆傛煇浜?PHY 鍙兘闇€瑕?`phy_set_mode <phy_set_mode_ext>`锛?鑰屽叾浠栧垯鍙兘浣跨敤榛樿妯″紡锛堥€氬父閫氳繃 devicetree 鎴栧叾浠栧浐浠堕厤缃級銆備负浜嗗吋瀹规€э紝濡傛灉浣犵煡閬?灏嗕娇鐢ㄧ殑妯″紡锛屽簲濮嬬粓璋冪敤姝ゅ嚱鏁般€傞€氬父锛屾鍑芥暟搴斿湪 `phy_power_on` 涔嬪悗璋冪敤锛?灏界鏌愪簺 PHY 椹卞姩鍙兘鍏佽鍦ㄤ换浣曟椂鍊欒皟鐢ㄥ畠銆?
## 閲婃斁瀵?PHY 鐨勫紩鐢?

褰撴帶鍒跺櫒涓嶅啀闇€瑕佽 PHY 鏃讹紝瀹冨繀椤婚噴鏀句娇鐢ㄤ笂杩扮珷鑺傛彁鍒扮殑 API 鎵€鑾峰緱鐨?PHY 寮曠敤銆侾HY 妗嗘灦鎻愪緵浜?2 涓?API 鏉ラ噴鏀惧 PHY 鐨勫紩鐢ㄣ€?
```
	void phy_put(struct phy *phy);
	void devm_phy_put(struct device *dev, struct phy *phy);
```
杩欎袱涓?API 閮界敤浜庨噴鏀惧 PHY 鐨勫紩鐢紝devm_phy_put 浼氶攢姣佷笌姝?PHY 鍏宠仈鐨?devres銆?
## 閿€姣?PHY


褰撳垱寤鸿 PHY 鐨勯┍鍔ㄨ鍗歌浇鏃讹紝瀹冨簲閿€姣佸畠鍒涘缓鐨?PHY锛?
```
	void phy_destroy(struct phy *phy);
	void devm_phy_destroy(struct device *dev, struct phy *phy);
```
杩欎袱涓?API 閮戒細閿€姣?PHY锛宒evm_phy_destroy 浼氶攢姣佷笌姝?PHY 鍏宠仈鐨?devres銆?
## PM Runtime


姝ゅ瓙绯荤粺鍚敤浜?pm runtime锛堢數婧愮鐞嗚繍琛屾椂锛夈€傚洜姝ゅ湪鍒涘缓 PHY 鏃讹紝
浼氳皟鐢ㄦ瀛愮郴缁熷垱寤虹殑 phy device 鐨?pm_runtime_enable锛岃€屽湪閿€姣?PHY 鏃讹紝
浼氳皟鐢?pm_runtime_disable銆傛敞鎰忥紝姝ゅ瓙绯荤粺鍒涘缓鐨?phy device 灏嗘槸璋冪敤
phy_create锛圥HY provider 璁惧锛夌殑璁惧鐨勫瓙璁惧銆?
鍥犳锛屾瀛愮郴缁熷垱寤虹殑 phy_device 鐨?pm_runtime_get_sync 浼氱敱浜庣埗瀛愬叧绯昏€岃皟鐢?PHY provider 璁惧鐨?pm_runtime_get_sync銆傝繕搴旀敞鎰忥紝phy_power_on 涓?phy_power_off 鍒嗗埆鎵ц
phy_pm_runtime_get_sync 涓?phy_pm_runtime_put銆?杩樻湁涓€浜涘鍑虹殑 API锛屽 phy_pm_runtime_get銆乸hy_pm_runtime_get_sync銆?phy_pm_runtime_put 涓?phy_pm_runtime_put_sync锛岀敤浜庢墽琛?PM 鎿嶄綔銆?
## PHY 鏄犲皠


涓轰簡鍦ㄦ病鏈?DeviceTree 甯姪鐨勬儏鍐典笅鑾峰彇瀵?PHY 鐨勫紩鐢紝妗嗘灦鎻愪緵浜嗘煡鎵撅紙lookup锛夋満鍒讹紝绫讳技浜?clkdev锛?鍚庤€呭厑璁稿皢 clk 缁撴瀯缁戝畾鍒拌澶囥€傚綋宸茬粡瀛樺湪鎸囧悜 struct phy 鐨勫彞鏌勬椂锛屽彲浠ュ湪杩愯鏃惰繘琛屾煡鎵俱€?
妗嗘灦鎻愪緵浜嗕互涓?API 鐢ㄤ簬娉ㄥ唽鍜屾敞閿€鏌ユ壘锛?
```
	int phy_create_lookup(struct phy *phy, const char *con_id,
			      const char *dev_id);
	void phy_remove_lookup(struct phy *phy, const char *con_id,
			       const char *dev_id);
```

## DeviceTree 缁戝畾


PHY dt 缁戝畾鐨勬枃妗ｅ彲鍦ㄤ互涓嬩綅缃壘鍒帮細
Documentation/devicetree/bindings/phy/phy-bindings.txt
