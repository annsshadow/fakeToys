## Devlink 鑷

`devlink-selftests` API 鍏佽鍦ㄨ澶囦笂鎵ц鑷銆?

## 娴嬭瘯鎺╃爜

`devlink-selftests` 鍛戒护搴旈厤鍚堜竴涓帺鐮佽繍琛岋紝浠ユ寚绀鸿鎵ц鐨勬祴璇曘€?

## 娴嬭瘯璇存槑

浠ヤ笅鏄┍鍔ㄥ彲鑳芥墽琛岀殑娴嬭瘯鍒楄〃銆?

   :widths: 5 90

   - - 鍚嶇О
     - 璇存槑
   - - `DEVLINK_SELFTEST_FLASH`
     - 璁惧鍙兘鍦ㄦ澘杞介潪鏄撳け鎬у瓨鍌ㄥ櫒锛堜緥濡?flash锛変笂瀛樻斁鍥轰欢銆傝娴嬭瘯鐢ㄤ簬鍦ㄨ澶囦笂鎵ц flash 鑷銆?
       娴嬭瘯鐨勫叿浣撳疄鐜扮敱椹卞姩/鍥轰欢璐熻矗銆?

### 浣跨敤绀轰緥


    # 鏌ヨ devlink 璁惧鏀寔鐨勮嚜妫€
    $ devlink dev selftests show DEV
    # 鏌ヨ鎵€鏈?devlink 璁惧鏀寔鐨勮嚜妫€
    $ devlink dev selftests show
    # 鍦ㄨ澶囦笂鎵ц鑷
    $ devlink dev selftests run DEV id flash
