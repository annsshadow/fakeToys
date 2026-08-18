
## 涓?GPIO 鐩稿叧鐨?_DSD 璁惧灞炴€?

闅忕潃 ACPI 5.1 鐨勫彂甯冿紝_DSD 閰嶇疆瀵硅薄缁堜簬鍏佽涓?_CRS 杩斿洖鐨?GPIO锛堜互鍙婂叾浠栦簨鐗╋級鎸囧畾鍚嶇О銆?浠ュ墠鎴戜滑鍙兘浣跨敤鏁存暟绱㈠紩鏉ユ煡鎵惧搴旂殑 GPIO锛岃繖闈炲父瀹规槗鍑洪敊锛堜緥濡傦紝瀹冧緷璧栦簬 _CRS 杈撳嚭鐨勯『搴忥級銆?
鍊熷姪 _DSD锛屾垜浠幇鍦ㄥ彲浠ヤ娇鐢ㄥ悕绉拌€屼笉鏄暣鏁版潵鏌ヨ GPIO锛?
```

  // Bluetooth device with reset and shutdown GPIOs
  Device (BTH)
  {
      Name (_HID, ...)

      Name (_CRS, ResourceTemplate ()
      {
          GpioIo (Exclusive, PullUp, 0, 0, IoRestrictionOutputOnly,
                  "\\_SB.GPO0", 0, ResourceConsumer) { 15 }
          GpioIo (Exclusive, PullUp, 0, 0, IoRestrictionOutputOnly,
                  "\\_SB.GPO0", 0, ResourceConsumer) { 27, 31 }
      })

      Name (_DSD, Package ()
      {
          ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
          Package ()
          {
              Package () { "reset-gpios", Package () { ^BTH, 1, 1, 0 } },
              Package () { "shutdown-gpios", Package () { ^BTH, 0, 0, 0 } },
          }
      })
  }

```

```

  Package () { "name", Package () { ref, index, pin, active_low }}

```

ref
  _CRS 涓寘鍚?GpioIo()/GpioInt() 璧勬簮鐨勮澶囷紝閫氬父灏辨槸璁惧鑷韩锛堟湰渚嬩腑涓?BTH锛夈€?index
  _CRS 涓?GpioIo()/GpioInt() 璧勬簮鐨勭储寮曪紝浠庨浂寮€濮嬨€?pin
  GpioIo()/GpioInt() 璧勬簮涓殑寮曡剼銆傞€氬父涓洪浂銆?active_low
  濡傛灉涓?1锛屽垯璇?GPIO 琚爣璁颁负浣庣數骞虫湁鏁堬紙active-low锛夈€?
鐢变簬 ACPI GpioIo() 璧勬簮娌℃湁瀛楁璇存槑瀹冩槸浣庣數骞虫湁鏁堣繕鏄珮鐢靛钩鏈夋晥锛屸€渁ctive_low鈥濆弬鏁板彲鍦ㄦ澶勪娇鐢ㄣ€?灏嗗叾璁剧疆涓?1 鍙皢 GPIO 鏍囪涓轰綆鐢靛钩鏈夋晥銆?
娉ㄦ剰锛宊DSD 涓殑 active_low 瀵?GpioInt() 璧勬簮娌℃湁鎰忎箟锛屽繀椤讳负 0銆侴pioInt() 璧勬簮鏈夊叾鑷韩鐨勫畾涔夋柟寮忋€?
鍦ㄦ垜浠殑钃濈墮绀轰緥涓紝鈥渞eset-gpios鈥濇寚鐨勬槸绗簩涓?GpioIo() 璧勬簮銆佽璧勬簮涓殑绗簩涓紩鑴氾紝GPIO 缂栧彿涓?31銆?
閬楁喚鐨勬槸锛孏pioIo() 璧勬簮娌℃湁鏄惧紡鎻愪緵椹卞姩鍦ㄥ叾鍒濆鍖栨湡闂村簲褰撲娇鐢ㄧ殑杈撳嚭寮曡剼鐨勫垵濮嬬姸鎬併€?
Linux 鍦ㄨ繖閲屽皾璇曚娇鐢ㄥ父璇嗭紝骞朵粠鍋忕疆锛坆ias锛夊拰鏋佹€ц缃腑鎺ㄥ鐘舵€併€備笅琛ㄦ樉绀轰簡棰勬湡锛?
+-------------+-------------+-----------------------------------------------+
| Pull Bias   | Polarity    | Requested...                                  |
+=============+=============+===============================================+
| Implicit                                                                  |
+-------------+-------------+-----------------------------------------------+
| **Default** | x           | AS IS锛堝亣璁惧浐浠跺凡涓烘垜浠厤缃ソ锛?             |
+-------------+-------------+-----------------------------------------------+
| Explicit                                                                  |
+-------------+-------------+-----------------------------------------------+
| **None**    | x           | AS IS锛堝亣璁惧浐浠跺凡涓烘垜浠厤缃ソ锛?             |
|             |             | 涓旀棤 Pull Bias                                |
+-------------+-------------+-----------------------------------------------+
| **Up**      | x (no _DSD) |                                               |
|             +-------------+ 鍋囪闈炴縺娲伙紝瑙嗕负楂樼數骞?                         |
|             | Low         |                                               |
|             +-------------+-----------------------------------------------+
|             | High        | 鍋囪婵€娲伙紝瑙嗕负楂樼數骞?                         |
+-------------+-------------+-----------------------------------------------+
| **Down**    | x (no _DSD) |                                               |
|             +-------------+ 鍋囪闈炴縺娲伙紝瑙嗕负浣庣數骞?                         |
|             | High        |                                               |
|             +-------------+-----------------------------------------------+
|             | Low         | 鍋囪婵€娲伙紝瑙嗕负浣庣數骞?                         |
+-------------+-------------+-----------------------------------------------+

