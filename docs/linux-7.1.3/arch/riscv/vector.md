
## RISC-V Linux 鐨勫悜閲忥紙Vector锛夋墿灞曟敮鎸?


鏈枃妗ｇ畝瑕佹杩颁簡 Linux 鎻愪緵缁欑敤鎴风┖闂淬€佺敤浜庢敮鎸?RISC-V 鍚戦噺鎵╁睍浣跨敤鐨勬帴鍙ｃ€?

### 1. prctl() 鎺ュ彛


鏂板浜嗕袱涓?prctl() 璋冪敤锛岀敤浜庤绋嬪簭绠＄悊鍦ㄧ敤鎴风┖闂翠娇鐢?Vector 鐨勫惎鐢ㄧ姸鎬併€傝繖浜涙帴鍙ｇ殑棰勬湡浣跨敤鍑嗗垯鏄负 init 绯荤粺鎻愪緵涓€绉嶆柟寮忥紝鐢ㄤ簬淇敼鍏跺煙涓嬭繍琛岀殑杩涚▼瀵?V 鐨勫彲鐢ㄦ€с€備笉寤鸿鍦ㄥ簱渚嬬▼涓皟鐢ㄨ繖浜涙帴鍙ｏ紝鍥犱负搴撲笉搴旇鐩栫敱鐖惰繘绋嬮厤缃殑绛栫暐銆傛澶栵紝鐢ㄦ埛蹇呴』娉ㄦ剰杩欎簺鎺ュ彛涓嶅彲绉绘鍒伴潪 Linux 浠ュ強闈?RISC-V 鐜锛屽洜姝や笉榧撳姳鍦ㄥ彲绉绘浠ｇ爜涓娇鐢ㄣ€傝鑾峰彇 ELF 绋嬪簭涓?V 鐨勫彲鐢ㄦ€э紝璇疯鍙栬緟鍔╁悜閲忎腑 `ELF_HWCAP` 鐨?`COMPAT_HWCAP_ISA_V` 浣嶃€?

