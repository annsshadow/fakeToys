
## Intel Image Processing Unit 6锛圛PU6锛夎緭鍏ョ郴缁熼┍鍔?

Copyright |copy| 2023--2024 Intel Corporation

## 绠€浠?

鏈枃浠惰褰曚簡浣嶄簬 drivers/media/pci/intel/ipu6 涓嬬殑 Intel IPU6锛堢 6 浠ｅ浘鍍忓鐞嗗崟鍏冿級
杈撳叆绯荤粺锛圡IPI CSI2 鎺ユ敹鍣級椹卞姩銆?
Intel IPU6 鍙互鍦ㄦ煇浜?Intel SoC 涓壘鍒帮紝浣嗗苟闈炴墍鏈?SKU 閮芥湁锛?
- Tiger Lake
- Jasper Lake
- Alder Lake
- Raptor Lake
- Meteor Lake

Intel IPU6 鐢变袱涓儴鍒嗙粍鎴愨€斺€旇緭鍏ョ郴缁燂紙ISYS锛夊拰澶勭悊绯荤粺锛圥SYS锛夈€?
杈撳叆绯荤粺涓昏浣滀负 MIPI CSI-2 鎺ユ敹鍣ㄥ伐浣滐紝鎺ユ敹骞跺鐞嗘潵鑷紶鎰熷櫒鐨勫浘鍍忔暟鎹紝骞跺皢甯?杈撳嚭鍒板唴瀛樸€?
鍏辨湁 2 涓┍鍔ㄦā鍧椻€斺€攊ntel-ipu6 鍜?intel-ipu6-isys銆俰ntel-ipu6 鏄竴涓?IPU6 閫氱敤
椹卞姩锛岃礋璐?PCI 閰嶇疆銆佸浐浠跺姞杞戒笌瑙ｆ瀽銆佸浐浠惰璇併€丏MA 鏄犲皠浠ュ強 IPU-MMU锛堝唴閮ㄥ唴瀛?鏄犲皠鍗曞厓锛夐厤缃€俰ntel_ipu6_isys 瀹炵幇浜?V4L2銆丮edia Controller 鍜?V4L2 瀛愯澶?鎺ュ彛銆侷PU6 ISYS 椹卞姩鏀寔閫氳繃 V4L2 瀛愯澶囦紶鎰熷櫒椹卞姩杩炴帴鍒?IPU6 ISYS 鐨勬憚鍍忓ご浼犳劅鍣ㄣ€?
	 鏈夊叧 IPU6 纭欢鐨勪俊鎭€?
## 杈撳叆绯荤粺椹卞姩


杈撳叆绯荤粺椹卞姩涓昏閰嶇疆 CSI-2 D-PHY锛屾瀯寤哄浐浠舵祦閰嶇疆锛屽悜鍥轰欢鍙戦€佸懡浠わ紝浠庣‖浠跺拰鍥轰欢
鑾峰彇鍝嶅簲锛岀劧鍚庡皢缂撳啿鍖鸿繑鍥炵粰鐢ㄦ埛銆侷SYS 琚〃绀轰负澶氫釜 V4L2 瀛愯澶囦互鍙婅棰戣妭鐐广€?
   :alt: 鏀寔澶氭祦鐨?ipu6 isys 濯掍綋鍥?
   IPU6 ISYS 濯掍綋鍥撅紝鏀寔澶氭祦

璇ュ浘鏄娇鐢ㄤ互涓嬪懡浠ょ敓鎴愮殑锛?

   fdp -Gsplines=true -Tsvg < dot > dot.svg

### 浣跨敤 IPU6 ISYS 鎹曡幏甯?

IPU6 ISYS 鐢ㄤ簬浠庤繛鎺ュ埌 CSI2 绔彛鐨勬憚鍍忓ご浼犳劅鍣ㄦ崟鑾峰抚銆侷SYS 鏀寔鐨勮緭鍏ユ牸寮忓垪浜庝笅
琛細


    :header-rows: 1

    - - IPU6 ISYS 鏀寔鐨勮緭鍏ユ牸寮?
    - - RGB565, RGB888

    - - UYVY8, YUYV8

    - - RAW8, RAW10, RAW12


#### 绀轰緥


