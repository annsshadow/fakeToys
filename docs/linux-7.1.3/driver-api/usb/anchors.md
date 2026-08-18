#### USB Anchors


## 浠€涔堟槸 anchor锛?

USB 椹卞姩闇€瑕佹敮鎸佷竴浜涘洖璋冿紝杩欎簺鍥炶皟瑕佹眰椹卞姩鍋滄瀵规煇涓帴鍙ｇ殑鎵€鏈?IO銆備负姝わ紝椹卞姩蹇呴』璁板綍瀹冨凡鎻愪氦鐨?URB锛屼互纭瀹冧滑鍏ㄩ儴瀹屾垚锛屾垨瀵瑰叾璋冪敤 usb_kill_urb銆俛nchor 鏄竴绉嶆暟鎹粨鏋勶紝璐熻矗璁板綍 URB 骞舵彁渚涘鐞嗗涓?URB 鐨勬柟娉曘€?
## 鍒嗛厤涓庡垵濮嬪寲


娌℃湁鐢ㄤ簬鍒嗛厤 anchor 鐨?API銆傚畠鍙槸琚０鏄庝负 struct usb_anchor銆傚繀椤昏皟鐢?`init_usb_anchor` 鏉ュ垵濮嬪寲璇ユ暟鎹粨鏋勩€?
## 閲婃斁


涓€鏃?anchor 涓嶅啀鍏宠仈浠讳綍 URB锛屽氨鍙互浣跨敤鏅€氱殑鍐呭瓨绠＄悊鎿嶄綔灏嗗叾閲婃斁銆?
## URB 涓?anchor 鐨勫叧鑱斾笌瑙ｉ櫎鍏宠仈


閫氳繃灏?URB 鏄惧紡璋冪敤 `usb_anchor_urb` 鏉ュ缓绔?URB 涓?anchor 鐨勫叧鑱斻€傝鍏宠仈浼氫竴鐩翠繚鎸侊紝鐩村埌 URB 閫氳繃锛堟垚鍔燂級瀹屾垚鑰岀粨鏉熴€傚洜姝よВ闄ゅ叧鑱旀槸鑷姩鐨勩€傝繕鎻愪緵浜嗕竴涓嚱鏁扮敤浜庡己鍒剁粨鏉燂紙kill锛変笌鏌愪釜 anchor 鍏宠仈鐨勬墍鏈?URB銆?姝ゅ锛屼篃鍙互閫氳繃 `usb_unanchor_urb` 杩涜瑙ｉ櫎鍏宠仈銆?
## 瀵瑰涓?URB 鐨勬搷浣?

### :c:func:`usb_kill_anchored_urbs`


璇ュ嚱鏁版潃姝讳笌鏌愪釜 anchor 鍏宠仈鐨勬墍鏈?URB銆俇RB 浼氭寜鐓у叾鎻愪氦鐨勬椂闂撮€嗗簭琚皟鐢ㄣ€傝繖鏍峰彲浠ヤ繚璇佹暟鎹笉浼氳閲嶆帓搴忋€?
### :c:func:`usb_scuttle_anchored_urbs`


鏌愪釜 anchor 鐨勬墍鏈?URB 浼氳涓€娆℃€у叏閮ㄨВ闄ら敋瀹氥€?
### :c:func:`usb_wait_anchor_empty_timeout`


璇ュ嚱鏁扮瓑寰呬笌鏌愪釜 anchor 鍏宠仈鐨勬墍鏈?URB 瀹屾垚鎴栬秴鏃讹紝浠ュ厛鍙戠敓鑰呬负鍑嗐€傚叾杩斿洖鍊间細鍛婅瘔浣犳槸鍚﹁揪鍒颁簡瓒呮椂銆?
### :c:func:`usb_anchor_empty`


濡傛灉娌℃湁 URB 涓?anchor 鍏宠仈锛屽垯杩斿洖 true銆傚姞閿佺敱璋冪敤鑰呰礋璐ｃ€?
### :c:func:`usb_get_from_anchor`


杩斿洖鏌愪釜 anchor 涓渶鏃х殑閿氬畾 URB銆傝 URB 浼氳瑙ｉ櫎閿氬畾骞跺甫寮曠敤杩斿洖銆傜敱浜庝綘鍙互鍦ㄤ竴涓?anchor 涓贩鍚堟寚鍚戝涓洰鏍囩殑 URB锛屽洜姝ゆ棤娉曚繚璇佽繑鍥炵殑鏄寜鏃堕棿椤哄簭鏈€鍏堟彁浜ょ殑 URB銆?