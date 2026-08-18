## 鎺ㄦ祴杩斿洖鏍堟孩鍑猴紙Speculative Return Stack Overflow锛孲RSO锛?

杩欐槸閽堝鍦?AMD 澶勭悊鍣ㄤ笂鍙戠幇鐨勬帹娴嬭繑鍥炴爤婧㈠嚭锛圫RSO锛夋紡娲炵殑缂撹В鎺柦銆傚叾鏈哄埗濡備粖鏄紬鎵€鍛ㄧ煡鐨勫満鏅細姣掑寲锛坧oisoning锛塁PU 鍔熻兘鍗曞厓 鈥斺€?鍦ㄨ繖绉嶆儏鍐典笅鏄垎鏀洰鏍囩紦鍐插尯锛圔TB锛夊拰杩斿洖鍦板潃棰勬祴鍣紙RAP锛?鈥斺€?鐒跺悗璇遍獥鎻愬崌鐨勭壒鏉冨煙锛堝唴鏍革級娉勬紡鏁忔劅鏁版嵁銆?
AMD CPU 浣跨敤杩斿洖鍦板潃棰勬祴鍣紙鍙堢О杩斿洖鍦板潃鏍?杩斿洖鏍堢紦鍐插尯锛孯eturn Address Stack/Return Stack Buffer锛夋潵棰勬祴 RET 鎸囦护銆傚湪鏌愪簺鎯呭喌涓嬶紝涓€涓潪鏋舵瀯锛坣on-architectural锛夌殑 CALL 鎸囦护锛堝嵆琚娴嬩负 CALL 浣嗗疄闄呭苟闈?CALL 鐨勬寚浠わ級鍙互鍦?RAP 涓垱寤轰竴涓潯鐩紝璇ユ潯鐩彲鑳借鐢ㄦ潵棰勬祴鍚庣画 RET 鎸囦护鐨勭洰鏍囥€?
瀵艰嚧杩欎竴鐐圭殑鍏蜂綋鎯呭喌鍥犲井鏋舵瀯鑰屽紓锛屼絾浠や汉鎷呭咖鐨勬槸锛屾敾鍑昏€呭彲浠ラ敊璇湴璁粌锛坢is-train锛塁PU BTB 鏉ラ娴嬪唴鏍哥┖闂翠腑鐨勯潪鏋舵瀯 CALL 鎸囦护锛屽苟鍒╃敤瀹冩潵鎺у埗鍚庣画鍐呮牳 RET 鐨勬帹娴嬬洰鏍囷紝浠庤€屽彲鑳介€氳繃鎺ㄦ祴渚т俊閬擄紙speculative side-channel锛夊鑷翠俊鎭硠闇层€?
璇ラ棶棰樺湪 CVE-2023-20569 涓嬭璺熻釜銆?
### 鍙楀奖鍝嶇殑澶勭悊鍣?

AMD Zen锛岀 1-4 浠ｃ€傚嵆鎵€鏈?family 0x17 鍜?0x19銆傝緝鏃х殑澶勭悊鍣ㄥ皻鏈鐮旂┒銆?
### 绯荤粺淇℃伅涓庨€夐」


棣栧厛锛岃浣跨紦瑙ｆ帾鏂芥湁鏁堬紝蹇呴』鍔犺浇鏈€鏂扮殑寰爜锛坢icrocode锛夈€?
鏄剧ず SRSO 缂撹В鐘舵€佺殑 sysfs 鏂囦欢鏄細

  /sys/devices/system/cpu/vulnerabilities/spec_rstack_overflow

姝ゆ枃浠朵腑鍙兘鐨勫€间负锛?
 - 'Not affected'锛堜笉鍙楀奖鍝嶏級锛?
   澶勭悊鍣ㄤ笉鏄撳彈鏀诲嚮銆?
