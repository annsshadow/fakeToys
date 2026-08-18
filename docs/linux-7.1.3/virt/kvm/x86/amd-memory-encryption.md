
## 瀹夊叏鍔犲瘑铏氭嫙鍖栵紙SEV锛?

## 姒傝堪


瀹夊叏鍔犲瘑铏氭嫙鍖栵紙SEV锛夋槸 AMD 澶勭悊鍣ㄤ笂鎻愪緵鐨勪竴椤圭壒鎬с€?
SEV 鏄?AMD-V 鏋舵瀯鐨勬墿灞曪紝鏀寔鍦ㄨ櫄鎷熸満鐩戞帶鍣紙hypervisor锛夋帶鍒朵笅杩愯铏氭嫙鏈猴紙VM锛夈€傚惎鐢ㄥ悗锛岃櫄鎷熸満鐨勫瓨鍌ㄥ櫒鍐呭灏嗕娇鐢ㄤ笓灞炰簬璇ヨ櫄鎷熸満鐨勫瘑閽ヨ繘琛岄€忔槑鍔犲瘑銆?
铏氭嫙鏈虹洃鎺у櫒鍙互閫氳繃 CPUID 鎸囦护纭畾鏄惁鏀寔 SEV銆侰PUID 鍔熻兘 0x8000001f 鎶ュ憡鐩稿叧淇℃伅

```

	0x8000001f[eax]:
			Bit[1] 	indicates support for SEV
	    ...
		  [ecx]:
			Bits[31:0]  Number of encrypted guests supported simultaneously

```
濡傛灉瀛樺湪 SEV 鏀寔锛屽垯 MSR 0xc001_0010锛圡SR_AMD64_SYSCFG锛夊拰 MSR 0xc001_0015

```

	0xc001_0010:
		Bit[23]	   1 = memory encryption can be enabled
			   0 = memory encryption can not be enabled

	0xc001_0015:
		Bit[0]	   1 = memory encryption can be enabled
			   0 = memory encryption can not be enabled

```
褰?SEV 鏀寔鍙敤鏃讹紝鍙互閫氳繃濡備笅鏂瑰紡鍦ㄧ壒瀹氱殑铏氭嫙鏈轰腑鍚敤瀹?
```

	VMCB[0x90]:
		Bit[1]	    1 = SEV is enabled
			    0 = SEV is disabled

```
SEV 纭欢浣跨敤 ASID 灏嗗唴瀛樺姞瀵嗗瘑閽ヤ笌铏氭嫙鏈哄叧鑱斻€傚洜姝わ紝鍚敤 SEV 鐨勫鎴锋満鐨?ASID 蹇呴』浠嬩簬 1 涓?CPUID 0x8000001f[ecx] 瀛楁瀹氫箟鐨勬渶澶у€间箣闂淬€?
## KVM_MEMORY_ENCRYPT_OP ioctl


璁块棶 SEV 鐨勪富瑕?ioctl 鏄?KVM_MEMORY_ENCRYPT_OP锛屽畠浣滅敤浜?VM 鏂囦欢鎻忚堪绗︺€傚鏋?KVM_MEMORY_ENCRYPT_OP 鐨勫弬鏁颁负 NULL锛屽垯褰?SEV 鍚敤鏃惰 ioctl 杩斿洖 0锛岀鐢ㄦ椂杩斿洖 `ENOTTY`锛堝湪鏌愪簺杈冩棫鐨?Linux 鐗堟湰涓婏紝鍗充娇鍙傛暟涓?NULL锛岃 ioctl 涔熶細灏濊瘯姝ｅ父杩愯锛屽洜姝ゅ綋 SEV 鍚敤鏃跺緢鍙兘杩斿洖 `EFAULT` 鑰岄潪闆讹級銆傚鏋滈潪 NULL锛屽垯鍙傛暟鎸囧悜

