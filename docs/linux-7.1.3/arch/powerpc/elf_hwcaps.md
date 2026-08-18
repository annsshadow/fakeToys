
## POWERPC ELF HWCAPs锛堢‖浠惰兘鍔涙爣蹇楋級

鏈枃妗ｆ弿杩?POWERPC ELF HWCAPs 鐨勪娇鐢ㄨ涔夈€?


### 1. 绠€浠?

鏌愪簺纭欢涓庤蒋浠剁壒鎬т粎鍦ㄧ壒瀹氱殑 CPU 涓婂彲鐢紝鎴栦粎鍦ㄧ壒瀹氱殑鍐呮牳閰嶇疆涓嬪彲鐢ㄣ€傜敤鎴风┖闂翠唬鐮佸彲鐢ㄧ殑鍙戠幇鏈哄埗鏄?HWCAPs鈥斺€旇繖鏄竴缁勭敱鍐呮牳鍦ㄨ緟鍔╁悜閲忥紙auxiliary vector锛変腑鍚戠敤鎴风┖闂存毚闇茬殑鏍囧織浣嶃€?

鐢ㄦ埛绌洪棿杞欢鍙互閫氳繃鑾峰彇杈呭姪鍚戦噺涓殑 `AT_HWCAP` 鎴?`AT_HWCAP2` 椤癸紝骞舵祴璇曠浉搴旂殑鏍囧織浣嶏紝鏉ュ垽鏂煇椤圭壒鎬ф槸鍚﹀彲鐢ㄣ€?

```
	bool floating_point_is_present(void)
	{
		unsigned long HWCAPs = getauxval(AT_HWCAP);
		if (HWCAPs & PPC_FEATURE_HAS_FPU)
			return true;

		return false;
	}
```

渚濊禆鏌愰」 HWCAP 鎵€鎻忚堪鐗规€х殑杞欢锛屽簲褰撴鏌ョ浉搴旂殑 HWCAP 鏍囧織浣嶄互纭璇ョ壒鎬х‘瀹炲瓨鍦紝鐒跺悗鍐嶅幓浣跨敤瀹冦€?

鐩告瘮涓诲姩鎺㈡祴锛坧robing锛夌瓑鎵嬫锛孒WCAP 鏄祴璇曠壒鎬ф槸鍚﹀瓨鍦ㄧ殑棣栭€夋柟寮忥紝鍥犱负鎺㈡祴鎵嬫鍙兘瀵艰嚧涓嶅彲棰勬湡鐨勮涓恒€?

闈㈠悜鐗瑰畾骞冲彴鐨勮蒋浠朵笉涓€瀹氶渶瑕佹祴璇曢偅浜涘叾鎵€渚濊禆鐨勩€侀殣鍚繀澶囩殑鐗规€с€備緥濡傦紝涓€涓渶瑕?FPU銆乂MX銆乂SX 鐨勭▼搴忥紝蹇呴』娴嬭瘯鐩稿簲鐨?HWCAPs锛屽惁鍒欑紪璇戝櫒鐢熸垚鐨勩€佽姹傝繖浜涚壒鎬х殑浠ｇ爜灏嗘棤娉曡繍琛屻€?

### 2. Facilities锛堣鏂斤級

Power ISA 浣跨敤鏈 "facility"锛堣鏂斤級鏉ユ弿杩颁竴绫绘寚浠ゃ€佸瘎瀛樺櫒銆佷腑鏂瓑銆傛煇涓?facility 鐨勫瓨鍦ㄤ笌鍚︼紝琛ㄧず璇ョ被涓浉鍏冲姛鑳芥槸鍚﹀彲鐢紝鍏蜂綋缁嗚妭鍒欏彇鍐充簬 ISA 鐗堟湰銆備緥濡傦紝鑻?VSX facility 鍙敤锛屽垯 VSX 鎸囦护鐨勪娇鐢ㄦ柟寮忓湪 v3.0B 涓?v3.1B 绛?ISA 鐗堟湰涔嬮棿浼氭湁鎵€涓嶅悓銆?

### 3. Categories锛堢被鍒級

Power ISA v3.0 浣跨敤鏈 "category"锛堢被鍒級鏉ユ弿杩版煇浜涙寚浠ょ被鎴栨搷浣滄ā寮忥紝瀹冧滑鍙兘鏄彲閫夌殑銆佷篃鍙兘浜掓枼銆傚叾纭垏鍚箟鍙栧喅浜庡叿浣撶殑 HWCAP 鏍囧織浣嶄笌涓婁笅鏂囥€備緥濡傦紝瀛樺湪 BOOKE 鐗规€ф剰鍛崇潃瀹炵幇浜?server category銆?

### 4. HWCAP 鍒嗛厤

