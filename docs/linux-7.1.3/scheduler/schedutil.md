## Schedutil


   鎵€鏈夎繖涓€鍒囬兘鍋囪棰戠巼涓庡伐浣滆兘鍔涗箣闂村瓨鍦ㄧ嚎鎬у叧绯伙紝鎴戜滑鐭ラ亾杩欐湁缂洪櫡锛屼絾瀹冩槸鐜版湁鏈€鍙鐨勮繎浼笺€?

## PELT锛堟瘡瀹炰綋璐熻浇璺熻釜锛孭er Entity Load Tracking锛?


閫氳繃 PELT锛屾垜浠法鍚勭璋冨害瀹炰綋璺熻釜涓€浜涙寚鏍囷紝浠庡崟涓换鍔°€佷换鍔＄粍鍒囩墖鍒?CPU 杩愯闃熷垪銆備綔涓哄熀纭€锛屾垜浠娇鐢ㄦ寚鏁板姞鏉冪Щ鍔ㄥ钩鍧囷紙EWMA锛夛紝姣忎釜鍛ㄦ湡锛?024us锛夎琛板噺锛屼娇寰?y^32 = 0.5銆備篃灏辨槸璇达紝鏈€杩戠殑 32ms 璐＄尞涓€鍗婏紝鑰屽巻鍙茬殑鍏朵綑閮ㄥ垎璐＄尞鍙︿竴鍗娿€?

鍏蜂綋涓猴細

  ewma_sum(u) := u_0 + u_1**y + u_2**y^2 + ...

  ewma(u) = ewma_sum(u) / ewma_sum(1)

鐢变簬杩欐湰璐ㄤ笂鏄竴涓棤绌风瓑姣旀暟鍒楃殑绾ф暟锛岀粨鏋滄槸鍙粍鍚堢殑锛屽嵆 ewma(A) + ewma(B) = ewma(A+B)銆傝繖涓€鎬ц川寰堝叧閿紝鍥犱负瀹冧娇寰楀湪浠诲姟杩佺Щ鏃惰兘澶熼噸鏂扮粍鍚堣繖浜涘钩鍧囧€笺€?

娉ㄦ剰锛岃闃诲鐨勪换鍔′粛鐒朵細瀵硅仛鍚堝€硷紙浠诲姟缁勫垏鐗囧拰 CPU 杩愯闃熷垪锛変骇鐢熻础鐚紝杩欏弽鏄犱簡瀹冧滑鍦ㄦ仮澶嶈繍琛屾椂鐨勯鏈熻础鐚€?

鍒╃敤杩欎竴鐐癸紝鎴戜滑璺熻釜涓や釜鍏抽敭鎸囨爣锛?running'锛堣繍琛岋級鍜?'runnable'锛堝彲杩愯锛夈€?running' 鍙嶆槧涓€涓疄浣撳湪 CPU 涓婅姳璐圭殑鏃堕棿锛岃€?'runnable' 鍙嶆槧涓€涓疄浣撳湪杩愯闃熷垪涓婅姳璐圭殑鏃堕棿銆傚綋鍙湁涓€涓换鍔℃椂锛岃繖涓や釜鎸囨爣鐩稿悓锛涗絾涓€鏃?CPU 鍑虹幇浜夌敤锛?running' 浼氫笅闄嶄互鍙嶆槧姣忎釜浠诲姟鍦?CPU 涓婅姳璐圭殑鏃堕棿姣斾緥锛岃€?'runnable' 浼氫笂鍗囦互鍙嶆槧浜夌敤鐨勭▼搴︺€?

鏇村缁嗚妭鍙傝锛歬ernel/sched/pelt.c


## 棰戠巼 / CPU 涓嶅彉鎬?


鐢变簬鍦?1GHz 涓嬪崰鐢?CPU 50% 涓嶅悓浜庡湪 2GHz 涓嬪崰鐢?CPU 50%锛屽湪 LITTLE CPU 涓婅繍琛?50% 涔熶笉鍚屼簬鍦?big CPU 涓婅繍琛?50%锛屾垜浠厑璁告灦鏋勭敤涓や釜姣斾緥瀵规椂闂村閲忚繘琛岀缉鏀撅細涓€涓槸鍔ㄦ€佺數鍘嬩笌棰戠巼璋冩暣锛圖VFS锛夋瘮渚嬶紝涓€涓槸寰灦鏋勬瘮渚嬨€?

