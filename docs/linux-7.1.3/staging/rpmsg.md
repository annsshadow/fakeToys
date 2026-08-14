## 杩滅▼澶勭悊鍣ㄦ秷鎭紶閫掞紙Remote Processor Messaging锛宺pmsg锛夋鏋?

  鏈枃妗ｆ弿杩颁簡 rpmsg 鎬荤嚎浠ュ強濡備綍缂栧啓 rpmsg 椹卞姩銆傝浜嗚В濡備綍涓烘柊骞冲彴娣诲姞 rpmsg 鏀寔锛?  璇锋煡鐪?remoteproc.txt锛堝悓鏍蜂綅浜?Documentation/ 涓級銆?
## 绠€浠?
鐜颁唬 SoC 閫氬父閲囩敤寮傛瀯杩滅▼澶勭悊鍣ㄨ澶囷紝浠ラ潪瀵圭О澶氬鐞嗭紙AMP锛夐厤缃繍琛岋紝杩欎簺澶勭悊鍣ㄥ彲鑳?杩愯涓嶅悓瀹炰緥鐨勬搷浣滅郴缁燂紝鏃犺鏄?Linux 杩樻槸浠讳綍鍏朵粬瀹炴椂鎿嶄綔绯荤粺銆?
渚嬪锛孫MAP4 鎷ユ湁鍙屾牳 Cortex-A9銆佸弻鏍?Cortex-M3 浠ュ強涓€涓?C64x+ DSP銆傞€氬父锛屽弻鏍?Cortex-A9
浠?SMP 閰嶇疆杩愯 Linux锛岃€屽叾浠栦笁涓牳蹇冿紙涓や釜 M3 鏍稿績鍜屼竴涓?DSP锛夊悇鑷互 AMP 閰嶇疆杩愯鑷繁鐨?RTOS 瀹炰緥銆?
閫氬父 AMP 杩滅▼澶勭悊鍣ㄤ細浣跨敤涓撶敤鐨?DSP 缂栬В鐮佸櫒鍜屽濯掍綋纭欢鍔犻€熷櫒锛屽洜姝ゅ父琚敤鏉ュ皢 CPU 瀵嗛泦鐨?澶氬獟浣撲换鍔′粠涓诲簲鐢ㄥ鐞嗗櫒涓婂嵏杞戒笅鏉ャ€?
杩欎簺杩滅▼澶勭悊鍣ㄤ篃鍙敤浜庢帶鍒跺寤惰繜鏁忔劅鐨勪紶鎰熷櫒銆侀┍鍔ㄥ悇绉嶇‖浠舵ā鍧楋紝鎴栬€呬粎浠呭湪涓?CPU 绌洪棽鏃?鎵ц鍚庡彴浠诲姟銆?
杩欎簺杩滅▼澶勭悊鍣ㄧ殑浣跨敤鑰呮棦鍙互鏄敤鎴锋€佸簲鐢紙渚嬪涓庤繙绋?OMX 缁勪欢閫氫俊鐨勫濯掍綋妗嗘灦锛夛紝涔熷彲浠?鏄唴鏍搁┍鍔紙鎺у埗鍙湁杩滅▼澶勭悊鍣ㄦ墠鑳借闂殑纭欢銆佷唬琛ㄨ繙绋嬪鐞嗗櫒淇濈暀鍐呮牳鎺у埗鐨勮祫婧愮瓑锛夈€?
Rpmsg 鏄竴涓熀浜?virtio 鐨勬秷鎭紶閫掓€荤嚎锛屽厑璁稿唴鏍搁┍鍔ㄤ笌绯荤粺涓婂彲鐢ㄧ殑杩滅▼澶勭悊鍣ㄩ€氫俊銆傝繘鑰岋紝
椹卞姩鍙互鍦ㄩ渶瑕佹椂鏆撮湶鍚堥€傜殑鐢ㄦ埛绌洪棿鎺ュ彛銆?
鍦ㄧ紪鍐欏悜鐢ㄦ埛鎬佹毚闇?rpmsg 閫氫俊鐨勯┍鍔ㄦ椂锛岃璁颁綇杩滅▼澶勭悊鍣ㄥ彲鑳界洿鎺ヨ闂郴缁熺殑鐗╃悊鍐呭瓨鍜屽叾浠?鏁忔劅纭欢璧勬簮锛堜緥濡傦紝鍦?OMAP4 涓婏紝杩滅▼鏍稿績鍜岀‖浠跺姞閫熷櫒鍙兘鐩存帴璁块棶鐗╃悊鍐呭瓨銆乬pio bank銆?dma controller銆乮2c 鎬荤嚎銆乬ptimer銆乵ailbox 璁惧銆乭wspinlock 绛夛級銆傛澶栵紝杩欎簺杩滅▼澶勭悊鍣?鍙兘杩愯 RTOS锛屽叾涓瘡涓换鍔￠兘鍙互璁块棶鏆撮湶缁欒澶勭悊鍣ㄧ殑鏁翠釜鍐呭瓨/璁惧銆備负浜嗗皢鎭舵剰锛堟垨鏈夌己闄凤級
鐨勭敤鎴锋€佷唬鐮佸埄鐢ㄨ繙绋嬫紡娲炲苟鍊熸鎺ョ绯荤粺鐨勯闄╅檷鍒版渶浣庯紝閫氬父甯屾湜灏嗙敤鎴锋€侀檺鍒跺湪瀹冨彲浠ュ彂閫?娑堟伅鐨勭壒瀹?rpmsg 閫氶亾涓婏紙瑙佷笅瀹氫箟锛夛紝骞跺湪鍙兘鐨勬儏鍐典笅灏介噺鍑忓皯瀹冨娑堟伅鍐呭鐨勬帶鍒躲€?
姣忎釜 rpmsg 璁惧閮芥槸涓庤繙绋嬪鐞嗗櫒鐨勪竴鏉￠€氫俊閫氶亾锛堝洜姝?rpmsg 璁惧琚О涓洪€氶亾锛夈€傞€氶亾鐢辨枃鏈悕绉?鏍囪瘑锛屽苟鍏锋湁涓€涓湰鍦帮紙鈥滄簮鈥濓級rpmsg 鍦板潃鍜屼竴涓繙绋嬶紙鈥滅洰鐨勨€濓級rpmsg 鍦板潃銆?
褰撻┍鍔ㄥ紑濮嬪湪鏌愪釜閫氶亾涓婄洃鍚椂锛屽叾 rx 鍥炶皟鍑芥暟浼氱粦瀹氬埌涓€涓敮涓€鐨?rpmsg 鏈湴鍦板潃锛堜竴涓?32 浣?鏁存暟锛夈€傝繖鏍凤紝褰撳叆绔欐秷鎭埌杈炬椂锛宺pmsg 鏍稿績浼氭牴鎹叾鐩殑鍦板潃灏嗗畠浠垎娲剧粰鍚堥€傜殑椹卞姩锛堣繖鏄€氳繃
鐢ㄥ叆绔欐秷鎭殑璐熻浇璋冪敤椹卞姩鐨?rx 澶勭悊绋嬪簭鏉ュ畬鎴愮殑锛夈€?
## 鐢ㄦ埛 API