```

       struct kvm_sev_cmd {
               __u32 id;
               __u64 data;
               __u32 error;
               __u32 sev_fd;
       };


```
`id` 瀛楁鍖呭惈瀛愬懡浠わ紝`data` 瀛楁鎸囧悜鍙︿竴涓寘鍚鍛戒护鐗瑰畾鍙傛暟鐨勭粨鏋勪綋銆俙sev_fd` 搴旀寚鍚戝湪 `/dev/sev` 璁惧涓婃墦寮€鐨勬枃浠舵弿杩扮锛堝鏋滈渶瑕佺殑璇濓紝瑙佸悇鍛戒护璇存槑锛夈€?
杈撳嚭鏃讹紝`error` 鍦ㄦ垚鍔熸椂涓洪浂锛屽惁鍒欎负閿欒鐮併€傞敊璇爜瀹氫箟浜?`<linux/psp-dev.h>`銆?
KVM 瀹炵幇浜嗕互涓嬪懡浠わ紝浠ユ敮鎸?SEV 瀹㈡埛鏈虹殑甯歌鐢熷懡鍛ㄦ湡浜嬩欢锛屼緥濡傚惎鍔ㄣ€佽繍琛屻€佸揩鐓с€佽縼绉诲拰閿€姣併€?
### 1. KVM_SEV_INIT2


KVM_SEV_INIT2 鍛戒护鐢辫櫄鎷熸満鐩戞帶鍣ㄧ敤浜庡垵濮嬪寲 SEV 骞冲彴涓婁笅鏂囥€傚湪鍏稿瀷鐨勫伐浣滄祦涓紝姝ゅ懡浠ゅ簲鏄彂鍑虹殑绗竴涓懡浠ゃ€?
瑕佽鎺ュ彈姝ゅ懡浠わ紝蹇呴』宸插皢 KVM_X86_SEV_VM 鎴?KVM_X86_SEV_ES_VM 浼犵粰 KVM_CREATE_VM ioctl銆備娇鐢ㄨ繖浜涙満鍣ㄧ被鍨嬪垱寤虹殑铏氭嫙鏈猴紝鍦ㄨ皟鐢?KVM_SEV_INIT2 涔嬪墠鏃犳硶杩愯銆?
鍙傛暟锛歴truct kvm_sev_init锛堣緭鍏ワ級

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_init {
                __u64 vmsa_features;  /* initial value of features field in VMSA */
                __u32 flags;          /* must be 0 */
                __u16 ghcb_version;   /* maximum guest GHCB version allowed */
                __u16 pad1;
                __u32 pad2[8];
        };

```
濡傛灉铏氭嫙鏈虹洃鎺у櫒涓嶆敮鎸?`flags` 鎴?`vmsa_features` 涓缃殑浠讳綍浣嶏紝鍒欎负閿欒銆傚浜?SEV 铏氭嫙鏈猴紝`vmsa_features` 蹇呴』涓洪浂锛屽洜涓哄畠浠病鏈?VMSA銆?
瀵逛簬 SEV 铏氭嫙鏈猴紝`ghcb_version` 蹇呴』涓洪浂锛屽洜涓哄畠浠笉鍙戝嚭 GHCB 璇锋眰銆傚鏋滃叾浠栦换浣曞鎴锋満绫诲瀷鐨?`ghcb_version` 涓洪浂锛屽垯鍏佽鐨勬渶澶у鎴锋満 GHCB 鍗忚灏嗛粯璁や娇鐢ㄧ増鏈?2銆?
姝ゅ懡浠ゅ彇浠ｄ簡宸插簾寮冪殑 KVM_SEV_INIT 鍜?KVM_SEV_ES_INIT 鍛戒护銆傝繖浜涘懡浠ゆ病鏈変换浣曞弬鏁帮紙``data`` 瀛楁鏈娇鐢級锛屽苟涓斾粎閫傜敤浜?KVM_X86_DEFAULT_VM 鏈哄櫒绫诲瀷锛?锛夈€?
瀹冧滑鐨勮涓哄鍚岋細

- KVM_SEV_INIT 鐨?VM 绫诲瀷涓?KVM_X86_SEV_VM锛孠VM_SEV_ES_INIT 涓?KVM_X86_SEV_ES_VM

- `struct kvm_sev_init` 鐨?`flags` 鍜?`vmsa_features` 瀛楁琚涓洪浂锛屼笖 KVM_SEV_INIT 鐨?`ghcb_version` 璁句负 0锛孠VM_SEV_ES_INIT 璁句负 1銆?
濡傛灉 `KVM_X86_SEV_VMSA_FEATURES` 灞炴€т笉瀛樺湪锛屽垯铏氭嫙鏈虹洃鎺у櫒浠呮敮鎸?KVM_SEV_INIT 鍜?KVM_SEV_ES_INIT銆傚湪姝ゆ儏鍐典笅锛岃娉ㄦ剰 KVM_SEV_ES_INIT 鍙兘浼氭牴鎹?`kvm-amd.ko` 鐨?`debug_swap` 鍙傛暟鐨勫€艰缃?debug swap VMSA 鐗规€э紙浣?5锛夈€?
### 2. KVM_SEV_LAUNCH_START


KVM_SEV_LAUNCH_START 鍛戒护鐢ㄤ簬鍒涘缓鍐呭瓨鍔犲瘑涓婁笅鏂囥€傝鍒涘缓鍔犲瘑涓婁笅鏂囷紝鐢ㄦ埛蹇呴』鎻愪緵瀹㈡埛鏈虹瓥鐣ャ€佹墍鏈夎€呯殑鍏挜 Diffie-Hellman锛圥DH锛夊瘑閽ュ拰浼氳瘽淇℃伅銆?
鍙傛暟锛歴truct kvm_sev_launch_start锛堣緭鍏?杈撳嚭锛?
杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_launch_start {
                __u32 handle;           /* if zero then firmware creates a new handle */
                __u32 policy;           /* guest's policy */

                __u64 dh_uaddr;         /* userspace address pointing to the guest owner's PDH key */
                __u32 dh_len;

                __u64 session_addr;     /* userspace address which points to the guest session information */
                __u32 session_len;
        };

