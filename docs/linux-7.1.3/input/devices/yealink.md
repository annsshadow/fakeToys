yealink usb-p1k 鐢佃瘽椹卞姩鏂囨。



鐘舵€?



p1k 鏄竴娆剧浉瀵瑰粔浠风殑 USB 1.1 鐢佃瘽锛屽叿澶囷細


  - 閿洏		瀹屽叏鏀寔锛寉ealink.ko / input event API
  - LCD		瀹屽叏鏀寔锛寉ealink.ko / sysfs API
  - LED		瀹屽叏鏀寔锛寉ealink.ko / sysfs API
  - 鎷ㄥ彿闊?	瀹屽叏鏀寔锛寉ealink.ko / sysfs API
  - 閾冨０		瀹屽叏鏀寔锛寉ealink.ko / sysfs API
  - 闊抽鎾斁		瀹屽叏鏀寔锛宻nd_usb_audio.ko / alsa API
  - 闊抽褰曞埗		瀹屽叏鏀寔锛宻nd_usb_audio.ko / alsa API


鏈夊叧鍘傚晢鏂囨。锛岃鍙傞槄 http://www.yealink.com



閿洏鐗规€?



鍐呮牳涓綋鍓嶇殑鏄犲皠鐢?map_p1k_to_key 鎻愪緵
```

   Physical USB-P1K button layout	input events


              up			     up
        IN           OUT		left,	right
             down			    down

      pickup   C    hangup		enter, backspace, escape
        1      2      3			1, 2, 3
        4      5      6			4, 5, 6,
        7      8      9			7, 8, 9,
        *      0      #			*, 0, #,

```

鈥渦p鈥濆拰鈥渄own鈥濋敭鍦ㄦ寜閽笂浠ョ澶磋〃绀恒€傗€減ickup鈥濆拰鈥渉angup鈥濋敭鍦ㄦ寜閽笂浠ョ豢鑹插拰绾㈣壊鐢佃瘽琛ㄧず銆?



LCD 鐗规€?


```

    |[]   [][]   [][]   [][]   in   |[][]
    |[] M [][] D [][] : [][]   out  |[][]
                              store

    NEW REP         SU MO TU WE TH FR SA

    [] [] [] [] [] [] [] [] [] [] [] []
    [] [] [] [] [] [] [] [] [] [] [] []


  Line 1  Format (see below)	: 18.e8.M8.88...188
	  Icon names		:   M  D  :  IN OUT STORE
  Line 2  Format		: .........
	  Icon name		: NEW REP SU MO TU WE TH FR SA
  Line 3  Format		: 888888888888


```

鏍煎紡鎻忚堪锛?
  浠庣敤鎴风┖闂寸殑瑙掑害鐪嬶紝涓栫晫琚垝鍒嗕负鈥渄igits鈥濓紙鏁板瓧锛夊拰鈥渋cons鈥濓紙鍥炬爣锛夈€?
  涓€涓暟瀛楀彲浠ユ湁瀛楃闆嗭紝涓€涓浘鏍囧彧鑳藉浜庘€滃紑鈥濇垨鈥滃叧鈥濈姸鎬併€?

```

    '8' :  Generic 7 segment digit with individual addressable segments

    Reduced capability 7 segment digit, when segments are hard wired together.
    '1' : 2 segments digit only able to produce a 1.
    'e' : Most significant day of the month digit,
          able to produce at least 1 2 3.
    'M' : Most significant minute digit,
          able to produce at least 0 1 2 3 4 5.

    Icons or pictograms:
    '.' : For example like AM, PM, SU, a 'dot' .. or other single segment
	  elements.


```

椹卞姩浣跨敤鏂规硶


```

  /sys/.../
           line1	Read/Write, lcd line1
           line2	Read/Write, lcd line2
           line3	Read/Write, lcd line3

	   get_icons    Read, returns a set of available icons.
	   hide_icon    Write, hide the element by writing the icon name.
	   show_icon    Write, display the element by writing the icon name.

	   map_seg7	Read/Write, the 7 segments char set, common for all
			yealink phones. (see map_to_7segment.h)

	   ringtone	Write, upload binary representation of a ringtone,
			see yealink.c. status EXPERIMENTAL due to potential
			races between async. and sync usb calls.


```

lineX



璇诲彇 /sys/../lineX 灏嗚繑鍥炲甫鏈夊綋鍓嶅€肩殑鏍煎紡瀛楃涓层€?

```

    cat ./line3
    888888888888
    Linux Rocks!

```