涔熷氨鏄锛屽浜庢垜浠笂闈㈢殑绀轰緥锛岀敱浜庡亸缃缃槸鏄惧紡鐨勪笖瀛樺湪 _DSD锛屼袱涓?GPIO 閮藉皢琚涓洪珮鐢靛钩鏈夋晥锛?骞朵笖 Linux 浼氬皢寮曡剼閰嶇疆涓烘鐘舵€侊紝鐩村埌椹卞姩浠ヤ笉鍚屾柟寮忛噸鏂扮紪绋嬪畠浠€?
鍙互鍦?GPIO 鏁扮粍涓暀涓嬬┖娲炪€傝繖鍦ㄥ儚 SPI 涓绘満鎺у埗鍣ㄨ繖鏍风殑鎯呭喌涓嬪緢鏈夌敤锛屽叾涓竴浜涚墖閫夊彲鑳藉疄鐜颁负
GPIO锛岃€屽彟涓€浜涘疄鐜颁负鍘熺敓淇″彿銆備緥濡傦紝涓€涓?SPI 涓绘満鎺у埗鍣ㄥ彲浠ュ皢鐗囬€?0 鍜?2 瀹炵幇涓?GPIO锛岃€屽皢 1
瀹炵幇涓?
```

  Package () {
      "cs-gpios",
      Package () {
          ^GPIO, 19, 0, 0, // chip select 0: GPIO
          0,               // chip select 1: native signal
          ^GPIO, 20, 0, 0, // chip select 2: GPIO
      }
  }

```

娉ㄦ剰锛屽巻鍙蹭笂 ACPI 娌℃湁琛ㄧず GPIO 鏋佹€х殑鎵嬫锛屽洜姝?SPISerialBus() 璧勬簮鎸夋瘡鑺墖瀹氫箟鏋佹€с€備负浜嗛伩鍏?涓€杩炰覆鐨勫惁瀹氾紝GPIO 鏋佹€ц瑙嗕负楂樼數骞虫湁鏁堬紙Active High锛夈€傚嵆浣垮湪娑夊強 _DSD() 鐨勬儏鍐典笅锛堣涓婇潰鐨?绀轰緥锛夛紝GPIO CS 鏋佹€т篃蹇呴』瀹氫箟涓洪珮鐢靛钩鏈夋晥浠ラ伩鍏嶆涔夈€?
## 鍏朵粬鍙楁敮鎸佺殑灞炴€?

浠ヤ笅涓庤澶囨爲鍏煎鐨勮澶囧睘鎬т篃鍙?GPIO 鎺у埗鍣ㄧ殑 _DSD 璁惧灞炴€ф敮鎸侊細

- gpio-hog
- output-high
- output-low
- input
- line-name

