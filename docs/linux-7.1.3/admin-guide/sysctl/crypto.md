## /proc/sys/crypto/


杩欎簺鏂囦欢鏄惁鍑虹幇鍦?`/proc/sys/crypto/`锛屽彇鍐充簬鍐呮牳閰嶇疆锛?

## fips_enabled


鍙鏍囧織锛屾寚绀烘槸鍚﹀惎鐢ㄤ簡 FIPS 妯″紡銆?
- `0`锛氱鐢?FIPS 妯″紡锛堥粯璁わ級銆?- `1`锛氬惎鐢?FIPS 妯″紡銆?
璇ュ€煎湪鍚姩鏃堕€氳繃 `fips=1` 鍐呮牳鍛戒护琛屽弬鏁拌缃€傚惎鐢ㄥ悗锛屽姞瀵?API 灏嗛檺鍒舵煇浜?绠楁硶鐨勪娇鐢ㄥ苟杩涜鑷锛屼互纭繚绗﹀悎 FIPS锛堣仈閭︿俊鎭鐞嗘爣鍑嗭級瑕佹眰锛屼緥濡?FIPS 140-2 涓庤緝鏂扮殑 FIPS 140-3锛屽叿浣撳彇鍐充簬鍐呮牳閰嶇疆涓庢墍鐢ㄦā鍧椼€?
## fips_name


鍙鏂囦欢锛屽寘鍚綋鍓嶆墍鐢?FIPS 妯″潡鐨勫悕绉般€?璇ュ€奸€氬父閫氳繃 `CONFIG_CRYPTO_FIPS_NAME` 鍐呮牳閰嶇疆閫夐」閰嶇疆銆?
## fips_version


鍙鏂囦欢锛屽寘鍚?FIPS 妯″潡鐨勭増鏈瓧绗︿覆銆?濡傛灉璁剧疆浜?`CONFIG_CRYPTO_FIPS_CUSTOM_VERSION`锛屽垯浣跨敤 `CONFIG_CRYPTO_FIPS_VERSION`
鐨勫€笺€傚惁鍒欓粯璁や负鍐呮牳鍙戝竷鐗堟湰锛坄UTS_RELEASE`锛夈€?
Copyright (c) 2026, Shubham Chakraborty <chakrabortyshubham66@gmail.com>

鏈夊叧涓€鑸俊鎭笌娉曞緥澹版槑锛岃鍙傞槄
Documentation/admin-guide/sysctl/index.rst銆?