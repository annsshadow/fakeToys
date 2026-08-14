## DM 缁熻


Device Mapper 鏀寔鍦?DM 璁惧鐢ㄦ埛瀹氫箟鐨勫尯鍩熶笂鏀堕泦 I/O 缁熻銆傚鏋滄病鏈夊畾涔変换浣曞尯鍩燂紝
灏变笉浼氭敹闆嗕换浣曠粺璁★紝鍥犳涓嶄細甯︽潵浠讳綍鎬ц兘褰卞搷銆傜洰鍓嶄粎鏀寔鍩轰簬 bio 鐨?DM 璁惧銆?
姣忎釜鐢ㄦ埛瀹氫箟鐨勫尯鍩熸寚瀹氫竴涓捣濮嬫墖鍖恒€侀暱搴﹀拰姝ラ暱銆傚皢涓烘寚瀹氳寖鍥村唴姣忎釜姝ラ暱澶у皬鐨?鍖哄煙鍒嗗埆鏀堕泦缁熻銆?
鍖哄煙鍐呮瘡涓闀垮ぇ灏忓尯鍩熺殑 I/O 缁熻璁℃暟鍣ㄦ牸寮忎笌 `/sys/block/*/stat` 鎴?`/proc/diskstats` 鐩稿悓锛堝弬瑙?Documentation/admin-guide/iostats.rst锛夈€備絾杩樻彁渚涗簡
涓や釜棰濆鐨勮鏁板櫒锛?2 鍜?13锛夛細璇诲彇鍜屽啓鍏ユ墍鑺辫垂鐨勬€绘椂闂淬€傚綋浣跨敤浜?histogram 鍙傛暟鏃讹紝
浼氭姤鍛婄 14 涓弬鏁帮紝瀹冭〃绀哄欢杩熺殑鐩存柟鍥俱€傛墍鏈夎繖浜涜鏁板櫒閮藉彲浠ラ€氳繃鍚戠浉搴旂殑 DM 璁惧
鍙戦€?@stats_print 娑堟伅锛堢粡鐢?dmsetup锛夋潵璁块棶銆?
鎶ュ憡鐨勬椂闂翠互姣涓哄崟浣嶏紝绮掑害鍙栧喅浜庡唴鏍告椂閽熸淮绛斻€傚綋浣跨敤 precise_timestamps 閫夐」鏃讹紝
鎶ュ憡鐨勬椂闂翠互绾崇涓哄崟浣嶃€?
姣忎釜鍖哄煙閮芥湁涓€涓搴旂殑鍞竴鏍囪瘑绗︼紝鎴戜滑绉颁箣涓?region_id锛屽畠鍦ㄥ尯鍩熷垱寤烘椂鍒嗛厤銆?鍦ㄦ煡璇㈣鍖哄煙鐨勭粺璁°€佸垹闄よ鍖哄煙绛夋椂锛屽繀椤绘彁渚?region_id銆傚敮涓€鐨?region_id 浣垮緱澶氫釜
鐢ㄦ埛绌洪棿绋嬪簭鑳藉璇锋眰骞跺鐞嗗悓涓€涓?DM 璁惧鐨勭粺璁★紝鑰屼笉浼氫簰鐩歌俯鍒板鏂圭殑鏁版嵁銆?
DM 缁熻鐨勫垱寤哄皢閫氳繃 kmalloc 鍒嗛厤鍐呭瓨锛屾垨鍥為€€鍒颁娇鐢?vmalloc 绌洪棿銆侱M 缁熻鏈€澶氬彲鑳?鍒嗛厤绯荤粺鎬诲唴瀛樼殑 1/4銆傜鐞嗗憳鍙互閫氳繃璇诲彇浠ヤ笅鍐呭鏌ョ湅浣跨敤浜嗗灏戝唴瀛橈細

	/sys/module/dm_mod/parameters/stats_current_allocated_bytes

