## kcopyd


kcopyd 鎻愪緵灏嗕竴娈垫墖鍖鸿寖鍥翠粠涓€涓潡璁惧澶嶅埗鍒颁竴涓垨澶氫釜鍏朵粬鍧楄澶囩殑鑳藉姏锛屽苟甯︽湁寮傛瀹屾垚閫氱煡銆傚畠鐢?dm-snapshot 鍜?dm-mirror 浣跨敤銆?
kcopyd 鐨勭敤鎴峰繀椤诲厛鍒涘缓涓€涓鎴风锛屽苟鎸囨槑瑕佷负鍏跺鍒朵綔涓氶鐣欏灏戝唴瀛橀〉銆傝繖鏄€氳繃璋冪敤
```

   int kcopyd_client_create(unsigned int num_pages,
                            struct kcopyd_client **result);

```
鏉ュ畬鎴愮殑銆?
瑕佸惎鍔ㄤ竴涓鍒朵綔涓氾紝鐢ㄦ埛蹇呴』璁剧疆 io_region 缁撴瀯浣撴潵鎻忚堪澶嶅埗鐨勬簮鍜岀洰鐨勫湴銆傛瘡涓?io_region 琛ㄧず涓€涓潡璁惧浠ュ強璇ュ尯鍩熺殑璧峰鎵囧尯鍜屽ぇ灏忋€傚鍒剁殑婧愪互涓€涓?io_region 缁撴瀯浣撶粰鍑猴紝鐩殑鍦颁互
```

   struct io_region {
      struct block_device *bdev;
      sector_t sector;
      sector_t count;
   };

```
缁欏嚭銆?
瑕佸惎鍔ㄥ鍒讹紝鐢ㄦ埛璋冪敤 kcopyd_copy()锛屼紶鍏ュ鎴风鎸囬拡銆佹寚鍚戞簮鍜岀洰鏍?io_region 鐨勬寚閽堛€佸悕绉?```

   int kcopyd_copy(struct kcopyd_client *kc, struct io_region *from,
                   unsigned int num_dests, struct io_region *dests,
                   unsigned int flags, kcopyd_notify_fn fn, void *context);

   typedef void (*kcopyd_notify_fn)(int read_err, unsigned int write_err,
				    void *context);

```
褰撳鍒跺畬鎴愭椂锛宬copyd 灏嗚皟鐢ㄧ敤鎴风殑瀹屾垚渚嬬▼锛屼紶鍥炵敤鎴风殑 context 鎸囬拡銆傚畠杩樹細鎸囩ず澶嶅埗杩囩▼涓槸鍚﹀彂鐢熶簡璇绘垨鍐欓敊璇€?
褰撶敤鎴峰畬鎴愭墍鏈夊鍒朵綔涓氬悗锛屽簲璋冪敤 kcopyd_client_destroy() 鏉ュ垹闄?kcopyd 瀹㈡埛绔紝杩欏皢閲婃斁
```

   void kcopyd_client_destroy(struct kcopyd_client *kc);

```
