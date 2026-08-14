
## HDMI CEC


## 涓荤嚎涓彈鏀寔鐨勭‖浠?

HDMI 鍙戦€佸櫒锛?
- Exynos4
- Exynos5
- STIH4xx HDMI CEC
- V4L2 adv7511锛堢浉鍚岀殑纭欢锛屼絾鏄笌 drm adv7511 涓嶅悓鐨勯┍鍔級
- stm32
- Allwinner A10 (sun4i)
- Raspberry Pi
- dw-hdmi (Synopsis IP)
- amlogic (meson ao-cec 鍜?ao-cec-g12a)
- drm adv7511/adv7533
- omap4
- tegra
- rk3288, rk3399
- tda998x
- DisplayPort CEC-Tunneling-over-AUX on i915, nouveau and amdgpu
- ChromeOS EC CEC
- CEC for SECO boards (UDOO x86).
- Chrontel CH7322


HDMI 鎺ユ敹鍣細

- adv7604/11/12
- adv7842
- tc358743

USB 鍔犲瘑鐙楋紙鏈夊叧濡備綍浣跨敤杩欎簺鍔犲瘑鐙楃殑鏇村淇℃伅锛岃鍙傝涓嬫枃锛夛細

- Pulse-Eight锛歱ulse8-cec 椹卞姩瀹炵幇浜嗕互涓嬫ā鍧楅€夐」锛歚persistent_config`锛氶粯璁ゆ儏鍐典笅瀹冩槸
  鍏抽棴鐨勶紝浣嗗綋璁句负 1 鏃讹紝椹卞姩浼氬皢褰撳墠璁剧疆瀛樺偍鍒拌澶囧唴閮?eeprom 涓紝骞跺湪涓嬫璁惧杩炴帴鍒?  USB 绔彛鏃舵仮澶嶃€?
- RainShadow Tech銆傛敞鎰忥細璇ラ┍鍔ㄤ笉鏀寔 Pulse-Eight 椹卞姩鐨?persistent_config 妯″潡閫夐」銆?  纭欢鏀寔璇ュ姛鑳斤紝浣嗘垜涓嶆墦绠楁坊鍔犳鐗规€с€備笉杩囨垜鎺ュ彈琛ヤ竵 :-)

- Extron DA HD 4K PLUS HDMI 鍒嗛厤鏀惧ぇ鍣ㄣ€傛洿澶氫俊鎭鍙傝 extron_da_hd_4k_plus銆?
鏉傞」锛?
- vivid锛氭ā鎷熶竴涓?CEC 鎺ユ敹鍣ㄥ拰涓€涓?CEC 鍙戦€佸櫒銆傚彲鐢ㄤ簬鍦ㄦ病鏈夊疄闄?CEC 纭欢鐨勬儏鍐典笅娴嬭瘯
  CEC 搴旂敤绋嬪簭銆?
- cec-gpio銆傚鏋?CEC 寮曡剼杩炴帴鍒?GPIO 寮曡剼锛屽垯鍙互閫氳繃姝ら┍鍔ㄦ帶鍒?CEC 绾胯矾銆傚畠杩樻敮鎸侀敊璇?  娉ㄥ叆銆?
- cec-gpio 浠ュ強 Allwinner A10锛堟垨浠讳綍鍏朵粬浣跨敤 CEC 寮曡剼妗嗘灦鐩存帴椹卞姩 CEC 寮曡剼鐨勯┍鍔級锛?  CEC 寮曡剼妗嗘灦浣跨敤楂樺垎杈ㄧ巼瀹氭椂鍣ㄣ€傝繖浜涘畾鏃跺櫒浼氬彈鍒?NTP 瀹堟姢杩涚▼鐨勫奖鍝嶏紝鍚庤€呬細鍔犲揩鎴栧噺鎱?  鏃堕挓浠ヤ笌瀹樻柟鏃堕棿鍚屾銆俢hronyd 鏈嶅姟鍣ㄩ粯璁や細灏嗘椂閽熷姞蹇垨鍑忔參 1/12銆傝繖浼氬鑷?CEC 鏃跺簭
  瓒呭嚭瑙勮寖銆傝淇姝ら棶棰橈紝鍙湪 chronyd.conf 涓坊鍔犱竴琛?'maxslewrate 40000'銆傝繖浼氬皢鏃堕挓
  棰戠巼鍙樺寲闄愬埗鍦?1/25锛屼粠鑰屼娇 CEC 鏃跺簭淇濇寔鍦ㄨ鑼冨唴銆?

## 宸ュ叿


宸ュ叿鍙湪姝ゅ鑾峰彇锛歨ttps://git.linuxtv.org/v4l-utils.git

`utils/cec-ctl`锛氭帶鍒?CEC 璁惧

`utils/cec-compliance`锛氭祴璇曡繙绋?CEC 璁惧鐨勫悎瑙勬€?
`utils/cec-follower`锛氭ā鎷熶竴涓?CEC 浠庤澶?
娉ㄦ剰锛宍cec-ctl` 鏀寔鏌愪簺閰掑簵鏄剧ず灞忎娇鐢ㄧ殑 CEC Hospitality Profile銆傝鍙傝
http://www.htng.org銆?
娉ㄦ剰锛宭ibcec 搴擄紙https://github.com/Pulse-Eight/libcec锛夋敮鎸?linux CEC 妗嗘灦銆?
濡傛灉浣犳兂鑾峰彇 CEC 瑙勮寖锛岃鏌ョ湅 HDMI 缁村熀鐧剧椤甸潰鐨勫弬鑰冩枃鐚細
https://en.wikipedia.org/wiki/HDMI銆侰EC 鏄?HDMI 瑙勮寖鐨勪竴閮ㄥ垎銆侶DMI 1.3 鍙互鍏嶈垂鑾峰彇
锛堝湪 CEC 鏂归潰涓?HDMI 1.4 闈炲父鐩镐技锛夛紝瀵瑰ぇ澶氭暟鐢ㄩ€斿簲璇ヨ冻澶熴€?

