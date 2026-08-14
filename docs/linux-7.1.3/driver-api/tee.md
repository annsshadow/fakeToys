## TEE锛堝彲淇℃墽琛岀幆澧冿級椹卞姩 API


鍐呮牳鎻愪緵 TEE 鎬荤嚎鍩虹璁炬柦锛屽叾涓彲淇″簲鐢ㄧ▼搴忚琛ㄧず涓洪€氳繃閫氱敤鍞竴鏍囪瘑绗?锛圲UID锛夋爣璇嗙殑璁惧锛屽鎴风椹卞姩娉ㄥ唽涓€寮犲彈鏀寔璁惧 UUID 鐨勮〃銆?
TEE 鎬荤嚎鍩虹璁炬柦娉ㄥ唽浠ヤ笅 API锛?
match()锛?  閬嶅巻瀹㈡埛绔┍鍔?UUID 琛紝涓鸿澶?UUID 鏌ユ壘瀵瑰簲鐨勫尮閰嶃€傚鏋滄壘鍒板尮閰嶏紝鍒欓€氳繃
  瀹㈡埛绔┍鍔ㄦ敞鍐岀殑鐩稿簲 probe API 鎺㈡祴璇ョ壒瀹氳澶囥€傛瘡褰撹澶囨垨瀹㈡埛绔┍鍔ㄥ湪 TEE
  鎬荤嚎涓婃敞鍐屾椂锛岄兘浼氬彂鐢熸杩囩▼銆?
uevent()锛?  姣忓綋 TEE 鎬荤嚎涓婃敞鍐屾柊璁惧鏃堕€氱煡鐢ㄦ埛绌洪棿锛坲dev锛夛紝浠ヨ嚜鍔ㄥ姞杞芥ā鍧楀寲鐨勫鎴风
  椹卞姩銆?
TEE 鎬荤嚎璁惧鏋氫妇鐗瑰畾浜庡簳灞?TEE 瀹炵幇锛屽洜姝ょ暀缁?TEE 椹卞姩鎻愪緵鐩稿簲鐨勫疄鐜般€?
鐒跺悗 TEE 瀹㈡埛绔┍鍔ㄥ彲浠ヤ娇鐢?include/linux/tee_drv.h 涓垪鍑虹殑 API 涓庡尮閰嶇殑
Trusted Application 閫氫俊銆?
### TEE 瀹㈡埛绔┍鍔ㄧず渚?

鍋囪鏌愪釜 TEE 瀹㈡埛绔┍鍔ㄩ渶瑕佷笌涓€涓叿鏈変互涓?UUID 鐨?Trusted Application 閫氫俊锛?`ac6a4085-0e82-4c33-bf98-8eb8e118b6c2`锛屽垯椹卞姩娉ㄥ唽濡備笅锛?
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
