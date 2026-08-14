
## NVMEM 瀛愮郴缁?

 Srinivas Kandagatla <srinivas.kandagatla@linaro.org>

鏈枃妗ｈВ閲?NVMEM 妗嗘灦鍙婂叾鎻愪緵鐨?API锛屼互鍙婂浣曚娇鐢ㄥ畠銆?
## 1. 绠€浠?
**NVMEM** 鏄?Non Volatile Memory锛堥潪鏄撳け鎬у唴瀛橈級灞傜殑缂╁啓銆傚畠鐢ㄤ簬浠?eeprom銆乪fuse 绛夐潪
鏄撳け鎬у瓨鍌ㄥ櫒涓绱?SoC 鎴栬澶囩壒瀹氱殑閰嶇疆鏁版嵁銆?
鍦ㄨ繖涓鏋跺嚭鐜颁箣鍓嶏紝鍍?eeprom 杩欐牱鐨?NVMEM 椹卞姩瀛樻斁鍦?drivers/misc 涓紝瀹冧滑閮戒笉寰椾笉
閲嶅鍑犱箮鐩稿悓鐨勪唬鐮佹潵娉ㄥ唽涓€涓?sysfs 鏂囦欢銆佸厑璁稿唴鏍告€佺敤鎴疯闂畠浠墍椹卞姩璁惧鐨勫唴瀹癸紝绛夌瓑銆?
灏卞叾浠栧唴鏍告€佺敤鎴疯€岃█杩欎篃鏄竴涓棶棰橈紝鍥犱负鎵€浣跨敤鐨勬柟妗堝湪涓嶅悓椹卞姩涔嬮棿宸紓寰堝ぇ锛屽瓨鍦ㄧ浉褰?涓ラ噸鐨勬娊璞℃硠婕忋€?
璇ユ鏋舵棬鍦ㄨВ鍐宠繖浜涢棶棰樸€傚畠杩樺紩鍏ヤ簡璁惧鏍戯紙DT锛夎〃绀猴紝渚涙秷璐硅€呰澶囦粠 NVMEM 涓幏鍙栧畠浠?鎵€闇€鐨勬暟鎹紙MAC 鍦板潃銆丼oC/鐗堟湰 ID銆侀儴浠跺彿绛夛級銆?
NVMEM Providers
+++++++++++++++

NVMEM 鎻愪緵鑰咃紙provider锛夋寚瀹炵幇浜嗗垵濮嬪寲銆佽鍙栧拰鍐欏叆闈炴槗澶辨€у唴瀛樻柟娉曠殑瀹炰綋銆?
## 2. 娉ㄥ唽/娉ㄩ攢 NVMEM 鎻愪緵鑰?

NVMEM 鎻愪緵鑰呭彲浠ラ€氳繃鍚?nvmem_register() 鎻愪緵鐩稿叧鐨?nvmem 閰嶇疆鏉ュ悜 NVMEM 鏍稿績娉ㄥ唽锛屾垚鍔熸椂
鏍稿績浼氳繑鍥炰竴涓湁鏁堢殑 nvmem_device 鎸囬拡銆?
nvmem_unregister() 鐢ㄤ簬娉ㄩ攢鍏堝墠娉ㄥ唽鐨勬彁渚涜€呫€?
```

  static int brcm_nvram_probe(struct platform_device *pdev)
  {
	struct nvmem_config config = {
		.name = "brcm-nvram",
		.reg_read = brcm_nvram_read,
	};
	...
	config.dev = &pdev->dev;
	config.priv = priv;
	config.size = resource_size(res);

	devm_nvmem_register(&config);
  }

```
璁惧椹卞姩鍙互浣跨敤 nvmem_cell_info 瀹氫箟骞舵敞鍐屼竴涓?nvmem cell锛?
```

  static const struct nvmem_cell_info foo_nvmem_cell = {
	{
		.name		= "macaddr",
		.offset		= 0x7f00,
		.bytes		= ETH_ALEN,
	}
  };

  int nvmem_add_one_cell(nvmem, &foo_nvmem_cell);

```
姝ゅ锛岃繕鍙互鍒涘缓 nvmem cell 鏌ユ壘椤瑰苟娉ㄥ唽锛?
```

  static struct nvmem_cell_lookup foo_nvmem_lookup = {
	.nvmem_name		= "i2c-eeprom",
	.cell_name		= "macaddr",
	.dev_id			= "foo_mac.0",
	.con_id			= "mac-address",
  };

  nvmem_add_cell_lookups(&foo_nvmem_lookup, 1);

```
NVMEM Consumers
+++++++++++++++

NVMEM 娑堣垂鑰咃紙consumer锛夋槸鍒╃敤 NVMEM 鎻愪緵鑰呰繘琛岃鍙栦笌鍐欏叆鐨勫疄浣撱€?
## 3. 鍩轰簬 NVMEM cell 鐨勬秷璐硅€?API


