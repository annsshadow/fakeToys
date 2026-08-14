## 搴旂敤鏁版嵁瀹屾暣鎬э紙ADI锛?

SPARC M7 澶勭悊鍣ㄦ柊澧炰簡搴旂敤鏁版嵁瀹屾暣鎬э紙ADI锛夌壒鎬с€?ADI 鍏佽浠诲姟鍦ㄥ叾鍦板潃绌洪棿鐨勪换鎰忓瓙闆嗕笂璁剧疆鐗堟湰鏍囩銆備竴鏃﹀惎鐢ㄤ簡 ADI 骞朵负浠诲姟鐨?鍦板潃绌洪棿鑼冨洿璁剧疆浜嗙増鏈爣绛撅紝澶勭悊鍣ㄥ氨浼氬皢鎸囧悜杩欎簺鑼冨洿鍐呭唴瀛樼殑鎸囬拡涓殑鏍囩
涓庡簲鐢ㄧ▼搴忓厛鍓嶈缃殑鐗堟湰杩涜姣旇緝銆傚彧鏈夊綋缁欏畾鎸囬拡涓殑鏍囩涓庡簲鐢ㄧ▼搴忚缃殑
鏍囩鐩稿尮閰嶆椂锛屾墠鍏佽璁块棶鍐呭瓨銆傚鏋滀笉鍖归厤锛屽鐞嗗櫒浼氬紩鍙戝紓甯搞€?
涓轰簡瀹屽叏鍚敤 ADI锛屼换鍔″繀椤婚噰鍙栦互涓嬫楠わ細

1. 璁剧疆鐢ㄦ埛妯″紡鐨?PSTATE.mcde 浣嶃€傚畠浣滀负浠诲姟鏁翠釜鍦板潃绌洪棿鐨勪富寮€鍏筹紝鐢ㄤ簬鍚敤/
   绂佺敤璇ヤ换鍔＄殑 ADI銆?
2. 鍦ㄤ笌鍚敤 ADI 鐨勫湴鍧€鑼冨洿瀵瑰簲鐨勪换浣?TLB 琛ㄩ」涓婅缃?TTE.mcd 浣嶃€侻MU 鍙細瀵?   璁剧疆浜?TTE.mcd 浣嶇殑椤甸潰妫€鏌ョ増鏈爣绛俱€?
3. 浣跨敤 stxa 鎸囦护浠ュ強鏌愪釜 MCD 涓撶敤鐨?ASI 鏉ヨ缃櫄鎷熷湴鍧€鐨勭増鏈爣绛俱€傛瘡鏉?stxa
   鎸囦护涓轰竴涓?ADI 鍧楀ぇ灏忔暟閲忕殑瀛楄妭璁剧疆缁欏畾鐨勬爣绛俱€傚繀椤诲鏁撮〉閲嶅姝ゆ楠わ紝鎵嶈兘
   涓烘暣椤佃缃爣绛俱€?
