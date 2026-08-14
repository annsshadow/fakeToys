## 鍖呭惈 uAPI 澶存枃浠?

鏈夋椂锛屽寘鍚ご鏂囦欢鍜?C 绀轰緥浠ｇ爜鏄緢鏈夌敤鐨勶紝浠ヤ究鎻忚堪鐢ㄦ埛绌洪棿 API锛屽苟鍦ㄤ唬鐮佷笌
鏂囨。涔嬮棿鐢熸垚浜ゅ弶寮曠敤銆備负鐢ㄦ埛绌洪棿 API 鏂囦欢娣诲姞浜ゅ弶寮曠敤杩樻湁涓€涓澶栫殑濂藉锛?濡傛灉鏌愪釜绗﹀彿鍦ㄦ枃妗ｄ腑鎵句笉鍒帮紝Sphinx 浼氱敓鎴愯鍛娿€傝繖鏈夊姪浜庝娇 uAPI 鏂囨。涓?鍐呮牳鏀瑰姩淇濇寔鍚屾銆俻arse_headers.py <parse_headers> 鎻愪緵浜嗕竴绉嶇敓鎴愭绫?浜ゅ弶寮曠敤鐨勬柟娉曘€傚畠蹇呴』鍦ㄦ瀯寤烘枃妗ｆ椂閫氳繃 Makefile 璋冪敤銆傚叧浜庡浣曞湪鍐呮牳鏍戜腑
浣跨敤瀹冿紝璇峰弬闃?`Documentation/userspace-api/media/Makefile` 涓殑绀轰緥銆?

##### tools/docs/parse_headers.py


######## 鍚嶇О


parse_headers.py - 瑙ｆ瀽涓€涓?C 鏂囦欢锛屼互璇嗗埆鍑芥暟銆佺粨鏋勪綋銆佹灇涓惧拰瀹忓畾涔夛紝骞?鍒涘缓鍒?Sphinx 鏂囨。鐨勪氦鍙夊紩鐢ㄣ€?
######## 鐢ㄦ硶


parse-headers.py [-h] [-d] [-t] `FILE_IN` `FILE_OUT` `FILE_RULES`

######## 绠€浠?

灏嗚緭鍏ョ殑 C 澶存枃浠舵垨婧愭枃浠?`FILE_IN` 杞崲涓轰竴涓?ReStructured Text锛岄€氳繃
..parsed-literal 鍧楀寘鍚紝骞朵负鎻忚堪璇?API 鐨勬枃妗ｆ枃浠跺垱寤轰氦鍙夊紩鐢ㄣ€傚畠鎺ュ彈涓€涓?鍙€夌殑 `FILE_RULES` 鏂囦欢锛岀敤浜庢弿杩板摢浜涘厓绱犲皢琚拷鐣ワ紝鎴栨寚鍚戦潪榛樿寮曠敤
绫诲瀷/鍚嶇О銆?
杈撳嚭鍐欏叆 `FILE_OUT`銆?
瀹冭兘澶熻瘑鍒?`define`銆乣struct`銆乣typedef`銆乣enum` 浠ュ強鏋氫妇 `symbol`锛屽苟涓哄畠浠?鍏ㄩ儴鍒涘缓浜ゅ弶寮曠敤銆?
瀹冭繕鑳藉鍖哄垎鐢ㄤ簬鎸囧畾 Linux 鐗瑰畾瀹忥紙鐢ㄤ互瀹氫箟 `ioctl`锛夌殑 `#define`銆?
```

    ignore ioctl VIDIOC_ENUM_FMT
    replace ioctl VIDIOC_DQBUF vidioc_qbuf
    replace define V4L2_EVENT_MD_FL_HAVE_FRAME_SEQ :c:type:`v4l2_event_motion_det`

```
######## 浣嶇疆鍙傛暟


  `FILE_IN`
      杈撳叆鐨?C 鏂囦欢

  `FILE_OUT`
      杈撳嚭鐨?RST 鏂囦欢

  `FILE_RULES`
      渚嬪鏂囦欢锛堝彲閫夛級

######## 閫夐」


  `-h`, `--help`
      鏄剧ず甯姪淇℃伅骞堕€€鍑?  `-d`, `--debug`
      鎻愰珮璋冭瘯绾у埆銆傚彲浠ュ娆′娇鐢?  `-t`, `--toc`
      涓嶅湪瀛楅潰鍧椾腑杈撳嚭锛岃€屾槸鍦?RST 鏂囦欢涓緭鍑轰竴涓洰褰曡〃锛圱OC锛?
######## 鎻忚堪


