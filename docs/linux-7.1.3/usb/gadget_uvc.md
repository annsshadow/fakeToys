## Linux UVC Gadget 椹卞姩

### Overview锛堟杩帮級

UVC Gadget 椹卞姩鏄竴涓敤浜?USB 杩炴帴涓?**璁惧锛坉evice锛?* 渚х‖浠剁殑椹卞姩銆傚畠鎰忓湪杩愯鍦ㄥ叿澶?USB 璁惧渚х‖浠讹紙渚嬪甯︽湁 OTG 绔彛鐨勫紑鍙戞澘锛夌殑 Linux 绯荤粺涓娿€?
鍦ㄨ澶囩郴缁熶笂锛屼竴鏃﹂┍鍔ㄨ缁戝畾锛屽畠灏变細琛ㄧ幇涓轰竴涓叿鏈夎緭鍑鸿兘鍔涚殑 V4L2 璁惧銆?
鍦ㄤ富鏈轰晶锛堜竴鏃﹂€氳繃 USB 绾跨紗杩炴帴锛夛紝杩愯 UVC Gadget 椹卞姩 **骞剁敱鎭板綋鐨勭敤鎴风┖闂寸▼搴忔帶鍒?* 鐨勮澶囧簲褰撹〃鐜颁负涓€涓鍚?UVC 瑙勮寖鐨勬憚鍍忓ご锛屽苟鑳戒笌浠讳綍涓哄鐞嗚繖绫昏澶囪€岃璁＄殑绋嬪簭姝ｅ父閰嶅悎宸ヤ綔銆傝繍琛屽湪璁惧绯荤粺涓婄殑鐢ㄦ埛绌洪棿绋嬪簭鍙互浠庡悇绉嶆潵婧愭帓闃熷浘鍍忕紦鍐插尯锛屼互渚块€氳繃 USB 杩炴帴浼犺緭銆傞€氬父杩欐剰鍛崇潃浠庢憚鍍忓ご浼犳劅鍣ㄥ璁捐浆鍙戠紦鍐插尯锛屼絾缂撳啿鍖虹殑鏉ユ簮瀹屽叏鍙栧喅浜庣敤鎴风┖闂寸殑閰嶅绋嬪簭銆?
### Configuring the device kernel锛堥厤缃澶囧唴鏍革級

蹇呴』閫変腑 Kconfig 閫夐」 USB_CONFIGFS銆乁SB_LIBCOMPOSITE銆乁SB_CONFIGFS_F_UVC 鍜?USB_F_UVC 浠ュ惎鐢ㄥ UVC gadget 鐨勬敮鎸併€?
### Configuring the gadget through configfs锛堥€氳繃 configfs 閰嶇疆 gadget锛?
UVC Gadget 鏈熸湜閫氳繃 configfs 浣跨敤 UVC 鍑芥暟鏉ラ厤缃€傝繖鎻愪緵浜嗙浉褰撶▼搴︾殑鐏垫椿鎬э紝鍥犱负 UVC 璁惧鐨勮澶氳缃兘鍙互閫氳繃杩欑鏂瑰紡鏉ユ帶鍒躲€?
姝ゅ骞舵湭鎻忚堪鎵€鏈夊彲鐢ㄥ睘鎬с€傚畬鏁寸殑鏋氫妇璇疯 Documentation/ABI/testing/configfs-usb-gadget-uvc

#### Assumptions锛堝墠鎻愬亣璁撅級

鏈妭鍋囪鎮ㄥ凡灏?configfs 鎸傝浇鍒?`/sys/kernel/config`锛屽苟宸插皢鏌愪釜 gadget 鍒涘缓涓?`/sys/kernel/config/usb_gadget/g1`銆?
#### The UVC Function锛圲VC 鍑芥暟锛?
绗竴姝ユ槸鍒涘缓 UVC 鍑芥暟锛?

	# These variables will be assumed throughout the rest of the document
	CONFIGFS="/sys/kernel/config"
	GADGET="$CONFIGFS/usb_gadget/g1"
	FUNCTION="$GADGET/functions/uvc.0"

	mkdir -p $FUNCTION

#### Formats and Frames锛堟牸寮忎笌甯э級