NVMEM cell 鏄?NVMEM 涓殑鏁版嵁鏉＄洰/瀛楁銆?
```

  struct nvmem_cell *nvmem_cell_get(struct device *dev, const char *name);
  struct nvmem_cell *devm_nvmem_cell_get(struct device *dev, const char *name);

  void nvmem_cell_put(struct nvmem_cell *cell);
  void devm_nvmem_cell_put(struct device *dev, struct nvmem_cell *cell);

  void *nvmem_cell_read(struct nvmem_cell *cell, ssize_t *len);
  int nvmem_cell_write(struct nvmem_cell *cell, void *buf, ssize_t len);

```
`*nvmem_cell_get()` API 浼氳幏鍙栫粰瀹?id 鐨?nvmem cell 鐨勫紩鐢紝闅忓悗 nvmem_cell_read/write()
鍙互璇诲彇鎴栧啓鍏ヨ cell銆備竴鏃?cell 鐨勪娇鐢ㄧ粨鏉燂紝娑堣垂鑰呭簲璋冪敤 `*nvmem_cell_put()` 鏉ラ噴鏀句负璇?cell 鍒嗛厤鐨勬墍鏈夊唴瀛樸€?
## 4. 鍩轰簬鐩存帴 NVMEM 璁惧鐨勬秷璐硅€?API


鍦ㄦ煇浜涙儏鍐典笅锛屾湁蹇呰鐩存帴璇诲彇/鍐欏叆 NVMEM銆?
```

  struct nvmem_device *nvmem_device_get(struct device *dev, const char *name);
  struct nvmem_device *devm_nvmem_device_get(struct device *dev,
					   const char *name);
  struct nvmem_device *nvmem_device_find(void *data,
			int (*match)(struct device *dev, const void *data));
  void nvmem_device_put(struct nvmem_device *nvmem);
  int nvmem_device_read(struct nvmem_device *nvmem, unsigned int offset,
		      size_t bytes, void *buf);
  int nvmem_device_write(struct nvmem_device *nvmem, unsigned int offset,
		       size_t bytes, void *buf);
  int nvmem_device_cell_read(struct nvmem_device *nvmem,
			   struct nvmem_cell_info *info, void *buf);
  int nvmem_device_cell_write(struct nvmem_device *nvmem,
			    struct nvmem_cell_info *info, void *buf);

```
鍦ㄦ秷璐硅€呭彲浠ョ洿鎺ヨ鍙?鍐欏叆 NVMEM 涔嬪墠锛屽畠搴斿綋閫氳繃鏌愪釜 `*nvmem_device_get()` API 鑾峰彇
nvmem_controller銆?
杩欎簺 API 涓庡熀浜?cell 鐨?API 涔嬮棿鐨勫尯鍒湪浜庯紝杩欎簺 API 鎬绘槸浠?nvmem_device 浣滀负鍙傛暟銆?
## 5. 閲婃斁瀵?NVMEM 鐨勫紩鐢?

褰撴秷璐硅€呬笉鍐嶉渶瑕?NVMEM 鏃讹紝瀹冨繀椤婚噴鏀句娇鐢ㄤ笂杩扮珷鑺傛墍杩?API 鑾峰彇鐨?NVMEM 寮曠敤銆?
```

  void nvmem_cell_put(struct nvmem_cell *cell);
  void devm_nvmem_cell_put(struct device *dev, struct nvmem_cell *cell);
  void nvmem_device_put(struct nvmem_device *nvmem);
  void devm_nvmem_device_put(struct device *dev, struct nvmem_device *nvmem);

```
杩欎袱涓?API 閮界敤浜庨噴鏀惧 NVMEM 鐨勫紩鐢紝鑰?devm_nvmem_cell_put 涓?devm_nvmem_device_put
浼氶攢姣佷笌姝?NVMEM 鍏宠仈鐨?devres銆?
Userspace
+++++++++

## 6. 鐢ㄦ埛绌洪棿浜岃繘鍒舵帴鍙?

```

	/sys/bus/nvmem/devices/*/nvmem

```
```

  hexdump /sys/bus/nvmem/devices/qfprom0/nvmem

  0000000 0000 0000 0000 0000 0000 0000 0000 0000
  *
  00000a0 db10 2240 0000 e000 0c00 0c00 0000 0c00
  0000000 0000 0000 0000 0000 0000 0000 0000 0000
  ...
  *
  0001000

```
## 7. 璁惧鏍戠粦瀹?

鍙傝 Documentation/devicetree/bindings/nvmem/nvmem.txt

## 8. NVMEM 甯冨眬


NVMEM 甯冨眬鏄彟涓€绉嶅垱寤?cell 鐨勬満鍒躲€傚€熷姪璁惧鏍戠粦瀹氾紝鍙互閫氳繃浣跨敤鍋忕Щ涓庨暱搴︽潵鎸囧畾绠€鍗?鐨?cell銆傛湁鏃讹紝cell 娌℃湁闈欐€佸亸绉伙紝浣嗗唴瀹逛粛鐒跺畾涔夎壇濂斤紝渚嬪 tag-length-values銆傚湪杩欑
鎯呭喌涓嬶紝蹇呴』鍏堣В鏋?NVMEM 璁惧鐨勫唴瀹癸紝骞剁浉搴斿湴娣诲姞 cell銆傚竷灞€璁╀綘鑳藉璇诲彇 NVMEM 璁惧鐨?鍐呭锛屽苟鍔ㄦ€佸湴娣诲姞 cell銆?
甯冨眬鐨勫彟涓€涓敤渚嬫槸瀵?cell 杩涜鍚庡鐞嗐€傞€氳繃甯冨眬锛屽彲浠ュ皢涓€涓嚜瀹氫箟鐨勫悗澶勭悊閽╁瓙鍏宠仈鍒版煇涓?cell銆傜敋鑷冲彲浠ュ皢姝ら挬瀛愭坊鍔犲埌骞堕潪鐢卞竷灞€鏈韩鍒涘缓鐨?cell 涓娿€?
## 9. 鍐呴儴鍐呮牳 API


   :export:
