## ALSA 涓婄殑 OSS 闊冲簭鍣ㄦā鎷?

Copyright (c) 1998,1999 by Takashi Iwai

ver.0.1.8; 1999骞?1鏈?6鏃?
## 鎻忚堪


鏈洰褰曞寘鍚?ALSA 涓婄殑 OSS 闊冲簭鍣ㄦā鎷熼┍鍔ㄣ€傛敞鎰忥紝鏈▼搴忎粛澶勪簬寮€鍙戠姸鎬併€?
瀹冪殑浣滅敤鈥斺€旀彁渚?OSS 闊冲簭鍣ㄧ殑妯℃嫙锛岄€氳繃 `/dev/sequencer` 涓?`/dev/music` 璁惧璁块棶銆傚彧瑕佸噯澶囧ソ鐩稿簲鐨?ALSA 闊冲簭鍣紝澶у鏁颁娇鐢?OSS 鐨勫簲鐢ㄧ▼搴忛兘鑳借繍琛屻€?
鏈┍鍔ㄦā鎷熶簡浠ヤ笅鐗规€э細

- 鏅€氶煶搴忓櫒涓?MIDI 浜嬩欢锛?
    瀹冧滑琚浆鎹负 ALSA 闊冲簭鍣ㄤ簨浠讹紝骞跺彂閫佸埌鐩稿簲鐨勭鍙ｃ€?
- 瀹氭椂鍣ㄤ簨浠讹細

    瀹氭椂鍣ㄤ笉鑳介€氳繃 ioctl 閫夋嫨銆傛帶鍒堕€熺巼鍥哄畾涓?100锛屼笌 HZ 鏃犲叧銆備篃灏辨槸璇达紝鍗充娇鍦?Alpha 绯荤粺涓婏紝涓€涓?tick 涔熷缁堟槸 1/100 绉掋€傚熀鍑嗛€熺巼鍜岄€熷害锛坱empo锛夊彲浠ュ湪 `/dev/music` 涓洿鏀广€?
- 闊宠壊锛坧atch锛夊姞杞斤細

    鐢变簬闊宠壊鍔犺浇鏄€氳繃鍥炶皟鍒板悎鎴愬櫒椹卞姩鏉ュ疄鐜扮殑锛屾槸鍚︽敮鎸佸畠瀹屽叏鍙栧喅浜庡悎鎴愬櫒椹卞姩銆?
- I/O 鎺у埗锛?
    澶у鏁版帶鍒堕兘琚帴鍙椼€傛湁浜涙帶鍒朵緷璧栦簬鍚堟垚鍣ㄩ┍鍔紝灏卞儚鍦ㄥ師濮嬬殑 OSS 涓篃涓€鏍枫€?
姝ゅ锛屼綘杩樿兘鍙戠幇浠ヤ笅楂樼骇鐗规€э細

- 鏇村ソ鐨勯槦鍒楁満鍒讹細

    浜嬩欢鍦ㄥ鐞嗕箣鍓嶄細琚帓鍏ラ槦鍒椼€?
- 澶氬簲鐢ㄧ▼搴忥細

    浣犲彲浠ュ悓鏃惰繍琛屼袱涓垨鏇村搴旂敤绋嬪簭锛堝嵆渚挎槸 OSS 闊冲簭鍣級锛?    涓嶈繃锛屾瘡涓?MIDI 璁惧鏄嫭鍗犵殑鈥斺€斾篃灏辨槸璇达紝濡傛灉鏌愪釜 MIDI 璁惧宸茶鏌愬簲鐢ㄧ▼搴忔墦寮€涓€娆★紝鍏朵粬搴旂敤绋嬪簭灏辨棤娉曞啀浣跨敤瀹冦€傚悎鎴愬櫒璁惧娌℃湁杩欑闄愬埗銆?
- 瀹炴椂浜嬩欢澶勭悊锛?
    浜嬩欢鍙互鍦ㄤ笉浣跨敤瓒婄晫 ioctl 鐨勬儏鍐典笅瀹炴椂澶勭悊銆傝鍒囨崲鍒板疄鏃舵ā寮忥紝鍙戦€?ABSTIME 0 浜嬩欢銆傞殢鍚庣殑浜嬩欢灏嗗湪瀹炴椂涓嬪鐞嗚€屼笉鍏ラ槦銆傝鍏抽棴瀹炴椂妯″紡锛屽彂閫?RELTIME 0 浜嬩欢銆?
