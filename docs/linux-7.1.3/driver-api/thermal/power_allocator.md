## power allocator 璋冭妭鍣ㄥ彲璋冨弬鏁?

### 瑙﹀彂鐐癸紙Trip points锛?

璇ヨ皟鑺傚櫒鍦ㄥ叿鏈変互涓嬩袱涓鍔ㄨЕ鍙戠偣鏃跺伐浣滄渶浣筹細

1. 鈥渟witch on鈥濓紙寮€鍚級瑙﹀彂鐐癸細娓╁害楂樹簬姝ゅ€兼椂锛岃皟鑺傚櫒鐨勬帶鍒跺惊鐜紑濮嬭繍琛屻€?   杩欐槸鐑尯锛坱hermal zone锛夌殑绗竴涓鍔ㄨЕ鍙戠偣銆?
2. 鈥渄esired temperature鈥濓紙鏈熸湜娓╁害锛夎Е鍙戠偣锛氬畠搴斿綋楂樹簬鈥渟witch on鈥濊Е鍙戠偣銆?   杩欐槸璋冭妭鍣ㄦ墍鎺у埗鐨勭洰鏍囨俯搴︺€傝繖鏄儹鍖虹殑鏈€鍚庝竴涓鍔ㄨЕ鍙戠偣銆?

### PID 鎺у埗鍣?

power allocator 璋冭妭鍣ㄥ疄鐜颁簡涓€涓瘮渚?绉垎-瀵兼暟鎺у埗鍣紙PID 鎺у埗鍣級锛屼互娓╁害涓?鎺у埗杈撳叆銆佷互鍔熺巼涓哄彈鎺ц緭鍑猴細

    P_max = k_p ** e + k_i ** err_integral + k_d * diff_err + sustainable_power

鍏朵腑
   - e = desired_temperature - current_temperature锛堟湡鏈涙俯搴?- 褰撳墠娓╁害锛?   - err_integral 鏄箣鍓嶆墍鏈夎宸殑绱姞鍜?   - diff_err = e - previous_error锛堝綋鍓嶈宸?- 涓婁竴娆¤宸級

```
				      k_d
				       |
  current_temp                         |
       |                               v
       |              +----------+   +---+
       |       +----->| diff_err |-->| X |------+
       |       |      +----------+   +---+      |
       |       |                                |      tdp        actor
       |       |                      k_i       |       |  get_requested_power()
       |       |                       |        |       |        |     |
       |       |                       |        |       |        |     | ...
       v       |                       v        v       v        v     v
     +---+     |      +-------+      +---+    +---+   +---+   +----------+
     | S |-----+----->| sum e |----->| X |--->| S |-->| S |-->|power     |
     +---+     |      +-------+      +---+    +---+   +---+   |allocation|
       ^       |                                ^             +----------+
       |       |                                |                |     |
       |       |        +---+                   |                |     |
       |       +------->| X |-------------------+                v     v
       |                +---+                               granted performance
  desired_temperature     ^
			  |
			  |
		      k_po/k_pu
```

### 鍙寔缁姛鐜囷紙Sustainable power锛?

鍦ㄦ敞鍐岀儹鍖烘椂锛屽簲褰撴彁渚涗竴涓彲鎸佺画鑰楁暎鍔熺巼锛堝崟浣?mW锛夌殑浼拌鍊笺€傚畠浼拌浜嗗湪鏈熸湜鐨?鎺у埗娓╁害涓嬪彲浠ヨ€楁暎鐨勬寔缁姛鐜囥€傝繖鏄湪鏈熸湜鐨勬渶楂樻俯搴︿笅鍙垎閰嶇殑鏈€澶ф寔缁姛鐜囥€傚疄闄?鎸佺画鍔熺巼鍙兘浼氬洜涓哄绉嶅師鍥犺€屽彉鍖栥€傞棴鐜帶鍒跺櫒浼氬鐞嗚濡傜幆澧冩潯浠讹紝浠ュ強涓庣鐗囬€熷害
绛夌骇锛坰peed-grade锛夌浉鍏崇殑涓€浜涘洜绱犳墍甯︽潵鐨勫彉鍖栥€傚洜姝?`sustainable_power` 浠呬粎鏄竴
涓及璁″€硷紝骞朵笖鍙互琚皟浼樹互褰卞搷鐑埇鍗囷紙thermal ramp锛夌殑婵€杩涚▼搴︺€備綔涓哄弬鑰冿紝涓€閮?4 鑻卞鎵嬫満鐨勫彲鎸佺画鍔熺巼閫氬父涓?2000mW锛岃€屼竴鍙?10 鑻卞骞虫澘绾︿负 4500mW锛堝彲鑳介殢灞忓箷
灏哄鑰屽彉锛夈€備篃鍙互鐢ㄤ竴涓娊璞＄殑鏍囧害鏉ヨ〃杈惧姛鐜囧€笺€傛寔缁姛鐜囧簲褰撲笌鐩稿叧鍐峰嵈璁惧鎵€浣跨敤鐨?鏍囧害瀵归綈銆?
濡傛灉浣犱娇鐢ㄧ殑鏄澶囨爲锛岃鎶婂畠浣滀负
```
	thermal-zones {
		soc_thermal {
			polling-delay = <1000>;
			polling-delay-passive = <100>;
			sustainable-power = <2500>;
			...
```
鐨勫睘鎬ф潵娣诲姞銆?
鐩稿弽锛屽鏋滅儹鍖烘槸浠庡钩鍙颁唬鐮佹敞鍐岀殑锛屽垯浼犲叆涓€涓甫鏈?`sustainable_power` 鐨?`thermal_zone_params`銆傚鏋滃師鏈病鏈変紶鍏?`thermal_zone_params`锛岄偅涔堢被浼间笅闈㈣繖鏍?```
	static const struct thermal_zone_params tz_params = {
		.sustainable_power = 3500,
	};
```
鐒跺悗锛屾妸 `tz_params` 浣滀负绗?5 涓弬鏁颁紶缁?`thermal_zone_device_register()`銆?

### k_po 涓?k_pu


power allocator 鐑皟鑺傚櫒涓?PID 鎺у埗鍣ㄧ殑瀹炵幇鍏佽閰嶇疆涓や釜姣斾緥椤瑰父鏁帮細`k_po` 鍜?`k_pu`銆俙k_po` 鏄俯搴﹁秴璋冩湡闂达紙褰撳墠娓╁害楂樹簬鈥渄esired temperature鈥濊Е鍙戠偣锛夌殑
姣斾緥椤瑰父鏁般€傚弽涔嬶紝`k_pu` 鏄俯搴︽瑺璋冩湡闂达紙褰撳墠娓╁害浣庝簬鈥渄esired temperature鈥濊Е鍙戠偣锛?鐨勬瘮渚嬮」甯告暟銆?
杩欎簺鎺у埗椤规棬鍦ㄤ綔涓洪厤缃郴缁熷厑璁哥殑thermal 鈥渞amp鈥濓紙鐑埇鍗囷級鐨勪富瑕佹満鍒躲€備緥濡傦紝杈冧綆
鐨?`k_pu` 鍊间細鎻愪緵鏇存參鐨勭埇鍗囷紝浠ｄ环鏄湪浣庢俯涓嬮檺鍒跺彲鐢ㄥ閲忋€傚彟涓€鏂归潰锛岃緝楂樼殑
`k_pu` 鍊间細瀵艰嚧璋冭妭鍣ㄥ湪娓╁害杈冧綆鏃舵巿浜堥潪甯搁珮鐨勫姛鐜囷紝骞跺彲鑳藉鑷存俯搴﹁秴璋冦€?
```
    2 * sustainable_power / (desired_temperature - switch_on_temp)
```
杩欐剰鍛崇潃鍦?`switch_on_temp` 澶勶紝鎺у埗鍣ㄦ瘮渚嬮」鐨勮緭鍑哄皢鏄?2 * `sustainable_power`銆?榛樿鍊?```
    sustainable_power / (desired_temperature - switch_on_temp)
```
鍏虫敞 PID 鐨勬瘮渚嬮」鍜屽墠棣堝€?```
    P_max = k_p * e + sustainable_power
```
姣斾緥椤逛笌鏈熸湜娓╁害鍜屽綋鍓嶆俯搴︿箣宸垚姝ｆ瘮銆傚綋褰撳墠娓╁害灏辨槸鏈熸湜娓╁害鏃讹紝姣斾緥鍒嗛噺涓洪浂锛?`P_max` = `sustainable_power`銆備篃灏辨槸璇达紝鍦ㄦ亽瀹氳礋杞戒笅锛岀郴缁熷簲褰撹繍琛屽湪鐑钩琛＄姸鎬併€?`sustainable_power` 浠呬粎鏄竴涓及璁″€硷紝杩欐鏄渶瑕佹绫婚棴鐜帶鍒剁殑鍘熷洜銆?
```
    P_max = 2 * sustainable_power * (T_set - T) / (T_set - T_on) +
	sustainable_power