## 鏀寔 CEC 鐨?DisplayPort 杞?HDMI 閫傞厤鍣?

鑳屾櫙锛氬ぇ澶氭暟閫傞厤鍣ㄤ笉鏀寔 CEC Tunneling 鐗规€э紝鑰屽湪鏀寔璇ョ壒鎬х殑閫傞厤鍣ㄤ腑锛岃澶氬疄闄呬笂骞舵湭
杩炴帴 CEC 寮曡剼銆備笉骞哥殑鏄紝杩欐剰鍛崇潃铏界劧鍒涘缓浜?CEC 璁惧锛屼絾瀹冨疄闄呬笂鍦ㄤ笘鐣屼笂褰㈠崟褰卞彧锛屾案杩?鏃犳硶鐪嬪埌鍏朵粬 CEC 璁惧銆?
杩欐槸涓€涓凡鐭ョ殑鍙敤閫傞厤鍣ㄥ垪琛紝瀹冧滑鍏锋湁 CEC Tunneling 骞朵笖姝ｇ‘杩炴帴浜?CEC 寮曡剼銆傚鏋滀綘鍙戠幇
鍙敤鐨勯€傞厤鍣ㄤ絾涓嶅湪鏈垪琛ㄤ腑锛岃缁欐垜鐣欒█銆?
娴嬭瘯鏂规硶锛氬皢 DP 杞?HDMI 閫傞厤鍣ㄨ繛鎺ュ埌鏀寔 CEC 鐨勮澶?
```

	cec-ctl --playback	# Configure the PC as a CEC Playback device
	cec-ctl -S		# Show the CEC topology

```
`cec-ctl -S` 鍛戒护搴旇嚦灏戞樉绀轰袱涓?CEC 璁惧锛氭垜浠嚜宸卞拰鎵€杩炴帴鐨?CEC 璁惧锛堥€氬父鏄數瑙嗭級銆?
涓€鑸鏄庯細鎴戝彧鍦?Parade PS175銆丳S176 鍜?PS186 鑺墖缁勪互鍙?MegaChips 2900 涓婅杩囧畠姝ｅ父宸ヤ綔銆?铏界劧 MegaChips 28x0 澹扮О鏀寔 CEC锛屼絾鎴戜粠鏈杩囧畠宸ヤ綔銆?
### USB-C 杞?HDMI


Samsung Multiport Adapter EE-PW700: https://www.samsung.com/ie/support/model/EE-PW700BBEGWW/

