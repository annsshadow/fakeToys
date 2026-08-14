


######## ioctls LIRC_GET_REC_MODE and LIRC_SET_REC_MODE


## 鍚嶇О


LIRC_GET_REC_MODE/LIRC_SET_REC_MODE - 鑾峰彇/璁剧疆褰撳墠鎺ユ敹妯″紡銆?
## 姒傝



`int ioctl(int fd, LIRC_GET_REC_MODE, __u32 *mode)`


`int ioctl(int fd, LIRC_SET_REC_MODE, __u32 *mode)`

## 鍙傛暟


`fd`
    鐢?open() 杩斿洖鐨勬枃浠舵弿杩扮銆?
`mode`
    鐢ㄤ簬鎺ユ敹鐨?mode銆?
## 鎻忚堪


鑾峰彇骞惰缃綋鍓嶆帴鏀舵ā寮忋€備粎鏀寔 LIRC_MODE_MODE2 <lirc-mode-mode2> 鍜?LIRC_MODE_SCANCODE <lirc-mode-scancode>銆備娇鐢?lirc_get_features 鏌ユ槑椹卞姩
绋嬪簭鏀寔鍝簺妯″紡銆?
## 杩斿洖鍊?


    :header-rows:  0
    :stub-columns: 0

    - .. row 1

       - `ENODEV`

       - 璁惧涓嶅彲鐢ㄣ€?
    - .. row 2

       - `ENOTTY`

       - 璁惧涓嶆敮鎸佹帴鏀躲€?
    - .. row 3

       - `EINVAL`

       - 鏃犳晥妯″紡锛屾垨瀵规璁惧鏃犳晥鐨勬ā寮忋€?