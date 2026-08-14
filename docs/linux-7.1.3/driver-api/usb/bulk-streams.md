#### USB bulk 娴?


## 鑳屾櫙


鎵归噺绔偣娴侊紙bulk endpoint streams锛夊湪 USB 3.0 瑙勮寖涓紩鍏ャ€傛祦鍏佽璁惧椹卞姩瀵逛竴涓壒閲忕鐐硅繘琛屽鐢紝浠庤€屽彲浠ヤ竴娆℃€ф帓闃熷涓紶杈撱€?

娴佸湪 https://www.usb.org/developers/docs/ 鐨?Universal Serial Bus 3.0 瑙勮寖鐨勭 4.4.6.4 鑺備笌绗?8.12.1.4 鑺備腑瀹氫箟銆備娇鐢ㄦ祦鏉ユ帓闃熷涓?SCSI 鍛戒护鐨?USB Attached SCSI Protocol 鍙湪 T10 缃戠珯锛坔ttps://t10.org/锛変笂鎵惧埌銆?


## 璁惧渚у奖鍝?


涓€鏃︾紦鍐插尯琚帓闃熷埌鏌愪釜娴佺幆锛坰tream ring锛夛紝璁惧灏变細锛堥€氳繃鍙︿竴涓鐐逛笂鐨勫甫澶栨満鍒讹級鏀跺埌閫氱煡锛岃〃鏄庤 stream ID 鐨勬暟鎹凡灏辩华銆傞殢鍚庤澶囧憡璇変富鏈哄畠鎯冲惎鍔ㄥ摢涓€滄祦鈥濄€備富鏈轰篃鍙互鍦ㄦ病鏈夎澶囪姹傜殑鎯呭喌涓嬩富鍔ㄥ湪鏌愪釜娴佷笂鍙戣捣浼犺緭锛屼絾璁惧鍙互鎷掔粷璇ヤ紶杈撱€傝澶囧彲浠ラ殢鏃跺湪娴佷箣闂村垏鎹€?


## 椹卞姩褰卞搷


```

  int usb_alloc_streams(struct usb_interface *interface,
		struct usb_host_endpoint **eps, unsigned int num_eps,
		unsigned int num_streams, gfp_t mem_flags);

```

璁惧椹卞姩灏嗚皟鐢ㄦ API锛岃姹備富鏈烘帶鍒跺櫒椹卞姩鍒嗛厤鍐呭瓨锛屼互渚胯椹卞姩鑳藉浣跨敤澶氳揪 num_streams 涓?stream ID銆傚畠浠繀椤讳紶鍏ヤ竴涓渶瑕佷互鐩镐技 stream ID 杩涜璁剧疆鐨?usb_host_endpoints 鏁扮粍銆傝繖鏄负浜嗙‘淇?UASP 椹卞姩鑳藉鍦ㄥ弻鍚戝懡浠ゅ簭鍒楁墍鐢ㄧ殑鎵归噺 IN 涓?OUT 绔偣涓婁娇鐢ㄧ浉鍚岀殑 stream ID銆?

杩斿洖鍊兼槸涓€涓敊璇姸鎬侊紙濡傛灉鏌愪釜绔偣涓嶆敮鎸佹祦锛屾垨 xHCI 椹卞姩鍐呭瓨鑰楀敖锛夛紝鎴栬€呮槸涓绘満鎺у埗鍣ㄤ负璇ョ鐐瑰垎閰嶇殑娴佹暟閲忋€倄HCI 涓绘満鎺у埗鍣ㄧ‖浠跺０鏄庝簡瀹冭兘鏀寔澶氬皯涓?stream ID锛岃€?SuperSpeed 璁惧涓婄殑姣忎釜鎵归噺绔偣涔熶細澹版槑瀹冭兘澶勭悊澶氬皯涓?stream ID銆傚洜姝わ紝椹卞姩搴斿綋鑳藉澶勭悊琚垎閰嶇殑 stream ID 灏戜簬鍏惰姹傛暟閲忕殑鎯呭喌銆?

濡傛灉瀵逛綔涓哄弬鏁颁紶鍏ョ殑鏌愪釜绔偣鏈?URB 宸叉帓闃燂紝璇峰嬁璋冪敤姝ゅ嚱鏁般€備笉瑕佽皟鐢ㄦ鍑芥暟璇锋眰灏戜簬涓や釜娴併€?

鍦ㄦ病鏈夎皟鐢?usb_free_streams() 鐨勬儏鍐典笅锛岄┍鍔ㄥ彧鍏佽瀵瑰悓涓€绔偣璋冪敤姝?API 涓€娆°€傝繖鏄 xHCI 涓绘満鎺у埗鍣ㄩ┍鍔ㄧ殑绠€鍖栵紝鏈潵鍙兘浼氭敼鍙樸€?


## 閫夋嫨瑕佷娇鐢ㄧ殑鏂扮殑 Stream ID


Stream ID 0 鏄繚鐣欑殑锛屼笉搴旇鐢ㄤ簬涓庤澶囬€氫俊銆傚鏋?usb_alloc_streams() 杩斿洖鍊间负 N锛屽垯浣犲彲浠ヤ娇鐢?1 鍒?N 鐨勬祦銆傝涓轰竴涓壒瀹氱殑娴佹帓闃?URB锛岃璁剧疆 urb->stream_id 鐨勫€笺€傚鏋滆绔偣涓嶆敮鎸佹祦锛屽皢杩斿洖閿欒銆?

娉ㄦ剰锛屽鏋?xHCI 椹卞姩鏀寔娆＄骇 stream ID锛屽垯闇€瑕佹柊澧炵敤浜庨€夋嫨涓嬩竴涓?stream ID 鐨?API銆?


## 娓呯悊


濡傛灉椹卞姩甯屾湜鍋滄浣跨敤娴佹潵涓庤澶囬€氫俊锛屽畠

```

  void usb_free_streams(struct usb_interface *interface,
		struct usb_host_endpoint **eps, unsigned int num_eps,
		gfp_t mem_flags);

```

褰撻┍鍔ㄩ噴鏀炬帴鍙ｆ椂锛屾墍鏈?stream ID 閮戒細琚噴鏀撅紝浠ョ‘淇濅笉鏀寔娴佺殑椹卞姩涔熻兘浣跨敤璇ョ鐐广€?
