## EISA 鎬荤嚎鏀寔


:Author: Marc Zyngier <maz@wild-wind.fr.eu.org>

鏈枃妗ｆ眹闆嗕簡鍏充簬灏?EISA 椹卞姩绉绘鍒版柊鐨?EISA/sysfs API 鐨勪竴浜涢浂鏁ｇ瑪璁般€?
浠?2.5.59 鐗堟湰寮€濮嬶紝EISA 鎬荤嚎鍑犱箮鑾峰緱浜嗕笌 PCI 鎴?USB 绛夊叾瀹冩洿涓绘祦鎬荤嚎鐩稿悓鐨?鍦颁綅銆傝繖閫氳繃 sysfs 寰椾互瀹炵幇锛宻ysfs 瀹氫箟浜嗕竴濂楄冻澶熷畬鍠勭殑鎶借薄鏉ョ鐞嗘€荤嚎銆佽澶囧拰
椹卞姩銆?
灏界鏂?API 鐢ㄨ捣鏉ョ浉褰撶畝鍗曪紝浣嗗皢鐜版湁椹卞姩杞崲鍒版柊鍩虹璁炬柦骞堕潪鏄撲簨锛堜富瑕佹槸鍥犱负
鎺㈡祴浠ｇ爜閫氬父涔熺敤浜庢帰娴?ISA 鍗★級銆傛澶栵紝澶у鏁?EISA 椹卞姩閮芥槸鏈€鑰佺殑涓€鎵?Linux
椹卞姩锛屾墍浠ュ彲鎯宠€岀煡锛岃繖浜涘勾閲岃繖閲岀Н浜嗕笉灏戠伆灏樸€?
EISA 鍩虹璁炬柦鐢变笁閮ㄥ垎缁勬垚锛?
    - 鎬荤嚎浠ｇ爜瀹炵幇浜嗗ぇ閮ㄥ垎閫氱敤浠ｇ爜銆傚畠鍦ㄨ繍琛?EISA 浠ｇ爜鐨勬墍鏈夋灦鏋勪箣闂村叡浜€傚畠
      瀹炵幇鎬荤嚎鎺㈡祴锛堟娴嬫€荤嚎涓婂彲鐢ㄧ殑 EISA 鍗★級銆佸垎閰?I/O 璧勬簮銆侀€氳繃 sysfs 瀹炵幇
      鑺卞摠鐨勫懡鍚嶏紝骞朵负椹卞姩鎻愪緵娉ㄥ唽鎺ュ彛銆?
    - 鎬荤嚎鏍归┍鍔ㄥ疄鐜颁簡鎬荤嚎纭欢涓庨€氱敤鎬荤嚎浠ｇ爜涔嬮棿鐨勭矘鍚堛€傚畠璐熻矗鍙戠幇瀹炵幇璇ユ€荤嚎鐨?      璁惧锛屽苟灏嗗叾璁剧疆濂戒互渚跨◢鍚庣敱鎬荤嚎浠ｇ爜鎺㈡祴銆傝繖鍙互鏄儚鍦?x86 涓婁繚鐣欎竴涓?I/O
      鍖哄煙杩欐牱绠€鍗曠殑浜嬫儏锛屼篃鍙互鏄儚 hppa 鐨?EISA 浠ｇ爜閭ｆ牱鐩稿綋澶嶆潅鐨勪簨鎯呫€傝繖鏄?      涓轰簡璁?EISA 鍦ㄢ€滄柊鈥濆钩鍙颁笂杩愯鑰岄渶瑕佸疄鐜扮殑閮ㄥ垎銆?
    - 椹卞姩鍚戞€荤嚎鎻愪緵瀹冩墍绠＄悊璁惧鐨勪竴涓垪琛紝骞跺疄鐜板繀瑕佺殑鍥炶皟锛屼互渚垮湪琚憡鐭ユ椂
      鎺㈡祴鍜岄噴鏀捐澶囥€?
涓嬮潰姣忎釜鍑芥暟/缁撴瀯浣撻兘浣嶄簬 <linux/eisa.h> 涓紝璇ユ枃浠朵弗閲嶄緷璧?<linux/device.h>銆?
## 鎬荤嚎鏍归┍鍔?

