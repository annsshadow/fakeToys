######## ioctl NET_REMOVE_IF


## Name


NET_REMOVE_IF - 绉婚櫎涓€涓綉缁滄帴鍙ｃ€?

## Synopsis



`int ioctl(int fd, NET_REMOVE_IF, int ifnum)`

## Arguments


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`net_if`
    瑕佺Щ闄ょ殑鎺ュ彛缂栧彿

## Description


NET_REMOVE_IF ioctl 鍒犻櫎涔嬪墠閫氳繃 NET_ADD_IF <net> 鍒涘缓鐨勬帴鍙ｃ€?

## Return Value


鎴愬姛鏃惰繑鍥?0锛屽苟濉厖 `ca_slot_info`銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