骞冲彴涓婄殑 ADI 鍧楀ぇ灏忕敱 hypervisor锛堣櫄鎷熸満鐩戞帶鍣級鍦ㄦ満鍣ㄦ弿杩拌〃涓彁渚涚粰鍐呮牳銆?Hypervisor 杩樻彁渚涜櫄鎷熷湴鍧€涓敤浜庢寚瀹氱増鏈爣绛剧殑楂樹綅浣嶆暟銆備竴鏃︿负鏌愪釜鍐呭瓨浣嶇疆
璁剧疆浜嗙増鏈爣绛撅紝璇ユ爣绛惧氨瀛樺偍鍦ㄧ墿鐞嗗唴瀛樹腑锛屽苟涓斿湪鍛堢幇缁?MMU 鐨勮櫄鎷熷湴鍧€鐨?ADI
鐗堟湰鏍囩浣嶄腑蹇呴』瀛樺湪鐩稿悓鐨勬爣绛俱€備緥濡傦紝鍦?SPARC M7 澶勭悊鍣ㄤ笂锛孧MU 浣跨敤浣?63-60
浣滀负鐗堟湰鏍囩锛孉DI 鍧楀ぇ灏忎笌缂撳瓨琛屽ぇ灏忕浉鍚岋紝鍗?64 瀛楄妭銆備竴涓湪涓€娈靛唴瀛樹笂灏?ADI
鐗堟湰璁剧疆涓猴紙渚嬪锛?0 鐨勪换鍔★紝蹇呴』浣跨敤鍦?63-60 浣嶄腑鍖呭惈 0xa 鐨勮櫄鎷熷湴鍧€鏉ヨ闂?璇ュ唴瀛樸€?
ADI 閫氳繃甯︽湁 PROT_ADI 鏍囧織鐨?mprotect() 鍦ㄤ竴缁勯〉闈笂鍚敤銆傚綋浠诲姟棣栨鍦ㄤ竴缁勯〉闈?涓婂惎鐢?ADI 鏃讹紝鍐呮牳涓鸿浠诲姟璁剧疆 PSTATE.mcde 浣嶃€傚唴瀛樺湴鍧€鐨勭増鏈爣绛鹃€氳繃 stxa
鎸囦护銆佷娇鐢ㄥ湴鍧€涓婄殑 ASI_MCD_PRIMARY 鎴?ASI_MCD_ST_BLKINIT_PRIMARY 鏉ヨ缃€侫DI 鍧?澶у皬鐢?hypervisor 鎻愪緵缁欏唴鏍搞€傚唴鏍搁€氳繃杈呭姪鍚戦噺锛坅uxiliary vector锛夊皢 ADI 鍧楀ぇ灏?鐨勫€间笌鍏朵粬 ADI 淇℃伅涓€璧疯繑鍥炵粰鐢ㄦ埛绌洪棿銆傚唴鏍告彁渚涗互涓嬭緟鍔╁悜閲忥細

	============	===========================================
	AT_ADI_BLKSZ	ADI 鍧楀ぇ灏忋€傝繖鏄?ADI 鐗堟湰鎺у埗鐨勭矑搴﹀拰
			瀵归綈鏂瑰紡锛屼互瀛楄妭涓哄崟浣嶃€?	AT_ADI_NBITS	VA 涓?ADI 鐗堟湰浣嶇殑浣嶆暟
	============	===========================================


## 閲嶈璇存槑


- 0x0 鍜?0xf 鐨勭増鏈爣绛惧€艰淇濈暀銆傝繖浜涘€煎尮閰嶈櫄鎷熷湴鍧€涓殑浠讳綍鏍囩锛屾案杩滀笉浼?  浜х敓涓嶅尮閰嶅紓甯搞€?
- 鐗堟湰鏍囩鏄湪鐢ㄦ埛绌洪棿瀵硅櫄鎷熷湴鍧€璁剧疆鐨勶紝灏界鏍囩瀛樺偍鍦ㄧ墿鐞嗗唴瀛樹腑銆傛爣绛炬槸鍦?  鐗╃悊椤甸潰鍒嗛厤缁欎换鍔″苟涓哄叾鍒涘缓浜?pte 涔嬪悗锛屽湪璇ョ墿鐞嗛〉闈笂璁剧疆鐨勩€?
- 褰撲换鍔￠噴鏀惧畠鏇捐缃繃鐗堟湰鏍囩鐨勫唴瀛橀〉鏃讹紝璇ラ〉浼氬洖鍒扮┖闂查〉姹犮€傚綋姝ら〉琚噸鏂?  鍒嗛厤缁欐煇涓换鍔℃椂锛屽唴鏍镐娇鐢ㄥ潡鍒濆鍖?ASI 娓呴櫎璇ラ〉锛屽悓鏃朵篃娓呴櫎浜嗚椤电殑鐗堟湰
  鏍囩銆傚鏋滀竴涓垎閰嶇粰浠诲姟鐨勯〉琚噴鏀惧悗鍙堝垎閰嶅洖鍚屼竴涓换鍔★紝璇ヤ换鍔′箣鍓嶅湪璇ラ〉涓?  璁剧疆鐨勬棫鐗堟湰鏍囩灏嗕笉鍐嶅瓨鍦ㄣ€?
