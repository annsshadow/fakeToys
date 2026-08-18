## 搴曞眰涓插彛 API


鏈枃妗ｆ棬鍦ㄧ畝瑕佹杩版柊涓插彛椹卞姩鐨勪竴浜涙柟闈€傚畠骞朵笉瀹屾暣锛屼綘鏈変换浣曢棶棰樺簲鐩存帴鑱旂郴 <rmk@arm.linux.org.uk>

鍙傝€冨疄鐜板寘鍚湪 amba-pl011.c 涓€?


### 搴曞眰涓插彛纭欢椹卞姩


搴曞眰涓插彛纭欢椹卞姩璐熻矗鍚戞牳蹇冧覆鍙ｉ┍鍔ㄦ彁渚涚鍙ｄ俊鎭紙鐢?uart_port 瀹氫箟锛変笌涓€缁勬帶鍒舵柟娉曪紙鐢?uart_ops 瀹氫箟锛夈€傚簳灞傞┍鍔ㄨ繕璐熻矗澶勭悊璇ョ鍙ｇ殑涓柇锛屽苟鎻愪緵浠讳綍鎺у埗鍙版敮鎸併€?


### 鎺у埗鍙版敮鎸?


涓插彛鏍稿績鎻愪緵浜嗕竴浜涜緟鍔╁嚱鏁般€傝繖鍖呮嫭瑙ｆ瀽鍛戒护琛屽弬鏁帮紙uart_parse_options()锛夈€?

杩樻湁涓€涓緟鍔╁嚱鏁帮紙uart_console_write()锛夋墽琛岄€愬瓧绗﹀啓鍏ワ紝灏嗘崲琛岀杞崲涓?CRLF 搴忓垪銆傚缓璁┍鍔ㄧ紪鍐欒€呬娇鐢ㄦ鍑芥暟锛岃€屼笉鏄疄鐜拌嚜宸辩殑鐗堟湰銆?


### 閿佸畾


搴曞眰纭欢椹卞姩鏈夎矗浠讳娇鐢?port->lock 鎵ц蹇呰鐨勯攣瀹氥€傛湁涓€浜涗緥澶栵紙鍦ㄤ笅闈㈢殑 struct uart_ops 鍒楄〃涓湁鎻忚堪锛夈€?

鏈変袱鎶婇攣銆備竴鎶婃槸姣忎釜绔彛鐨勮嚜鏃嬮攣锛屽彟涓€鎶婃槸鏁翠綋鐨勪俊鍙烽噺銆?

浠庢牳蹇冮┍鍔ㄧ殑瑙嗚鐪嬶紝port->lock 閿佸畾浠ヤ笅鍐呭
```

	port->mctrl
	port->icount
	port->state->xmit.head (circ_buf->head)
	port->state->xmit.tail (circ_buf->tail)

```

搴曞眰椹卞姩鍙互鑷敱浣跨敤杩欐妸閿佹潵鎻愪緵浠讳綍棰濆鐨勯攣瀹氥€?

port_sem 淇″彿閲忕敤浜庨槻姝㈢鍙ｅ湪涓嶆伆褰撴椂鏈鸿娣诲姞/绉婚櫎鎴栭噸鏂伴厤缃€傝嚜 v2.6.27 璧凤紝杩欐妸淇″彿閲忓凡鎴愪负 tty_port 缁撴瀯浣撶殑 'mutex' 鎴愬憳锛岄€氬父绉颁负绔彛浜掓枼浣擄紙port mutex锛夈€?


### uart_ops


   :identifiers: uart_ops

### 鍏朵粬鍑芥暟


   :identifiers: uart_update_timeout uart_get_baud_rate uart_get_divisor
           uart_match_port uart_write_wakeup uart_register_driver
           uart_unregister_driver uart_suspend_port uart_resume_port
           uart_add_one_port uart_remove_one_port uart_console_write
           uart_parse_earlycon uart_parse_options uart_set_options
           uart_get_lsr_info uart_handle_dcd_change uart_handle_cts_change
           uart_try_toggle_sysrq

   :identifiers: uart_port_tx_limited uart_port_tx

### 鍏朵粬璇存槑


璁″垝鏈夋湞涓€鏃ヤ粠 uart_port 涓Щ闄?'unused' 鏉＄洰锛屽苟鍏佽搴曞眰椹卞姩鍚戞牳蹇冩敞鍐屽畠浠悇鑷殑 uart_port銆傝繖灏嗗厑璁搁┍鍔ㄦ妸 uart_port 鐢ㄤ綔涓€涓寚鍚戠粨鏋勪綋鐨勬寚閽堬紝璇ョ粨鏋勪綋鏃㈠寘鍚?uart_port 鏉＄洰锛屼篃鍖呭惈瀹冧滑鑷繁鐨勬墿灞曪紝
```

	struct my_port {
		struct uart_port	port;
		int			my_stuff;
	};

```

### 閫氳繃 GPIO 鐨勮皟鍒惰В璋冨櫒鎺у埗绾?


鎻愪緵浜嗕竴浜涜緟鍔╁嚱鏁帮紝鐢ㄤ簬閫氳繃 GPIO 璁剧疆/鑾峰彇璋冨埗瑙ｈ皟鍣ㄦ帶鍒剁嚎銆?

   :identifiers: mctrl_gpio_init mctrl_gpio_to_gpiod
           mctrl_gpio_set mctrl_gpio_get mctrl_gpio_enable_ms
           mctrl_gpio_disable_ms_sync mctrl_gpio_disable_ms_no_sync
