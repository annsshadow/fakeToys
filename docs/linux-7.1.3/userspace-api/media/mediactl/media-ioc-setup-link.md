
######## ioctl MEDIA_IOC_SETUP_LINK


## Name


MEDIA_IOC_SETUP_LINK - 淇敼閾捐矾鐨勫睘鎬?

## Synopsis


`int ioctl(int fd, MEDIA_IOC_SETUP_LINK, struct media_link_desc *argp)`

## Arguments


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜缁撴瀯浣?`media_link_desc` 鐨勬寚閽堛€?

## Description


涓轰簡鏇存敼閾捐矾灞炴€э紝搴旂敤绋嬪簭闇€鍏堢敤閾捐矾鐨勬爣璇嗕俊鎭紙婧?pad 鍜岀洰鐨?pad锛変互鍙婅姹傜殑鏂伴摼璺爣蹇楀～鍏呬竴涓?
`media_link_desc` 缁撴瀯浣擄紝鐒跺悗
浣跨敤璇ョ粨鏋勭殑鎸囬拡璋冪敤 MEDIA_IOC_SETUP_LINK ioctl銆?

鍞竴鍙厤缃殑灞炴€ф槸鐢ㄤ簬鍚敤/绂佺敤閾捐矾鐨?`ENABLED` 閾捐矾鏍囧織銆傝
`IMMUTABLE` 閾捐矾鏍囧織鏍囪鐨勯摼璺棤娉曡鍚敤鎴栫鐢ㄣ€?

閾捐矾閰嶇疆涓嶄細瀵瑰叾浠栭摼璺骇鐢熷壇浣滅敤銆傚鏋滃湪鐩殑 pad 涓婂凡鍚敤鐨勯摼璺樆姝㈣閾捐矾琚惎鐢紝椹卞姩浼氳繑鍥?
`EBUSY` 閿欒鐮併€?

鍙湁琚?`DYNAMIC` 閾捐矾鏍囧織鏍囪鐨勯摼璺墠鑳藉湪濯掍綋鏁版嵁娴佷紶杈撹繃绋嬩腑琚惎鐢?绂佺敤銆傝瘯鍥惧惎鐢ㄦ垨绂佺敤姝ｅ湪浼犺緭鏁版嵁鐨勯潪鍔ㄦ€侀摼璺皢杩斿洖
`EBUSY` 閿欒鐮併€?

濡傛灉鎸囧畾鐨勯摼璺壘涓嶅埌锛岄┍鍔ㄤ細杩斿洖 `EINVAL` 閿欒鐮併€?

## Return Value


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

EINVAL
    缁撴瀯浣?`media_link_desc` 寮曠敤浜嗕竴涓?
    涓嶅瓨鍦ㄧ殑閾捐矾锛屾垨鑰呰閾捐矾鏄笉鍙彉鐨勪笖璇曞浘淇敼鍏堕厤缃€?