- prctl(PR_RISCV_V_SET_CONTROL, unsigned long arg)

    璁剧疆璋冪敤绾跨▼鐨?Vector 鍚敤鐘舵€侊紝鍏朵腑鎺у埗鍙傛暟鐢变袱涓?2 浣嶇殑鍚敤鐘舵€佸拰涓€涓敤浜庣户鎵挎ā寮忕殑浣嶇粍鎴愩€傝皟鐢ㄨ繘绋嬬殑鍏朵粬绾跨▼涓嶅彈褰卞搷銆?

    鍚敤鐘舵€佹槸涓€涓笁鎬佸€硷紝鍚勫崰鐢ㄦ帶鍒跺弬鏁颁腑鐨?2 浣嶇┖闂达細

    - `PR_RISCV_V_VSTATE_CTRL_DEFAULT`锛氬湪 execve() 鏃朵娇鐢ㄧ郴缁熻寖鍥寸殑榛樿鍚敤鐘舵€併€傜郴缁熻寖鍥寸殑榛樿璁剧疆鍙互閫氳繃 sysctl 鎺ュ彛鎺у埗锛堣涓嬫枃 sysctl 灏忚妭锛夈€?

    - `PR_RISCV_V_VSTATE_CTRL_ON`锛氬厑璁歌绾跨▼杩愯 Vector銆?

    - `PR_RISCV_V_VSTATE_CTRL_OFF`锛氱姝?Vector銆傚湪姝ゆ儏鍐典笅鎵ц Vector 鎸囦护浼氳Е鍙戦櫡闃卞苟瀵艰嚧绾跨▼缁堟銆?

    arg锛氭帶鍒跺弬鏁版槸涓€涓敱 3 閮ㄥ垎缁勬垚鐨?5 浣嶅€硷紝鍒嗗埆閫氳繃 3 涓帺鐮佽闂€?

    杩?3 涓帺鐮?PR_RISCV_V_VSTATE_CTRL_CUR_MASK銆丳R_RISCV_V_VSTATE_CTRL_NEXT_MASK 鍜?PR_RISCV_V_VSTATE_CTRL_INHERIT 鍒嗗埆琛ㄧず bit[1:0]銆乥it[3:2] 鍜?bit[4]銆俠it[1:0] 瀵瑰簲璋冪敤绾跨▼鐨勫惎鐢ㄧ姸鎬侊紝bit[3:2] 鐨勮缃彂鐢熷湪涓嬩竴娆?execve() 鏃躲€俠it[4] 瀹氫箟 bit[3:2] 涓缃殑缁ф壙妯″紡銆?

        - `PR_RISCV_V_VSTATE_CTRL_CUR_MASK`锛歜it[1:0]锛氬搴旇皟鐢ㄧ嚎绋嬬殑 Vector 鍚敤鐘舵€併€備竴鏃﹀惎鐢紝璋冪敤绾跨▼鏃犳硶鍏抽棴 Vector銆傚鏋滆鎺╃爜涓殑鍊间负 PR_RISCV_V_VSTATE_CTRL_OFF锛屼絾褰撳墠鍚敤鐘舵€佷笉鏄?off锛屽垯 prctl() 璋冪敤灏嗕互 EPERM 澶辫触銆傚湪姝ゅ璁剧疆 PR_RISCV_V_VSTATE_CTRL_DEFAULT 娌℃湁鏁堟灉锛屽彧鏄皢鍘熷鍚敤鐘舵€佽鍥炪€?

        - `PR_RISCV_V_VSTATE_CTRL_NEXT_MASK`锛歜it[3:2]锛氬搴旇皟鐢ㄧ嚎绋嬪湪涓嬩竴娆?execve() 绯荤粺璋冪敤鏃剁殑 Vector 鍚敤璁剧疆銆傚鏋滃湪姝ゆ帺鐮佷腑浣跨敤 PR_RISCV_V_VSTATE_CTRL_DEFAULT锛屽垯鍚敤鐘舵€佸皢鍦?execve() 鍙戠敓鏃剁敱绯荤粺鑼冨洿鐨勫惎鐢ㄧ姸鎬佸喅瀹氥€?

        - `PR_RISCV_V_VSTATE_CTRL_INHERIT`锛歜it[4]锛歅R_RISCV_V_VSTATE_CTRL_NEXT_MASK 涓缃殑缁ф壙妯″紡銆傚鏋滆缃簡璇ヤ綅锛屽垯鍚庣画鐨?execve() 涓嶄細娓呴櫎 PR_RISCV_V_VSTATE_CTRL_NEXT_MASK 鍜?PR_RISCV_V_VSTATE_CTRL_INHERIT 涓殑璁剧疆銆傝璁剧疆璺ㄧ郴缁熻寖鍥撮粯璁ゅ€肩殑鏇存敼鑰屾寔缁瓨鍦ㄣ€?

    杩斿洖鍊硷細
        - 鎴愬姛鏃惰繑鍥?0锛?
        - EINVAL锛氫笉鏀寔 Vector锛屾垨褰撳墠/涓嬩竴涓帺鐮佺殑鍚敤鐘舵€佹棤鏁堬紱
        - EPERM锛氬湪 PR_RISCV_V_VSTATE_CTRL_CUR_MASK 涓叧闂?Vector锛岃€岃皟鐢ㄧ嚎绋嬬殑 Vector 宸插惎鐢ㄣ€?

    鎴愬姛鏃讹細
        - 瀵?PR_RISCV_V_VSTATE_CTRL_CUR_MASK 鐨勬湁鏁堣缃細绔嬪嵆鐢熸晥銆侾R_RISCV_V_VSTATE_CTRL_NEXT_MASK 涓寚瀹氱殑鍚敤鐘舵€佸彂鐢熷湪涓嬩竴娆?execve() 璋冪敤鏃讹紝鎴栬€呭鏋滆缃簡 PR_RISCV_V_VSTATE_CTRL_INHERIT 浣嶏紝鍒欏彂鐢熷湪鎵€鏈夊悗缁殑 execve() 璋冪敤鏃躲€?
        - 姣忔鎴愬姛鐨勮皟鐢ㄩ兘浼氳鐩栬皟鐢ㄧ嚎绋嬩箣鍓嶇殑涓€娆¤缃€?