```

  Name (_DSD, Package () {
      // _DSD Hierarchical Properties Extension UUID
      ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
      Package () {
          Package () { "hog-gpio8", "G8PU" }
      }
  })

  Name (G8PU, Package () {
      ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
      Package () {
          Package () { "gpio-hog", 1 },
          Package () { "gpios", Package () { 8, 0 } },
          Package () { "output-high", 1 },
          Package () { "line-name", "gpio8-pullup" },
      }
  })

```

- gpio-line-names

`gpio-line-names` 澹版槑鏄竴涓瓧绗︿覆鍒楄〃锛堚€渘ames鈥濓級锛屾弿杩?GPIO 鎺у埗鍣?鎵╁睍鍣ㄧ殑姣忎釜绾胯矾/寮曡剼銆?姝ゅ垪琛ㄥ寘鍚湪涓€涓寘涓紝蹇呴』鎻掑叆鍒?ACPI 琛紙閫氬父浣嶄簬 DSDT 鍐咃級鐨?GPIO 鎺у埗鍣ㄥ０鏄庡唴閮ㄣ€俙gpio-line-names`
鍒楄〃蹇呴』閬靛畧浠ヤ笅瑙勫垯锛堝彟瑙佺ず渚嬶級锛?
  - 鍒楄〃涓殑绗竴涓悕绉板搴?GPIO 鎺у埗鍣?鎵╁睍鍣ㄧ殑绗竴涓嚎璺?寮曡剼
  - 鍒楄〃鍐呯殑鍚嶇О蹇呴』杩炵画锛堜笉鍏佽鏈夆€滅┖娲炩€濓級
  - 鍒楄〃鍙互涓嶅畬鏁达紝骞跺彲浠ュ湪鏈€鍚庝竴涓?GPIO 绾胯矾涔嬪墠缁撴潫锛氭崲鍙ヨ瘽璇达紝涓嶅己鍒跺～鍏呮墍鏈?GPIO 绾胯矾
  - 鍏佽绌哄悕绉帮紙涓や釜寮曞彿 `""` 瀵瑰簲涓€涓┖鍚嶇О锛?  - 鍚屼竴涓?GPIO 鎺у埗鍣?鎵╁睍鍣ㄥ唴鐨勫悕绉板繀椤诲敮涓€

涓€涓叿鏈?16 鏉＄嚎璺殑 GPIO 鎺у埗鍣ㄧず渚嬶紝甯︽湁涓€涓笉瀹屾暣鐨勫垪琛紝鍏朵腑鍖呭惈涓や釜

```

  Package () {
      "gpio-line-names",
      Package () {
          "pin_0",
          "pin_1",
          "",
          "",
          "pin_3",
          "pin_4_push_button",
      }
  }

```

鍦ㄨ繍琛屾椂锛屼笂杩板０鏄庝骇鐢熷涓嬬粨鏋滐紙浣跨敤

```

  root@debian:~# gpioinfo gpiochip4
  gpiochip4 - 16 lines:
          line   0:      "pin_0"       unused   input  active-high
          line   1:      "pin_1"       unused   input  active-high
          line   2:      unnamed       unused   input  active-high
          line   3:      unnamed       unused   input  active-high
          line   4:      "pin_3"       unused   input  active-high
          line   5: "pin_4_push_button" unused input active-high
          line   6:      unnamed       unused   input  active-high
          line   7       unnamed       unused   input  active-high
          line   8:      unnamed       unused   input  active-high
          line   9:      unnamed       unused   input  active-high
          line  10:      unnamed       unused   input  active-high
          line  11:      unnamed       unused   input  active-high
          line  12:      unnamed       unused   input  active-high
          line  13:      unnamed       unused   input  active-high
          line  14:      unnamed       unused   input  active-high
          line  15:      unnamed       unused   input  active-high
  root@debian:~# gpiofind pin_4_push_button
  gpiochip4 5
  root@debian:~#

```

```

  Package () {
      "gpio-line-names",
      Package () {
          "SPI0_CS_N", "EXP2_INT", "MUX6_IO", "UART0_RXD",
          "MUX7_IO", "LVL_C_A1", "MUX0_IO", "SPI1_MISO",
      }
  }