- 'Vulnerable'锛堟槗鍙楁敾鍑伙級锛?
   澶勭悊鍣ㄦ槗鍙楁敾鍑讳笖鏈簲鐢ㄤ换浣曠紦瑙ｆ帾鏂姐€?
 - 'Vulnerable: No microcode'锛堟槗鍙楁敾鍑伙細鏃犲井鐮侊級锛?
   澶勭悊鍣ㄦ槗鍙楁敾鍑伙紝鏈簲鐢ㄦ墿灞?IBPB 鍔熻兘浠ヨВ鍐宠婕忔礊鐨勫井鐮併€?
 - 'Vulnerable: Safe RET, no microcode'锛堟槗鍙楁敾鍑伙細Safe RET锛屾棤寰爜锛夛細

   宸插簲鐢?鈥淪afe RET鈥?缂撹В鎺柦锛堣涓嬫枃锛変互淇濇姢鍐呮牳锛屼絾鏈簲鐢ㄦ墿灞?IBPB 鐨勫井鐮併€傜敤鎴风┖闂翠换鍔″彲鑳戒粛鐒舵槗鍙楁敾鍑汇€?
 - 'Vulnerable: Microcode, no safe RET'锛堟槗鍙楁敾鍑伙細寰爜锛屾棤 Safe RET锛夛細

   宸插簲鐢ㄦ墿灞?IBPB 鍔熻兘寰爜琛ヤ竵銆傚畠涓嶈В鍐?User->Kernel 鍜?Guest->Host 杞崲淇濇姢锛屼絾瀹冭В鍐充簡 User->User 鍜?VM->VM 鏀诲嚮鍚戦噺銆?
   娉ㄦ剰锛孶ser->User 缂撹В鐢?Spectre v2 缂撹В涓?IBPB 鏂归潰鐨勯€夋嫨鏂瑰紡鎺у埗锛?
     - conditional IBPB锛堟潯浠?IBPB锛夛細

       姣忎釜杩涚▼鍙互閫夋嫨鏄惁闇€瑕佸湪鍏跺懆鍥村彂鍑?IBPB锛岄€氳繃 PR_SPEC_DISABLE/_ENABLE 绛夛紝鍙傝 [spectre](spectre)

     - strict锛堜弗鏍硷級锛?
       鍗冲缁堝紑鍚?鈥斺€?閫氳繃鍦ㄥ唴鏍稿懡浠よ涓婃彁渚?spectre_v2_user=on

   (spec_rstack_overflow=microcode)

 - 'Mitigation: Safe RET'锛堢紦瑙ｏ細Safe RET锛夛細

   寰爜/杞欢缁勫悎缂撹В銆傚畠閫氳繃瑙ｅ喅 User->Kernel 鍜?Guest->Host 杞崲淇濇姢鏉ヨˉ鍏呮墿灞?IBPB 寰爜琛ヤ竵鍔熻兘銆?
   榛樿閫夋嫨鎴栫粡鐢?spec_rstack_overflow=safe-ret 閫夋嫨銆?
 - 'Mitigation: IBPB'锛堢紦瑙ｏ細IBPB锛夛細

   涓庝笂闈㈢殑 鈥渟afe RET鈥?绫讳技鐨勪繚鎶わ紝浣嗗湪鐗规潈鍩熶氦鍙夛紙User->Kernel锛孏uest->Host锛夋椂閲囩敤 IBPB 灞忛殰銆?
  (spec_rstack_overflow=ibpb)

 - 'Mitigation: IBPB on VMEXIT'锛堢紦瑙ｏ細VMEXIT 涓婄殑 IBPB锛夛細

   瑙ｅ喅浜戞彁渚涘晢鍦烘櫙鐨勭紦瑙?鈥斺€?浠?Guest->Host 杞崲銆?
   (spec_rstack_overflow=ibpb-vmexit)

 - 'Mitigation: Reduced Speculation'锛堢紦瑙ｏ細鍑忓皯鐨勬帹娴嬶級锛?
   褰撻€夋嫨浜嗕笂闈㈢殑 鈥淚BPB on VMEXIT鈥?骞朵笖 CPU 鏀寔 BpSpecReduce 浣嶆椂锛屾缂撹В浼氳嚜鍔ㄥ惎鐢ㄣ€?
   瀹冨湪鍏锋湁 SRSO_USER_KERNEL_NO=1 CPUID 浣嶇殑鏈哄櫒涓婅嚜鍔ㄥ惎鐢ㄣ€傚湪杩欑鎯呭喌涓嬶紝浠ｇ爜閫昏緫鍒囨崲鍒颁笂闈㈢殑 =ibpb-vmexit 缂撹В锛屽洜涓虹敤鎴?鍐呮牳杈圭晫涓嶅啀鍙楀奖鍝嶏紝鍥犳涓嶅啀闇€瑕?鈥渟afe RET鈥濄€?
   鍦ㄥ惎鐢?IBPB on VMEXIT 缂撹В閫夐」鍚庯紝浼氭娴嬪埌 BpSpecReduce 浣嶏紙鎵€鏈夋绫绘満鍣ㄤ笂閮藉瓨鍦ㄨ鍔熻兘锛夛紝杩欏疄闄呬笂浼氳鐩?IBPB on VMEXIT锛屽洜涓哄畠鐨勬€ц兘褰卞搷灏忓緱澶氾紝骞朵笖涔熷鐞嗕簡 guest->host 鏀诲嚮鍚戦噺銆?
