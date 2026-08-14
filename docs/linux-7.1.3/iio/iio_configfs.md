## 宸ヤ笟 IIO configfs 鏀寔


## 1. 姒傝堪


Configfs 鏄竴涓熀浜庢枃浠剁郴缁熺殑鍐呮牳瀵硅薄绠＄悊鍣ㄣ€侷IO 浣跨敤涓€浜涘彲浠ユ柟渚垮湴閫氳繃 configfs 閰嶇疆鐨勫璞★紙渚嬪锛氳澶囥€佽Е鍙戝櫒锛夈€?
鏈夊叧 configfs 鐨勫伐浣滄柟寮忥紝璇峰弬闃?Documentation/filesystems/configfs.rst銆?
## 2. 鐢ㄦ硶


涓轰簡鍦?IIO 涓娇鐢?configfs 鏀寔锛屾垜浠渶瑕佸湪缂栬瘧鏃堕€氳繃 CONFIG_IIO_CONFIGFS 閰嶇疆閫夐」灏嗗叾閫変腑銆?
```

  $ mkdir /config
  $ mount -t configfs none /config

```
姝ゆ椂锛屾墍鏈夐粯璁ょ殑 IIO 缁勯兘灏嗚鍒涘缓锛屽苟鍙湪 /config/iio 涓嬭闂€傚悗缁珷鑺傚皢鎻忚堪鍙敤鐨?IIO 閰嶇疆瀵硅薄銆?
## 3. 杞欢瑙﹀彂鍣?

IIO 榛樿 configfs 缁勪箣涓€鏄?鈥渢riggers锛堣Е鍙戝櫒锛夆€濈粍銆傚畠鍦?configfs 鎸傝浇鍚庤嚜鍔ㄥ彲璁块棶锛屽彲鍦?/config/iio/triggers 涓嬫壘鍒般€?
IIO 杞欢瑙﹀彂鍣ㄧ殑瀹炵幇鏀寔鍒涘缓澶氱瑙﹀彂鍣ㄧ被鍨嬨€備竴涓柊鐨勮Е鍙戝櫒绫诲瀷閫氬父浣滀负涓€涓嫭绔嬬殑
```

  /*
   * drivers/iio/trigger/iio-trig-sample.c
   * sample kernel module implementing a new trigger type
   */
  #include <linux/iio/sw_trigger.h>


  static struct iio_sw_trigger *iio_trig_sample_probe(const char *name)
  {
	/*
	 * This allocates and registers an IIO trigger plus other
	 * trigger type specific initialization.
	 */
  }

  static int iio_trig_sample_remove(struct iio_sw_trigger *swt)
  {
	/*
	 * This undoes the actions in iio_trig_sample_probe
	 */
  }

  static const struct iio_sw_trigger_ops iio_trig_sample_ops = {
	.probe		= iio_trig_sample_probe,
	.remove		= iio_trig_sample_remove,
  };

  static struct iio_sw_trigger_type iio_trig_sample = {
	.name = "trig-sample",
	.owner = THIS_MODULE,
	.ops = &iio_trig_sample_ops,
  };

  module_iio_sw_trigger_driver(iio_trig_sample);

```
姣忕瑙﹀彂鍣ㄧ被鍨嬪湪 /config/iio/triggers 涓嬮兘鏈夎嚜宸辩殑鐩綍銆傚姞杞?iio-trig-sample 妯″潡灏嗗垱寤?'trig-sample' 瑙﹀彂鍣ㄧ被鍨嬬洰褰?/config/iio/triggers/trig-sample銆?
鎴戜滑鏀寔浠ヤ笅涓柇婧愶紙瑙﹀彂鍣ㄧ被鍨嬶級锛?
 - hrtimer锛屼娇鐢ㄩ珮鍒嗚鲸鐜囧畾鏃跺櫒浣滀负涓柇婧?
### 3.1 hrtimer 瑙﹀彂鍣ㄧ殑鍒涘缓涓庨攢姣?

鍔犺浇 iio-trig-hrtimer 妯″潡灏嗘敞鍐?hrtimer 瑙﹀彂鍣ㄧ被鍨嬶紝鍏佽鐢ㄦ埛鍦?/config/iio/triggers/hrtimer 涓嬪垱寤?hrtimer 瑙﹀彂鍣ㄣ€?
```

  $ mkdir /config/iio/triggers/hrtimer/instance1
  $ rmdir /config/iio/triggers/hrtimer/instance1

```
姣忎釜瑙﹀彂鍣ㄥ彲浠ユ嫢鏈変竴涓垨澶氫釜鐗瑰畾浜庤瑙﹀彂鍣ㄧ被鍨嬬殑灞炴€с€?
### 3.2 "hrtimer" 瑙﹀彂鍣ㄧ被鍨嬬殑灞炴€?

"hrtimer" 瑙﹀彂鍣ㄧ被鍨嬪湪 /config 鐩綍涓嬫病鏈変换浣曞彲閰嶇疆灞炴€с€傚畠浼氬悜瑙﹀彂鍣ㄧ洰褰曞紩鍏?sampling_frequency 灞炴€с€傝灞炴€т互 Hz 涓哄崟浣嶈缃疆璇㈤鐜囷紝绮惧害涓?mHz銆?