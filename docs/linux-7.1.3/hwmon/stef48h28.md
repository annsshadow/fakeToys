
## 鍐呮牳椹卞姩 stef48h28


鏀寔鐨勮姱鐗囷細

  - Analog Devices STEF48H28

    Prefix: 'stef48h28'

    Addresses scanned: -

    Datasheet: https://www.st.com/resource/en/data_brief/stef48h28.pdf

Author:

  - Charles Hsu <hsu.yungteng@gmail.com>


### 鎻忚堪


STEF48H28 鏄竴娆鹃潰鍚?9-80 V DC 鐢垫簮杞ㄧ殑 30 A 闆嗘垚 e-fuse銆傚畠鎻愪緵娑屽叆鐢垫祦鎺у埗锛坕nrush control锛夈€佹瑺鍘?杩囧帇閿佸畾锛坲ndervoltage/overvoltage lockout锛変互鍙婁娇鐢ㄨ嚜閫傚簲锛圛 x t锛夋柟妗堢殑杩囨祦淇濇姢锛岃鏂规鍏佽 CPU/GPU 璐熻浇鍏稿瀷鐨勭煭鏃跺ぇ鐢垫祦鑴夊啿銆?
璇ュ櫒浠舵彁渚涙ā鎷熺數娴佺洃瑙嗚緭鍑轰笌鐗囦笂娓╁害鐩戣淇″彿鐢ㄤ簬绯荤粺鐩戠銆傚惎鍔ㄨ涓哄彲閫氳繃鎻掑叆寤惰繜锛坕nsertion-delay锛変笌杞惎鍔紙soft-start锛夎缃繘琛岀紪绋嬨€?
闄勫姞鐗规€у寘鎷數婧愯壇濂斤紙power-good锛夋寚绀恒€佽嚜妫€锛坰elf-diagnostics锛夈€佺儹鍏虫柇锛坱hermal shutdown锛変互鍙婄敤浜庨仴娴嬶紙telemetry锛変笌鐘舵€佹姤鍛婄殑 PMBus 鎺ュ彛銆?
### 骞冲彴鏁版嵁鏀寔


璇ラ┍鍔ㄦ敮鎸佹爣鍑嗙殑 PMBus 椹卞姩骞冲彴鏁版嵁銆?
### Sysfs 鎺ュ彛


======================  ========================================================
in1_label		"vin".
in1_input		娴嬪緱鐨勭數鍘嬨€傛潵鑷?READ_VIN 瀵勫瓨鍣ㄣ€?in1_min			鏈€灏忕數鍘嬨€傛潵鑷?VIN_UV_WARN_LIMIT 瀵勫瓨鍣ㄣ€?in1_max			鏈€澶х數鍘嬨€傛潵鑷?VIN_OV_WARN_LIMIT 瀵勫瓨鍣ㄣ€?
in2_label		"vout1".
in2_input		娴嬪緱鐨勭數鍘嬨€傛潵鑷?READ_VOUT 瀵勫瓨鍣ㄣ€?in2_min			鏈€灏忕數鍘嬨€傛潵鑷?VOUT_UV_WARN_LIMIT 瀵勫瓨鍣ㄣ€?in2_max			鏈€澶х數鍘嬨€傛潵鑷?VOUT_OV_WARN_LIMIT 瀵勫瓨鍣ㄣ€?
curr1_label "iin".      curr1_input 娴嬪緱鐨勭數娴併€傛潵鑷?READ_IIN 瀵勫瓨鍣ㄣ€?
curr2_label "iout1".    curr2_input 娴嬪緱鐨勭數娴併€傛潵鑷?READ_IOUT 瀵勫瓨鍣ㄣ€?
power1_label		"pin"
power1_input		娴嬪緱鐨勮緭鍏ュ姛鐜囥€傛潵鑷?READ_PIN 瀵勫瓨鍣ㄣ€?
power2_label		"pout1"
power2_input		娴嬪緱鐨勮緭鍑哄姛鐜囥€傛潵鑷?READ_POUT 瀵勫瓨鍣ㄣ€?
temp1_input		娴嬪緱鐨勬俯搴︺€傛潵鑷?READ_TEMPERATURE_1 瀵勫瓨鍣ㄣ€?temp1_max		鏈€澶ф俯搴︺€傛潵鑷?OT_WARN_LIMIT 瀵勫瓨鍣ㄣ€?temp1_crit		涓寸晫楂樻俯銆傛潵鑷?OT_FAULT_LIMIT 瀵勫瓨鍣ㄣ€?
temp2_input		娴嬪緱鐨勬俯搴︺€傛潵鑷?READ_TEMPERATURE_2 瀵勫瓨鍣ㄣ€?======================  ========================================================