```

鏈夊叧杩欎簺灞炴€х殑鏇村淇℃伅锛岃鍙傞槄 Documentation/devicetree/bindings/gpio/gpio.txt銆?
## 椹卞姩鎻愪緵鐨?ACPI GPIO 鏄犲皠


鏈変簺绯荤粺鐨?ACPI 琛ㄤ笉鍖呭惈 _DSD锛屼絾鎻愪緵浜嗗甫鏈?GpioIo()/GpioInt() 璧勬簮鐨?_CRS锛岃€岃澶囬┍鍔ㄤ粛闇€瑕?涓庝箣閰嶅悎宸ヤ綔銆?
鍦ㄨ繖浜涙儏鍐典笅锛岄┍鍔ㄥ彲鐢ㄧ殑 ACPI 璁惧鏍囪瘑瀵硅薄锛坃HID銆乢CID銆乢CLS銆乢SUB銆乢HRV锛夊彲鐢ㄤ簬鏍囪瘑璁惧锛岃繖搴斿綋
瓒充互纭畾 _CRS 杩斿洖鐨?GpioIo()/GpioInt() 璧勬簮鎵€鍒楀嚭鐨勬墍鏈?GPIO 绾胯矾鐨勫惈涔夊拰鐢ㄩ€斻€傛崲鍙ヨ瘽璇达紝涓€鏃?椹卞姩鏍囪瘑浜嗚澶囷紝瀹冨氨搴旇鐭ラ亾瑕佷娇鐢?GpioIo()/GpioInt() 璧勬簮涓殑鍝簺鍐呭銆傚畬鎴愭宸ヤ綔鍚庯紝瀹冨彲浠ョ畝鍗曞湴
涓哄畠灏嗚浣跨敤鐨?GPIO 绾胯矾鍒嗛厤鍚嶇О锛屽苟鍚?GPIO 瀛愮郴缁熸彁渚涜繖浜涘悕绉颁笌瀵瑰簲 ACPI GPIO 璧勬簮涔嬮棿鐨勬槧灏勩€?
涓烘锛岄┍鍔ㄩ渶瑕佸畾涔変竴涓槧灏勮〃锛屼綔涓?struct acpi_gpio_mapping 瀵硅薄鐨勪互 NULL 缁撳熬鐨勬暟缁勶紝姣忎釜瀵硅薄鍖呭惈
涓€涓悕绉般€佷竴涓寚鍚戠嚎璺暟鎹紙struct acpi_gpio_params锛夊璞℃暟缁勭殑鎸囬拡锛屼互鍙婅鏁扮粍鐨勫ぇ灏忋€傛瘡涓?struct acpi_gpio_params 瀵硅薄鐢变笁涓瓧娈电粍鎴愶細crs_entry_index銆乴ine_index銆乤ctive_low锛屽垎鍒〃绀?_CRS 涓洰鏍?GpioIo()/GpioInt() 璧勬簮鐨勭储寮曪紙浠庨浂寮€濮嬶級銆佽璧勬簮涓洰鏍囩嚎璺殑绱㈠紩锛堜粠闆跺紑濮嬶級浠ュ強璇ョ嚎璺?鐨勪綆鐢靛钩鏈夋晥鏍囧織锛屼笌涓婃枃涓寚瀹氱殑 _DSD GPIO 灞炴€ф牸寮忕浉瀵瑰簲銆?
瀵逛簬鍓嶉潰璁ㄨ鐨勭ず渚嬭摑鐗欒澶囷紝鏁版嵁缁撴瀯浣嶄簬

```

  static const struct acpi_gpio_params reset_gpio = { 1, 1, false };
  static const struct acpi_gpio_params shutdown_gpio = { 0, 0, false };

  static const struct acpi_gpio_mapping bluetooth_acpi_gpios[] = {
      { "reset-gpios", &reset_gpio, 1 },
      { "shutdown-gpios", &shutdown_gpio, 1 },
      { }
  };

