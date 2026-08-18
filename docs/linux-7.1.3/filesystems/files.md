
## Linux 鍐呮牳涓殑鏂囦欢绠＄悊


鏈枃妗ｄ粙缁嶆枃浠讹紙`struct file`锛変笌鏂囦欢鎻忚堪绗﹁〃锛坄struct files`锛夌殑鍔犻攣鏈哄埗鏄浣曞伐浣滅殑銆?
鍦?2.6.12 涔嬪墠锛屾枃浠舵弿杩扮琛ㄧ敱涓€鎶婇攣锛坒iles->file_lock锛夊拰寮曠敤璁℃暟锛坒iles->count锛変繚鎶ゃ€?->file_lock 淇濇姢瀵硅〃涓墍鏈変笌鏂囦欢鐩稿叧瀛楁鐨勮闂€?>count 鐢ㄤ簬鍦ㄩ€氳繃 CLONE_FILES 鏍囧織鍏嬮殕鐨勪换鍔′箣闂村叡浜?鏂囦欢鎻忚堪绗﹁〃銆傚浜?POSIX 绾跨▼閫氬父灏辨槸杩欑鎯呭喌銆備笌鍐呮牳涓父瑙佺殑寮曠敤璁℃暟妯″瀷涓€鏍凤紝鏈€鍚庝竴涓墽琛?put_files_struct() 鐨勪换鍔￠噴鏀炬枃浠舵弿杩扮锛坒d锛夎〃銆傛枃浠讹紙`struct file`锛夋湰韬娇鐢ㄥ紩鐢ㄨ鏁帮紙->f_count锛変繚鎶ゃ€?
鍦ㄦ枃浠舵弿杩扮绠＄悊鏂扮殑鏃犻攣妯″瀷涓紝寮曠敤璁℃暟鏂瑰紡绫讳技锛屼絾鍔犻攣鍩轰簬 RCU銆傛枃浠舵弿杩扮琛ㄥ寘鍚涓厓绱犫€斺€?fd 闆嗗悎锛坥pen_fds 涓?close_on_exec锛夈€佹枃浠舵寚閽堟暟缁勩€侀泦鍚堜笌鏁扮粍鐨勫ぇ灏忕瓑銆備负浜嗕娇鏇存柊瀵规棤閿佽鍙栬€?鍛堢幇鍘熷瓙鎬э紝鏂囦欢鎻忚堪绗﹁〃鐨勬墍鏈夊厓绱犻兘鏀惧湪涓€涓嫭绔嬬殑缁撴瀯浣?struct fdtable 涓€俧iles_struct 鍖呭惈涓€涓?鎸囧悜 struct fdtable 鐨勬寚閽堬紝瀹為檯鐨?fd 琛ㄩ€氳繃璇ユ寚閽堣闂€傛渶鍒?fdtable 宓屽叆鍦?files_struct 鑷韩涓€?鍦ㄥ悗缁?fdtable 鎵╁睍鏃讹紝浼氬垎閰嶄竴涓柊鐨?fdtable 缁撴瀯锛宖iles->fdtab 鎸囧悜鏂扮粨鏋勩€俧dtable 缁撴瀯閫氳繃
RCU 閲婃斁锛屾棤閿佽鍙栬€呰涔堢湅鍒版棫鐨?fdtable锛岃涔堢湅鍒版柊鐨?fdtable锛屼粠鑰屼娇鏇存柊鍛堢幇鍘熷瓙鎬с€?浠ヤ笅鏄?fdtable 缁撴瀯鐨勫姞閿佽鍒欙細

1. 鎵€鏈夊 fdtable 鐨勫紩鐢ㄩ兘蹇呴』閫氳繃

```

	struct fdtable *fdt;

	rcu_read_lock();

	fdt = files_fdtable(files);
	....
	if (n <= fdt->max_fds)
		....
	...
	rcu_read_unlock();

   files_fdtable() 浣跨敤 rcu_dereference() 瀹忥紝璇ュ畯璐熻矗澶勭悊鏃犻攣瑙ｅ紩鐢ㄦ墍闇€鐨勫唴瀛樺睆闅滆姹傘€?   fdtable 鎸囬拡蹇呴』鍦ㄨ绔复鐣屽尯鍐呴儴璇诲彇銆?
