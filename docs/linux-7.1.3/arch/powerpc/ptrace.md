## Ptrace


GDB 鎵撶畻鏀寔 BookE 澶勭悊鍣ㄧ殑浠ヤ笅纭欢璋冭瘯鐗规€э細

4 涓‖浠舵柇鐐癸紙IAC锛?
2 涓‖浠惰瀵熺偣锛堣銆佸啓鍜岃-鍐欙級锛圖AC锛?
2 涓敤浜庣‖浠惰瀵熺偣鐨勫€兼潯浠讹紙DVC锛?

涓烘锛屾垜浠渶瑕佹墿灞?ptrace锛屼互渚?GDB 鑳藉鏌ヨ骞惰缃繖浜涜祫婧愩€傜敱浜庢垜浠鍦ㄦ墿灞曪紝鎴戜滑璇曞浘鍒涘缓涓€涓彲鎵╁睍銆佸苟涓斿悓鏃惰鐩?BookE 鍜屾湇鍔″櫒澶勭悊鍣ㄧ殑鎺ュ彛锛岃繖鏍?GDB 灏变笉蹇呭瀹冧滑鍚勮嚜鍋氱壒娈婂鐞嗐€傛垜浠坊鍔犱簡浠ヤ笅 3 涓柊鐨?ptrace 璇锋眰銆?

## 1. PPC_PTRACE_GETHWDBGINFO


渚?GDB 鏌ヨ浠ュ彂鐜扮‖浠惰皟璇曠壒鎬с€傝繖閲岃杩斿洖鐨勪富瑕佷俊鎭槸閽堝纭欢瑙傚療鐐圭殑鏈€灏忓榻愩€侭ookE 澶勭悊鍣ㄥ湪姝ゆ病鏈夐檺鍒讹紝浣嗘湇鍔″櫒澶勭悊鍣ㄥ纭欢瑙傚療鐐规湁 8 瀛楄妭瀵归綈鐨勯檺鍒躲€傛垜浠笇鏈涢伩鍏嶅湪 GDB 涓熀浜庡畠鍦?AUXV 涓湅鍒扮殑鍐呭娣诲姞鐗规畩鎯呭喌銆?

鏃㈢劧鍦ㄥ仛杩欎欢浜嬶紝鎴戜滑杩樻坊鍔犱簡鍐呮牳鍙互杩斿洖缁?GDB 鐨勫叾浠栨湁鐢ㄤ俊鎭細璇ユ煡璇㈠皢杩斿洖纭欢鏂偣鐨勬暟閲忋€佺‖浠惰瀵熺偣鐨勬暟閲忥紝浠ュ強瀹冩槸鍚︽敮鎸佷竴娈靛湴鍧€鑼冨洿鍜屼竴涓潯浠躲€?
```

  struct ppc_debug_info {
       unit32_t version;
       unit32_t num_instruction_bps;
       unit32_t num_data_bps;
       unit32_t num_condition_regs;
       unit32_t data_bp_alignment;
       unit32_t sizeof_condition; /* size of the DVC register */
       uint64_t features; /* bitmask of the individual flags */
  };

```
```

  #define PPC_DEBUG_FEATURE_INSN_BP_RANGE		0x1
  #define PPC_DEBUG_FEATURE_INSN_BP_MASK		0x2
  #define PPC_DEBUG_FEATURE_DATA_BP_RANGE		0x4
  #define PPC_DEBUG_FEATURE_DATA_BP_MASK		0x8
  #define PPC_DEBUG_FEATURE_DATA_BP_DAWR		0x10
  #define PPC_DEBUG_FEATURE_DATA_BP_ARCH_31		0x20

```
2. PPC_PTRACE_SETHWDEBUG

```

  struct ppc_hw_breakpoint {
        uint32_t version;
  #define PPC_BREAKPOINT_TRIGGER_EXECUTE  0x1
  #define PPC_BREAKPOINT_TRIGGER_READ     0x2
 #define PPC_BREAKPOINT_TRIGGER_WRITE    0x4
        uint32_t trigger_type;       /* only some combinations allowed */
  #define PPC_BREAKPOINT_MODE_EXACT               0x0
  #define PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE     0x1
  #define PPC_BREAKPOINT_MODE_RANGE_EXCLUSIVE     0x2
  #define PPC_BREAKPOINT_MODE_MASK                0x3
        uint32_t addr_mode;          /* address match mode */

  #define PPC_BREAKPOINT_CONDITION_MODE   0x3
  #define PPC_BREAKPOINT_CONDITION_NONE   0x0
  #define PPC_BREAKPOINT_CONDITION_AND    0x1
  #define PPC_BREAKPOINT_CONDITION_EXACT  0x1	/* different name for the same thing as above */
  #define PPC_BREAKPOINT_CONDITION_OR     0x2
  #define PPC_BREAKPOINT_CONDITION_AND_OR 0x3
  #define PPC_BREAKPOINT_CONDITION_BE_ALL 0x00ff0000	/* byte enable bits */
  #define PPC_BREAKPOINT_CONDITION_BE(n)  (1<<((n)+16))
        uint32_t condition_mode;     /* break/watchpoint condition flags */

        uint64_t addr;
        uint64_t addr2;
        uint64_t condition_value;
  };

```
涓€涓姹傛寚瀹氫竴涓簨浠讹紝鑰屼笉涓€瀹氬彧鏄璁剧疆鐨勪竴涓瘎瀛樺櫒銆備緥濡傦紝濡傛灉璇锋眰鏄竴涓甫鏉′欢鐨勮瀵熺偣锛孌AC 鍜?DVC 瀵勫瓨鍣ㄩ兘灏嗗湪鍚屼竴涓姹備腑琚缃€?

