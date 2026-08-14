## PXA2xx/PXA3xx 澶勭悊鍣ㄧ殑 MFP 閰嶇疆


			Eric Miao <eric.miao@marvell.com>

MFP 鏄?Multi-Function Pin锛堝鍔熻兘寮曡剼锛夌殑缂╁啓锛屾槸 PXA3xx 鍙婂悗缁?PXA
绯诲垪澶勭悊鍣ㄤ笂鐨勫紩鑴氬鐢紙pin-mux锛夐€昏緫銆傛湰鏂囨。鎻忚堪浜嗙幇鏈夌殑 MFP API锛?浠ュ強鏉跨骇/骞冲彴椹卞姩浣滆€呭簲褰撳浣曚娇鐢ㄥ畠銆?
## 鍩烘湰姒傚康


涓?PXA25x 鍜?PXA27x 涓婄殑 GPIO 澶嶇敤鍔熻兘璁剧疆涓嶅悓锛屼粠 PXA3xx 璧峰紩鍏ヤ簡涓€绉?鍏ㄦ柊鐨?MFP 鏈哄埗锛屾妸寮曡剼澶嶇敤鍔熻兘褰诲簳绉诲嚭浜?GPIO 鎺у埗鍣ㄣ€傞櫎浜嗗紩鑴氬鐢ㄩ厤缃?涔嬪锛孧FP 杩樻帶鍒剁潃姣忎釜寮曡剼鐨勪綆鍔熻€楃姸鎬併€侀┍鍔ㄥ己搴︺€佷笂鎷?涓嬫媺浠ュ強浜嬩欢妫€娴嬨€?涓嬮潰鏄悇鍐呴儴妯″潡涔嬮棿杩炴帴鍏崇郴鐨勭ず鎰忓浘锛?
```
 +--------+
 |        |--(GPIO19)--+
 |  GPIO  |            |
 |        |--(GPIO...) |
 +--------+            |
                       |       +---------+
 +--------+            +------>|         |
 |  PWM2  |--(PWM_OUT)-------->|   MFP   |
 +--------+            +------>|         |-------> to external PAD
                       | +---->|         |
 +--------+            | | +-->|         |
 |  SSP2  |---(TXD)----+ | |   +---------+
 +--------+              | |
                         | |
 +--------+              | |
 | Keypad |--(MKOUT4)----+ |
 +--------+                |
                           |
 +--------+                |
 |  UART2 |---(TXD)--------+
 +--------+
```

娉ㄦ剰锛氬閮ㄧ剨鐩橈紙pad锛夎鍛藉悕涓?MFP_PIN_GPIO19锛岃繖骞朵笉蹇呯劧鎰忓懗鐫€瀹冩槸涓撶敤浜?GPIO19 鐨勶紝鑰屽彧鏄彁绀鸿寮曡剼鍦ㄥ唴閮ㄥ彲浠ョ敱 GPIO 鎺у埗鍣ㄧ殑 GPIO19 璺敱鑰屾潵銆?
涓轰簡鏇村ソ鍦扮悊瑙ｄ粠 PXA25x/PXA27x 鐨?GPIO 澶嶇敤鍔熻兘鍒拌繖绉嶆柊 MFP 鏈哄埗鐨勫彉鍖栵紝
涓嬮潰鏄嚑涓叧閿偣锛?
  1. PXA3xx 涓婄殑 GPIO 鎺у埗鍣ㄧ幇鍦ㄦ槸涓€涓笓鐢ㄦ帶鍒跺櫒锛屼笌鍏朵粬鍐呴儴鎺у埗鍣紙濡?     PWM銆丼SP 鍜?UART锛変竴鏍凤紝鎷ユ湁 128 涓唴閮ㄤ俊鍙凤紝杩欎簺淇″彿鍙互閫氳繃涓€涓垨澶氫釜
     MFP 璺敱鍒板閮紙渚嬪 GPIO<0> 鏃㈠彲浠ラ€氳繃 MFP_PIN_GPIO0锛屼篃鍙互閫氳繃
     MFP_PIN_GPIO0_2 璺敱锛屽弬瑙?arch/arm/mach-pxa/mfp-pxa300.h锛?
  2. 澶嶇敤鍔熻兘閰嶇疆宸蹭粠璇?GPIO 鎺у埗鍣ㄤ腑绉婚櫎锛屽墿涓嬬殑鍔熻兘閮芥槸绾?GPIO 鐩稿叧鐨勶紝鍗?
       - GPIO 淇″彿鐢靛钩鎺у埗
       - GPIO 鏂瑰悜鎺у埗
       - GPIO 鐢靛钩鍙樺寲妫€娴?
  3. 姣忎釜寮曡剼鐨勪綆鍔熻€楃姸鎬佺幇鍦ㄧ敱 MFP 鎺у埗锛岃繖鎰忓懗鐫€ PXA2xx 涓婄殑 PGSRx 瀵勫瓨鍣?     鍦?PXA3xx 涓婂凡缁忔病鏈夌敤澶勪簡

  4. 鍞ら啋妫€娴嬬幇鍦ㄧ敱 MFP 鎺у埗锛孭WER 涓嶅啀鎺у埗鏉ヨ嚜 GPIO 鐨勫敜閱掞紱鏍规嵁鐫＄湢鐘舵€佺殑涓嶅悓锛?     鐢?ADxER锛堝畾涔変簬 pxa3xx-regs.h锛夋帶鍒舵潵鑷?MFP 鐨勫敜閱?
