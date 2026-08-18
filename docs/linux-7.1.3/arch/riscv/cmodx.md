
## RISC-V Linux 鐨勬寚浠ゅ苟鍙戜慨鏀逛笌鎵ц锛圕MODX锛?

CMODX 鏄竴绉嶇紪绋嬫妧鏈紝绋嬪簭鎵ц鐢辩▼搴忚嚜韬慨鏀硅繃鐨勬寚浠ゃ€傚湪 RISC-V 纭欢涓婏紝鎸囦护瀛樺偍
鍜屾寚浠ょ紦瀛橈紙icache锛夊苟涓嶄繚璇佸悓姝ャ€傚洜姝わ紝绋嬪簭蹇呴』鍊熷姪闈炵壒鏉冩寚浠?fence.i 鏉ュ己鍒惰繘琛?鑷韩鐨勫悓姝ャ€?
### 鍐呮牳绌洪棿涓殑 CMODX


### 鍔ㄦ€?ftrace


鏈川涓婏紝鍔ㄦ€?ftrace 閫氳繃鍦ㄦ瘡涓彲鎵撹ˉ涓佺殑鍑芥暟鍏ュ彛鎻掑叆涓€涓嚱鏁拌皟鐢ㄦ潵寮曞鎺у埗娴侊紝
骞跺湪杩愯鏃跺姩鎬佸湴缁欏叾鎵撹ˉ涓佷互鍚敤鎴栫鐢ㄩ噸瀹氬悜銆傚湪 RISC-V 鐨勬儏鍐典笅锛岄渶瑕?2 鏉℃寚浠?锛圓UIPC + JALR锛夋潵缁勬垚涓€涓嚱鏁拌皟鐢ㄣ€傜劧鑰岋紝涓嶅彲鑳藉湪鎵?2 鏉℃寚浠よˉ涓佺殑鍚屾椂鏈熸湜骞跺彂鐨?璇荤鏃犵珵浜夋潯浠跺湴鎵ц瀹冧滑銆傝繖涓郴鍒椾娇寰楀湪 RISC-V ftrace 涓繘琛屽師瀛愪唬鐮佽ˉ涓佹垚涓哄彲鑳姐€?鍐呮牳鎶㈠崰浣挎儏鍐垫洿绯燂紝鍥犱负瀹冨厑璁告棫鐘舵€佸湪鎵撹ˉ涓佺殑杩囩▼涓紙閫氳繃 stop_machine()锛夋寔缁瓨鍦ㄣ€?
涓轰簡鎽嗚劚 stop_machine() 骞跺湪瀹屽叏鍐呮牳鎶㈠崰涓嬭繍琛屽姩鎬?ftrace锛屾垜浠湪鍚姩鏃堕儴鍒嗗垵濮嬪寲
姣忎釜鍙墦琛ヤ竵鐨勫嚱鏁板叆鍙ｏ紝灏嗙涓€鏉℃寚浠よ涓?AUIPC锛岀浜屾潯璁句负 NOP銆傜幇鍦ㄥ師瀛愭墦琛ヤ竵鎴愪负
鍙兘锛屽洜涓哄唴鏍稿彧闇€鏇存柊涓€鏉℃寚浠ゃ€傛牴鎹?Ziccif锛屽彧瑕佹寚浠ゆ槸鑷劧瀵归綈鐨勶紝ISA 灏变繚璇佸師瀛?鏇存柊銆?
閫氳繃鍥哄畾绗竴鏉℃寚浠?AUIPC锛岀敱浜?RISC-V 涓珛鍗虫暟缂栫爜绌洪棿涓嶈冻锛宖trace 璺虫澘鐨勫鍧€鑼冨洿
琚檺鍒跺湪璺濈棰勫畾鐩爣 ftrace_caller 鐨?+-2K 涔嬪唴銆備负浜嗚В鍐宠繖涓棶棰橈紝鎴戜滑寮曞叆浜?CALL_OPS锛屽湪姣忎釜鍙墦琛ヤ竵鐨勫嚱鏁板墠闈㈡坊鍔犱竴涓?8 瀛楄妭鑷劧瀵归綈鐨勫厓鏁版嵁銆傝鍏冩暟鎹湪绗竴涓?璺虫澘澶勮瑙ｆ瀽锛岀劧鍚庢墽琛屽彲浠ヨ寮曞鍒板彟涓€涓嚜瀹氫箟璺虫澘銆?
### 鐢ㄦ埛绌洪棿涓殑 CMODX


