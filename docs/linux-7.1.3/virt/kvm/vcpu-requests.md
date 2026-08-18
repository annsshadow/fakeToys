
## KVM VCPU 璇锋眰锛圞VM VCPU Requests锛?

## 姒傝堪


KVM 鏀寔涓€涓唴閮?API锛屼娇绾跨▼鑳藉璇锋眰鏌愪釜 VCPU 绾跨▼鎵ц鏌愪簺娲诲姩銆備緥濡傦紝涓€涓嚎绋嬪彲浠ヨ姹傛煇涓?VCPU 鍒锋柊
```

  /* 妫€鏌?VCPU @vcpu 鏄惁鏈変换浣曞緟澶勭悊璇锋眰銆?*/
  bool kvm_request_pending(struct kvm_vcpu *vcpu);

  /* 妫€鏌?VCPU @vcpu 鏄惁鏈夎姹?@req 寰呭鐞嗐€?*/
  bool kvm_test_request(int req, struct kvm_vcpu *vcpu);

  /* 娓呴櫎 VCPU @vcpu 鐨勮姹?@req銆?*/
  void kvm_clear_request(int req, struct kvm_vcpu *vcpu);

  /*
   * 妫€鏌?VCPU @vcpu 鏄惁鏈夎姹?@req 寰呭鐞嗐€傚綋鏈夎姹傚緟澶勭悊鏃讹紝
   * 瀹冨皢琚竻闄わ紝骞跺彂鍑轰竴涓唴瀛樺睆闅滐紙memory barrier锛夛紝璇ュ睆闅滀笌
   * kvm_make_request() 涓殑鍙︿竴涓睆闅滈厤瀵广€?   */
  bool kvm_check_request(int req, struct kvm_vcpu *vcpu);

  /*
   * 瀵?VCPU @vcpu 鍙戝嚭璇锋眰 @req銆傚彂鍑轰竴涓唴瀛樺睆闅滐紝璇ュ睆闅滀笌
   * kvm_check_request() 涓殑鍙︿竴涓睆闅滈厤瀵癸紝鐒跺悗鍐嶈缃姹傘€?   */
  void kvm_make_request(int req, struct kvm_vcpu *vcpu);

  /* 瀵?struct kvm @kvm 鎵€琛ㄧず鐨?VM 鐨勬墍鏈?VCPU 鍙戝嚭璇锋眰 @req銆?*/
  bool kvm_make_all_cpus_request(struct kvm *kvm, unsigned int req);

```
閫氬父锛岃姹傛柟甯屾湜 VCPU 鍦ㄥ彂鍑鸿姹傚悗灏藉揩鎵ц璇ユ椿鍔ㄣ€傝繖鎰忓懗鐫€澶у鏁拌姹傦紙kvm_make_request() 璋冪敤锛変箣鍚庝細璺熼殢涓€娆″ kvm_vcpu_kick() 鐨勮皟鐢紝鑰?kvm_make_all_cpus_request() 宸茬粡灏嗚涪閱掞紙kick锛夋墍鏈?VCPU 鐨勬搷浣滃唴寤哄叾涓€?

### VCPU 韪㈤啋锛圴CPU Kicks锛?

VCPU 韪㈤啋鐨勭洰鏍囨槸浣夸竴涓?VCPU 绾跨▼閫€鍑哄鎴锋満锛坓uest锛夋ā寮忥紝浠ヤ究鎵ц鏌愪簺 KVM 缁存姢宸ヤ綔銆備负姝わ紝浼氬彂閫佷竴涓?IPI锛屽己鍒跺鎴锋満妯″紡閫€鍑恒€傜劧鑰岋紝VCPU 绾跨▼鍦ㄨ涪閱掓椂鍙兘骞朵笉澶勪簬瀹㈡埛鏈烘ā寮忋€傚洜姝わ紝鏍规嵁 VCPU 绾跨▼鐨勬ā寮忓拰鐘舵€侊紝韪㈤啋杩樺彲鑳介噰鍙栧彟澶栦袱绉嶅姩浣溿€備互涓嬪垪鍑哄叏閮ㄤ笁绉嶅姩浣滐細

