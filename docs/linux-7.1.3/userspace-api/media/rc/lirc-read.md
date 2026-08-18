
######## LIRC read()


## 鍚嶇О


lirc-read - 浠?LIRC 璁惧璇诲彇

## 姒傝


    #include <unistd.h>


## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`buf`
   寰呭～鍏呯殑缂撳啿鍖?

`count`
   鏈€澶氳鍙栫殑瀛楄妭鏁?

## 鎻忚堪


`read()` 灏濊瘯浠庢枃浠舵弿杩扮 `fd` 鍚戣捣濮嬩簬 `buf` 鐨勭紦鍐插尯璇诲彇鏈€澶?`count` 涓瓧鑺傘€傚鏋?`count` 涓洪浂锛宍read()` 杩斿洖闆朵笖娌℃湁鍏朵粬缁撴灉銆傚鏋?`count` 澶т簬 `SSIZE_MAX`锛岀粨鏋滄槸鏈寚瀹氱殑銆?

鏁版嵁鐨勭‘鍒囨牸寮忓彇鍐充簬椹卞姩浣跨敤鐨?lirc_modes銆備娇鐢?lirc_get_features 鑾峰彇鏀寔鐨勬ā寮忥紝骞朵娇鐢?lirc_set_rec_mode 璁剧疆褰撳墠娲诲姩妯″紡銆?

LIRC_MODE_MODE2 <lirc-mode-mode2> 妯″紡鐢ㄤ簬鍘熷 IR锛屽叾涓鍙栬嚜瀛楃璁惧鐨勫寘鍖呭惈涓€涓弿杩?IR 淇″彿鐨勬棤绗﹀彿 int 鍊笺€?

鍙﹀锛孡IRC_MODE_SCANCODE <lirc-mode-scancode> 涔熷彲鑳藉彲鐢紝鍦ㄨ妯″紡涓嬶紝鎵弿鐮佺敱杞欢瑙ｇ爜鍣ㄦ垨纭欢瑙ｇ爜鍣ㄨВ鐮併€俙rc_proto` 鎴愬憳琚涓虹敤浜庝紶杈撶殑 IR 鍗忚 <Remote_controllers_Protocols>锛宍scancode` 琚涓鸿В鐮佸悗鐨勬壂鎻忕爜锛宍keycode` 琚涓洪敭鐮佹垨 `KEY_RESERVED`銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥炶鍙栫殑瀛楄妭鏁般€傚鏋滆繖涓暟瀛楀皬浜庤姹傜殑瀛楄妭鏁帮紝鎴栦竴甯ф墍闇€鐨勬暟鎹噺锛岃繖涓嶇畻閿欒銆傚嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?