瀵逛簬绠€鍗曠殑 DVFS 鏋舵瀯锛堣蒋浠跺畬鍏ㄥ彲鎺х殑鎯呭喌锛夛紝鎴戜滑鍙互杞绘槗鍦?
```

	    f_cur
  r_dvfs := -----
            f_max

```
瀵逛簬纭欢鎺у埗 DVFS 鐨勬洿鍏峰姩鎬佹€х殑绯荤粺锛屾垜浠娇鐢ㄧ‖浠惰鏁板櫒锛圛ntel APERF/MPERF銆丄RMv8.4-AMU锛夋潵鎻愪緵璇ユ瘮渚嬨€?
```

	   APERF
  f_cur := ----- * P0
	   MPERF

	     4C-turbo;	if available and turbo enabled
  f_max := { 1C-turbo;	if turbo enabled
	     P0;	otherwise

                    f_cur
  r_dvfs := min( 1, ----- )
                    f_max

```
鎴戜滑閫夋嫨 4C turbo 鑰岄潪 1C turbo锛屼互浣垮叾鐣ュ井鏇村叿鍙寔缁€с€?

r_cpu 琚‘瀹氫负褰撳墠 CPU 鐨勬渶楂樻€ц兘绾у埆涓庣郴缁熶腑浠讳綍鍏朵粬 CPU 鐨勬渶楂樻€ц兘绾у埆涔嬫瘮銆?

  r_tot = r_dvfs * r_cpu

缁撴灉鏄紝涓婅堪鐨?'running' 鍜?'runnable' 鎸囨爣鍙樺緱涓?DVFS 鍜?CPU 绫诲瀷鏃犲叧銆傛崲瑷€涔嬶紝鎴戜滑鍙互鍦?CPU 涔嬮棿杞Щ骞舵瘮杈冨畠浠€?

鏇村缁嗚妭鍙傝锛?

 - kernel/sched/pelt.h:update_rq_clock_pelt()
 - arch/x86/kernel/smpboot.c:"APERF/MPERF frequency ratio computation."
 - Documentation/scheduler/sched-capacity.rst:"1. CPU Capacity + 2. Task utilization"


## UTIL_EST


鐢变簬鍛ㄦ湡鎬т换鍔″湪鐫＄湢鏃跺叾骞冲潎鍊间細琛板噺锛屽嵆渚胯繍琛屾椂鍏堕鏈熷埄鐢ㄧ巼鐩稿悓锛屽畠浠篃浼氬湪鍐嶆杩愯鏃堕伃鍙楋紙DVFS锛夌埇鍗囥€?

涓虹紦瑙ｈ繖涓€鐐癸紙涓€涓粯璁ゅ紑鍚殑閫夐」锛夛紝UTIL_EST 鍦ㄥ嚭闃熸椂鈥斺€斿綋鍏舵渶楂樻椂鈥斺€斾互涓€涓?'running' 鍊奸┍鍔ㄤ竴涓棤闄愬啿婵€鍝嶅簲锛圛IR锛塃WMA銆俇TIL_EST 婊ゆ尝鍣ㄤ細绔嬪嵆澧炲ぇ锛屽苟涓斿彧鍦ㄥ噺灏忔椂琛板噺銆?

杩涗竴姝ョ淮鎶や竴涓繍琛岄槦鍒楄寖鍥寸殑銆侀拡瀵癸紙鍙繍琛屼换鍔★級鐨勬眰鍜岋細

  util_est := \Sum_t max( t_running, t_util_est_ewma )

鏇村缁嗚妭鍙傝锛歬ernel/sched/fair.c:util_est_dequeue()


## UCLAMP


鍙互涓烘瘡涓?CFS 鎴?RT 浠诲姟璁剧疆鏈夋晥鐨?u_min 鍜?u_max 閽冲埗鍊硷紱杩愯闃熷垪涓烘墍鏈夋鍦ㄨ繍琛岀殑浠诲姟缁存姢杩欎簺閽冲埗鍊肩殑鏈€澶ц仛鍚堛€?

鏇村缁嗚妭鍙傝锛歩nclude/uapi/linux/sched/types.h


## Schedutil / DVFS


姣忓綋璋冨害鍣ㄨ礋杞借窡韪鏇存柊锛堜换鍔″敜閱掋€佷换鍔¤縼绉汇€佹椂闂存帹杩涳級鏃讹紝鎴戜滑閮戒細璋冪敤 schedutil 鏉ユ洿鏂扮‖浠?DVFS 鐘舵€併€?

鍏跺熀纭€鏄?CPU 杩愯闃熷垪鐨?'running' 鎸囨爣锛屾牴鎹笂杩板唴瀹癸紝瀹冩槸 CPU 鐨勯鐜囦笉鍙樺埄鐢ㄧ巼浼拌銆傜敱姝ゆ垜浠绠?
```

             max( running, util_est );	if UTIL_EST
  u_cfs := { running;			otherwise

               clamp( u_cfs + u_rt , u_min, u_max );	if UCLAMP_TASK
  u_clamp := { u_cfs + u_rt;				otherwise

  u := u_clamp + u_irq + u_dl;		[approx. see source for more detail]

  f_des := min( f_max, 1.25 u * f_max )

```
XXX IO-wait锛氬綋鏇存柊鏄敱浜?IO 瀹屾垚瀵艰嚧鐨勪换鍔″敜閱掓椂锛屾垜浠皢涓婇潰鐨?'u' 鎻愬崌銆?

闅忓悗璇ラ鐜囪鐢ㄦ潵閫夋嫨涓€涓?P-state/OPP锛屾垨琚洿鎺ヨ浆鎹负涓€涓彂閫佺粰纭欢鐨?CPPC 椋庢牸璇锋眰銆?

XXX锛氭埅姝㈡湡闄愪换鍔★紙Sporadic Task Model锛岀獊鍙戜换鍔℃ā鍨嬶級鍏佽鎴戜滑璁＄畻鍑烘弧瓒宠宸ヤ綔璐熻浇鎵€闇€鐨勭‖ f_min銆?

鐢变簬杩欎簺鍥炶皟鐩存帴鏉ヨ嚜璋冨害鍣紝DVFS 纭欢浜や簰搴斿綋鈥滃揩閫熲€濅笖闈為樆濉炪€係chedutil 鏀寔瀵?DVFS 璇锋眰杩涜闄愭祦锛屼互搴斿纭欢浜や簰缂撴參涓斾唬浠烽珮鏄傜殑鎯呭喌锛岃繖浼氶檷浣庢湁鏁堟€с€?

鏇村淇℃伅鍙傝锛歬ernel/sched/cpufreq_schedutil.c


## 娉ㄦ剰浜嬮」


 - 鍦ㄤ綆璐熻浇鍦烘櫙涓嬶紝DVFS 鏈€涓洪噸瑕侊紝'running' 鏁板€煎皢瀵嗗垏鍙嶆槧鍒╃敤鐜囥€?

 - 鍦ㄩケ鍜屽満鏅笅锛屼换鍔¤縼绉讳細瀵艰嚧涓€浜涚灛鏃朵笅闄嶏紝鍋囪鎴戜滑鏈変竴涓 4 涓换鍔￠ケ鍜岀殑 CPU锛岄偅涔堝綋鎴戜滑灏嗕竴涓换鍔¤縼绉诲埌绌洪棽 CPU 鏃讹紝鏃?CPU 灏嗗叿鏈?0.75 鐨?'running' 鍊硷紝鑰屾柊 CPU 灏嗚幏寰?0.25銆傝繖鏄笉鍙伩鍏嶇殑锛屾椂闂存帹杩涗細绾犳杩欎竴鐐广€俋XX 鐢变簬涓嶅瓨鍦ㄧ┖闂叉椂闂达紝鎴戜滑鏄惁浠嶇劧淇濊瘉 f_max锛?

 - 涓婅堪澶ч儴鍒嗗唴瀹规槸鍏充簬閬垮厤 DVFS 涓嬮檷锛屼互鍙婄嫭绔嬬殑 DVFS 鍩熷湪璐熻浇杞Щ鏃朵笉寰椾笉閲嶆柊瀛︿範/鐖崌銆?