1) 鍙戦€佷竴涓?IPI銆傝繖浼氬己鍒堕€€鍑哄鎴锋満妯″紡銆?2) 鍞ら啋涓€涓潯鐪犱腑鐨?VCPU銆傜潯鐪犱腑鐨?VCPU 鏄浜庡鎴锋満妯″紡涔嬪銆佸湪绛夊緟闃熷垪锛坵aitqueue锛変笂绛夊緟鐨?VCPU 绾跨▼銆傚敜閱掑畠浠細灏嗙嚎绋嬩粠绛夊緟闃熷垪绉婚櫎锛屼娇绾跨▼鑳藉鍐嶆杩愯銆傛琛屼负鍙兘琚姂鍒讹紝鍙傝涓嬫枃鐨?KVM_REQUEST_NO_WAKEUP銆?3) 浠€涔堥兘涓嶅仛銆傚綋 VCPU 涓嶅浜庡鎴锋満妯″紡涓?VCPU 绾跨▼娌℃湁鐫＄湢鏃讹紝鍒欐棤浜嬪彲鍋氥€?

### VCPU 妯″紡锛圴CPU Mode锛?

VCPU 鏈変竴涓ā寮忕姸鎬?`vcpu->mode`锛岀敤浜庤窡韪鎴锋満鏄惁姝ｅ湪瀹㈡埛鏈烘ā寮忎笅杩愯锛屼互鍙婁竴浜涚壒瀹氱殑瀹㈡埛鏈烘ā寮忎箣澶栫殑鐘舵€併€傛灦鏋勫眰鍙互浣跨敤 `vcpu->mode` 鏉ョ‘淇?VCPU 璇锋眰琚?VCPU 鐪嬪埌锛堝弬瑙?纭繚璇锋眰琚湅鍒?锛夛紝浠ュ強閬垮厤鍙戦€佷笉蹇呰鐨?IPI锛堝弬瑙?IPI 绮剧畝"锛夛紝鐢氳嚦纭繚绛夊緟 IPI 纭锛堝弬瑙?绛夊緟纭"锛夈€傚畾涔変簡浠ヤ笅妯″紡锛?
OUTSIDE_GUEST_MODE

  VCPU 绾跨▼澶勪簬瀹㈡埛鏈烘ā寮忎箣澶栥€?
IN_GUEST_MODE

  VCPU 绾跨▼澶勪簬瀹㈡埛鏈烘ā寮忎箣涓€?
EXITING_GUEST_MODE

  VCPU 绾跨▼姝ｄ粠 IN_GUEST_MODE 杩囨浮鍒?OUTSIDE_GUEST_MODE銆?
READING_SHADOW_PAGE_TABLES

  VCPU 绾跨▼澶勪簬瀹㈡埛鏈烘ā寮忎箣澶栵紝浣嗗畠甯屾湜鏌愪簺 VCPU 璇锋眰锛堝嵆 KVM_REQ_TLB_FLUSH锛夌殑鍙戦€佹柟绛夊緟锛岀洿鍒?VCPU 绾跨▼瀹屾垚椤佃〃璇诲彇銆?

## VCPU 璇锋眰鍐呴儴鏈哄埗


VCPU 璇锋眰浠呬粎鏄?`vcpu->requests` 浣嶅浘涓殑浣嶇储寮曘€傝繖鎰忓懗鐫€閫氱敤鐨勪綅鎿嶄綔锛坆itop锛夛紝渚嬪 [atomic-ops]_ 涓褰曠殑閭ｄ簺锛屽彲鐢ㄤ簬
```

  clear_bit(KVM_REQ_UNBLOCK & KVM_REQUEST_MASK, &vcpu->requests);

```
涓嶈繃锛孷CPU 璇锋眰鐨勪娇鐢ㄨ€呭簲褰撻伩鍏嶈繖鏍峰仛锛屽洜涓洪偅浼氱牬鍧忔娊璞°€傚墠 8 浣嶄繚鐣欑粰涓庢灦鏋勬棤鍏崇殑璇锋眰锛涙墍鏈夐澶栫殑浣嶅彲渚涗笌鏋舵瀯鐩稿叧鐨勮姹備娇鐢ㄣ€?

### 涓庢灦鏋勬棤鍏崇殑璇锋眰


KVM_REQ_TLB_FLUSH

  KVM 閫氱敤鐨?MMU notifier 鍙兘闇€瑕佸埛鏂板鎴锋満鎵€鏈夌殑 TLB 椤癸紝璋冪敤 kvm_flush_remote_tlbs() 鏉ュ畬鎴愩€傞€夋嫨浣跨敤閫氱敤 kvm_flush_remote_tlbs() 瀹炵幇鐨勬灦鏋勯渶瑕佸鐞嗘 VCPU 璇锋眰銆?
