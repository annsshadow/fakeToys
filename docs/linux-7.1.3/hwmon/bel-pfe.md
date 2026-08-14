## 鍐呮牳椹卞姩 bel-pfe


鏀寔鐨勮姱鐗囷細

  - BEL PFE1100

    Prefixes: 'pfe1100'

    Addresses scanned: -

    Datasheet: https://www.belfuse.com/resources/datasheets/powersolutions/ds-bps-pfe1100-12-054xa.pdf

  - BEL PFE3000

    Prefixes: 'pfe3000'

    Addresses scanned: -

    Datasheet: https://www.belfuse.com/resources/datasheets/powersolutions/ds-bps-pfe3000-series.pdf

浣滆€咃細Tao Ren <rentao.bupt@gmail.com>


### 鎻忚堪


璇ラ┍鍔ㄦ敮鎸佷互涓嬫敮鎸?PMBus 鍗忚鐨勭數婧愯澶囩殑纭欢鐩戞帶锛?

  - BEL PFE1100

    1100 鐡?AC 杞?DC 鍔熺巼鍥犳暟鏍℃锛圥FC锛夌數婧愩€侾MBus 閫氫俊鎵嬪唽鏈叕寮€鎻愪緵銆?

  - BEL PFE3000

    3000 鐡?AC/DC 鍔熺巼鍥犳暟鏍℃锛圥FC锛変笌 DC-DC 鐢垫簮銆侾MBus 閫氫俊鎵嬪唽鏈叕寮€鎻愪緵銆?

璇ラ┍鍔ㄦ槸鏍稿績 PMBus 椹卞姩鐨勫鎴风椹卞姩銆傛湁鍏?PMBus 瀹㈡埛绔┍鍔ㄧ殑璇︽儏锛岃鍙傞槄 Documentation/hwmon/pmbus.rst銆?


### 浣跨敤璇存槑


璇ラ┍鍔ㄤ笉浼氳嚜鍔ㄦ娴嬭澶囥€備綘闇€瑕佹樉寮忓疄渚嬪寲璁惧銆傝鎯呰鍙傞槄 Documentation/i2c/instantiating-devices.rst銆?

绀轰緥锛氫互涓嬪懡浠ゅ皢涓哄湴鍧€ 0x20 澶勭殑 PFE3000 鍔犺浇椹卞姩
```

	$ modprobe bel-pfe
	$ echo pfe3000 0x20 > /sys/bus/i2c/devices/i2c-1/new_device


```

### 骞冲彴鏁版嵁鏀寔


璇ラ┍鍔ㄦ敮鎸佹爣鍑嗙殑 PMBus 椹卞姩骞冲彴鏁版嵁銆?


### Sysfs 鏉＄洰


======================= =======================================================
curr1_label		"iin"
curr1_input		娴嬮噺鐨勮緭鍏ョ數娴?
curr1_max               杈撳叆鐢垫祦鏈€澶у€?
curr1_max_alarm         杈撳叆鐢垫祦鏈€澶ф姤璀?

curr[2-3]_label		"iout[1-2]"
curr[2-3]_input		娴嬮噺鐨勮緭鍑虹數娴?
curr[2-3]_max           杈撳嚭鐢垫祦鏈€澶у€?
curr[2-3]_max_alarm     杈撳嚭鐢垫祦鏈€澶ф姤璀?

fan[1-2]_input          椋庢墖 1 涓?2 鐨勮浆閫燂紙RPM锛?
fan1_target             涓轰袱涓鎵囪缃浆閫熷弬鑰?

in1_label		"vin"
in1_input		娴嬮噺鐨勮緭鍏ョ數鍘?
in1_crit		杈撳叆鐢靛帇涓寸晫鏈€澶у€?
in1_crit_alarm		杈撳叆鐢靛帇涓寸晫鏈€澶ф姤璀?
in1_lcrit               杈撳叆鐢靛帇涓寸晫鏈€灏忓€?
in1_lcrit_alarm         杈撳叆鐢靛帇涓寸晫鏈€灏忔姤璀?
in1_max                 杈撳叆鐢靛帇鏈€澶у€?
in1_max_alarm           杈撳叆鐢靛帇鏈€澶ф姤璀?

in2_label               "vcap"
in2_input               淇濇寔鐢靛鐢靛帇

in[3-8]_label		"vout[1-3,5-7]"
in[3-8]_input		娴嬮噺鐨勮緭鍑虹數鍘?
in[3-4]_alarm           vout[1-2] 杈撳嚭鐢靛帇鎶ヨ

power[1-2]_label	"pin[1-2]"
power[1-2]_input        娴嬮噺鐨勮緭鍏ュ姛鐜?
power[1-2]_alarm	杈撳叆鍔熺巼杩囬珮鎶ヨ

power[3-4]_label	"pout[1-2]"
power[3-4]_input	娴嬮噺鐨勮緭鍑哄姛鐜?

temp[1-3]_input		娴嬮噺鐨勬俯搴?
temp[1-3]_alarm         娓╁害鎶ヨ
======================= =======================================================


    - curr3銆乫an2銆乿out[2-7]銆乿cap銆乸in2銆乸out2 涓?temp3 灞炴€т粎瀛樺湪浜?PFE3000銆?