```

	int eisa_root_register (struct eisa_root_device *root);

```
eisa_root_register 鍑芥暟鐢ㄤ簬灏嗕竴涓澶囧０鏄庝负 EISA 鎬荤嚎鐨勬牴銆俥isa_root_device
缁撴瀯浣撴寔鏈変竴涓紩鐢?```

	struct eisa_root_device {
		struct device   *dev;	 /* Pointer to bridge device */
		struct resource *res;
		unsigned long    bus_base_addr;
		int		 slots;  /* Max slot number */
		int		 force_probe; /* Probe even when no slot 0 */
		u64		 dma_mask; /* from bridge device */
		int              bus_nr; /* Set by eisa_root_register */
		struct resource  eisa_root_res;	/* ditto */
	};

```
============= ======================================================
node          鐢ㄤ簬 eisa_root_register 鐨勫唴閮ㄧ敤閫?dev           鎸囧悜鏍硅澶囩殑鎸囬拡
res           鏍硅澶?I/O 璧勬簮
bus_base_addr 姝ゆ€荤嚎涓?slot 0 鐨勫湴鍧€
slots	     鏈€澶ф帰娴?slot 鍙?force_probe   鍗充娇 slot 0 涓虹┖锛堟棤 EISA 涓绘澘锛変篃杩涜鎺㈡祴
dma_mask      榛樿 DMA 鎺╃爜銆傞€氬父涓烘ˉ璁惧鐨?dma_mask銆?bus_nr	     鍞竴鎬荤嚎 id锛岀敱 eisa_root_register 璁剧疆
============= ======================================================

## 椹卞姩


```

	int eisa_driver_register (struct eisa_driver *edrv);
	void eisa_driver_unregister (struct eisa_driver *edrv);

```
澶熸竻妤氫簡鍚楋紵

```

	struct eisa_device_id {
		char sig[EISA_SIG_LEN];
		unsigned long driver_data;
	};

	struct eisa_driver {
		const struct eisa_device_id *id_table;
		struct device_driver         driver;
	};

```
=============== ====================================================
id_table	涓€涓互 NULL 缁撳熬鐨?EISA id 瀛楃涓叉暟缁勶紝
		鍚庤窡涓€涓┖瀛楃涓层€傛瘡涓瓧绗︿覆鍙€夋嫨鎬у湴
		涓庝竴涓┍鍔ㄧ浉鍏崇殑鍊硷紙driver_data锛夐厤瀵广€?
driver		涓€涓€氱敤椹卞姩锛屽
		Documentation/driver-api/driver-model/driver.rst
		鎵€杩般€傚彧鏈?.name銆?probe 鍜?.remove 鎴愬憳鏄繀濉殑銆?=============== ====================================================

```

	static struct eisa_device_id vortex_eisa_ids[] = {
		{ "TCM5920", EISA_3C592_OFFSET },
		{ "TCM5970", EISA_3C597_OFFSET },
		{ "" }
	};

	static struct eisa_driver vortex_eisa_driver = {
		.id_table = vortex_eisa_ids,
		.driver   = {
			.name    = "3c59x",
			.probe   = vortex_eisa_probe,
			.remove  = vortex_eisa_remove
		}
	};

```
## 璁惧


sysfs 妗嗘灦鍦ㄨ澶囪鍙戠幇鍜岀Щ闄ゆ椂璋冪敤 .probe 鍜?.remove 鍑芥暟锛堟敞鎰忥紝.remove 鍑芥暟
浠呭湪椹卞姩浣滀负妯″潡鏋勫缓鏃舵墠浼氳璋冪敤锛夈€?
杩欎袱涓嚱鏁伴兘浼犲叆涓€涓寚鍚?'struct device' 鐨勬寚閽堬紝璇ョ粨鏋勪綋涓?```

	struct eisa_device {
		struct eisa_device_id id;
		int                   slot;
		int                   state;
		unsigned long         base_addr;
		struct resource       res[EISA_MAX_RESOURCES];
		u64                   dma_mask;
		struct device         dev; /* generic device */
	};

```
======== ============================================================
id	 EISA id锛屼粠璁惧璇诲彇銆俰d.driver_data 浠庡尮閰嶇殑椹卞姩 EISA id 璁剧疆銆?slot	 妫€娴嬪埌璇ヨ澶囩殑 slot 鍙?state    涓€缁勬寚绀鸿澶囩姸鎬佺殑鏍囧織銆傚綋鍓嶇殑鏍囧織鏈?EISA_CONFIG_ENABLED 鍜?	 EISA_CONFIG_FORCED銆?res	 鍒嗛厤缁欒璁惧鐨勫洓缁?256 瀛楄妭 I/O 鍖哄煙
dma_mask 浠庣埗璁惧璁剧疆鐨?DMA 鎺╃爜
dev	 閫氱敤璁惧锛堝弬瑙?Documentation/driver-api/driver-model/device.rst锛?======== ============================================================

浣犲彲浠ヤ娇鐢?'to_eisa_device' 瀹忎粠 'struct device' 鑾峰彇 'struct eisa_device'銆?
## 鏉傞」


