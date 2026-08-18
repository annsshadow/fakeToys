## 妫€鏌ヨ繘绋嬮〉琛?

pagemap 鏄唴鏍镐腑涓€缁勮緝鏂扮殑鎺ュ彛锛堣嚜 2.6.25 璧峰紩鍏ワ級锛屽畠鍏佽
鐢ㄦ埛绌洪棿绋嬪簭閫氳繃璇诲彇 `/proc` 涓嬬殑鏂囦欢鏉ユ鏌ラ〉琛ㄤ互鍙婄浉鍏充俊鎭€?
pagemap 鍖呭惈鍥涗釜缁勬垚閮ㄥ垎锛?
 - `/proc/pid/pagemap`銆傝鏂囦欢璁╀竴涓敤鎴风┖闂磋繘绋嬫煡鎵惧嚭姣忎釜铏氭嫙椤垫槧灏勫埌
   鍝釜鐗╃悊椤靛抚锛坧hysical frame锛夈€傚畠涓烘瘡涓櫄鎷熼〉鍖呭惈涓€涓?64 浣嶇殑鍊硷紝鍏朵腑
   鍚湁濡備笅鏁版嵁锛堟潵鑷?`fs/proc/task_mmu.c`锛宲agemap_read 涔嬩笂锛夛細

    - Bit 0-54  椤靛抚鍙凤紙PFN锛宲age frame number锛夛紝鑻ュ瓨鍦?    - Bit 0-4   浜ゆ崲绫诲瀷锛坰wap type锛夛紝鑻ュ凡鎹㈠嚭
    - Bit 5-54  浜ゆ崲鍋忕Щ锛坰wap offset锛夛紝鑻ュ凡鎹㈠嚭
    - Bit 55    pte 涓鸿蒋鑴忥紙soft-dirty锛夛紙鍙傝
      Documentation/admin-guide/mm/soft-dirty.rst锛?    - Bit 56    椤佃鐙崰鏄犲皠锛堣嚜 4.2 璧凤級
    - Bit 57    pte 琚?uffd-wp 鍐欎繚鎶わ紙鑷?5.13 璧凤級锛堝弬瑙?      Documentation/admin-guide/mm/userfaultfd.rst锛?    - Bit 58    pte 鏄竴涓繚鎶ゅ尯鍩燂紙guard region锛夛紙鑷?6.15 璧凤級锛堝弬瑙?madvise (2) 鎵嬪唽椤碉級
    - Bit 59-60 鍏ㄩ浂
    - Bit 61    椤典负鏂囦欢椤垫垨鍏变韩鍖垮悕椤碉紙鑷?3.5 璧凤級
    - Bit 62    椤靛凡鎹㈠嚭
    - Bit 63    椤靛瓨鍦?
   鑷?Linux 4.0 璧凤紝鍙湁鎷ユ湁 CAP_SYS_ADMIN 鑳藉姏鐨勭敤鎴锋墠鑳借幏鍙?PFN銆傚湪 4.0 鍜?   4.1 涓紝鏃犵壒鏉冪敤鎴风殑鎵撳紑鎿嶄綔浼氫互 -EPERM 澶辫触銆備粠 4.2 寮€濮嬶紝濡傛灉鐢ㄦ埛娌℃湁
   CAP_SYS_ADMIN锛孭FN 瀛楁浼氳娓呴浂銆傚師鍥狅細鍏充簬 PFN 鐨勪俊鎭湁鍔╀簬鍒╃敤 Rowhammer
   婕忔礊銆?
   濡傛灉椤典笉瀛樺湪浣嗕綅浜庝氦鎹㈠尯涓紝鍒?PFN 鍖呭惈浜ゆ崲鏂囦欢缂栧彿浠ュ強璇ラ〉鍦ㄤ氦鎹㈠尯涓亸绉婚噺鐨?   缂栫爜銆傛湭鏄犲皠鐨勯〉杩斿洖绌?PFN銆傝繖鏍峰彲浠ョ簿纭湴鍒ゆ柇鍝簺椤佃鏄犲皠锛堟垨浣嶄簬浜ゆ崲鍖轰腑锛夛紝
   骞跺湪杩涚▼涔嬮棿姣旇緝琚槧灏勭殑椤点€?
   浼犵粺涓婏紝bit 56 琛ㄧず涓€椤垫伆濂借鏄犲皠涓€娆★紝鑰屽綋涓€椤佃澶氭鏄犲皠鏃讹紙鍗充娇鍦ㄥ悓涓€杩涚▼涓
   澶氭鏄犲皠锛夛紝bit 56 浼氳娓呴櫎銆傚湪鏌愪簺鍐呮牳閰嶇疆涓紝瀵逛簬灞炰簬杈冨ぇ鍒嗛厤锛堝 THP锛夌殑椤碉紝
   鍏惰涔夊彲鑳戒笉鍚岋細濡傛灉瀵瑰簲澶у垎閰嶇殑鎵€鏈夐〉**纭疄**閮芥槧灏勫湪鍚屼竴杩涚▼涓紝鍗充娇璇ラ〉鍦ㄨ
   杩涚▼涓鏄犲皠澶氭锛屼篃浼氳缃?bit 56銆傚綋澶у垎閰嶄腑鐨勪换涓€椤?*鍙兘**鏄犲皠鍦ㄥ彟涓€涓繘绋嬫椂锛?   bit 56 浼氳娓呴櫎銆傚湪鏌愪簺鎯呭喌涓嬶紝涓€涓ぇ鍒嗛厤鍙兘浼氳瑙嗕负"鍙兘琚涓繘绋嬫槧灏?锛屽嵆浣?   瀹為檯鎯呭喌宸蹭笉鍐嶅姝ゃ€?
   璇ユ帴鍙ｇ殑楂樻晥浣跨敤鑰呬細鍒╃敤 `/proc/pid/maps` 鏉ョ‘瀹氬唴瀛樹腑瀹為檯琚槧灏勭殑鍖哄煙锛屽苟浣跨敤
   llseek 璺宠繃鏈槧灏勭殑鍖哄煙銆?
 - `/proc/kpagecount`銆傝鏂囦欢鍖呭惈涓€涓?64 浣嶇殑璁℃暟锛岃〃绀烘瘡涓〉琚槧灏勭殑娆℃暟锛屼互 PFN
   涓虹储寮曘€傛煇浜涘唴鏍搁厤缃笉浼氱簿纭窡韪睘浜庤緝澶у垎閰嶏紙濡?THP锛夌殑椤佃鏄犲皠鐨勬鏁般€傚湪杩欎簺
   閰嶇疆涓紝浼氳繑鍥炶澶у垎閰嶄腑姣忛〉鏄犲皠娆℃暟鐨勫钩鍧囧€硷紱浣嗗彧瑕佸ぇ鍒嗛厤涓殑浠讳竴椤佃鏄犲皠锛岃繑鍥?   鍊煎氨鑷冲皯涓?1銆?
