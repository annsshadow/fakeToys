
## Intel(R) 涓诲姩绠＄悊鎶€鏈紙Intel AMT锛?

Intel ME 鎺ュ彛涓€涓獊鍑虹殑鐢ㄩ€旀槸涓庤繍琛屽湪 Intel ME 涓婄殑鍥轰欢涓疄鐜扮殑 Intel(R) 涓诲姩绠＄悊鎶€鏈紙Intel AMT锛夎繘琛岄€氫俊銆?
Intel AMT 鎻愪緵浜嗚繙绋嬪甫澶栵紙OOB锛夌鐞嗕富鏈虹殑鑳藉姏锛屽嵆浣胯繍琛屽湪涓绘満澶勭悊鍣ㄤ笂鐨勬搷浣滅郴缁熷凡缁忓穿婧冩垨澶勪簬鐫＄湢鐘舵€併€?
Intel AMT 鐨勪竴浜涗娇鐢ㄧず渚嬪涓嬶細
   - 鐩戞帶纭欢鐘舵€佷笌骞冲彴缁勪欢
   - 杩滅▼鏂數/涓婄數锛堝缁胯壊璁＄畻鎴栧闂?IT 缁存姢寰堟湁鐢級
   - 鎿嶄綔绯荤粺鏇存柊
   - 瀛樺偍鏈夌敤鐨勫钩鍙颁俊鎭紝渚嬪杞欢璧勪骇
   - 鍐呯疆纭欢 KVM
   - 鍩轰簬杩滅▼绠＄悊鎺у埗鍙版墍璁剧瓥鐣ワ紝瀵逛互澶綉涓?IP 鍗忚娴佽繘琛岄€夋嫨鎬х綉缁滈殧绂?   - 鏉ヨ嚜杩滅▼绠＄悊鎺у埗鍙扮殑 IDE 璁惧閲嶅畾鍚?
Intel AMT锛圤OB锛夐€氫俊鍩轰簬 SOAP锛堣嚜 6.0 鐗堟湰璧峰凡寮冪敤锛塷ver HTTP/S锛屾垨鍩轰簬 WS-Management 鍗忚 over HTTP/S锛岃繖浜涜姹傛潵鑷繙绋嬬鐞嗘帶鍒跺彴搴旂敤绋嬪簭銆?
鍏充簬 Intel AMT 鐨勬洿澶氫俊鎭細
https://software.intel.com/sites/manageability/AMT_Implementation_and_Reference_Guide/default.htm


