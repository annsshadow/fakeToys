
## TPM 浜嬩欢鏃ュ織锛圗vent Log锛?

鏈枃妗ｇ畝瑕佷粙缁嶄粈涔堟槸 TPM 鏃ュ織锛屼互鍙婂畠鏄浣曚粠鍓嶅紩瀵煎浐浠讹紙preboot firmware锛夌Щ浜ょ粰鎿嶄綔绯荤粺鐨勩€?
## 绠€浠?

鍓嶅紩瀵煎浐浠剁淮鎶や竴涓簨浠舵棩蹇楋紝姣忓綋鏈夊唴瀹硅瀹冨搱甯屽埌浠讳竴 PCR 瀵勫瓨鍣ㄦ椂锛岄兘浼氬悜鍏朵腑娣诲姞鏂版潯鐩€備簨浠舵寜鍏剁被鍨嬪垎缁勶紝骞跺寘鍚鍝堝笇鐨?PCR 瀵勫瓨鍣ㄧ殑鍊笺€傞€氬父锛屽墠寮曞鍥轰欢浼氬灏嗚绉讳氦鎵ц鐨勭粍浠舵垨涓庡惎鍔ㄨ繃绋嬬浉鍏崇殑鎿嶄綔杩涜鍝堝笇銆?
姝ゆ満鍒剁殑涓昏搴旂敤鏄繙绋嬭瘉鏄庯紙remote attestation锛夛紝鑰屽畠涔嬫墍浠ユ湁鐢ㄧ殑鍘熷洜鍦?[^1^] 鐨勭涓€鑺備腑鏈夌簿杈熺殑鎬荤粨锛?
"Attestation is used to provide information about the platform鈥檚 state to a challenger. However, PCR contents are difficult to interpret; therefore, attestation is typically more useful when the PCR contents are accompanied by a measurement log. While not trusted on their own, the measurement log contains a richer set of information than do the PCR contents. The PCR contents are used to provide the validation of the measurement log."

锛堣瘉鏄庣敤浜庡悜鎸戞垬鑰呮彁渚涙湁鍏冲钩鍙扮姸鎬佺殑淇℃伅銆傜劧鑰岋紝PCR 鍐呭闅句互瑙ｈ锛涘洜姝わ紝褰?PCR 鍐呭浼撮殢娴嬮噺鏃ュ織鏃讹紝璇佹槑閫氬父鏇存湁鐢ㄣ€傛祴閲忔棩蹇楁湰韬櫧涓嶅彲淇★紝浣嗗叾鍖呭惈鐨勪俊鎭瘮 PCR 鍐呭鏇翠赴瀵屻€侾CR 鍐呭鐢ㄤ簬鎻愪緵瀵规祴閲忔棩蹇楃殑楠岃瘉銆傦級

## UEFI 浜嬩欢鏃ュ織


UEFI 鎻愪緵鐨勪簨浠舵棩蹇楁湁涓€浜涙湁鐐瑰鎬殑鎬櫀銆?
鍦ㄨ皟鐢?ExitBootServices() 涔嬪墠锛孡inux EFI stub 灏嗕簨浠舵棩蹇楀鍒跺埌鐢?stub 鑷韩瀹氫箟鐨勮嚜瀹氫箟閰嶇疆琛紙configuration table锛変腑銆傞仐鎲剧殑鏄紝鐢?ExitBootServices() 鐢熸垚鐨勪簨浠舵渶缁堝苟鏈繘鍏ヨ琛ㄣ€?
鍥轰欢鎻愪緵浜嗘墍璋撶殑 final events 閰嶇疆琛ㄦ潵瑙ｅ喅杩欎釜闂銆傚湪 EFI_TCG2_PROTOCOL.GetEventLog() 绗竴娆¤璋冪敤涔嬪悗锛屼簨浠朵細琚暅鍍忓埌璇ヨ〃涓€?
杩欏紩鍏ヤ簡鍙︿竴涓棶棰橈細娌℃湁浠讳綍淇濊瘉瀹冧笉浼氬湪 Linux EFI stub 杩愯涔嬪墠琚皟鐢ㄣ€傚洜姝わ紝stub 鍦ㄤ粛鐒惰繍琛屾椂闇€瑕佽绠楀苟淇濆瓨 final events 琛ㄧ殑澶у皬鍒拌嚜瀹氫箟閰嶇疆琛ㄤ腑锛屼互渚?TPM 椹卞姩涔嬪悗鍦ㄦ嫾鎺ユ潵鑷嚜瀹氫箟閰嶇疆琛ㄤ笌 final events 琛ㄤ袱鍗婄殑浜嬩欢鏃ュ織鏃惰烦杩囪繖浜涗簨浠躲€?
## 鍙傝€?

- [^1^] https://trustedcomputinggroup.org/resource/pc-client-specific-platform-firmware-profile-specification/
- [^2^] 鏈€缁堢殑鎷兼帴鍦?drivers/char/tpm/eventlog/efi.c 涓畬鎴?