娉ㄦ剰锛氱敱浜?MFP 涓?GPIO 涔嬮棿鏈夊姝ゆ竻鏅扮殑鍒嗗伐锛岄€氬父鎴戜滑鐢?GPIO<xx> 琛ㄧず涓€涓?GPIO 淇″彿锛岃€岀敤 MFP<xxx> 鎴栧紩鑴?xxx 琛ㄧず涓€涓墿鐞嗙剨鐩橈紙鎴栫悆鏍咃級銆?
## MFP API 鐢ㄦ硶


瀵逛簬鏉跨骇浠ｇ爜缂栧啓鑰咃紝涓嬮潰鏄竴浜涙寚瀵煎師鍒欙細

1. 鍦ㄤ綘鐨?<board>.c 涓寘鍚笅鍒楀ご鏂囦欢涔嬩竴锛?
   - #include "mfp-pxa25x.h"
   - #include "mfp-pxa27x.h"
   - #include "mfp-pxa300.h"
   - #include "mfp-pxa320.h"
   - #include "mfp-pxa930.h"

   娉ㄦ剰锛氫綘鐨?<board>.c 涓彧鍖呭惈鍏朵腑涓€涓枃浠讹紝鍏蜂綋鍙栧喅浜庢墍浣跨敤鐨勫鐞嗗櫒锛屽洜涓?   杩欎簺鏂囦欢涓殑寮曡剼閰嶇疆瀹氫箟鍙兘浼氬啿绐侊紙鍗冲悓鍚嶅湪涓嶅悓澶勭悊鍣ㄤ笂鍚箟鍜岃缃笉鍚岋級銆?   渚嬪瀵逛簬鍚屾椂鏀寔 PXA300/PXA310 鍜?PXA320 鐨?zylonite 骞冲彴锛屽紩鍏ヤ簡涓や釜鐙珛
   鐨勬枃浠讹細zylonite_pxa300.c 鍜?zylonite_pxa320.c锛堥櫎浜嗗鐞?MFP 閰嶇疆鐨勫樊寮傦紝
   瀹冧滑杩樺鐞嗚繖涓ょ缁勫悎涔嬮棿鐨勫叾浠栧樊寮傦級銆?
   娉ㄦ剰锛歅XA300 鍜?PXA310 鍦ㄥ紩鑴氶厤缃笂鍑犱箮瀹屽叏鐩稿悓锛圥XA310 棰濆鏀寔鍏朵腑涓€浜涳級锛?   鍥犳杩欎竴宸紓瀹為檯涓婅娑电洊鍦ㄥ崟涓?mfp-pxa300.h 涓€?
```
     static unsigned long mainstone_pin_config[] __initdata = {
	/* Chip Select */
	GPIO15_nCS_1,

	/* LCD - 16bpp Active TFT */
	GPIOxx_TFT_LCD_16BPP,
	GPIO16_PWM0_OUT,	/* Backlight */

	/* MMC */
	GPIO32_MMC_CLK,
	GPIO112_MMC_CMD,
	GPIO92_MMC_DAT_0,
	GPIO109_MMC_DAT_1,
	GPIO110_MMC_DAT_2,
	GPIO111_MMC_DAT_3,

	...

	/* GPIO */
	GPIO1_GPIO | WAKEUP_ON_EDGE_BOTH,
     };

   a) 涓€鏃﹀紩鑴氶厤缃浼犻€掔粰 pxa{2xx,3xx}_mfp_config() 骞跺啓鍏ュ疄闄呭瘎瀛樺櫒鍚庯紝
   瀹冧滑灏辨病鏈夌敤浜嗭紝鍙兘浼氳涓㈠純锛屽姞涓?'__initdata' 鍙互鍦ㄨ繖閲岃妭鐪佷竴浜涢澶栫殑瀛楄妭銆?
   b) 褰撲竴涓儴浠跺彧鏈変竴绉嶅彲琛岀殑寮曡剼閰嶇疆鏃讹紝鍙互浣跨敤涓€浜涚畝鍖栫殑瀹氫箟锛屼緥濡?   PXA25x 鍜?PXA27x 澶勭悊鍣ㄤ笂鐨?GPIOxx_TFT_LCD_16BPP

   c) 濡傛灉鎸夋澘绾ц璁★紝鏌愪釜寮曡剼鍙互琚厤缃负浠庝綆鍔熻€楃姸鎬佸敜閱掔郴缁燂紝瀹冨彲浠ョ敤
   涓嬪垪浠绘剰涓€椤瑰仛鈥滄垨鈥濊繍绠楋細

      WAKEUP_ON_EDGE_BOTH
      WAKEUP_ON_EDGE_RISE
      WAKEUP_ON_EDGE_FALL
      WAKEUP_ON_LEVEL_HIGH - 涓撻棬鐢ㄤ簬鍚敤閿洏 GPIO

   浠ヨ〃鏄庤寮曡剼鍏峰鍞ら啋绯荤粺鐨勮兘鍔涳紝浠ュ強鍦ㄥ摢浜涜竟娌夸笂銆傜劧鑰岋紝杩欏苟涓嶅繀鐒舵剰鍛崇潃
   璇ュ紩鑴氣€滀細鈥濆敜閱掔郴缁燂紝鍙湁褰撲娇鐢ㄧ浉搴旂殑 GPIO IRQ锛圙PIO_IRQ(xx) 鎴?   gpio_to_irq()锛夎皟鐢?set_irq_wake()锛屽苟鏈€缁堜负瀹為檯鐨勫瘎瀛樺櫒璁剧疆璋冪敤
   gpio_set_wake() 鏃讹紝瀹冩墠浼氬敜閱掔郴缁熴€?
   d) 灏界 PXA3xx 鐨?MFP 鏀寔姣忎釜寮曡剼鐨勮竟娌挎娴嬶紝鍐呴儴閫昏緫鍙湪 ADxER 瀵勫瓨鍣ㄤ腑鐨?   閭ｄ簺鐗瑰畾浣嶈缃綅鏃舵墠浼氬敜閱掔郴缁燂紝鑰岃繖浜涗綅鍙互寰堝ソ鍦版槧灏勫埌鐩稿簲鐨勫璁撅紝鍥犳
   鍙互閽堝澶栬 IRQ 璋冪敤 set_irq_wake() 鏉ュ惎鐢ㄥ敜閱掋€?

```
## PXA3xx 涓婄殑 MFP