```
鍏朵腑锛?
    - T_set 鏄湡鏈涙俯搴?    - T 鏄綋鍓嶆俯搴?    - T_on 鏄紑鍚俯搴︼紙switch on temperature锛?
褰撳綋鍓嶆俯搴﹀氨鏄?switch_on 娓╁害鏃讹紝涓婂紡
```
    P_max = 2 * sustainable_power * (T_set - T_on) / (T_set - T_on) +
	sustainable_power = 2 * sustainable_power + sustainable_power =
	3 * sustainable_power
```
鍥犳锛屼粎姣斾緥椤瑰氨浼氶殢鐫€娓╁害浠庡紑鍚俯搴﹀崌楂樺埌鏈熸湜娓╁害锛屽皢鍔熺巼浠?3 * `sustainable_power`
绾挎€ч檷浣庡埌 `sustainable_power`銆?

### k_i 涓?integral_cutoff


`k_i` 閰嶇疆 PID 寰幆鐨勭Н鍒嗛」甯告暟銆傝繖涓€椤逛娇 PID 鎺у埗鍣ㄨ兘澶熻ˉ鍋块暱鏈熸紓绉伙紝浠ュ強杈撳嚭鐨?閲忓寲鐗规€э細鍐峰嵈璁惧鏃犳硶璁剧疆璋冭妭鍣ㄦ墍璇锋眰鐨勭簿纭姛鐜囥€傚綋娓╁害璇樊浣庝簬 `integral_cutoff`
鏃讹紝璇樊琚疮鍔犺繘绉垎椤广€傝繖涓€椤归殢鍚庝箻浠?`k_i`锛屽叾缁撴灉琚姞鍒版帶鍒跺櫒鐨勮緭鍑轰腑銆傞€氬父
`k_i` 璁惧緱杈冧綆锛? 鎴?2锛夛紝鑰?`integral_cutoff` 涓?0銆?

### k_d


`k_d` 閰嶇疆 PID 寰幆鐨勫鏁伴」甯告暟銆傚缓璁繚鎸侀粯璁ゅ€硷細0銆?

## 鍐峰嵈璁惧鍔熺巼 API


鐢辫璋冭妭鍣ㄦ帶鍒剁殑鍐峰嵈璁惧蹇呴』鍦ㄥ叾 `cooling_device_ops` 涓彁渚涢澶栫殑鈥減ower鈥?API銆?瀹冪敱涓変釜鎿嶄綔缁勬垚锛?
```
    int get_requested_power(struct thermal_cooling_device *cdev,
			    struct thermal_zone_device *tz, u32 *power);
```
@cdev:
	`struct thermal_cooling_device` 鎸囬拡
@tz:
	褰撳墠鎵€澶勭殑鐑尯
@power:
	鐢ㄤ簬瀛樻斁璁＄畻鎵€寰楀姛鐜囩殑鎸囬拡

`get_requested_power()` 璁＄畻璁惧鎵€璇锋眰鐨勫姛鐜囷紙鍗曚綅姣摝锛夊苟瀛樺叆 @power銆傛垚鍔熸椂杩斿洖
0锛屽け璐ユ椂杩斿洖 -E*銆傜洰鍓?power allocator 璋冭妭鍣ㄧ敤瀹冮€氳繃璁＄畻瑕佺粰姣忎釜鍐峰嵈璁惧鍒嗛厤澶氬皯
鍔熺巼銆?
```
	int state2power(struct thermal_cooling_device *cdev, struct
			thermal_zone_device *tz, unsigned long state,
			u32 *power);
```
@cdev:
	`struct thermal_cooling_device` 鎸囬拡
@tz:
	褰撳墠鎵€澶勭殑鐑尯
@state:
	涓€涓喎鍗磋澶囩姸鎬?@power:
	鐢ㄤ簬瀛樻斁绛夋晥鍔熺巼鐨勬寚閽?
鎶婂喎鍗磋澶囩姸鎬?@state 杞崲涓哄姛鑰楋紙姣摝锛夊苟瀛樺叆 @power銆傛垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖
-E*銆傜洰鍓?thermal core 鐢ㄥ畠閫氳繃璁＄畻涓€涓墽琛屼綋锛坅ctor锛夎兘澶熸秷鑰楃殑鏈€澶у姛鐜囥€?
```
	int power2state(struct thermal_cooling_device *cdev, u32 power,
			unsigned long *state);
```
@cdev:
	`struct thermal_cooling_device` 鎸囬拡
@power:
	鍔熺巼锛堟鐡︼級
@state:
	鐢ㄤ簬瀛樻斁鎵€寰楃姸鎬佺殑鎸囬拡

璁＄畻涓€涓喎鍗磋澶囩姸鎬侊紝浣胯璁惧鏈€澶氭秷鑰?@power 姣摝锛屽苟瀛樺叆 @state銆傛垚鍔熸椂杩斿洖 0锛?澶辫触鏃惰繑鍥?-E*銆傜洰鍓?thermal core 鐢ㄥ畠閫氳繃鎶?power allocator 璋冭妭鍣ㄨ瀹氱殑鏌愪釜鍔熺巼
杞崲涓哄喎鍗磋澶囪兘澶熻缃殑鐘舵€併€傚畠鏄竴涓嚱鏁帮紝鍥犱负杩欑杞崲鍙兘渚濊禆浜庡彲鑳藉彂鐢熷彉鍖栫殑
澶栭儴鍥犵礌锛屽洜姝よ鍑芥暟搴斿綋鍦ㄢ€滃綋鍓嶆儏鍐碘€濅笅缁欏嚭鏈€浣宠浆鎹€?

### 鍐峰嵈璁惧鏉冮噸


鏉冮噸鏄竴绉嶅湪鍐峰嵈璁惧涔嬮棿鍋忕疆鍒嗛厤鐨勬満鍒躲€傚畠浠〃杈句簡涓嶅悓鍐峰嵈璁惧鐨勭浉瀵瑰姛鐜囨晥鐜囥€?鍙互鐢ㄨ緝楂樼殑鏉冮噸鏉ヨ〃杈捐緝楂樼殑鍔熺巼鏁堢巼銆傛潈閲嶆槸鐩稿鐨勶紝濡傛灉姣忎釜鍐峰嵈璁惧鐨勬潈閲嶉兘鏄?1锛屽垯璁や负瀹冧滑鐩哥瓑銆傝繖鍦ㄥ紓鏋勭郴缁熶腑灏ゅ叾鏈夌敤锛屼緥濡備袱涓喎鍗磋澶囧彲鑳芥墽琛屽悓绫昏绠楋紝浣?鏁堢巼涓嶅悓銆備緥濡備竴涓嫢鏈変袱绉嶄笉鍚岀被鍨嬪鐞嗗櫒鐨勭郴缁熴€?
濡傛灉鐑尯鏄€氳繃 `thermal_zone_device_register()`锛堝嵆骞冲彴浠ｇ爜锛夋敞鍐岀殑锛岄偅涔堟潈閲?浣滀负鐑尯鐨?`thermal_bind_parameters` 鐨勪竴閮ㄥ垎浼犲叆銆傚鏋滃钩鍙版槸閫氳繃璁惧鏍戞敞鍐岀殑锛?閭ｄ箞瀹冧滑浣滀负 `cooling-maps` 鑺傜偣涓瘡涓?map 鐨?`contribution` 灞炴€т紶鍏ャ€?

## power allocator 璋冭妭鍣ㄧ殑灞€闄愭€?

power allocator 璋冭妭鍣ㄧ殑 PID 鎺у埗鍣ㄥ湪瀛樺湪鍛ㄦ湡鎬?tick 鏃跺伐浣滄渶浣炽€傚鏋滀綘鏈変竴涓?椹卞姩鍙嶅璋冪敤 `thermal_zone_device_update()`锛堟垨浠讳綍鏈€缁堜細璋冪敤璋冭妭鍣?`throttle()`
鍑芥暟鐨勪笢瑗匡級锛岃皟鑺傚櫒鐨勫搷搴斿氨涓嶄細寰堝ソ銆傛敞鎰忥紝杩欏苟闈炶璋冭妭鍣ㄧ壒鏈夆€斺€攕tep-wise 璋冭妭鍣?涔熸槸濡傛锛屽鏋滀綘姣旀甯哥殑 thermal 妗嗘灦 tick 鏇撮绻佸湴璋冪敤瀹冪殑 throttle()锛堜緥濡傜敱浜?涓柇锛夛紝瀹冧篃浼氳涓哄紓甯革紝鍥犱负瀹冧細鍙嶅簲杩囧害銆?

## Energy Model 瑕佹眰


鍙︿竴浠堕噸瑕佺殑浜嬫儏鏄喎鍗磋澶囨墍鎻愪緵鐨勫姛鐜囧€兼爣搴﹁涓€鑷淬€傚崟涓儹鍖轰腑鐨勬墍鏈夊喎鍗磋澶囷紝鍏?鍔熺巼鍊煎簲褰撹涔堜互姣摝鎶ュ憡锛岃涔堢缉鏀惧埌鐩稿悓鐨勨€滄娊璞℃爣搴︹€濄€?