- `/proc` 鎺ュ彛锛?
    搴旂敤绋嬪簭鍜岃澶囩殑鐘舵€佸彲浠ラ殢鏃堕€氳繃 `/proc/asound/seq/oss` 鏌ョ湅銆傚湪鍚庣画鐗堟湰涓紝閰嶇疆涔熷皢閫氳繃 `/proc` 鎺ュ彛鏇存敼銆?
## 瀹夎


杩愯 configure 鑴氭湰鏃跺悓鏃跺甫涓婇煶搴忓櫒鏀寔锛坄--with-sequencer=yes`锛夊拰 OSS 妯℃嫙锛坄--with-oss=yes`锛夐€夐」銆傚皢鍒涘缓涓€涓?`snd-seq-oss.o` 妯″潡銆傚鏋滀綘鐨勫０鍗＄殑鍚堟垚鍣ㄦā鍧楁敮鎸?OSS 妯℃嫙锛堝埌鐩墠涓烘鍙湁 Emu8000 椹卞姩锛夛紝璇ユā鍧椾細琚嚜鍔ㄥ姞杞姐€?鍚﹀垯锛屼綘闇€瑕佹墜鍔ㄥ姞杞借妯″潡銆?
涓€寮€濮嬶紝鏈ā鍧椾細鎺㈡祴鎵€鏈夊凡缁忚繛鎺ュ埌闊冲簭鍣ㄧ殑 MIDI 绔彛銆傛鍚庯紝绔彛鐨勫垱寤哄拰鍒犻櫎鐢?ALSA 闊冲簭鍣ㄧ殑閫氬憡鏈哄埗鐩戣銆?
鍙敤鐨勫悎鎴愬櫒鍜?MIDI 璁惧鍙互鍦?proc 鎺ュ彛涓壘鍒般€傝繍琛?`cat /proc/asound/seq/oss`锛屾鏌ヨ澶囥€備緥濡傦紝濡傛灉浣犱娇鐢?AWE64 澹板崱锛屼綘浼氱湅鍒板涓嬪唴瀹癸細
```

    OSS sequencer emulation version 0.1.8
    ALSA client number 63
    ALSA receiver port 0

    Number of applications: 0

    Number of synth devices: 1
    synth 0: [EMU8000]
      type 0x1 : subtype 0x20 : voices 32
      capabilities : ioctl enabled / load_patch enabled

    Number of MIDI devices: 3
    midi 0: [Emu8000 Port-0] ALSA port 65:0
      capability write / opened none

    midi 1: [Emu8000 Port-1] ALSA port 65:1
      capability write / opened none

    midi 2: [0: MPU-401 (UART)] ALSA port 64:0
      capability read/write / opened none

```
娉ㄦ剰锛岃澶囩紪鍙峰彲鑳戒笉鍚屼簬 `/proc/asound/oss-devices` 鐨勪俊鎭垨鍘熷 OSS 椹卞姩鐨勪俊鎭€?璇蜂娇鐢?`/proc/asound/seq/oss` 涓垪鍑虹殑璁惧缂栧彿鏉ラ€氳繃 OSS 闊冲簭鍣ㄦā鎷熸挱鏀俱€?
## 浣跨敤鍚堟垚鍣ㄨ澶?

杩愯浣犲枩娆㈢殑绋嬪簭銆傛垜娴嬭瘯杩?playmidi-2.4銆乤wemidi-0.4.3銆乬mod-3.1 鍜?xmp-1.1.5銆備綘涔熷彲浠ュ儚 sfxload 閭ｆ牱閫氳繃 `/dev/sequencer` 鍔犺浇鏍锋湰銆?
濡傛灉搴曞眰椹卞姩鏀寔瀵瑰悎鎴愬櫒璁惧鐨勫璺闂紙濡?Emu8000 椹卞姩锛夛紝鍒欏厑璁镐袱涓垨鏇村搴旂敤绋嬪簭鍚屾椂杩愯銆?
## 浣跨敤 MIDI 璁惧


鍒扮洰鍓嶄负姝紝鍙祴璇曚簡 MIDI 杈撳嚭銆侻IDI 杈撳叆瀹屽叏娌℃湁妫€鏌ヨ繃锛屼絾鏈夊笇鏈涘彲浠ュ伐浣溿€傝浣跨敤 `/proc/asound/seq/oss` 涓垪鍑虹殑璁惧缂栧彿銆?娉ㄦ剰锛岃繖浜涚紪鍙峰ぇ澶氫笉鍚屼簬 `/proc/asound/oss-devices` 涓殑鍒楄〃銆?
## 妯″潡閫夐」