- 瀵逛簬闈炴晠闅滃姞杞斤紙non-faulting loads锛夛紝涓嶄細妫€娴嬪埌 ADI 鏍囩涓嶅尮閰嶃€?
- 鍐呮牳涓嶄細涓虹敤鎴烽〉璁剧疆浠讳綍鏍囩锛岃缃换浣曠増鏈爣绛惧畬鍏ㄦ槸浠诲姟鑷繁鐨勮矗浠汇€傚唴鏍?  纭疄浼氱‘淇濓細濡傛灉涓€涓〉琚崲鍑哄埌纾佺洏鍐嶆崲鍏ワ紝鐗堟湰鏍囩浼氳淇濈暀锛涘鏋滈〉琚縼绉伙紝
  鐗堟湰鏍囩涔熶細琚繚鐣欍€?
- ADI 閫傜敤浜庝换鎰忓ぇ灏忕殑椤甸潰銆傜敤鎴风┖闂翠换鍔″湪浣跨敤 ADI 鏃朵笉闇€瑕佺煡閬撻〉闈㈠ぇ灏忋€傚畠
  鍙渶閫夋嫨涓€涓櫄鎷熷湴鍧€鑼冨洿锛屼娇鐢?mprotect() 鍦ㄨ鑼冨洿涓婂惎鐢?ADI锛屽苟涓烘暣涓寖鍥?  璁剧疆鐗堟湰鏍囩銆俶protect() 纭繚鑼冨洿鎸夐〉闈㈠ぇ灏忓榻愪笖鏄〉闈㈠ぇ灏忕殑鏁存暟鍊嶃€?
- ADI 鏍囩鍙兘璁剧疆鍦ㄥ彲鍐欏唴瀛樹笂銆備緥濡傦紝ADI 鏍囩涓嶈兘璁剧疆鍦ㄥ彧璇绘槧灏勪笂銆?

## ADI 鐩稿叧鐨勯櫡闃?

鍚敤 ADI 鍚庯紝鍙兘浼氬彂鐢熶互涓嬫柊鐨勯櫡闃憋細

