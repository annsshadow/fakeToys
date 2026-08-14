
## Nitro Enclaves锛圢E锛孨itro 椋炲湴锛?

## 姒傝堪


Nitro Enclaves锛圢E锛夋槸 Amazon Elastic Compute Cloud锛圗C2锛夌殑涓€椤规柊鑳藉姏锛?鍏佽瀹㈡埛鍦?EC2 瀹炰緥涓垏鍒嗗嚭闅旂鐨勮绠楃幆澧?[^1^]銆?
渚嬪锛屼竴涓鐞嗘晱鎰熸暟鎹苟杩愯鍦?VM 涓殑搴旂敤绋嬪簭锛屽彲浠ヤ笌杩愯鍦ㄥ悓涓€涓?VM 涓殑鍏朵粬搴旂敤绋嬪簭鍒嗙寮€鏉ャ€傜劧鍚庤搴旂敤绋嬪簭杩愯鍦ㄤ竴涓嫭绔嬬殑 VM 涓紝
鑰屼笉鏄富 VM锛屽嵆涓€涓?enclave锛堥鍦帮級銆傚畠涓庣敓鎴愬畠鐨?VM 骞惰杩愯銆傝繖绉嶈缃?绗﹀悎浣庡欢杩熷簲鐢ㄧ▼搴忕殑闇€姹傘€?
涓婃父 Linux 鍐呮牳涓彲鐢ㄧ殑 NE 鍐呮牳椹卞姩褰撳墠鏀寔鐨勬灦鏋勬槸 x86 鍜?ARM64銆?
涓?enclave 鍒嗛厤鐨勮祫婧愶紝渚嬪鍐呭瓨鍜?CPU锛屾槸浠庝富 VM 涓垏鍒嗗嚭鏉ョ殑銆傛瘡涓?enclave 鏄犲皠鍒颁富 VM 涓繍琛岀殑涓€涓繘绋嬶紝璇ヨ繘绋嬮€氳繃 ioctl 鎺ュ彛涓?NE 鍐呮牳椹卞姩
閫氫俊銆?
浠庤繖涓剰涔変笂璇达紝鏈変袱涓粍浠讹細

1. 涓€涓?enclave 鎶借薄杩涚▼鈥斺€旇繍琛屽湪涓?VM 瀹㈡埛鏈轰腑鐨勭敤鎴风┖闂磋繘绋嬶紝瀹冧娇鐢?   NE 椹卞姩鎻愪緵鐨?ioctl 鎺ュ彛鏉ョ敓鎴愪竴涓?enclave VM锛堝嵆涓嬮潰鐨?2锛夈€?
   鏈変竴涓毚闇茬粰涓?VM 鐨?NE 浠跨湡 PCI 璁惧銆傝鏂?PCI 璁惧鐨勯┍鍔ㄥ寘鍚湪
   NE 椹卞姩涓€?
   ioctl 閫昏緫鏄犲皠鍒?PCI 璁惧鍛戒护锛屼緥濡?NE_START_ENCLAVE ioctl 鏄犲皠鍒?   涓€涓?enclave 鍚姩 PCI 鍛戒护銆傜劧鍚?PCI 璁惧鍛戒护琚浆鎹负鍦ㄧ鐞嗙▼搴忎竴渚?   閲囧彇鐨勫姩浣滐紱鍗宠繍琛屼富 VM 鎵€鍦ㄤ富鏈轰笂鐨?Nitro 绠＄悊绋嬪簭銆侼itro 绠＄悊绋嬪簭
   鍩轰簬鏍稿績 KVM 鎶€鏈€?
2. enclave 鏈韩鈥斺€斾竴涓繍琛屽湪涓庣敓鎴愬畠鐨勪富 VM 鐩稿悓涓绘満涓婄殑 VM銆傚唴瀛樺拰
   CPU 浠庝富 VM 涓垏鍒嗗嚭鏉ワ紝骞朵笓鐢ㄤ簬 enclave VM銆俥nclave 娌℃湁闄勫姞鐨勬寔涔呭瓨鍌ㄣ€?