## 娑堟伅


    @stats_create <range> <step> [<number_of_optional_arguments> <optional_arguments>...] [<program_id> [<aux_data>]]
	鍒涘缓涓€涓柊鍖哄煙骞惰繑鍥?region_id銆?
	<range>
	  "-"
		鏁翠釜璁惧
	  "<start_sector>+<length>"
		涓€娈甸暱搴︿负 <length> 鐨?512 瀛楄妭鎵囧尯锛?		浠?<start_sector> 寮€濮嬨€?
	<step>
	  "<area_size>"
		璇ヨ寖鍥磋缁嗗垎涓哄涓尯鍩燂紝姣忎釜鍖哄煙鍖呭惈
		<area_size> 涓墖鍖恒€?	  "/<number_of_areas>"
		璇ヨ寖鍥磋缁嗗垎涓烘寚瀹氭暟閲忕殑
		鍖哄煙銆?
	<number_of_optional_arguments>
	  鍙€夊弬鏁扮殑鏁伴噺

	<optional_arguments>
	  鏀寔浠ヤ笅鍙€夊弬鏁帮細

	  precise_timestamps
		浣跨敤鍏锋湁绾崇鍒嗚鲸鐜囩殑绮剧‘璁℃椂鍣紝
		鑰岄潪 "jiffies" 鍙橀噺銆備娇鐢ㄦ鍙傛暟鏃讹紝
		缁撴灉鏃堕棿浠ョ撼绉掕€岄潪姣涓哄崟浣嶃€傜簿纭椂闂存埑
		姣斿熀浜?jiffies 鐨勬椂闂存埑鑾峰彇璧锋潵绋嶆參涓€浜涖€?	  histogram:n1,n2,n3,n4,...
		鏀堕泦寤惰繜鐨勭洿鏂瑰浘銆傛暟瀛?		n1銆乶2 绛夋槸浠ｈ〃鐩存柟鍥捐竟鐣岀殑鏃堕棿銆傚鏋滄湭浣跨敤
		precise_timestamps锛屾椂闂翠互姣涓哄崟浣嶏紝鍚﹀垯浠?		绾崇涓哄崟浣嶃€傚浜庢瘡涓寖鍥达紝鍐呮牳灏嗘姤鍛婂湪璇ヨ寖鍥村唴
		瀹屾垚鐨勮姹傛暟閲忋€備緥濡傦紝濡傛灉鎴戜滑浣跨敤
		"histogram:10,20,30"锛屽唴鏍稿皢鎶ュ憡鍥涗釜鏁板瓧
		a:b:c:d銆俛 鏄€楁椂 0-10 姣瀹屾垚鐨勮姹傛暟锛宐 鏄?		鑰楁椂 10-20 姣瀹屾垚鐨勮姹傛暟锛宑 鏄€楁椂 20-30 姣
		瀹屾垚鐨勮姹傛暟锛宒 鏄€楁椂瓒呰繃 30 姣瀹屾垚鐨勮姹傛暟銆?
	<program_id>
	  涓€涓彲閫夊弬鏁般€備竴涓敮涓€鏍囪瘑璇ヨ寖鍥寸敤鎴风┖闂存嫢鏈夎€呯殑鍚嶇О銆?	  杩欏皢鑼冨洿鍒嗙粍鍦ㄤ竴璧凤紝浠ヤ究鐢ㄦ埛绌洪棿绋嬪簭鑳藉璇嗗埆瀹冧滑
	  鍒涘缓鐨勮寖鍥达紝骞跺拷鐣ョ敱浠栦汉鍒涘缓鐨勮寖鍥淬€傚唴鏍稿湪
	  @stats_list 娑堟伅鐨勮緭鍑轰腑灏嗚瀛楃涓茶繑鍥烇紝浣嗕笉灏嗗叾鐢ㄤ簬
	  浠讳綍鍏跺畠鐢ㄩ€斻€傚鏋滄垜浠渷鐣ュ彲閫夊弬鏁扮殑鏁伴噺锛宲rogram id 涓嶈兘
	  鏄竴涓暟瀛楋紝鍚﹀垯瀹冧細琚В閲婁负鍙€夊弬鏁扮殑鏁伴噺銆?
	<aux_data>
	  涓€涓彲閫夊弬鏁般€備竴涓彁渚涜緟鍔╂暟鎹殑璇嶏紝瀵瑰垱寤鸿鑼冨洿鐨?	  瀹㈡埛绔▼搴忔湁鐢ㄣ€傚唴鏍稿湪 @stats_list 娑堟伅鐨勮緭鍑轰腑灏嗚瀛楃涓?	  杩斿洖锛屼絾涓嶄娇鐢ㄨ鍊煎仛浠讳綍浜嬫儏銆?
    @stats_delete <region_id>
	鍒犻櫎鍏锋湁鎸囧畾 id 鐨勫尯鍩熴€?
	<region_id>
	  浠?@stats_create 杩斿洖鐨?region_id

    @stats_clear <region_id>
	娓呴櫎闄よ繘琛屼腑 I/O 璁℃暟鍣ㄤ箣澶栫殑鎵€鏈夎鏁板櫒銆?
	<region_id>
	  浠?@stats_create 杩斿洖鐨?region_id

    @stats_list [<program_id>]
	鍒楀嚭鎵€鏈夌敤 @stats_create 娉ㄥ唽鐨勫尯鍩熴€?
	<program_id>
	  涓€涓彲閫夊弬鏁般€?	  濡傛灉鎸囧畾浜嗚鍙傛暟锛屽彧杩斿洖鍖归厤鐨勫尯鍩熴€?	  濡傛灉鏈寚瀹氾紝鍒欒繑鍥炴墍鏈夊尯鍩熴€?
	杈撳嚭鏍煎紡锛?	  <region_id>: <start_sector>+<length> <step> <program_id> <aux_data>
	        precise_timestamps histogram:n1,n2,n3,...

	瀛楃涓?"precise_timestamps" 鍜?"histogram" 浠呭湪鍒涘缓鍖哄煙鏃?	鎸囧畾浜嗗畠浠殑鎯呭喌涓嬫墠浼氳鎵撳嵃銆?
    @stats_print <region_id> [<starting_line> <number_of_lines>]
	鎵撳嵃涓€涓尯鍩熶腑姣忎釜姝ラ暱澶у皬鍖哄煙鐨勮鏁板櫒銆?
	<region_id>
	  浠?@stats_create 杩斿洖鐨?region_id

	<starting_line>
	  杈撳嚭涓捣濮嬭鐨勭储寮曘€?	  濡傛灉鐪佺暐锛屽垯杩斿洖鎵€鏈夎銆?
	<number_of_lines>
	  杈撳嚭涓鍖呭惈鐨勮鏁般€?	  濡傛灉鐪佺暐锛屽垯杩斿洖鎵€鏈夎銆?
	鍖哄煙涓瘡涓闀垮ぇ灏忓尯鍩熺殑杈撳嚭鏍煎紡锛?
	  <start_sector>+<length>
		counters

	  鍓?11 涓鏁板櫒涓?`/sys/block/*/stat 鎴?/proc/diskstats`
	  鍚箟鐩稿悓銆?
	  璇︽儏璇峰弬闃?Documentation/admin-guide/iostats.rst銆?
   1. 宸插畬鎴愮殑璇诲彇娆℃暟
   2. 宸插悎骞剁殑璇诲彇娆℃暟
   3. 璇诲彇鐨勬墖鍖烘暟
   4. 璇诲彇鎵€鑺辫垂鐨勬绉掓暟
   5. 宸插畬鎴愮殑鍐欏叆娆℃暟
   6. 宸插悎骞剁殑鍐欏叆娆℃暟
   7. 鍐欏叆鐨勬墖鍖烘暟
   8. 鍐欏叆鎵€鑺辫垂鐨勬绉掓暟
   9. 褰撳墠杩涜涓殑 I/O 鏁伴噺
   10. 鎵ц I/O 鎵€鑺辫垂鐨勬绉掓暟
   11. 鎵ц I/O 鎵€鑺辫垂鐨勫姞鏉冩绉掓暟

	  棰濆鐨勮鏁板櫒锛?
   12. 璇诲彇鎵€鑺辫垂鐨勬€绘椂闂达紙姣锛?   13. 鍐欏叆鎵€鑺辫垂鐨勬€绘椂闂达紙姣锛?
    @stats_print_clear <region_id> [<starting_line> <number_of_lines>]
	鍘熷瓙鍦版墦鍗扮劧鍚庢竻闄ら櫎杩涜涓?I/O 璁℃暟鍣ㄤ箣澶栫殑鎵€鏈夎鏁板櫒銆傚綋娑堣垂
	缁熻鐨勫鎴风涓嶆兂涓㈠け浠讳綍缁熻锛堥偅浜涘湪鎵撳嵃鍜屾竻闄や箣闂磋鏇存柊鐨勶級
	鏃跺緢鏈夌敤銆?
	<region_id>
	  浠?@stats_create 杩斿洖鐨?region_id

	<starting_line>
	  杈撳嚭涓捣濮嬭鐨勭储寮曘€?	  濡傛灉鐪佺暐锛屽垯鎵撳嵃骞舵竻闄ゆ墍鏈夎銆?
	<number_of_lines>
	  瑕佸鐞嗙殑琛屾暟銆?	  濡傛灉鐪佺暐锛屽垯鎵撳嵃骞舵竻闄ゆ墍鏈夎銆?
    @stats_set_aux <region_id> <aux_data>
	涓烘寚瀹氬尯鍩熷瓨鍌ㄨ緟鍔╂暟鎹?aux_data銆?
	<region_id>
	  浠?@stats_create 杩斿洖鐨?region_id

	<aux_data>
	  鏍囪瘑瀵瑰垱寤鸿鑼冨洿鐨勫鎴风绋嬪簭鏈夌敤鐨勬暟鎹殑瀛楃涓层€傚唴鏍稿湪
	  @stats_list 娑堟伅鐨勮緭鍑轰腑灏嗚瀛楃涓茶繑鍥烇紝浣嗕笉灏嗗叾鐢ㄤ簬浠讳綍
	  鐢ㄩ€斻€?
## 绀轰緥


灏?DM 璁惧 'vol' 缁嗗垎涓?100 鍧楋紝骞跺紑濮嬫敹闆?```

  dmsetup message vol 0 @stats_create - /100

```
灏嗚緟鍔╂暟鎹瓧绗︿覆璁句负 "foo bar baz"锛堟瘡涓?```

  dmsetup message vol 0 @stats_set_aux 0 foo\\ bar\\ baz

```
```

  dmsetup message vol 0 @stats_list

```
```

  dmsetup message vol 0 @stats_print 0

```
```

  dmsetup message vol 0 @stats_delete 0

```
