## CA_GET_CAP


### Name


CA_GET_CAP

### Synopsis



`int ioctl(fd, CA_GET_CAP, struct ca_caps *caps)`

### Arguments


`fd`
  鐢卞厛鍓?`open()` 璋冪敤杩斿洖鐨勬枃浠舵弿杩扮銆?

`caps`
  鎸囧悜 struct `ca_caps` 鐨勬寚閽堛€?

### Description


鍚戝唴鏍告煡璇㈡湁鍏冲彲鐢?CA 鍜岃В鎵板櫒鎻掓Ы鍙婂叾绫诲瀷鐨勪俊鎭€?

### Return Value


鎴愬姛鏃惰繑鍥?0 骞跺～鍏?`ca_caps`銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