tools/mm 鐩綍涓嬬殑 page-types 宸ュ叿鍙敤浜庢煡璇竴涓〉琚槧灏勭殑娆℃暟銆?
 - `/proc/kpageflags`銆傝鏂囦欢涓烘瘡涓〉鍖呭惈涓€涓?64 浣嶇殑鏍囧織闆嗗悎锛屼互 PFN 涓虹储寮曘€?
   杩欎簺鏍囧織濡備笅锛堟潵鑷?`fs/proc/page.c`锛宬pageflags_read 涔嬩笂锛夛細

    0. LOCKED
    1. ERROR
    2. REFERENCED
    3. UPTODATE
    4. DIRTY
    5. LRU
    6. ACTIVE
    7. SLAB
    8. WRITEBACK
    9. RECLAIM
    10. BUDDY
    11. MMAP
    12. ANON
    13. SWAPCACHE
    14. SWAPBACKED
    15. COMPOUND_HEAD
    16. COMPOUND_TAIL
    17. HUGE
    18. UNEVICTABLE
    19. HWPOISON
    20. NOPAGE
    21. KSM
    22. THP
    23. OFFLINE
    24. ZERO_PAGE
    25. IDLE
    26. PGTABLE

 - `/proc/kpagecgroup`銆傝鏂囦欢鍖呭惈姣忎釜椤垫墍璁″叆锛坈harged锛夌殑鍐呭瓨 cgroup 鐨?64 浣?   inode 缂栧彿锛屼互 PFN 涓虹储寮曘€備粎褰撹缃簡 CONFIG_MEMCG 鏃舵墠鍙敤銆?
