## CA_SEND_MSG


### Name


CA_SEND_MSG

### Synopsis



`int ioctl(fd, CA_SEND_MSG, struct ca_msg *msg)`

### Arguments


`fd`
  鐢卞厛鍓?`open()` 璋冪敤杩斿洖鐨勬枃浠舵弿杩扮銆?

`msg`
  鎸囧悜 struct `ca_msg` 鐨勬寚閽堛€?

### Description


閫氳繃 CI CA 妯″潡鍙戦€佷竴鏉℃秷鎭€?


   璇锋敞鎰忥紝鍦ㄥぇ澶氭暟椹卞姩涓婏紝杩欐槸閫氳繃鍐欏叆 /dev/adapter?/ca? 璁惧鑺傜偣瀹屾垚鐨勩€?

### Return Value


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
