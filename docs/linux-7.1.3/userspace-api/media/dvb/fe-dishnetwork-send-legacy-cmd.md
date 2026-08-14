######## FE_DISHNETWORK_SEND_LEGACY_CMD


## 鍚嶇О


FE_DISHNETWORK_SEND_LEGACY_CMD

## 鎽樿



`int ioctl(int fd, FE_DISHNETWORK_SEND_LEGACY_CMD, unsigned long cmd)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`cmd`
    閫氳繃 DISEqC 鍚戝ぉ绾匡紙dish锛夊彂閫佹寚瀹氱殑鍘熷鍛戒护銆?

## 璇存槑


   杩欐槸涓€涓潪甯稿喎闂紙obscure锛夌殑閬楃暀鍛戒护锛屼粎鐢ㄤ簬 stv0299 椹卞姩銆備笉搴斿湪鏂伴┍鍔ㄤ腑浣跨敤銆?

瀹冧负鍓嶇锛坒rontend锛夋彁渚涗簡涓€绉嶉潪鏍囧噯鏂规硶锛岀敤浜庝负 Dish Network 閬楃暀鍒囨崲閫夋嫨 Diseqc 鐢靛帇銆?

鐢变簬瀵硅 ioctl 鐨勬敮鎸佹槸鍦?2004 骞村姞鍏ョ殑锛岃繖鎰忓懗鐫€姝ょ被澶╃嚎鍦?2004 骞存椂灏卞凡缁忔槸閬楃暀璁惧浜嗐€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