閫氳繃杩欑鏂瑰紡锛孏DB 鍙互璇锋眰 BookE 鏀寔鐨勬墍鏈夌被鍨嬬殑纭欢鏂偣鍜岃瀵熺偣銆傛湇鍔″櫒澶勭悊鍣ㄤ腑鍙敤鐨?COMEFROM 鏂偣涓嶅湪鑰冭檻涔嬪垪锛屼絾杩欒秴鍑轰簡鏈伐浣滅殑鑼冨洿銆?

ptrace 灏嗚繑鍥炰竴涓敮涓€鏍囪瘑鍒氬垰鍒涘缓鐨勬柇鐐规垨瑙傚療鐐圭殑鏁存暟锛堝彞鏌勶級銆傝鏁存暟灏嗗湪 PPC_PTRACE_DELHWDEBUG 璇锋眰涓敤浜庤姹傚垹闄ゅ畠銆傚鏋滄墍璇锋眰鐨勬柇鐐规棤娉曞湪瀵勫瓨鍣ㄤ笂鍒嗛厤锛屽垯杩斿洖 -ENOSPC銆?

涓嬮潰鏄娇鐢ㄨ缁撴瀯鐨勪竴浜涚ず渚嬶細

```

    p.version         = PPC_DEBUG_CURRENT_VERSION;
    p.trigger_type    = PPC_BREAKPOINT_TRIGGER_EXECUTE;
    p.addr_mode       = PPC_BREAKPOINT_MODE_EXACT;
    p.condition_mode  = PPC_BREAKPOINT_CONDITION_NONE;
    p.addr            = (uint64_t) address;
    p.addr2           = 0;
    p.condition_value = 0;

```
```

    p.version         = PPC_DEBUG_CURRENT_VERSION;
    p.trigger_type    = PPC_BREAKPOINT_TRIGGER_READ;
    p.addr_mode       = PPC_BREAKPOINT_MODE_EXACT;
    p.condition_mode  = PPC_BREAKPOINT_CONDITION_NONE;
    p.addr            = (uint64_t) address;
    p.addr2           = 0;
    p.condition_value = 0;

```
```

    p.version         = PPC_DEBUG_CURRENT_VERSION;
    p.trigger_type    = PPC_BREAKPOINT_TRIGGER_READ;
    p.addr_mode       = PPC_BREAKPOINT_MODE_EXACT;
    p.condition_mode  = PPC_BREAKPOINT_CONDITION_AND | PPC_BREAKPOINT_CONDITION_BE_ALL;
    p.addr            = (uint64_t) address;
    p.addr2           = 0;
    p.condition_value = (uint64_t) condition;

```
```

    p.version         = PPC_DEBUG_CURRENT_VERSION;
    p.trigger_type    = PPC_BREAKPOINT_TRIGGER_EXECUTE;
    p.addr_mode       = PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE;
    p.condition_mode  = PPC_BREAKPOINT_CONDITION_NONE;
    p.addr            = (uint64_t) begin_range;
    p.addr2           = (uint64_t) end_range;
    p.condition_value = 0;

```
```

    p.version         = 1;
    p.trigger_type    = PPC_BREAKPOINT_TRIGGER_RW;
    p.addr_mode       = PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE;
    or
    p.addr_mode       = PPC_BREAKPOINT_MODE_EXACT;

    p.condition_mode  = PPC_BREAKPOINT_CONDITION_NONE;
    p.addr            = (uint64_t) begin_range;
    /* For PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE addr2 needs to be specified, where
     * addr2 - addr <= 8 Bytes.
     */
    p.addr2           = (uint64_t) end_range;
    p.condition_value = 0;

```
3. PPC_PTRACE_DELHWDEBUG


鎺ュ彈涓€涓爣璇嗙幇鏈夋柇鐐规垨瑙傚療鐐圭殑鏁存暟锛堝嵆 PTRACE_SETHWDEBUG 杩斿洖鐨勫€硷級锛屽苟鍒犻櫎鐩稿簲鐨勬柇鐐规垨瑙傚療鐐广€?