```
鎴愬姛鏃讹紝'handle' 瀛楁鍖呭惈涓€涓柊鍙ユ焺锛涘嚭閿欐椂涓鸿礋鏁般€?
KVM_SEV_LAUNCH_START 瑕佹眰 `sev_fd` 瀛楁鏈夋晥銆?
鏇村缁嗚妭锛岃鍙傝 SEV 瑙勮寖绗?6.2 鑺傘€?
### 3. KVM_SEV_LAUNCH_UPDATE_DATA


KVM_SEV_LAUNCH_UPDATE_DATA 鐢ㄤ簬鍔犲瘑涓€涓唴瀛樺尯鍩熴€傚畠杩樹細璁＄畻鍐呭瓨鍐呭鐨勫害閲忓€硷紙measurement锛夈€傝搴﹂噺鏄唴瀛樺唴瀹圭殑绛惧悕锛屽彲浠ュ彂閫佺粰瀹㈡埛鏈烘墍鏈夎€咃紝浣滀负鍐呭瓨宸茶鍥轰欢姝ｇ‘鍔犲瘑鐨勮瘉鏄庯紙attestation锛夈€?
鍙傛暟锛堣緭鍏ワ級锛歴truct kvm_sev_launch_update_data

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_launch_update {
                __u64 uaddr;    /* userspace address to be encrypted (must be 16-byte aligned) */
                __u32 len;      /* length of the data to be encrypted (must be 16-byte aligned) */
        };

```
鏇村缁嗚妭锛岃鍙傝 SEV 瑙勮寖绗?6.3 鑺傘€?
### 4. KVM_SEV_LAUNCH_MEASURE


KVM_SEV_LAUNCH_MEASURE 鍛戒护鐢ㄤ簬鑾峰彇鐢?KVM_SEV_LAUNCH_UPDATE_DATA 鍛戒护鍔犲瘑鐨勬暟鎹殑搴﹂噺鍊笺€傚鎴锋満鎵€鏈夎€呭彲鑳戒細绛夊埌鑳藉楠岃瘉搴﹂噺鍊煎悗锛屾墠鍚戝鎴锋満鎻愪緵鏈哄瘑淇℃伅銆傜敱浜庡鎴锋満鎵€鏈夎€呭湪鍚姩鏃剁煡閬撳鎴锋満鐨勫垵濮嬪唴瀹癸紝鍥犳鍙互閫氳繃灏嗗害閲忓€间笌鍏舵湡鏈涚殑鍊艰繘琛屾瘮杈冩潵楠岃瘉銆?
濡傛灉杈撳叆鏃?len 涓洪浂锛屽垯浼氬皢搴﹂噺鍊?blob 鐨勯暱搴﹀啓鍏?len锛寀addr 涓嶈浣跨敤銆?
鍙傛暟锛堣緭鍏ワ級锛歴truct kvm_sev_launch_measure

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_launch_measure {
                __u64 uaddr;    /* where to copy the measurement */
                __u32 len;      /* length of measurement blob */
        };

```
鍏充簬搴﹂噺鍊奸獙璇佹祦绋嬬殑鏇村缁嗚妭锛岃鍙傝 SEV 瑙勮寖绗?6.4 鑺傘€?
### 5. KVM_SEV_LAUNCH_FINISH


鍚姩娴佺▼瀹屾垚鍚庯紝鍙互鍙戝嚭 KVM_SEV_LAUNCH_FINISH 鍛戒护锛屼娇瀹㈡埛鏈哄噯澶囧ソ鎵ц銆?
杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

### 6. KVM_SEV_GUEST_STATUS


KVM_SEV_GUEST_STATUS 鍛戒护鐢ㄤ簬鑾峰彇宸插惎鐢?SEV 鐨勫鎴锋満鐨勭姸鎬佷俊鎭€?
鍙傛暟锛堣緭鍑猴級锛歴truct kvm_sev_guest_status

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_guest_status {
                __u32 handle;   /* guest handle */
                __u32 policy;   /* guest policy */
                __u8 state;     /* guest state (see enum below) */
        };

```
SEV 瀹㈡埛鏈虹姸鎬侊細

```

        enum {
        SEV_STATE_INVALID = 0;
        SEV_STATE_LAUNCHING,    /* guest is currently being launched */
        SEV_STATE_SECRET,       /* guest is being launched and ready to accept the ciphertext data */
        SEV_STATE_RUNNING,      /* guest is fully launched and running */
        SEV_STATE_RECEIVING,    /* guest is being migrated in from another SEV machine */
        SEV_STATE_SENDING       /* guest is getting migrated out to another SEV machine */
        };

```
### 7. KVM_SEV_DBG_DECRYPT


