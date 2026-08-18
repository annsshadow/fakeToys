## GPIO 椹卞姩鎺ュ彛


鏈枃妗ｄ綔涓虹紪鍐?GPIO 鑺墖椹卞姩鐨勫紑鍙戣€呯殑鎸囧崡銆?
姣忎釜 GPIO 鎺у埗鍣ㄩ┍鍔ㄩ兘闇€瑕佸寘鍚互涓嬪ご鏂囦欢锛屽畠瀹氫箟浜?```

  #include <linux/gpio/driver.h>


```
## GPIO 鐨勫唴閮ㄨ〃绀?

涓€涓?GPIO 鑺墖澶勭悊涓€鏉℃垨澶氭潯 GPIO 绾裤€傝琚涓?GPIO 鑺墖锛岃繖浜涚嚎蹇呴』绗﹀悎瀹氫箟锛氶€氱敤杈撳叆/杈撳嚭锛圙eneral Purpose Input/Output锛夈€傚鏋滆绾垮苟闈為€氱敤鐢ㄩ€旓紝閭ｄ箞瀹冨氨涓嶆槸 GPIO锛屼篃涓嶅簲鐢?GPIO 鑺墖鏉ュ鐞嗐€傜敤渚嬪叿鏈夋寚绀烘€э細绯荤粺涓煇浜涚嚎鍙兘琚О涓?GPIO锛屽嵈鏈嶅姟浜庨潪甯哥壒瀹氱殑鐢ㄩ€旓紝鍥犳涓嶆弧瓒抽€氱敤 I/O 鐨勫垽鎹€傚彟涓€鏂归潰锛孡ED 椹卞姩鐨勪竴鏉＄嚎鍙兘琚綋浣?GPIO 浣跨敤锛屽洜姝や粛搴旂敱 GPIO 鑺墖椹卞姩鏉ュ鐞嗐€?
鍦?GPIO 椹卞姩鍐呴儴锛屾瘡鏉?GPIO 绾跨敱鍏剁‖浠剁紪鍙锋爣璇嗭紝鏈夋椂涔熺О涓?`offset`锛岃繖鏄竴涓粙浜?0 涓?n-1 涔嬮棿鐨勫敮涓€缂栧彿锛屽叾涓?n 鏄姱鐗囩鐞嗙殑 GPIO 鏁伴噺銆?
纭欢 GPIO 缂栧彿搴斿綋瀵逛簬纭欢鑰岃█鏄洿瑙傜殑锛屼緥濡傦紝濡傛灉鏌愪釜绯荤粺浣跨敤涓€缁勫唴瀛樻槧灏勭殑 I/O 瀵勫瓨鍣紝鍏朵腑 32 鏉?GPIO 绾垮湪 32 浣嶅瘎瀛樺櫒涓敱姣忎綅瀵瑰簲涓€鏉＄嚎鏉ュ鐞嗭紝閭ｄ箞瀵硅繖浜涚嚎浣跨敤纭欢鍋忕Щ 0..31 鏄悎鐞嗙殑锛屽畠浠垎鍒搴斿瘎瀛樺櫒涓殑浣?0..31銆?
杩欎釜缂栧彿绾补鏄唴閮ㄧ殑锛氱壒瀹?GPIO 绾跨殑纭欢缂栧彿姘歌繙涓嶄細鍦ㄩ┍鍔ㄤ箣澶栧彲瑙併€?
鍦ㄨ繖涓唴閮ㄧ紪鍙蜂箣涓婏紝姣忔潯 GPIO 绾胯繕闇€瑕佸湪鏁存暟 GPIO 鍛藉悕绌洪棿涓嫢鏈変竴涓叏灞€缂栧彿锛屼互渚胯兘澶熶笌浼犵粺鐨?GPIO 鎺ュ彛涓€璧蜂娇鐢ㄣ€傚洜姝ゆ瘡涓姱鐗囧繀椤绘湁涓€涓€渂ase鈥濈紪鍙凤紙鍙互鑷姩鍒嗛厤锛夛紝鑰屽浜庢瘡鏉?GPIO 绾匡紝鍏跺叏灞€缂栧彿灏嗘槸锛坆ase + 纭欢缂栧彿锛夈€傚敖绠℃暣鏁拌〃绀烘硶琚涓哄凡搴熷純锛屼絾瀹冧粛鏈夎澶氫娇鐢ㄨ€咃紝鍥犳闇€瑕佺户缁淮鎶ゃ€?
渚嬪锛屾煇涓钩鍙板彲浠ュ GPIO 浣跨敤鍏ㄥ眬缂栧彿 32-159锛屽叾涓竴涓帶鍒跺櫒鍦ㄢ€渂ase鈥濅负 32 鐨勪綅缃畾涔変簡 128 鏉?GPIO锛涜€屽彟涓€涓钩鍙颁娇鐢ㄥ叏灞€缂栧彿 0..63 閰嶅悎涓€缁?GPIO 鎺у埗鍣ㄣ€?4-79 閰嶅悎鍙︿竴绉嶇被鍨嬬殑 GPIO 鎺у埗鍣紝鑰屽湪鏌愬潡鐗瑰畾鏉垮崱涓?80-95 閰嶅悎涓€涓?FPGA銆備紶缁熺紪鍙锋棤闇€杩炵画锛涜繖涓や釜骞冲彴涓殑浠讳綍涓€涓篃閮藉彲浠ヤ娇鐢ㄧ紪鍙?2000-2063 鏉ユ爣璇?I2C GPIO 鎵╁睍鍣ㄧ粍涓殑 GPIO 绾裤€?

## 鎺у埗鍣ㄩ┍鍔細gpio_chip


鍦?gpiolib 妗嗘灦涓紝姣忎釜 GPIO 鎺у埗鍣ㄨ灏佽涓轰竴涓€渟truct gpio_chip鈥濓紙瀹屾暣瀹氫箟瑙?<linux/gpio/driver.h>锛夛紝鍏朵腑鍖呭惈璇ョ被鍨嬫瘡涓帶鍒跺櫒鍏辨湁鐨勬垚鍛橈紝杩欎簺搴旂敱椹卞姩浠ｇ爜璧嬪€硷細

 - 鐢ㄤ簬纭畾 GPIO 绾挎柟鍚戠殑鏂规硶
 - 鐢ㄤ簬璁块棶 GPIO 绾垮€肩殑鏂规硶
 - 鐢ㄤ簬璁剧疆缁欏畾 GPIO 绾跨數姘旈厤缃殑鏂规硶
 - 鐢ㄤ簬杩斿洖涓庣粰瀹?GPIO 绾垮叧鑱旂殑 IRQ 缂栧彿鐨勬柟娉? - 鎸囩ず瀵瑰叾鏂规硶鐨勮皟鐢ㄦ槸鍚﹀彲鑳戒紤鐪犵殑鏍囧織
 - 鍙€夌殑銆佺敤浜庢爣璇嗗悇绾跨殑绾垮悕绉版暟缁? - 鍙€夌殑 debugfs dump 鏂规硶锛堟樉绀洪澶栫殑鐘舵€佷俊鎭級
 - 鍙€夌殑 base 缂栧彿锛堣嫢鐪佺暐鍒欒嚜鍔ㄥ垎閰嶏級
 - 鍙€夌殑銆佺敤浜庤瘖鏂拰鍊熷姪骞冲彴鏁版嵁杩涜 GPIO 鑺墖鏄犲皠鐨?label

