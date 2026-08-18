:orphan:

## MSM 宕╂簝杞偍鏍煎紡


鍦?GPU 鎸傝捣涔嬪悗锛孧SM 椹卞姩閫氳繃 /sys/kernel/dri/X/show 鎴栭€氳繃 devcoredump
锛?sys/class/devcoredump/dcdX/data锛夎緭鍑鸿皟璇曚俊鎭€傛湰鏂囨。鎻忚堪杈撳嚭鐨勬牸寮忋€?
姣忎釜鏉＄洰閮芥槸 key: value 鐨勫舰寮忋€傝妭鐨勬爣棰樻病鏈夊€硷紝骞朵笖璇ヨ妭鐨勬墍鏈夊唴瀹逛細浠庢爣棰樼缉杩?涓や釜绌烘牸銆傛瘡涓妭鍙兘鏈夊涓暟缁勬潯鐩紝鏁扮粍鏉＄洰鐨勫紑濮嬬敱涓€涓?(-) 鏍囪銆?
### 鏄犲皠锛圡appings锛?

kernel
	鐢熸垚璇ヨ浆鍌ㄧ殑鍐呮牳鐗堟湰锛圲TS_RELEASE锛夈€?
module
	鐢熸垚璇ュ穿婧冭浆鍌ㄧ殑妯″潡銆?
time
	宕╂簝鏃剁殑鍐呮牳鏃堕棿锛屾牸寮忎负 绉?寰銆?
comm
	浜х敓鏁呴殰鐨勪簩杩涘埗鏂囦欢鐨?comm 瀛楃涓层€?
cmdline
	浜х敓鏁呴殰鐨勪簩杩涘埗鏂囦欢鐨勫懡浠よ銆?
revision
	浜х敓宕╂簝鐨?GPU 鐨?ID锛屾牸寮忎负 core.major.minor.patchlevel锛屼互鍙ョ偣鍒嗛殧銆?
rbbm-status
	RBBM_STATUS 鐨勫綋鍓嶅€硷紝鏄剧ず宕╂簝鏃舵鍦ㄤ娇鐢ㄧ殑椤跺眰 GPU 缁勪欢銆?
ringbuffer
	鍖呭惈姣忎釜 ringbuffer 鍐呭鐨勮妭銆傛瘡涓?ringbuffer 鐢ㄤ竴涓?id 缂栧彿鏍囪瘑銆?
	id
		Ringbuffer ID锛堜粠 0 寮€濮嬬殑绱㈠紩锛夈€傝鑺備腑鐨勬瘡涓?ringbuffer 閮芥湁
		鑷繁鍞竴鐨?id銆?	iova
		ringbuffer 鐨?GPU 鍦板潃銆?
	last-fence
		鍦ㄨ ringbuffer 涓婂彂鍑虹殑鏈€鍚庝竴涓?fence

	retired-fence
		鍦ㄨ ringbuffer 涓婇€€褰圭殑鏈€鍚庝竴涓?fence銆?
	rptr
		璇?ringbuffer 鐨勫綋鍓嶈鎸囬拡锛坮ptr锛夈€?
	wptr
		璇?ringbuffer 鐨勫綋鍓嶅啓鎸囬拡锛坵ptr锛夈€?
	size
		鍦ㄧ‖浠朵腑缂栫▼鐨?ringbuffer 鐨勬渶澶уぇ灏忋€?
	data
		浠?ascii85 缂栫爜鐨?ring 鍐呭銆傚彧浼氭墦鍗?ring 涓浣跨敤鐨勯儴鍒嗐€?
bo
	鏉ヨ嚜鎸傝捣鎻愪氦鐨勭紦鍐插尯鍒楄〃锛堝鏋滃彲鐢級銆傛瘡涓紦鍐插尯瀵硅薄浼氭湁涓€涓敮涓€鐨?iova銆?
	iova
		缂撳啿鍖哄璞＄殑 GPU 鍦板潃銆?
	size
		缂撳啿鍖哄璞″垎閰嶇殑澶у皬銆?
	data
		浠?ascii85 缂栫爜鐨勭紦鍐插尯瀵硅薄鍐呭銆傚彧浼氳烦杩囩紦鍐插尯鏈熬鐨勫熬闅忛浂銆?
registers
	涓€缁勫瘎瀛樺櫒鍊笺€傛瘡涓潯鐩嫭鍗犱竴琛岋紝鐢ㄦ嫭鍙?{ } 鎷捣鏉ャ€?
	offset
		瀵勫瓨鍣ㄨ窛 GPU 鍐呭瓨鍖哄煙璧峰澶勭殑瀛楄妭鍋忕Щ銆?
	value
		瀵勫瓨鍣ㄧ殑鍗佸叚杩涘埗鍊笺€?
registers-hlsq
		锛堜粎 5xx锛夋潵鑷?HLSQ 瀛斿緞鐨勫瘎瀛樺櫒鍊笺€傛牸寮忎笌 register 鑺傜浉鍚屻€?