## 椤垫爣蹇楃畝杩?

0 - LOCKED
   璇ラ〉姝ｈ閿佸畾浠ヨ繘琛岀嫭鍗犺闂紝渚嬪姝ｅ湪杩涜璇?鍐?IO銆?
7 - SLAB
   璇ラ〉鐢?SLAB/SLUB 鍐呮牳鍐呭瓨鍒嗛厤鍣ㄧ鐞嗐€?   浣跨敤澶嶅悎椤碉紙compound page锛夋椂锛屼簩鑰呴兘鍙細鍦ㄥご椤典笂璁剧疆璇ユ爣蹇椼€?
10 - BUDDY
   鐢变紮浼寸郴缁熷垎閰嶅櫒绠＄悊鐨勭┖闂插唴瀛樺潡銆?   浼欎即绯荤粺灏嗙┖闂插唴瀛樼粍缁囦负涓嶅悓闃讹紙order锛夌殑鍧椼€?   涓€涓樁涓?N 鐨勫潡鍖呭惈 2^N 涓墿鐞嗚繛缁殑椤碉紝鎵€鏈夐〉閮借缃簡 BUDDY 鏍囧織銆?   鍦?4.6 涔嬪墠锛屽彧鏈夎鍧楃殑绗竴涓〉璁剧疆浜嗚鏍囧織銆?
15 - COMPOUND_HEAD
   涓€涓樁涓?N 鐨勫鍚堥〉鐢?2^N 涓墿鐞嗚繛缁殑椤电粍鎴愩€?   涓€涓樁涓?2 鐨勫鍚堥〉褰㈠ "HTTT"锛屽叾涓?H 琛ㄧず鍏跺ご椤碉紝T 琛ㄧず鍏跺熬椤点€?   澶嶅悎椤电殑涓昏浣跨敤鑰呮湁 hugeTLB 椤碉紙Documentation/admin-guide/mm/hugetlbpage.rst锛夈€?   SLUB 绛夊唴瀛樺垎閰嶅櫒浠ュ強鍚勭璁惧椹卞姩銆?   浣嗗湪璇ユ帴鍙ｄ腑锛屽彧鏈?huge/giga 椤靛鏈€缁堢敤鎴峰彲瑙併€?
16 - COMPOUND_TAIL
   澶嶅悎椤电殑灏鹃〉锛堝弬瑙佷笂闈㈢殑鎻忚堪锛夈€?
17 - HUGE
   杩欐槸 HugeTLB 椤电殑缁勬垚閮ㄥ垎銆?
19 - HWPOISON
   璇ラ〉涓婅纭欢妫€娴嬪埌鍐呭瓨鎹熷潖锛氫笉瑕佺杩欎簺鏁版嵁锛?
20 - NOPAGE
   鍦ㄨ姹傚湴鍧€澶勪笉瀛樺湪椤靛抚銆?
21 - KSM
   鍦ㄤ竴涓垨澶氫釜杩涚▼涔嬮棿鍔ㄦ€佸叡浜殑鐩稿悓鍐呭瓨椤点€?
