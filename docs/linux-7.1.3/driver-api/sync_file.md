## Sync File API 鎸囧崡


:Author: Gustavo Padovan <gustavo at padovan dot org>

鏈枃妗ｄ綔涓鸿澶囬┍鍔ㄧ紪鍐欒€呯殑鎸囧崡锛岃鏄?sync_file API 鏄粈涔堬紝浠ュ強椹卞姩濡備綍鏀寔瀹冦€係ync file 鏄悓姝ワ紙struct dma_fence锛夌殑杞戒綋锛岃繖浜涘悓姝ユ槸鍦ㄩ┍鍔ㄤ箣闂存垨璺ㄨ繘绋嬭竟鐣岃繘琛屽悓姝ユ墍蹇呴渶鐨勩€?

sync_file API 鏃ㄥ湪鐢ㄤ簬鍚戠敤鎴风┖闂村彂閫佸拰浠庣敤鎴风┖闂存帴鏀?fence 淇℃伅銆傚畠浣跨敤鎴风┖闂磋兘澶熻繘琛屾樉寮忓洿鏍忥紙fencing锛夛紝鍗崇敓浜ц€呴┍鍔紙濡?GPU 鎴?V4L 椹卞姩锛変笉鏄皢 fence 闄勫姞鍒扮紦鍐插尯锛岃€屾槸閫氳繃 sync_file 灏嗕笌璇ョ紦鍐插尯鐩稿叧鐨?fence 鍙戦€佺粰鐢ㄦ埛绌洪棿銆?

鐒跺悗璇?sync_file 鍙互琚彂閫佺粰娑堣垂鑰咃紙渚嬪 DRM 椹卞姩锛夛紝鍦?fence 鍙戝嚭淇″彿涔嬪墠锛岃娑堣垂鑰呬笉浼氬缂撳啿鍖哄仛浠讳綍浜嬫儏鈥斺€斿嵆鍙戝嚭 fence 鐨勯┍鍔ㄤ笉鍐嶄娇鐢?澶勭悊璇ョ紦鍐插尯锛屽洜姝ゅ畠鍙戝嚭淇″彿琛ㄧず璇ョ紦鍐插尯宸插彲浣跨敤銆傚浜庢秷璐硅€?-> 鐢熶骇鑰呯殑寰幆閮ㄥ垎鍙嶄箣浜︾劧銆?

Sync file 浣跨敤鎴风┖闂磋兘澶熸劅鐭ラ┍鍔ㄤ箣闂寸紦鍐插尯鍏变韩鐨勫悓姝ャ€?

Sync file 鏈€鍒濇坊鍔犱簬 Android 鍐呮牳锛屼絾褰撳墠 Linux 妗岄潰涔熻兘浠庝腑鑾风泭鑹銆?

### in-fence 涓?out-fence


Sync file 鏃㈠彲浠ュ彂寰€鐢ㄦ埛绌洪棿锛屼篃鍙互鏉ヨ嚜鐢ㄦ埛绌洪棿銆傚綋 sync_file 浠庨┍鍔ㄥ彂閫佸埌鐢ㄦ埛绌洪棿鏃讹紝鎴戜滑绉板叾鍖呭惈鐨?fence 涓衡€渙ut-fence鈥濄€傚畠浠笌涓€涓┍鍔ㄦ鍦ㄥ鐞嗘垨鍗冲皢澶勭悊鐨勭紦鍐插尯鐩稿叧锛屽洜姝ら┍鍔ㄥ垱寤轰竴涓?out-fence锛屼互渚胯兘澶熷湪瀹冮€氳繃 dma_fence_signal() 瀹屾垚浣跨敤璇ョ紦鍐插尯锛堟垨澶勭悊瀹岋級鏃堕€氱煡銆侽ut-fence 鏄┍鍔ㄥ垱寤虹殑 fence銆?

鍙︿竴鏂归潰锛屽鏋滈┍鍔ㄩ€氳繃 sync_file 浠庣敤鎴风┖闂存帴鏀跺埌 fence锛屾垜浠О杩欎簺 fence 涓衡€渋n-fence鈥濄€傛帴鏀跺埌 in-fence 鎰忓懗鐫€鎴戜滑闇€瑕佸湪浣跨敤璇?in-fence 鐩稿叧鐨勪换浣曠紦鍐插尯涔嬪墠锛岀瓑寰呰 fence 鍙戝嚭淇″彿銆?

### 鍒涘缓 Sync File


褰撻┍鍔ㄩ渶瑕佸悜鐢ㄦ埛绌洪棿鍙戦€?out-fence 鏃讹紝瀹冨垱寤轰竴涓?sync_file銆?

```

	struct sync_file *sync_file_create(struct dma_fence *fence);

```

璋冪敤鑰呬紶鍏?out-fence锛屽彇鍥?sync_file銆傝繖鍙槸绗竴姝ワ紝鎺ヤ笅鏉ュ畠闇€瑕佸湪 sync_file->file 涓婂畨瑁呬竴涓?fd銆傚洜姝ゅ畠鑾峰彇涓€涓?
```

	fd = get_unused_fd_flags(O_CLOEXEC);

```

```
	fd_install(fd, sync_file->file);

```

璇?sync_file fd 鐜板湪鍙互琚彂閫佺粰鐢ㄦ埛绌洪棿銆?

濡傛灉鍒涘缓杩囩▼澶辫触锛屾垨鑰呯敱浜庝换浣曞叾浠栧師鍥犻渶瑕侀噴鏀?sync_file锛屽簲浣跨敤 fput(sync_file->file)銆?

### 浠庣敤鎴风┖闂存帴鏀?Sync File


褰撶敤鎴风┖闂撮渶瑕佸悜椹卞姩鍙戦€?in-fence 鏃讹紝瀹冨皢 Sync File 鐨勬枃浠舵弿杩扮浼犻€掔粰鍐呮牳銆傚唴鏍搁殢鍚庡彲浠ヤ粠涓绱?fence銆?

```

	struct dma_fence *sync_file_get_fence(int fd);


```

杩斿洖鐨勫紩鐢ㄧ敱璋冪敤鑰呮嫢鏈夛紝涔嬪悗蹇呴』浣跨敤 dma_fence_put() 閲婃斁銆傚湪鍑洪敊鐨勬儏鍐典笅锛岃繑鍥炵殑鏄?NULL 鑰岄潪寮曠敤銆?

鍙傝€冿細

1. include/linux/sync_file.h 涓殑 struct sync_file
2. 涓婅堪鎵€鏈夋帴鍙ｅ潎瀹氫箟鍦?include/linux/sync_file.h 涓?