Kramer ADC-U31C/HF: https://www.kramerav.com/product/ADC-U31C/HF

Club3D CAC-2504: https://www.club-3d.com/en/detail/2449/usb_3.1_type_c_to_hdmi_2.0_uhd_4k_60hz_active_adapter/

### DisplayPort 杞?HDMI


Club3D CAC-1080: https://www.club-3d.com/en/detail/2442/displayport_1.4_to_hdmi_2.0b_hdr/

CableCreation (SKU: CD0712): https://www.cablecreation.com/products/active-displayport-to-hdmi-adapter-4k-hdr

HP DisplayPort to HDMI True 4k Adapter (P/N 2JA63AA): https://www.hp.com/us-en/shop/pdp/hp-displayport-to-hdmi-true-4k-adapter

### Mini-DisplayPort 杞?HDMI


Club3D CAC-1180: https://www.club-3d.com/en/detail/2443/mini_displayport_1.4_to_hdmi_2.0b_hdr/

娉ㄦ剰锛屾棤婧愰€傞厤鍣ㄦ案杩滀笉浼氬伐浣滐紝浣犻渶瑕佹湁婧愰€傞厤鍣ㄣ€?
鏈垪琛ㄤ腑鐨?Club3D 閫傞厤鍣ㄩ兘鍩轰簬 MegaChips 2900銆傚叾浠?Club3D 閫傞厤鍣ㄥ熀浜?PS176 涓旀病鏈夎繛鎺?CEC 寮曡剼锛屽洜姝ゅ彧鏈変笂杩颁笁娆?Club3D 閫傞厤鍣ㄥ凡鐭ュ彲鐢ㄣ€?
鎴戞€€鐤戝熀浜?MegaChips 2900 鐨勮璁￠€氬父鍙兘鍙敤锛岃€?PS176 鍒欐洿鍍忔槸纰拌繍姘旓紙澶у涓嶅彲鐢級銆?PS186 寰堝彲鑳借繛鎺ヤ簡 CEC 寮曡剼锛岀湅璧锋潵浠栦滑鏇存敼浜嗚鑺墖缁勭殑鍙傝€冭璁°€?

## USB CEC 鍔犲瘑鐙?

杩欎簺鍔犲瘑鐙楁樉绀轰负 `/dev/ttyACMX` 璁惧锛岄渶瑕?`inputattach` 宸ュ叿鏉ュ垱寤?`/dev/cecX` 璁惧銆?瀵?Pulse-Eight 鐨勬敮鎸佸凡娣诲姞鍒?`inputattach` 1.6.0銆傚 Rainshadow Tech 鐨勬敮鎸佸凡娣诲姞鍒?`inputattach` 1.6.1銆?
```

	SUBSYSTEM=="tty", KERNEL=="ttyACM[0-9]*", ATTRS{idVendor}=="2548", ATTRS{idProduct}=="1002", ACTION=="add", TAG+="systemd", ENV{SYSTEMD_WANTS}+="pulse8-cec-inputattach@%k.service"
	SUBSYSTEM=="tty", KERNEL=="ttyACM[0-9]*", ATTRS{idVendor}=="2548", ATTRS{idProduct}=="1001", ACTION=="add", TAG+="systemd", ENV{SYSTEMD_WANTS}+="pulse8-cec-inputattach@%k.service"
	SUBSYSTEM=="tty", KERNEL=="ttyACM[0-9]*", ATTRS{idVendor}=="04d8", ATTRS{idProduct}=="ff59", ACTION=="add", TAG+="systemd", ENV{SYSTEMD_WANTS}+="rainshadow-cec-inputattach@%k.service"

```
浠ュ強杩欎簺 systemd 鏈嶅姟锛?
```

	[Unit]
	Description=inputattach for pulse8-cec device on %I

	[Service]
	Type=simple
	ExecStart=/usr/bin/inputattach --pulse8-cec /dev/%I

```
```

	[Unit]
	Description=inputattach for rainshadow-cec device on %I

	[Service]
	Type=simple
	ExecStart=/usr/bin/inputattach --rainshadow-cec /dev/%I


```
```

	[Unit]
	Description=restart inputattach for cec devices
	After=suspend.target

	[Service]
	Type=forking
	ExecStart=/bin/bash -c 'for d in /dev/serial/by-id/usb-Pulse-Eight*; do /usr/bin/inputattach --daemon --pulse8-cec $d; done; for d in /dev/serial/by-id/usb-RainShadow_Tech*; do /usr/bin/inputattach --daemon --rainshadow-cec $d; done'

	[Install]
	WantedBy=suspend.target

```
骞惰繍琛?`systemctl enable restart-cec-inputattach`銆?
瑕佸湪 CEC 璁惧鍒涘缓鏃惰嚜鍔ㄨ缃叾鐗╃悊鍦板潃