22 - THP
   鏋勬垚浠绘剰澶у皬鐨?THP 骞朵互浠绘剰绮掑害鏄犲皠鐨勮繛缁〉銆?
23 - OFFLINE
   璇ラ〉鍦ㄩ€昏緫涓婂凡绂荤嚎銆?
24 - ZERO_PAGE
   鐢ㄤ簬 pfn_zero 鎴?huge_zero 鐨勯浂椤点€?
25 - IDLE
   璇ラ〉鑷鏍囪涓?idle 浠ユ潵灏氭湭琚闂紙鍙傝
   Documentation/admin-guide/mm/idle_page_tracking.rst锛夈€?   娉ㄦ剰锛屽鏋滆椤垫槸閫氳繃 PTE 璁块棶鐨勶紝璇ユ爣蹇楀彲鑳戒細杩囨椂銆備负纭繚鏍囧織鏄渶鏂扮殑锛岄渶瑕佸厛璇诲彇
   `/sys/kernel/mm/page_idle/bitmap`銆?
26 - PGTABLE
   璇ラ〉姝ｈ鐢ㄤ綔椤佃〃銆?
### 涓?IO 鐩稿叧鐨勯〉鏍囧織


1 - ERROR
   鍙戠敓浜?IO 閿欒銆?
3 - UPTODATE
   璇ラ〉鍚湁鏈€鏂扮殑鏁版嵁銆?   鍗筹紝瀵逛簬鏂囦欢鍚庡椤碉細锛堝唴瀛樹腑鏁版嵁鐗堟湰 >= 纾佺洏涓婄増鏈級

4 - DIRTY
   璇ラ〉宸茶鍐欏叆锛屽洜鑰屽惈鏈夋柊鏁版嵁銆?   鍗筹紝瀵逛簬鏂囦欢鍚庡椤碉細锛堝唴瀛樹腑鏁版嵁鐗堟湰 > 纾佺洏涓婄増鏈級

8 - WRITEBACK
   璇ラ〉姝ｈ鍚屾鍒扮鐩樸€?
### 涓?LRU 鐩稿叧鐨勯〉鏍囧織


5 - LRU
   璇ラ〉浣嶄簬鏌愪釜 LRU 閾捐〃涓€?
6 - ACTIVE
   璇ラ〉浣嶄簬娲昏穬鐨?LRU 閾捐〃涓€?
18 - UNEVICTABLE
   璇ラ〉浣嶄簬涓嶅彲鍥炴敹锛堥潪锛塋RU 閾捐〃涓€傚畠浠ユ煇绉嶆柟寮忚閽変綇锛屼笉鏄?LRU 椤靛洖鏀剁殑鍊欓€夊璞★紝
   渚嬪 ramfs 椤点€乻hmctl(SHM_LOCK) 浠ュ強 mlock() 鍐呭瓨娈点€?
2 - REFERENCED
   鑷笂娆″叆闃?閲嶆柊鍏ラ槦 LRU 閾捐〃浠ユ潵锛岃椤靛凡琚紩鐢ㄣ€?
9 - RECLAIM
   鍦ㄥ叾椤垫崲鍑?IO 瀹屾垚鍚庯紝璇ラ〉灏嗗緢蹇鍥炴敹銆?
11 - MMAP
   涓€涓唴瀛樻槧灏勯〉銆?
12 - ANON
   涓€涓笉灞炰簬鏂囦欢鐨勫唴瀛樻槧灏勯〉銆?
13 - SWAPCACHE
   璇ラ〉琚槧灏勫埌浜ゆ崲绌洪棿锛屽嵆鎷ユ湁涓€涓叧鑱旂殑浜ゆ崲椤癸紙swap entry锛夈€?
14 - SWAPBACKED
   璇ラ〉鐢变氦鎹㈢┖闂?RAM 鏀寔銆?
tools/mm 鐩綍涓嬬殑 page-types 宸ュ叿鍙敤浜庢煡璇笂杩版爣蹇椼€?
## 鍏变韩鍐呭瓨鐨勪緥澶栨儏鍐?