鍙娇鐢ㄤ互涓嬫ā鍧楅€夐」锛?
maxqlen
  鎸囧畾鏈€澶ц/鍐欓槦鍒楅暱搴︺€傝闃熷垪涓?OSS 闊冲簭鍣ㄧ鏈夛紝鍥犳鐙珛浜?ALSA 闊冲簭鍣ㄧ殑闃熷垪闀垮害銆傞粯璁ゅ€间负 1024銆?
seq_oss_debug
  鎸囧畾璋冭瘯绾у埆锛屾帴鍙楅浂锛?鏃犺皟璇曟秷鎭級鎴栨鏁存暟銆傞粯璁ゅ€间负 0銆?
## 闃熷垪鏈哄埗


OSS 闊冲簭鍣ㄦā鎷熶娇鐢ㄤ竴涓?ALSA 浼樺厛闃熷垪銆?鏉ヨ嚜 `/dev/sequencer` 鐨勪簨浠惰澶勭悊锛屽苟鏀惧叆鐢辨ā鍧楅€夐」鎸囧畾鐨勯槦鍒椾腑銆?
鏉ヨ嚜 `/dev/sequencer` 鐨勬墍鏈変簨浠跺湪寮€澶村氨琚В鏋愩€傚畾鏃朵簨浠朵篃鍦ㄦ鏃惰В鏋愶紝鍥犳浜嬩欢鍙互瀹炴椂澶勭悊銆傚彂閫?ABSTIME 0 浜嬩欢灏嗘搷浣滄ā寮忓垏鎹㈠埌瀹炴椂妯″紡锛屽彂閫?RELTIME 0 浜嬩欢灏嗗叾鍏抽棴銆?鍦ㄥ疄鏃舵ā寮忎笅锛屾墍鏈変簨浠堕兘绔嬪嵆鍒嗗彂銆?
鎺掗槦鐨勪簨浠剁敱 ALSA 闊冲簭鍣ㄥ垎鍙戝櫒鍦ㄩ瀹氭椂闂翠箣鍚庡垎鍙戝埌鐩稿簲鐨?ALSA 闊冲簭鍣ㄧ鍙ｃ€?
濡傛灉鍐欓槦鍒楀凡婊★紝鍦ㄩ樆濉炴ā寮忎笅搴旂敤绋嬪簭浼氫紤鐪狅紝鐩村埌绌哄嚭涓€瀹氶噺锛堥粯璁ゆ槸涓€鍗婏級銆傚鍐欏叆瀹氭椂鐨勫悓姝ヤ篃瀹炵幇浜嗐€?
鏉ヨ嚜 MIDI 璁惧鐨勮緭鍏ユ垨鍥炴樉浜嬩欢琚瓨鍌ㄥ湪璇?FIFO 闃熷垪涓€傚鏋滃簲鐢ㄧ▼搴忎互闃诲妯″紡璇诲彇 `/dev/sequencer`锛岃杩涚▼灏嗚鍞ら啋銆?
## 涓庡悎鎴愬櫒璁惧鐨勬帴鍙?

### 娉ㄥ唽


瑕佹敞鍐屼竴涓?OSS 鍚堟垚鍣ㄨ澶囷紝浣跨敤 snd_seq_oss_synth_register() 鍑芥暟锛?```

  int snd_seq_oss_synth_register(char *name, int type, int subtype, int nvoices,
          snd_seq_oss_callback_t *oper, void *private_data)

```
鍙傛暟 `name`銆乣type`銆乣subtype` 鍜?`nvoices` 鐢ㄤ簬鏋勯€犱緵 ioctl 浣跨敤鐨勭浉搴?synth_info 缁撴瀯浣撱€傝繑鍥炲€兼槸璇ヨ澶囩殑绱㈠紩鍙枫€傚繀椤昏浣忚繖涓储寮曚互渚挎敞閿€銆傚鏋滄敞鍐屽け璐ワ紝灏嗚繑鍥?-errno銆?
瑕侀噴鏀捐璁惧锛岃皟鐢?snd_seq_oss_synth_unregister() 鍑芥暟锛?```

  int snd_seq_oss_synth_unregister(int index)

```
鍏朵腑 `index` 鏄敞鍐屽嚱鏁拌繑鍥炵殑绱㈠紩鍙枫€?
### 鍥炶皟