KVM_REQ_VM_DEAD

  姝よ姹傞€氱煡鎵€鏈?VCPU 璇?VM 宸叉浜′笖涓嶅彲鐢紝渚嬪鐢变簬鑷村懡閿欒鎴?VM 鐨勭姸鎬佽鏈夋剰閿€姣併€?
KVM_REQ_UNBLOCK

  姝よ姹傞€氱煡 vCPU 閫€鍑?kvm_vcpu_block銆備緥濡傦紝瀹冪敤浜庝唬琛?vCPU 鍦ㄤ富鏈轰笂杩愯鐨勫畾鏃跺櫒澶勭悊绋嬪簭锛屾垨鑰呯敤浜庢洿鏂颁腑鏂矾鐢卞苟纭繚宸插垎閰嶇殑锛坅ssigned锛夎澶囪兘澶熷敜閱?vCPU銆?
KVM_REQ_OUTSIDE_GUEST_MODE

  姝?璇锋眰"纭繚鐩爣 vCPU 鍦ㄨ姹傚彂閫佹柟缁х画鎵ц涔嬪墠宸茬粡閫€鍑哄鎴锋満妯″紡銆傜洰鏍囨棤闇€閲囧彇浠讳綍鍔ㄤ綔锛屽洜姝ゅ疄闄呬笂涓嶄細涓虹洰鏍囪褰曚换浣曡姹傘€傛璇锋眰绫讳技浜?韪㈤啋锛坘ick锛?锛屼絾涓庤涪閱掍笉鍚岀殑鏄紝瀹冧繚璇?vCPU 纭疄宸茬粡閫€鍑哄鎴锋満妯″紡銆傝涪閱掑彧淇濊瘉 vCPU 浼氬湪灏嗘潵鐨勬煇涓椂鍒婚€€鍑猴紝渚嬪涔嬪墠鐨勮涪閱掑彲鑳藉凡缁忓惎鍔ㄤ簡璇ヨ繃绋嬶紝浣嗘棤娉曚繚璇佸嵆灏嗚韪㈤啋鐨?vCPU 宸茬粡瀹屽叏閫€鍑哄鎴锋満妯″紡銆?

### KVM_REQUEST_MASK


鍦ㄤ娇鐢ㄤ綅鎿嶄綔澶勭悊 VCPU 璇锋眰涔嬪墠锛屽簲褰撳厛鐢?KVM_REQUEST_MASK 瀵瑰叾鎺╃爜銆傝繖鏄洜涓哄彧鏈変綆 8 浣嶇敤浜庤〃绀鸿姹傜紪鍙枫€傞珮浣嶇敤浣滄爣蹇椼€傜洰鍓嶅彧瀹氫箟浜嗕袱涓爣蹇椼€?

### VCPU 璇锋眰鏍囧織


KVM_REQUEST_NO_WAKEUP

  姝ゆ爣蹇楀簲鐢ㄤ簬鍙渶瑕佸浜庡鎴锋満妯″紡鐨?VCPU 绔嬪嵆鍏虫敞鐨勮姹傘€備篃灏辨槸璇达紝鐫＄湢涓殑 VCPU 涓嶉渶瑕佷负杩欎簺璇锋眰鑰岃鍞ら啋銆傜潯鐪犱腑鐨?VCPU 浼氬湪绋嶅悗鐢变簬鍏朵粬鍘熷洜琚敜閱掓椂澶勭悊杩欎簺璇锋眰銆?
KVM_REQUEST_WAIT

  褰撳甫鏈夋鏍囧織鐨勮姹傞€氳繃 kvm_make_all_cpus_request() 鍙戝嚭鏃讹紝璋冪敤鏂瑰皢绛夊緟姣忎釜 VCPU 纭鍏?IPI 鍚庡啀缁х画銆傛鏍囧織鍙€傜敤浜庝細鏀跺埌 IPI 鐨?VCPU銆備緥濡傦紝濡傛灉 VCPU 姝ｅ湪鐫＄湢锛屽洜姝や笉闇€瑕?IPI锛岄偅涔堣姹傜嚎绋嬪氨涓嶄細绛夊緟銆傝繖鎰忓懗鐫€姝ゆ爣蹇楀彲浠ュ畨鍏ㄥ湴涓?KVM_REQUEST_NO_WAKEUP 缁勫悎浣跨敤銆傛湁鍏冲甫鏈?KVM_REQUEST_WAIT 鐨勮姹傜殑鏇村淇℃伅锛岃鍙傞槄"绛夊緟纭"銆?

## 甯︽湁鐩稿叧鐘舵€佺殑 VCPU 璇锋眰


璇锋眰鏂瑰笇鏈涙帴鏀?VCPU 澶勭悊鏂扮姸鎬佺殑璇濓紝闇€瑕佺‘淇濆湪鎺ユ敹 VCPU 绾跨▼鐨?CPU 瑙傚療鍒拌璇锋眰鏃讹紝鏂板啓鍏ョ殑鐘舵€佸鍏跺彲瑙併€傝繖鎰忓懗鐫€蹇呴』鍦ㄥ啓鍏ユ柊鐘舵€佷箣鍚庛€佽缃?VCPU 璇锋眰浣嶄箣鍓嶆彃鍏ヤ竴涓啓鍐呭瓨灞忛殰锛坵rite memory barrier锛夈€傛澶栵紝鍦ㄦ帴鏀?VCPU 绾跨▼涓€渚э紝蹇呴』鍦ㄨ鍙栬姹備綅涔嬪悗銆佺户缁鍙栦笌涔嬪叧鑱旂殑鏂扮姸鎬佷箣鍓嶏紝鎻掑叆涓€涓浉搴旂殑璇诲睆闅滐紙read barrier锛夈€傝鍙傞槄 [lwn-mb]_ 鐨勫満鏅?3锛堟秷鎭笌鏍囧織锛夛紝浠ュ強鍐呮牳鏂囨。 [memory-barriers]_銆?
kvm_check_request() 鍜?kvm_make_request() 杩欎竴瀵瑰嚱鏁版彁渚涗簡鍐呭瓨灞忛殰锛屼娇寰楄瑕佹眰鍙敱 API 鍦ㄥ唴閮ㄥ鐞嗐€?

## 纭繚璇锋眰琚湅鍒?