- prctl(PR_RISCV_V_GET_CONTROL)

    鑾峰彇璋冪敤绾跨▼鐩稿悓鐨?Vector 鍚敤鐘舵€併€備笅涓€娆?execve() 璋冪敤鐨勮缃拰缁ф壙浣嶉兘浼氳 OR 鍦ㄤ竴璧枫€?

    娉ㄦ剰锛孍LF 绋嬪簭鑳藉閫氳繃璇诲彇杈呭姪鍚戦噺涓?`ELF_HWCAP` 鐨?`COMPAT_HWCAP_ISA_V` 浣嶆潵鑾峰彇鑷韩 V 鐨勫彲鐢ㄦ€с€?

    杩斿洖鍊硷細
        - 鎴愬姛鏃惰繑鍥為潪璐熷€硷紱
        - EINVAL锛氫笉鏀寔 Vector銆?

### 2. 绯荤粺杩愯鏃堕厤缃紙sysctl锛?


涓轰簡缂撹В淇″彿鏍堟墿灞曞 ABI 鐨勫奖鍝嶏紝鎻愪緵浜嗕竴涓瓥鐣ユ満鍒讹紝渚涚鐞嗗憳銆佸彂琛岀増缁存姢鑰呭拰寮€鍙戣€呬互 sysctl 鏃嬮挳鐨勫舰寮忔帶鍒剁敤鎴风┖闂磋繘绋嬮粯璁ょ殑 Vector 鍚敤鐘舵€侊細

- /proc/sys/abi/riscv_v_default_allow

    鍚戣鏂囦欢鍐欏叆 0 鎴?1 鐨勬枃鏈〃绀猴紝鍙缃柊鍚姩鐨勭敤鎴风┖闂寸▼搴忕殑榛樿绯荤粺鍚敤鐘舵€併€傛湁鏁堝€间负锛?

    - 0锛氶粯璁や笉鍏佽鏂拌繘绋嬫墽琛?Vector 浠ｇ爜銆?
    - 1锛氶粯璁ゅ厑璁告柊杩涚▼鎵ц Vector 浠ｇ爜銆?

    璇诲彇璇ユ枃浠朵細杩斿洖褰撳墠鐨勭郴缁熼粯璁ゅ惎鐢ㄧ姸鎬併€?

    鍦ㄦ瘡娆?execve() 璋冪敤鏃讹紝鏂拌繘绋嬬殑鍚敤鐘舵€佽璁句负绯荤粺榛樿鍊硷紝闄ら潪锛?

      - 璋冪敤杩涚▼璁剧疆浜?PR_RISCV_V_VSTATE_CTRL_INHERIT锛屼笖 PR_RISCV_V_VSTATE_CTRL_NEXT_MASK 涓殑璁剧疆涓嶆槸 PR_RISCV_V_VSTATE_CTRL_DEFAULT銆傛垨鑰咃紝

      - PR_RISCV_V_VSTATE_CTRL_NEXT_MASK 涓殑璁剧疆涓嶆槸 PR_RISCV_V_VSTATE_CTRL_DEFAULT銆?

    淇敼绯荤粺榛樿鍚敤鐘舵€佷笉浼氬奖鍝嶄换浣曟湭鍙戣捣 execve() 璋冪敤鐨勭幇鏈夎繘绋嬫垨绾跨▼鐨勫惎鐢ㄧ姸鎬併€?

### 3. 绯荤粺璋冪敤闂寸殑鍚戦噺瀵勫瓨鍣ㄧ姸鎬?


姝ｅ V 鎵╁睍鐨?1.0 鐗堟湰 [^1^] 鎵€鎸囧嚭鐨勶紝鍚戦噺瀵勫瓨鍣ㄤ細琚郴缁熻皟鐢ㄧ牬鍧忋€?

1: https://github.com/riscv/riscv-v-spec/blob/master/calling-convention.adoc
