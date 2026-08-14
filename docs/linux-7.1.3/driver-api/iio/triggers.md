## 瑙﹀彂鍣紙Triggers锛?

- struct iio_trigger 鈥?宸ヤ笟 I/O 瑙﹀彂鍣ㄨ澶?- `devm_iio_trigger_alloc` 鈥?璧勬簮鎵樼鐨?iio_trigger_alloc
- `devm_iio_trigger_register` 鈥?璧勬簮鎵樼鐨?iio_trigger_register
  iio_trigger_unregister
- `iio_trigger_validate_own_device` 鈥?妫€鏌ヨЕ鍙戝櫒鍜?IIO
  璁惧鏄惁灞炰簬鍚屼竴涓澶?
鍦ㄨ澶氭儏鍐典笅锛岄┍鍔ㄨ兘澶熷熀浜庢煇浜涘閮ㄤ簨浠讹紙瑙﹀彂鍣級鏉ユ崟鑾锋暟鎹紝鑰屼笉鏄懆鏈熸€у湴杞
鏁版嵁锛岃繖闈炲父鏈夌敤銆侷IO 瑙﹀彂鍣ㄥ彲浠ョ敱涓€涓悓鏃舵嫢鏈夊熀浜庣‖浠剁敓鎴愪簨浠讹紙渚嬪鏁版嵁灏辩华鎴?瓒呰繃闃堝€硷級鐨?IIO 璁惧鐨勮澶囬┍鍔ㄦ彁渚涳紝涔熷彲浠ョ敱涓€涓潵鑷嫭绔嬩腑鏂簮锛堜緥濡傝繛鎺ュ埌鏌愪釜
澶栭儴绯荤粺鐨?GPIO 绾胯矾銆佸畾鏃跺櫒涓柇锛屾垨鐢ㄦ埛绌洪棿鍐欏叆 sysfs 涓殑鏌愪釜鐗瑰畾鏂囦欢锛夌殑鍗曠嫭
椹卞姩鎻愪緵銆備竴涓Е鍙戝櫒鍙互涓哄涓紶鎰熷櫒鍙戣捣鏁版嵁鎹曡幏锛屽苟涓斿畠涔熷彲鑳戒笌浼犳劅鍣ㄦ湰韬畬鍏?鏃犲叧銆?
## IIO 瑙﹀彂鍣ㄧ殑 sysfs 鎺ュ彛


sysfs 涓笌瑙﹀彂鍣ㄧ浉鍏崇殑浣嶇疆鏈変袱澶勶細

- `/sys/bus/iio/devices/trigger{Y}/*`锛岃鏂囦欢鍦?IIO 瑙﹀彂鍣ㄦ敞鍐屽埌 IIO 鏍稿績鏃跺垱寤猴紝
  瀵瑰簲浜庣储寮曚负 Y 鐨勮Е鍙戝櫒銆傜敱浜庤Е鍙戝櫒鏍规嵁绫诲瀷鍙兘澶т笉鐩稿悓锛岃繖閲屽彧鏈夊皯鏁版爣鍑嗗睘鎬?  鍙互鎻忚堪锛?
  - `name`锛岃Е鍙戝櫒鐨勫悕绉帮紝涔嬪悗鍙敤浜庝笌璁惧鍏宠仈銆?  - `sampling_frequency`锛屾煇浜涘熀浜庡畾鏃跺櫒鐨勮Е鍙戝櫒浣跨敤姝ゅ睘鎬ф潵鎸囧畾瑙﹀彂璋冪敤鐨勯鐜囥€?
- `/sys/bus/iio/devices/iio:device{X}/trigger/*`锛岃鐩綍鍦ㄨ澶囨敮鎸佽Е鍙戠紦鍐插尯鏃?  鍒涘缓銆傛垜浠彲浠ラ€氳繃鍦?`current_trigger` 鏂囦欢涓啓鍏ヨЕ鍙戝櫒鐨勫悕绉版潵灏嗚Е鍙戝櫒涓庢垜浠殑
  璁惧鍏宠仈銆?
## IIO 瑙﹀彂鍣ㄨ缃?

```

      struct iio_trigger_ops trigger_ops = {
          .set_trigger_state = sample_trigger_state,
          .validate_device = sample_validate_device,
      }

      struct iio_trigger *trig;

      /* 棣栧厛锛屼负鎴戜滑鐨勮Е鍙戝櫒鍒嗛厤鍐呭瓨 */
      trig = iio_trigger_alloc(dev, "trig-%s-%d", name, idx);

      /* 璁剧疆瑙﹀彂鍣ㄦ搷浣滃瓧娈?*/
      trig->ops = &trigger_ops;

      /* 鐜板湪灏嗚Е鍙戝櫒娉ㄥ唽鍒?IIO 鏍稿績 */
      iio_trigger_register(trig);

```
## IIO 瑙﹀彂鍣?ops


- struct iio_trigger_ops 鈥?iio_trigger 鐨勬搷浣滅粨鏋勪綋銆?
娉ㄦ剰瑙﹀彂鍣ㄩ檮甯︿簡涓€缁勬搷浣滐細

- `set_trigger_state`锛屾寜闇€鎵撳紑/鍏抽棴瑙﹀彂鍣ㄣ€?- `validate_device`锛屽綋褰撳墠瑙﹀彂鍣ㄨ鏇存敼鏃剁敤浜庨獙璇佽澶囩殑鍑芥暟銆?
## 鏇村缁嗚妭


   :export:
