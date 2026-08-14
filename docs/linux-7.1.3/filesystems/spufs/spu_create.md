## spu_create


## 鍚嶇О

       spu_create - 鍒涘缓涓€涓柊鐨?spu 涓婁笅鏂?


## 姒傝


```

         #include <sys/types.h>
         #include <sys/spu.h>

         int spu_create(const char *pathname, int flags, mode_t mode);

```
## 鎻忚堪

       spu_create 绯荤粺璋冪敤鐢ㄤ簬瀹炵幇 Cell 瀹藉甫寮曟搸鏋舵瀯鐨?PowerPC 鏈哄櫒锛屼互璁块棶
       鍗忓悓澶勭悊鍣ㄥ崟鍏冿紙SPU锛夈€傚畠鍦?pathname 涓负 SPU 鍒涘缓涓€涓柊鐨勯€昏緫涓婁笅鏂囷紝
       骞惰繑鍥炰竴涓笌涔嬪叧鑱旂殑澶勭悊鍙ユ焺銆俻athname 蹇呴』鎸囧悜 SPU 鏂囦欢绯荤粺锛坰pufs锛?
       鎸傝浇鐐逛腑涓€涓笉瀛樺湪鐨勭洰褰曘€俿pu_create 鎴愬姛鏃讹紝浼氬湪 pathname 澶勫垱寤轰竴涓?
       鐩綍锛屽苟鍚戝叾涓～鍏呮枃浠躲€?

       杩斿洖鐨勬枃浠跺彞鏌勫彧鑳戒紶閫掔粰 spu_run(2) 鎴栧叧闂紝鍏朵粬鎿嶄綔鏈湪鍏朵笂瀹氫箟銆傚綋
       瀹冭鍏抽棴鏃讹紝spufs 涓墍鏈夊叧鑱旂殑鐩綍椤逛細琚Щ闄ゃ€傚綋鎸囧悜涓婁笅鏂囩洰褰曞唴閮ㄦ垨
       鎸囧悜姝ゆ枃浠舵弿杩扮鐨勬渶鍚庝竴涓枃浠跺彞鏌勮鍏抽棴鏃讹紝璇ラ€昏緫 SPU 涓婁笅鏂囪閿€姣併€?

       鍙傛暟 flags 鍙互涓?0锛屼篃鍙互鏄笅鍒楀父閲忔寜浣嶆垨鐨勭粍鍚堬細

       SPU_RAWIO
              鍏佽灏?SPU 鐨勯儴鍒嗙‖浠跺瘎瀛樺櫒鏄犲皠鍒扮敤鎴风┖闂淬€傛鏍囧織闇€瑕?
              CAP_SYS_RAWIO 鑳藉姏锛屽弬瑙?capabilities(7)銆?

       mode 鍙傛暟鎸囧畾鍦?spufs 涓垱寤烘柊鐩綍鏃舵墍浣跨敤鐨勬潈闄愩€俶ode 浼氫笌鐢ㄦ埛鐨?
       umask(2) 鍊艰繘琛屼慨楗帮紝鐒跺悗鍚屾椂鐢ㄤ簬璇ョ洰褰曞強鍏跺寘鍚殑鏂囦欢銆傛枃浠舵潈闄愪細
       灞忚斀 mode 涓殑鏇村浣嶏紝鍥犱负瀹冧滑閫氬父鍙敮鎸佽鎴栧啓璁块棶銆傛湁鍏冲彲鑳界殑 mode
       鍊肩殑瀹屾暣鍒楄〃锛岃鍙傝 stat(2)銆?


## 杩斿洖鍊?

       spu_create 杩斿洖涓€涓柊鐨勬枃浠舵弿杩扮銆傚畠鍙兘杩斿洖 -1 浠ヨ〃绀洪敊璇潯浠讹紝骞?
       灏?errno 璁剧疆涓轰笅鍒楅敊璇爜涔嬩竴銆?


## 閿欒

       EACCES
              褰撳墠鐢ㄦ埛瀵?spufs 鎸傝浇鐐规病鏈夊啓璁块棶鏉冮檺銆?

       EEXIST 缁欏畾璺緞鍚嶅宸插瓨鍦?SPU 涓婁笅鏂囥€?

       EFAULT pathname 鍦ㄥ綋鍓嶅湴鍧€绌洪棿涓笉鏄竴涓湁鏁堢殑瀛楃涓叉寚閽堛€?

       EINVAL pathname 涓嶆槸 spufs 鎸傝浇鐐逛腑鐨勭洰褰曘€?

       ELOOP  瑙ｆ瀽 pathname 鏃跺彂鐜颁簡杩囧绗﹀彿閾炬帴銆?

       EMFILE 杩涚▼宸茶揪鍒板叾鏈€澶ф墦寮€鏂囦欢鏁伴檺鍒躲€?

       ENAMETOOLONG
              pathname 杩囬暱銆?

       ENFILE 绯荤粺宸茶揪鍒板叏灞€鎵撳紑鏂囦欢鏁伴檺鍒躲€?

       ENOENT pathname 鐨勬煇閮ㄥ垎鏃犳硶瑙ｆ瀽銆?

       ENOMEM 鍐呮牳鏃犳硶鍒嗛厤鎵€闇€鐨勫叏閮ㄨ祫婧愩€?

       ENOSPC 娌℃湁瓒冲鐨?SPU 璧勬簮鏉ュ垱寤烘柊涓婁笅鏂囷紝鎴栧凡杈惧埌鐢ㄦ埛瀵?SPU 涓婁笅鏂?
              鏁伴噺鐨勭壒瀹氶檺鍒躲€?

       ENOSYS 褰撳墠绯荤粺鏈彁渚涜鍔熻兘锛屽洜涓虹‖浠舵湭鎻愪緵 SPU锛屾垨 spufs 妯″潡鏈姞杞姐€?

       ENOTDIR
              pathname 鐨勬煇閮ㄥ垎涓嶆槸鐩綍銆?


## 澶囨敞

       spu_create 鏃ㄥ湪渚涘疄鐜?SPU 鏇撮珮灞傛娊璞℃帴鍙ｇ殑搴撲娇鐢紝鑰岄潪渚涙櫘閫氬簲鐢ㄧ▼搴?
       鐩存帴浣跨敤銆傛湁鍏虫帹鑽愮殑搴擄紝璇峰弬瑙?http://www.bsc.es/projects/deepcomputing/linuxoncell/銆?


## 鏂囦欢

       pathname 蹇呴』鎸囧悜 spufs 鎸傝浇鐐逛箣涓嬬殑浣嶇疆銆傛寜鐓ф儻渚嬶紝瀹冭鎸傝浇鍦?/spu銆?


## 閬靛惊鏍囧噯

       姝よ皟鐢ㄦ槸 Linux 鐗规湁鐨勶紝浠呯敱 ppc64 鏋舵瀯瀹炵幇銆備娇鐢ㄨ绯荤粺璋冪敤鐨勭▼搴忎笉鍙Щ妞嶃€?


## 缂洪櫡

       璇ヤ唬鐮佸皻鏈畬鍏ㄥ疄鐜版澶勫垪鍑虹殑鎵€鏈夌壒鎬с€?


## 浣滆€?

       Arnd Bergmann <arndb@de.ibm.com>

## 鍙傝

       capabilities(7), close(2), spu_run(2), spufs(7)