PXA3xx 涓婄殑姣忎釜澶栭儴 I/O 鐒婄洏锛堢壒娈婄敤閫旂殑闄ゅ锛夐兘鍏宠仈鐫€涓€涓?MFP 閫昏緫锛屽苟
鐢变竴涓?MFP 瀵勫瓨鍣紙MFPR锛夋帶鍒躲€?
```
 31                        16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
  +-------------------------+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
  |         RESERVED        |PS|PU|PD|  DRIVE |SS|SD|SO|EC|EF|ER|--| AF_SEL |
  +-------------------------+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

  Bit 3:   RESERVED
  Bit 4:   EDGE_RISE_EN - enable detection of rising edge on this pin
  Bit 5:   EDGE_FALL_EN - enable detection of falling edge on this pin
  Bit 6:   EDGE_CLEAR   - disable edge detection on this pin
  Bit 7:   SLEEP_OE_N   - enable outputs during low power modes
  Bit 8:   SLEEP_DATA   - output data on the pin during low power modes
  Bit 9:   SLEEP_SEL    - selection control for low power modes signals
  Bit 13:  PULLDOWN_EN  - enable the internal pull-down resistor on this pin
  Bit 14:  PULLUP_EN    - enable the internal pull-up resistor on this pin
  Bit 15:  PULL_SEL     - pull state controlled by selected alternate function
                          (0) or by PULL{UP,DOWN}_EN bits (1)

  Bit 0 - 2: AF_SEL - alternate function selection, 8 possibilities, from 0-7
  Bit 10-12: DRIVE  - drive strength and slew rate
			0b000 - fast 1mA
			0b001 - fast 2mA
			0b002 - fast 3mA
			0b003 - fast 4mA
			0b004 - slow 6mA
			0b005 - fast 6mA
			0b006 - slow 10mA
			0b007 - fast 10mA

```
## PXA2xx/PXA3xx 鐨?MFP 璁捐


鐢变簬 PXA2xx 涓?PXA3xx 鍦ㄥ紩鑴氬鐢ㄥ鐞嗕笂鐨勫樊寮傦紝寮曞叆浜嗕竴濂楃粺涓€鐨?MFP API
鏉ュ悓鏃舵兜鐩栬繖涓や釜绯诲垪鐨勫鐞嗗櫒銆?
璇ヨ璁＄殑鍩烘湰鎬濇兂鏄紩鍏ラ拡瀵规墍鏈夊彲鑳藉紩鑴氶厤缃殑瀹氫箟锛岃繖浜涘畾涔変笌澶勭悊鍣ㄥ拰骞冲彴
鏃犲叧锛屽啀璋冪敤瀹為檯鐨?API 鎶婅繖浜涘畾涔夎浆鎹负瀵勫瓨鍣ㄨ缃苟浣夸箣鐢熸晥銆?
### 娑夊強鐨勬枃浠?

  - arch/arm/mach-pxa/include/mach/mfp.h

  鐢ㄤ簬
    1. 缁熶竴鐨勫紩鑴氬畾涔?鈥斺€?鎵€鏈夊彲閰嶇疆寮曡剼鐨勬灇涓惧父閲?    2. 涓庡鐞嗗櫒鏃犲叧鐨勩€侀拡瀵逛竴绉嶅彲鑳界殑 MFP 閰嶇疆鐨勪綅瀹氫箟

  - arch/arm/mach-pxa/mfp-pxa3xx.h

  鐢ㄤ簬 PXA3xx 鐗规湁鐨?MFPR 瀵勫瓨鍣ㄤ綅瀹氫箟浠ュ強 PXA3xx 閫氱敤寮曡剼閰嶇疆

  - arch/arm/mach-pxa/mfp-pxa2xx.h

  鐢ㄤ簬 PXA2xx 鐗规湁鐨勫畾涔変互鍙?PXA25x/PXA27x 閫氱敤寮曡剼閰嶇疆

  - arch/arm/mach-pxa/mfp-pxa25x.h
    arch/arm/mach-pxa/mfp-pxa27x.h
    arch/arm/mach-pxa/mfp-pxa300.h
    arch/arm/mach-pxa/mfp-pxa320.h
    arch/arm/mach-pxa/mfp-pxa930.h

  鐢ㄤ簬澶勭悊鍣ㄧ壒鏈夌殑瀹氫箟

  - arch/arm/mach-pxa/mfp-pxa3xx.c
  - arch/arm/mach-pxa/mfp-pxa2xx.c

  鐢ㄤ簬浣垮紩鑴氶厤缃湪瀹為檯涓婂鍏蜂綋澶勭悊鍣ㄧ敓鏁堢殑瀹炵幇銆?