OSS 鍚堟垚鍣ㄨ澶囧叿澶囨牱鏈笅杞藉拰 ioctl锛堝鏍锋湰閲嶇疆锛夌瓑鑳藉姏銆傚湪 OSS 妯℃嫙涓紝杩欎簺鐗规畩鐗规€ч€氳繃鍥炶皟瀹炵幇銆傛敞鍐屽弬鏁?oper 鐢ㄤ簬鎸囧畾杩欎簺鍥炶皟銆傚繀椤诲畾涔変互涓嬪洖璋冨嚱鏁帮細
```

  snd_seq_oss_callback_t:
   int (*open)(snd_seq_oss_arg_t *p, void *closure);
   int (*close)(snd_seq_oss_arg_t *p);
   int (*ioctl)(snd_seq_oss_arg_t *p, unsigned int cmd, unsigned long arg);
   int (*load_patch)(snd_seq_oss_arg_t *p, int format, const char *buf, int offs, int count);
   int (*reset)(snd_seq_oss_arg_t *p);

```
闄や簡 `open` 鍜?`close` 鍥炶皟澶栵紝鍏朵綑鍏佽涓?NULL銆?
姣忎釜鍥炶皟鍑芥暟閮戒互 `snd_seq_oss_arg_t` 绫诲瀷鐨勫弬鏁颁綔涓虹涓€涓弬鏁般€?```

  struct snd_seq_oss_arg_t {
      int app_index;
      int file_mode;
      int seq_mode;
      snd_seq_addr_t addr;
      void *private_data;
      int event_passing;
  };

```
鍓嶄笁涓瓧娈?`app_index`銆乣file_mode` 鍜?`seq_mode` 鐢?OSS 闊冲簭鍣ㄥ垵濮嬪寲銆俙app_index` 鏄簲鐢ㄧ▼搴忕储寮曪紝瀵规瘡涓墦寮€ OSS 闊冲簭鍣ㄧ殑搴旂敤绋嬪簭閮芥槸鍞竴鐨勩€俙file_mode` 鏄寚绀烘枃浠舵搷浣滄ā寮忕殑浣嶆爣蹇椼€傚叾鍚箟瑙?`seq_oss.h`銆俙seq_mode` 鏄煶搴忓櫒鎿嶄綔妯″紡銆傚湪褰撳墠鐗堟湰涓紝鍙娇鐢?`SND_OSSSEQ_MODE_SYNTH`銆?
鎺ヤ笅鏉ョ殑涓や釜瀛楁 `addr` 鍜?`private_data` 蹇呴』鐢卞悎鎴愬櫒椹卞姩鍦?open 鍥炶皟涓～鍐欍€俙addr` 鍖呭惈鍒嗛厤缁欒璁惧鐨?ALSA 闊冲簭鍣ㄧ鍙ｅ湴鍧€銆傚鏋滈┍鍔ㄤ负 `private_data` 鍒嗛厤浜嗗唴瀛橈紝鍒欏繀椤诲湪 close 鍥炶皟涓嚜琛岄噴鏀俱€?
鏈€鍚庝竴涓瓧娈?`event_passing` 鎸囩ず濡備綍缈昏瘧 note-on/off 浜嬩欢銆傚湪 `PROCESS_EVENTS` 妯″紡涓嬶紝闊崇 255 琚涓哄姏搴﹀彉鍖栵紝鎸夐敭鍘嬪姏浜嬩欢琚紶閫掑埌绔彛銆傚湪 `PASS_EVENTS` 妯″紡涓嬶紝鎵€鏈?note on/off 浜嬩欢閮藉師鏍蜂紶閫掑埌绔彛鑰屼笉鍔犱慨鏀广€俙PROCESS_KEYPRESS` 妯″紡妫€鏌ュぇ浜?128 鐨勯煶绗︼紝骞跺皢鍏惰涓烘寜閿帇鍔涗簨浠讹紙涓昏鐢ㄤ簬 Emu8000 椹卞姩锛夈€?
### Open 鍥炶皟