褰撳叡浜〉琚?zap 鎴栨崲鍑烘椂锛屽叾椤佃〃椤逛細琚竻闄ゃ€傝繖浣垮緱琚崲鍑虹殑椤典笌浠庢湭鍒嗛厤鐨勯〉鏃犳硶鍖哄垎銆?
鍦ㄥ唴鏍哥┖闂达紝浠嶇劧鍙互浠庨〉缂撳瓨锛坧age cache锛変腑鍙栧洖浜ゆ崲浣嶇疆銆備絾鏄紝浠呭瓨鍌ㄥ湪鏅€?PTE
涓婄殑鍊煎湪椤佃鎹㈠嚭鏃朵細姘镐箙涓㈠け锛堝嵆 SOFT_DIRTY锛夈€?
鍦ㄧ敤鎴风┖闂达紝璇ラ〉鏄瓨鍦ㄣ€佸凡鎹㈠嚭杩樻槸鏃狅紝鍙互鍊熷姪 lseek 鍜?鎴?mincore 绯荤粺璋冪敤鏉ユ帹鏂€?
lseek() 鍙互閫氳繃鍦ㄩ〉鎵€鍚庡鐨勬枃浠朵笂鎸囧畾 SEEK_DATA 鏍囧織锛屾潵鍖哄垎琚闂繃鐨勯〉
锛堝瓨鍦ㄦ垨宸叉崲鍑猴級涓庣┖娲烇紙鏃?鏈垎閰嶏級銆傚浜庡尶鍚嶅叡浜〉锛岃鏂囦欢鍙互鍦?`/proc/pid/map_files/` 涓壘鍒般€?
mincore() 鍙互鍖哄垎浣嶄簬鍐呭瓨涓殑椤碉紙瀛樺湪锛屽寘鎷氦鎹㈢紦瀛橈級鍜屼笉鍦ㄥ唴瀛樹腑鐨勯〉
锛堝凡鎹㈠嚭鎴栨棤/鏈垎閰嶏級銆?
## 鍏朵粬娉ㄦ剰浜嬮」


濡傛灉璇诲彇涓嶆槸浠?8 瀛楄妭杈圭晫澶勫紑濮嬶紙渚嬪锛屼綘 seek 鍒版枃浠朵腑濂囨暟涓瓧鑺傚锛夛紝鎴栬€呰鍙栫殑
澶у皬涓嶆槸 8 瀛楄妭鐨勬暣鏁板€嶏紝閭ｄ箞浠庝换浣曡繖浜涙枃浠朵腑璇诲彇閮戒細杩斿洖 -EINVAL銆?
鍦?Linux 3.11 涔嬪墠锛宲agemap 鐨?bit 55-60 鐢ㄤ簬 "page-shift"锛堝湪澶у鏁版灦鏋勪笂濮嬬粓涓?12锛夈€?鑷?Linux 3.11 璧凤紝鍏跺惈涔夊湪棣栨娓呴櫎杞剰浣嶅悗鍙戠敓鍙樺寲銆傝嚜 Linux 4.2 璧凤紝瀹冧滑琚棤鏉′欢鍦?鐢ㄤ簬鏍囧織銆?
## Pagemap Scan IOCTL


