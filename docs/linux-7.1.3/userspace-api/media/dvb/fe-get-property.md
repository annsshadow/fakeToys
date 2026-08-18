


######## ioctl FE_SET_PROPERTY, FE_GET_PROPERTY


## 鍚嶇О


FE_SET_PROPERTY - FE_GET_PROPERTY - FE_SET_PROPERTY 璁剧疆涓€涓垨澶氫釜鍓嶇灞炴€с€? FE_GET_PROPERTY 杩斿洖涓€涓垨澶氫釜鍓嶇灞炴€с€?

## 姒傝



`int ioctl(int fd, FE_GET_PROPERTY, struct dtv_properties *argp)`


`int ioctl(int fd, FE_SET_PROPERTY, struct dtv_properties *argp)`

## 鍙傛暟


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜缁撴瀯浣?`dtv_properties` 鐨勬寚閽堛€?

## 鎻忚堪


鎵€鏈夋暟瀛楃數瑙嗗墠绔澶囬兘鏀寔 `FE_SET_PROPERTY` 鍜?
`FE_GET_PROPERTY` ioctls銆傛敮鎸佺殑灞炴€у拰缁熻淇℃伅
鍙栧喅浜庝紶杈撶郴缁熷拰璁惧锛?

- `FE_SET_PROPERTY:`

   - 璇?ioctl 鐢ㄤ簬璁剧疆涓€涓垨澶氬墠绔睘鎬с€?

   - 杩欐槸璇锋眰鍓嶇璋冭皭鍒版煇涓鐜囧苟寮€濮嬭В鐮?
      鏁板瓧鐢佃淇″彿鐨勫熀鏈懡浠ゃ€?

   - 璇ヨ皟鐢ㄩ渶瑕佸璁惧鍏锋湁璇诲啓璁块棶鏉冮檺銆?


   杩斿洖鏃讹紝鍊间笉浼氭洿鏂颁互鍙嶆槧瀹為檯
   浣跨敤鐨勫弬鏁般€傚鏋滈渶瑕佸疄闄呭弬鏁帮紝鍒欓渶鏄惧紡
   璋冪敤 `FE_GET_PROPERTY`銆?

- `FE_GET_PROPERTY:`

   - 璇?ioctl 鐢ㄤ簬浠庡墠绔幏鍙栧睘鎬?
      鍜岀粺璁′俊鎭€?

   - 涓嶄細鏇存敼浠讳綍灞炴€э紝涔熶笉浼氶噸缃粺璁′俊鎭€?

   - 璇ヨ皟鐢ㄤ粎闇€瑕佸璁惧鍏锋湁鍙璁块棶鏉冮檺銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