```

	cec-ctl -E /sys/class/drm/card0-DP-1/edid

```
杩欏亣璁惧姞瀵嗙嫍杩炴帴鍒?card0-DP-1 杈撳嚭锛坄xrandr` 浼氬憡璇変綘浣跨敤鐨勬槸鍝釜杈撳嚭锛夛紝瀹冧細杞 EDID
鐨勫彉鍖栧苟鍦ㄥ彂鐢熷彉鍖栨椂鏇存柊鐗╃悊鍦板潃銆?
瑕佽嚜鍔ㄨ繍琛屾鍛戒护锛屽彲浠ヤ娇鐢?cron銆傜敤浠ヤ笅鏂瑰紡缂栬緫 crontab锛?
```

	@reboot /usr/local/bin/cec-ctl -E /sys/class/drm/card0-DP-1/edid

```
杩欎粎閫傜敤浜庡湪 `/sys/class/drm` 涓毚闇?EDID 鐨勬樉绀洪┍鍔紝渚嬪 i915 椹卞姩銆?

## 鏃?HPD 鐨?CEC


鏌愪簺鏄剧ず鍣ㄥ湪寰呮満妯″紡涓嬫病鏈?HDMI 鐑彃鎷旀娴嬶紙Hotplug Detect锛変俊鍙凤紝浣?CEC 浠嶇劧鍚敤锛屽洜姝?杩炴帴鐨勮澶囧彲浠ュ彂閫?<Image View On> CEC 娑堟伅浠ュ敜閱掓绫绘樉绀哄櫒銆備笉骞哥殑鏄紝骞堕潪鎵€鏈?CEC 閫傞厤鍣?閮芥敮鎸佽繖涓€鐐广€備緥濡?Odroid-U3 SBC锛屽叾鐢靛钩杞崲鍣ㄥ湪 HPD 淇″彿涓轰綆鐢靛钩鏃舵柇鐢碉紝浠庤€岄樆濉?CEC
寮曡剼銆傚嵆浣?SoC 鍙互鍦ㄦ棤 HPD 鐨勬儏鍐典笅浣跨敤 CEC锛岀數骞宠浆鎹㈠櫒涔熶細闃绘鍏舵甯稿伐浣溿€?
鏈変竴涓?CEC 鑳藉姏鏍囧織鏉ヨ〃绀鸿繖涓€鐐癸細`CEC_CAP_NEEDS_HPD`銆傚鏋滆缃簡璇ユ爣蹇楋紝鍒欑‖浠舵棤娉曚互
杩欑鏂瑰紡鍞ら啋鏄剧ず鍣ㄣ€?
缁?CEC 搴旂敤绋嬪簭瀹炵幇鑰呯殑鎻愮ず锛?Image View On> 娑堟伅蹇呴』鏄綘鍙戦€佺殑绗竴鏉℃秷鎭紝鍦ㄦ涔嬪墠涓嶈
鍙戦€佷换浣曞叾浠栨秷鎭€傛煇浜涢潪甯哥碂绯曚絾涓嶅垢鍦板苟涓嶅皯瑙佺殑 CEC 瀹炵幇锛屽鏋滄敹鍒伴櫎璇ユ秷鎭互澶栫殑浠讳綍
娑堟伅锛屽氨浼氬彉寰楅潪甯告贩涔憋紝浠庤€屼笉浼氬敜閱掋€?
缂栧啓椹卞姩鏃讹紝娴嬭瘯杩欎竴鐐瑰彲鑳藉緢妫樻墜銆傛湁涓ょ鏂规硶鍙互鍋氬埌锛?
1) 鑾峰彇涓€涓?Pulse-Eight USB CEC 鍔犲瘑鐙楋紝鐢?HDMI 绾跨紗灏嗕綘鐨勮澶囪繛鎺ュ埌 Pulse-Eight锛屼絾
   涓嶈灏?Pulse-Eight 杩炴帴鍒版樉绀哄櫒銆?
```

	cec-ctl -p0.0.0.0 --tv

   and start monitoring::

	sudo cec-ctl -M

   On the device you are testing run::

	cec-ctl --playback

   It should report a physical address of f.f.f.f. Now run this
   command::

	cec-ctl -t0 --image-view-on

   The Pulse-Eight should see the <Image View On> message. If not,
   then something (hardware and/or software) is preventing the CEC
   message from going out.

   To make sure you have the wiring correct just connect the
   Pulse-Eight to a CEC-enabled display and run the same command
   on your device: now there is a HPD, so you should see the command
   arriving at the Pulse-Eight.