### Disrupting memory corruption锛堢牬鍧忔€у唴瀛樻崯鍧忥級


	褰撲竴娆″瓨鍌ㄨ闂竴涓叿鏈?TTE.mcd=1 鐨勫唴瀛樹綅缃€佷换鍔℃鍦ㄤ互 ADI 鍚敤鐘舵€?	杩愯锛圥STATE.mcde=1锛夈€佷笖鎵€鐢ㄥ湴鍧€涓殑 ADI 鏍囩锛堜綅 63:60锛変笌鐩稿簲缂撳瓨琛?	涓婅缃殑鏍囩涓嶅尮閰嶆椂锛屽氨浼氬彂鐢熷唴瀛樻崯鍧忛櫡闃便€傞粯璁ゆ儏鍐典笅锛屽畠鏄竴涓牬鍧忔€?	闄烽槺锛岄鍏堣鍙戦€佺粰 hypervisor銆侶ypervisor 鍒涘缓涓€涓?sun4v 閿欒鎶ュ憡锛屽苟鍚?	鍐呮牳鍙戦€佷竴涓彲鎭㈠閿欒锛圱T=0x7e锛夐櫡闃便€傚唴鏍稿悜瀵艰嚧姝ら櫡闃辩殑浠诲姟鍙戦€佷竴涓?	SIGSEGV锛屽叾鍐呭涓轰互涓?```

		siginfo.si_signo = SIGSEGV;
		siginfo.errno = 0;
		siginfo.si_code = SEGV_ADIDERR;
		siginfo.si_addr = addr; /* 棣栨鍙戠敓涓嶅尮閰嶇殑 PC */
		siginfo.si_trapno = 0;


```
### Precise memory corruption锛堢簿纭唴瀛樻崯鍧忥級


	褰撲竴娆″瓨鍌ㄨ闂竴涓叿鏈?TTE.mcd=1 鐨勫唴瀛樹綅缃€佷换鍔℃鍦ㄤ互 ADI 鍚敤鐘舵€?	杩愯锛圥STATE.mcde=1锛夈€佷笖鎵€鐢ㄥ湴鍧€涓殑 ADI 鏍囩锛堜綅 63:60锛変笌鐩稿簲缂撳瓨琛?	涓婅缃殑鏍囩涓嶅尮閰嶆椂锛屽氨浼氬彂鐢熷唴瀛樻崯鍧忛櫡闃便€傚鏋滃惎鐢ㄤ簡 MCD 绮剧‘寮傚父
	锛圡CDPERR=1锛夛紝鍒欎細鍚戝唴鏍稿彂閫佷竴涓簿纭紓甯革紝TT=0x1a銆傚唴鏍稿悜瀵艰嚧姝ら櫡闃辩殑
	浠诲姟鍙戦€佷竴涓?SIGSEGV锛屽叾鍐呭涓轰互涓?```

		siginfo.si_signo = SIGSEGV;
		siginfo.errno = 0;
		siginfo.si_code = SEGV_ADIPERR;
		siginfo.si_addr = addr;	/* 寮曞彂闄烽槺鐨勫湴鍧€ */
		siginfo.si_trapno = 0;

	娉ㄦ剰锛?		瀵瑰姞杞界殑 ADI 鏍囩涓嶅尮閰嶆€绘槸瀵艰嚧绮剧‘闄烽槺銆?

```
### MCD disabled锛圡CD 宸茬鐢級


	褰撲换鍔″皻鏈惎鐢?ADI 鍗村皾璇曞湪鍐呭瓨鍦板潃涓婅缃?ADI 鐗堟湰鏃讹紝澶勭悊鍣ㄤ細鍙戦€佷竴涓?	MCD 宸茬鐢ㄩ櫡闃便€傛闄烽槺棣栧厛鐢?hypervisor 澶勭悊锛宧ypervisor 閫氳繃灏嗚闄烽槺
	鍚戦噺鍖栧埌鍐呮牳锛屼綔涓烘晠闅滅被鍨嬭缃负 0xa锛堟棤鏁?ASI锛夌殑鏁版嵁璁块棶寮傚父闄烽槺銆傚綋
	鍙戠敓杩欑鎯呭喌鏃讹紝鍐呮牳
```

		siginfo.si_signo = SIGSEGV;
		siginfo.errno = 0;
		siginfo.si_code = SEGV_ACCADI;
		siginfo.si_addr = addr;	/* 寮曞彂闄烽槺鐨勫湴鍧€ */
		siginfo.si_trapno = 0;


```
### Sample program to use ADI锛堜娇鐢?ADI 鐨勭ず渚嬬▼搴忥級