### Intel AMT 搴旂敤绋嬪簭


    1) Intel 鏈湴绠＄悊鏈嶅姟锛圛ntel LMS锛?
       鍦ㄥ钩鍙颁笂鏈湴杩愯鐨勫簲鐢ㄧ▼搴忎笌 Intel AMT 2.0 鍙婃洿楂樼増鏈€氫俊鐨勬柟寮忥紝鍚岀綉缁滃簲鐢ㄧ▼搴忛€氳繃 SOAP over HTTP锛堣嚜 6.0 鐗堟湰璧峰凡寮冪敤锛夋垨 WS-Management over SOAP over HTTP 閫氫俊鐨勬柟寮忎竴鑷淬€傝繖鎰忓懗鐫€鏌愪簺 Intel AMT 鐗规€у彲浠ヤ粠鏈湴搴旂敤绋嬪簭璁块棶锛屼娇鐢ㄤ笌閫氳繃缃戠粶涓?Intel AMT 閫氫俊鐨勮繙绋嬪簲鐢ㄧ▼搴忕浉鍚岀殑缃戠粶鎺ュ彛銆?
       褰撴湰鍦板簲鐢ㄧ▼搴忓彂閫佷竴鏉″彂寰€鏈湴 Intel AMT 涓绘満鍚嶇殑娑堟伅鏃讹紝鐩戝惉鍙戝線璇ヤ富鏈哄悕娴侀噺鐨?Intel LMS 浼氭嫤鎴娑堟伅骞跺皢鍏惰矾鐢卞埌 Intel MEI銆?       鏇村淇℃伅锛?       https://software.intel.com/sites/manageability/AMT_Implementation_and_Reference_Guide/default.htm
       鍦?"About Intel AMT" => "Local Access" 涓?
       涓嬭浇 Intel LMS锛?       https://github.com/intel/lms

       Intel LMS 浣跨敤 Intel MEI 椹卞姩锛岄€氳繃瀹氫箟鐨?GUID 鎵撳紑鍒?Intel LMS 鍥轰欢鐗规€х殑杩炴帴锛岀劧鍚庝娇鐢ㄨ鐗规€ц繘琛岄€氫俊锛岄€氫俊閲囩敤涓€绉嶇О涓?Intel AMT 绔彛杞彂鍗忚锛圛ntel APF 鍗忚锛夌殑鍗忚銆傝鍗忚鐢ㄤ簬浠庡崟涓€搴旂敤绋嬪簭缁存姢涓?Intel AMT 鐨勫涓細璇濄€?
       鍗忚瑙勮寖鍙傝 Intel AMT 杞欢寮€鍙戝伐鍏峰寘锛圫DK锛?       https://software.intel.com/sites/manageability/AMT_Implementation_and_Reference_Guide/default.htm
       鍦?"SDK Resources" => "Intel(R) vPro(TM) Gateway (MPS)"
       => "Information for Intel(R) vPro(TM) Gateway Developers"
       => "Description of the Intel AMT Port Forwarding (APF) Protocol" 涓?
    2) 浣跨敤鏈湴浠ｇ悊杩涜 Intel AMT 杩滅▼閰嶇疆

       鏈湴浠ｇ悊浣?IT 浜哄憳鑳藉寮€绠卞嵆鐢ㄥ湴閰嶇疆 Intel AMT锛岃€屾棤闇€瀹夎棰濆鐨勬暟鎹潵鍚敤璁剧疆銆傝繙绋嬮厤缃繃绋嬪彲鑳芥秹鍙婁竴涓繍琛屽湪涓绘満涓婄殑銆佺敱 ISV 寮€鍙戠殑杩滅▼閰嶇疆浠ｇ悊銆?       鏇村淇℃伅锛?       https://software.intel.com/sites/manageability/AMT_Implementation_and_Reference_Guide/default.htm
       鍦?"Setup and Configuration of Intel AMT" =>
       "SDK Tools Supporting Setup and Configuration" =>
       "Using the Local Agent Sample" 涓?
### Intel AMT 鎿嶄綔绯荤粺鍋ュ悍鐪嬮棬鐙?

Intel AMT 鐪嬮棬鐙楁槸涓€涓搷浣滅郴缁熷仴搴凤紙鎸傝捣/宕╂簝锛夌湅闂ㄧ嫍銆?姣忓綋鎿嶄綔绯荤粺鎸傝捣鎴栧穿婧冩椂锛孖ntel AMT 浼氬悜璇ヤ簨浠剁殑浠讳綍璁㈤槄鑰呭彂閫佷竴涓簨浠躲€傝繖涓€鏈哄埗鎰忓懗鐫€锛屽嵆浣夸富鏈哄彂鐢熺‖鎬ф晠闅滐紝IT 涔熻兘鐭ラ亾骞冲彴浣曟椂宕╂簝銆?
Intel AMT 鐪嬮棬鐙楃敱涓ら儴鍒嗙粍鎴愶細
    1) 鍥轰欢鐗规€?鈥斺€?鎺ユ敹蹇冭烦锛屽苟鍦ㄥ績璺冲仠姝㈡椂鍙戦€佷簨浠躲€?    2) Intel MEI iAMT 鐪嬮棬鐙楅┍鍔?鈥斺€?杩炴帴鍒扮湅闂ㄧ嫍鐗规€э紝閰嶇疆鐪嬮棬鐙楀苟鍙戦€佸績璺炽€?
Intel iAMT 鐪嬮棬鐙?MEI 椹卞姩浣跨敤鍐呮牳鐪嬮棬鐙?API 鏉ラ厤缃?Intel AMT 鐪嬮棬鐙楀苟鍚戝叾鍙戦€佸績璺炽€傜湅闂ㄧ嫍鐨勯粯璁よ秴鏃舵椂闂翠负 120 绉掋€?
濡傛灉鍥轰欢涓湭鍚敤 Intel AMT锛屽垯鐪嬮棬鐙楀鎴风涓嶄細鍦?me 瀹㈡埛绔€荤嚎涓婃灇涓撅紝鐪嬮棬鐙楄澶囦篃涓嶄細琚毚闇层€?
---
linux-mei@linux.intel.com