```

鎺ヤ笅鏉ワ紝闇€瑕佸皢鏄犲皠琛ㄤ綔涓虹浜屼釜鍙傛暟浼犻€掔粰 acpi_dev_add_driver_gpios() 鎴栧叾鎵樼鐗堟湰锛屽悗鑰呭皢鎶婂畠
娉ㄥ唽鍒板叾绗竴涓弬鏁版墍鎸囧悜鐨?ACPI 璁惧瀵硅薄銆傝繖搴斿綋鍦ㄩ┍鍔ㄧ殑 .probe() 渚嬬▼涓畬鎴愩€傚湪绉婚櫎鏃讹紝椹卞姩搴旈€氳繃
鍦ㄥ厛鍓嶆敞鍐岃琛ㄧ殑 ACPI 璁惧瀵硅薄涓婅皟鐢?acpi_dev_remove_driver_gpios() 鏉ユ敞閿€鍏?GPIO 鏄犲皠琛ㄣ€?
## 浣跨敤 _CRS 鍥為€€


濡傛灉璁惧娌℃湁 _DSD锛屾垨鑰呴┍鍔ㄦ病鏈夊垱寤?ACPI GPIO 鏄犲皠锛孡inux GPIO 妗嗘灦浼氭嫆缁濊繑鍥炰换浣?GPIO銆傝繖鏄洜涓?椹卞姩涓嶇煡閬撳畠瀹為檯寰楀埌鐨勬槸浠€涔堛€備緥濡傦紝濡傛灉

```

  Device (BTH)
  {
      Name (_HID, ...)

      Name (_CRS, ResourceTemplate () {
          GpioIo (Exclusive, PullNone, 0, 0, IoRestrictionNone,
                  "\\_SB.GPO0", 0, ResourceConsumer) { 15 }
          GpioIo (Exclusive, PullNone, 0, 0, IoRestrictionNone,
                  "\\_SB.GPO0", 0, ResourceConsumer) { 27 }
      })
  }

```

```

  desc = gpiod_get(dev, "reset", GPIOD_OUT_LOW);
  if (IS_ERR(desc))
	...error handling...

```

浣嗙敱浜庢棤娉曠煡閬撯€渞eset鈥濅笌 _CRS 涓殑 GpioIo() 涔嬮棿鐨勬槧灏勶紝desc 灏嗘寔鏈?ERR_PTR(-ENOENT)銆?
椹卞姩浣滆€呭彲浠ラ€氳繃鏄惧紡浼犻€掓槧灏勬潵瑙ｅ喅杩欎釜闂锛堣繖鏄帹鑽愮殑鏂瑰紡锛屽凡鍦ㄤ笂涓€绔犱腑璇存槑锛夈€?
ACPI GPIO 鏄犲皠琛ㄤ笉搴旀薄鏌撻偅浜涗笉鐭ラ亾鑷繁姝ｅ湪鏈嶅姟鍝釜鍏蜂綋璁惧鐨勯┍鍔ㄣ€傝繖鎰忓懗鐫€ ACPI GPIO 鏄犲皠琛ㄤ笌
ACPI ID 浠ュ強涓婃枃鎵€鍒楃殑璇ヨ澶囩壒瀹氬璞＄揣瀵嗙浉杩炪€?
## 鑾峰彇 GPIO 鎻忚堪绗?

```

  desc = gpiod_get(dev, connection_id, flags);
  desc = gpiod_get_index(dev, connection_id, index, flags);

```

鎴戜滑鍦ㄨ繖閲屽彲浠ヨ€冭檻涓ょ涓嶅悓鐨勬儏褰紝鍗虫槸鍚︽彁渚涗簡杩炴帴 ID锛坈onnection ID锛夈€?
```

  desc = gpiod_get(dev, "non-null-connection-id", flags);
  desc = gpiod_get_index(dev, "non-null-connection-id", index, flags);

```

鎯呭舰 1 鍋囧畾鐩稿簲鐨?ACPI 璁惧鎻忚堪蹇呴』宸插畾涔変簡璁惧灞炴€э紝鍚﹀垯灏嗛樆姝㈣幏鍙栦换浣?GPIO 璧勬簮銆?
```

  desc = gpiod_get(dev, NULL, flags);
  desc = gpiod_get_index(dev, NULL, index, flags);

```

鎯呭舰 2 鏄惧紡鍛婄煡 GPIO 鏍稿績鍦?_CRS 涓煡鎵捐祫婧愩€?
璇锋敞鎰忥紝鍦ㄦ儏褰?1 鍜屾儏褰?2 涓紝鍋囪鎻愪緵浜嗕袱涓増鏈殑 ACPI 璁惧鎻忚堪涓旈┍鍔ㄤ腑娌℃湁鏄犲皠锛実piod_get_index()
灏嗚繑鍥炰笉鍚岀殑璧勬簮銆傝繖灏辨槸涓轰粈涔堟煇涓壒瀹氶┍鍔ㄥ繀椤诲涓婁竴绔犳墍杩板皬蹇冨鐞嗗畠浠€?