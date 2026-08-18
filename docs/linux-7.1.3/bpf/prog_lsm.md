
## LSM BPF 绋嬪簭


杩欎簺 BPF 绋嬪簭鍏佽鐗规潈鐢ㄦ埛鍦ㄨ繍琛屾椂瀵?LSM 閽╁瓙锛坔ook锛夎繘琛屾彃妗╋紙instrumentation锛夛紝浠ヤ娇鐢?eBPF 瀹炵幇绯荤粺绾х殑 MAC锛堝己鍒惰闂帶鍒讹紝Mandatory Access Control锛夊拰瀹¤锛圓udit锛夌瓥鐣ャ€?
### 缁撴瀯


绀轰緥灞曠ず浜嗕竴涓彲浠ラ檮鍔犲埌 `file_mprotect` LSM 閽╁瓙鐨?eBPF 绋嬪簭锛?

鍏朵粬鍙鎻掓々鐨?LSM 閽╁瓙鍙湪 `security/security.c` 涓壘鍒般€?
浣跨敤 Documentation/bpf/btf.rst 鐨?eBPF 绋嬪簭鏃犻渶鍖呭惈鍐呮牳澶存枃浠跺嵆鍙闂墍闄勫姞 eBPF 绋嬪簭涓婁笅鏂囦腑鐨勪俊鎭€傚畠浠彧闇€鍦?eBPF 绋嬪簭涓０鏄庤繖浜涚粨鏋勶紝骞朵粎鎸囧畾闇€瑕佽闂殑瀛楁鍗冲彲銆?

	struct mm_struct {
		unsigned long start_brk, brk, start_stack;
	} __attribute__((preserve_access_index));

	struct vm_area_struct {
		unsigned long start_brk, brk, start_stack;
		unsigned long vm_start, vm_end;
		struct mm_struct *vm_mm;
	} __attribute__((preserve_access_index));



濡傛灉锛堝湪鏋勫缓鏃跺彲璁块棶 BTF 淇℃伅锛夐€氳繃浠ヤ笅鍛戒护鐢熸垚 `vmlinux.h`锛屽垯鍙繘涓€姝ョ畝鍖栵細


	# bpftool btf dump file <path-to-btf-vmlinux> format c > vmlinux.h

	  鏋勫缓鐜涓?BPF 绋嬪簭閮ㄧ讲鐨勭幆澧冪浉鍖归厤銆?
鐒跺悗鍙渶鍦?BPF 绋嬪簭涓寘鍚?`vmlinux.h`锛岃€屾棤闇€瀹氫箟杩欎簺绫诲瀷銆?
eBPF 绋嬪簭鍙互浣跨敤 `tools/lib/bpf/bpf_tracing.h`_ 涓畾涔夌殑 `BPF_PROG` 瀹忓０鏄庛€傚湪鏈緥涓細

 - `"lsm/file_mprotect"` 琛ㄧず璇ョ▼搴忓繀椤婚檮鍔犲埌鐨?LSM 閽╁瓙
 - `mprotect_audit` 鏄?eBPF 绋嬪簭鐨勫悕绉?

	SEC("lsm/file_mprotect")
	int BPF_PROG(mprotect_audit, struct vm_area_struct *vma,
		     unsigned long reqprot, unsigned long prot, int ret)
	{
		/* ret 鏄墠涓€涓?BPF 绋嬪簭鐨勮繑鍥炲€?   - 濡傛灉鏄涓€涓挬瀛愬垯涓?0銆?		 */
		if (ret != 0)
			return ret;

		int is_heap;

		is_heap = (vma->vm_start >= vma->vm_mm->start_brk &&
			   vma->vm_end <= vma->vm_mm->brk);

		/* 杩斿洖 -EPERM锛屾垨鍚?perf 浜嬩欢缂撳啿鍖哄啓鍏ヤ俊鎭?   - 鐢ㄤ簬瀹¤
		 */
		if (is_heap)
			return -EPERM;
	}

`__attribute__((preserve_access_index))` 鏄?clang 鐨勪竴涓壒鎬э紝鍏佽 BPF 楠岃瘉鍣紙verifier锛夊湪杩愯鏃朵娇鐢?Documentation/bpf/btf.rst 淇℃伅鏇存柊璁块棶鐨勫亸绉婚噺銆傜敱浜?BPF 楠岃瘉鍣ㄤ簡瑙ｈ繖浜涚被鍨嬶紝瀹冭繕浼氶獙璇?eBPF 绋嬪簭涓鍚勭绫诲瀷鐨勬墍鏈夎闂€?
### 鍔犺浇


eBPF 绋嬪簭鍙互閫氳繃 `bpf(2)` 绯荤粺璋冪敤鐨?`BPF_PROG_LOAD` 鎿嶄綔鍔犺浇锛?

	struct bpf_object *obj;

	obj = bpf_object__open("./my_prog.o");
	bpf_object__load(obj);

浣跨敤 `bpftool` 鐢熸垚鐨?skeleton 澶存枃浠跺彲浠ョ畝鍖栬繖涓€杩囩▼锛?

	# bpftool gen skeleton my_prog.o > my_prog.skel.h

绋嬪簭鍙互閫氳繃鍖呭惈 `my_prog.skel.h` 骞朵娇鐢ㄧ敓鎴愮殑杈呭姪鍑芥暟 `my_prog__open_and_load` 鏉ュ姞杞姐€?
### 闄勫姞鍒?LSM 閽╁瓙


LSM 鍏佽浣跨敤 `bpf(2)` 绯荤粺璋冪敤鐨?`BPF_RAW_TRACEPOINT_OPEN` 鎿嶄綔灏?eBPF 绋嬪簭浣滀负 LSM 閽╁瓙闄勫姞锛屾垨鑰呮洿绠€鍗曞湴浣跨敤 libbpf 杈呭姪鍑芥暟 `bpf_program__attach_lsm`銆?
鍙互閫氳繃**閿€姣?*锛坉estroying锛塦bpf_program__attach_lsm` 杩斿洖鐨?`link` 閾炬帴锛堜娇鐢?`bpf_link__destroy`锛夊皢绋嬪簭浠?LSM 閽╁瓙鍒嗙銆?
涔熷彲浠ヤ娇鐢?`my_prog.skel.h` 涓敓鎴愮殑杈呭姪鍑芥暟锛屽嵆 `my_prog__attach` 鐢ㄤ簬闄勫姞銆乣my_prog__destroy` 鐢ㄤ簬娓呯悊銆?
### 绀轰緥


绀轰緥 eBPF 绋嬪簭鍙湪 `tools/testing/selftests/bpf/progs/lsm.c`_ 涓壘鍒帮紝鐩稿簲鐨勭敤鎴锋€佷唬鐮佸湪 `tools/testing/selftests/bpf/prog_tests/test_lsm.c`_

   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/tools/lib/bpf/bpf_tracing.h
   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/tools/testing/selftests/bpf/progs/lsm.c
   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/tools/testing/selftests/bpf/prog_tests/test_lsm.c