```

2. 涓婅堪瀵?fdtable 鐨勮鍙栧繀椤荤敱 rcu_read_lock()/rcu_read_unlock() 淇濇姢銆?
3. 瀵逛簬浠讳綍瀵?fd 琛ㄧ殑鏇存柊锛屽繀椤绘寔鏈?files->file_lock銆?
4. 缁欏畾涓€涓?fd 鏌ユ壘 file 缁撴瀯鏃讹紝璇诲彇鑰呭繀椤讳娇鐢?lookup_fdget_rcu() 鎴?files_lookup_fdget_rcu() API銆?   瀹冧滑璐熻矗澶勭悊鍥犳棤閿佹煡鎵捐€屼骇鐢熺殑灞忛殰瑕佹眰銆?
```

	struct file *file;

	rcu_read_lock();
	file = lookup_fdget_rcu(fd);
	rcu_read_unlock();
	if (file) {
		...
                fput(file);
	}
	....

```

5. 鐢变簬 fdtable 涓?file 缁撴瀯閮藉彲浠ユ棤閿佹煡鎵撅紝瀹冧滑蹇呴』浣跨敤 rcu_assign_pointer() API 瀹夎銆?   濡傛灉瀹冧滑琚棤閿佹煡鎵撅紝鍒欏繀椤讳娇鐢?rcu_dereference()銆備笉杩囧缓璁娇鐢?files_fdtable() 浠ュ強
   lookup_fdget_rcu()/files_lookup_fdget_rcu()锛屽畠浠細澶勭悊杩欎簺闂銆?
6. 鍦ㄦ洿鏂版椂锛宖dtable 鎸囬拡蹇呴』鍦ㄦ寔鏈?files->file_lock 鐨勬儏鍐典笅鏌ユ壘銆傚鏋滈噴鏀句簡 ->file_lock锛屽垯
   鍙︿竴涓嚎绋嬪彲鑳芥墿灞?files锛屼粠鑰屽垱寤轰竴涓柊鐨?fdtable 骞朵娇鍏堝墠鐨?fdtable 鎸囬拡澶辨晥銆?
```

	spin_lock(&files->file_lock);
	fd = locate_fd(files, file, start);
	if (fd >= 0) {
		/* locate_fd() 鍙兘宸叉墿灞?fdtable锛屽姞杞借鎸囬拡 */
		fdt = files_fdtable(files);
		__set_open_fd(fd, fdt);
		__clear_close_on_exec(fd, fdt);
		spin_unlock(&files->file_lock);
	.....

   鐢变簬 locate_fd() 鍙兘閲婃斁 ->file_lock锛堝苟閲嶆柊鑾峰彇 ->file_lock锛夛紝fdtable 鎸囬拡锛坒dt锛夊繀椤诲湪
   locate_fd() 涔嬪悗鍔犺浇銆?
```
鍦ㄨ緝鏂扮殑鍐呮牳涓紝鍩轰簬 RCU 鐨勬枃浠舵煡鎵惧凡鍒囨崲涓轰緷璧?SLAB_TYPESAFE_BY_RCU 鑰岄潪 call_rcu()銆備粎浠呭湪 RCU 涓?浣跨敤 atomic_long_inc_not_zero() 鑾峰彇鐩稿叧鏂囦欢鐨勫紩鐢ㄥ凡缁忎笉澶燂紝鍥犱负璇ユ枃浠跺彲鑳藉凡缁忚鍥炴敹锛岃€屽叾浠栦汉鍙兘宸茬粡
澧炲姞浜嗗紩鐢ㄨ鏁般€傛崲鍙ヨ瘽璇达紝璋冪敤鑰呭彲鑳界湅鍒版潵鑷緝鏂扮敤鎴风殑寮曠敤璁℃暟澧炲姞銆傚嚭浜庤繖涓師鍥狅紝鏈夊繀瑕佸湪寮曠敤璁℃暟
澧炲姞鍓嶅悗楠岃瘉鎸囬拡鏄浉鍚岀殑銆傝繖涓€妯″紡鍙浜?get_file_rcu() 涓?__files_get_rcu()銆?
姝ゅ锛屽湪 RCU 鏌ユ壘涓嬶紝鑻ユ湭鍏堝湪鏂囦欢涓婅幏鍙栧紩鐢紝灏辨棤娉曡闂垨妫€鏌?struct file 涓殑瀛楁銆備笉杩欐牱鍋氫竴鐩撮潪甯?涓嶅彲闈狅紝骞朵笖瀹冨彧閫傜敤浜?struct file 涓殑闈炴寚閽堟暟鎹€傛湁浜?SLAB_TYPESAFE_BY_RCU锛岃皟鐢ㄨ€呮湁蹇呰瑕佷箞鍏?鑾峰彇涓€涓紩鐢紝瑕佷箞蹇呴』鎸佹湁 fdtable 鐨?files_lock銆?