鍦ㄥ悜 VCPU 鍙戝嚭璇锋眰鏃讹紝鎴戜滑甯屾湜閬垮厤鎺ユ敹 VCPU 鍦ㄥ鎴锋満妯″紡涓嬫墽琛屼换鎰忛暱鏃堕棿鑰屼笉澶勭悊璇ヨ姹傘€傚彧瑕佺‘淇?VCPU 绾跨▼鍦ㄨ繘鍏ュ鎴锋満妯″紡涔嬪墠妫€鏌?kvm_request_pending()锛屽苟涓斿湪蹇呰鏃惰涪閱掍細鍙戦€?IPI 浠ュ己鍒堕€€鍑哄鎴锋満妯″紡锛屾垜浠氨鍙互纭俊杩欑鎯呭喌涓嶄細鍙戠敓銆傚繀椤绘牸澶栧皬蹇冿紝浠ヨ鐩?VCPU 绾跨▼鏈€鍚庝竴娆?kvm_request_pending() 妫€鏌ヤ箣鍚庛€佸埌瀹冭繘鍏ュ鎴锋満妯″紡涔嬪墠鐨勮繖娈垫椂闂达紝鍥犱负韪㈤啋 IPI 鍙細瀵瑰浜庡鎴锋満妯″紡鐨?VCPU 绾跨▼銆佹垨鑷冲皯宸茬粡绂佺敤涓柇浠ュ噯澶囪繘鍏ュ鎴锋満妯″紡鐨?VCPU 绾跨▼瑙﹀彂瀹㈡埛鏈烘ā寮忛€€鍑恒€傝繖鎰忓懗鐫€涓€涓紭鍖栧疄鐜帮紙鍙傝"IPI 绮剧畝"锛夊繀椤荤‘瀹氫綍鏃朵笉鍙戦€?IPI 鏄畨鍏ㄧ殑銆備竴涓櫎 s390 涔嬪鐨勬墍鏈夋灦鏋勯兘閲囩敤鐨勮В鍐虫柟妗堟槸锛?
- 鍦ㄧ鐢ㄤ腑鏂拰鏈€鍚庝竴娆?kvm_request_pending() 妫€鏌ヤ箣闂达紝灏?`vcpu->mode` 璁剧疆涓?IN_GUEST_MODE锛?- 鍦ㄨ繘鍏ュ鎴锋満鏃跺師瀛愬湴鍚敤涓柇銆?
姝よВ鍐虫柟妗堣繕闇€瑕佸湪璇锋眰绾跨▼鍜屾帴鏀?VCPU 涓皑鎱庢斁缃唴瀛樺睆闅溿€傚€熷姪鍐呭瓨灞忛殰锛屾垜浠彲浠ユ帓闄よ繖鏍蜂竴绉嶅彲鑳芥€э細鍗充竴涓?VCPU 绾跨▼鍦ㄦ渶鍚庝竴娆℃鏌ヤ腑瑙傚療鍒?!kvm_request_pending()锛岀劧鍚庡嵈娌℃湁鏀跺埌閽堝绱ф帴鐫€璇ユ鏌ヤ箣鍚庡彂鍑虹殑涓嬩竴涓姹傜殑 IPI銆傝繖鏄€氳繃 Dekker 鍐呭瓨灞忛殰妯″紡锛圼lwn-mb]_ 鐨勫満鏅?10锛夊疄鐜扮殑銆傜敱浜?Dekker 妯″紡闇€瑕佷袱涓彉閲忥紝姝ゆ柟妗堝皢 `vcpu->mode` 涓?`vcpu->requests` 閰嶅銆備唬鍏?```

  CPU1                                    CPU2
  =================                       =================
  local_irq_disable();
  WRITE_ONCE(vcpu->mode, IN_GUEST_MODE);  kvm_make_request(REQ, vcpu);
  smp_mb();                               smp_mb();
  if (kvm_request_pending(vcpu)) {        if (READ_ONCE(vcpu->mode) ==
                                              IN_GUEST_MODE) {
      ...abort guest entry...                 ...send IPI...
  }                                       }

```
濡備笂鎵€杩帮紝IPI 鍙澶勪簬瀹㈡埛鏈烘ā寮忔垨宸茬粡绂佺敤涓柇鐨?VCPU 绾跨▼鏈夌敤銆傝繖灏辨槸涓轰粈涔?Dekker 妯″紡鐨勮繖绉嶇壒瀹氭儏褰㈣鎵╁睍涓哄湪灏?`vcpu->mode` 璁剧疆涓?IN_GUEST_MODE 涔嬪墠鍏堢鐢ㄤ腑鏂€備娇鐢?WRITE_ONCE() 鍜?READ_ONCE() 鏄负浜嗕弗璋ㄥ湴瀹炵幇鍐呭瓨灞忛殰妯″紡锛屼繚璇佺紪璇戝櫒涓嶄細骞叉壈 `vcpu->mode` 琚簿蹇冨畨鎺掔殑璁块棶銆?

### IPI 绮剧畝锛圛PI Reduction锛?

鐢变簬鍙渶瑕佷竴涓?IPI 鍗冲彲璁?VCPU 妫€鏌ヤ换鎰?鎵€鏈夎姹傦紝鍥犳杩欎簺 IPI 鍙互琚悎骞躲€傝繖寰堝鏄撳仛鍒帮細璁╃涓€娆″彂閫?IPI 鐨勮涪閱掑悓鏃跺皢 VCPU 妯″紡鏀逛负闈?IN_GUEST_MODE 鐨勬煇绉嶇姸鎬併€傝繃娓＄姸鎬?EXITING_GUEST_MODE 灏辨槸涓烘鐩殑鑰屼娇鐢ㄧ殑銆?

### 绛夊緟纭锛圵aiting for Acknowledgements锛?

鏌愪簺璇锋眰锛堝嵆甯︽湁 KVM_REQUEST_WAIT 鏍囧織鐨勮姹傦級闇€瑕佸彂閫?IPI锛屽苟涓旈渶瑕佺瓑寰呯‘璁わ紝鍗充娇鐩爣 VCPU 绾跨▼澶勪簬 IN_GUEST_MODE 涔嬪鐨勬ā寮忋€備緥濡傦紝涓€涓儏褰㈡槸鐩爣 VCPU 绾跨▼澶勪簬 READING_SHADOW_PAGE_TABLES 妯″紡锛岃妯″紡鏄湪绂佺敤涓柇鍚庤缃殑銆備负浜嗘敮鎸佽繖浜涙儏褰紝KVM_REQUEST_WAIT 鏍囧織灏嗗彂閫?IPI 鐨勬潯浠朵粠妫€鏌?VCPU 鏄惁澶勪簬 IN_GUEST_MODE 鏀逛负妫€鏌ュ畠鏄惁涓嶅浜?OUTSIDE_GUEST_MODE銆?