```
  int rpmsg_send(struct rpmsg_endpoint *ept, void *data, int len);

```
浠庣粰瀹氱鐐瑰悜杩滅▼澶勭悊鍣ㄥ彂閫佷竴鏉℃秷鎭€傝皟鐢ㄨ€呭簲鎸囧畾绔偣銆佹兂瑕佸彂閫佺殑鏁版嵁鍙婂叾闀垮害锛堜互瀛楄妭涓哄崟浣嶏級銆?娑堟伅灏嗗湪鎸囧畾绔偣鐨勯€氶亾涓婂彂閫侊紝鍗冲叾婧愬湴鍧€鍜岀洰鐨勫湴鍧€瀛楁灏嗗垎鍒璁句负绔偣鐨?src 鍦板潃鍙婂叾鐖堕€氶亾鐨?dst 鍦板潃銆?
濡傛灉娌℃湁鍙敤鐨?TX 缂撳啿鍖猴紝璇ュ嚱鏁颁細闃诲锛岀洿鍒版湁涓€涓彉寰楀彲鐢紙鍗崇洿鍒拌繙绋嬪鐞嗗櫒娑堣垂浜嗕竴涓?tx
缂撳啿鍖哄苟灏嗗叾鏀惧洖 virtio 鐨?used descriptor ring锛夛紝鎴栬€呯洿鍒?15 绉掕秴鏃躲€傚綋鍚庤€呭彂鐢熸椂锛岃繑鍥?-ERESTARTSYS銆?
璇ュ嚱鏁扮洰鍓嶅彧鑳戒粠杩涚▼涓婁笅鏂囦腑璋冪敤銆傛垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖鐩稿簲鐨勯敊璇€笺€?
```
  int rpmsg_sendto(struct rpmsg_endpoint *ept, void *data, int len, u32 dst);

```
浠庣粰瀹氱鐐瑰悜杩滅▼澶勭悊鍣ㄥ彂閫佷竴鏉℃秷鎭紝鐩殑鍦板潃鐢辫皟鐢ㄨ€呮彁渚涖€?
璋冪敤鑰呭簲鎸囧畾绔偣銆佹兂瑕佸彂閫佺殑鏁版嵁銆佸叾闀垮害锛堜互瀛楄妭涓哄崟浣嶏級锛屼互鍙婁竴涓樉寮忕殑鐩殑鍦板潃銆?
娑堟伅闅忓悗灏嗕娇鐢ㄧ鐐圭殑 src 鍦板潃鍜岀敤鎴锋彁渚涚殑 dst 鍦板潃锛屽彂閫佸埌绔偣鎵€灞為€氶亾鎵€瀵瑰簲鐨勮繙绋嬪鐞嗗櫒
锛堝洜姝ら€氶亾鐨?dst 鍦板潃浼氳蹇界暐锛夈€?
濡傛灉娌℃湁鍙敤鐨?TX 缂撳啿鍖猴紝璇ュ嚱鏁颁細闃诲锛岀洿鍒版湁涓€涓彉寰楀彲鐢紙鍗崇洿鍒拌繙绋嬪鐞嗗櫒娑堣垂浜嗕竴涓?tx
缂撳啿鍖哄苟灏嗗叾鏀惧洖 virtio 鐨?used descriptor ring锛夛紝鎴栬€呯洿鍒?15 绉掕秴鏃躲€傚綋鍚庤€呭彂鐢熸椂锛岃繑鍥?-ERESTARTSYS銆?
璇ュ嚱鏁扮洰鍓嶅彧鑳戒粠杩涚▼涓婁笅鏂囦腑璋冪敤銆傛垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖鐩稿簲鐨勯敊璇€笺€?
```
  int rpmsg_trysend(struct rpmsg_endpoint *ept, void *data, int len);

```
浠庣粰瀹氱鐐瑰悜杩滅▼澶勭悊鍣ㄥ彂閫佷竴鏉℃秷鎭€傝皟鐢ㄨ€呭簲鎸囧畾绔偣銆佹兂瑕佸彂閫佺殑鏁版嵁鍙婂叾闀垮害锛堜互瀛楄妭涓哄崟浣嶏級銆?娑堟伅灏嗗湪鎸囧畾绔偣鐨勯€氶亾涓婂彂閫侊紝鍗冲叾婧愬湴鍧€鍜岀洰鐨勫湴鍧€瀛楁灏嗗垎鍒璁句负绔偣鐨?src 鍦板潃鍙婂叾鐖堕€氶亾鐨?dst 鍦板潃銆?
濡傛灉娌℃湁鍙敤鐨?TX 缂撳啿鍖猴紝璇ュ嚱鏁颁細绔嬪嵆杩斿洖 -ENOMEM锛岃€屼笉绛夊緟鏈夌紦鍐插尯鍙樺緱鍙敤銆?
璇ュ嚱鏁扮洰鍓嶅彧鑳戒粠杩涚▼涓婁笅鏂囦腑璋冪敤銆傛垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖鐩稿簲鐨勯敊璇€笺€?
```
  int rpmsg_trysendto(struct rpmsg_endpoint *ept, void *data, int len, u32 dst)


```
浠庣粰瀹氱鐐瑰悜杩滅▼澶勭悊鍣ㄥ彂閫佷竴鏉℃秷鎭紝鐩殑鍦板潃鐢辩敤鎴锋彁渚涖€?
鐢ㄦ埛搴旀寚瀹氶€氶亾銆佹兂瑕佸彂閫佺殑鏁版嵁銆佸叾闀垮害锛堜互瀛楄妭涓哄崟浣嶏級锛屼互鍙婁竴涓樉寮忕殑鐩殑鍦板潃銆?
娑堟伅闅忓悗灏嗕娇鐢ㄩ€氶亾鐨?src 鍦板潃鍜岀敤鎴锋彁渚涚殑 dst 鍦板潃鍙戦€佸埌閫氶亾鎵€灞炵殑杩滅▼澶勭悊鍣紙鍥犳閫氶亾鐨?dst 鍦板潃浼氳蹇界暐锛夈€?
濡傛灉娌℃湁鍙敤鐨?TX 缂撳啿鍖猴紝璇ュ嚱鏁颁細绔嬪嵆杩斿洖 -ENOMEM锛岃€屼笉绛夊緟鏈夌紦鍐插尯鍙樺緱鍙敤銆?
璇ュ嚱鏁扮洰鍓嶅彧鑳戒粠杩涚▼涓婁笅鏂囦腑璋冪敤銆傛垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖鐩稿簲鐨勯敊璇€笺€?
```
  struct rpmsg_endpoint *rpmsg_create_ept(struct rpmsg_device *rpdev,
					  rpmsg_rx_cb_t cb, void *priv,
					  struct rpmsg_channel_info chinfo);

```
绯荤粺涓瘡涓€涓?rpmsg 鍦板潃閮介€氳繃涓€涓?rpmsg_endpoint 缁撴瀯缁戝畾鍒颁竴涓?rx 鍥炶皟鍑芥暟锛堝洜姝ゅ綋鍏ョ珯娑堟伅
鍒拌揪鏃讹紝瀹冧滑鐢?rpmsg 鎬荤嚎浣跨敤鐩稿簲鐨勫洖璋冨鐞嗙▼搴忔潵鍒嗘淳锛夈€?
璇ュ嚱鏁板厑璁搁┍鍔ㄥ垱寤鸿繖鏍蜂竴涓鐐癸紝骞跺€熸灏嗕竴涓洖璋冿紙鍙兘杩樻湁鏌愪簺绉佹湁鏁版嵁锛夌粦瀹氬埌涓€涓?rpmsg 鍦板潃
锛堟棦鍙互鏄鍏堝凡鐭ョ殑鍦板潃锛屼篃鍙互鏄负瀹冧滑鍔ㄦ€佸垎閰嶇殑鍦板潃锛夈€?
绠€鍗曠殑 rpmsg 椹卞姩鏃犻渶璋冪敤 rpmsg_create_ept锛屽洜涓哄綋瀹冧滑琚?rpmsg 鎬荤嚎鎺㈡祴锛坧robe锛夋椂锛屽凡缁忎负瀹冧滑
鍒涘缓浜嗕竴涓鐐癸紙浣跨敤瀹冧滑鍚?rpmsg 鎬荤嚎娉ㄥ唽鏃舵彁渚涚殑 rx 鍥炶皟锛夈€?
鍥犳瀵逛簬绠€鍗曢┍鍔ㄦ潵璇翠竴鍒囧簲褰撳紑绠卞嵆鐢細瀹冧滑宸茬粡鎷ユ湁绔偣锛屽叾 rx 鍥炶皟缁戝畾鍒颁簡瀹冧滑鐨?rpmsg 鍦板潃锛?褰撶浉鍏崇殑鍏ョ珯娑堟伅鍒拌揪鏃讹紙鍗崇洰鐨勫湴鍧€绛変簬鍏?rpmsg 閫氶亾 src 鍦板潃鐨勬秷鎭級锛岄┍鍔ㄧ殑 handler 浼氳
璋冪敤鏉ュ鐞嗗畠銆?
涔熷氨鏄锛屾洿澶嶆潅鐨勯┍鍔ㄥ彲鑳界‘瀹為渶瑕佹湁棰濆鍒嗛厤鐨?rpmsg 鍦板潃锛屽苟灏嗗畠浠粦瀹氬埌涓嶅悓鐨?rx 鍥炶皟銆?涓烘锛岃繖浜涢┍鍔ㄩ渶瑕佽皟鐢ㄨ鍑芥暟銆傞┍鍔ㄥ簲鎻愪緵瀹冧滑鐨勯€氶亾锛堣繖鏍锋柊绔偣浼氱粦瀹氬埌鍏堕€氶亾鎵€灞炵殑鍚屼竴杩滅▼
澶勭悊鍣級銆佷竴涓?rx 鍥炶皟鍑芥暟銆佸彲閫夌殑绉佹湁鏁版嵁锛堝湪 rx 鍥炶皟琚皟鐢ㄦ椂浼氫紶鍥烇級锛屼互鍙婂畠浠兂瑕佺粦瀹氬洖璋冪殑
鍦板潃銆傚鏋?addr 涓?RPMSG_ADDR_ANY锛岄偅涔?rpmsg_create_ept 浼氫负瀹冧滑鍔ㄦ€佸垎閰嶄竴涓彲鐢ㄧ殑 rpmsg 鍦板潃
锛堥┍鍔ㄥ簲褰撴湁闈炲父鍏呭垎鐨勭悊鐢辨墠涓嶅湪杩欓噷濮嬬粓浣跨敤 RPMSG_ADDR_ANY锛夈€?
鎴愬姛鏃惰繑鍥炴寚鍚戠鐐圭殑鎸囬拡锛屽嚭閿欐椂杩斿洖 NULL銆?
```
  void rpmsg_destroy_ept(struct rpmsg_endpoint *ept);


```
閿€姣佷竴涓凡瀛樺湪鐨?rpmsg 绔偣銆傜敤鎴峰簲鎻愪緵涓€涓箣鍓嶇敱 rpmsg_create_ept() 鍒涘缓鐨?rpmsg 绔偣鎸囬拡銆?
```
  int register_rpmsg_driver(struct rpmsg_driver *rpdrv);


```
鍚?rpmsg 鎬荤嚎娉ㄥ唽涓€涓?rpmsg 椹卞姩銆傜敤鎴峰簲鎻愪緵涓€涓寚鍚?rpmsg_driver 缁撴瀯鐨勬寚閽堬紝鍏朵腑鍖呭惈椹卞姩鐨?->probe() 鍜?->remove() 鍑芥暟銆佷竴涓?rx 鍥炶皟锛屼互鍙婁竴涓?id_table锛屾寚瀹氳椹卞姩甯屾湜琚帰娴嬪埌鐨勯€氶亾
鍚嶇О銆?
```
  void unregister_rpmsg_driver(struct rpmsg_driver *rpdrv);


```
浠?rpmsg 鎬荤嚎娉ㄩ攢涓€涓?rpmsg 椹卞姩銆傜敤鎴峰簲鎻愪緵涓€涓箣鍓嶆敞鍐岀殑 rpmsg_driver 缁撴瀯鎸囬拡銆?鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖鐩稿簲鐨勯敊璇€笺€?
## 鍏稿瀷鐢ㄦ硶

