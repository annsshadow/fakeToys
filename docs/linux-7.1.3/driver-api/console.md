## 鎺у埗鍙伴┍鍔?

Linux 鍐呮牳鏈変袱绫婚€氱敤鐨勬帶鍒跺彴椹卞姩銆傜涓€绫荤敱鍐呮牳鍦ㄥ惎鍔ㄨ繃绋嬩腑鍒嗛厤缁欐墍鏈夎櫄鎷熸帶鍒跺彴銆傝繖绫婚┍鍔ㄨ绉颁负鈥滅郴缁熼┍鍔紙system driver锛夆€濓紝骞朵笖鍙厑璁稿瓨鍦ㄤ竴涓郴缁熼┍鍔ㄣ€傜郴缁熼┍鍔ㄦ槸甯搁┗鐨勶紝瀹冩案杩滀笉浼氳鍗歌浇锛屽敖绠″畠鍙兘鍙樹负闈炴椿鍔ㄧ姸鎬併€?

绗簩绫诲繀椤昏鏄惧紡鍦板姞杞藉拰鍗歌浇銆傛湰鏂囧皢绉板叾涓衡€滄ā鍧楀寲椹卞姩锛坢odular driver锛夆€濄€備换鎰忔椂鍒婚兘鍙互鏈夊涓ā鍧楀寲椹卞姩鍏卞瓨锛屾瘡涓┍鍔ㄩ兘涓庡叾浠栭┍鍔紙鍖呮嫭绯荤粺椹卞姩锛夊叡浜帶鍒跺彴銆備笉杩囷紝妯″潡鍖栭┍鍔ㄦ棤娉曟帴绠″綋鍓嶆琚彟涓€涓ā鍧楀寲椹卞姩鍗犵敤鐨勬帶鍒跺彴銆傦紙渚嬪锛氳皟鐢ㄤ簡 do_take_over_console() 鐨勯┍鍔紝鏃犺鍗犵敤鎺у埗鍙扮殑鏄摢绫婚┍鍔紝閮借兘鎴愬姛瀹屾垚鎺ョ銆傦級瀹冧滑鍙兘鎺ョ琚郴缁熼┍鍔ㄥ崰鐢ㄧ殑鎺у埗鍙般€傚悓鐞嗭紝濡傛灉妯″潡鍖栭┍鍔ㄨ鎺у埗鍙伴噴鏀撅紝绯荤粺椹卞姩灏变細鎺ョ鍥炴潵銆?

```

	 do_take_over_console() - load and bind driver to console layer
	 give_up_console() - unload driver; it will only work if driver
			     is fully unbound

```
```

	 do_register_con_driver()
	 do_unregister_con_driver()

```
濡傛灉鍚敤浜?sysfs锛屽彲浠ユ鏌?/sys/class/vtconsole 鐨勫唴瀹广€傚畠灞曠ず浜嗙郴缁熷綋鍓嶆敞鍐岀殑鎺у埗鍙板悗绔紝鍏跺懡鍚嶄负 vtcon<n>锛屽叾涓?<n> 鏄粠 0 鍒?15 鐨勬暣鏁般€?
```

       ls /sys/class/vtconsole
       .  ..  vtcon0  vtcon1

```
```

     ls /sys/class/vtconsole/vtcon0
     .  ..  bind  name  uevent

```
杩欎簺鏂囦欢浠ｈ〃浠€涔堬紵

     1. bind - 杩欐槸涓€涓彲璇?鍐欐枃浠躲€傝鍙栨椂瀹冩樉绀洪┍鍔ㄧ殑鐘舵€侊紱鍐欏叆鏃跺畠鐢ㄤ簬灏嗚椹卞姩缁戝畾鎴栬В闄ょ粦瀹氬埌铏氭嫙鎺у埗鍙般€傚彲鑳界殑鍊间负锛?

	0
   - 琛ㄧず椹卞姩鏈粦瀹氾紝濡傛灉鍚戝叾 echo锛屽垯鍛戒护椹卞姩瑙ｉ櫎缁戝畾

        1
   - 琛ㄧず椹卞姩宸茬粦瀹氾紝濡傛灉鍚戝叾 echo锛屽垯鍛戒护椹卞姩缁戝畾

```

	  cat /sys/class/vtconsole/vtcon0/name
	  (S) VGA+

	      '(S)' stands for a (S)ystem driver, i.e., it cannot be directly
	      commanded to bind or unbind

	      'VGA+' is the name of the driver

	  cat /sys/class/vtconsole/vtcon1/name
	  (M) frame buffer device

	      In this case, '(M)' stands for a (M)odular driver, one that can be
	      directly commanded to bind or unbind.

     3. uevent - ignore this file

```
瑙ｉ櫎缁戝畾鏃讹紝妯″潡鍖栭┍鍔ㄩ鍏堣鍒嗙锛岀劧鍚庣郴缁熼┍鍔ㄦ帴绠¤椹卞姩鑵惧嚭鐨勬帶鍒跺彴銆傚彟涓€鏂归潰锛岀粦瀹氭椂锛屼細鎶婅椹卞姩缁戝畾鍒板綋鍓嶇敱绯荤粺椹卞姩鍗犵敤鐨勬帶鍒跺彴銆?

娉ㄦ剰 1锛?
```

    Device Drivers ->
	Character devices ->
		Support for binding and unbinding console drivers

```
娉ㄦ剰 2锛氬鏋滀换鎰忚櫄鎷熸帶鍒跺彴澶勪簬 KD_GRAPHICS 妯″紡锛岄偅涔堢粦瀹氭垨瑙ｉ櫎缁戝畾閮戒笉浼氭垚鍔熴€備竴涓細鎶婃帶鍒跺彴璁句负 KD_GRAPHICS 鐨勪緥瀛愭槸 X銆?