HWCAPs 鐨勫垎閰嶆柟寮忓湪 Power 鏋舵瀯 64 浣?ELF V2 ABI 瑙勮寖涓弿杩帮紙骞跺弽鏄犲湪鍐呮牳鐨?uapi 澶存枃浠朵腑锛夈€?

### 5. HWCAPs exposed AT_HWCAP

PPC_FEATURE_32
32 浣?CPU銆?

PPC_FEATURE_64
64 浣?CPU锛堢敤鎴风┖闂磋繍琛屼簬 32 浣嶆ā寮忥級銆?

PPC_FEATURE_601_INSTR
PowerPC 601 澶勭悊鍣ㄣ€傝嚜鎻愪氦 f0ed73f3fa2c锛?powerpc: 绉婚櫎 PowerPC 601"锛夎捣鍐呮牳涓嶅啀浣跨敤銆?

PPC_FEATURE_HAS_ALTIVEC
鍚戦噺锛堝張绉?Altivec銆乂MX锛塮acility 鍙敤銆?

PPC_FEATURE_HAS_FPU
娴偣 facility 鍙敤銆?

PPC_FEATURE_HAS_MMU
瀛樺湪骞跺凡鍚敤鍐呭瓨绠＄悊鍗曞厓锛圡MU锛夈€?

PPC_FEATURE_HAS_4xxMAC
40x 鎴?44x 绯诲垪澶勭悊鍣ㄣ€傝嚜鎻愪氦 732b32daef80锛?powerpc: 绉婚櫎鏍稿績鏀寔 40x"锛夎捣鍐呮牳涓嶅啀浣跨敤銆?

PPC_FEATURE_UNIFIED_CACHE
澶勭悊鍣ㄩ噰鐢ㄧ粺涓€鐨?L1 缂撳瓨锛堟寚浠や笌鏁版嵁鍏变韩锛夛紝瑙佷簬 NXP e200銆傝嚜鎻愪氦 39c8bf2b3cc1锛?powerpc: Retire e200 鏍稿績 (mpc555x processor)"锛夎捣鍐呮牳涓嶅啀浣跨敤銆?

PPC_FEATURE_HAS_SPE
淇″彿澶勭悊寮曟搸锛圫ignal Processing Engine锛塮acility 鍙敤銆?

PPC_FEATURE_HAS_EFP_SINGLE
宓屽叆寮忔诞鐐瑰崟绮惧害鎿嶄綔鍙敤銆?

PPC_FEATURE_HAS_EFP_DOUBLE
宓屽叆寮忔诞鐐瑰弻绮惧害鎿嶄綔鍙敤銆?

PPC_FEATURE_NO_TB
timebase facility锛坢ftb 鎸囦护锛夊彲鐢ㄣ€傝繖鏄?601 鐗规湁鐨?HWCAP锛涗竴鏃︾‘瀹氬鐞嗗櫒涓?601锛堢敱 HWCAPs 鎸囩ず锛夛紝灏卞繀椤绘祴璇曡浣嶄互浣跨敤 timebase銆傝嚜鎻愪氦 f0ed73f3fa2c锛?powerpc: 绉婚櫎 PowerPC 601"锛夎捣鍐呮牳涓嶅啀浣跨敤銆?

PPC_FEATURE_POWER4
POWER4 鎴?PPC970/FX/MP 澶勭悊鍣ㄣ€傚 POWER4 鐨勬敮鎸佽嚜鎻愪氦 471d7ff8b51b锛?powerpc/64s: 绉婚櫎 POWER4 鏀寔"锛夎捣宸蹭粠鍐呮牳涓Щ闄ゃ€?

PPC_FEATURE_POWER5
POWER5 澶勭悊鍣ㄣ€?

PPC_FEATURE_POWER5_PLUS
POWER5+ 澶勭悊鍣ㄣ€?

PPC_FEATURE_CELL
Cell 澶勭悊鍣ㄣ€?

PPC_FEATURE_BOOKE
澶勭悊鍣ㄥ疄鐜颁簡宓屽叆寮忕被鍒紙"BookE"锛夋灦鏋勩€?

PPC_FEATURE_SMT
澶勭悊鍣ㄥ疄鐜颁簡鍚屾澶氱嚎绋嬶紙SMT锛夈€?

PPC_FEATURE_ICACHE_SNOOP
澶勭悊鍣ㄧ殑鎸囦护缂撳瓨涓庢暟鎹紦瀛樹竴鑷达紱涓轰娇鎸囦护瀛樺偍涓庢暟鎹瓨鍌ㄤ繚鎸佷竴鑷翠互渚挎墽琛屾寚浠ゅ簭鍒楋紙濡?POWER9 澶勭悊鍣ㄤ腑鎵€杩帮級锛岄渶瑕侊細

```
        sync
        icbi (to any address)
        isync
```

