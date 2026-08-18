
## 鍩轰簬 FF-A 鐨?TPM CRB 椹卞姩


TPM 鍛戒护鍝嶅簲缂撳啿鍖猴紙CRB锛夋帴鍙ｆ槸 TCG PC Client Platform TPM Profile (PTP)
瑙勮寖 [^1^]_ 涓畾涔夌殑涓€涓爣鍑?TPM 鎺ュ彛銆侰RB 鎻愪緵浜嗕竴缁勭粨鏋勫寲鐨勬帶鍒跺瘎瀛樺櫒锛?瀹㈡埛绔湪涓?TPM 浜や簰鏃朵細鐢ㄥ埌瀹冧滑锛屽悓鏃惰繕鎻愪緵浜嗕竴涓敤浜庡瓨鍌?TPM 鍛戒护涓庡搷搴旂殑
鏁版嵁缂撳啿鍖恒€侰RB 鎺ュ彛鍙互鍦ㄤ互涓嬩綅缃疄鐜帮細

- 鐙珛 TPM 鑺墖涓殑纭欢瀵勫瓨鍣?
- 鍦ㄥ唴瀛樹腑锛岀敤浜庤繍琛屽湪闅旂鐜涓殑 TPM锛屽叾涓叡浜唴瀛樺厑璁稿鎴风涓?TPM 浜や簰

Arm A 绯诲垪鍥轰欢妗嗘灦锛團F-A锛塠^2^]_ 鏄竴浠借鑼冿紝瀹氫箟浜嗙敤浜庝互涓嬬洰鐨勭殑鎺ュ彛涓?鍗忚锛?
- 灏嗗浐浠跺垝鍒嗗埌杩愯鍦?Arm Secure 涓栫晫鐜锛堜篃绉颁负 TrustZone锛変腑鐨勮蒋浠跺垎鍖轰腑

- 涓哄浜庨潪瀹夊叏锛圢on-secure锛夌姸鎬佺殑杞欢缁勪欢锛堜緥濡傛搷浣滅郴缁熶笌 Hypervisor锛夋彁渚?  鏍囧噯鎺ュ彛锛屼互渚夸笌杩欎簺鍥轰欢閫氫俊

TPM 鍙互浣滀负 FF-A 瀹夊叏鏈嶅姟鏉ュ疄鐜般€傚畠鍙互鏄浐浠?TPM锛屼篃鍙兘鏄厖褰撶嫭绔?TPM
鑺墖浠ｇ悊鐨?TPM 鏈嶅姟銆傚熀浜?FF-A 鐨?TPM 灏嗙‖浠剁粏鑺傦紙渚嬪鎬荤嚎鎺у埗鍣ㄤ笌鐗囬€夛級浠?鎿嶄綔绯荤粺涓娊璞″嚭鏉ワ紝骞朵笖鍙互淇濇姢 locality 4 涓嶈鎿嶄綔绯荤粺璁块棶銆傚鎴风浣跨敤
TCG 瀹氫箟鐨?CRB 鎺ュ彛涓?TPM 鏈嶅姟浜や簰銆?
Arm TPM Service Command Response Buffer Interface Over FF-A [^3^]_ 瑙勮寖瀹氫箟浜?瀹㈡埛绔彲浠ョ敤鏉ュ湪 CRB 鍙戠敓鏇存柊鏃跺彂鍑轰俊鍙风殑 FF-A 娑堟伅銆?
Linux 鐨?CRB 椹卞姩涓?FF-A 鐨勪氦浜掓柟寮忔瑕佸涓嬶細

- tpm_crb_ffa 椹卞姩浠?CRB over FF-A 瑙勮寖涓畾涔夌殑鏋舵瀯鍖?TPM 鏈嶅姟 UUID 鍚戝唴鏍哥殑
  FF-A 瀛愮郴缁熸敞鍐屻€?
- 濡傛灉 FF-A 鍙戠幇浜嗘煇涓?TPM 鏈嶅姟锛屽垯 tpm_crb_ffa 椹卞姩涓殑 probe() 鍑芥暟浼氳繍琛岋紝
  椹卞姩瀹屾垚鍒濆鍖栥€?
- Linux CRB 椹卞姩鐨勬帰娴嬩笌鍒濆鍖栨槸鐢卞彂鐜伴€氳繃 ACPI 閫氬憡鐨?TPM 瑙﹀彂鐨勩€侰RB 椹卞姩
  鍙互閫氳繃 ACPI 鐨?'start' 鏂规硶妫€娴?TPM 鐨勭被鍨嬨€侫rm FF-A 鐨?start 鏂规硶瀹氫箟浜?  TCG ACPI v1.4 [^4^]_ 涓€?
- 褰?CRB 椹卞姩鎵ц鍏跺父瑙勫姛鑳斤紙渚嬪鍙戝嚭 'start' 淇″彿浠ュ強 locality 鐨勮姹?閲婃斁锛?  鏃讹紝瀹冧細璋冪敤 tpm_crb_ffa 椹卞姩涓殑 tpm_crb_ffa_start() 鍑芥暟锛岃鍑芥暟璐熻矗澶勭悊
  鍙戝線 TPM 鐨?FF-A 娑堟伅銆?
## 鍙傝€冭祫鏂?

   https://trustedcomputinggroup.org/resource/pc-client-platform-tpm-profile-ptp-specification/
   https://developer.arm.com/documentation/den0077/latest/
   https://developer.arm.com/documentation/den0138/latest/
   https://trustedcomputinggroup.org/resource/tcg-acpi-specification/
