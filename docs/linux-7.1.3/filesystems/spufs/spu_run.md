
## spu_run

## 鍚嶇О

       spu_run - 鎵ц涓€涓?spu 涓婁笅鏂?
## 姒傝

```

	    #include <sys/spu.h>

	    int spu_run(int fd, unsigned int *npc, unsigned int *event);

```
## 鎻忚堪

       spu_run 绯荤粺璋冪敤鐢ㄤ簬瀹炵幇浜?Cell Broadband Engine Architecture 鐨?PowerPC
       鏈哄櫒锛屼互渚胯闂崗鍚屽鐞嗗櫒鍗曞厓锛圫ynergistic Processor Units锛孲PUs锛夈€傚畠浣跨敤
       浠?spu_create(2) 杩斿洖鐨?fd 鏉ュ鍧€涓€涓壒瀹氱殑 SPU 涓婁笅鏂囥€傚綋璇ヤ笂涓嬫枃琚皟搴﹀埌
       涓€涓墿鐞?SPU 涓婃椂锛屽畠浼氫粠浼犲叆鐨?npc 涓殑鎸囦护鎸囬拡澶勫紑濮嬫墽琛屻€?
       SPU 浠ｇ爜鐨勬墽琛屾槸鍚屾鐨勶紝鎰忓懗鐫€鍦?SPU 浠嶅湪杩愯鏃?spu_run 涓嶄細杩斿洖銆傚鏋滈渶瑕?       涓庝富绾?CPU 鎴栧叾瀹?SPU 涓婄殑鍏跺畠浠ｇ爜骞惰鎵ц SPU 浠ｇ爜锛屼綘闇€瑕佸厛鍒涘缓涓€涓柊鐨?       鎵ц绾跨▼锛屼緥濡備娇鐢?pthread_create(3) 璋冪敤銆?
       褰?spu_run 杩斿洖鏃讹紝SPU 鎸囦护鎸囬拡鐨勫綋鍓嶅€间細琚啓鍥?npc锛屽洜姝や綘鍙互鍐嶆璋冪敤
       spu_run 鑰屾棤闇€鏇存柊鎸囬拡銆?
       event 鍙互鏄竴涓?NULL 鎸囬拡锛屼篃鍙互鎸囧悜涓€涓墿灞曠姸鎬佺爜锛岃鐘舵€佺爜鍦?spu_run
       杩斿洖鏃惰濉厖銆傚畠鍙互鏄互涓嬪父閲忎箣涓€锛?
       SPE_EVENT_DMA_ALIGNMENT
              涓€涓?DMA 瀵归綈閿欒

       SPE_EVENT_SPE_DATA_SEGMENT
              涓€涓?DMA 鍒嗘閿欒

       SPE_EVENT_SPE_DATA_STORAGE
              涓€涓?DMA 瀛樺偍閿欒

       濡傛灉 event 鍙傛暟浼犲叆 NULL锛岃繖浜涢敊璇皢瀵艰嚧鍚戣皟鐢ㄨ繘绋嬪彂閫佷竴涓俊鍙枫€?
## 杩斿洖鍊?
       spu_run 杩斿洖 spu_status 瀵勫瓨鍣ㄧ殑鍊硷紝鎴栬繑鍥?-1 琛ㄧず鍑洪敊锛屽苟灏?errno 璁剧疆涓?       涓嬮潰鍒楀嚭鐨勯敊璇爜涔嬩竴銆俿pu_status 瀵勫瓨鍣ㄧ殑鍊煎寘鍚竴涓姸鎬佺爜鐨勪綅鎺╃爜锛屼互鍙?       锛堝彲閫夊湴锛変粠 SPU 涓婄殑 stop-and-signal 鎸囦护杩斿洖鐨?14 浣嶄唬鐮併€傜姸鎬佺爜鐨勪綅鎺╃爜
       濡備笅锛?
       0x02
	      SPU 琚?stop-and-signal 鍋滄銆?
       0x04
	      SPU 琚?halt 鍋滄銆?
       0x08
	      SPU 姝ｅ湪绛夊緟涓€涓€氶亾銆?
       0x10
	      SPU 澶勪簬鍗曟锛坰ingle-step锛夋ā寮忋€?
       0x20
	      SPU 璇曞浘鎵ц涓€鏉℃棤鏁堟寚浠ゃ€?
       0x40
	      SPU 璇曞浘璁块棶涓€涓棤鏁堥€氶亾銆?
       0x3fff0000
              涓庢鍊肩浉鎺╃爜鐨勪綅鍖呭惈浠?stop-and-signal 杩斿洖鐨勪唬鐮併€?
       鎬绘槸浼氳缃綆 8 浣嶄腑鐨勪竴涓垨澶氫釜锛屾垨鑰呬粠 spu_run 杩斿洖涓€涓敊璇爜銆?
## 閿欒

       EAGAIN 鎴?EWOULDBLOCK
	      fd 澶勪簬闈為樆濉炴ā寮忥紝涓?spu_run 浼氶樆濉炪€?
       EBADF  fd 涓嶆槸鏈夋晥鐨勬枃浠舵弿杩扮銆?
       EFAULT npc 涓嶆槸鏈夋晥鎸囬拡锛屾垨鑰?status 鏃笉鏄?NULL 涔熶笉鏄湁鏁堟寚閽堛€?
       EINTR  鍦?spu_run 杩涜鏈熼棿鍙戠敓浜嗕俊鍙枫€傚繀瑕佹椂 npc 鍊煎凡琚洿鏂颁负鏂扮殑绋嬪簭璁℃暟鍣?	      鍊笺€?
       EINVAL fd 涓嶆槸浠?spu_create(2) 杩斿洖鐨勬枃浠舵弿杩扮銆?
       ENOMEM 娌℃湁瓒冲鐨勫唴瀛樻潵澶勭悊鐢?MFC 鐩存帴鍐呭瓨璁块棶寮曞彂鐨勯〉閿欒銆?
       ENOSYS 褰撳墠绯荤粺鏈彁渚涜鍔熻兘锛屽洜涓鸿涔堢‖浠朵笉鎻愪緵 SPU锛岃涔?spufs 妯″潡鏈姞杞姐€?
## 娉ㄦ剰

       spu_run 鏃ㄥ湪鐢卞疄鐜颁簡瀵?SPU 鏇存娊璞℃帴鍙ｇ殑搴撲娇鐢紝鑰屼笉鏄敱甯歌搴旂敤绋嬪簭浣跨敤銆?       鍏充簬鎺ㄨ崘鐨勫簱锛岃鍙傝 http://www.bsc.es/projects/deepcomputing/linuxoncell/銆?
## 閬靛惊鏍囧噯

       姝よ皟鐢ㄦ槸 Linux 鐗规湁鐨勶紝骞朵笖浠呯敱 ppc64 鏋舵瀯瀹炵幇銆備娇鐢ㄦ绯荤粺璋冪敤鐨勭▼搴忎笉鍙Щ妞嶃€?
## 缂洪櫡

       浠ｇ爜灏氭湭瀹屽叏瀹炵幇姝ゅ鍒楀嚭鐨勬墍鏈夊姛鑳姐€?
## 浣滆€?
       Arnd Bergmann <arndb@de.ibm.com>

## 鍙﹁鍙傞槄

       capabilities(7), close(2), spu_create(2), spufs(7)