杩欎釜鍔熻兘鏈夊澶х敤澶勶紵瀹冨鎺у埗鍙伴┍鍔ㄥ紑鍙戣€呴潪甯告湁鐢ㄣ€傞€氳繃鎶婇┍鍔ㄤ粠鎺у埗鍙板眰瑙ｉ櫎缁戝畾锛屽彲浠ュ嵏杞介┍鍔ㄣ€佸仛鍑轰慨鏀广€侀噸鏂扮紪璇戙€侀噸鏂板姞杞藉苟閲嶆柊缁戝畾椹卞姩锛岃€屾棤闇€閲嶅惎鍐呮牳銆傚浜庢兂瑕佸湪甯х紦鍐叉帶鍒跺彴涓?VGA 鎺у埗鍙颁箣闂达紙鎴栧弽杩囨潵锛夊垏鎹㈢殑鏅€氱敤鎴凤紝杩欎釜鍔熻兘涔熶娇涔嬫垚涓哄彲鑳姐€傦紙娉ㄦ剰 娉ㄦ剰 娉ㄦ剰锛氭洿澶氱粏鑺傝闃呰 Documentation/fb 涓嬬殑 fbcon.txt銆傦級

## 缁欏紑鍙戣€呯殑璇存槑

```

     do_register_con_driver()
     do_bind_con_driver() - private function

```
give_up_console() 鏄?do_unregister_con_driver() 鐨勫皝瑁咃紝骞朵笖鍙湁褰撻┍鍔ㄨ瀹屽叏瑙ｉ櫎缁戝畾鏃惰繖涓皟鐢ㄦ墠浼氭垚鍔熴€俢on_is_bound() 浼氭鏌ラ┍鍔ㄦ槸鍚﹀凡缁戝畾銆?

## 鎺у埗鍙伴┍鍔ㄧ紪鍐欒€呮寚鍗?

涓轰簡璁╃粦瀹氬拰瑙ｉ櫎缁戝畾鍒版帶鍒跺彴鑳藉姝ｅ父宸ヤ綔锛屾帶鍒跺彴椹卞姩蹇呴』閬靛惊浠ヤ笅鍑嗗垯锛?

1. 闄ょ郴缁熼┍鍔ㄥ锛屾墍鏈夐┍鍔ㄩ兘蹇呴』璋冪敤 do_register_con_driver() 鎴?do_take_over_console()銆俤o_register_con_driver() 鍙槸鎶婇┍鍔ㄥ姞鍏ユ帶鍒跺彴鐨勫唴閮ㄥ垪琛紝瀹冧笉浼氭帴绠℃帶鍒跺彴銆傝€岄【鍚嶆€濅箟锛宒o_take_over_console() 杩樹細鎺ョ锛堟垨缁戝畾鍒帮級鎺у埗鍙般€?

2. 鍦?con->con_init() 鏈熼棿鍒嗛厤鐨勬墍鏈夎祫婧愬繀椤诲湪 con->con_deinit() 涓噴鏀俱€?

3. 鍦?con->con_startup() 涓垎閰嶇殑鎵€鏈夎祫婧愶紝蹇呴』鍦ㄤ箣鍓嶅凡缁戝畾鐨勯┍鍔ㄨ瑙ｉ櫎缁戝畾鏃堕噴鏀俱€傛帶鍒跺彴灞傛病鏈変笌 con->con_startup() 鐩稿搴旂殑璋冪敤锛屽洜姝や綍鏃跺彲浠ュ悎娉曢噴鏀捐繖浜涜祫婧愮敱椹卞姩鑷繁鍒ゆ柇銆傚湪 con->con_deinit() 涓皟鐢?con_is_bound() 浼氭湁鎵€甯姪銆傚鏋滆璋冪敤杩斿洖 false()锛岄偅涔堥噴鏀捐繖浜涜祫婧愭槸瀹夊叏鐨勩€傚繀椤荤‘淇濊繖绉嶅钩琛★紝鍥犱负褰撲竴涓噸鏂扮粦瀹氳椹卞姩鍒版帶鍒跺彴鐨勮姹傚埌鏉ユ椂锛宑on->con_startup() 鍙兘浼氬啀娆¤璋冪敤銆?

4. 鍦ㄩ┍鍔ㄩ€€鍑烘椂锛岀‘淇濊椹卞姩宸茶瀹屽叏瑙ｉ櫎缁戝畾銆傚鏋滄潯浠舵弧瓒筹紝閭ｄ箞椹卞姩蹇呴』璋冪敤 do_unregister_con_driver() 鎴?give_up_console()銆?

5. do_unregister_con_driver() 涔熷彲浠ュ湪椹卞姩鏃犳硶鍐嶄负鎺у埗鍙拌姹傛彁渚涙湇鍔＄殑鎯呭喌涓嬭璋冪敤銆備娇鐢ㄥ抚缂撳啿鎺у埗鍙版椂鍙兘鍙戠敓杩欑鎯呭喌鈥斺€斿畠绐佺劧澶卞幓浜嗘墍鏈夌殑椹卞姩銆?

褰撳墠鐨勮繖鎵规帶鍒跺彴椹卞姩搴斿綋浠嶈兘姝ｅ父宸ヤ綔锛屼絾缁戝畾鍜岃В闄ょ粦瀹氬畠浠彲鑳戒細鏈夐棶棰樸€傚彧闇€鍋氭渶灏忕殑淇锛屽氨鑳借杩欎簺椹卞姩姝ｅ父宸ヤ綔銆?

Antonino Daplas <adaplas@pol.net>