```
2) 濡傛灉浣犳湁鍙︿竴鍙版敮鎸佹棤 HPD CEC 鐨?linux 璁惧锛屽垯鍙互鐩存帴灏嗕綘鐨勮澶囪繛鎺ュ埌璇ヨ澶囥€傛槸鐨勶紝
   浣犲彲浠ュ皢涓や釜 HDMI 杈撳嚭杩炴帴鍦ㄤ竴璧枫€備綘灏嗘病鏈?HPD锛堣繖姝ｆ槸鎴戜滑鍦ㄦ娴嬭瘯涓兂瑕佺殑锛夛紝浣嗙浜屽彴
   璁惧鍙互鐩戞帶 CEC 寮曡剼銆傚惁鍒欎娇鐢ㄤ笌 1 鐩稿悓鐨勫懡浠ゃ€?
濡傛灉娌℃湁 HPD 鏃?CEC 娑堟伅鏃犳硶閫氳繃锛屽垯闇€瑕佹壘鍑哄師鍥犮€傞€氬父杩欒涔堟槸纭欢闄愬埗锛岃涔堟槸杞欢鍦?HPD
鍙樹綆鏃跺叧闂簡 CEC 鏍稿績銆傚墠鑰呭綋鐒舵棤娉曠籂姝ｏ紝鍚庤€呭緢鍙兘闇€瑕佷慨鏀归┍鍔ㄣ€?

## 寰帶鍒跺櫒涓?CEC


鎴戜滑瑙佽繃涓€浜涙樉绀哄櫒涓殑 CEC 瀹炵幇浣跨敤寰帶鍒跺櫒瀵规€荤嚎杩涜閲囨牱銆傝繖涓嶄竴瀹氭槸涓棶棰橈紝浣嗘煇浜涘疄鐜?瀛樺湪鏃跺簭闂銆傞櫎闈炰綘鑳芥帴涓婁竴涓簳灞傜殑 CEC 璋冭瘯鍣紙鍙傝涓嬩竴鑺傦級锛屽惁鍒欏緢闅惧彂鐜拌繖涓€鐐广€?
浣犱細鐪嬪埌 CEC 鍙戦€佸櫒灏?CEC 绾胯矾鎷夐珮鎴栨媺浣庣殑鏃堕棿瓒呰繃鍏佽鍊肩殑鎯呭喌銆傚浜庡畾鍚戞秷鎭紝杩欎笉鏄棶棰橈紝
鍥犱负濡傛灉鍙戠敓杩欑鎯呭喌锛屾秷鎭笉浼氳纭锛圓cked锛夛紝骞跺皢琚噸浼犮€傚浜庡箍鎾秷鎭垯涓嶅瓨鍦ㄨ繖绉嶆満鍒躲€?
鐩墠灏氫笉娓呮璇ュ浣曞鐞嗐€傛槑鏅虹殑鍋氭硶鍙兘鏄皢鏌愪簺骞挎挱娑堟伅鍙戦€佷袱娆★紝浠ラ檷浣庡畠浠涪澶辩殑姒傜巼銆?鍏蜂綋鑰岃█锛?Standby> 鍜?<Active Source> 鏄繖绫绘秷鎭殑鍊欓€夈€?

## 鍒朵綔涓€涓?CEC 璋冭瘯鍣?

閫氳繃浣跨敤 Raspberry Pi 4B 鍜屼竴浜涘粔浠风粍浠讹紝浣犲彲浠ュ埗浣滆嚜宸辩殑搴曞眰 CEC 璋冭瘯鍣ㄣ€?
鍏抽敭缁勪欢鏄互涓?HDMI 姣嶅姣嶇洿閫氳繛鎺ュ櫒涔嬩竴锛堝叏鐒婃帴鍨?1锛夛細

https://elabbay.myshopify.com/collections/camera/products/hdmi-af-af-v1a-hdmi-type-a-female-to-hdmi-type-a-female-pass-through-adapter-breakout-board?variant=45533926147

瑙嗛璐ㄩ噺涓嶇ǔ瀹氾紝鑲畾涓嶈冻浠ョ洿閫?4kp60锛?94 MHz锛夎棰戙€備綘鍙兘鑳藉鏀寔 4kp30锛屼絾鏇村彲鑳藉彈闄愪簬
1080p60锛?48.5 MHz锛夈€備絾瀵逛簬 CEC 娴嬭瘯鏉ヨ杩欏凡缁忚冻澶熴€?
浣犻渶瑕佷竴涓潰鍖呮澘鍜屼竴浜涢潰鍖呮澘绾匡細

http://www.dx.com/p/diy-40p-male-to-female-male-to-male-female-to-female-dupont-line-wire-3pcs-356089#.WYLOOXWGN7I

濡傛灉浣犺繕鎯崇洃鎺?HPD 鍜?鎴?5V 绾胯矾锛岄偅涔堜綘闇€瑕佷互涓?5V 鍒?3.3V 鐢靛钩杞崲鍣ㄤ箣涓€锛?
https://www.adafruit.com/product/757

锛堣繖鍙槸鎴戣喘涔拌繖浜涚粍浠剁殑鍦版柟锛屼綘杩樺彲浠ヤ粠璁稿鍏朵粬鍦版柟涔板埌绫讳技鐨勪笢瑗匡級銆?
褰撶劧锛孒DMI 杩炴帴鍣ㄧ殑鍦板紩鑴氶渶瑕佽繛鎺ュ埌 Raspberry Pi 鐨勫湴寮曡剼銆?
HDMI 杩炴帴鍣ㄧ殑 CEC 寮曡剼闇€瑕佽繛鎺ュ埌浠ヤ笅寮曡剼锛欸PIO 6 鍜?GPIO 7銆侶DMI 杩炴帴鍣ㄥ彲閫夌殑 HPD 寮曡剼
搴旈€氳繃鐢靛钩杞崲鍣ㄨ繛鎺ュ埌浠ヤ笅寮曡剼锛欸PIO 23 鍜?GPIO 12銆侶DMI 杩炴帴鍣ㄥ彲閫夌殑 5V 寮曡剼搴旈€氳繃鐢靛钩
杞崲鍣ㄨ繛鎺ュ埌浠ヤ笅寮曡剼锛欸PIO 25 鍜?GPIO 22銆傜洃鎺?HPD 鍜?5V 绾胯矾涓嶆槸蹇呴渶鐨勶紝浣嗗緢鏈夊府鍔┿€?
鍦?`arch/arm/boot/dts/bcm2711-rpi-4-b.dts` 涓坊鍔犱互涓嬭澶囨爲锛?
```

	cec@6 {
		compatible = "cec-gpio";
		cec-gpios = <&gpio 6 (GPIO_ACTIVE_HIGH|GPIO_OPEN_DRAIN)>;
		hpd-gpios = <&gpio 23 GPIO_ACTIVE_HIGH>;
		v5-gpios = <&gpio 25 GPIO_ACTIVE_HIGH>;
	};

	cec@7 {
		compatible = "cec-gpio";
		cec-gpios = <&gpio 7 (GPIO_ACTIVE_HIGH|GPIO_OPEN_DRAIN)>;
		hpd-gpios = <&gpio 12 GPIO_ACTIVE_HIGH>;
		v5-gpios = <&gpio 22 GPIO_ACTIVE_HIGH>;
	};

