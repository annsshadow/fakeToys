######## ioctl FE_ENABLE_HIGH_LNB_VOLTAGE


## 鍚嶇О


FE_ENABLE_HIGH_LNB_VOLTAGE - 鍦ㄦ甯?LNBf 鐢靛帇涓庢洿楂樼殑 LNBf 鐢靛帇涔嬮棿閫夋嫨杈撳嚭鐩存祦鐢靛钩銆?

## 鎽樿



`int ioctl(int fd, FE_ENABLE_HIGH_LNB_VOLTAGE, unsigned int high)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`high`
    鏈夋晥鏍囧織锛?

    - 0 - 姝ｅ父鐨?13V 鍜?18V銆?

    - >0 - 鍚敤鐣ラ珮鐨勭數鍘嬩互鏇夸唬 13/18V锛岀敤浜庤ˉ鍋胯繃闀跨殑澶╃嚎鐢电紗銆?

## 璇存槑


鍦ㄦ甯?LNBf 鐢靛帇涓庢洿楂?LNBf 鐢靛帇涔嬮棿閫夋嫨杈撳嚭鐩存祦鐢靛钩锛?锛堟甯革級鎴栧ぇ浜?0 鐨勫€硷紙鏇撮珮鐢靛帇锛夈€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