瑕佸埄鐢ㄨ婕忔礊锛屾敾鍑昏€呴渶瑕侊細

 - 鍦ㄦ満鍣ㄤ笂鑾峰緱鏈湴璁块棶鏉冮檺

 - 绐佺牬 kASLR

 - 鍦ㄨ繍琛岀殑鍐呮牳涓壘鍒板彲鐢ㄤ簬婕忔礊鍒╃敤鐨?gadget

 - 鏍规嵁寰灦鏋勶紝鍙兘闇€瑕佸湪鍏勫紵绾跨▼涓婂垱寤哄苟鍥哄畾涓€涓澶栫殑宸ヤ綔璐熻浇锛堝湪 fam 0x19 涓婁笉蹇呰锛?
 - 杩愯婕忔礊鍒╃敤

鑰冭檻鍒版瘡绉嶇紦瑙ｇ被鍨嬬殑鎬ц兘褰卞搷锛岄粯璁ょ殑鏄?'Mitigation: safe RET'锛屽畠搴斿鐞嗗ぇ澶氭暟鏀诲嚮鍚戦噺锛屽寘鎷湰鍦扮殑 User->Kernel 鍚戦噺銆?
涓€濡傛棦寰€锛屽缓璁敤鎴烽€氳繃瀹氭湡搴旂敤杞欢鏇存柊鏉ヤ繚鎸佸叾绯荤粺澶勪簬鏈€鏂扮姸鎬併€?
榛樿璁剧疆灏嗗湪闇€瑕佹椂閲嶆柊璇勪及锛岀壒鍒槸褰撳嚭鐜版柊鐨勬敾鍑诲悜閲忔椂銆?
姝ｅ鍙互鎺ㄦ祴鐨勶紝'Mitigation: safe RET' 纭疄浼氫互涓€瀹氱殑鎬ц兘涓轰唬浠凤紝鍏蜂綋鍙栧喅浜庡伐浣滆礋杞姐€傚鏋滀綘淇′换浣犵殑鐢ㄦ埛绌洪棿骞朵笖涓嶆兂鎵垮彈鎬ц兘褰卞搷锛屼綘鎬绘槸鍙互浣跨敤 spec_rstack_overflow=off 绂佺敤璇ョ紦瑙ｆ帾鏂姐€?
绫讳技鍦帮紝'Mitigation: IBPB' 鏄彟涓€绉嶅畬鏁寸殑缂撹В绫诲瀷锛屽湪搴旂敤浜嗙郴缁熸墍闇€鐨勫井鐮佽ˉ涓佸悗浣跨敤闂存帴鍒嗘敮棰勬祴灞忛殰銆傛缂撹В涔熶細甯︽潵鎬ц兘鎴愭湰銆?
### 缂撹В锛歋afe RET