姣忓綋鏈夊簲鐢ㄧ▼搴忛€氳繃 OSS 闊冲簭鍣ㄦ墦寮€璇ヨ澶囨椂锛屽氨浼氳皟鐢?`open`銆傚畠涓嶈兘涓?NULL銆傞€氬父锛宱pen 鍥炶皟鎵ц浠ヤ笅杩囩▼锛?
#. 鍒嗛厤绉佹湁鏁版嵁璁板綍銆?#. 鍒涘缓涓€涓?ALSA 闊冲簭鍣ㄧ鍙ｃ€?#. 鍦?`arg->addr` 涓婅缃柊绔彛鍦板潃銆?#. 鍦?`arg->private_data` 涓婅缃鏈夋暟鎹褰曟寚閽堛€?
娉ㄦ剰锛岃鍚堟垚鍣ㄧ鍙ｇ殑 port_info 涓殑绫诲瀷浣嶆爣蹇椾笉寰楀寘鍚?`TYPE_MIDI_GENERIC` 浣嶃€傜浉鍙嶏紝搴斿綋浣跨敤 `TYPE_SPECIFIC`銆傚悓鏍凤紝涔熶笉搴斿寘鍚?`CAP_SUBSCRIPTION` 浣嶃€傝繖鏄负浜嗘妸瀹冧笌鍏朵粬鏅€?MIDI 璁惧鍖哄垎寮€銆傚鏋?open 杩囩▼鎴愬姛锛岃繑鍥為浂锛涘惁鍒欒繑鍥?-errno銆?
### Ioctl 鍥炶皟


褰撻煶搴忓櫒鏀跺埌璁惧鐗瑰畾鐨?ioctl 鏃讹紝浼氳皟鐢?`ioctl` 鍥炶皟銆傝鍥炶皟搴斿綋澶勭悊浠ヤ笅涓や釜 ioctl锛?
IOCTL_SEQ_RESET_SAMPLES
    閲嶇疆鍐呭瓨涓殑鎵€鏈夋牱鏈€斺€旇繑鍥?0

IOCTL_SYNTH_MEMAVL
    杩斿洖鍙敤鍐呭瓨澶у皬

FM_4OP_ENABLE
    閫氬父鍙互蹇界暐

鍏朵粬 ioctl 鍦ㄩ煶搴忓櫒鍐呴儴澶勭悊锛屼笉浼氫紶閫掔粰搴曞眰椹卞姩銆?
### Load_Patch 鍥炶皟


`load_patch` 鍥炶皟鐢ㄤ簬鏍锋湰涓嬭浇銆傝鍥炶皟蹇呴』璇诲彇鐢ㄦ埛绌洪棿鐨勬暟鎹苟浼犺緭鍒板悇涓澶囥€傛垚鍔熻繑鍥?0锛屽け璐ヨ繑鍥?-errno銆俧ormat 鍙傛暟鏄?patch_info 璁板綍涓殑 patch 閿€俠uf 鏄瓨鍌?patch_info 璁板綍鐨勭敤鎴风┖闂存寚閽堛€俹ffs 鍙互蹇界暐銆俢ount 鏄鏍锋湰鏁版嵁鐨勬€诲ぇ灏忋€?
### Close 鍥炶皟


褰撳簲鐢ㄧ▼搴忓叧闂璁惧鏃讹紝浼氳皟鐢?`close` 鍥炶皟銆傚鏋滃湪 open 鍥炶皟涓垎閰嶄簡浠讳綍绉佹湁鏁版嵁锛屽繀椤诲湪 close 鍥炶皟涓噴鏀俱€侫LSA 绔彛鐨勫垹闄や篃搴斿綋鍦ㄦ瀹屾垚銆傝鍥炶皟涓嶈兘涓?NULL銆?
### Reset 鍥炶皟


褰撻煶搴忓櫒璁惧琚簲鐢ㄧ▼搴忛噸缃垨鍏抽棴鏃讹紝浼氳皟鐢?`reset` 鍥炶皟銆傝鍥炶皟搴斿綋绔嬪埢鍏抽棴鐩稿叧绔彛涓婄殑澹伴煶锛屽苟鍒濆鍖栫鍙ｇ殑鐘舵€併€傚鏋滆鍥炶皟鏈畾涔夛紝OSS seq 浼氬悜璇ョ鍙ｅ彂閫佷竴涓?`HEARTBEAT` 浜嬩欢銆?
## 浜嬩欢