铏氭嫙鏈虹洃鎺у櫒鍙互浣跨敤 KVM_SEV_DEBUG_DECRYPT 鍛戒护璇锋眰鍥轰欢瑙ｅ瘑缁欏畾鍐呭瓨鍖哄煙鐨勬暟鎹€?
鍙傛暟锛堣緭鍏ワ級锛歴truct kvm_sev_dbg

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_dbg {
                __u64 src_uaddr;        /* userspace address of data to decrypt */
                __u64 dst_uaddr;        /* userspace address of destination */
                __u32 len;              /* length of memory region to decrypt */
        };

```
濡傛灉瀹㈡埛鏈虹瓥鐣ヤ笉鍏佽璋冭瘯锛岃鍛戒护浼氳繑鍥為敊璇€?
### 8. KVM_SEV_DBG_ENCRYPT


铏氭嫙鏈虹洃鎺у櫒鍙互浣跨敤 KVM_SEV_DEBUG_ENCRYPT 鍛戒护璇锋眰鍥轰欢鍔犲瘑缁欏畾鍐呭瓨鍖哄煙鐨勬暟鎹€?
鍙傛暟锛堣緭鍏ワ級锛歴truct kvm_sev_dbg

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_dbg {
                __u64 src_uaddr;        /* userspace address of data to encrypt */
                __u64 dst_uaddr;        /* userspace address of destination */
                __u32 len;              /* length of memory region to encrypt */
        };

```
濡傛灉瀹㈡埛鏈虹瓥鐣ヤ笉鍏佽璋冭瘯锛岃鍛戒护浼氳繑鍥為敊璇€?
### 9. KVM_SEV_LAUNCH_SECRET


铏氭嫙鏈虹洃鎺у櫒鍙互浣跨敤 KVM_SEV_LAUNCH_SECRET 鍛戒护鍦ㄥ害閲忓€煎凡琚鎴锋満鎵€鏈夎€呴獙璇佸悗娉ㄥ叆鏈哄瘑鏁版嵁銆?
鍙傛暟锛堣緭鍏ワ級锛歴truct kvm_sev_launch_secret

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_launch_secret {
                __u64 hdr_uaddr;        /* userspace address containing the packet header */
                __u32 hdr_len;

                __u64 guest_uaddr;      /* the guest memory region where the secret should be injected */
                __u32 guest_len;

                __u64 trans_uaddr;      /* the hypervisor memory region which contains the secret */
                __u32 trans_len;
        };

```
### 10. KVM_SEV_GET_ATTESTATION_REPORT


铏氭嫙鏈虹洃鎺у櫒鍙互浣跨敤 KVM_SEV_GET_ATTESTATION_REPORT 鍛戒护鏌ヨ璇佹槑锛坅ttestation锛夋姤鍛婏紝璇ユ姤鍛婂寘鍚€氳繃 KVM_SEV_LAUNCH 鍛戒护浼犲叆鐨勫鎴锋満鍐呭瓨鍜?VMSA 鐨?SHA-256 鎽樿锛屽苟鐢?PEK 绛惧悕銆傝鍛戒护杩斿洖鐨勬憳瑕佸簲涓庡鎴锋満鎵€鏈夎€呴€氳繃 KVM_SEV_LAUNCH_MEASURE 浣跨敤鐨勬憳瑕佺浉鍖归厤銆?
濡傛灉杈撳叆鏃?len 涓洪浂锛屽垯浼氬皢搴﹂噺鍊?blob 鐨勯暱搴﹀啓鍏?len锛寀addr 涓嶈浣跨敤銆?
鍙傛暟锛堣緭鍏ワ級锛歴truct kvm_sev_attestation

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_attestation_report {
                __u8 mnonce[16];        /* A random mnonce that will be placed in the report */

                __u64 uaddr;            /* userspace address where the report should be copied */
                __u32 len;
        };

```
### 11. KVM_SEV_SEND_START


铏氭嫙鏈虹洃鎺у櫒鍙互浣跨敤 KVM_SEV_SEND_START 鍛戒护鍒涘缓澶栧嚭鐨勫鎴锋満鍔犲瘑涓婁笅鏂囥€?
濡傛灉杈撳叆鏃?session_len 涓洪浂锛屽垯浼氬皢瀹㈡埛鏈轰細璇濅俊鎭殑闀垮害鍐欏叆 session_len锛屽叾浠栨墍鏈夊瓧娈典笉琚娇鐢ㄣ€?
鍙傛暟锛堣緭鍏ワ級锛歴truct kvm_sev_send_start

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_send_start {
                __u32 policy;                 /* guest policy */

                __u64 pdh_cert_uaddr;         /* platform Diffie-Hellman certificate */
                __u32 pdh_cert_len;

                __u64 plat_certs_uaddr;        /* platform certificate chain */
                __u32 plat_certs_len;

                __u64 amd_certs_uaddr;        /* AMD certificate */
                __u32 amd_certs_len;

                __u64 session_uaddr;          /* Guest session information */
                __u32 session_len;
        };