灏界 fence.i 鏄潪鐗规潈鎸囦护锛屼絾榛樿鐨?Linux ABI 绂佹鍦ㄧ敤鎴风┖闂村簲鐢ㄧ▼搴忎腑浣跨敤 fence.i銆?璋冨害鍣ㄩ殢鏃跺彲鑳藉皢浠诲姟杩佺Щ鍒颁竴涓柊鐨?hart 涓娿€傚鏋滆縼绉诲彂鐢熷湪鐢ㄦ埛绌洪棿鐢?fence.i 鍚屾浜?icache 鍜屾寚浠ゅ瓨鍌ㄤ箣鍚庯紝鏂?hart 涓婄殑 icache 灏嗕笉鍐嶅共鍑€銆傝繖鏄洜涓?fence.i 鐨勮涓哄彧褰卞搷
璋冪敤瀹冪殑閭ｄ釜 hart銆傚洜姝わ紝浠诲姟琚縼绉诲埌鐨?hart 鍙兘灏氭湭鍚屾鎸囦护瀛樺偍鍜?icache銆?
鏈変袱绉嶆柟娉曞彲浠ヨВ鍐宠繖涓棶棰橈細浣跨敤 riscv_flush_icache() 绯荤粺璋冪敤锛屾垨鑰呬娇鐢?`PR_RISCV_SET_ICACHE_FLUSH_CTX` prctl() 骞跺湪鐢ㄦ埛绌洪棿鍙戝嚭 fence.i銆傜郴缁熻皟鐢ㄦ墽琛屼竴娆℃€х殑
icache 鍒锋柊鎿嶄綔銆俻rctl 鏀瑰彉 Linux ABI 浠ュ厑璁哥敤鎴风┖闂村彂鍑?icache 鍒锋柊鎿嶄綔銆?
椤轰究涓€鎻愶紝鍦ㄥ唴鏍镐腑鏈夋椂鍙兘浼氳Е鍙戔€滃欢杩熲€濈殑 icache 鍒锋柊銆傚湪鎾板啓鏈枃鏃讹紝杩欎粎鍙戠敓鍦?riscv_flush_icache() 绯荤粺璋冪敤鏈熼棿锛屼互鍙婂唴鏍镐娇鐢?copy_to_user_page() 鏃躲€傝繖浜涘欢杩熷埛鏂?鍙湪璇?hart 姝ｅ湪浣跨敤鐨勫唴瀛樻槧灏勫彂鐢熷彉鍖栨椂鍙戠敓銆傚鏋?prctl() 涓婁笅鏂囧凡缁忓鑷翠簡涓€娆?icache
鍒锋柊锛屽垯璇ュ欢杩?icache 鍒锋柊灏嗚璺宠繃锛屽洜涓哄畠鏄啑浣欑殑銆傚洜姝わ紝鍦?prctl() 涓婁笅鏂囧唴閮ㄤ娇鐢?riscv_flush_icache() 绯荤粺璋冪敤鏃朵笉浼氬彂鐢熼澶栫殑鍒锋柊銆?
### prctl() 鎺ュ彛


浠?`PR_RISCV_SET_ICACHE_FLUSH_CTX` 浣滀负绗竴涓弬鏁拌皟鐢?prctl()銆傚叾浣欏弬鏁板皢濮旀墭缁欎笅鏂?璇﹁堪鐨?riscv_set_icache_flush_ctx 鍑芥暟銆?
	:identifiers: riscv_set_icache_flush_ctx

浣跨敤绀轰緥锛?
浠ヤ笅鏂囦欢鏃ㄥ湪鐩镐簰缂栬瘧骞堕摼鎺ュ湪涓€璧枫€俶odify_instruction() 鍑芥暟灏嗕竴涓姞 0 鐨勫姞娉曟浛鎹负
涓€涓姞 1 鐨勫姞娉曪紝浣?get_value() 涓殑鎸囦护搴忓垪浠庤繑鍥為浂鍙樹负杩斿洖涓€銆?
```

	#include <stdio.h>
	#include <sys/prctl.h>

	extern int get_value();
	extern void modify_instruction();

	int main()
	{
		int value = get_value();
		printf("Value before cmodx: %d\n", value);

		// Call prctl before first fence.i is called inside modify_instruction
		prctl(PR_RISCV_SET_ICACHE_FLUSH_CTX, PR_RISCV_CTX_SW_FENCEI_ON, PR_RISCV_SCOPE_PER_PROCESS);
		modify_instruction();
		// Call prctl after final fence.i is called in process
		prctl(PR_RISCV_SET_ICACHE_FLUSH_CTX, PR_RISCV_CTX_SW_FENCEI_OFF, PR_RISCV_SCOPE_PER_PROCESS);

		value = get_value();
		printf("Value after cmodx: %d\n", value);
		return 0;
	}

```
```

	.option norvc

	.text
	.global modify_instruction
	modify_instruction:
	lw a0, new_insn
	lui a5,%hi(old_insn)
	sw  a0,%lo(old_insn)(a5)
	fence.i
	ret

	.section modifiable, "awx"
	.global get_value
	get_value:
	li a0, 0
	old_insn:
	addi a0, a0, 0
	ret

	.data
	new_insn:
	addi a0, a0, 1

```