鎮ㄥ繀椤婚€氳繃鍛婄煡 gadget 鎮ㄦ墍鏀寔鐨勬牸寮忥紝浠ュ強姣忕鏍煎紡鎵€鏀寔鐨勫抚澶у皬涓庡抚闂撮殧锛屾潵閰嶇疆 gadget銆傚湪褰撳墠瀹炵幇涓紝gadget 娌℃湁鍔炴硶鎷掔粷涓绘満鎸囦护瀹冭缃殑鏌愪釜鏍煎紡锛屽洜姝ゆ湰姝ラ **鍑嗙‘鍦?* 瀹屾垚闈炲父閲嶈锛屼互纭繚涓绘満姘歌繙涓嶄細璇锋眰涓€涓棤娉曟彁渚涚殑鏍煎紡銆?
鏍煎紡鍒涘缓浜?streaming/uncompressed 鍜?streaming/mjpeg 杩欎袱涓?configfs 缁勪箣涓嬶紝甯уぇ灏忓垯鍒涘缓浜庢牸寮忎箣涓嬶紝鍏剁粨鏋勫涓嬶細

```

	uvc.0 +
	      |
	      + streaming +
			  |
			  + mjpeg +
			  |       |
			  |       + mjpeg +
			  |	       |
			  |	       + 720p
			  |	       |
			  |	       + 1080p
			  |
			  + uncompressed +
					 |
					 + yuyv +
						|
						+ 720p
						|
						+ 1080p

```

姣忎釜甯ч殢鍚庡彲浠ラ厤缃搴﹀拰楂樺害锛屽姞涓婂瓨鍌ㄥ崟甯ф墍闇€鐨勬渶澶х紦鍐插尯澶у皬锛屾渶鍚庢槸鐩稿簲鏍煎紡鍜屽抚澶у皬鎵€鏀寔鐨勫抚闂撮殧銆傚搴﹀拰楂樺害浠ュ儚绱犱负鍗曚綅鏋氫妇锛屽抚闂撮殧浠?100ns 涓哄崟浣嶃€備緥濡傦紝瑕佷负涓婇潰瀵规瘡涓抚澶у皬鍒涘缓鍚?2銆?5 鍜?100 fps 甯ч棿闅旂殑缁撴瀯锛屾偍鍙互杩欐牱鍋氾細


	create_frame() {
		# Example usage:
		# create_frame <width> <height> <group> <format name>

		WIDTH=$1
		HEIGHT=$2
		FORMAT=$3
		NAME=$4

		wdir=$FUNCTION/streaming/$FORMAT/$NAME/${HEIGHT}p

		mkdir -p $wdir
		echo $WIDTH > $wdir/wWidth
		echo $HEIGHT > $wdir/wHeight
		echo $(( $WIDTH ** $HEIGHT ** 2 )) > $wdir/dwMaxVideoFrameBufferSize
		cat <<EOF > $wdir/dwFrameInterval
	666666
	100000
	5000000
	EOF
	}

	create_frame 1280 720 mjpeg mjpeg
	create_frame 1920 1080 mjpeg mjpeg
	create_frame 1280 720 uncompressed yuyv
	create_frame 1920 1080 uncompressed yuyv

褰撳墠鍞竴鏀寔鐨勯潪鍘嬬缉鏍煎紡鏄?YUYV锛屽叾缁嗚妭瑙?Documentation/userspace-api/media/v4l/pixfmt-packed-yuv.rst銆?
#### Color Matching Descriptors锛堣壊褰╁尮閰嶆弿杩扮锛?
鍙互涓烘偍鍒涘缓鐨勬瘡涓牸寮忔寚瀹氫竴浜涜壊搴︼紙colorimetry锛変俊鎭€傝繖涓€姝ユ槸鍙€夌殑锛屽鏋滆烦杩囷紝灏嗗寘鍚粯璁や俊鎭紱杩欎簺榛樿鍊奸伒寰?UVC 瑙勮寖涓?鈥滆壊褰╁尮閰嶆弿杩扮鈥濓紙Color Matching Descriptor锛変竴鑺傜殑瀹氫箟銆?
瑕佸垱寤轰竴涓壊褰╁尮閰嶆弿杩扮锛岄渶鍒涘缓涓€涓?configfs 椤瑰苟灏嗗叾涓変釜灞炴€ц涓烘湡鏈涚殑璁剧疆锛岀劧鍚庝粠鎮ㄥ笇鏈涘畠鍏宠仈鍒扮殑鏍煎紡澶勫缓绔嬫寚鍚戝畠鐨勯摼鎺ワ細


	# Create a new Color Matching Descriptor

	mkdir $FUNCTION/streaming/color_matching/yuyv
	pushd $FUNCTION/streaming/color_matching/yuyv

	echo 1 > bColorPrimaries
	echo 1 > bTransferCharacteristics
	echo 4 > bMatrixCoefficients

	popd

	# Create a symlink to the Color Matching Descriptor from the format's config item
	ln -s $FUNCTION/streaming/color_matching/yuyv $FUNCTION/streaming/uncompressed/yuyv

鏈夊叧鏈夋晥鍙栧€肩殑璇︾粏璇存槑锛岃鏌ラ槄 UVC 瑙勮寖銆傛敞鎰忥紝瀛樺湪涓€涓粯璁ょ殑鑹插僵鍖归厤鎻忚堪绗︼紝骞惰浠讳綍娌℃湁閾炬帴鍒板叾浠栬壊褰╁尮閰嶆弿杩扮鐨勬牸寮忔墍浣跨敤銆傚彲浠ユ洿鏀归粯璁ゆ弿杩扮鐨勫睘鎬ц缃紝鍥犳璇疯浣忥紝濡傛灉鎮ㄨ繖鏍峰仛锛屽氨鏄湪鏇存敼浠讳綍鏈摼鎺ュ埌鍏朵粬鎻忚堪绗︾殑鏍煎紡鐨勯粯璁ゅ€笺€?

#### Header linking锛堝ご閮ㄩ摼鎺ワ級

UVC 瑙勮寖瑕佹眰 Format 鍜?Frame 鎻忚堪绗︿箣鍓嶈鏈?Header锛岀敤浜庢弿杩拌濡備笅鏂囦笉鍚?Format 鎻忚堪绗︾殑鏁伴噺涓庣疮璁″ぇ灏忕瓑淇℃伅銆傝繖涓€姝ヤ互鍙婄被浼肩殑鎿嶄綔锛屽湪 configfs 涓€氳繃閾炬帴浠ｈ〃 header 鐨?configfs 椤逛笌浠ｈ〃閭ｄ簺鍏朵粬鎻忚堪绗︾殑 config 椤规潵瀹炵幇锛屾柟寮忓涓嬶細


	mkdir $FUNCTION/streaming/header/h

	# This section links the format descriptors and their associated frames
	# to the header
	cd $FUNCTION/streaming/header/h
	ln -s ../../uncompressed/yuyv
	ln -s ../../mjpeg/mjpeg

	# This section ensures that the header will be transmitted for each
	# speed's set of descriptors. If support for a particular speed is not
	# needed then it can be skipped here.
	cd ../../class/fs
	ln -s ../../header/h
	cd ../../class/hs
	ln -s ../../header/h
	cd ../../class/ss
	ln -s ../../header/h
	cd ../../../control
	mkdir header/h
	ln -s header/h class/fs
	ln -s header/h class/ss


#### Extension Unit Support锛堟墿灞曞崟鍏冩敮鎸侊級

涓€涓?UVC 鎵╁睍鍗曞厓锛圶U锛夋湰璐ㄤ笂鎻愪緵浜嗕竴涓嫭绔嬬殑鍗曞厓锛屾帶鍒?set 鍜?get 璇锋眰鍙互瀵诲潃鍒板畠銆傝繖浜涙帶鍒惰姹傜殑鍚箟瀹屽叏鍙栧喅浜庡疄鐜帮紝浣嗗彲鐢ㄤ簬鎺у埗鍦?UVC 瑙勮寖涔嬪鐨勮缃紙渚嬪鍚敤鎴栫鐢ㄨ棰戠壒鏁堬級銆備竴涓?XU 鍙互鎻掑叆鍒?UVC 鍗曞厓閾句腑锛屼篃鍙互淇濇寔娓哥銆?
閰嶇疆鎵╁睍鍗曞厓娑夊強鍦ㄧ浉搴旂殑鐩綍涓垱寤轰竴涓潯鐩苟鎭板綋鍦拌缃叾灞炴€э紝濡備笅鎵€绀猴細


	mkdir $FUNCTION/control/extensions/xu.0
	pushd $FUNCTION/control/extensions/xu.0

	# Set the bUnitID of the Processing Unit as the source for this
	# Extension Unit
	echo 2 > baSourceID

	# Set this XU as the source of the default output terminal. This inserts
	# the XU into the UVC chain between the PU and OT such that the final
	# chain is IT > PU > XU.0 > OT
	cat bUnitID > ../../terminal/output/default/baSourceID

	# Flag some controls as being available for use. The bmControl field is
	# a bitmap with each bit denoting the availability of a particular
	# control. For example to flag the 0th, 2nd and 3rd controls available:
	echo 0x0d > bmControls

	# Set the GUID; this is a vendor-specific code identifying the XU.
	echo -e -n "\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10" > guidExtensionCode

	popd