```
### 12. KVM_SEV_SEND_UPDATE_DATA


铏氭嫙鏈虹洃鎺у櫒鍙互浣跨敤 KVM_SEV_SEND_UPDATE_DATA 鍛戒护锛屼娇鐢?KVM_SEV_SEND_START 鍒涘缓鐨勫姞瀵嗕笂涓嬫枃鏉ュ姞瀵嗗鍑虹殑瀹㈡埛鏈哄唴瀛樺尯鍩熴€?
濡傛灉杈撳叆鏃?hdr_len 鎴?trans_len 涓洪浂锛屽垯浼氬皢鍖呭ご鍜屼紶杈撳尯鍩熺殑闀垮害鍒嗗埆鍐欏叆 hdr_len 鍜?trans_len锛屽叾浠栨墍鏈夊瓧娈典笉琚娇鐢ㄣ€?
鍙傛暟锛堣緭鍏ワ級锛歴truct kvm_sev_send_update_data

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_launch_send_update_data {
                __u64 hdr_uaddr;        /* userspace address containing the packet header */
                __u32 hdr_len;

                __u64 guest_uaddr;      /* the source memory region to be encrypted */
                __u32 guest_len;

                __u64 trans_uaddr;      /* the destination memory region  */
                __u32 trans_len;
        };

```
### 13. KVM_SEV_SEND_FINISH


杩佺Щ娴佺▼瀹屾垚鍚庯紝铏氭嫙鏈虹洃鎺у櫒鍙互鍙戝嚭 KVM_SEV_SEND_FINISH 鍛戒护鏉ュ垹闄ゅ姞瀵嗕笂涓嬫枃銆?
杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

### 14. KVM_SEV_SEND_CANCEL


鍦ㄥ畬鎴?SEND_START 涔嬪悗銆丼END_FINISH 涔嬪墠锛屾簮 VMM 鍙互鍙戝嚭 SEND_CANCEL 鍛戒护鏉ュ仠姝㈣縼绉汇€傝繖鏄繀瑕佺殑锛屼互渚胯鍙栨秷鐨勮縼绉荤◢鍚庡彲浠ヤ娇鐢ㄦ柊鐨勭洰鏍囬噸鏂板惎鍔ㄣ€?
杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

### 15. KVM_SEV_RECEIVE_START


KVM_SEV_RECEIVE_START 鍛戒护鐢ㄤ簬涓鸿繘鍏ョ殑 SEV 瀹㈡埛鏈哄垱寤哄唴瀛樺姞瀵嗕笂涓嬫枃銆傝鍒涘缓鍔犲瘑涓婁笅鏂囷紝鐢ㄦ埛蹇呴』鎻愪緵瀹㈡埛鏈虹瓥鐣ャ€佸钩鍙板叕閽?Diffie-Hellman锛圥DH锛夊瘑閽ュ拰浼氳瘽淇℃伅銆?
鍙傛暟锛歴truct kvm_sev_receive_start锛堣緭鍏?杈撳嚭锛?
杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_receive_start {
                __u32 handle;           /* if zero then firmware creates a new handle */
                __u32 policy;           /* guest's policy */

                __u64 pdh_uaddr;        /* userspace address pointing to the PDH key */
                __u32 pdh_len;

                __u64 session_uaddr;    /* userspace address which points to the guest session information */
                __u32 session_len;
        };

```
鎴愬姛鏃讹紝'handle' 瀛楁鍖呭惈涓€涓柊鍙ユ焺锛涘嚭閿欐椂涓鸿礋鏁般€?
鏇村缁嗚妭锛岃鍙傝 SEV 瑙勮寖绗?6.12 鑺傘€?
### 16. KVM_SEV_RECEIVE_UPDATE_DATA


铏氭嫙鏈虹洃鎺у櫒鍙互浣跨敤 KVM_SEV_RECEIVE_UPDATE_DATA 鍛戒护锛屽皢杩涘叆鐨勭紦鍐插尯澶嶅埗鍒板湪 KVM_SEV_RECEIVE_START 鏈熼棿鍒涘缓浜嗗姞瀵嗕笂涓嬫枃鐨勫鎴锋満鍐呭瓨鍖哄煙銆?
鍙傛暟锛堣緭鍏ワ級锛歴truct kvm_sev_receive_update_data

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_launch_receive_update_data {
                __u64 hdr_uaddr;        /* userspace address containing the packet header */
                __u32 hdr_len;

                __u64 guest_uaddr;      /* the destination guest memory region */
                __u32 guest_len;

                __u64 trans_uaddr;      /* the incoming buffer memory region  */
                __u32 trans_len;
        };

```
### 17. KVM_SEV_RECEIVE_FINISH


杩佺Щ娴佺▼瀹屾垚鍚庯紝铏氭嫙鏈虹洃鎺у櫒鍙互鍙戝嚭 KVM_SEV_RECEIVE_FINISH 鍛戒护浣垮鎴锋満鍑嗗濂芥墽琛屻€?
杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