澶у鏁颁簨浠剁敱闊冲簭鍣ㄥ鐞嗭紝骞惰浆鎹负閫傚綋鐨?ALSA 闊冲簭鍣ㄤ簨浠讹紝浠ヤ究姣忎釜鍚堟垚鍣ㄨ澶囪兘閫氳繃 ALSA 闊冲簭鍣ㄧ鍙ｇ殑 input_event 鍥炶皟鎺ユ敹銆傞┍鍔ㄥ簲褰撳疄鐜颁互涓?ALSA 浜嬩欢锛?
=============	===================
ALSA 浜嬩欢	鍘熷 OSS 浜嬩欢
=============	===================
NOTEON		SEQ_NOTEON, MIDI_NOTEON
NOTE		SEQ_NOTEOFF, MIDI_NOTEOFF
KEYPRESS	MIDI_KEY_PRESSURE
CHANPRESS	SEQ_AFTERTOUCH, MIDI_CHN_PRESSURE
PGMCHANGE	SEQ_PGMCHANGE, MIDI_PGM_CHANGE
PITCHBEND	SEQ_CONTROLLER(CTRL_PITCH_BENDER),
		MIDI_PITCH_BEND
CONTROLLER	MIDI_CTL_CHANGE,
		SEQ_BALANCE (with CTL_PAN)
CONTROL14	SEQ_CONTROLLER
REGPARAM	SEQ_CONTROLLER(CTRL_PITCH_BENDER_RANGE)
SYSEX		SEQ_SYSEX
=============	===================

杩欎簺琛屼负澶у鍙敱 Emu8000 搴曞眰椹卞姩涓檮甯︾殑 MIDI 妯℃嫙椹卞姩瀹炵幇銆傚湪鏈潵鐨勭増鏈腑锛屾湰妯″潡灏嗙嫭绔嬪嚭鏉ャ€?
涓€浜?OSS 浜嬩欢锛坄SEQ_PRIVATE` 鍜?`SEQ_VOLUME` 浜嬩欢锛変綔涓轰簨浠剁被鍨?SND_SEQ_OSS_PRIVATE 浼犻€掋€侽SS 闊冲簭鍣ㄥ師鏍蜂紶閫掕繖浜涗簨浠剁殑 8 瀛楄妭鏁版嵁鍖咃紝涓嶄綔浠讳綍淇敼銆傚簳灞傞┍鍔ㄥ簲褰撴伆褰撳鐞嗚繖浜涗簨浠躲€?
## 涓?MIDI 璁惧鐨勬帴鍙?

鐢变簬 OSS 妯℃嫙浼氶€氳繃鎺ユ敹鏉ヨ嚜 ALSA 闊冲簭鍣ㄧ殑閫氬憡锛岃嚜鍔ㄦ帰娴?ALSA MIDI 闊冲簭鍣ㄧ鍙ｇ殑鍒涘缓鍜屽垹闄わ紝鍥犳 MIDI 璁惧鏃犻渶鍍忓悎鎴愬櫒璁惧閭ｆ牱鏄惧紡娉ㄥ唽銆?涓嶈繃锛屾敞鍐屽埌 ALSA 闊冲簭鍣ㄧ殑 MIDI port_info 蹇呴』鍖呭惈涓€涓粍鍚?`SND_SEQ_GROUP_DEVICE` 鍜屼竴涓兘鍔涗綅 `CAP_READ` 鎴?`CAP_WRITE`銆傚悓鏃讹紝璁㈤槄鑳藉姏 `CAP_SUBS_READ` 鎴?`CAP_SUBS_WRITE` 涔熷繀椤诲畾涔夈€傚鏋滀笉婊¤冻杩欎簺鏉′欢锛岃绔彛涓嶄細浣滀负 OSS 闊冲簭鍣?MIDI 璁惧娉ㄥ唽銆?
缁忕敱 MIDI 璁惧鐨勪簨浠跺湪 OSS 闊冲簭鍣ㄤ腑琚В鏋愶紝骞惰浆鎹负鐩稿簲鐨?ALSA 闊冲簭鍣ㄤ簨浠躲€傛潵鑷?MIDI 闊冲簭鍣ㄧ殑杈撳叆涔熻 OSS 闊冲簭鍣ㄨ浆鎹负 MIDI 瀛楄妭浜嬩欢銆傚畠鐨勫伐浣滄柟寮忎笌 seq_midi 妯″潡姝ｅソ鐩稿弽銆?
## 宸茬煡闂 / TODO


- 閫氳繃 ALSA instrument 灞傜殑闊宠壊鍔犺浇灏氭湭瀹炵幇銆?