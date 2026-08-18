AI 缂栫▼鍔╂墜
++++++++++++++++++++

鏈枃妗ｄ负鍦ㄥ弬涓?Linux 鍐呮牳寮€鍙戞椂浣跨敤 AI 杈呭姪鐨?AI 宸ュ叿涓庡紑鍙戣€呮彁渚涙寚瀵笺€?
鍗忓姪 Linux 鍐呮牳寮€鍙戠殑 AI 宸ュ叿搴旈伒寰爣鍑嗙殑鍐呮牳寮€鍙戞祦绋嬶細

- Documentation/process/development-process.rst
- Documentation/process/coding-style.rst
- Documentation/process/submitting-patches.rst

## 璁稿彲涓庢硶寰嬭姹?

鎵€鏈夎础鐚繀椤荤鍚堝唴鏍哥殑璁稿彲瑕佹眰锛?
- 鎵€鏈変唬鐮佸繀椤讳笌 GPL-2.0-only 鍏煎
- 浣跨敤閫傚綋鐨?SPDX 璁稿彲鏍囪瘑绗?- 璇﹁ Documentation/process/license-rules.rst

## Signed-off-by 涓庡紑鍙戣€呮潵婧愯瘉涔?

AI 浠ｇ悊涓嶅緱娣诲姞 Signed-off-by 鏍囩銆傚彧鏈変汉绫绘墠鑳藉悎娉曞湴璁よ瘉寮€鍙戣€呮潵婧愯瘉涔?锛圖CO锛夈€備汉绫绘彁浜よ€呰礋璐ｏ細

- 瀹℃煡鎵€鏈?AI 鐢熸垚鐨勪唬鐮?- 纭繚绗﹀悎璁稿彲瑕佹眰
- 娣诲姞鍏惰嚜宸辩殑 Signed-off-by 鏍囩浠ヨ璇?DCO
- 瀵硅础鐚壙鎷呭叏閮ㄨ矗浠?
## 缃插悕


褰?AI 宸ュ叿鍙備笌鍐呮牳寮€鍙戞椂锛岄€傚綋鐨勭讲鍚嶆湁鍔╀簬杩借釜 AI 鍦ㄥ紑鍙戣繃绋嬩腑涓嶆柇婕斿彉鐨?瑙掕壊銆?
```
  Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]

```

鍏朵腑锛?
- `AGENT_NAME` 鏄?AI 宸ュ叿鎴栨鏋剁殑鍚嶇О
- `MODEL_VERSION` 鏄墍浣跨敤鐨勭壒瀹氭ā鍨嬬増鏈?- `[TOOL1] [TOOL2]` 鏄彲閫夌殑涓撲笟鍒嗘瀽宸ュ叿锛堜緥濡?coccinelle銆乻parse銆乻match銆乧lang-tidy锛?
鍩烘湰鐨勫紑鍙戝伐鍏凤紙git銆乬cc銆乵ake銆佺紪杈戝櫒锛変笉搴斿垪鍑恒€?
```

  Assisted-by: Claude:claude-3-opus coccinelle sparse

```