鍐欏叆 /sys/../lineX 灏嗚缃搴旂殑 LCD 琛屻€?


 - 澶氫綑鐨勫瓧绗︿細琚拷鐣ャ€?
 - 濡傛灉鍐欏叆鐨勫瓧绗﹀皯浜庡厑璁告暟閲忥紝鍓╀綑鐨勬暟瀛椾繚鎸佷笉鍙樸€?
 - 鍒惰〃绗?'\t' 鍜?'\n' 瀛楃涓嶄細瑕嗙洊鍘熸湁鍐呭銆?
 - 鍚戝浘鏍囧啓鍏ョ┖鏍煎皢濮嬬粓闅愯棌鍏跺唴瀹广€?

```

    date +"%m.%e.%k:%M"  | sed 's/^0/ /' > ./line1

  Will update the LCD with the current date & time.


```

get_icons


```

  cat ./get_icons
  on M
  on D
  on :
     IN
     OUT
     STORE
     NEW
     REP
     SU
     MO
     TU
     WE
     TH
     FR
     SA
     LED
     DIALTONE
     RINGTONE


```

鏄剧ず/闅愯棌鍥炬爣



鍐欏叆杩欎簺鏂囦欢灏嗘洿鏂板浘鏍囩殑鐘舵€併€?
涓€娆″彧鑳芥洿鏂颁竴涓浘鏍囥€?


濡傛灉鏌愪釜鍥炬爣涔熶綅浜?./lineX 涓婏紝鍒欏叾瀵瑰簲鐨勫€间細浣跨敤璇ュ浘鏍囩殑棣栧瓧姣嶈繘琛屾洿鏂般€?

```

    echo -n "STORE" > ./show_icon

    cat ./line1
    18.e8.M8.88...188
		  S

  Example - sound the ringtone for 10 seconds::

    echo -n RINGTONE > /sys/..../show_icon
    sleep 10
    echo -n RINGTONE > /sys/..../hide_icon


```

澹伴煶鐗规€?



澹伴煶鐢?ALSA 椹卞姩 snd_usb_audio 鎻愪緵鏀寔


璁惧瀹為檯鐨勬瀬闄愭槸涓€涓?16 浣嶉€氶亾锛岄噰鏍风巼涓庢挱鏀剧巼鍧囦负 8000 Hz銆?

```

    arecord -v -d 10 -r 8000 -f S16_LE -t wav  foobar.wav

  Example - playback test::

    aplay foobar.wav


```

鏁呴殰鎺掓煡



:Q: 妯″潡 yealink 缂栬瘧骞跺畨瑁呮病鏈変换浣曢棶棰橈紝浣嗙數璇濇湭琚垵濮嬪寲锛屼笖瀵逛换浣曟搷浣滈兘娌℃湁鍙嶅簲銆?
:A: 濡傛灉浣犲湪 dmesg 涓湅鍒扮被浼煎涓嬪唴瀹癸細
    hiddev0: USB HID v1.00 Device [Yealink Network Technology Ltd. VOIP USB Phone
    杩欐剰鍛崇潃 hid 椹卞姩鎶㈠厛鍗犵敤浜嗚璁惧銆傝灏濊瘯鍦ㄤ换浣曞叾浠?usb hid 椹卞姩涔嬪墠鍔犺浇 yealink 妯″潡銆傝鍙傞槄浣犳墍浣跨敤鍙戣鐗堟彁渚涚殑鍏充簬妯″潡閰嶇疆鐨勮鏄庛€?


:Q: 鐢佃瘽鐜板湪鍙互宸ヤ綔浜嗭紙鏄剧ず鐗堟湰骞舵帴鍙楁寜閿緭鍏ワ級锛屼絾鎴戞壘涓嶅埌 sysfs 鏂囦欢銆?
:A: sysfs 鏂囦欢浣嶄簬鐗瑰畾鐨?usb 绔偣涓娿€傚湪澶у鏁板彂琛岀増涓紝浣犲彲浠ユ墽琛岋細"find /sys/ -name get_icons" 鏉ヨ幏鍙栫嚎绱€?



鑷磋阿涓庢劅璋?



  - Olivier Vandorpe锛屽洜鍚姩 usbb2k-api 椤圭洰骞跺畬鎴愪簡澶ч噺閫嗗悜宸ョ▼宸ヤ綔銆?
  - Martin Diehl锛屽洜鎸囧嚭濡備綍澶勭悊 USB 鍐呭瓨鍒嗛厤銆?
  - Dmitry Torokhov锛屽洜澶ч噺鐨勪唬鐮佸鏌ヤ笌寤鸿銆?