浠庝富 VM 涓垏鍒嗗嚭鏉ュ苟缁?enclave 鐨勫唴瀛樺尯鍩熼渶瑕佹槸瀵归綈鐨?2 MiB / 1 GiB 鐗╃悊
杩炵画鍐呭瓨鍖哄煙锛堟垨姝ゅぇ灏忕殑鍊嶆暟锛屼緥濡?8 MiB锛夈€傚唴瀛樺彲浠ラ€氳繃渚嬪浠庣敤鎴风┖闂翠娇鐢?hugetlbfs 鏉ュ垎閰?[^2^][^3^][^7^]銆俥nclave 鐨勫唴瀛樺ぇ灏忚嚦灏戦渶瑕?64 MiB銆?enclave 鐨勫唴瀛樺拰 CPU 闇€瑕佹潵鑷悓涓€涓?NUMA 鑺傜偣銆?
enclave 杩愯鍦ㄤ笓鐢ㄦ牳蹇冧笂銆侰PU 0 鍙婂叾 CPU 鍏勫紵锛坰ibling锛夐渶瑕佷繚鐣欑粰涓?VM
鍙敤銆傚繀椤荤敱鍏锋湁绠＄悊鍛樿兘鍔涚殑鐢ㄦ埛涓?NE 鐩殑璁剧疆涓€涓?CPU 姹犮€傛湁鍏?CPU 姹犳牸寮忥紝
璇峰弬瑙佸唴鏍告枃妗?[^4^] 涓殑 cpu 鍒楄〃涓€鑺傘€?
enclave 閫氳繃鏈湴閫氫俊閫氶亾浣跨敤 virtio-vsock [^5^] 涓庝富 VM 閫氫俊銆備富 VM 鏈?virtio-pci vsock 浠跨湡璁惧锛岃€?enclave VM 鏈?virtio-mmio vsock 浠跨湡璁惧銆?vsock 璁惧浣跨敤 eventfd 杩涜淇″彿閫氱煡銆俥nclave VM 鐪嬪埌閫氬父鐨勬帴鍙ｂ€斺€旀湰鍦?APIC
鍜?IOAPIC鈥斺€斾互浠?virtio-vsock 璁惧鑾峰彇涓柇銆倂irtio-mmio 璁惧琚斁缃湪鍏稿瀷
4 GiB 浠ヤ笅鐨勭殑鍐呭瓨涓€?
鍦?enclave 涓繍琛岀殑搴旂敤绋嬪簭闇€瑕佷笌鍏跺皢鍦?enclave VM 涓繍琛岀殑 OS锛堜緥濡傚唴鏍搞€?ramdisk銆乮nit锛変竴璧锋墦鍖呰繘涓€涓?enclave 闀滃儚涓€俥nclave VM 鏈夎嚜宸辩殑鍐呮牳骞堕伒寰?鏍囧噯 Linux 鍚姩鍗忚 [^6^][^8^]銆?
鍐呮牳 bzImage銆佸唴鏍稿懡浠よ銆乺amdisk(s) 鏄?Enclave Image Format锛圗IF锛宔nclave
闀滃儚鏍煎紡锛夌殑涓€閮ㄥ垎锛涘鍔犱竴涓?EIF 澶达紝鍖呭惈璇稿榄旀暟銆乪if 鐗堟湰銆侀暅鍍忓ぇ灏忓拰
CRC 绛夊厓鏁版嵁銆?
涓烘暣涓?enclave 闀滃儚锛圗IF锛夈€佸唴鏍稿拰 ramdisk(s) 璁＄畻鍝堝笇鍊笺€傝繖鐢ㄤ簬渚嬪妫€鏌?鍔犺浇鍒?enclave VM 涓殑 enclave 闀滃儚灏辨槸棰勬湡瑕佽繍琛岀殑閭ｄ釜銆?
杩欎簺鍔犲瘑搴﹂噺锛坈rypto measurement锛夎鍖呭惈鍦ㄤ竴涓敱 Nitro 绠＄悊绋嬪簭鐢熸垚鐨勭鍚?璇佹槑鏂囨。涓紝骞惰繘涓€姝ョ敤浜庤瘉鏄?enclave 鐨勮韩浠斤紱KMS 鏄?NE 闆嗘垚骞朵細妫€鏌ヨ璇佹槑
鏂囨。鐨勬湇鍔＄殑绀轰緥銆?
enclave 闀滃儚锛圗IF锛夎鍔犺浇鍒?enclave 鍐呭瓨鐨?8 MiB 鍋忕Щ澶勩€俥nclave 涓殑 init
杩涚▼杩炴帴鍒颁富 VM 鐨?vsock CID 鍜屼竴涓瀹氫箟绔彛鈥斺€?000鈥斺€斾互鍙戦€佷竴涓績璺冲€?鈥斺€?xb7銆傝鏈哄埗鐢ㄤ簬鍦ㄤ富 VM 涓鏌?enclave 鏄惁宸插惎鍔ㄣ€備富 VM 鐨?CID 鏄?3銆?
濡傛灉 enclave VM 宕╂簝鎴栦紭闆呴€€鍑猴紝NE 椹卞姩浼氭敹鍒颁竴涓腑鏂簨浠躲€傝浜嬩欢閫氳繃杞
閫氱煡鏈哄埗杩涗竴姝ュ彂閫佺粰杩愯鍦ㄤ富 VM 涓殑鐢ㄦ埛绌洪棿 enclave 杩涚▼銆傜劧鍚庣敤鎴风┖闂?enclave 杩涚▼鍙互閫€鍑恒€?
[^1^] https://aws.amazon.com/ec2/nitro/nitro-enclaves/
[^2^] https://www.kernel.org/doc/html/latest/admin-guide/mm/hugetlbpage.html
[^3^] https://lwn.net/Articles/807108/
[^4^] https://www.kernel.org/doc/html/latest/admin-guide/kernel-parameters.html
[^5^] https://man7.org/linux/man-pages/man7/vsock.7.html
[^6^] https://www.kernel.org/doc/html/latest/x86/boot.html
[^7^] https://www.kernel.org/doc/html/latest/arm64/hugetlbpage.html
[^8^] https://www.kernel.org/doc/html/latest/arm64/booting.html