bmControls 灞炴€у拰 baSourceID 灞炴€ф槸澶氬€煎睘鎬с€傝繖鎰忓懗鐫€鎮ㄥ彲浠ュ悜瀹冧滑鍐欏叆澶氫釜浠ユ崲琛屽垎闅旂殑鍊笺€備緥濡傝灏嗙 1銆?銆?銆?0 涓帶鍒舵爣璁颁负鍙敤锛屾偍闇€瑕佸悜 bmControls 鍐欏叆涓や釜鍊硷紝濡備笅鎵€绀猴細


	cat << EOF > bmControls
	0x03
	0x03
	EOF

baSourceID 灞炴€х殑澶氬€肩壒鎬ф帺鐩栦簡 XU 鍙互鏄杈撳叆杩欎竴浜嬪疄锛屼笉杩囪娉ㄦ剰锛岀洰鍓嶈繖骞舵病鏈変粈涔堟樉钁楀奖鍝嶃€?
bControlSize 灞炴€у弽鏄犱簡 bmControls 灞炴€х殑澶у皬锛岀被浼煎湴锛宐NrInPins 鍙嶆槧浜?baSourceID 灞炴€х殑澶у皬銆傚綋鎮ㄨ缃?bmControls 鍜?baSourceID 鏃讹紝杩欎袱涓睘鎬ч兘浼氳嚜鍔ㄥ澶?鍑忓皬銆備篃鍙互鎵嬪姩澧炲ぇ鎴栧噺灏?bControlSize锛屽叾鏁堟灉鏄皢鏉＄洰鎴柇鍒版柊澶у皬锛屾垨鐢?0x00 濉厖鏉＄洰锛屼緥濡傦細

```

	$ cat bmControls
	0x03
	0x05

	$ cat bControlSize
	2

	$ echo 1 > bControlSize
	$ cat bmControls
	0x03

	$ echo 2 > bControlSize
	$ cat bmControls
	0x03
	0x00

```

bNrInPins 鍜?baSourceID 浠ョ浉鍚屾柟寮忓伐浣溿€?
#### Configuring Supported Controls for Camera Terminal and Processing Unit锛堜负 Camera Terminal 鍜?Processing Unit 閰嶇疆鍙楁敮鎸佺殑鎺у埗锛?
UVC 閾句腑鐨?Camera Terminal 鍜?Processing Unit 涔熸嫢鏈?bmControls 灞炴€э紝鍏朵綔鐢ㄧ被浼间簬鎵╁睍鍗曞厓涓殑鍚屽悕瀛楁銆備笉杩囦笌 XU 涓嶅悓鐨勬槸锛岃繖浜涘崟鍏冪殑浣嶆爣蹇楀惈涔夊湪 UVC 瑙勮寖涓湁瀹氫箟锛涙偍搴斿綋鏌ラ槄 鈥淐amera Terminal Descriptor鈥?鍜?鈥淧rocessing Unit Descriptor鈥?涓よ妭浠ヨ幏鍙栬繖浜涙爣蹇楃殑鏋氫妇銆?

        # Set the Processing Unit's bmControls, flagging Brightness, Contrast
        # and Hue as available controls:
        echo 0x05 > $FUNCTION/control/processing/default/bmControls

        # Set the Camera Terminal's bmControls, flagging Focus Absolute and
        # Focus Relative as available controls:
        echo 0x60 > $FUNCTION/control/terminal/camera/default/bmControls

濡傛灉鎮ㄤ笉璁剧疆杩欎簺瀛楁锛岄粯璁ゆ儏鍐典笅 Camera Terminal 鐨?Auto-Exposure Mode 鎺у埗鍜?Processing Unit 鐨?Brightness 鎺у埗浼氳鏍囪涓哄彲鐢紱濡傛灉瀹冧滑涓嶈鏀寔锛屾偍搴斿綋灏嗚瀛楁璁句负 0x00銆?
娉ㄦ剰锛孋amera Terminal 鎴?Processing Unit 鐨?bmControls 瀛楁鐨勫ぇ灏忕敱 UVC 瑙勮寖鍥哄畾锛屽洜姝よ繖閲岀殑 bControlSize 灞炴€ф槸鍙鐨勩€?
#### Custom Strings Support锛堣嚜瀹氫箟瀛楃涓叉敮鎸侊級