浠ヤ笅鏄?IPU6 ISYS 鍦?Dell XPS 9315 绗旇鏈笂杩涜 raw 鎹曡幏鐨勭ず渚嬨€傚湪璇ユ満鍣ㄤ笂锛宱v01a10
浼犳劅鍣ㄨ繛鎺ュ埌 IPU ISYS CSI-2 绔彛 2锛屽彲浠ヤ互 1280x800 鍒嗚鲸鐜囩敓鎴?sBGGR10 鍥惧儚銆?
浣跨敤濯掍綋鎺у埗鍣?API锛屾垜浠彲浠ラ€氳繃 media-ctl [#f1]_ 鍜?yavta [#f2]_ 閰嶇疆 ov01a10
浼犳劅鍣紝灏嗗抚浼犺緭鍒?IPU6 ISYS銆?

    # Example 1 capture frame from ov01a10 camera sensor
    # This example assumes /dev/media0 as the IPU ISYS media device
    export MDEV=/dev/media0

    # Establish the link for the media devices using media-ctl
    media-ctl -d $MDEV -l "\"ov01a10 3-0036\":0 -> \"Intel IPU6 CSI2 2\":0[^1^]"

    # Set the format for the media devices
    media-ctl -d $MDEV -V "ov01a10:0 [fmt:SBGGR10/1280x800]"
    media-ctl -d $MDEV -V "Intel IPU6 CSI2 2:0 [fmt:SBGGR10/1280x800]"
    media-ctl -d $MDEV -V "Intel IPU6 CSI2 2:1 [fmt:SBGGR10/1280x800]"

閰嶇疆濂藉獟浣撴祦姘寸嚎涔嬪悗锛屽彲浠ヤ娇鐢?yavta 宸ュ叿璁剧疆鎵€闇€鐨勪紶鎰熷櫒鐗瑰畾璁剧疆锛堜緥濡傛洕鍏夊拰澧炵泭
璁剧疆锛夈€?
渚嬪


    # and that ov01a10 sensor is connected to i2c bus 3 with address 0x36
    export SDEV=$(media-ctl -d $MDEV -e "ov01a10 3-0036")

    yavta -w 0x009e0903 400 $SDEV
    yavta -w 0x009e0913 1000 $SDEV
    yavta -w 0x009e0911 2000 $SDEV

璁剧疆濂芥墍闇€鐨勪紶鎰熷櫒璁剧疆鍚庯紝灏卞彲浠ュ涓嬭繘琛屽抚鎹曡幏銆?
渚嬪


    yavta --data-prefix -u -c10 -n5 -I -s 1280x800 --file=/tmp/frame-#.bin \
            -f SBGGR10 $(media-ctl -d $MDEV -e "Intel IPU6 ISYS Capture 0")

閫氳繃涓婅堪鍛戒护锛屼互 1280x800 鍒嗚鲸鐜囧拰 sBGGR10 鏍煎紡鎹曡幏 10 甯с€傛崟鑾风殑甯т互
/tmp/frame-#.bin 鏂囦欢鐨勫舰寮忔彁渚涖€?
浠ヤ笅鏄彟涓€涓ず渚嬶紝鍦?Lenovo X1 Yoga 绗旇鏈笂浠庢憚鍍忓ご浼犳劅鍣?ov2740 杩涜 IPU6 ISYS
RAW 鍜屽厓鏁版嵁鎹曡幏銆?

    media-ctl -l "\"ov2740 14-0036\":0 -> \"Intel IPU6 CSI2 1\":0[^1^]"
    media-ctl -l "\"Intel IPU6 CSI2 1\":1 -> \"Intel IPU6 ISYS Capture 0\":0[^1^]"
    media-ctl -l "\"Intel IPU6 CSI2 1\":2 -> \"Intel IPU6 ISYS Capture 1\":0[^1^]"

    # set routing
    media-ctl -R "\"Intel IPU6 CSI2 1\" [0/0->1/0[^1^],0/1->2/1[^1^]]"

    media-ctl -V "\"Intel IPU6 CSI2 1\":0/0 [fmt:SGRBG10/1932x1092]"
    media-ctl -V "\"Intel IPU6 CSI2 1\":0/1 [fmt:GENERIC_8/97x1]"
    media-ctl -V "\"Intel IPU6 CSI2 1\":1/0 [fmt:SGRBG10/1932x1092]"
    media-ctl -V "\"Intel IPU6 CSI2 1\":2/1 [fmt:GENERIC_8/97x1]"

    CAPTURE_DEV=$(media-ctl -e "Intel IPU6 ISYS Capture 0")
    ./yavta --data-prefix -c100 -n5 -I -s1932x1092 --file=/tmp/frame-#.bin \
        -f SGRBG10 ${CAPTURE_DEV}

    CAPTURE_META=$(media-ctl -e "Intel IPU6 ISYS Capture 1")
    ./yavta --data-prefix -c100 -n5 -I -s97x1 -B meta-capture \
        --file=/tmp/meta-#.bin -f GENERIC_8 ${CAPTURE_META}

## 鍙傝€冭祫鏂?