pagemap 鏂囦欢涓婄殑 `PAGEMAP_SCAN` IOCTL 鍙敤浜庤幏鍙栨垨锛堝彲閫夊湴锛夋竻闄ゅ叧浜庨〉琛ㄩ」鐨?淇℃伅銆傝 IOCTL 鏀寔浠ヤ笅鎿嶄綔锛?
- 鎵弿鍦板潃鑼冨洿骞惰幏鍙栦笌鎵€鎻愪緵鏉′欢鍖归厤鐨勫唴瀛樿寖鍥淬€傝繖鍦ㄨ繘琛屾寚瀹氳緭鍑虹紦鍐插尯鏃舵墽琛屻€?
- 鍐欎繚鎶よ繖浜涢〉銆備娇鐢?`PM_SCAN_WP_MATCHING` 鏉ュ啓淇濇姢鎰熷叴瓒ｇ殑椤点€俙PM_SCAN_CHECK_WPASYNC`
  鍦ㄥ彂鐜伴潪寮傛鍐欎繚鎶ょ殑椤垫椂涓鎿嶄綔銆俙PM_SCAN_WP_MATCHING` 鍙互閰嶅悎鎴栦笉閰嶅悎
  `PM_SCAN_CHECK_WPASYNC` 浣跨敤銆?
- 杩欎袱涓搷浣滃彲浠ョ粍鍚堟垚涓€涓師瀛愭搷浣滐紝鍦ㄨ鎿嶄綔涓彲浠ュ悓鏃惰幏鍙栧苟鍐欎繚鎶よ繖浜涢〉銆?
褰撳墠鏀寔浠ヤ笅鍏充簬椤电殑鏍囧織锛?
- `PAGE_IS_WPALLOWED` - 椤靛凡鍚敤寮傛鍐欎繚鎶?- `PAGE_IS_WRITTEN` - 椤佃嚜琚啓淇濇姢浠ユ潵宸茶鍐欏叆
- `PAGE_IS_FILE` - 椤电敱鏂囦欢鍚庡
- `PAGE_IS_PRESENT` - 椤靛瓨鍦ㄤ簬鍐呭瓨涓?- `PAGE_IS_SWAPPED` - 椤靛凡鎹㈠嚭
- `PAGE_IS_PFNZERO` - 椤电殑 PFN 涓洪浂
- `PAGE_IS_HUGE` - 椤电敱 PMD 鏄犲皠鐨?THP 鎴?Hugetlb 鍚庡
- `PAGE_IS_SOFT_DIRTY` - 椤典负杞剰
- `PAGE_IS_GUARD` - 椤垫槸淇濇姢鍖哄煙鐨勪竴閮ㄥ垎

`struct pm_scan_arg` 琚敤浣滆 IOCTL 鐨勫弬鏁般€?
 1. `struct pm_scan_arg` 鐨勫ぇ灏忓繀椤诲湪 `size` 瀛楁涓寚瀹氥€傚鏋滀互鍚庤繘琛屼簡鎵╁睍锛岃瀛楁
    鏈夊姪浜庤瘑鍒粨鏋勪綋銆?
 2. 鏍囧織鍙互鍦?`flags` 瀛楁涓寚瀹氥€傜洰鍓嶄粎娣诲姞浜?`PM_SCAN_WP_MATCHING` 鍜?    `PM_SCAN_CHECK_WPASYNC` 涓や釜鏍囧織銆傛槸鍚︽墽琛岃幏鍙栨搷浣滃彇鍐充簬鏄惁鎻愪緵浜嗚緭鍑虹紦鍐插尯銆?
 3. 鑼冨洿閫氳繃 `start` 鍜?`end` 鎸囧畾銆?
 4. 閬嶅巻鍙兘鍦ㄨ瀹屾暣鑼冨洿琚闂箣鍓嶄腑姝紝渚嬪鐢ㄦ埛缂撳啿鍖哄彲鑳藉凡婊＄瓑鎯呭喌銆傞亶鍘嗙粨鏉熷湴鍧€
    鍦?`end_walk` 涓寚瀹氥€?
 5. `struct page_region` 鏁扮粍鐨勮緭鍑虹紦鍐插尯鍙婂叾澶у皬鍦?`vec` 鍜?`vec_len` 涓寚瀹氥€?
 6. 鍙€夌殑鏈€澶ц姹傞〉鏁板湪 `max_pages` 涓寚瀹氥€?
 7. 鎺╃爜鍦?`category_mask`銆乣category_anyof_mask`銆乣category_inverted` 鍜?    `return_mask` 涓寚瀹氥€?
