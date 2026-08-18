## dm-io


dm-io 鎻愪緵鍚屾涓庡紓姝?I/O 鏈嶅姟銆傚叡鏈変笁绫?I/O 鏈嶅姟锛屾瘡绫婚兘鏈夊悓姝ュ拰寮傛涓や釜鐗堟湰銆?
鐢ㄦ埛蹇呴』寤虹珛涓€涓?io_region 缁撴瀯浣撴潵鎻忚堪鏈熸湜鐨?I/O 浣嶇疆銆傛瘡涓?io_region 琛ㄧず涓€涓?鍧楄澶囦互鍙婅捣濮嬩綅缃?```

   struct io_region {
      struct block_device *bdev;
      sector_t sector;
      sector_t count;
   };

```
dm-io 鍙互浠庝竴涓?io_region 璇诲彇锛屾垨鍐欏叆涓€涓垨澶氫釜 io_region銆傚澶氫釜鍖哄煙鐨勫啓鍏ョ敱
io_region 缁撴瀯浣撴暟缁勬寚瀹氥€?
绗竴绫?I/O 鏈嶅姟灏嗕竴缁勫唴瀛橀〉浣滀负 I/O 鐨勬暟鎹紦鍐插尯
```

   struct page_list {
      struct page_list *next;
      struct page *page;
   };

   int dm_io_sync(unsigned int num_regions, struct io_region *where, int rw,
                  struct page_list *pl, unsigned int offset,
                  unsigned long *error_bits);
   int dm_io_async(unsigned int num_regions, struct io_region *where, int rw,
                   struct page_list *pl, unsigned int offset,
                   io_notify_fn fn, void *context);

```
绗簩绫?I/O 鏈嶅姟灏嗕竴涓?bio 鍚戦噺鏁扮粍浣滀负 I/O 鐨勬暟鎹紦鍐插尯銆傚鏋滆皟鐢ㄦ柟宸茬粡棰勫厛缁勮濂?涓€涓?bio锛岃鏈嶅姟浼氶潪甯告柟渚?```

   int dm_io_sync_bvec(unsigned int num_regions, struct io_region *where,
                       int rw, struct bio_vec *bvec,
                       unsigned long *error_bits);
   int dm_io_async_bvec(unsigned int num_regions, struct io_region *where,
                        int rw, struct bio_vec *bvec,
                        io_notify_fn fn, void *context);

```
绗笁绫?I/O 鏈嶅姟灏嗕竴涓寚鍚?vmalloc 鍒嗛厤鐨勫唴瀛樼紦鍐插尯鐨勬寚閽堜綔涓?I/O 鐨勬暟鎹紦鍐插尯銆?濡傛灉璋冪敤鏂归渶瑕佸涓€涓ぇ鍖哄煙鎵ц I/O锛屼絾鍙堜笉鎯冲垎閰嶅ぇ閲忕嫭绔嬬殑
```

   int dm_io_sync_vm(unsigned int num_regions, struct io_region *where, int rw,
                     void *data, unsigned long *error_bits);
   int dm_io_async_vm(unsigned int num_regions, struct io_region *where, int rw,
                      void *data, io_notify_fn fn, void *context);

```
寮傛 I/O 鏈嶅姟鐨勮皟鐢ㄦ柟蹇呴』鍖呭惈涓€涓畬鎴愬洖璋冪殑鍚嶇О
```

   typedef void (*io_notify_fn)(unsigned long error, void *context);

```
姝ゅ洖璋冧腑鐨?"error" 鍙傛暟锛屼互鍙婃墍鏈夊悓姝ョ増鏈腑鐨?`*error` 鍙傛暟锛岄兘鏄竴涓綅闆嗭紙鑰岄潪
绠€鍗曠殑閿欒鍊硷級銆傚湪鍐欏叆澶氫釜鍖哄煙鐨勫啓 I/O 鎯呭喌涓嬶紝璇ヤ綅闆嗕娇 dm-io 鑳藉鎸囩ず姣忎釜鍗曠嫭
鍖哄煙鐨勬垚鍔熸垨澶辫触銆?
鍦ㄤ娇鐢ㄤ换浣?dm-io 鏈嶅姟涔嬪墠锛岀敤鎴峰簲璋冪敤 dm_io_get() 骞舵寚瀹氬叾鏈熸湜骞跺彂鎵ц I/O 鐨?椤垫暟銆俤m-io 浼氬皾璇曡皟鏁村叾鍐呭瓨姹犵殑澶у皬锛屼互纭繚濮嬬粓鏈夎冻澶熺殑椤靛彲鐢紝浠庤€屽湪鎵ц I/O
鏃堕伩鍏嶄笉蹇呰鐨勭瓑寰呫€?
褰撶敤鎴蜂娇鐢ㄥ畬 dm-io 鏈嶅姟鍚庯紝搴旇皟鐢?dm_io_put() 骞舵寚瀹氫笌 dm_io_get() 璋冪敤鏃剁浉鍚岀殑
椤垫暟銆?