瀹炵幇 gpio_chip 鐨勪唬鐮佸簲褰撴敮鎸佹帶鍒跺櫒鐨勫涓疄渚嬶紝鏈€濂戒娇鐢ㄩ┍鍔ㄦā鍨嬨€傝浠ｇ爜浼氶厤缃瘡涓?gpio_chip 骞跺彂鍑?gpiochip_add_data() 鎴?devm_gpiochip_add_data()銆傜Щ闄?GPIO 鎺у埗鍣ㄥ簲褰撳緢灏戣锛涘彧鏈夊湪涓嶅彲閬垮厤鏃舵墠浣跨敤 gpiochip_remove()銆?
gpio_chip 甯稿父鏄睘浜庢煇涓疄渚嬬壒瀹氱粨鏋勭殑涓€閮ㄥ垎锛岃缁撴瀯鍚湁 GPIO 鎺ュ彛鏈毚闇茬殑鐘舵€侊紝渚嬪瀵诲潃銆佺數婧愮鐞嗙瓑銆傚儚闊抽缂栬В鐮佸櫒杩欐牱鐨勮姱鐗囧氨浼氭嫢鏈夊鏉傜殑闈?GPIO 鐘舵€併€?
浠讳綍 debugfs dump 鏂规硶閫氬父搴斿綋蹇界暐灏氭湭琚姹傜殑绾裤€傚畠浠彲浠ヤ娇鐢?gpiochip_is_requested()锛岃鍑芥暟瑕佷箞杩斿洖 NULL锛岃涔堣繑鍥炶姹傝 GPIO 绾挎椂鍏宠仈鐨?label銆?
瀹炴椂锛圧ealtime锛夎€冮噺锛氬鏋滈鏈熻鍦ㄥ疄鏃讹紙realtime锛夊唴鏍镐笂浠庡師瀛愪笂涓嬫枃锛堢‖ IRQ 澶勭悊鍑芥暟鍙婄被浼间笂涓嬫枃涓級璋冪敤 GPIO API锛岄偅涔?GPIO 椹卞姩涓嶅簲鍦ㄥ叾 gpio_chip 瀹炵幇锛?get/.set 浠ュ強鏂瑰悜鎺у埗鍥炶皟锛変腑浣跨敤 spinlock_t 鎴栦换浣曞彲浼戠湢鐨?API锛堝 PM runtime锛夈€傞€氬父杩欏苟闈炲繀闇€銆?

### GPIO 鐢靛钩璇箟


gpiolib 鐨?.get/set[_multiple]() 绾垮€艰闄愬埗鍦ㄥ竷灏旂┖闂?[0, 1]锛屽嵆浣庣數骞虫垨楂樼數骞炽€?
浣庣數骞充笌楂樼數骞冲畾涔変负杩炴帴鍒拌繛鎺ュ櫒锛堝鐗╃悊鐒婄洏銆佸紩鑴氭垨鐢垫簮杞級鐨勭嚎涓婄殑鐗╃悊浣庣數骞?楂樼數骞炽€?
GPIO 搴撳叿鏈夊唴閮ㄩ€昏緫鏉ュ鐞嗕綆鐢靛钩鏈夋晥锛坅ctive low锛夌殑绾匡紝渚嬪鍘熺悊鍥句腑浠ュ垹闄ょ嚎鎴?#name 鏍囨敞鐨勭嚎锛岄┍鍔ㄤ笉搴旇瘯鍥惧幓鐚滄祴涓€鏉＄嚎鐨勯€昏緫鍊笺€?
娑堣垂鑰呭鐞?GPIO 鍊肩殑鏂瑰紡鏄紝搴撳悜娑堣垂鑰呭憟鐜?*閫昏緫锛坙ogical锛?*鍊笺€備竴鏉＄嚎鍦ㄥ叾**閫昏緫**鍊间负 1 鏃惰瑙嗕负**鏈夋晥锛坅sserted锛?*锛屽湪鍏堕€昏緫鍊间负 0 鏃惰瑙嗕负**鏃犳晥锛坉e-asserted锛?*銆傚鏋滈渶瑕佸弽杞紝杩欑敱 gpiolib 澶勭悊锛屽苟鍊熷姪纭欢鎻忚堪锛堝璁惧鏍戞垨 ACPI锛夎繘琛岄厤缃紝杩欎簺鎻忚堪鑳藉鏄庣‘鎸囧嚭涓€鏉＄嚎鏄珮鐢靛钩鏈夋晥杩樻槸浣庣數骞虫湁鏁堛€?
鐢变簬鐢靛瓙璁惧閫氬父浼氬湪 GPIO 绾垮墠闈㈡彃鍏ュ弽鐩稿櫒浣滀负椹卞姩绾ф垨淇濇姢缂撳啿鍣紝鍥犳杩欑璇箟蹇呴』鏄‖浠舵弿杩扮殑涓€閮ㄥ垎锛岃繖鏍蜂竴鏉ユ秷璐硅€咃紙濡傚唴鏍搁┍鍔級灏辨棤闇€涓烘鎷呭咖锛屼緥濡傚嵆浣挎煇鏉?RESET 绾垮湪鐗╃悊涓婃槸浣庣數骞虫湁鏁堬紝涔熷彲浠ュ皢鍏惰涓洪€昏緫 1 鏉ヤ娇鍏舵湁鏁堛€?

### GPIO 鐢垫皵閰嶇疆


GPIO 绾垮彲浠ラ€氳繃 .set_config() 鍥炶皟閰嶇疆涓哄绉嶇數姘斿伐浣滄ā寮忋€傜洰鍓嶈 API 鏀寔璁剧疆锛?
- 鍘绘姈锛圖ebouncing锛?- 鍗曠妯″紡锛坥pen drain/open source锛屽紑婕?寮€婧愶級
- 涓婃媺涓庝笅鎷夌數闃讳娇鑳?
浠ヤ笅瀵硅繖浜涜缃繘琛岃鏄庛€?
.set_config() 鍥炶皟浣跨敤涓庨€氱敤寮曡剼鎺у埗锛坧in control锛夐┍鍔ㄧ浉鍚岀殑鏋氫妇閲忎笌閰嶇疆璇箟銆傝繖骞堕潪宸у悎锛氬彲浠ュ皢 .set_config() 鎸囧畾涓哄嚱鏁?gpiochip_generic_config()锛岃繖浼氬鑷磋皟鐢?pinctrl_gpio_set_config()锛屽苟鏈€缁堣惤鍒?GPIO 鎺у埗鍣ㄢ€滆儗鍚庘€濈殑寮曡剼鎺у埗鍚庣锛岄€氬父鏇撮潬杩戝疄闄呭紩鑴氥€傝繖鏍凤紝寮曡剼鎺у埗鍣ㄥ氨鍙互绠＄悊涓嬮潰鍒楀嚭鐨?GPIO 閰嶇疆銆?
濡傛灉浣跨敤浜嗗紩鑴氭帶鍒跺櫒鍚庣锛孏PIO 鎺у埗鍣ㄦ垨纭欢鎻忚堪闇€瑕佹彁渚涒€淕PIO ranges鈥濓紝灏?GPIO 绾垮亸绉绘槧灏勫埌寮曡剼鎺у埗鍣ㄤ笂鐨勫紩鑴氱紪鍙凤紝浠ヤ究瀹冧滑鑳藉姝ｇ‘鍦扮浉浜掍氦鍙夊紩鐢ㄣ€?

### 鏀寔鍘绘姈鐨?GPIO 绾?

鍘绘姈锛圖ebouncing锛夋槸涓€绉嶄负寮曡剼璁剧疆鐨勯厤缃紝琛ㄦ槑瀹冭繛鎺ュ埌浜嗗彲鑳戒細浜х敓鎶栧姩鐨勬満姊板紑鍏虫垨鎸夐挳绛夈€傛姈鍔ㄦ槸鎸囩敱浜庢満姊板師鍥狅紝绾垮湪鏋佺煭闂撮殧鍐呰蹇€熸媺楂?鎷変綆銆傝繖浼氬鑷存暟鍊间笉绋冲畾鎴?IRQ 鍙嶅瑙﹀彂锛岄櫎闈炶绾胯鍘绘姈銆?
瀹炶返涓紝鍘绘姈鐨勫仛娉曟槸锛氬綋绾夸笂鍙戠敓鏌愪簨浠舵椂璁剧疆涓€涓畾鏃跺櫒锛岀◢绛夌墖鍒诲悗鍐嶆閲囨牱璇ョ嚎锛岀湅瀹冩槸鍚︿粛鍏锋湁鐩稿悓鐨勫€硷紙浣庢垨楂橈級銆傝繖涔熷彲浠ョ敱涓€涓阀濡欑殑鐘舵€佹満閲嶅杩涜锛岀瓑寰呰绾垮彉寰楃ǔ瀹氥€傛棤璁哄摢绉嶆儏鍐碉紝瀹冮兘浼氫负鍘绘姈璁剧疆涓€涓‘瀹氱殑姣鏁帮紝鎴栬€呭鏋滆鏃堕棿涓嶅彲閰嶇疆锛屽垯绠€鍗曞湴璁句负鈥滃紑/鍏斥€濄€?

### 鏀寔寮€婕?寮€婧愮殑 GPIO 绾?

寮€婕忥紙open drain锛孋MOS锛夋垨寮€闆嗭紙open collector锛孴TL锛夋剰鍛崇潃璇ョ嚎涓嶄細琚富鍔ㄩ┍鍔ㄤ负楂樼數骞筹細鐩稿弽锛屼綘鎶婃紡鏋?闆嗙數鏋佷綔涓鸿緭鍑猴紝鍥犳褰撴櫠浣撶
```



   CMOS CONFIGURATION      TTL CONFIGURATION

            ||--- out              +--- out
     in ----||                   |/
            ||--+         in ----|
                |                |\
               GND                 GND

```
杩欑閰嶇疆閫氬父鐢ㄦ潵瀹炵幇浠ヤ笅涓や欢浜嬩箣涓€锛?
- 鐢靛钩杞崲锛圠evel-shifting锛夛細杈惧埌楂樹簬杈撳嚭鎵€鍦ㄧ鐗囩殑閫昏緫鐢靛钩銆?- 鍦?I/O 绾匡紙渚嬪 GPIO 绾匡級涓婄殑鍙嶅悜绾夸笌锛坵ire-OR锛夛紝浣垮緱绾夸笂浠讳綍椹卞姩绾ч兘鍙互鎶婂畠鎷変綆锛屽嵆浣垮悓涓€鏍圭嚎鐨勪换浣曞叾浠栬緭鍑哄悓鏃舵妸瀹冮┍鍔ㄤ负楂樸€備竴涓壒渚嬫槸椹卞姩 I2C 鎬荤嚎鐨?SCL 鍜?SDA 绾匡紝鎸夊叾瀹氫箟锛屽畠灏辨槸涓€涓嚎涓庯紙wire-OR锛夋€荤嚎銆?
杩欎袱绉嶇敤渚嬮兘瑕佹眰璇ョ嚎閰嶅涓婃媺鐢甸樆銆傝鐢甸樆浼氫娇绾垮€惧悜浜庨珮鐢靛钩锛岄櫎闈炶建涓婄殑鏌愪釜鏅朵綋绠′富鍔ㄥ皢鍏舵媺浣庛€傜嚎涓婄殑鐢靛钩浼氬崌鍒颁笂鎷夌數闃荤殑 VDD 閭ｄ箞楂橈紝鑰岃 VDD 鍙兘楂樹簬鏅朵綋绠℃墍鏀寔鐨勭數骞筹紝浠庤€屽疄鐜板悜鏇撮珮 VDD 鐨勭數骞宠浆鎹€?
闆嗘垚鐢靛瓙鍣ㄤ欢閫氬父鍏锋湁 CMOS鈥滃浘鑵炬煴锛坱otem-pole锛夆€濆舰寮忕殑杈撳嚭椹卞姩绾э紝鍖呭惈涓€涓?N-MOS 鍜屼竴涓?P-MOS 鏅朵綋绠★紝鍏朵腑涓€涓皢绾块┍鍔ㄤ负楂橈紝鍙︿竴涓皢绾块┍鍔ㄤ负浣庛€傝繖琚О涓烘帹鎸斤紙push-pull锛?```

                     VDD
                      |
            OD    ||--+
         +--/ ---o||     P-MOS-FET
         |        ||--+
    IN --+            +----- out
         |        ||--+
         +--/ ----||     N-MOS-FET
            OS    ||--+
                      |
                     GND

```
鎵€闇€鐨勮緭鍑轰俊鍙凤紙渚嬪鐩存帴鏉ヨ嚜鏌愪釜 GPIO 杈撳嚭瀵勫瓨鍣級鍒拌揪 IN銆傚悕涓衡€淥D鈥濆拰鈥淥S鈥濈殑寮€鍏抽€氬父鏄棴鍚堢殑锛屼粠鑰屾瀯鎴愭帹鎸界數璺€?
鑰冭檻鍚嶄负鈥淥D鈥濆拰鈥淥S鈥濈殑灏忊€滃紑鍏斥€濓紝瀹冧滑鍦ㄨ緭鍏ュ垎鍙変箣鍚庡惎鐢?绂佺敤 P-MOS 鎴?N-MOS 鏅朵綋绠°€傚浣犳墍瑙侊紝濡傛灉姝ゅ紑鍏虫柇寮€锛屼换涓€涓櫠浣撶閮戒細瀹屽叏澶辨晥銆傚浘鑵炬煴浜庢槸琚噺鍗婏紝骞剁粰鍑洪珮闃绘€侊紝鑰岄潪鍒嗗埆涓诲姩灏嗙嚎椹卞姩涓洪珮鎴栦綆銆傝繖閫氬父鏄蒋浠舵帶鍒剁殑寮€婕?寮€婧愮殑宸ヤ綔鏂瑰紡銆?
涓€浜?GPIO 纭欢浠ュ紑婕?寮€婧愰厤缃嚭鐜般€傛湁浜涙槸纭繛绾匡紙hard-wired锛夌殑绾匡紝鏃犺濡備綍閮藉彧鏀寔寮€婕忔垨寮€婧愶細閭ｉ噷鍙湁涓€涓櫠浣撶銆傛湁浜涙槸鍙蒋浠堕厤缃殑锛氶€氳繃缈昏浆瀵勫瓨鍣ㄤ腑鐨勬煇涓€浣嶏紝杈撳嚭鍙互琚厤缃负寮€婕忔垨寮€婧愶紝瀹為檯涓婂氨鏄€氳繃鎷ㄥ紑涓婂浘涓爣娉ㄤ负鈥淥D鈥濆拰鈥淥S鈥濈殑寮€鍏虫潵瀹炵幇銆傞€氳繃绂佺敤 P-MOS 鏅朵綋绠★紝杈撳嚭鍙互鍦?GND 涓庨珮闃绘€佷箣闂磋椹卞姩锛堝紑婕忥級锛涢€氳繃绂佺敤 N-MOS 鏅朵綋绠★紝杈撳嚭鍙互鍦?VDD 涓庨珮闃绘€佷箣闂磋椹卞姩锛堝紑婧愶級銆傜涓€绉嶆儏鍐甸渶瑕佸湪杈撳嚭杞ㄤ笂閰嶅涓婃媺鐢甸樆浠ュ畬鎴愮數璺紝绗簩绉嶆儏鍐靛垯闇€瑕佸湪杞ㄤ笂閰嶅涓嬫媺鐢甸樆銆傛敮鎸佸紑婕忋€佸紑婧愭垨涓よ€呯殕鏀寔鐨勭‖浠讹紝鍙互鍦?gpio_chip 涓疄鐜颁竴涓壒娈婄殑鍥炶皟锛?set_config()锛屽畠鎺ュ彈涓€涓€氱敤鐨?pinconf 鎵撳寘鍊硷紝鎸囨槑鏄皢绾块厤缃负寮€婕忋€佸紑婧愯繕鏄帹鎸姐€傝繖浼氬湪鍝嶅簲 machine 鏂囦欢涓缃殑 GPIO_OPEN_DRAIN 鎴?GPIO_OPEN_SOURCE 鏍囧織鏃跺彂鐢燂紝涔熷彲鏉ヨ嚜鍏朵粬纭欢鎻忚堪銆?
濡傛灉杩欑鐘舵€佹棤娉曞湪纭欢涓厤缃紝鍗冲鏋?GPIO 纭欢涓嶆敮鎸佺‖浠跺眰闈㈢殑寮€婕?寮€婧愶紝GPIO 搴撲細鏀圭敤涓€绉嶆妧宸э細褰撲竴鏉＄嚎琚涓鸿緭鍑烘椂锛屽鏋滆绾胯鏍囪涓哄紑婕忥紝涓?IN 杈撳嚭鍊间负浣庯紝瀹冧細鍍忓線甯镐竴鏍疯椹卞姩涓轰綆銆備絾濡傛灉 IN 杈撳嚭鍊艰璁句负楂橈紝瀹?*涓嶄細**琚┍鍔ㄤ负楂橈紝鑰屾槸浼氳鍒囨崲鍒拌緭鍏ユā寮忥紝鍥犱负杈撳叆妯″紡绛変环浜庨珮闃绘€侊紝浠庤€屽疄鐜版煇绉嶁€滃紑婕忎豢鐪燂紙open drain emulation锛夆€濓細鍦ㄧ數姘旇涓轰笂浜岃€呯浉鍚岋紝鍞竴鐨勪緥澶栨槸鍦ㄥ垏鎹㈢嚎鐨勬ā寮忔椂鍙兘鍑虹幇纭欢姣涘埡銆?
瀵逛簬寮€婧愰厤缃紝浣跨敤鐩稿悓鐨勫師鐞嗭紝鍙槸瀹冨苟闈炰富鍔ㄥ皢绾块┍鍔ㄤ负浣庯紝鑰屾槸灏嗗叾璁句负杈撳叆銆?

### 鏀寔涓婃媺/涓嬫媺鐢甸樆鐨?GPIO 绾?

GPIO 绾垮彲浠ラ€氳繃 .set_config() 鍥炶皟鏀寔涓婃媺/涓嬫媺銆傝繖鎰忓懗鐫€ GPIO 绾胯緭鍑虹閰嶅鏈変笂鎷夋垨涓嬫媺鐢甸樆锛屼笖璇ョ數闃荤敱杞欢鎺у埗銆?
鍦ㄥ垎绔嬶紙discrete锛夎璁′腑锛屼笂鎷夋垨涓嬫媺鐢甸樆鐩存帴鐒婃帴鍦ㄧ數璺澘涓娿€傝繖涓嶆槸鎴戜滑鍦ㄨ蒋浠朵腑澶勭悊鎴栧缓妯＄殑涓滆タ銆備綘瀵硅繖浜涚嚎鏈€澶氬彧浼氭兂鍒板畠浠緢鍙兘琚厤缃负寮€婕忔垨寮€婧愶紙瑙佷笂涓€鑺傦級銆?
.set_config() 鍥炶皟鍙兘寮€鍚垨鍏抽棴涓婃媺/涓嬫媺锛岃€屼笉浼氬鎵€浣跨敤鐢甸樆鐨勯樆鍊兼湁浠讳綍璇箟灞傞潰鐨勪簡瑙ｃ€傚畠鍙細鍒囨崲瀵勫瓨鍣ㄤ腑鐨勬煇涓€浣嶏紝浠ュ惎鐢ㄦ垨绂佺敤涓婃媺/涓嬫媺銆?
濡傛灉 GPIO 绾挎敮鎸佷互涓嶅悓鐨勯樆鍊煎涓婃媺鎴栦笅鎷夌數闃昏繘琛屽垎娴侊紙shunting锛夛紝閭ｄ箞 GPIO 鑺墖鍥炶皟 .set_config() 灏变笉澶熺敤浜嗐€傚浜庤繖浜涘鏉傜敤渚嬶紝闇€瑕佸疄鐜?GPIO 鑺墖涓庡紩鑴氭帶鍒跺櫒鐨勭粍鍚堬紝鍥犱负寮曡剼鎺у埗鍣ㄧ殑寮曡剼閰嶇疆鎺ュ彛鏀寔瀵圭數姘斿睘鎬ц繘琛屾洿鐏垫椿鐨勬帶鍒讹紝骞惰兘澶勭悊涓嶅悓鐨勪笂鎷夋垨涓嬫媺闃诲€笺€?

## 鎻愪緵 IRQ 鐨?GPIO 椹卞姩


GPIO 椹卞姩锛圙PIO 鑺墖锛夊悓鏃舵彁渚涗腑鏂槸涓€绉嶆儻渚嬶紝鏈€甯歌鐨勬槸绾ц仈锛坈ascaded锛夎嚜涓€涓埗涓柇鎺у埗鍣紝鑰屽湪鏌愪簺鐗规畩鎯呭喌涓嬶紝GPIO 閫昏緫浼氫笌 SoC 鐨勪富涓柇鎺у埗鍣ㄨ瀺鍚堝湪涓€璧枫€侴PIO 鍧楃殑 IRQ 閮ㄥ垎浣跨敤 irq_chip 瀹炵幇锛岀敤鍒板ご鏂囦欢 <linux/irq.h>銆傚洜姝よ繖绉嶇粍鍚堥┍鍔ㄥ悓鏃跺埄鐢ㄤ簡涓や釜瀛愮郴缁燂細gpio 鍜?irq銆?
浠讳綍 IRQ 娑堣垂鑰呴兘鍚堟硶鍦颁粠浠讳綍 irqchip 璇锋眰 IRQ锛屽嵆浣垮畠鏄竴涓粍鍚堢殑 GPIO+IRQ 椹卞姩銆傚熀鏈墠鎻愭槸 gpio_chip 鍜?irq_chip 鏄浜ょ殑锛屽郊姝ょ嫭绔嬪湴鎻愪緵鏈嶅姟銆俫piod_to_irq() 鍙槸涓€涓负浜嗘柟渚胯€屾壘鍑烘煇鏉?GPIO 绾垮搴旂殑 IRQ 鐨勫嚱鏁帮紝涓嶅簲渚濊禆瀹冨湪 IRQ 琚娇鐢ㄤ箣鍓嶅凡琚皟鐢ㄣ€傚缁堝湪鏉ヨ嚜 GPIO 鍜?irq_chip API 鐨勫悇鑷洖璋冧腑鍑嗗濂界‖浠跺苟浣垮叾灏辩华銆備笉瑕佷緷璧?gpiod_to_irq() 宸茶棣栧厛璋冪敤銆?
鎴戜滑鍙互灏?GPIO irqchip 澶ц嚧鍒嗕负涓ょ被锛?
- 绾ц仈涓柇鑺墖锛圕ASCADED INTERRUPT CHIPS锛夛細杩欐剰鍛崇潃 GPIO 鑺墖鏈変竴鏉″叕鍏辩殑涓柇杈撳嚭绾匡紝瀹冪敱璇ヨ姱鐗囦笂浠讳綍宸蹭娇鑳界殑 GPIO 绾胯Е鍙戙€傝繖鏉′腑鏂緭鍑虹嚎闅忓悗浼氳璺敱鍒颁笂涓€绾х殑鐖朵腑鏂帶鍒跺櫒锛屽湪鏈€绠€鍗曠殑鎯呭喌涓嬪氨鏄郴缁熺殑涓讳腑鏂帶鍒跺櫒銆傝繖鐢变竴涓?irqchip 寤烘ā锛屽畠浼氭鏌?GPIO 鎺у埗鍣ㄥ唴閮ㄧ殑浣嶏紝浠ュ垽鏂槸鍝潯绾胯Е鍙戜簡瀹冦€傞┍鍔ㄤ腑鐨?irqchip 閮ㄥ垎闇€瑕佹鏌ュ瘎瀛樺櫒鏉ュ垽鏂繖涓€鐐癸紝骞朵笖寰堝彲鑳借繕闇€瑕侀€氳繃娓呴櫎鏌愪釜浣嶏紙鏈夋椂鏄殣寮忓湴锛屼粎閫氳繃璇诲彇鐘舵€佸瘎瀛樺櫒锛夋潵纭瀹冩鍦ㄥ鐞嗚涓柇锛屽苟涓旈€氬父杩橀渶瑕佽缃濡傝竟娌挎晱鎰熷害锛堜緥濡備笂鍗囨部鎴栦笅闄嶆部锛屾垨楂?浣庣數骞充腑鏂級涔嬬被鐨勯厤缃€?
- 灞傜骇涓柇鑺墖锛圚IERARCHICAL INTERRUPT CHIPS锛夛細杩欐剰鍛崇潃姣忔潯 GPIO 绾块兘鏈変竴鏉′笓鐢ㄤ簬涓婁竴绾х埗涓柇鎺у埗鍣ㄧ殑 irq 绾裤€傛棤闇€鏌ヨ GPIO 纭欢鏉ュ垽鏂槸鍝潯绾胯Е鍙戜簡涓柇锛屼絾浠嶅彲鑳介渶瑕佺‘璁や腑鏂苟璁剧疆璇稿杈规部鏁忔劅搴︿箣绫荤殑閰嶇疆銆?
瀹炴椂锛圧ealtime锛夎€冮噺锛氫竴涓疄鏃跺吋瀹圭殑 GPIO 椹卞姩涓嶅簲鍦ㄥ叾 irqchip 瀹炵幇涓娇鐢?spinlock_t 鎴栦换浣曞彲浼戠湢鐨?API锛堝 PM runtime锛夈€?- spinlock_t 搴斿綋鏇挎崲涓?raw_spinlock_t銆俒^1^]
- 濡傛灉蹇呴』浣跨敤鍙紤鐪犵殑 API锛屽彲浠ヤ粠 .irq_bus_lock() 鍜?.irq_bus_unlock() 鍥炶皟涓畬鎴愶紝鍥犱负杩欐槸 irqchip 涓婂敮涓€鐨勬參璺緞鍥炶皟銆傚繀瑕佹椂鍒涘缓杩欎簺鍥炶皟銆俒^2^]


### 绾ц仈 GPIO irqchip


绾ц仈 GPIO irqchip 閫氬父灞炰簬浠ヤ笅涓夌被涔嬩竴锛?
- 閾惧紡绾ц仈 GPIO IRQCHIP锛圕HAINED CASCADED GPIO IRQCHIPS锛夛細杩欑被閫氬父鏄唴宓屼簬 SoC 涓婄殑绫诲瀷銆傝繖鎰忓懗鐫€ GPIO 鏈変竴涓揩閫熺殑 IRQ 娴佸鐞嗗嚱鏁帮紝瀹冧粠鐖?IRQ 澶勭悊鍑芥暟浠ラ摼鐨勬柟寮忚璋冪敤锛屾渶甯歌鐨勫氨鏄郴缁熶腑鏂帶鍒跺櫒銆傝繖鎰忓懗鐫€ GPIO irqchip 澶勭悊鍑芥暟浼氬湪淇濇寔 IRQ 绂佺敤鐨勬儏鍐典笅绔嬪嵆浠庣埗 irqchip 琚皟鐢ㄣ€侴PIO irqchip 闅忓悗鏈€缁堜細璋冪敤绫讳技杩欐牱鐨勪唬鐮?```

    static irqreturn_t foo_gpio_irq(int irq, void *data)
        chained_irq_enter(...);
        generic_handle_irq(...);
        chained_irq_exit(...);

```
  閾惧紡 GPIO irqchip 閫氬父涓嶈兘璁剧疆 struct gpio_chip 涓婄殑 .can_sleep 鏍囧織锛屽洜涓轰竴鍒囬兘鐩存帴鍙戠敓鍦ㄥ洖璋冧腑锛氫笉鑳戒娇鐢ㄥ儚 I2C 杩欐牱鐨勬參閫熸€荤嚎閫氫俊銆?
  瀹炴椂锛圧ealtime锛夎€冮噺锛氭敞鎰忛摼寮?IRQ 澶勭悊鍑芥暟涓嶄細琚己鍒剁嚎绋嬪寲鍒?-RT 涓娿€傚洜姝わ紝spinlock_t 鎴栦换浣曞彲浼戠湢鐨?API锛堝 PM runtime锛夐兘涓嶈兘鍦ㄩ摼寮?IRQ 澶勭悊鍑芥暟涓娇鐢ㄣ€?
  濡傛灉闇€瑕侊紙骞朵笖濡傛灉瀹冩棤娉曡浆鎹负宓屽绾跨▼鍖?GPIO irqchip锛岃涓嬫枃锛夛紝鍙互灏嗛摼寮?IRQ 澶勭悊鍑芥暟杞崲涓洪€氱敤 IRQ 澶勭悊鍑芥暟锛岃繖鏍峰湪 -RT 涓婂畠灏嗘垚涓虹嚎绋嬪寲 IRQ 澶勭悊鍑芥暟锛屽湪闈?RT 涓婃垚涓虹‖ IRQ 澶勭悊鍑芥暟锛堜緥濡傦紝瑙?[3]锛夈€?
  generic_handle_irq() 棰勬湡鍦?IRQ 绂佺敤鐨勬儏鍐典笅琚皟鐢紝鍥犳濡傛灉瀹冧粠涓€涓寮哄埗绾跨▼鍖栫殑 IRQ 澶勭悊鍑芥暟涓皟鐢紝IRQ 鏍稿績浼氭姤閿欍€傞偅涓€渇ake?鈥?鍘熷閿佸彲鐢ㄤ簬缁曡繃姝ら棶棰橈細

```
    raw_spinlock_t wa_lock;
    static irqreturn_t omap_gpio_irq_handler(int irq, void *gpiobank)
        unsigned long wa_lock_flags;
        raw_spin_lock_irqsave(&bank->wa_lock, wa_lock_flags);
        generic_handle_irq(irq_find_mapping(bank->chip.irq.domain, bit));
        raw_spin_unlock_irqrestore(&bank->wa_lock, wa_lock_flags);

```
- 閫氱敤閾惧紡 GPIO IRQCHIP锛圙ENERIC CHAINED GPIO IRQCHIPS锛夛細杩欑被涓庘€淐HAINED GPIO irqchip鈥濈浉鍚岋紝浣嗕笉浣跨敤閾惧紡 IRQ 澶勭悊鍑芥暟銆傚彇鑰屼唬涔嬶紝GPIO IRQ 鐨勫垎娲剧敱閫氳繃 request_irq() 閰嶇疆鐨勯€氱敤 IRQ 澶勭悊鍑芥暟鎵ц銆侴PIO irqchip 闅忓悗鏈€缁堜細鍦?```

    static irqreturn_t gpio_rcar_irq_handler(int irq, void *dev_id)
        for each detected GPIO IRQ
            generic_handle_irq(...);

```
  鐨勫簭鍒椾腑璋冪敤绫讳技杩欐牱鐨勪唬鐮併€傚疄鏃讹紙Realtime锛夎€冮噺锛氳繖绫诲鐞嗗嚱鏁颁細琚己鍒剁嚎绋嬪寲鍒?-RT 涓婏紝鍥犳 IRQ 鏍稿績浼氭姤閿欒 generic_handle_irq() 鏄湪 IRQ 鍚敤鐨勬儏鍐典笅琚皟鐢ㄧ殑锛屽彲浠ュ簲鐢ㄤ笌鈥淐HAINED GPIO irqchips鈥濈浉鍚岀殑鍙橀€氬姙娉曘€?
```
- 宓屽绾跨▼鍖?GPIO IRQCHIP锛圢ESTED THREADED GPIO IRQCHIPS锛夛細杩欑被鏄墖澶栵紙off-chip锛塆PIO 鎵╁睍鍣紝浠ュ強椹荤暀鍦?I2C 鎴?SPI 绛夌潯鐪犳€荤嚎鍙︿竴绔殑浠讳綍鍏朵粬 GPIO irqchip銆?
  褰撶劧锛岃繖绫婚渶瑕佹參閫熸€荤嚎閫氫俊鏉ヨ鍙?IRQ 鐘舵€併€佷笖姝ょ被閫氫俊鍙堝彲鑳藉紩鍙戝叾浠?IRQ 鐨勯┍鍔紝鏃犳硶鍦?IRQ 绂佺敤鐨勬儏鍐典笅浜庡揩閫?IRQ 澶勭悊鍑芥暟涓鐞嗐€傚彇鑰屼唬涔嬶紝瀹冧滑闇€瑕佺敓鎴愪竴涓嚎绋嬶紝鐒跺悗灞忚斀鐖?IRQ 绾匡紝鐩村埌璇ヤ腑鏂椹卞姩澶勭悊瀹屻€傝繖绫婚┍鍔ㄧ殑鏍囧織鏄皟鐢ㄧ被浼艰繖鏍风殑浠ｇ爜
```

    static irqreturn_t foo_gpio_irq(int irq, void *data)
        ...
        handle_nested_irq(irq);

```
  绾跨▼鍖?GPIO irqchip 鐨勬爣蹇楁槸瀹冧滑灏?struct gpio_chip 涓婄殑 .can_sleep 鏍囧織璁句负 true锛岃〃鏄庤鑺墖鍦ㄨ闂?GPIO 鏃跺彲鑳戒細浼戠湢銆?
  杩欑被 irqchip 澶╃敓瀵瑰疄鏃讹紙realtime锛夊叿鏈夊蹇嶅害锛屽洜涓哄畠浠凡缁忚璁剧疆涓哄鐞嗙潯鐪犱笂涓嬫枃銆?

```
### 闈㈠悜 GPIO irqchip 鐨勫熀纭€璁炬柦宸ュ叿


涓轰簡甯姪澶勭悊 GPIO irqchip 鍙婂叾鍏宠仈鐨?irqdomain 鍜岃祫婧愬垎閰嶅洖璋冪殑璁剧疆涓庣鐞嗐€傝繖浜涢€氳繃閫夋嫨 Kconfig 绗﹀彿 GPIOLIB_IRQCHIP 鏉ユ縺娲汇€傚鏋滃悓鏃惰繕閫夋嫨浜?IRQ_DOMAIN_HIERARCHY 绗﹀彿锛屽垯涔熶細鎻愪緵灞傜骇锛坔ierarchical锛夊伐鍏枫€傚湪鍋囪浣犵殑涓柇涓?GPIO 绾跨储寮曟槸涓€涓€鏄犲皠鐨勫墠鎻愪笅锛実piolib 浼氱鐞嗗叾涓緢澶т竴閮ㄥ垎寮€閿€浠ｇ爜锛?
    :header: GPIO 绾垮亸绉? 纭欢 IRQ

    0,0
    1,1
    2,2
    ...,...
    ngpio-1, ngpio-1


濡傛灉鏌愪簺 GPIO 绾挎病鏈夊搴旂殑 IRQ锛屽彲浠ヤ娇鐢?gpio_irq_chip 涓殑浣嶆帺鐮?valid_mask 涓庢爣蹇?need_valid_mask锛屽皢涓€浜涚嚎灞忚斀涓轰笉鍙敤浜庡叧鑱?IRQ銆?
璁剧疆杩欎簺宸ュ叿鐨勯閫夋柟寮忔槸锛屽湪娣诲姞 gpio_chip 涔嬪墠锛屽厛鍦?struct gpio_chip 鍐呴儴濉厖 struct gpio_irq_chip銆傚鏋滆繖鏍峰仛锛岄澶栫殑 irq_chip 浼氱敱 gpiolib 鍦ㄤ笌璁剧疆鍏朵綑 GPIO 鍔熻兘鐨勫悓鏃惰寤虹珛璧锋潵銆備互涓嬫槸涓€涓娇鐢?gpio_irq_chip 鐨勯摼寮忕骇鑱斾腑鏂鐞嗗嚱鏁扮殑鍏稿瀷绀轰緥銆傛敞鎰?mask/unmask锛堟垨 disable/enable锛夊嚱鏁版槸濡備綍璋冪敤鏍稿績 gpiolib 浠ｇ爜鐨勶細


  /** Typical state container **/
  struct my_gpio {
      struct gpio_chip gc;
  };

  static void my_gpio_mask_irq(struct irq_data *d)
  {
      struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
      irq_hw_number_t hwirq = irqd_to_hwirq(d);

      /*
       - 鎵ц浠讳綍蹇呰鐨勬搷浣滀互灞忚斀涓柇锛?       - 鐒跺悗璋冪敤鏍稿績浠ｇ爜浠ュ悓姝ョ姸鎬併€?       */
      gpiochip_disable_irq(gc, hwirq);
  }

  static void my_gpio_unmask_irq(struct irq_data *d)
  {
      struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
      irq_hw_number_t hwirq = irqd_to_hwirq(d);

      gpiochip_enable_irq(gc, hwirq);

      /*
       - 鎵ц浠讳綍蹇呰鐨勬搷浣滀互瑙ｉ櫎灞忚斀涓柇锛?       - 鍦ㄨ皟鐢ㄦ牳蹇冧唬鐮佷互鍚屾鐘舵€佷箣鍚庛€?       */
  }

  /*
   - 闈欐€佸～鍏?irqchip銆傛敞鎰忓畠琚涓?const
     锛堝苟鐢?IRQCHIP_IMMUTABLE 鏍囧織杩涗竴姝ユ爣鏄庯級锛屽苟涓?     GPIOCHIP_IRQ_RESOURCE_HELPER 瀹忓悜璇ョ粨鏋勬坊鍔犱簡
     涓€浜涢澶栫殑鍥炶皟銆?   */
  static const struct irq_chip my_gpio_irq_chip = {
      .name		= "my_gpio_irq",
      .irq_ack		= my_gpio_ack_irq,
      .irq_mask		= my_gpio_mask_irq,
      .irq_unmask	= my_gpio_unmask_irq,
      .irq_set_type	= my_gpio_set_irq_type,
      .flags		= IRQCHIP_IMMUTABLE,
      /** Provide the gpio resource callbacks **/
      GPIOCHIP_IRQ_RESOURCE_HELPERS,
  };

  int irq; /** from platform etc **/
  struct my_gpio *g;
  struct gpio_irq_chip *girq;

  /** 鑾峰彇鎸囧悜 gpio_irq_chip 鐨勬寚閽?**/
  girq = &g->gc.irq;
  gpio_irq_chip_set_chip(girq, &my_gpio_irq_chip);
  girq->parent_handler = ftgpio_gpio_irq_handler;
  girq->num_parents = 1;
  girq->parents = devm_kcalloc(dev, 1, sizeof(*girq->parents),
                               GFP_KERNEL);
  if (!girq->parents)
      return -ENOMEM;
  girq->default_type = IRQ_TYPE_NONE;
  girq->handler = handle_bad_irq;
  girq->parents[^0^] = irq;

  return devm_gpiochip_add_data(dev, &g->gc, g);

杩欎簺宸ュ叿涔熸敮鎸佷娇鐢ㄧ嚎绋嬪寲涓柇銆傝繖鏃朵綘鍙渶鍗曠嫭璇锋眰璇ヤ腑鏂苟鐓ф澶勭悊锛?

  /** Typical state container **/
  struct my_gpio {
      struct gpio_chip gc;
  };

  static void my_gpio_mask_irq(struct irq_data *d)
  {
      struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
      irq_hw_number_t hwirq = irqd_to_hwirq(d);

      /*
       - 鎵ц浠讳綍蹇呰鐨勬搷浣滀互灞忚斀涓柇锛?       - 鐒跺悗璋冪敤鏍稿績浠ｇ爜浠ュ悓姝ョ姸鎬併€?       */
      gpiochip_disable_irq(gc, hwirq);
  }

  static void my_gpio_unmask_irq(struct irq_data *d)
  {
      struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
      irq_hw_number_t hwirq = irqd_to_hwirq(d);

      gpiochip_enable_irq(gc, hwirq);

      /*
       - 鎵ц浠讳綍蹇呰鐨勬搷浣滀互瑙ｉ櫎灞忚斀涓柇锛?       - 鍦ㄨ皟鐢ㄦ牳蹇冧唬鐮佷互鍚屾鐘舵€佷箣鍚庛€?       */
  }

  /*
   - 闈欐€佸～鍏?irqchip銆傛敞鎰忓畠琚涓?const
     锛堝苟鐢?IRQCHIP_IMMUTABLE 鏍囧織杩涗竴姝ユ爣鏄庯級锛屽苟涓?     GPIOCHIP_IRQ_RESOURCE_HELPER 瀹忓悜璇ョ粨鏋勬坊鍔犱簡
     涓€浜涢澶栫殑鍥炶皟銆?   */
  static const struct irq_chip my_gpio_irq_chip = {
      .name		= "my_gpio_irq",
      .irq_ack		= my_gpio_ack_irq,
      .irq_mask		= my_gpio_mask_irq,
      .irq_unmask	= my_gpio_unmask_irq,
      .irq_set_type	= my_gpio_set_irq_type,
      .flags		= IRQCHIP_IMMUTABLE,
      /** Provide the gpio resource callbacks **/
      GPIOCHIP_IRQ_RESOURCE_HELPERS,
  };

  int irq; /** from platform etc **/
  struct my_gpio *g;
  struct gpio_irq_chip *girq;

  ret = devm_request_threaded_irq(dev, irq, NULL, irq_thread_fn,
                                  IRQF_ONESHOT, "my-chip", g);
  if (ret < 0)
      return ret;

  /** 鑾峰彇鎸囧悜 gpio_irq_chip 鐨勬寚閽?**/
  girq = &g->gc.irq;
  gpio_irq_chip_set_chip(girq, &my_gpio_irq_chip);
  /** 杩欏皢璁╂垜浠湪椹卞姩涓鐞嗙埗 IRQ **/
  girq->parent_handler = NULL;
  girq->num_parents = 0;
  girq->parents = NULL;
  girq->default_type = IRQ_TYPE_NONE;
  girq->handler = handle_bad_irq;

  return devm_gpiochip_add_data(dev, &g->gc, g);

杩欎簺宸ュ叿涔熸敮鎸佷娇鐢ㄥ眰绾т腑鏂帶鍒跺櫒銆傚湪杩欑鎯呭喌涓嬶紝鍏稿瀷鐨勮缃涓嬫墍绀猴細


  /** 甯︽湁鍔ㄦ€?irqchip 鐨勫吀鍨嬬姸鎬佸鍣?**/
  struct my_gpio {
      struct gpio_chip gc;
      struct fwnode_handle *fwnode;
  };

  static void my_gpio_mask_irq(struct irq_data *d)
  {
      struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
      irq_hw_number_t hwirq = irqd_to_hwirq(d);

      /*
       - 鎵ц浠讳綍蹇呰鐨勬搷浣滀互灞忚斀涓柇锛?       - 鐒跺悗璋冪敤鏍稿績浠ｇ爜浠ュ悓姝ョ姸鎬併€?       */
      gpiochip_disable_irq(gc, hwirq);
      irq_mask_mask_parent(d);
  }

  static void my_gpio_unmask_irq(struct irq_data *d)
  {
      struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
      irq_hw_number_t hwirq = irqd_to_hwirq(d);

      gpiochip_enable_irq(gc, hwirq);

      /*
       - 鎵ц浠讳綍蹇呰鐨勬搷浣滀互瑙ｉ櫎灞忚斀涓柇锛?       - 鍦ㄨ皟鐢ㄦ牳蹇冧唬鐮佷互鍚屾鐘舵€佷箣鍚庛€?       */
      irq_mask_unmask_parent(d);
  }

  /*
   - 闈欐€佸～鍏?irqchip銆傛敞鎰忓畠琚涓?const
     锛堝苟鐢?IRQCHIP_IMMUTABLE 鏍囧織杩涗竴姝ユ爣鏄庯級锛屽苟涓?     GPIOCHIP_IRQ_RESOURCE_HELPER 瀹忓悜璇ョ粨鏋勬坊鍔犱簡
     涓€浜涢澶栫殑鍥炶皟銆?   */
  static const struct irq_chip my_gpio_irq_chip = {
      .name		= "my_gpio_irq",
      .irq_ack		= my_gpio_ack_irq,
      .irq_mask		= my_gpio_mask_irq,
      .irq_unmask	= my_gpio_unmask_irq,
      .irq_set_type	= my_gpio_set_irq_type,
      .flags		= IRQCHIP_IMMUTABLE,
      /** Provide the gpio resource callbacks **/
      GPIOCHIP_IRQ_RESOURCE_HELPERS,
  };

  struct my_gpio *g;
  struct gpio_irq_chip *girq;

  /** 鑾峰彇鎸囧悜 gpio_irq_chip 鐨勬寚閽?**/
  girq = &g->gc.irq;
  gpio_irq_chip_set_chip(girq, &my_gpio_irq_chip);
  girq->default_type = IRQ_TYPE_NONE;
  girq->handler = handle_bad_irq;
  girq->fwnode = g->fwnode;
  girq->parent_domain = parent;
  girq->child_to_parent_hwirq = my_gpio_child_to_parent_hwirq;

  return devm_gpiochip_add_data(dev, &g->gc, g);

濡備綘鎵€瑙侊紝闈炲父鐩镐技锛屼絾浣犱笉鍐嶄负 IRQ 鎻愪緵鐖跺鐞嗗嚱鏁帮紝鑰屾槸鎻愪緵涓€涓埗 irqdomain銆佷竴涓敤浜庣‖浠剁殑 fwnode锛屼互鍙婁竴涓?.child_to_parent_hwirq() 鍑芥暟锛屽叾鐢ㄩ€旀槸浠庡瓙锛堝嵆姝?gpio 鑺墖锛夌‖浠?irq 鏌ユ壘鐖剁‖浠?irq銆備竴濡傛棦寰€锛屾煡鐪嬪唴鏍告爲涓殑绀轰緥浠ヨ幏鍙栧叧浜庡浣曟壘鍒版墍闇€閮ㄤ欢鐨勫弬鑰冩槸寰堝ソ鐨勫仛娉曘€?
濡傛灉闇€瑕佸皢杩欎簺宸ュ叿鎵€澶勭悊鐨?IRQ 鍩熶腑鐨勬煇浜?GPIO 绾挎帓闄ゅ湪澶栵紝鎴戜滑鍙互鍦ㄨ皟鐢?devm_gpiochip_add_data() 鎴?gpiochip_add_data() 涔嬪墠璁剧疆 gpiochip 鐨?.irq.need_valid_mask銆傝繖浼氬垎閰嶄竴涓?.irq.valid_mask锛屽叾涓疆浣嶇殑浣嶆暟涓庤姱鐗囦腑鐨?GPIO 绾挎暟閲忕浉鍚岋紝姣忎竴浣嶄唬琛ㄧ嚎 0..n-1銆傞┍鍔ㄥ彲浠ラ€氳繃娓呴櫎姝ゆ帺鐮佷腑鐨勪綅鏉ユ帓闄?GPIO 绾裤€傝鎺╃爜鍙互鍦ㄥ睘浜?struct gpio_irq_chip 涓€閮ㄥ垎鐨?init_valid_mask() 鍥炶皟涓～鍏呫€?
浣跨敤杩欎簺宸ュ叿鏃讹紝璇疯浣忎互涓嬪嚑鐐癸細

- 纭繚璧嬪€?struct gpio_chip 鐨勬墍鏈夌浉鍏虫垚鍛橈紝浠ヤ究 irqchip 鑳藉鍒濆鍖栥€備緥濡傦紝.dev 鍜?.can_sleep 搴斿綋琚纭缃€?
- 鍚嶄箟涓婂皢 gpio_irq_chip.handler 璁句负 handle_bad_irq銆傜劧鍚庯紝濡傛灉浣犵殑 irqchip 鏄骇鑱旂殑锛屽垯鏍规嵁鎺у埗鍣ㄦ墍鏀寔鐨勪互鍙婃秷璐硅€呮墍璇锋眰鐨勶紝鍦?irqchip 鐨?.set_type() 鍥炶皟涓皢澶勭悊鍑芥暟璁句负 handle_level_irq() 鍜?鎴?handle_edge_irq()銆?

### 閿佸畾 IRQ 鐨勪娇鐢?

鐢变簬 GPIO 涓?irq_chip 鏄浜ょ殑锛屾垜浠彲鑳戒細鍦ㄤ笉鍚岀殑鐢ㄤ緥涔嬮棿浜х敓鍐茬獊銆備緥濡傦紝鐢ㄤ簬 IRQ 鐨?GPIO 绾垮簲褰撴槸涓€鏉¤緭鍏ョ嚎锛屽湪杈撳嚭鍨?GPIO 涓婅Е鍙戜腑鏂槸娌℃湁鎰忎箟鐨勩€傚鏋滃瓙绯荤粺鍐呴儴瀛樺湪鍏充簬鍝竴渚ф鍦ㄤ娇鐢ㄨ祫婧愶紙渚嬪鏌愭潯鐗瑰畾鐨?GPIO 绾垮拰瀵勫瓨鍣級鐨勭珵浜夛紝瀹冨氨闇€瑕佹嫆缁濇煇浜涙搷浣滐紝骞跺湪 gpiolib 瀛愮郴缁熷唴閮ㄨ窡韪娇鐢ㄦ儏鍐点€?
杈撳叆鍨?GPIO 鍙互鐢ㄤ綔 IRQ 淇″彿銆傚綋鍙戠敓杩欑鎯呭喌鏃讹紝浼氳姹備竴涓┍鍔紝璋冪敤
```

  int gpiochip_lock_as_irq(struct gpio_chip *chip, unsigned int offset)

```
杩欏皢闃绘浣跨敤涓?irq 鏃犲叧鐨?GPIO API锛岀洿鍒?GPIO IRQ 閿佽
```

  void gpiochip_unlock_as_irq(struct gpio_chip *chip, unsigned int offset)

```
瑙ｉ櫎銆傚綋鍦?GPIO 椹卞姩鍐呴儴瀹炵幇 irqchip 鏃讹紝杩欎袱涓嚱鏁伴€氬父搴斿綋鍦?irqchip 鐨?.startup() 鍜?.shutdown() 鍥炶皟涓璋冪敤銆傚綋浣跨敤 gpiolib irqchip 宸ュ叿鏃讹紝杩欎簺鍥炶皟浼氳鑷姩鍒嗛厤銆?

### 绂佺敤涓庡惎鐢?IRQ


鍦ㄤ竴浜涳紙杈圭紭锛夌敤渚嬩腑锛岄┍鍔ㄥ彲鑳藉皢涓€鏉?GPIO 绾跨敤浣?IRQ 鐨勮緭鍏ワ紝浣嗗伓灏斾細灏嗚绾垮垏鎹负椹卞姩杈撳嚭锛岀劧鍚庡啀鍒囨崲鍥炲甫涓柇鐨勮緭鍏ャ€傝繖鍙戠敓鍦ㄨ濡?CEC锛堟秷璐圭數瀛愭帶鍒讹紝Consumer Electronics Control锛夎繖鏍风殑鍣ㄤ欢涓娿€?
褰?GPIO 琚敤浣?IRQ 淇″彿鏃讹紝gpiolib 涔熼渶瑕佺煡閬撹 IRQ 鏄惎鐢ㄨ繕鏄鐢ㄣ€備负浜嗗皢姝ゅ憡鐭?gpiolib锛?```

  void gpiochip_disable_irq(struct gpio_chip *chip, unsigned int offset)

```
杩欏厑璁搁┍鍔ㄥ湪 IRQ 澶勪簬
```

  void gpiochip_enable_irq(struct gpio_chip *chip, unsigned int offset)

```
绂佺敤鐘舵€佹椂灏?GPIO 椹卞姩涓鸿緭鍑恒€傚綋鍦?GPIO 椹卞姩鍐呴儴瀹炵幇 irqchip 鏃讹紝杩欎袱涓嚱鏁伴€氬父搴斿綋鍦?irqchip 鐨?.irq_disable() 鍜?.irq_enable() 鍥炶皟涓璋冪敤銆傚綋 irqchip 娌℃湁澹版槑 IRQCHIP_IMMUTABLE 鏃讹紝杩欎簺鍥炶皟浼氳鑷姩鍒嗛厤銆傝繖绉嶈涓哄凡琚簾寮冿紝骞舵鍦ㄤ粠鍐呮牳涓Щ闄ゃ€?

### GPIO IRQ 鑺墖鐨勫疄鏃讹紙Real-Time锛夊悎瑙勬€?

浠讳綍 irqchip 鐨勬彁渚涜€呴兘闇€瑕佺粡杩囩簿蹇冭皟鏁翠互鏀寔瀹炴椂锛圧eal-Time锛夋姠鍗犮€傛湡鏈?GPIO 瀛愮郴缁熶腑鐨勫叏閮?irqchip 閮借兘鐗㈣杩欎竴鐐癸紝骞惰繘琛岄€傚綋鐨勬祴璇曪紝浠ョ‘淇濆畠浠惎鐢ㄤ簡瀹炴椂鑳藉姏銆傚洜姝わ紝璇锋敞鎰忔枃妗ｄ腑涓婅堪鍏充簬瀹炴椂鐨勮€冮噺銆?
浠ヤ笅鏄湪涓哄疄鏃跺悎瑙勫噯澶囬┍鍔ㄦ椂瑕侀伒寰殑妫€鏌ユ竻鍗曪細

- 纭繚 spinlock_t 涓嶈鐢ㄤ綔 irq_chip 瀹炵幇鐨勪竴閮ㄥ垎
- 纭繚鍙紤鐪犵殑 API 涓嶈鐢ㄤ綔 irq_chip 瀹炵幇鐨勪竴閮ㄥ垎
  濡傛灉蹇呴』浣跨敤鍙紤鐪犵殑 API锛屽彲浠ヤ粠 .irq_bus_lock() 鍜?.irq_bus_unlock() 鍥炶皟涓畬鎴?- 閾惧紡 GPIO irqchip锛氱‘淇?spinlock_t 鎴栦换浣曞彲浼戠湢鐨?API 涓嶈鐢ㄤ簬閾惧紡 IRQ 澶勭悊鍑芥暟
- 閫氱敤閾惧紡 GPIO irqchip锛氭敞鎰?generic_handle_irq() 璋冪敤骞跺簲鐢ㄧ浉搴旂殑鍙橀€氬姙娉?- 閾惧紡 GPIO irqchip锛氬敖鍙兘鍘绘帀閾惧紡 IRQ 澶勭悊鍑芥暟锛屾敼鐢ㄩ€氱敤 irq 澶勭悊鍑芥暟
- regmap_mmio锛氬彲浠ラ€氳繃璁剧疆 .disable_locking 鏉ョ鐢?regmap 鍐呴儴鐨勯攣锛屽苟鍦?GPIO 椹卞姩涓嚜琛屽鐞嗛攣
- 浣跨敤鍐呮牳鍐呴€傚綋鐨勫疄鏃舵祴璇曠敤渚嬶紝閽堝鐢靛钩 IRQ 鍜岃竟娌?IRQ 鍒嗗埆娴嬭瘯浣犵殑椹卞姩

- [^1^] https://lore.kernel.org/r/1437496011-11486-1-git-send-email-bigeasy@linutronix.de/
- [^2^] https://lore.kernel.org/r/1443209283-20781-2-git-send-email-grygorii.strashko@ti.com
- [^3^] https://lore.kernel.org/r/1443209283-20781-3-git-send-email-grygorii.strashko@ti.com


## 璇锋眰鑷韩鎷ユ湁鐨?GPIO 寮曡剼


鏈夋椂鍏佽 GPIO 鑺墖椹卞姩閫氳繃 gpiolib API 璇锋眰鍏惰嚜韬殑 GPIO 鎻忚堪绗︽槸鏈夌敤鐨勩€侴PIO 椹卞姩鍙互浣跨敤浠ヤ笅
```

  struct gpio_desc *gpiochip_request_own_desc(struct gpio_desc *desc,
                                              u16 hwnum,
                                              const char *label,
                                              enum gpiod_flags flags)

  void gpiochip_free_own_desc(struct gpio_desc *desc)

```
閫氳繃 gpiochip_request_own_desc() 璇锋眰鐨勬弿杩扮蹇呴』鐢?gpiochip_free_own_desc() 閲婃斁銆?
杩欎簺鍑芥暟蹇呴』璋ㄦ厧浣跨敤锛屽洜涓哄畠浠笉褰卞搷妯″潡浣跨敤璁℃暟銆備笉瑕佺敤杩欎簺鍑芥暟鏉ヨ姹備笉灞炰簬璋冪敤椹卞姩鎵€鎷ユ湁鐨?gpio 鎻忚堪绗︺€?