## CA_GET_DESCR_INFO


### Name


CA_GET_DESCR_INFO

### Synopsis



`int ioctl(fd, CA_GET_DESCR_INFO, struct ca_descr_info *desc)`

### Arguments


`fd`
  鐢卞厛鍓?`open()` 璋冪敤杩斿洖鐨勬枃浠舵弿杩扮銆?

`desc`
  鎸囧悜 struct `ca_descr_info` 鐨勬寚閽堛€?

### Description


杩斿洖鏈夊叧鎵€鏈夎В鎵板櫒鎻掓Ы鐨勪俊鎭€?

### Return Value


鎴愬姛鏃惰繑鍥?0锛屽苟濉厖 `ca_descr_info`銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
