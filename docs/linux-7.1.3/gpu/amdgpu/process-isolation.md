
##  AMDGPU 杩涚▼闅旂


AMDGPU 椹卞姩鍖呭惈涓€涓壒鎬э紝鍙湪鍥惧舰寮曟搸涓婂惎鐢ㄨ嚜鍔ㄨ繘绋嬮殧绂汇€傝鐗规€у鍥惧舰寮曟搸鐨勮闂繘琛屼覆琛屽寲锛屽苟娣诲姞涓€涓?cleaner shader锛屽湪鍚勪綔涓氫箣闂存竻闄ゅ眬閮ㄦ暟鎹瓨鍌紙LDS锛変笌閫氱敤瀵勫瓨鍣紙GPR锛夈€傚綋鍚敤姝ょ壒鎬ф椂锛屾墍鏈変娇鐢?GPU 鐨勮繘绋嬶紙鍖呮嫭鍥惧舰涓庤绠楀伐浣滆礋杞斤級閮戒細琚覆琛屽寲銆傚湪鏀寔鍙垎鍖哄浘褰㈠紩鎿庣殑 GPU 涓婏紝姝ょ壒鎬у彲浠ユ寜鍒嗗尯鍚敤銆?

姝ゅ锛岃繕鎻愪緵浜嗕竴涓帴鍙ｏ紝鍙湪 GPU 浣跨敤瀹屾瘯鍚庢墜鍔ㄨ繍琛?cleaner shader銆傝繖鍦ㄦ煇浜涚敤渚嬩腑鍙兘鏇村彲鍙栵紝渚嬪鍗曠敤鎴风郴缁熶腑锛岀櫥褰曠鐞嗗櫒鍦ㄧ敤鎴风櫥鍑烘椂瑙﹀彂 cleaner shader銆?

## 杩涚▼闅旂


`run_cleaner_shader` 涓?`enforce_isolation` sysfs 鎺ュ彛鍒嗗埆鍏佽鐢ㄦ埛鎵嬪姩鎵ц cleaner shader 骞舵帶鍒惰繘绋嬮殧绂荤壒鎬с€?

### 鍒嗗尯澶勭悊


sysfs 涓殑 `enforce_isolation` 鏂囦欢鍙敤浜庡惎鐢ㄨ繘绋嬮殧绂讳互鍙婂湪杩涚▼闂磋嚜鍔ㄦ竻鐞?shader銆傚湪鏀寔鍥惧舰寮曟搸鍒嗗尯鐨?GPU 涓婏紝鍙互鎸夊垎鍖哄惎鐢ㄣ€傚垎鍖哄強鍏跺綋鍓嶈缃紙0 绂佺敤锛? 鍚敤锛夊彲浠?sysfs 璇诲彇銆傚湪涓嶆敮鎸佸浘褰㈠紩鎿庡垎鍖虹殑 GPU 涓婏紝灏嗗彧瀛樺湪涓€涓垎鍖恒€傚悜鍒嗗尯浣嶇疆鍐欏叆 1 鍚敤寮哄埗闅旂锛屽啓鍏?0 绂佺敤瀹冦€?

鍦ㄥ鍒嗗尯 GPU 涓婂惎鐢ㄥ己鍒堕殧绂荤殑绀轰緥锛?


    $ echo 1 0 1 0 > /sys/class/drm/card0/device/enforce_isolation
    $ cat /sys/class/drm/card0/device/enforce_isolation
    1 0 1 0

杈撳嚭琛ㄦ槑寮哄埗闅旂鍦ㄧ闆朵釜涓庣浜屼釜鍒嗗尯涓婂惎鐢紝鍦ㄧ涓€涓笌绗笁涓垎鍖轰笂绂佺敤銆?

瀵逛簬鍗曞垎鍖烘垨涓嶆敮鎸佸垎鍖虹殑璁惧锛屽皢鍙湁涓€涓厓绱狅細


    $ echo 1 > /sys/class/drm/card0/device/enforce_isolation
    $ cat /sys/class/drm/card0/device/enforce_isolation
    1

## Cleaner Shader 鎵ц


椹卞姩鍙互瑙﹀彂涓€涓?cleaner shader 鏉ユ竻鐞嗗浘褰㈠紩鎿庝笂鐨?LDS 涓?GPR 鐘舵€併€傚綋鍚敤杩涚▼闅旂鏃讹紝杩欎細鍦ㄨ繘绋嬮棿鑷姩鍙戠敓銆傛澶栵紝杩樻湁涓€涓?sysfs 鏂囦欢鐢ㄤ簬鎵嬪姩瑙﹀彂 cleaner shader 鎵ц銆?

瑕佹墜鍔ㄨЕ鍙?cleaner shader 鐨勬墽琛岋紝鍚?`run_cleaner_shader` sysfs 鏂囦欢鍐欏叆 `0`锛?


    $ echo 0 > /sys/class/drm/card0/device/run_cleaner_shader

瀵逛簬澶氬垎鍖鸿澶囷紝浣犲彲浠ュ湪瑙﹀彂 cleaner shader 鏃舵寚瀹氬垎鍖虹储寮曪細


    $ echo 0 > /sys/class/drm/card0/device/run_cleaner_shader # 瀵逛簬鍒嗗尯 0
    $ echo 1 > /sys/class/drm/card0/device/run_cleaner_shader # 瀵逛簬鍒嗗尯 1
    $ echo 2 > /sys/class/drm/card0/device/run_cleaner_shader # 瀵逛簬鍒嗗尯 2
    # ... 渚濇绫绘帹锛屾瘡涓垎鍖?

姝ゅ懡浠ゅ惎鍔?cleaner shader锛屽畠灏嗗湪 GPU 涓婅皟搴︿换浣曟柊浠诲姟涔嬪墠杩愯骞跺畬鎴愩€?