```

	void eisa_set_drvdata (struct eisa_device *edev, void *data);

```
灏嗘暟鎹瓨鍌ㄥ埌璁惧鐨?driver_data 鍖哄煙銆?
```

	void *eisa_get_drvdata (struct eisa_device *edev):

```
鑾峰彇鍏堝墠瀛樺偍鍒拌澶?driver_data 鍖哄煙鐨勬寚閽堛€?
```

	int eisa_get_region_index (void *addr);

```
杩斿洖缁欏畾鍦板潃鐨勫尯鍩熷彿锛? <= x < EISA_MAX_RESOURCES锛夈€?
## 鍐呮牳鍙傛暟


eisa_bus.enable_dev
	涓€涓互閫楀彿鍒嗛殧鐨勩€佽琚惎鐢ㄧ殑 slot 鍒楄〃锛屽嵆浣垮浐浠跺皢璇ュ崱璁句负绂佺敤銆傞┍鍔ㄥ繀椤?	鑳藉鍦ㄨ繖鏍风殑鏉′欢涓嬫纭湴鍒濆鍖栬澶囥€?
eisa_bus.disable_dev
	涓€涓互閫楀彿鍒嗛殧鐨勩€佽琚鐢ㄧ殑 slot 鍒楄〃锛屽嵆浣垮浐浠跺皢璇ュ崱璁句负鍚敤銆傞┍鍔ㄥ皢
	涓嶄細琚皟鐢ㄦ潵澶勭悊姝よ澶囥€?
virtual_root.force_probe
	寮哄埗鎺㈡祴浠ｇ爜鍘绘帰娴?EISA slot锛屽嵆浣垮畠鎵句笉鍒扮鍚?EISA 鐨勪富鏉匡紙slot 0 涓?	浠€涔堜篃娌℃湁鍑虹幇锛夈€傞粯璁や负 0锛堜笉寮哄埗锛夛紝褰撹缃簡 CONFIG_EISA_VLB_PRIMING
	鏃惰涓?1锛堝己鍒舵帰娴嬶級銆?
## 闆舵暎绗旇


灏?EISA 椹卞姩杞崲鍒版柊 API 涓昏娑夊強**鍒犻櫎**浠ｇ爜锛堝洜涓烘帰娴嬬幇鍦ㄤ綅浜庢牳蹇?EISA 浠ｇ爜
涓級銆傞仐鎲剧殑鏄紝澶у鏁伴┍鍔ㄥ湪 ISA 鍜?EISA 涔嬮棿鍏变韩瀹冧滑鐨勬帰娴嬩緥绋嬨€傚湪鍓ョ EISA
浠ｇ爜鏃跺繀椤荤壒鍒皬蹇冿紝浠ュ厤鍏跺畠鎬荤嚎閬彈杩欎簺鈥滃绉戞墜鏈紡鎵撳嚮鈥濈殑褰卞搷鈥︹€?
浣?*缁濅笉搴?*鏈熸湜浠?eisa_driver_register 杩斿洖鏃惰兘妫€娴嬪埌浠讳綍 EISA 璁惧锛屽洜涓烘€荤嚎
寰堝彲鑳藉皻鏈鎺㈡祴銆備簨瀹炰笂锛屽ぇ澶氭暟鏃跺€欐鏄姝わ紙鎬荤嚎鏍归┍鍔ㄩ€氬父鍦ㄥ惎鍔ㄨ繃绋嬩腑鐩稿綋
鏅氱殑鏃跺€欐墠浠嬪叆锛夈€傞仐鎲剧殑鏄紝澶у鏁伴┍鍔ㄩ兘鑷杩涜鎺㈡祴锛屽苟鏈熸湜鍦ㄩ€€鍑哄叾鎺㈡祴渚嬬▼鏃?宸茬粡鎺㈢储浜嗘暣鍙版満鍣ㄣ€?
渚嬪锛屽皢浣犲枩娆㈢殑 EISA SCSI 鍗″垏鎹㈠埌鈥滅儹鎻掓嫈鈥濇ā鍨嬫槸鈥滄纭箣涓锯€?tm)銆?
## 鑷磋阿


鎴戣鎰熻阿浠ヤ笅浜哄＋鐨勫府鍔╋細

- Xavier Benigni锛屽€熺粰鎴戜竴鍙扮粷濡欑殑 Alpha Jensen锛?- James Bottomley銆丣eff Garzik锛屽皢杩欓儴鍒嗕唬鐮佸悎鍏ュ唴鏍革紝
- Andries Brouwer锛岃础鐚簡澶ч噺 EISA id锛?- Catrin Jones锛屽湪瀹堕噷搴斾粯浜嗗お澶氱殑鏈哄櫒銆?