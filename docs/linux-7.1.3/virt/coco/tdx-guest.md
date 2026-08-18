
## TDX Guest API 鏂囨。


## 1. 姒傝堪


TDX guest 椹卞姩閫氳繃 /dev/tdx-guest 杩欎釜 misc 璁惧鏆撮湶 IOCTL 鎺ュ彛锛屼互鍏佽鐢ㄦ埛绌洪棿鑾峰彇鏌愪簺 TDX guest 鐗规湁鐨勭粏鑺傘€?
## 2. API 璇存槑


鏈妭閽堝姣忎釜鍙楁敮鎸佺殑 IOCTL锛屾彁渚涗互涓嬩俊鎭互鍙婇€氱敤璇存槑銆?
:Input parameters: 浼犵粰 IOCTL 鐨勫弬鏁板強鐩稿叧缁嗚妭銆?:Output: 鍏充簬杈撳嚭鏁版嵁鍜岃繑鍥炲€肩殑缁嗚妭锛堝惈闈炲父瑙侀敊璇€肩殑璇存槑锛夈€?
### 2.1 TDX_CMD_GET_REPORT0


:Input parameters: struct tdx_report_req
:Output: 鎴愬姛鎵ц鍚庯紝TDREPORT 鏁版嵁琚鍒跺埌 tdx_report_req.tdreport 骞惰繑鍥?0銆傚浜庢棤鏁堟搷浣滄暟杩斿洖 -EINVAL锛孴DCALL 澶辫触鏃惰繑鍥?-EIO锛屽叾浠栧父瑙佸け璐ユ椂杩斿洖鏍囧噯閿欒鍙枫€?
TDX_CMD_GET_REPORT0 IOCTL 鍙璇佹槑锛坅ttestation锛夎蒋浠剁敤鏉ラ€氳繃 TDCALL[TDG.MR.REPORT] 浠?TDX module 鑾峰彇 TDREPORT0锛堝嵆 TDREPORT subtype 0锛夈€?
璇?IOCTL CMD 鐨勬湯灏炬坊鍔犱簡涓€涓?subtype 绱㈠紩锛岀敤浠ュ敮涓€鏍囪瘑鐗瑰畾 subtype 鐨?TDREPORT 璇锋眰銆傚敖绠?subtype 閫夐」鍦?TDX Module v1.0 瑙勮寖涓爣棰樹负鈥淭DG.MR.REPORT鈥濈殑灏忚妭閲岃鎻愬強锛屼絾鐩墠骞舵湭浣跨敤锛屼笖瑕佹眰璇ュ€间负 0銆備负浜嗕娇 IOCTL 瀹炵幇淇濇寔绠€鍗曪紝subtype 閫夐」娌℃湁琚撼鍏ヨ緭鍏?ABI銆備笉杩囨湭鏉ヨ嫢 TDX Module 鏀寔澶氫釜 subtype锛屽皢浼氬垱寤轰竴涓柊鐨?IOCTL CMD 鏉ュ鐞嗐€備负浜嗕繚鎸?IOCTL 鍛藉悕涓€鑷达紝subtype 绱㈠紩浣滀负 IOCTL CMD 鐨勪竴閮ㄥ垎琚姞鍏ャ€?
### 鍙傝€?

TDX 鍙傝€冭祫鏂欐眹鎬讳簬姝わ細

https://www.intel.com/content/www/us/en/developer/articles/technical/intel-trust-domain-extensions.html

璇ラ┍鍔ㄥ熀浜?TDX module 瑙勮寖 v1.0 涓?TDX GHCI 瑙勮寖 v1.0銆?