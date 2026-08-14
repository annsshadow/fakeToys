## CA_GET_SLOT_INFO


### 鍚嶇О


CA_GET_SLOT_INFO

### 鎽樿



`int ioctl(fd, CA_GET_SLOT_INFO, struct ca_slot_info *info)`

### 鍙傛暟


`fd`
  鐢卞厛鍓?`open()` 璋冪敤杩斿洖鐨勬枃浠舵弿杩扮銆?

`info`
  鎸囧悜缁撴瀯浣?`ca_slot_info` 鐨勬寚閽堛€?

### 璇存槑


杩斿洖鐢?`ca_slot_info`.slot_num 鏍囪瘑鐨?CA 鎻掓Ы鐨勪俊鎭€?

### 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽苟濉厖 `ca_slot_info`銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 16

    - -  `ENODEV`
       - 璇ユ彃妲戒笉鍙敤銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