涓嬮潰鏄竴涓畝鍗曠殑 rpmsg 椹卞姩锛屽畠鍦?probe() 鏃跺彂閫佷竴鏉?"hello!" 娑堟伅锛屽苟鍦ㄦ瘡娆℃敹鍒板叆绔欐秷鎭椂
灏嗗叾鍐呭杞偍鍒版帶鍒跺彴銆?
```
  #include <linux/dev_printk.h>
  #include <linux/mod_devicetable.h>
  #include <linux/module.h>
  #include <linux/printk.h>
  #include <linux/rpmsg.h>
  #include <linux/types.h>

  static void rpmsg_sample_cb(struct rpmsg_channel *rpdev, void *data, int len,
						void *priv, u32 src)
  {
	print_hex_dump(KERN_INFO, "incoming message:", DUMP_PREFIX_NONE,
						16, 1, data, len, true);
  }

  static int rpmsg_sample_probe(struct rpmsg_channel *rpdev)
  {
	int err;

	dev_info(&rpdev->dev, "chnl: 0x%x -> 0x%x\n", rpdev->src, rpdev->dst);

	/* send a message on our channel */
	err = rpmsg_send(rpdev->ept, "hello!", 6);
	if (err) {
		dev_err(&rpdev->dev, "rpmsg_send failed: %d\n", err);
		return err;
	}

	return 0;
  }

  static void rpmsg_sample_remove(struct rpmsg_channel *rpdev)
  {
	dev_info(&rpdev->dev, "rpmsg sample client driver is removed\n");
  }

  static struct rpmsg_device_id rpmsg_driver_sample_id_table[] = {
	{ .name	= "rpmsg-client-sample" },
	{ },
  };
  MODULE_DEVICE_TABLE(rpmsg, rpmsg_driver_sample_id_table);

  static struct rpmsg_driver rpmsg_sample_client = {
	.drv.name	= KBUILD_MODNAME,
	.id_table	= rpmsg_driver_sample_id_table,
	.probe		= rpmsg_sample_probe,
	.callback	= rpmsg_sample_cb,
	.remove		= rpmsg_sample_remove,
  };
  module_rpmsg_driver(rpmsg_sample_client);

```

   涓€涓彲浠ユ瀯寤哄苟鍔犺浇鐨勭被浼肩ず渚嬪彲鍦?samples/rpmsg/ 涓壘鍒般€?
