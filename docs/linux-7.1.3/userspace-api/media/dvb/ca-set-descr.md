## CA_SET_DESCR


### Name


CA_SET_DESCR

### Synopsis



`int ioctl(fd, CA_SET_DESCR, struct ca_descr *desc)`

### Arguments


`fd`
  鐢卞厛鍓?`open()` 璋冪敤杩斿洖鐨勬枃浠舵弿杩扮銆?

`msg`
  鎸囧悜 struct `ca_descr` 鐨勬寚閽堛€?

### Description


CA_SET_DESCR 鐢ㄤ簬鍚戣В鎵板櫒 CA 鎻掓Ы鎻愪緵瑙ｆ壈瀵嗛挜锛堢О涓烘帶鍒跺瓧锛夈€?

### Return Value


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