### 18. KVM_SEV_SNP_LAUNCH_START


KVM_SNP_LAUNCH_START 鍛戒护鐢ㄤ簬涓?SEV-SNP 瀹㈡埛鏈哄垱寤哄唴瀛樺姞瀵嗕笂涓嬫枃銆傚繀椤诲湪鍙戝嚭 KVM_SEV_SNP_LAUNCH_UPDATE 鎴?KVM_SEV_SNP_LAUNCH_FINISH 涔嬪墠璋冪敤瀹冿紱

鍙傛暟锛堣緭鍏ワ級锛歴truct kvm_sev_snp_launch_start

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_snp_launch_start {
                __u64 policy;           /* Guest policy to use. */
                __u8 gosvw[16];         /* Guest OS visible workarounds. */
                __u16 flags;            /* Must be zero. */
                __u8 pad0[6];
                __u64 pad1[4];
        };

```
鍏充簬 `struct kvm_sev_snp_launch_start` 涓緭鍏ュ弬鏁扮殑鏇村缁嗚妭锛岃鍙傝 SEV-SNP 瑙勮寖 [snp-fw-abi]_ 涓殑 SNP_LAUNCH_START銆?
### 19. KVM_SEV_SNP_LAUNCH_UPDATE


KVM_SEV_SNP_LAUNCH_UPDATE 鍛戒护鐢ㄤ簬灏嗙敤鎴风┖闂存彁渚涚殑鏁版嵁鍔犺浇鍒板鎴锋満 GPA 鑼冨洿涓紝灏嗗唴瀹瑰害閲忓埌鐢?KVM_SEV_SNP_LAUNCH_START 鍒涘缓鐨?SNP 瀹㈡埛鏈轰笂涓嬫枃涓紝鐒跺悗瀵硅 GPA 鑼冨洿杩涜鍔犲瘑/楠岃瘉锛屼娇鍏跺湪鍚姩鍚庡嵆鍙娇鐢ㄤ笌璇ュ鎴锋満涓婁笅鏂囧叧鑱旂殑鍔犲瘑瀵嗛挜鐩存帴璇诲彇锛涙鍚庯紝瀹冨彲浠ュ湪瑙ｉ攣浠讳綍鏈哄瘑涔嬪墠锛屽鍏朵笂涓嬫枃鍏宠仈鐨勫害閲忓€艰繘琛岃瘉鏄庯紙attest锛夈€?
姝ゅ懡浠ゅ垵濮嬪寲鐨?GPA 鑼冨洿蹇呴』浜嬪厛璁剧疆 KVM_MEMORY_ATTRIBUTE_PRIVATE 灞炴€с€傚叧浜庤繖鏂归潰鐨勬洿澶氱粏鑺傦紝璇峰弬瑙?KVM_SET_MEMORY_ATTRIBUTES 鐨勬枃妗ｃ€?
鎴愬姛鏃讹紝涓嶈兘淇濊瘉姝ゅ懡浠ゅ凡澶勭悊鎵€璇锋眰鐨勬暣涓寖鍥淬€傜浉鍙嶏紝`struct kvm_sev_snp_launch_update` 鐨?`gfn_start`銆乣uaddr` 鍜?`len` 瀛楁浼氳鏇存柊涓哄搴斾簬灏氭湭澶勭悊鐨勫墿浣欒寖鍥淬€傝皟鐢ㄨ€呭簲缁х画璋冪敤姝ゅ懡浠わ紝鐩村埌杩欎簺瀛楁琛ㄦ槑鏁翠釜鑼冨洿宸插鐞嗗畬姣曪紝渚嬪 `len` 涓?0锛宍gfn_start` 绛変簬鑼冨洿涓渶鍚庝竴涓?GFN 鍔?1锛屼笖 `uaddr` 涓虹敤鎴风┖闂存彁渚涚殑婧愮紦鍐插尯鍦板潃鐨勬渶鍚庝竴涓瓧鑺傚姞 1銆傚湪 `type` 涓?KVM_SEV_SNP_PAGE_TYPE_ZERO 鐨勬儏鍐典笅锛宍uaddr` 灏嗚瀹屽叏蹇界暐銆?
鍙傛暟锛堣緭鍏ワ級锛歴truct kvm_sev_snp_launch_update

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 < 0锛岄渶瑕佽皟鐢ㄨ€呴噸璇曟椂 -EAGAIN

```

        struct kvm_sev_snp_launch_update {
                __u64 gfn_start;        /* Guest page number to load/encrypt data into. */
                __u64 uaddr;            /* 4k-aligned address of data to be loaded/encrypted. */
                __u64 len;              /* 4k-aligned length in bytes to copy into guest memory.*/
                __u8 type;              /* The type of the guest pages being initialized. */
                __u8 pad0;
                __u16 flags;            /* Must be zero. */
                __u32 pad1;
                __u64 pad2[4];

        };

```