PPC_FEATURE_ARCH_2_05
澶勭悊鍣ㄦ敮鎸?v2.05 鐢ㄦ埛鎬佹灦鏋勩€傛敮鎸佹洿楂樻灦鏋勭増鏈殑澶勭悊鍣ㄤ笉浼氳缃鐗规€с€?

PPC_FEATURE_PA6T
PA6T 澶勭悊鍣ㄣ€?

PPC_FEATURE_HAS_DFP
DFP锛堝崄杩涘埗娴偣锛塮acility 鍙敤銆?

PPC_FEATURE_POWER6_EXT
POWER6 澶勭悊鍣ㄣ€?

PPC_FEATURE_ARCH_2_06
澶勭悊鍣ㄦ敮鎸?v2.06 鐢ㄦ埛鎬佹灦鏋勩€傛敮鎸佹洿楂樻灦鏋勭増鏈殑澶勭悊鍣ㄤ細璁剧疆璇ョ壒鎬с€?

PPC_FEATURE_HAS_VSX
VSX facility 鍙敤銆?

PPC_FEATURE_PSERIES_PERFMON_COMPAT
澶勭悊鍣ㄦ敮鎸佹灦鏋勫畾涔夌殑 PMU 浜嬩欢鑼冨洿 0xE0-0xFF銆?

PPC_FEATURE_TRUE_LE
澶勭悊鍣ㄦ敮鎸佺湡姝ｇ殑 little-endian 妯″紡銆?

PPC_FEATURE_PPC_LE
澶勭悊鍣ㄦ敮鎸?"PowerPC Little-Endian"锛岄€氳繃鍦板潃鍙樻崲浣垮瓨鍌ㄨ闂〃鐜颁负 little-endian锛屼絾鏁版嵁浠ヤ笉鍚屾牸寮忓瓨鍌紝涓嶉€傚悎浠ヨ妯″紡杩愯鐨勫叾瀹冭闂€呬娇鐢ㄣ€?

### 6. HWCAPs exposed AT_HWCAP2

PPC_FEATURE2_ARCH_2_07
澶勭悊鍣ㄦ敮鎸?v2.07 鐢ㄦ埛鎬佹灦鏋勩€傛敮鎸佹洿楂樻灦鏋勭増鏈殑澶勭悊鍣ㄤ細璁剧疆璇ョ壒鎬с€?

PPC_FEATURE2_HTM
浜嬪姟鎬у唴瀛橈紙Transactional Memory锛夌壒鎬у彲鐢ㄣ€?

PPC_FEATURE2_DSCR
DSCR facility 鍙敤銆?

PPC_FEATURE2_EBB
EBB锛圗vent Based Branch锛塮acility 鍙敤銆?

PPC_FEATURE2_ISEL
isel 鎸囦护鍙敤銆傚湪 ARCH_2_07 鍙婁箣鍚庤鍙栦唬銆?

PPC_FEATURE2_TAR
TAR facility 鍙敤銆?

PPC_FEATURE2_VEC_CRYPTO
v2.07 鍔犲瘑鎸囦护鍙敤銆?

PPC_FEATURE2_HTM_NOSC
鍦ㄤ簨鍔℃€х姸鎬佷笅鍙戣捣绯荤粺璋冪敤灏嗗け璐ワ紝鍙傝 鏂囨。/arch/powerpc/syscall64-abi.rst銆?

PPC_FEATURE2_ARCH_3_00
澶勭悊鍣ㄦ敮鎸?v3.0B / v3.0C 鐢ㄦ埛鎬佹灦鏋勩€傛敮鎸佹洿楂樻灦鏋勭増鏈殑澶勭悊鍣ㄤ細璁剧疆璇ョ壒鎬с€?

PPC_FEATURE2_HAS_IEEE128
IEEE 128 浣嶄簩杩涘埗娴偣锛屾敮鎸?VSX 鍥涚簿搴︽寚浠や笌鏁版嵁绫诲瀷銆?

PPC_FEATURE2_DARN
darn 鎸囦护鍙敤銆?

PPC_FEATURE2_SCV
浣跨敤 scv 0 鎸囦护杩涜绯荤粺璋冪敤锛屽弬瑙?鏂囨。/arch/powerpc/syscall64-abi.rst銆?

PPC_FEATURE2_HTM_NO_SUSPEND
鏈夐檺鐨勪簨鍔℃€у唴瀛?facility 鏀寔锛堜笉鏀寔鎸傝捣锛夊彲鐢紝鍙傝 鏂囨。/arch/powerpc/transactional_memory.rst銆?

PPC_FEATURE2_ARCH_3_1
澶勭悊鍣ㄦ敮鎸?v3.1 鐢ㄦ埛鎬佹灦鏋勩€傛敮鎸佹洿楂樻灦鏋勭増鏈殑澶勭悊鍣ㄤ細璁剧疆璇ョ壒鎬с€?

PPC_FEATURE2_MMA
MMA facility 鍙敤銆?