### 鏃犺姹傜殑 VCPU 韪㈤啋锛圧equest-less VCPU Kicks锛?

鐢变簬鏄惁鍙戦€?IPI 鍙栧喅浜庡弻鍙橀噺 Dekker 鍐呭瓨灞忛殰妯″紡锛屽洜姝ゅ緢鏄庢樉锛屾棤璇锋眰鐨?VCPU 韪㈤啋鍑犱箮姘歌繙鏄笉姝ｇ‘鐨勩€傚鏋滄病鏈?闈?IPI 浜х敓鐨勮涪閱掍粛浼氬鑷存帴鏀?VCPU 閲囧彇鍔ㄤ綔"鐨勪繚璇侊紙姝ｅ鏈€缁堢殑 kvm_request_pending() 妫€鏌ュ浜庢湁璇锋眰浼撮殢鐨勮涪閱掓墍鍋氱殑閭ｆ牱锛夛紝閭ｄ箞璇ヨ涪閱掑彲鑳芥牴鏈笉浼氬仛浠讳綍鏈夌敤鐨勪簨鎯呫€備緥濡傦紝濡傛灉瀵逛竴涓垰鍒氳灏嗚嚜韬ā寮忚缃负 IN_GUEST_MODE 鐨?VCPU 鍙戝嚭鏃犺姹傝涪閱掞紙鎰忓懗鐫€涓嶄細鍙戦€?IPI锛夛紝閭ｄ箞璇?VCPU 绾跨▼鍙兘浼氱户缁叾杩涘叆杩囩▼锛岃€屽疄闄呬笂骞舵湭鎵ц璇ヨ涪閱掓湰搴斿惎鍔ㄧ殑浠讳綍鎿嶄綔銆?
涓€涓緥澶栨槸 x86 鐨?posted interrupt 鏈哄埗銆備笉杩囷紝鍗充究鍦ㄨ繖绉嶆儏褰笅锛屽嵆渚挎槸鏃犺姹傜殑 VCPU 韪㈤啋锛屼篃涓庝笂杩扮浉鍚岀殑 local_irq_disable() + smp_mb() 妯″紡鐩歌€﹀悎锛沺osted interrupt 鎻忚堪绗︿腑鐨?ON 浣嶏紙Outstanding Notification锛夋壆婕斾簡 `vcpu->requests` 鐨勮鑹层€傚彂閫?posted interrupt 鏃讹紝鍦ㄨ鍙?`vcpu->mode` 涔嬪墠璁剧疆 PIR.ON锛涜€屽湪 VCPU 绾跨▼涓紝vmx_sync_pir_to_irr() 鍦ㄥ皢 `vcpu->mode` 璁剧疆涓?IN_GUEST_MODE 涔嬪悗璇诲彇 PIR銆?

## 鍏朵粬鑰冭檻


### 鐫＄湢涓殑 VCPU


VCPU 绾跨▼鍙兘闇€瑕佸湪璋冪敤鍙兘浣垮叾鐫＄湢鐨勫嚱鏁帮紙渚嬪 kvm_vcpu_block()锛変箣鍓嶅拰/鎴栦箣鍚庤€冭檻璇锋眰銆傚畠浠槸鍚﹁繖鏍峰仛锛屼互鍙婂鏋滆繖鏍峰仛鐨勮瘽闇€瑕佽€冭檻鍝簺璇锋眰锛屽彇鍐充簬鏋舵瀯銆俴vm_vcpu_block() 璋冪敤 kvm_arch_vcpu_runnable() 鏉ユ鏌ユ槸鍚﹀簲璇ュ敜閱掋€傝繖鏍峰仛鐨勪竴涓師鍥犳槸涓烘灦鏋勬彁渚涗竴涓嚱鏁帮紝浠ヤ究鍦ㄥ繀瑕佹椂妫€鏌ヨ姹傘€?

## 鍙傝€冭祫鏂?
