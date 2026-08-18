
## 鍐呮牳椹卞姩 stpddc60


Supported chips:

  - ST STPDDC60

    Prefix: 'stpddc60', 'bmr481'

    Addresses scanned: -

    Datasheet: https://flexpowermodules.com/documents/fpm-techspec-bmr481

Author: Erik Rosen <erik.rosen@metormote.com>


### 鎻忚堪


鏈┍鍔ㄦ敮鎸佸 ST STPDDC60 鎺у埗鍣ㄨ姱鐗囧強鍏煎妯″潡鐨勭‖浠剁洃鎺с€?
璇ラ┍鍔ㄦ槸鏍稿績 PMBus 椹卞姩鐨勫鎴风椹卞姩銆傛湁鍏?PMBus 瀹㈡埛绔┍鍔ㄧ殑璇︾粏淇℃伅锛岃鍙傞槄 Documentation/hwmon/pmbus.rst 涓?Documentation.hwmon/pmbus-core銆?

### 浣跨敤娉ㄦ剰浜嬮」


鏈┍鍔ㄤ笉浼氳嚜鍔ㄦ娴嬭澶囥€備綘闇€瑕佹樉寮忓湴瀹炰緥鍖栬澶囥€傝鎯呰鍙傞槄 Documentation/i2c/instantiating-devices.rst銆?
vout 鐨勪笅闄愪笌涓婇檺杩囧帇闄愬€硷紝鏄浉瀵逛簬鎸囦护杈撳嚭鐢靛帇銆佷互 50mV 鍒?400mV 鍖洪棿銆佹寜 50mV 姝ラ暱鐨勬鎴栬礋鍋忕Щ鏉ヨ缃殑銆傝繖鎰忓懗鐫€褰撴寚浠よ緭鍑虹數鍘嬫敼鍙樻椂锛岃繖浜涢檺鍊肩殑缁濆鍊间篃浼氭敼鍙樸€傛澶栵紝鍦ㄥ啓鍏ヨ繖浜涢檺鍊兼椂搴斿綋灏忓績锛屽洜涓哄湪鏈€鍧忔儏鍐典笅锛屾寚浠よ緭鍑虹數鍘嬪彲鑳戒細涓庨檺鍊煎啓鍏ュ悓鏃舵敼鍙橈紝浠庤€屽鑷翠笉鍙娴嬬殑缁撴灉銆?

### 骞冲彴鏁版嵁鏀寔


璇ラ┍鍔ㄦ敮鎸佹爣鍑嗙殑 PMBus 椹卞姩骞冲彴鏁版嵁銆?

### Sysfs 鏉＄洰


鏀寔浠ヤ笅灞炴€с€俈in銆乮out銆乸out 涓?temp 闄愬€间负璇诲啓锛涙墍鏈夊叾浠栧睘鎬у潎涓哄彧璇汇€?
======================= ========================================================
in1_label		"vin"
in1_input		娴嬪緱鐨勮緭鍏ョ數鍘嬨€?in1_lcrit		涓寸晫鏈€灏忚緭鍏ョ數鍘嬨€?in1_crit		涓寸晫鏈€澶ц緭鍏ョ數鍘嬨€?in1_lcrit_alarm		杈撳叆鐢靛帇涓寸晫浣庢姤璀︺€?in1_crit_alarm		杈撳叆鐢靛帇涓寸晫楂樻姤璀︺€?
in2_label		"vout1"
in2_input		娴嬪緱鐨勮緭鍑虹數鍘嬨€?in2_lcrit		涓寸晫鏈€灏忚緭鍑虹數鍘嬨€?in2_crit		涓寸晫鏈€澶ц緭鍑虹數鍘嬨€?in2_lcrit_alarm		杈撳嚭鐢靛帇涓寸晫浣庢姤璀︺€?in2_crit_alarm		杈撳嚭鐢靛帇涓寸晫楂樻姤璀︺€?
curr1_label		"iout1"
curr1_input		娴嬪緱鐨勮緭鍑虹數娴併€?curr1_max		鏈€澶ц緭鍑虹數娴併€?curr1_max_alarm		杈撳嚭鐢垫祦楂樻姤璀︺€?curr1_crit		涓寸晫鏈€澶ц緭鍑虹數娴併€?curr1_crit_alarm	杈撳嚭鐢垫祦涓寸晫楂樻姤璀︺€?
power1_label		"pout1"
power1_input		娴嬪緱鐨勮緭鍑哄姛鐜囥€?power1_crit		涓寸晫鏈€澶ц緭鍑哄姛鐜囥€?power1_crit_alarm	杈撳嚭鍔熺巼涓寸晫楂樻姤璀︺€?
temp1_input		娴嬪緱鐨勬墍鏈夌浉鐨勬渶澶ф俯搴︺€?temp1_max		鏈€楂樻俯搴﹂檺鍊笺€?temp1_max_alarm		楂樻俯鎶ヨ銆?temp1_crit		涓寸晫鏈€楂樻俯搴﹂檺鍊笺€?temp1_crit_alarm	涓寸晫鏈€楂樻俯搴︽姤璀︺€?======================= ========================================================
