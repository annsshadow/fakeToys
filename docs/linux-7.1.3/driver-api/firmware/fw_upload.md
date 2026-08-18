
## 鍥轰欢涓婁紶 API


鍚戝浐浠跺姞杞藉櫒娉ㄥ唽鐨勮澶囬┍鍔ㄤ細鏆撮湶鎸佷箙鐨?sysfs 鑺傜偣锛屼娇鐢ㄦ埛鑳藉鍙戣捣閽堝璇ヨ澶囩殑
鍥轰欢鏇存柊銆傚鏀跺埌鐨勬暟鎹繘琛屼换浣曟牎楠岋紝鏄澶囬┍鍔ㄥ拰/鎴栬澶囨湰韬殑璐ｄ换銆傚浐浠朵笂浼?浣跨敤浜嗗浐浠跺洖閫€锛坒irmware fallback锛夋枃妗ｄ腑鎵€鎻忚堪鐨?**loading** 鍜?**data** 杩欎袱涓?sysfs 鏂囦欢锛屾澶栬繕鏂板浜嗚嫢骞?sysfs 鏂囦欢锛岀敤浜庢彁渚涘浐浠堕暅鍍忎紶杈撳埌璁惧杩囩▼涓殑鐘舵€佷俊鎭€?
## 娉ㄥ唽鍥轰欢涓婁紶


璁惧椹卞姩閫氳繃璋冪敤 firmware_upload_register() 鏉ユ敞鍐屽浐浠朵笂浼犮€傚湪鍙傛暟鍒楄〃涓寘鍚?涓€涓敤浜庡湪 /sys/class/firmware 涓嬫爣璇嗚璁惧鐨勫悕绉般€傜敤鎴峰彲浠ュ悜鐩爣璁惧鐨?**loading** sysfs 鏂囦欢鍐欏叆 1 鏉ュ彂璧蜂竴娆″浐浠朵笂浼犮€傛帴鐫€锛岀敤鎴峰皢鍥轰欢闀滃儚鍐欏叆
**data** sysfs 鏂囦欢銆傚啓瀹屽浐浠舵暟鎹悗锛岀敤鎴峰悜 **loading** sysfs 鏂囦欢鍐欏叆 0 琛ㄧず
浼犺緭瀹屾垚銆傚悜 **loading** 鍐欏叆 0 杩樹細瑙﹀彂鍦ㄥ唴瀛樺唴鏍稿伐浣滅嚎绋嬶紙worker thread锛変笂涓嬫枃
涓皢鍥轰欢浼犺緭缁欎笅灞傝澶囬┍鍔ㄣ€?
瑕佷娇鐢ㄥ浐浠朵笂浼?API锛岄渶缂栧啓涓€涓疄鐜颁簡鑻ュ共 ops 鐨勯┍鍔ㄣ€俻robe 鍑芥暟璋冪敤
firmware_upload_register()锛宺emove 鍑芥暟璋冪敤
```
firmware_upload_unregister()
```
銆?
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

### 鍥轰欢涓婁紶 Ops

   :identifiers: fw_upload_ops

### 鍥轰欢涓婁紶杩涘害鐮?
浠ヤ笅杩涘害鐮佺敱鍥轰欢鍔犺浇鍣ㄥ湪鍐呴儴浣跨敤銆傚搴旂殑瀛楃涓蹭細閫氳繃涓嬫枃鎻忚堪鐨?status sysfs
鑺傜偣涓婃姤锛屽苟鍦?ABI 鏂囨。涓湁璇存槑銆?
   :identifiers: fw_upload_prog

### 鍥轰欢涓婁紶閿欒鐮?
鍦ㄥけ璐ユ椂锛岄┍鍔?ops 鍙兘杩斿洖浠ヤ笅閿欒鐮侊細

   :identifiers: fw_upload_err

## Sysfs 灞炴€?

闄や簡 **loading** 鍜?**data** 杩欎袱涓?sysfs 鏂囦欢澶栵紝杩樻湁棰濆鐨?sysfs 鏂囦欢鐢ㄤ簬鐩戣
鏁版嵁浼犺緭鍒扮洰鏍囪澶囩殑鐘舵€侊紝骞剁‘瀹氫紶杈撴渶缁堢殑鎴愬姛/澶辫触鐘舵€併€傛牴鎹澶囧強鍥轰欢闀滃儚
澶у皬鐨勪笉鍚岋紝涓€娆″浐浠舵洿鏂板彲鑳借€楁椂鏁版绉掑埌鏁板垎閽熶笉绛夈€?
棰濆鐨?sysfs 鏂囦欢濡備笅锛?
- status - 鎻愪緵鍥轰欢鏇存柊杩涘害鐨勬寚绀?- error - 鎻愪緵澶辫触鍥轰欢鏇存柊鐨勯敊璇俊鎭?- remaining_size - 璺熻釜涓€娆℃洿鏂颁腑鏁版嵁浼犺緭鐨勯儴鍒?- cancel - 鍚戣鏂囦欢鍐欏叆 1 浠ュ彇娑堟洿鏂?