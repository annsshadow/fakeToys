## 鍥轰欢鎼滅储璺緞


鍦ㄦ偍鐨勬牴鏂囦欢绯荤粺涓婃煡鎵惧浐浠舵椂浣跨敤浠ヤ笅鎼滅储璺緞銆?
- fw_path_para - 妯″潡鍙傛暟 - 榛樿涓虹┖锛屽洜姝よ蹇界暐
- /lib/firmware/updates/UTS_RELEASE/
- /lib/firmware/updates/
- /lib/firmware/UTS_RELEASE/
- /lib/firmware/

妯″潡鍙傛暟 ''path'' 鍙互浼犻€掔粰 firmware_class 妯″潡锛屼互婵€娲荤涓€涓彲閫夌殑
鑷畾涔?fw_path_para銆傝嚜瀹氫箟璺緞鐨勯暱搴︽渶澶氫负 256 涓瓧绗︺€備紶鍏ョ殑鍐呮牳
鍙傛暟搴斾负锛?
- 'firmware_class.path=$CUSTOMIZED_PATH'

杩樻湁涓€绉嶆浛浠ｆ柟娉曞彲浠ュ湪鍚姩鍚庤繍琛屾椂鑷畾涔夎矾寰勶紝鎮ㄥ彲浠ヤ娇鐢ㄤ互涓嬫枃浠讹細

- /sys/module/firmware_class/parameters/path

鎮ㄥ彲浠ュ皢鑷畾涔夎矾寰?echo 鍐欏叆鍏朵腑锛屾墍璇锋眰鐨勫浐浠跺皢棣栧厛鍦ㄨ璺緞涓嬫悳绱€?璇锋敞鎰忥紝鎹㈣绗︿細琚€冭檻鍦ㄥ唴锛屽苟涓斿彲鑳戒笉浼氫骇鐢熼鏈熺殑鏁堟灉銆備緥濡傦紝鎮ㄥ彲鑳?甯屾湜浣跨敤锛?
echo -n /path/to/script > /sys/module/firmware_class/parameters/path

浠ョ‘淇濅娇鐢ㄦ偍鐨勮剼鏈€?