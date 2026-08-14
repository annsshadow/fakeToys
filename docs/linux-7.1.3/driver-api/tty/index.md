
## TTY


鐢典紶鎵撳瓧鏈猴紙TTY锛夊眰璐熻矗澶勭悊鎵€鏈夐偅浜涗覆琛岃澶囷紝鍖呮嫭鍍忎吉缁堢锛圥TY锛夎繖鏍风殑铏氭嫙璁惧銆?
## TTY 缁撴瀯


鏈夎嫢骞蹭富瑕佺殑 TTY 缁撴瀯銆傜郴缁熶腑鐨勬瘡涓?TTY 璁惧閮芥湁涓€涓搴旂殑 struct tty_port銆傝繖浜?璁惧鐢变竴涓?TTY 椹卞姩锛堝嵆 struct tty_driver锛夌淮鎶ゃ€傝缁撴瀯鎻忚堪浜嗛┍鍔紝鍚屾椂杩樺寘鍚
鍙湪 TTY 涓婃墽琛岀殑鎿嶄綔鐨勫紩鐢紝鍗?struct tty_operations銆傜劧鍚庯紝鍦ㄦ墦寮€鏃讹紝浼氬垎閰嶄竴涓?struct tty_struct锛屽苟涓€鐩村瓨娲诲埌鏈€缁堝叧闂€傚湪姝ゆ湡闂达紝TTY 灞備細璋冪敤 struct
tty_operations 涓殑鑻ュ共鍥炶皟銆?
鍐呮牳鎺ユ敹鍒扮殑姣忎釜瀛楃锛堟潵鑷澶囧拰鐢ㄦ埛涓ゆ柟锛夐兘浼氶€氳繃涓€涓閫夌殑
[tty_ldisc](tty_ldisc)锛堢畝绉?ldisc锛涘湪 C 涓负 struct tty_ldisc_ops锛変紶閫掋€傚畠鐨?浠诲姟鏄瀛楃杩涜杞崲锛岃浆鎹㈡柟寮忕敱鐗瑰畾鐨?ldisc 鎴栫敤鎴峰畾涔夈€傞粯璁ょ殑鏄?n_tty锛屽畠瀹炵幇浜?鍥炴樉銆佷俊鍙峰鐞嗐€佷綔涓氭帶鍒躲€佺壒娈婂瓧绗﹀鐞嗙瓑銆傝浆鎹㈠悗鐨勫瓧绗︿細鏍规嵁鏉ユ簮杩涗竴姝ヤ紶閫掔粰
鐢ㄦ埛/璁惧銆?
瀵逛笂杩板懡鍚?TTY 缁撴瀯鐨勮缁嗘弿杩板湪鍚勭嫭绔嬫枃妗ｄ腑锛?
- [tty_driver](tty_driver)
- [tty_port](tty_port)
- [tty_struct](tty_struct)
- [tty_ldisc](tty_ldisc)
- [tty_buffer](tty_buffer)
- [tty_ioctl](tty_ioctl)
- [tty_internals](tty_internals)
- [console](console)

## 缂栧啓 TTY 椹卞姩


鍦ㄧ潃鎵嬬紪鍐?TTY 椹卞姩涔嬪墠锛屽繀椤诲厛鑰冭檻 [Serial <../serial/driver>](Serial
<../serial/driver>) 涓?[USB Serial <../../usb/usb-serial>](USB Serial
<../../usb/usb-serial>) 灞傘€備覆琛岃澶囩殑椹卞姩閫氬父鍙互浣跨敤杩欎簺鐗瑰畾灞備箣涓€鏉ュ疄鐜颁竴涓?涓茶椹卞姩銆傚彧鏈夌壒娈婅澶囨墠搴旂敱 TTY 灞傜洿鎺ュ鐞嗐€傚鏋滀綘鎵撶畻缂栧啓杩欐牱鐨勯┍鍔紝璇风户缁槄璇汇€?
涓€涓?TTY 椹卞姩鎵ц鐨?*鍏稿瀷**搴忓垪濡備笅锛?
#. 鍒嗛厤骞舵敞鍐屼竴涓?TTY 椹卞姩锛堟ā鍧楀垵濮嬪寲锛?#. 鍦ㄦ帰娴嬪埌鏃跺垱寤哄苟娉ㄥ唽 TTY 璁惧锛坧robe 鍑芥暟锛?#. 澶勭悊 TTY 鎿嶄綔涓庝簨浠讹紙濡備腑鏂級锛堝墠鑰呯敱 TTY 鏍稿績璋冪敤锛屽悗鑰呯敱璁惧璋冪敤锛?#. 鍦ㄨ澶囩Щ闄ゆ椂绉婚櫎瀹冧滑锛坮emove 鍑芥暟锛?#. 娉ㄩ攢骞堕噴鏀?TTY 椹卞姩锛堟ā鍧楅€€鍑猴級

鏈夊叧椹卞姩鐨勬楠わ紙鍗?1.銆?. 涓?5.锛夊湪 [tty_driver](tty_driver) 涓湁璇︾粏鎻忚堪銆傚浜?鍙﹀涓ゆ锛堣澶囧鐞嗭級锛岃鍙傞槄 [tty_port](tty_port)銆?
## 鍏跺畠鏂囨。


鍏跺畠鏉傞」鏂囨。鍙繘涓€姝ュ湪杩欎簺鏂囨。涓壘鍒帮細

- [moxa-smartio](moxa-smartio)
- [n_gsm](n_gsm)
- [n_tty](n_tty)