```

   struct pm_scan_arg arg = {
   .size = sizeof(arg),
   .flags = PM_SCAN_CHECK_WPASYNC | PM_SCAN_CHECK_WPASYNC,
   ..
   .category_mask = PAGE_IS_WRITTEN,
   .return_mask = PAGE_IS_WRITTEN,
   };

```
鏌ユ壘宸茶鍐欏叆銆佺敱鏂囦欢鍚庡銆佹湭鎹㈠嚭涓旓紙婊¤冻涓嬪垪浠讳竴鏉′欢锛夌殑椤碉細
```

   struct pm_scan_arg arg = {
   .size = sizeof(arg),
   .flags = 0,
   ..
   .category_mask = PAGE_IS_WRITTEN | PAGE_IS_SWAPPED,
   .category_inverted = PAGE_IS_SWAPPED,
   .category_anyof_mask = PAGE_IS_PRESENT | PAGE_IS_HUGE,
   .return_mask = PAGE_IS_WRITTEN | PAGE_IS_SWAPPED |
                  PAGE_IS_PRESENT | PAGE_IS_HUGE,
   };

```
`PAGE_IS_WRITTEN` 鏍囧織鍙瑙嗕负姣旇蒋鑴忔爣蹇楁€ц兘鏇村ソ鐨勬浛浠ｆ柟妗堛€傚畠涓嶅彈鍐呮牳 VMA 鍚堝苟
鐨勫奖鍝嶏紝鍥犳鍦ㄦ櫘閫氶〉鐨勬儏鍐典笅锛岀敤鎴峰彲浠ユ壘鍒扮湡姝ｇ殑杞剰椤点€傦紙瀵逛簬 THP 鎴?Hugetlb 椤?浠嶅彲鑳芥姤鍛婇澶栫殑鑴忛〉銆傦級

"PAGE_IS_WRITTEN" 绫诲埆涓庡惎鐢ㄤ簡 uffd 鍐欎繚鎶ょ殑鑼冨洿閰嶅悎浣跨敤锛屼互鍦ㄧ敤鎴风┖闂村疄鐜板唴瀛樿剰
璺熻釜锛?
 1. 閫氳繃 `userfaultfd` 绯荤粺璋冪敤鍒涘缓 userfaultfd 鏂囦欢鎻忚堪绗︺€?
 2. 閫氳繃 `UFFDIO_API` IOCTL 璁剧疆 `UFFD_FEATURE_WP_UNPOPULATED` 鍜?    `UFFD_FEATURE_WP_ASYNC` 鐗规€с€?
 3. 閫氳繃 `UFFDIO_REGISTER` IOCTL 浠?`UFFDIO_REGISTER_MODE_WP` 妯″紡娉ㄥ唽鍐呭瓨鑼冨洿銆?
 4. 鐒跺悗锛屽繀椤讳娇鐢ㄥ甫 `PM_SCAN_WP_MATCHING` 鏍囧織鐨?`PAGEMAP_SCAN` IOCTL锛屾垨鑰呭彲浠ヤ娇鐢?    `UFFDIO_WRITEPROTECT` IOCTL锛屾潵鍐欎繚鎶ゅ凡娉ㄥ唽鍐呭瓨鐨勪换浣曢儴鍒嗘垨鏁翠釜鍐呭瓨鍖哄煙銆備袱鑰呮墽琛?    鐩稿悓鐨勬搷浣滐紝鍓嶈€呭湪鎬ц兘涓婃洿浼樸€?
 5. 鐜板湪鍙互浣跨敤 `PAGEMAP_SCAN` IOCTL 鏉ヤ粎鏌ユ壘鑷笂娆℃爣璁颁互鏉ュ凡琚啓鍏ョ殑椤碉紝鍜?鎴栧彲閫夊湴
    鍚屾椂鍐欎繚鎶よ繖浜涢〉銆?