```

        KVM_SEV_SNP_PAGE_TYPE_NORMAL
        KVM_SEV_SNP_PAGE_TYPE_ZERO
        KVM_SEV_SNP_PAGE_TYPE_UNMEASURED
        KVM_SEV_SNP_PAGE_TYPE_SECRETS
        KVM_SEV_SNP_PAGE_TYPE_CPUID

```
鍏充簬姣忕椤甸潰绫诲瀷濡備綍琚娇鐢?搴﹂噺锛岃鍙傝 SEV-SNP 瑙勮寖 [snp-fw-abi]_銆?
### 20. KVM_SEV_SNP_LAUNCH_FINISH


SNP 瀹㈡埛鏈哄惎鍔ㄦ祦绋嬪畬鎴愬悗锛屽彲浠ュ彂鍑?KVM_SEV_SNP_LAUNCH_FINISH 鍛戒护浣垮鎴锋満鍑嗗濂芥墽琛屻€?
鍙傛暟锛堣緭鍏ワ級锛歴truct kvm_sev_snp_launch_finish

杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

```

        struct kvm_sev_snp_launch_finish {
                __u64 id_block_uaddr;
                __u64 id_auth_uaddr;
                __u8 id_block_en;
                __u8 auth_key_en;
                __u8 vcek_disabled;
                __u8 host_data[32];
                __u8 pad0[3];
                __u16 flags;                    /* Must be zero */
                __u64 pad1[4];
        };


```
鍏充簬 `struct kvm_sev_snp_launch_finish` 涓緭鍏ュ弬鏁扮殑鏇村缁嗚妭锛岃鍙傝 SEV-SNP 瑙勮寖 [snp-fw-abi]_ 涓殑 SNP_LAUNCH_FINISH銆?
### 21. KVM_SEV_SNP_ENABLE_REQ_CERTS


KVM_SEV_SNP_ENABLE_REQ_CERTS 鍛戒护浼氬皢 KVM 閰嶇疆涓哄湪澶勭悊瀹㈡埛鏈鸿瘉鏄庢姤鍛婃椂锛屼互 `KVM_EXIT_SNP_REQ_CERTS` 閫€鍑虹被鍨嬮€€鍑哄埌鐢ㄦ埛绌洪棿锛屼粠鑰屽厑璁哥敤鎴风┖闂存彁渚涗笌鍥轰欢鐢ㄤ簬绛剧讲璇ヨ瘉鏄庢姤鍛婄殑鑳屼功瀵嗛挜锛坋ndorsement key锛夌浉瀵瑰簲鐨勮瘉涔︺€?
杩斿洖鍊硷細鎴愬姛鏃?0锛屽嚭閿欐椂 -璐熸暟