涓?USB 璁惧鍚勯儴鍒嗘彁渚涙枃瀛楁弿杩扮殑瀛楃涓叉弿杩扮锛屽彲浠ュ湪 USB configfs 涓€氬父鐨勪綅缃畾涔夛紝鐒跺悗鍙互浠?UVC 鍑芥暟鏍圭洰褰曟垨鎵╁睍鍗曞厓鐩綍閾炬帴杩囧幓锛屼互灏嗚繖浜涘瓧绗︿覆鎸囨淳涓烘弿杩扮锛?

	# Create a string descriptor in us-EN and link to it from the function
	# root. The name of the link is significant here, as it declares this
	# descriptor to be intended for the Interface Association Descriptor.
	# Other significant link names at function root are vs0_desc and vs1_desc
	# For the VideoStreaming Interface 0/1 Descriptors.

	mkdir -p $GADGET/strings/0x409/iad_desc
	echo -n "Interface Associaton Descriptor" > $GADGET/strings/0x409/iad_desc/s
	ln -s $GADGET/strings/0x409/iad_desc $FUNCTION/iad_desc

	# Because the link to a String Descriptor from an Extension Unit clearly
	# associates the two, the name of this link is not significant and may
	# be set freely.

	mkdir -p $GADGET/strings/0x409/xu.0
	echo -n "A Very Useful Extension Unit" > $GADGET/strings/0x409/xu.0/s
	ln -s $GADGET/strings/0x409/xu.0 $FUNCTION/control/extensions/xu.0

#### The interrupt endpoint锛堜腑鏂鐐癸級

VideoControl 鎺ュ彛鏈変竴涓彲閫夌殑涓柇绔偣锛岄粯璁ゆ槸绂佺敤鐨勩€傚畠鏃ㄥ湪鏀寔 UVC 鐨勫欢杩熷搷搴旀帶鍒?set 璇锋眰锛堝簲褰撻€氳繃璇ヤ腑鏂鐐硅€岄潪鍗犵敤绔偣 0 鏉ュ搷搴旓級銆傜洰鍓嶅皻涓嶆敮鎸侀€氳繃璇ョ鐐瑰彂閫佹暟鎹紝鍥犳灏嗗叾淇濇寔绂佺敤浠ュ厤娣锋穯銆傚鏋滄偍甯屾湜鍚敤瀹冿紝鍙互閫氳繃 configfs 灞炴€ф潵鍋氬埌锛?

	echo 1 > $FUNCTION/control/enable_interrupt_ep

#### Bandwidth configuration锛堝甫瀹介厤缃級

鏈変笁涓睘鎬ф帶鍒?USB 杩炴帴鐨勫甫瀹姐€傚畠浠綅浜庡嚱鏁版牴鐩綍锛屽彲浠ュ湪闄愬埗鑼冨洿鍐呰缃細


	# streaming_interval sets bInterval. Values range from 1..255
	echo 1 > $FUNCTION/streaming_interval

	# streaming_maxpacket sets wMaxPacketSize. Valid values are 1024/2048/3072
	echo 3072 > $FUNCTION/streaming_maxpacket

	# streaming_maxburst sets bMaxBurst. Valid values are 1..15
	echo 1 > $FUNCTION/streaming_maxburst


杩欓噷浼犲叆鐨勫€间細鏍规嵁 UVC 瑙勮寖锛堝彇鍐充簬 USB 杩炴帴鐨勯€熷害锛夎閽冲埗鍒版湁鏁堝€笺€傝鐞嗚В杩欎簺璁剧疆濡備綍褰卞搷甯﹀锛屾偍搴斿綋鏌ラ槄 UVC 瑙勮寖锛屼絾涓€鏉＄粡楠屾硶鍒欐槸锛氬澶?streaming_maxpacket 璁剧疆浼氭彁鍗囧甫瀹斤紙浠庤€屾彁鍗囨渶澶у彲鑳界殑甯х巼锛夛紝鍦?USB 杩炴帴杩愯浜?SuperSpeed 鏃讹紝streaming_maxburst 鍚岀悊銆傚澶?streaming_interval 浼氶檷浣庡甫瀹藉拰甯х巼銆?
### The userspace application锛堢敤鎴风┖闂村簲鐢ㄧ▼搴忥級

鍗曞嚟 UVC Gadget 椹卞姩鏈韩鏃犳硶鍋氫换浣曠壒鍒湁瓒ｇ殑浜嬨€傚畠蹇呴』涓庝竴涓搷搴旂敤 UVC 鎺у埗璇锋眰銆佸苟濉厖缂撳啿鍖轰互渚挎帓闃熷埌椹卞姩鎵€鍒涘缓鐨?V4L2 璁惧鐨勭敤鎴风┖闂寸▼搴忛厤鍚堜娇鐢ㄣ€傝繖浜涗簨鎯呭浣曡揪鎴愬彇鍐充簬鍏蜂綋瀹炵幇锛岃秴鍑轰簡鏈枃妗ｇ殑鑼冨洿锛屼絾鍙互鍦?https://gitlab.freedesktop.org/camera/uvc-gadget 鎵惧埌涓€涓弬鑰冨簲鐢ㄧ▼搴?