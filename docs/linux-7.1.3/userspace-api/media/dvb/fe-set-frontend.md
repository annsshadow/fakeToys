######## FE_SET_FRONTEND


## Name


FE_SET_FRONTEND

## Synopsis


`int ioctl(int fd, FE_SET_FRONTEND, struct dvb_frontend_parameters *p)`

## Arguments


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`p`
    鎸囧悜璋冭皭锛坱uning锛夋搷浣滄墍闇€鍙傛暟鐨勬寚閽堛€?
## Description


璇?ioctl 璋冪敤浣跨敤鎸囧畾鐨勫弬鏁板惎鍔ㄤ竴娆¤皟璋愭搷浣溿€傚鏋滃弬鏁版湁鏁堜笖鑳藉鍚姩璋冭皭锛屽垯璇ヨ皟鐢ㄧ殑缁撴灉灏嗘垚鍔熴€傜劧鑰岋紝璋冭皭鎿嶄綔鏈韩鐨勭粨鏋滃皢浣滀负浜嬩欢寮傛鍒拌揪锛堝弬瑙?FE_GET_EVENT 涓?FrontendEvent 鐨勬枃妗ｏ級銆傚鏋滃湪鍓嶄竴涓搷浣滃畬鎴愪箣鍓嶅彂璧蜂簡鏂扮殑 FE_SET_FRONTEND 鎿嶄綔锛屽垯鍓嶄竴涓搷浣滃皢琚腑姝紝浠ヤ究鎵ц鏂扮殑鎿嶄綔銆傝鍛戒护闇€瑕佸璁惧鍏锋湁璇诲啓璁块棶鏉冮檺銆?
## Return Value


鎴愬姛鏃惰繑鍥?0銆?
鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

    :header-rows:  0
    :stub-columns: 0
    :widths: 1 16

    - .. row 1

       - `EINVAL`

       - 杈惧埌鎵€鏀寔鐨勬渶澶х鍙风巼锛坰ymbol rate锛夈€?
Generic error codes are described at the
Generic Error Codes <gen-errors> chapter.
