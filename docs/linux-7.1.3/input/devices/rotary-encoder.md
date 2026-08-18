## rotary-encoder - 涓€涓敤浜?GPIO 杩炴帴璁惧鐨勯€氱敤椹卞姩


:Author: Daniel Mack <daniel@caiaq.de>, 2009 骞?2 鏈?

### 鍔熻兘


鏃嬭浆缂栫爜鍣ㄦ槸閫氳繃涓ゆ牴绾夸笌 CPU 鎴栧叾浠栧璁捐繛鎺ョ殑璁惧銆傝緭鍑虹浉浣嶅樊涓?90 搴︼紝閫氳繃鍦ㄤ笅闄嶆部鍜屼笂鍗囨部瑙﹀彂锛屽彲浠ョ‘瀹氳浆鍔ㄦ柟鍚戙€?

鏈変簺缂栫爜鍣ㄥ湪绋冲畾鐘舵€佷笅涓や釜杈撳嚭閮戒负浣庣數骞筹紝鍙︿竴浜涘湪绋冲畾鐘舵€佷笅涓や釜杈撳嚭閮戒负楂樼數骞筹紙鍗婂懆鏈熸ā寮忥級锛岃繕鏈変竴浜涘湪姣忎釜姝ヨ繘閮芥湁绋冲畾鐘舵€侊紙鍥涘垎涔嬩竴鍛ㄦ湡妯″紡锛夈€?

```

                  _____       _____       _____
                 |     |     |     |     |     |
  Channel A  ____|     |_____|     |_____|     |____

                 :  :  :  :  :  :  :  :  :  :  :  :
            __       _____       _____       _____
              |     |     |     |     |     |     |
  Channel B   |_____|     |_____|     |_____|     |__

                 :  :  :  :  :  :  :  :  :  :  :  :
  Event          a  b  c  d  a  b  c  d  a  b  c  d

                |<-------->|
	          one step

                |<-->|
	          one step (half-period mode)

                |<>|
	          one step (quarter-period mode)

```
鏇村淇℃伅锛岃鍙傝
	https://en.wikipedia.org/wiki/Rotary_encoder


### 浜嬩欢 / 鐘舵€佹満


鍦ㄥ崐鍛ㄦ湡妯″紡涓嬶紝浣跨敤涓婅堪鐘舵€?a) 鍜?c) 鍩轰簬鏈€鍚庝竴涓ǔ瀹氱姸鎬佹潵纭畾鏃嬭浆鏂瑰悜銆備簨浠跺湪鐘舵€?b) 鍜?d) 涓笂鎶ワ紝鍓嶆彁鏄柊鐨勭ǔ瀹氱姸鎬佷笌涓婁竴涓笉鍚岋紙鍗虫棆杞病鏈夊湪涓€斿弽杞級銆?

姝ゅ锛屼互涓嬫儏鍐甸€傜敤锛?

a) 閫氶亾 A 涓婄殑涓婂崌娌匡紝閫氶亾 B 澶勪簬浣庣數骞?
	姝ょ姸鎬佺敤浜庤瘑鍒『鏃堕拡杞姩

b) 閫氶亾 B 涓婄殑涓婂崌娌匡紝閫氶亾 A 澶勪簬楂樼數骞?
	杩涘叆姝ょ姸鎬佹椂锛岀紪鐮佸櫒琚疆浜庘€渁rmed鈥濈姸鎬侊紝鎰忓懗鐫€瀹冨凡缁忕湅鍒颁簡涓€涓崟姝ヨ浆鎹㈢殑涓€鍗婅矾绋嬨€?

c) 閫氶亾 A 涓婄殑涓嬮檷娌匡紝閫氶亾 B 澶勪簬楂樼數骞?
	姝ょ姸鎬佺敤浜庤瘑鍒€嗘椂閽堣浆鍔?

d) 閫氶亾 B 涓婄殑涓嬮檷娌匡紝閫氶亾 A 澶勪簬浣庣數骞?
	鍋滆溅浣嶇疆銆傚鏋滅紪鐮佸櫒杩涘叆姝ょ姸鎬侊紝搴斿綋宸茬粡鍙戠敓浜嗕竴涓畬鏁寸殑杞崲锛岄櫎闈炲畠鍦ㄤ腑閫旂炕杞洖鏉ャ€傗€渁rmed鈥濈姸鎬佸憡璇夋垜浠繖涓€鐐广€?

### 骞冲彴瑕佹眰


鐢变簬姝ら┍鍔ㄤ腑娌℃湁浠讳綍涓庣‖浠剁浉鍏崇殑璋冪敤锛屼娇鐢ㄥ畠鐨勫钩鍙板繀椤绘敮鎸?gpiolib銆傚彟涓€涓姹傛槸 IRQ 蹇呴』鑳藉鍦ㄤ袱涓竟娌夸笂瑙﹀彂銆?


### 鏉跨骇闆嗘垚


瑕佸湪浣犵殑绯荤粺涓娇鐢ㄦ椹卞姩锛岄渶娉ㄥ唽涓€涓悕涓?'rotary-encoder' 鐨?platform_device锛屽苟灏?IRQ 鍜屾煇浜涚壒瀹氬钩鍙版暟鎹笌涔嬪叧鑱斻€傜敱浜庤椹卞姩浣跨敤閫氱敤璁惧灞炴€э紝杩欏彲浠ラ€氳繃璁惧鏍戙€丄CPI 鎴栦娇鐢ㄩ潤鎬佹澘鏂囦欢鏉ュ畬鎴愶紝濡備笅渚嬫墍绀猴細

```

	/* board support file example */

	#include <linux/input.h>
	#include <linux/gpio/machine.h>
	#include <linux/property.h>

	#define GPIO_ROTARY_A 1
	#define GPIO_ROTARY_B 2

	static struct gpiod_lookup_table rotary_encoder_gpios = {
		.dev_id = "rotary-encoder.0",
		.table = {
			GPIO_LOOKUP_IDX("gpio-0",
					GPIO_ROTARY_A, NULL, 0, GPIO_ACTIVE_LOW),
			GPIO_LOOKUP_IDX("gpio-0",
					GPIO_ROTARY_B, NULL, 1, GPIO_ACTIVE_HIGH),
			{ },
		},
	};

	static const struct property_entry rotary_encoder_properties[] = {
		PROPERTY_ENTRY_U32("rotary-encoder,steps-per-period", 24),
		PROPERTY_ENTRY_U32("linux,axis",		      ABS_X),
		PROPERTY_ENTRY_U32("rotary-encoder,relative_axis",    0),
		{ },
	};

	static const struct software_node rotary_encoder_node = {
		.properties = rotary_encoder_properties,
	};

	static struct platform_device rotary_encoder_device = {
		.name		= "rotary-encoder",
		.id		= 0,
	};

	...

	gpiod_add_lookup_table(&rotary_encoder_gpios);
	device_add_software_node(&rotary_encoder_device.dev, &rotary_encoder_node);
	platform_device_register(&rotary_encoder_device);

	...

```
璇峰弬闃呰澶囨爲缁戝畾鏂囨。浠ヤ簡瑙ｈ椹卞姩鏀寔鐨勬墍鏈夊睘鎬с€?
