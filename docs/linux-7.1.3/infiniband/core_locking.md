## InfiniBand 涓棿灞傚姞閿?


  鏈寚鍗楄瘯鍥炬槑纭?InfiniBand 涓棿灞傛墍鍋氬嚭鐨勫姞閿佸亣璁俱€傚畠鎻忚堪浜嗗浣嶄簬涓棿灞備箣涓嬬殑搴曞眰椹卞姩浠ュ強浣跨敤涓棿灞傜殑涓婂眰鍗忚鐨勮姹傘€?

## 鐫＄湢涓庝腑鏂笂涓嬫枃


  闄や互涓嬩緥澶栵紝搴曞眰椹卞姩瀵?struct ib_device 涓墍鏈夋柟娉曠殑瀹炵幇閮藉彲鑳界潯鐪犮€備緥澶栨槸鏉ヨ嚜涓嬪垪鍒楄〃鐨勪换浣曟柟娉曪細

    - create_ah
    - modify_ah
    - query_ah
    - destroy_ah
    - post_send
    - post_recv
    - poll_cq
    - req_notify_cq

  杩欎簺鏂规硶涓嶈兘鐫＄湢锛屽苟涓斿繀椤诲彲浠ヤ粠浠讳綍涓婁笅鏂囪皟鐢ㄣ€?

  瀵煎嚭缁欎笂灞傚崗璁娇鐢ㄨ€呯殑鐩稿簲鍑芥暟锛?

    - rdma_create_ah
    - rdma_modify_ah
    - rdma_query_ah
    - rdma_destroy_ah
    - ib_post_send
    - ib_post_recv
    - ib_req_notify_cq

  鍥犳鍙互瀹夊叏鍦颁粠浠讳綍涓婁笅鏂囪皟鐢ㄣ€?

  姝ゅ锛屽嚱鏁?

    - ib_dispatch_event

  鐢卞簳灞傞┍鍔ㄧ敤鏉ラ€氳繃涓棿灞傛淳鍙戝紓姝ヤ簨浠讹紝涔熷彲浠ュ畨鍏ㄥ湴浠庝换浣曚笂涓嬫枃璋冪敤銆?

### 鍙噸鍏ユ€?


  搴曞眰椹卞姩瀵煎嚭鐨?struct ib_device 涓殑鎵€鏈夋柟娉曢兘蹇呴』瀹屽叏鍙噸鍏ャ€傚簳灞傞┍鍔ㄩ渶瑕佹墽琛屾墍鏈夊繀瑕佺殑鍚屾浠ヤ繚鎸佷竴鑷存€э紝鍗充娇浣跨敤鍚屼竴瀵硅薄鐨勫涓嚱鏁拌皟鐢ㄥ悓鏃惰繍琛屼篃鏄姝ゃ€?

  IB 涓棿灞備笉瀵瑰嚱鏁拌皟鐢ㄦ墽琛屼换浣曚覆琛屽寲銆?

  鐢变簬搴曞眰椹卞姩鏄彲閲嶅叆鐨勶紝涓婂眰鍗忚浣跨敤鑰呬笉闇€瑕佹墽琛屼换浣曚覆琛屽寲銆傜劧鑰岋紝涓轰簡鑾峰緱鍚堢悊鐨勭粨鏋滐紝鍙兘闇€瑕佷竴浜涗覆琛屽寲銆備緥濡傦紝浣跨敤鑰呭彲浠ュ畨鍏ㄥ湴鍦ㄥ涓?CPU 涓婂悓鏃惰皟鐢?ib_poll_cq()銆備絾鏄紝涓嶅悓 ib_poll_cq() 璋冪敤涔嬮棿鐨勫伐浣滃畬鎴愪俊鎭殑椤哄簭骞舵湭瀹氫箟銆?

### 鍥炶皟


  搴曞眰椹卞姩涓嶅緱鍦ㄤ笌 ib_device 鏂规硶璋冪敤鐩稿悓鐨勮皟鐢ㄩ摼涓洿鎺ユ墽琛屽洖璋冦€備緥濡傦紝搴曞眰椹卞姩涓嶅厑璁镐粠鍏?post_send 鏂规硶涓洿鎺ヨ皟鐢ㄤ娇鐢ㄨ€呯殑瀹屾垚浜嬩欢澶勭悊绋嬪簭銆傜浉鍙嶏紝搴曞眰椹卞姩搴旈€氳繃渚嬪璋冨害涓€涓?tasklet 鏉ユ墽琛屽洖璋冿紝浠庤€屾帹杩熻鍥炶皟銆?

  搴曞眰椹卞姩璐熻矗纭繚鍚屼竴 CQ 鐨勫涓畬鎴愪簨浠跺鐞嗙▼搴忎笉浼氳鍚屾椂璋冪敤銆傞┍鍔ㄥ繀椤讳繚璇佸浜庣粰瀹氱殑 CQ锛屼换涓€鏃跺埢鍙湁涓€涓?CQ 浜嬩欢澶勭悊绋嬪簭鍦ㄨ繍琛屻€傛崲鍙ヨ瘽璇达紝
```

          CPU1                                    CPU2

    low-level driver ->
      consumer CQ event callback:
        /* ... */
        ib_req_notify_cq(cq, ...);
                                          low-level driver ->
        /* ... */                           consumer CQ event callback:
                                              /* ... */
        return from CQ event handler

  The context in which completion event and asynchronous event
  callbacks run is not defined.  Depending on the low-level driver, it
  may be process context, softirq context, or interrupt context.
  Upper level protocol consumers may not sleep in a callback.

```
### 鐑彃鎷?


  搴曞眰椹卞姩鍦ㄨ皟鐢?ib_register_device() 鏃跺悜浣跨敤鑰呭鍛婅澶囧凡鍙緵浣跨敤锛屾墍鏈夊垵濮嬪寲蹇呴』鍦ㄦ璋冪敤涔嬪墠瀹屾垚銆傝澶囧繀椤讳繚鎸佸彲鐢紝鐩村埌椹卞姩鐨?ib_unregister_device() 璋冪敤杩斿洖銆?

  搴曞眰椹卞姩蹇呴』浠庤繘绋嬩笂涓嬫枃璋冪敤 ib_register_device() 鍜?ib_unregister_device()銆傚畠涓嶈兘鎸佹湁浠讳綍鍙兘鍦ㄤ娇鐢ㄨ€呴€氳繃杩欎簺璋冪敤鍥炶皟鍒伴┍鍔ㄦ椂瀵艰嚧姝婚攣鐨勪俊鍙烽噺銆?

  涓€鏃︿负鍏惰皟鐢ㄤ簡 struct ib_client 鐨?add 鏂规硶锛屼笂灞傚崗璁娇鐢ㄨ€呭氨鍙互寮€濮嬩娇鐢ㄨ IB 璁惧銆備娇鐢ㄨ€呭繀椤诲湪浠?remove 鏂规硶杩斿洖涔嬪墠瀹屾垚鎵€鏈夋竻鐞嗗苟閲婃斁涓庤璁惧鐩稿叧鐨勬墍鏈夎祫婧愩€?

  浣跨敤鑰呭彲浠ュ湪鍏?add 鍜?remove 鏂规硶涓潯鐪犮€?