## rpmsg 閫氶亾鐨勫垎閰?
鐩墠鎴戜滑鍙敮鎸佸姩鎬佸垎閰?rpmsg 閫氶亾銆?
杩欏彧鏈夊湪鍏峰 VIRTIO_RPMSG_F_NS virtio 璁惧鐗规€ч泦鐨勮繙绋嬪鐞嗗櫒涓婃墠鍙兘銆傝鐗规€т綅鎰忓懗鐫€杩滅▼
澶勭悊鍣ㄦ敮鎸佸姩鎬佸悕绉版湇鍔″鍛婃秷鎭€?
褰撳惎鐢ㄨ鐗规€ф椂锛宺pmsg 璁惧锛堝嵆閫氶亾锛夌殑鍒涘缓鏄畬鍏ㄥ姩鎬佺殑锛氳繙绋嬪鐞嗗櫒閫氳繃鍙戦€佷竴鏉″悕绉版湇鍔℃秷鎭?锛堝叾涓寘鍚繙绋嬫湇鍔＄殑鍚嶇О鍜?rpmsg 鍦板潃锛屽弬瑙?struct rpmsg_ns_msg锛夋潵瀹ｅ憡涓€涓繙绋?rpmsg 鏈嶅姟鐨?瀛樺湪銆?
杩欐潯娑堟伅闅忓悗鐢?rpmsg 鎬荤嚎澶勭悊锛屽苟鐢辨鍔ㄦ€佸垱寤哄苟娉ㄥ唽涓€涓?rpmsg 閫氶亾锛堜唬琛ㄨ杩滅▼鏈嶅姟锛夈€傚綋
锛堝鏋滐級涓€涓浉鍏崇殑 rpmsg 椹卞姩琚敞鍐屾椂锛屽畠浼氱珛鍗宠鎬荤嚎鎺㈡祴锛岀劧鍚庡氨鍙互寮€濮嬪悜杩滅▼鏈嶅姟鍙戦€佹秷鎭€?
鎴戜滑涔熻鍒掗€氳繃 virtio 閰嶇疆绌洪棿娣诲姞 rpmsg 閫氶亾鐨勯潤鎬佸垱寤猴紝浣嗚繖灏氭湭瀹炵幇銆?