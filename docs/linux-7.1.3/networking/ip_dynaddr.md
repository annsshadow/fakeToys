
## IP鍔ㄦ€佸湴鍧€ hack-port v0.03


杩欎釜涓滆タ鍏佽閫氳繃浠ヤ笅鏂瑰紡寤虹珛鎷ㄦ墦鐨?ONESHOT 杩炴帴
鍔ㄦ€佹洿鏀规暟鎹寘婧愬湴鍧€锛堜互鍙婂鎺ュ瓧鐨勬湰鍦拌繘绋嬶級銆?
瀹冩槸閽堝 TCP 鎷ㄥ彿鐩掕繛鎺?(1) 鍜?IP_MASQuerading(2) 瀹炵幇鐨勩€?

濡傛灉鍚敤\ [#]_骞朵笖杞彂鎺ュ彛宸叉洿鏀癸細

1) 濂楁帴瀛楋紙鍜屾暟鎹寘锛夋簮鍦板潃鍦ㄩ噸浼犳椂琚噸鍐?
澶勪簬 SYN_SENT 鐘舵€佹椂锛堟嫧鍙锋杩涚▼锛夈€?
2) 鍑虹晫 MASQueraded 婧愬湴鍧€鏇存敼 ON OUTPUT锛堝綋
鍐呴儴涓绘満杩涜閲嶄紶锛夌洿鍒版潵鑷閮ㄧ殑鏁版嵁鍖呰
鐢遍毀閬撴帴鏀躲€?

杩欏浜庤嚜鍔ㄦ嫧鍙烽摼鎺?(diaald) 鐗瑰埆鏈夊府鍔╋紝鍏朵腑
`actual` 浼犲嚭鍦板潃鐩墠鏈煡
涓婂崌銆傚洜姝わ紝**鐩稿悓**锛堟湰鍦板拰浼锛夎繛鎺ヨ姹?
寤虹珛閾炬帴灏卞彲浠ヤ簡銆?


```

     # echo 1 > /proc/sys/net/ipv4/ip_dynaddr

  To enable verbose mode::

    # echo 2 > /proc/sys/net/ipv4/ip_dynaddr

  To disable (default)::

     # echo 0 > /proc/sys/net/ipv4/ip_dynaddr

```
浜彈锛?

鑳″畨涔?<jjciarla@raiz.uncu.edu.ar>