### 寮曡剼閰嶇疆


  浠ヤ笅娉ㄩ噴鎽樿嚜 mfp.h锛堝弬瑙佸疄闄呮簮浠ｇ爜
```
    /*
     * a possible MFP configuration is represented by a 32-bit integer
     *
     * bit  0.. 9 - MFP Pin Number (1024 Pins Maximum)
     * bit 10..12 - Alternate Function Selection
     * bit 13..15 - Drive Strength
     * bit 16..18 - Low Power Mode State
     * bit 19..20 - Low Power Mode Edge Detection
     * bit 21..22 - Run Mode Pull State
     *
     * to facilitate the definition, the following macros are provided
     *
     * MFP_CFG_DEFAULT - default MFP configuration value, with
     * 		  alternate function = 0,
     * 		  drive strength = fast 3mA (MFP_DS03X)
     * 		  low power mode = default
     * 		  edge detection = none
     *
     * MFP_CFG	- default MFPR value with alternate function
     * MFP_CFG_DRV	- default MFPR value with alternate function and
     * 		  pin drive strength
     * MFP_CFG_LPM	- default MFPR value with alternate function and
     * 		  low power mode
     * MFP_CFG_X	- default MFPR value with alternate function,
     * 		  pin drive strength and low power mode
     */

   Examples of pin configurations are::

     #define GPIO94_SSP3_RXD		MFP_CFG_X(GPIO94, AF1, DS08X, FLOAT)

   鍏跺惈涔夋槸 GPIO94 鍙互琚厤缃负 SSP3_RXD锛屽鐢ㄥ姛鑳介€夋嫨涓?1锛岄┍鍔ㄥ己搴︿负
   0b101锛屽苟涓斿湪浣庡姛鑰楁ā寮忎笅澶勪簬娴┖锛坒loat锛夌姸鎬併€?
   娉ㄦ剰锛氳繖鏄皢璇ュ紩鑴氶厤缃负 SSP3_RXD 鐨勯粯璁よ缃紝鍦ㄦ澘绾т唬鐮佷腑鍙互绋嶄綔淇敼锛?   涓嶈繃骞朵笉鎺ㄨ崘杩欐牱鍋氾紝鍘熷洜浠呬粎鏄繖绉嶉粯璁よ缃€氬父缁忚繃绮惧績缂栫爜锛屽苟涓斿湪澶у鏁?   鎯呭喌涓嬮兘鑳芥甯稿伐浣溿€?
```
### 瀵勫瓨鍣ㄨ缃?

   鍦?PXA3xx 涓婏紝閽堝鏌愪釜寮曡剼閰嶇疆鐨勫瘎瀛樺櫒璁剧疆瀹為檯涓婇潪甯哥洿鎺ワ紝澶у鏁颁綅鍙互
   浠ヤ竴绉嶆洿绠€鍗曠殑鏂瑰紡鐩存帴杞崲涓?MFPR 鍊笺€傝绠楀嚭浜嗕袱缁?MFPR 鍊硷細杩愯鏃剁殑
   閭ｇ粍鍜屼綆鍔熻€楁ā寮忕殑閭ｇ粍锛屼互渚垮厑璁镐笉鍚岀殑璁剧疆銆?
   浠庨€氱敤鐨勫紩鑴氶厤缃埌 PXA2xx 涓婂疄闄呭瘎瀛樺櫒璁剧疆鐨勮浆鎹㈠垯绋嶆樉澶嶆潅锛氭秹鍙婅澶?   瀵勫瓨鍣紝鍖呮嫭 GAFRx銆丟PDRx銆丳GSRx銆丳WER銆丳KWR銆丳FER 鍜?PRER銆傚叧浜庤繖绉?   杞崲鏄浣曡繘琛岀殑锛岃鍙傞槄 mfp-pxa2xx.c銆?