```
濡傛灉浣犳病鏈夎繛鎺?HPD 鍜?鎴?5V 绾胯矾锛屽垯鍙渶鍒犻櫎杩欎簺琛屻€?
杩欎釜 dts 鏀瑰姩灏嗗惎鐢ㄤ袱涓?cec GPIO 璁惧锛氭垜閫氬父鐢ㄤ竴涓潵鍙戦€?鎺ユ敹 CEC 鍛戒护锛屽彟涓€涓敤浜庣洃鎺с€?濡傛灉浣犱娇鐢ㄦ湭閰嶇疆鐨?CEC 閫傞厤鍣ㄨ繘琛岀洃鎺э紝瀹冨皢浣跨敤 GPIO 涓柇锛屼粠鑰屼娇鐩戞帶闈炲父绮剧‘銆?
濡傛灉浣犲彧鎯崇洃鎺ф祦閲忥紝閭ｄ箞鍗曚釜瀹炰緥灏辫冻澶熶簡銆傛渶灏忛厤缃槸涓€涓?HDMI 姣嶅姣嶇洿閫氳繛鎺ュ櫒浠ュ強涓ゆ牴
姣嶅姣嶉潰鍖呮澘绾匡細涓€鏍圭敤浜庡皢 HDMI 鍦板紩鑴氳繛鎺ュ埌 Raspberry Pi 涓婄殑鍦板紩鑴氾紝鍙︿竴鏍圭敤浜庡皢 HDMI
CEC 寮曡剼杩炴帴鍒?Raspberry Pi 涓婄殑 GPIO 6銆?
鏈夊叧濡備綍浣跨敤閿欒娉ㄥ叆鐨勬枃妗ｈ鍙傝锛歝ec_pin_error_inj銆?
`cec-ctl --monitor-pin` 灏嗘墽琛屽簳灞傜殑 CEC 鎬荤嚎鍡呮帰鍜屽垎鏋愩€備綘杩樺彲浠ヤ娇鐢?`--store-pin` 灏?CEC 娴侀噺瀛樺偍鍒版枃浠讹紝骞朵娇鐢?`--analyze-pin` 绋嶅悗鍒嗘瀽銆?
浣犺繕鍙互灏嗗叾閰嶇疆涓哄畬鏁寸殑 CEC 璁惧锛屼娇鐢?`cec-ctl --tv -p0.0.0.0` 鎴?`cec-ctl --playback -p1.0.0.0`銆?

## Extron DA HD 4K PLUS CEC 閫傞厤鍣ㄩ┍鍔?

姝ら┍鍔ㄧ敤浜?Extron DA HD 4K PLUS 绯诲垪 HDMI 鍒嗛厤鏀惧ぇ鍣細
https://www.extron.com/product/dahd4kplusseries

鏀寔 2銆? 鍜?6 绔彛鍨嬪彿銆?
闇€瑕佸浐浠剁増鏈?1.02.0001 鎴栨洿楂樸€?
娉ㄦ剰锛岃緝鏃х殑 Extron 纭欢鐗堟湰瀛樺湪 CEC 鐢靛帇闂锛岃繖鍙兘鎰忓懗鐫€ CEC 鏃犳硶宸ヤ綔銆傝闂鍦ㄧ‖浠?鐗堟湰 E34814 鍙婃洿楂樼増鏈腑寰楀埌淇銆?
CEC 鏀寔鏈変袱绉嶆ā寮忥細绗竴绉嶆槸鎵嬪姩妯″紡锛岀敤鎴风┖闂村繀椤绘墜鍔ㄦ帶鍒?HDMI 杈撳叆鍜屾墍鏈?HDMI 杈撳嚭鐨?CEC銆傝櫧鐒惰繖鎻愪緵浜嗗畬鍏ㄧ殑鎺у埗鏉冿紝浣嗕篃姣旇緝澶嶆潅銆?
绗簩绉嶆槸鑷姩妯″紡锛屽綋璁剧疆浜嗘ā鍧楅€夐」 `vendor_id` 鏃堕€変腑銆傚湪杩欑鎯呭喌涓嬶紝椹卞姩鎺у埗 CEC锛屽苟涓?鍦ㄨ緭鍏ヤ腑鎺ユ敹鍒扮殑 CEC 娑堟伅灏嗚鍒嗗彂鍒板悇涓緭鍑恒€備粛鐒跺彲浠ヤ娇鐢?/dev/cecX 璁惧鐩存帴涓庢墍杩炴帴鐨?璁惧閫氫俊锛屼絾鎵€鏈夐厤缃互鍙婄儹鎻掓嫈妫€娴嬪彉鍖栫瓑浜嬪姟閮界敱椹卞姩澶勭悊銆?
椹卞姩杩樿礋璐?EDID锛氫細鍒涘缓 /dev/videoX 璁惧鏉ヨ鍙?EDID 浠ュ強锛堥拡瀵?HDMI 杈撳叆绔彛锛夎缃?EDID銆?
榛樿鎯呭喌涓嬶紝鐢ㄦ埛绌洪棿璐熻矗鏍规嵁鎵€杩炴帴鏄剧ず鍣ㄧ殑 EDID 涓?HDMI 杈撳叆璁剧疆 EDID銆備絾濡傛灉璁剧疆浜?`manufacturer_name` 妯″潡閫夐」锛屽垯椹卞姩灏嗘牴鎹墍杩炴帴鏄剧ず鍣ㄦ敮鎸佺殑鍒嗚鲸鐜囦负 HDMI 杈撳叆璁剧疆 EDID銆?鐩墠椹卞姩浠呮敮鎸?1080p60 鍜?4kp60 鍒嗚鲸鐜囷細濡傛灉鎵€鏈夎繛鎺ョ殑鏄剧ず鍣ㄩ兘鏀寔 4kp60锛屽垯瀹冧細鍦?HDMI
杈撳叆涓婇€氬憡 4kp60锛屽惁鍒欏皢鍥為€€鍒颁粎鎶ュ憡 1080p60 鐨?EDID銆?
Extron 鐨勭姸鎬佹姤鍛婂湪 `/sys/kernel/debug/cec/cecX/status` 涓€?
extron-da-hd-4k-plus 椹卞姩瀹炵幇浜嗕互涓嬫ā鍧楅€夐」锛?
### ``debug``

濡傛灉璁句负 1锛屽垯鏄剧ず鎵€鏈変覆鍙ｆ祦閲忋€?
### ``vendor_id``

瑕佹姤鍛婄粰鎵€杩炴帴鏄剧ず鍣ㄧ殑 CEC 鍘傚晢 ID銆?
濡傛灉璁剧疆锛屽垯椹卞姩璐熻矗灏嗚緭鍏ヤ腑鎺ユ敹鍒扮殑 CEC 娑堟伅鍒嗗彂鍒?HDMI 杈撳嚭銆備互涓?CEC 娑堟伅浼氳繘琛屾澶勭悊锛?
- <Standby>
- <Image View On> 鍜?<Text View On>
- <Give Device Power Status>
- <Set System Audio Mode>
- <Request Current Latency>

濡傛灉鏈缃紝鍒欑敤鎴风┖闂磋礋璐ｆ浜嬶紝骞朵笖蹇呴』鎵嬪姩涓?HDMI 杈撳叆鍜?HDMI 杈撳嚭閰嶇疆 CEC 璁惧銆?
### ``manufacturer_name``

鐢ㄤ簬 HDMI 杈撳叆 EDID 鐨勪笁瀛楃鍘傚晢鍚嶇О銆傚鏋滄湭璁剧疆锛屽垯鐢ㄦ埛绌洪棿璐熻矗閰嶇疆 EDID銆傚鏋滆缃紝鍒?椹卞姩灏嗘牴鎹墍杩炴帴鏄剧ず鍣ㄦ敮鎸佺殑鍒嗚鲸鐜囪嚜鍔ㄦ洿鏂?EDID锛屽苟涓斿皢鏃犳硶鍐嶆墜鍔ㄨ缃?HDMI 杈撳叆鐨?EDID銆?
### ``hpd_never_low``

濡傛灉璁剧疆锛屽垯 HDMI 杈撳叆鐨?Hotplug Detect 寮曡剼灏嗗缁堜负楂樼數骞筹紝鍗充娇娌℃湁浠讳綍涓滆タ杩炴帴鍒?HDMI
杈撳嚭銆傚鏋滄湭璁剧疆锛堥粯璁わ級锛屽垯褰?HDMI 杈撳嚭鐨勬墍鏈夋娴嬪埌鐨?Hotplug Detect 寮曡剼涔熶负浣庣數骞虫椂锛?HDMI 杈撳叆鐨?Hotplug Detect 寮曡剼灏嗗彉浣庛€?
姝ら€夐」鍙互鍔ㄦ€佹洿鏀广€?