璇ョ紦瑙ｉ€氳繃纭繚鎵€鏈?RET 鎸囦护閮芥帹娴嬪埌涓€涓彈鎺х殑浣嶇疆鏉ュ伐浣滐紝绫讳技浜庡湪 retpoline 搴忓垪涓帶鍒舵帹娴嬬殑鏂瑰紡銆備负姝わ紝__x86_return_thunk 寮哄埗 CPU 浣跨敤 鈥渟afe return鈥?搴忓垪鏉ヨ棰勬祴姣忎釜鍑芥暟杩斿洖銆?
涓轰簡纭繚姝ょ紦瑙ｇ殑瀹夊叏鎬э紝鍐呮牳蹇呴』纭繚 safe return 搴忓垪鏈韩涓嶅彈鏀诲嚮鑰呭共鎵般€傚湪 Zen3 鍜?Zen4 涓紝杩欐槸閫氳繃鍦ㄥ幓璁粌锛坲ntraining锛夊嚱鏁?srso_alias_untrain_ret() 鍜?safe return 鍑芥暟 srso_alias_safe_ret() 涔嬮棿鍒涘缓 BTB 鍒悕鏉ュ疄鐜扮殑锛岃繖浼氶┍閫愬彲鑳戒腑姣掔殑 BTB 鏉＄洰锛屽苟灏嗚瀹夊叏鐨勬潯鐩敤浜庢墍鏈夊嚱鏁拌繑鍥炪€?
鍦ㄨ緝鏃х殑 Zen1 鍜?Zen2 涓紝杩欐槸閫氳繃浣跨敤绫讳技浜?Retbleed 鐨勯噸瑙ｉ噴锛坮einterpretation锛夋妧鏈疄鐜扮殑锛歴rso_untrain_ret() 鍜?srso_safe_ret()銆?
### 妫€鏌?Safe RET 缂撹В纭疄鏈夋晥


濡傛灉鏈変汉鎯抽獙璇?SRSO safe RET 缂撹В鍦ㄥ唴鏍镐笂鏄惁宸ヤ綔锛屽彲浠ヤ娇鐢ㄤ袱涓€ц兘璁℃暟鍣細

- PMC_0xc8 - 閫€褰圭殑 RET/RET lw 璁℃暟
- PMC_0xc9 - 閫€褰圭殑 RET/RET lw 璇娴嬭鏁?
骞舵瘮杈冨湪鍐呮牳妯″紡涓嬫纭€€褰圭殑 RET 鏁颁笌璇娴嬮€€褰圭殑 RET 鏁般€傚彟涓€绉嶆寚瀹氳繖浜涗簨浠剁殑鏂瑰紡

```
        # perf list ex_ret_near_ret

        List of pre-defined events (to be used in -e or -M):

        core:
          ex_ret_near_ret
               [Retired Near Returns]
          ex_ret_near_ret_mispred
               [Retired Near Returns Mispredicted]
```
```
        # perf stat -e ex_ret_near_ret:k -e ex_ret_near_ret_mispred:k sleep 10s
```
```
        # perf stat -e cpu/event=0xc8,umask=0/k -e cpu/event=0xc9,umask=0/k sleep 10s
```
搴旇缁欏嚭鐩稿悓鐨勬暟閲忋€傚嵆锛屾瘡涓€€褰圭殑 RET 搴?
```
        [root@brent: ~/kernel/linux/tools/perf> ./perf stat -e cpu/event=0xc8,umask=0/k -e cpu/event=0xc9,umask=0/k sleep 10s

         Performance counter stats for 'sleep 10s':

                   137,167      cpu/event=0xc8,umask=0/k
                   137,173      cpu/event=0xc9,umask=0/k

              10.004110303 seconds time elapsed

               0.000000000 seconds user
               0.004462000 seconds sys
```
鐩稿浜庣紦瑙ｈ绂佺敤锛坰pec_rstack_overflow=off锛夋垨杩愪綔涓嶆甯哥殑鎯呭喌锛屽悗鑰呴€氬父鏄剧ず璇娴嬮€€褰?RET 鐨勬暟閲忚繙灏忎簬閫€褰?RET 鐨勬€绘暟锛屽湪

```
       [root@brent: ~/kernel/linux/tools/perf> ./perf stat -e cpu/event=0xc8,umask=0/k -e cpu/event=0xc9,umask=0/k sleep 10s

        Performance counter stats for 'sleep 10s':

                  201,627      cpu/event=0xc8,umask=0/k
                    4,074      cpu/event=0xc9,umask=0/k

             10.003267252 seconds time elapsed

              0.002729000 seconds user
              0.000000000 seconds sys
```
鍙﹀锛岃繕鏈変竴涓墽琛屼笂杩版搷浣滅殑 selftest锛屽墠寰€

```
        make srso
        ./srso
```
