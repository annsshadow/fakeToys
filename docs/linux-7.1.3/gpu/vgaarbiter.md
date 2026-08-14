# VGA 浠茶鍣?


鍥惧舰璁惧閫氳繃 I/O 鎴栧唴瀛樼┖闂翠腑鐨勫湴鍧€鑼冨洿杩涜璁块棶銆傝櫧鐒跺ぇ澶氭暟鐜颁唬璁惧鍏佽閲嶅畾浣嶈繖浜涜寖鍥达紝浣嗕竴浜涘湪 PCI 涓婂疄鐜扮殑鈥滀紶缁燂紙Legacy锛夆€漋GA 璁惧閫氬父鍏锋湁涓庡湪 ISA 涓婄浉鍚岀殑鈥滅‖瑙ｇ爜锛坔ard-decoded锛夆€濆湴鍧€銆傛洿澶氱粏鑺傝鍙傝鈥淧CI Bus Binding to IEEE Std 1275-1994 Standard for Boot (Initialization Configuration) Firmware Revision 2.1鈥濈 7 鑺傗€淟egacy Devices鈥濄€?

褰撳悓涓€鍙版満鍣ㄤ笂鍏卞瓨澶氫釜浼犵粺璁惧鏃讹紝X 鏈嶅姟鍣?[^0^] 鍐呴儴鐨勮祫婧愯闂帶鍒讹紙RAC锛夋ā鍧楋紙闄ゅ叾浠栨€荤嚎绠＄悊浠诲姟澶栵級璐熻矗浼犵粺鐨?VGA 浠茶浠诲姟銆備絾褰撹繖浜涜澶囪瘯鍥捐涓嶅悓鐨勭敤鎴风┖闂村鎴风锛堜緥濡備袱涓苟琛岃繍琛岀殑鏈嶅姟鍣級璁块棶鏃讹紝闂灏卞嚭鐜颁簡锛氬畠浠殑鍦板潃鍒嗛厤浼氬彂鐢熷啿绐併€傛澶栵紝鐞嗘兂鎯呭喌涓嬶紝浣滀负鐢ㄦ埛绌洪棿搴旂敤绋嬪簭锛屾帶鍒舵€荤嚎璧勬簮骞朵笉灞炰簬 X 鏈嶅姟鍣ㄧ殑鑱岃矗銆傚洜姝わ紝闇€瑕佸湪 X 鏈嶅姟鍣ㄤ箣澶栨湁涓€涓徊瑁佹柟妗堟潵鎺у埗杩欎簺璧勬簮鐨勫叡浜€傛湰鏂囨。浠嬬粛浜嗕负 Linux 鍐呮牳瀹炵幇鐨?VGA 浠茶鍣ㄧ殑杩愪綔鏂瑰紡銆?

## vgaarb 鍐呮牳/鐢ㄦ埛绌洪棿 ABI


vgaarb 鏄?Linux 鍐呮牳鐨勪竴涓ā鍧椼€傚畠鍦ㄥ垵濮嬪姞杞芥椂浼氭壂鎻忔墍鏈?PCI 璁惧锛屽苟灏嗗叾涓殑 VGA 璁惧鍔犲叆浠茶銆傞殢鍚庯紝浠茶鍣ㄤ細鍦ㄤ笉鍚岃澶囩殑 VGA 浼犵粺鎸囦护涓婂惎鐢?绂佺敤瑙ｇ爜銆傞偅浜涗笉鎯?涓嶉渶瑕佷娇鐢ㄤ徊瑁佸櫒鐨勮澶囧彲浠ラ€氳繃璋冪敤 vga_set_legacy_decoding() 鏄惧紡鍛婄煡瀹冦€?

鍐呮牳鍚戝鎴风瀵煎嚭浜嗕竴涓瓧绗﹁澶囨帴鍙ｏ紙/dev/vga_arbiter锛夛紝鍏惰涔夊涓嬶細

open
        鎵撳紑浠茶鍣ㄧ殑涓€涓敤鎴峰疄渚嬨€傞粯璁ゆ儏鍐典笅锛屽畠闄勫姞鍒扮郴缁熺殑榛樿 VGA 璁惧銆?

