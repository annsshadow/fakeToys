## VGA 杞厜鏍?
by Pavel Machek <pavel@atrey.karlin.mff.cuni.cz>
and Martin Mares <mj@atrey.karlin.mff.cuni.cz>

Linux 鐜板湪鍏峰涓€瀹氭搷绾靛厜鏍囧瑙傜殑鑳藉姏銆傞€氬父锛屼綘鍙互璁剧疆纭欢鍏夋爣鐨勫ぇ灏忋€傜幇鍦ㄤ綘鍙互鐜╁嚑涓柊鑺辨牱锛氬彲浠ヨ鍏夋爣鐪嬭捣鏉ュ儚涓€涓笉闂儊鐨勭孩鑹叉柟鍧楋紝璁╁畠鍙嶆樉鎵€鍦ㄥ瓧绗︾殑鑳屾櫙锛屾垨鑰呴珮浜瀛楃锛屽苟浠嶅彲閫夋嫨鍘熸潵鐨勭‖浠跺厜鏍囨槸鍚︿繚鎸佸彲瑙併€備篃璁歌繕鏈夊叾浠栨垜浠庢湭鎯冲埌鐨勭帺娉曘€?
鍏夋爣澶栬鐢?`<ESC>[?1;2;3c` 杞箟搴忓垪鎺у埗锛屽叾涓?1銆?銆? 鏄涓嬫墍杩扮殑鍙傛暟銆傝嫢鐪佺暐鍏朵腑浠讳綍涓€涓紝瀹冧滑灏嗛粯璁や负闆躲€?
绗竴涓弬鏁?```

		0=default
		1=invisible
		2=underline,
		...
		8=full block
		+ 16 if you want the software cursor to be applied
		+ 32 if you want to always change the background color
		+ 64 if you dislike having the background the same as the
		     foreground.

	Highlights are ignored for the last two flags.

```
绗簩涓弬鏁?	閫夋嫨浣犳兂瑕佹洿鏀圭殑瀛楃灞炴€т綅
	锛堝彧闇€鐢ㄦ湰鍙傛暟鐨勫€煎鍏惰繘琛屽紓鎴栧嵆鍙級銆傚湪鏍囧噯
	VGA 涓婏紝楂樺洓浣嶆寚瀹氳儗鏅壊锛屼綆鍥涗綅鎸囧畾
	鍓嶆櫙鑹层€傚湪涓ょ粍涓紝浣庝笁浣嶈缃鑹诧紙涓庢帶鍒跺彴浣跨敤鐨勬櫘閫?	棰滆壊鐮佺浉鍚岋級锛屾渶楂樹綅寮€鍚珮浜紙鏈夋椂鏄棯鐑佲€斺€旇繖鍙栧喅浜庝綘鐨?	VGA 鐨勯厤缃級銆?
绗笁涓弬鏁?	鐢变綘鎯宠璁剧疆鐨勫瓧绗﹀睘鎬т綅缁勬垚銆?
	浣嶇殑璁剧疆鍙戠敓鍦ㄤ綅缈昏浆涔嬪墠锛屽洜姝や綘鍙互绠€鍗曞湴閫氳繃灏嗘煇涓€浣嶇疆浜?	璁剧疆鎺╃爜涓庣炕杞帺鐮佷簩鑰呬箣涓潵娓呴櫎璇ヤ綅銆?
### 绀轰緥


```

	echo -e '\033[?2c'

```
```

	echo -e '\033[?6c'

```
```

	echo -e '\033[?17;0;64c'

```