娉ㄦ剰锛氬浐浠朵娇鐢ㄧ殑鑳屼功瀵嗛挜鍙兘浼氬洜涓烘洿鏂?SEV-SNP 鍥轰欢鎴栧姞杞芥柊鐨勮儗涔﹀瘑閽ョ瓑绠＄悊娲诲姩鑰屾敼鍙橈紝鍥犳闇€瑕佸皬蹇冪‘淇濊繑鍥炵殑璇佷功鏁版嵁涓庡彂閫佽瘉鏄庤姹傛椂鍥轰欢瀹為檯浣跨敤鐨勮儗涔﹀瘑閽ヤ繚鎸佸悓姝ャ€傚缓璁殑鏂规鏄娇鐢ㄦ枃浠堕攣锛堜緥濡傞€氳繃 fcntl() 鐨?F_OFD_SETLK锛夛紝鏂瑰紡濡備笅锛?
  - 鍦ㄤ綔涓哄鐞?`KVM_EXIT_SNP_REQ_CERTS` 閫€鍑虹被鍨嬬殑涓€閮ㄥ垎鑰岃幏鍙?鎻愪緵璇佷功鏁版嵁涔嬪墠锛孷MM 搴斿湪璇诲彇璇佷功 blob 鏂囦欢骞跺皢鍏惰繑鍥炵粰 KVM 涔嬪墠锛岃幏鍙栬鏂囦欢涓婄殑鍏变韩/璇婚攣鎴栫嫭鍗?鍐欓攣锛屽苟缁х画鎸佹湁璇ラ攣锛岀洿鍒拌瘉鏄庤姹傚疄闄呭彂閫佸埌鍥轰欢銆備负鏂逛究璧疯锛孷MM 鍙互鍦ㄦ彁渚涜瘉涔︽暟鎹箣鍚庛€佹仮澶?vCPU 涔嬪墠锛岃缃?kvm_run 鐨?`immediate_exit` 鏍囧織銆傝繖灏嗙‘淇?vCPU 鍦ㄤ粠鍥轰欢鍙栧洖璇佹槑璇锋眰鍚庝細浠?`-EINTR` 鍐嶆閫€鍑哄埌鐢ㄦ埛绌洪棿锛屾鏃?VMM 鍙互瀹夊叏鍦伴噴鏀炬枃浠堕攣銆?
  - 瀵?SNP 鍥轰欢 TCB 鍊兼垨鑳屼功瀵嗛挜鎵ц鏇存柊锛堜緥濡傞€氳繃 `/dev/sev` 鎺ュ彛濡?`SNP_COMMIT`銆乣SNP_SET_CONFIG` 鎴?`SNP_VLEK_LOAD`锛屾洿澶氱粏鑺傝鍙傝 Documentation/virt/coco/sev-guest.rst锛変笖闇€瑕佹洿鏂拌瘉涔?blob 鐨勫伐鍏?搴擄紝鍚屾牱搴斿湪浠讳綍瀵硅儗涔﹀瘑閽ユ垨璇佷功 blob 鍐呭鐨勬洿鏂版湡闂村璇佷功 blob 鎸佹湁鐙崰閿侊紝浠ョ‘淇濅娇鐢ㄤ笂杩版柟妗堢殑 VMM 涓嶄細杩斿洖涓庤瘉鏄庤姹傚疄闄呭彂鍑烘椂鍥轰欢浣跨敤鐨勮儗涔﹀瘑閽ヤ笉鍚屾鐨勮瘉涔?blob 鏁版嵁銆?
鎺ㄨ崘姝ゆ柟妗堬紝浠ヤ究宸ュ叿鍙互浣跨敤鐩稿綋閫氱敤/鑷劧鐨勬柟娉曢€氳繃鏂囦欢閿佹潵鍚屾鍥轰欢/璇佷功鏇存柊锛屼粠鑰屾洿瀹规槗鍦ㄥ伐鍏?VMM/渚涘簲鍟嗕箣闂翠繚鎸佷簰鎿嶄綔鎬с€?
## 璁惧灞炴€?API


SEV 瀹炵幇鐨勫睘鎬у彲浠ラ€氳繃 `/dev/kvm` 璁惧鑺傜偣涓婄殑 `KVM_HAS_DEVICE_ATTR` 鍜?`KVM_GET_DEVICE_ATTR` ioctl锛屼娇鐢ㄧ粍 `KVM_X86_GRP_SEV` 鏉ヨ幏鍙栥€?
褰撳墠瀹炵幇浜嗕互涓嬪睘鎬э細

- `KVM_X86_SEV_VMSA_FEATURES`锛氳繑鍥?`KVM_SEV_INIT2` 鐨?`vmsa_features` 涓鎺ュ彈鐨勬墍鏈変綅鐨勯泦鍚堛€?
- `KVM_X86_SEV_SNP_REQ_CERTS`锛氬鏋滃唴鏍告敮鎸?`KVM_EXIT_SNP_REQ_CERTS` 閫€鍑猴紝鍒欒繑鍥?1锛涜閫€鍑哄厑璁镐负姣忎釜 SNP 璇佹槑璇锋眰浠庣敤鎴风┖闂磋幏鍙栬儗涔﹀瘑閽ヨ瘉涔︺€?
## 鍥轰欢绠＄悊


SEV 瀹㈡埛鏈哄瘑閽ョ鐞嗙敱涓€涓О涓?AMD 瀹夊叏澶勭悊鍣紙AMD-SP锛夌殑鐙珛澶勭悊鍣ㄥ鐞嗐€傝繍琛屽湪 AMD-SP 鍐呴儴鐨勫浐浠舵彁渚涗簡涓€涓畨鍏ㄧ殑瀵嗛挜绠＄悊鎺ュ彛锛岀敤浜庢墽琛屽父瑙佺殑铏氭嫙鏈虹洃鎺у櫒娲诲姩锛屼緥濡傚姞瀵嗗紩瀵间唬鐮併€佸揩鐓с€佽縼绉诲拰璋冭瘯瀹㈡埛鏈恒€傛洿澶氫俊鎭鍙傝 SEV 瀵嗛挜绠＄悊瑙勮寖 [api-spec]_

AMD-SP 鍥轰欢鍙互閫氳繃鍏惰嚜韬殑闈炴槗澶辨€у瓨鍌ㄥ垵濮嬪寲锛屾垨鑰呮搷浣滅郴缁熷彲浠ヤ娇鐢?`ccp` 妯″潡鐨?`init_ex_path` 鍙傛暟鏉ョ鐞嗗浐浠剁殑 NV 瀛樺偍銆傚鏋?`init_ex_path` 鎸囧畾鐨勬枃浠朵笉瀛樺湪鎴栨棤鏁堬紝鎿嶄綔绯荤粺灏嗙敤 PSP 闈炴槗澶辨€у瓨鍌ㄥ垱寤烘垨瑕嗙洊璇ユ枃浠躲€?
## 鍙傝€?

鏇村淇℃伅璇峰弬瑙?[white-paper]_銆乕api-spec]_銆乕amd-apm]_銆乕kvm-forum]_ 鍜?[snp-fw-abi]_銆?