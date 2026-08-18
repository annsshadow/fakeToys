
浣跨敤 pstore 璋冭瘯鍐呮牳鍏虫満鎸傝捣
+++++++++++++++++++++++++++++++++++++++++++

## 姒傝堪


濡傛灉绯荤粺鍦ㄥ叧鏈烘椂鎸傝捣锛屽彲鑳介渶瑕佽幏鍙栧唴鏍告棩蹇?
浠ヨ皟璇曡闂銆?

鍦ㄦ湁鍙敤 UART 鐨勭郴缁熶腑锛屾渶濂藉皢鍐呮牳閰嶇疆涓轰娇鐢ㄨ
UART 浣滀负鍐呮牳鎺у埗鍙拌緭鍑恒€?

濡傛灉娌℃湁鍙敤鐨?UART锛宍pstore` 瀛愮郴缁熸彁渚涗簡涓€绉嶆満鍒讹紝鍙?
鍦ㄧ郴缁熷浣嶆椂鎸佷箙鍖栬繖浜涙暟鎹紝浠庤€屽湪涓嬫
鍚姩鏃惰幏鍙栥€?

## 鍐呮牳閰嶇疆


瑕佸惎鐢?`pstore` 骞朵繚瀛樺唴鏍哥幆褰㈢紦鍐插尯鏃ュ織锛岃璁剧疆
浠ヤ笅鍐呮牳閰嶇疆閫夐」锛?

- `CONFIG_PSTORE=y`
- `CONFIG_PSTORE_CONSOLE=y`

姝ゅ锛岄渶鍚敤涓€涓悗绔潵瀛樺偍鏁版嵁銆傛牴鎹綘鐨勫钩鍙帮紝
涓€浜涘彲閫夋柟妗堝寘鎷細

- `CONFIG_EFI_VARS_PSTORE=y`
- `CONFIG_PSTORE_RAM=y`
- `CONFIG_CHROMEOS_PSTORE=y`
- `CONFIG_PSTORE_BLK=y`

## 鍐呮牳鍛戒护琛屽弬鏁?


灏嗚繖浜涘弬鏁版坊鍔犲埌浣犵殑鍐呮牳鍛戒护琛岋細

- `printk.always_kmsg_dump=Y`
 - 寮哄埗鍐呮牳鍦ㄥ叧鏈烘湡闂村皢鏁翠釜娑堟伅缂撳啿鍖鸿浆鍌ㄥ埌 pstore
		shutdown
- `efi_pstore.pstore_disable=N`
 - 瀵逛簬鍩轰簬 EFI 鐨勭郴缁燂紝纭繚 EFI 鍚庣澶勪簬娲诲姩鐘舵€?

## 鐢ㄦ埛绌洪棿浜や簰涓庢棩蹇楄幏鍙?


鍦ㄦ寕璧峰悗鐨勪笅娆″惎鍔ㄦ椂锛宲store 鏃ュ織灏嗕綅浜?pstore
鏂囦欢绯荤粺锛坄/sys/fs/pstore`锛変腑锛屽苟鍙敱鐢ㄦ埛绌洪棿鑾峰彇銆?

鍦?systemd 绯荤粺涓紝`systemd-pstore` 鏈嶅姟灏嗗府鍔╁畬鎴愪互涓嬫搷浣滐細

#. 鍦?`/sys/fs/pstore` 涓畾浣?pstore 鏁版嵁
#. 灏嗗叾璇诲彇骞朵繚瀛樺埌 `/var/lib/systemd/pstore`
#. 涓轰笅涓€娆′簨浠舵竻闄?pstore 鏁版嵁