close
        鍏抽棴涓€涓敤鎴峰疄渚嬨€傞噴鏀捐鐢ㄦ埛鎸佹湁鐨勯攣

read
        杩斿洖涓€涓寚绀虹洰鏍囩姸鎬佺殑瀛楃涓诧紝渚嬪锛?

        "<card_ID>,decodes=<io_state>,owns=<io_state>,locks=<io_state> (ic,mc)"

        IO 鐘舵€佸瓧绗︿覆鐨勫舰寮忎负 {io,mem,io+mem,none}锛宮c 鍜?ic 鍒嗗埆鏄唴瀛樺拰 IO 鐨勯攣璁℃暟锛堜粎鐢ㄤ簬璋冭瘯/璇婃柇锛夈€傗€渄ecodes鈥?琛ㄧず鏄惧崱褰撳墠瑙ｇ爜鐨勫唴瀹癸紝鈥渙wns鈥?琛ㄧず褰撳墠鍦ㄥ叾涓婂惎鐢ㄧ殑鍐呭锛屸€渓ocks鈥?琛ㄧず琚鏄惧崱閿佸畾鐨勫唴瀹广€傚鏋滄樉鍗¤鎷斿嚭锛屽垯 card_ID 澶勪細杩斿洖 鈥渋nvalid鈥濓紝骞朵笖瀵逛簬浠讳綍鍛戒护閮戒細杩斿洖 -ENODEV 閿欒锛岀洿鍒版湁鏂扮殑鏄惧崱鎴愪负鐩爣銆?


write
        鍚戜徊瑁佸櫒鍐欏叆涓€鏉″懡浠ゃ€傚懡浠ゅ垪琛ㄥ涓嬶細

        target <card_ID>
                switch target to card <card_ID> (see below)
        lock <io_state>
                acquires locks on target ("none" is an invalid io_state)
        trylock <io_state>
                non-blocking acquire locks on target (returns EBUSY if
                unsuccessful)
        unlock <io_state>
                release locks on target
        unlock all
                release all locks on target held by this user (not implemented
                yet)
        decodes <io_state>
                set the legacy decoding attributes for the card

        poll
                褰撲换浣曟樉鍗★紙鑰屼笉浠呮槸鐩爣锛夊彂鐢熷彉鍖栨椂浜х敓浜嬩欢

        card_ID 鐨勫舰寮忎负 鈥淧CI:domain:bus:dev.fn鈥濄€傚彲浠ュ皢鍏惰涓?鈥渄efault鈥?浠ュ洖鍒扮郴缁熼粯璁ゆ樉鍗★紙TODO锛氬皻鏈疄鐜帮級銆傜洰鍓嶄粎鏀寔浠?PCI 浣滀负鍓嶇紑锛屼絾鍗充究褰撳墠鍐呮牳瀹炵幇涓嶆敮鎸侊紝鐢ㄦ埛鎬?API 鏈潵涔熷彲鑳芥敮鎸佸叾浠栨€荤嚎绫诲瀷銆?

## 鍏充簬閿佺殑璇存槑锛?

椹卞姩浼氳窡韪摢涓敤鎴峰湪鍝釜鏄惧崱涓婃寔鏈夊摢浜涢攣銆傚畠鏀寔宓屽锛坰tacking锛夛紝绫讳技浜庡唴鏍哥殑瀹炵幇銆傝繖浣垮疄鐜扮◢寰鏉備簡涓€浜涳紝浣嗕娇浠茶鍣ㄥ鐢ㄦ埛绌洪棿闂鏇村叿瀹归敊鎬э紝骞惰兘鍦ㄨ繘绋嬫浜＄殑鎵€鏈夋儏鍐典笅姝ｇ‘娓呯悊銆傜洰鍓嶏紝瀵逛簬缁欏畾鐨勭敤鎴凤紙鏂囦欢鎻忚堪绗﹀疄渚嬶級鑰岃█锛屾渶澶氬彲浠ユ湁 16 寮犳樉鍗″悓鏃舵寔鏈夋潵鑷敤鎴风┖闂寸殑閿併€?

鍦ㄨ澶囩儹鎻掓嫈锛坔ot-{un,}plugged锛夌殑鎯呭喌涓嬶紝鏈変竴涓挬瀛愨€斺€攑ci_notify()鈥斺€旂敤浜庨€氱煡瀹冧滑琚姞鍏?绉诲嚭绯荤粺锛屽苟鑷姩鍦ㄤ徊瑁佸櫒涓姞鍏?绉婚櫎銆?

濡傛灉 DRM銆乿gacon 鎴栧叾浠栭┍鍔ㄥ笇鏈涗娇鐢ㄤ徊瑁佸櫒锛岃繕鎻愪緵浜嗕竴涓唴鏍稿唴鐨勪徊瑁佸櫒 API銆?

## 鍐呮牳鍐呮帴鍙?


## :internal:

## :export:

## libpciaccess


涓轰簡浣跨敤 vgaarb 瀛楃璁惧锛屽湪 libpciaccess 搴撲腑瀹炵幇浜嗕竴涓?API銆傚悜 struct pci_device锛堟瘡涓澶?
```

    /* the type of resource decoded by the device */
    int vgaarb_rsrc;

```
```

    int vgaarb_fd;
    int vga_count;
    struct pci_device *vga_target;
    struct pci_device *vga_default_dev;

```
vga_count 鐢ㄤ簬璺熻釜姝ｅ湪琚徊瑁佺殑鏄惧崱鏁伴噺锛屽洜姝や緥濡傦紝濡傛灉鍙湁涓€寮犳樉鍗★紝閭ｄ箞瀹冨氨鍙互瀹屽叏閬垮紑浠茶銆?

涓嬮潰杩欎簺鍑芥暟浼氫负缁欏畾鏄惧崱鑾峰彇 VGA 璧勬簮锛屽苟灏嗚繖浜涜祫婧愭爣璁颁负宸查攣瀹氥€傚鏋滄墍璇锋眰鐨勮祫婧愭槸鈥滄櫘閫氣€濓紙鑰岄潪浼犵粺锛夎祫婧愶紝浠茶鍣ㄥ皢棣栧厛妫€鏌ヨ鏄惧崱鏄惁姝ｅ湪瀵硅绫诲瀷璧勬簮杩涜浼犵粺瑙ｇ爜銆傚鏋滄槸锛屽垯璇ラ攣浼氳鈥滆浆鎹⑩€濅负浼犵粺璧勬簮閿併€備徊瑁佸櫒灏嗛鍏堟煡鎵炬墍鏈夊彲鑳藉啿绐佺殑 VGA 鏄惧崱锛屽苟绂佺敤瀹冧滑鐨?IO 鍜?鎴栧唴瀛樿闂紙蹇呰鏃跺寘鎷?P2P 妗ヤ笂鐨?VGA 杞彂锛夛紝浠ヤ究鎵€璇锋眰鐨勮祫婧愬彲浠ヤ娇鐢ㄣ€傜劧鍚庯紝璇ユ樉鍗¤鏍囪涓洪攣瀹氳繖浜涜祫婧愶紝骞跺湪鍏朵笂鍚敤 IO 鍜?鎴栧唴瀛樿闂紙濡傛灉鏈夌殑璇濓紝鍖呮嫭鐖?P2P 妗ヤ笂鐨?VGA 杞彂锛夈€傚湪 vga_arb_lock() 鐨勬儏鍐典笅锛屽鏋滄煇涓啿绐佺殑鏄惧崱宸茬粡閿佸畾浜嗘煇涓墍闇€璧勬簮锛堟垨涓嶅悓鎬荤嚎娈典笂鐨勪换浣曡祫婧愶紝鍥犱负鎹垜鎵€鐭?P2P 妗ヤ笉浼氬尯鍒?VGA 鍐呭瓨鍜?IO锛夛紝璇ュ嚱鏁颁細闃诲銆傚鏋滄樉鍗″凡缁忔嫢鏈夎繖浜涜祫婧愶紝鍒欏嚱鏁版垚鍔熴€倂ga_arb_trylock() 浼氳繑鍥?(-EBUSY) 鑰屼笉鏄樆濉炪€傛敮鎸佸祵濂楄皟鐢紙缁存姢涓€涓瘡璧勬簮鐨勮鏁板櫒锛夈€?

```

    int  pci_device_vgaarb_set_target   (struct pci_device *dev);

```
渚嬪锛屽湪 x86 涓婏紝濡傛灉鍚屼竴鎬荤嚎涓婄殑涓や釜璁惧鎯宠閿佸畾涓嶅悓鐨勮祫婧愶紝涓よ€呴兘浼氭垚鍔燂紙閿佸畾锛夈€傚鏋滆澶囦綅浜庝笉鍚岀殑鎬荤嚎涓婏紝涓?
```

    int  pci_device_vgaarb_lock         (void);
    int  pci_device_vgaarb_trylock      (void);

```
```

    int  pci_device_vgaarb_unlock       (void);

```
鍚戜徊瑁佸櫒鎸囩ず璇ユ樉鍗℃槸鍚﹁В鐮佷紶缁?VGA IO銆佷紶缁?VGA 鍐呭瓨銆佷袱鑰咃紝杩樻槸閮戒笉瑙ｇ爜銆傛墍鏈夋樉鍗￠粯璁や袱鑰呴兘瑙ｇ爜锛屾樉鍗￠┍鍔紙渚嬪 fbdev锛夊簲鍛婄煡浠茶鍣ㄥ畠鏄惁宸茬鐢ㄤ紶缁熻В鐮侊紝浠ヤ究璇ユ樉鍗″彲浠ヨ鎺掗櫎鍦ㄤ徊瑁佽繃绋嬩箣澶栵紙骞朵笖鍙互瀹夊叏鍦板崰鐢?
```

    int  pci_device_vgaarb_decodes      (int new_vgaarb_rsrc);

```
```

    int  pci_device_vgaarb_init         (void);

```
```

    void pci_device_vgaarb_fini         (void);

```
xf86VGAArbiter锛圶 鏈嶅姟鍣ㄥ疄鐜帮級


X 鏈嶅姟鍣ㄥ熀鏈笂鍖呰浜嗘墍鏈変互鏌愮鏂瑰紡瑙﹀強 VGA 瀵勫瓨鍣ㄧ殑鍑芥暟銆?

## 鍙傝€冭祫鏂?


Benjamin Herrenschmidt锛圛BM?锛夊湪 2005 骞翠笌 Xorg 绀惧尯璁ㄨ杩欑璁捐鏃跺惎鍔ㄤ簡杩欓」宸ヤ綔 [1, 2]銆?007 骞村簳锛孭aulo Zanoni 鍜?Tiago Vignatti锛堝潎鏉ヨ嚜 C3SL/宸存媺閭ｈ仈閭﹀ぇ瀛︼級缁х画浜嗕粬鐨勫伐浣滐紝澧炲己浜嗗唴鏍镐唬鐮佷互閫傞厤涓轰竴涓唴鏍告ā鍧楋紝骞跺畬鎴愪簡鐢ㄦ埛绌洪棿涓€渚х殑瀹炵幇 [^3^]銆傚浠婏紙2009 骞达級锛孴iago Vignatti 鍜?Dave Airlie 鏈€缁堝皢杩欓」宸ヤ綔鏁寸悊鎴愬瀷锛屽苟鎺掑叆 Jesse Barnes 鐨?PCI 鏍戜腑銆?

0) https://cgit.freedesktop.org/xorg/xserver/commit/?id=4b42448a2388d40f257774fbffdccaea87bd0347
1) https://lists.freedesktop.org/archives/xorg/2005-March/006663.html
2) https://lists.freedesktop.org/archives/xorg/2005-March/006745.html
3) https://lists.freedesktop.org/archives/xorg/2007-October/029507.html