浠?`FILE_IN` 鍒涘缓鍐呮牳澶存枃浠剁殑澧炲己鐗堟湰锛屼负鍏舵瘡涓?C 鏁版嵁缁撴瀯绫诲瀷娣诲姞浜ゅ弶閾炬帴锛?骞朵娇鐢?reStructuredText 鏍囪杩涜鏍煎紡鍖栵紝鍙互鏄師鏍凤紝涔熷彲浠ユ槸涓€涓洰褰曡〃銆?
瀹冩帴鍙椾竴涓彲閫夌殑 `FILE_RULES`锛岀敤浜庢弿杩板摢浜涘厓绱犲皢琚拷鐣ユ垨鎸囧悜闈為粯璁ゅ紩鐢紝
骞跺彲閫夋嫨鎬у湴瀹氫箟瑕佷娇鐢ㄧ殑 C 鍛藉悕绌洪棿銆?
鍏剁洰鏍囨槸鍏佽鎷ユ湁鏇村叏闈㈢殑鏂囨。锛屽叾涓?uAPI 澶存枃浠跺皢涓轰唬鐮佸垱寤轰氦鍙夊紩鐢ㄩ摼鎺ャ€?
杈撳嚭鍐欏叆 `FILE_OUT`銆?
`FILE_RULES` 鍙兘鍖呭惈涓夌绫诲瀷鐨勮鍙ワ細**ignore**銆?*replace** 鍜?**namespace**銆?
榛樿鎯呭喌涓嬶紝瀹冧細涓烘墍鏈夌鍙峰拰瀹忓畾涔夊垱寤鸿鍒欙紝浣嗕篃鍏佽瑙ｆ瀽涓€涓緥澶栨枃浠躲€傛绫?鏂囦欢鍖呭惈涓€缁勪娇鐢ㄤ互涓嬭娉曠殑瑙勫垯锛?
1. 蹇界暐瑙勫垯锛?
    ignore **type** **symbol**

灏嗙鍙蜂粠寮曠敤鐢熸垚涓Щ闄ゃ€?
2. 鏇挎崲瑙勫垯锛?
    replace **type** **old_symbol** **new_reference**

    灏?**old_symbol** 鏇挎崲涓?**new_reference**銆?    **new_reference** 鍙互鏄細

    - 涓€涓畝鍗曠殑绗﹀彿鍚嶏紱
    - 涓€涓畬鏁寸殑 Sphinx 寮曠敤銆?
3. 鍛藉悕绌洪棿瑙勫垯

    namespace **namespace**

    璁剧疆浜ゅ弶寮曠敤鐢熸垚鏈熼棿瑕佷娇鐢ㄧ殑 C **namespace**銆傚彲琚浛鎹㈣鍒欒鐩栥€?
鍦ㄥ拷鐣ュ拰鏇挎崲瑙勫垯涓紝**type** 鍙互鏄細

    - ioctl锛?        鐢ㄤ簬褰㈠ `_IO*` 鐨勫畯瀹氫箟锛屼緥濡?ioctl 瀹氫箟

    - define锛?        鐢ㄤ簬鍏跺畠瀹忓畾涔?
    - symbol锛?        鐢ㄤ簬鏋氫妇涓畾涔夌殑绗﹀彿锛?
    - typedef锛?        鐢ㄤ簬 typedef锛?
    - enum锛?        鐢ㄤ簬闈炲尶鍚嶆灇涓剧殑鍚嶇О锛?
    - struct锛?        鐢ㄤ簬缁撴瀯浣撱€?
######## 绀轰緥


```
    ignore define _VIDEODEV2_H

```
```
    enum foo { BAR1, BAR2, PRIVATE };

  It won't generate cross-references for ``PRIVATE``::

    ignore symbol PRIVATE

  瀵逛簬鍚屼竴涓粨鏋勪綋锛屼笌鍏朵负姣忎釜绗﹀彿鍒涘缓涓€涓氦鍙夊紩鐢紝涓嶅璁╁畠浠叏閮ㄦ寚鍚?  ``enum foo`` C 绫诲瀷::

    replace symbol BAR1 :c:type:`foo`
    replace symbol BAR2 :c:type:`foo`

```
```
    namespace MC

```
######## 缂洪櫡


Report bugs to Mauro Carvalho Chehab <mchehab@kernel.org>

######## 鐗堟潈


Copyright (c) 2016, 2025 by Mauro Carvalho Chehab <mchehab+huawei@kernel.org>.

License GPLv2: GNU GPL version 2 <https://gnu.org/licenses/gpl.html>.

This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