浠ヤ笅绀轰緥绋嬪簭鏃ㄥ湪璇存槑濡備綍浣跨敤 ADI
```

  #include <unistd.h>
  #include <stdio.h>
  #include <stdlib.h>
  #include <elf.h>
  #include <sys/ipc.h>
  #include <sys/shm.h>
  #include <sys/mman.h>
  #include <asm/asi.h>

  #ifndef AT_ADI_BLKSZ
  #define AT_ADI_BLKSZ	48
  #endif
  #ifndef AT_ADI_NBITS
  #define AT_ADI_NBITS	49
  #endif

  #ifndef PROT_ADI
  #define PROT_ADI	0x10
  #endif

  #define BUFFER_SIZE     32*1024*1024UL

  main(int argc, char* argv[], char* envp[])
  {
          unsigned long i, mcde, adi_blksz, adi_nbits;
          char *shmaddr, *tmp_addr, *end, *veraddr, *clraddr;
          int shmid, version;
	Elf64_auxv_t *auxv;

	adi_blksz = 0;

	while(*envp++ != NULL);
	for (auxv = (Elf64_auxv_t *)envp; auxv->a_type != AT_NULL; auxv++) {
		switch (auxv->a_type) {
		case AT_ADI_BLKSZ:
			adi_blksz = auxv->a_un.a_val;
			break;
		case AT_ADI_NBITS:
			adi_nbits = auxv->a_un.a_val;
			break;
		}
	}
	if (adi_blksz == 0) {
		fprintf(stderr, "Oops! ADI is not supported\n");
		exit(1);
	}

	printf("ADI capabilities:\n");
	printf("\tBlock size = %ld\n", adi_blksz);
	printf("\tNumber of bits = %ld\n", adi_nbits);

          if ((shmid = shmget(2, BUFFER_SIZE,
                                  IPC_CREAT | SHM_R | SHM_W)) < 0) {
                  perror("shmget failed");
                  exit(1);
          }

          shmaddr = shmat(shmid, NULL, 0);
          if (shmaddr == (char *)-1) {
                  perror("shm attach failed");
                  shmctl(shmid, IPC_RMID, NULL);
                  exit(1);
          }

	if (mprotect(shmaddr, BUFFER_SIZE, PROT_READ|PROT_WRITE|PROT_ADI)) {
		perror("mprotect failed");
		goto err_out;
	}

          /* 鍦?shm 娈典笂璁剧疆 ADI 鐗堟湰鏍囩
           */
          version = 10;
          tmp_addr = shmaddr;
          end = shmaddr + BUFFER_SIZE;
          while (tmp_addr < end) {
                  asm volatile(
                          "stxa %1, [%0]0x90\n\t"
                          :
                          : "r" (tmp_addr), "r" (version));
                  tmp_addr += adi_blksz;
          }
	asm volatile("membar #Sync\n\t");

          /* 閫氳繃灏嗙増鏈爣绛炬斁鍦ㄩ珮浣?adi_nbits 浣嶄腑锛?	 * 鐢辨櫘閫氬湴鍧€鍒涘缓涓€涓甫鐗堟湰鐨勫湴鍧€
           */
          tmp_addr = (void *) ((unsigned long)shmaddr << adi_nbits);
          tmp_addr = (void *) ((unsigned long)tmp_addr >> adi_nbits);
          veraddr = (void *) (((unsigned long)version << (64-adi_nbits))
                          | (unsigned long)tmp_addr);

          printf("Starting the writes:\n");
          for (i = 0; i < BUFFER_SIZE; i++) {
                  veraddr[i] = (char)(i);
                  if (!(i % (1024 * 1024)))
                          printf(".");
          }
          printf("\n");

          printf("Verifying data...");
	fflush(stdout);
          for (i = 0; i < BUFFER_SIZE; i++)
                  if (veraddr[i] != (char)i)
                          printf("\nIndex %lu mismatched\n", i);
          printf("Done.\n");

          /* 绂佺敤 ADI 骞舵竻鐞?           */
	if (mprotect(shmaddr, BUFFER_SIZE, PROT_READ|PROT_WRITE)) {
		perror("mprotect failed");
		goto err_out;
	}

          if (shmdt((const void *)shmaddr) != 0)
                  perror("Detach failure");
          shmctl(shmid, IPC_RMID, NULL);

          exit(0);

  err_out:
          if (shmdt((const void *)shmaddr) != 0)
                  perror("Detach failure");
          shmctl(shmid, IPC_RMID, NULL);
          exit(1);
  }

```
