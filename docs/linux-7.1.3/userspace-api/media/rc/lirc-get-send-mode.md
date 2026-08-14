######## ioctls LIRC_GET_SEND_MODE and LIRC_SET_SEND_MODE


## 鍚嶇О


LIRC_GET_SEND_MODE/LIRC_SET_SEND_MODE - 鑾峰彇/璁剧疆褰撳墠鍙戦€佹ā寮忋€?
## 姒傝


`int ioctl(int fd, LIRC_GET_SEND_MODE, __u32 *mode)`


`int ioctl(int fd, LIRC_SET_SEND_MODE, __u32 *mode)`

## 鍙傛暟


`fd`
    open() 杩斿洖鐨勬枃浠舵弿杩扮銆?
`mode`
    鐢ㄤ簬鍙戦€佺殑 mode銆?
## 鎻忚堪


鑾峰彇/璁剧疆褰撳墠鍙戦€佹ā寮忋€?
鏍规嵁椹卞姩鐨勪笉鍚岋紝IR 鍙戦€佷粎鏀寔 LIRC_MODE_PULSE <lirc-mode-pulse> 涓?LIRC_MODE_SCANCODE <lirc-mode-scancode>銆備娇鐢?lirc_get_features 鍙煡鏄庨┍鍔ㄦ敮鎸佸摢浜涙ā寮忋€?
## 杩斿洖鍊?

    :header-rows:  0
    :stub-columns: 0

    - .. row 1

       - `ENODEV`

       - 璁惧涓嶅彲鐢ㄣ€?
    - .. row 2

       - `ENOTTY`

       - 璁惧涓嶆敮鎸佸彂閫併€?
    - .. row 3

       - `EINVAL`

       - 鏃犳晥鐨勬ā寮忔垨璇ヨ澶囩殑鏃犳晥妯″紡銆?