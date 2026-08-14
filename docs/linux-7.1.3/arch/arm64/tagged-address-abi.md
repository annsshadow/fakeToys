## AArch64 甯︽爣绛惧湴鍧€ ABI


Authors: Vincenzo Frascino <vincenzo.frascino@arm.com>
         Catalin Marinas <catalin.marinas@arm.com>

Date: 21 August 2019

鏈枃妗ｆ弿杩颁簡 AArch64 Linux 涓婂甫鏍囩鍦板潃锛圱agged Address锛堿BI 鐨勭敤娉曞拰璇箟銆?
### 1. 绠€浠?

鍦?AArch64 涓婏紝`TCR_EL1.TBI0` 浣嶉粯璁よ璁剧疆锛屽厑璁哥敤鎴风┖闂达紙EL0锛夐€氳繃鍏锋湁闈為浂鏈€楂樺瓧鑺傜殑 64 浣嶆寚閽堟墽琛屽唴瀛樿闂€傛湰鏂囨。鎻忚堪浜?syscall ABI 鐨勬斁瀹斤紝璇ユ斁瀹藉厑璁哥敤鎴风┖闂村皢鏌愪簺甯︽爣绛剧殑鎸囬拡浼犻€掔粰鍐呮牳绯荤粺璋冪敤銆?
### 2. AArch64 甯︽爣绛惧湴鍧€ ABI


浠庡唴鏍哥郴缁熻皟鐢ㄦ帴鍙ｇ殑瑙掑害浠ュ強鍑轰簬鏈枃妗ｇ殑鐩殑锛屸€滄湁鏁堢殑甯︽爣绛炬寚閽堚€濇槸鎸囦竴涓彲鑳藉叿鏈夐潪闆舵渶楂樺瓧鑺傘€佷笖寮曠敤浜嗙敤鎴疯繘绋嬪湴鍧€绌洪棿涓€氳繃浠ヤ笅鏂瑰紡涔嬩竴鑾峰緱鐨勫湴鍧€鐨勬寚閽堬細

- `mmap()` 绯荤粺璋冪敤锛屼笖婊¤冻浠ヤ笅浠讳竴鏉′欢锛?
  - 鏍囧織璁剧疆浜?`MAP_ANONYMOUS` 浣嶏紝鎴?  - 鏂囦欢鎻忚堪绗﹀紩鐢ㄤ竴涓櫘閫氭枃浠讹紙鍖呮嫭鐢?`memfd_create()` 杩斿洖鐨勬枃浠讹級鎴?`/dev/zero`

- `brk()` 绯荤粺璋冪敤锛堝嵆杩涚▼鍒涘缓鏃剁▼搴忔柇鐐瑰垵濮嬩綅缃笌鍏跺綋鍓嶄綅缃箣闂寸殑鍫嗗尯鍩燂級銆?
- 鍐呮牳鍦ㄨ繘绋嬪湴鍧€绌洪棿涓垱寤虹殑銆佸叿鏈変笌涓婇潰 `mmap()` 鐩稿悓闄愬埗锛堜緥濡傛暟鎹€乥ss銆佹爤锛夌殑浠讳綍鍐呭瓨鏄犲皠銆?
AArch64 甯︽爣绛惧湴鍧€ ABI 鏍规嵁鍐呮牳濡備綍浣跨敤鐢ㄦ埛鍦板潃锛屽垎涓轰袱涓樁娈电殑鏀惧锛?
1. 涓嶈鍐呮牳璁块棶浣嗙敤浜庡湴鍧€绌洪棿绠＄悊鐨勭敤鎴峰湴鍧€锛堜緥濡?`mprotect()`銆乣madvise()`锛夈€傚湪姝や笂涓嬫枃涓厑璁镐娇鐢ㄦ湁鏁堢殑甯︽爣绛炬寚閽堬紝浣嗘湁浠ヤ笅渚嬪锛?
   - `brk()`銆乣mmap()` 浠ュ強 `mremap()` 鐨?`new_address` 鍙傛暟锛屽洜涓哄畠浠湁鍙兘涓庣幇鏈夌殑鐢ㄦ埛鍦板潃浜х敓鍒悕銆?
     锘挎敞鎰忥細姝よ涓哄湪 v5.6 涓彂鐢熶簡鍙樺寲锛屽洜姝ゆ煇浜涜緝鏃╃殑鍐呮牳鍙兘浼氶敊璇湴鎺ュ彈閽堝 `brk()`銆乣mmap()` 鍜?`mremap()` 绯荤粺璋冪敤鐨勬湁鏁堝甫鏍囩鎸囬拡銆?
   - 鍦ㄤ粠 `userfaultfd()` 鑾峰緱鐨勬枃浠舵弿杩扮涓婁娇鐢ㄧ殑 `UFFDIO_*` `ioctl()` 鐨?`range.start`銆乣start` 鍜?`dst` 鍙傛暟锛屽洜涓洪殢鍚庨€氳繃璇诲彇璇ユ枃浠舵弿杩扮鑾峰緱鐨勬晠闅滃湴鍧€灏嗘槸鍘绘爣绛剧殑锛屽惁鍒欏彲鑳戒細璁╀笉鐭ラ亾鏍囩鐨勭▼搴忔劅鍒板洶鎯戙€?
     锘挎敞鎰忥細姝よ涓哄湪 v5.14 涓彂鐢熶簡鍙樺寲锛屽洜姝ゆ煇浜涜緝鏃╃殑鍐呮牳鍙兘浼氶敊璇湴鎺ュ彈閽堝璇ョ郴缁熻皟鐢ㄧ殑鏈夋晥甯︽爣绛炬寚閽堛€?
2. 琚唴鏍歌闂殑鐢ㄦ埛鍦板潃锛堜緥濡?`write()`锛夈€傛 ABI 鏀惧榛樿鏄鐢ㄧ殑锛屽簲鐢ㄧ▼搴忕嚎绋嬮渶瑕侀€氳繃 `prctl()` 鏄惧紡鍚敤锛屽涓嬫墍绀猴細

   - `PR_SET_TAGGED_ADDR_CTRL`锛氫负璋冪敤绾跨▼鍚敤鎴栫鐢?AArch64 甯︽爣绛惧湴鍧€ ABI銆?
     `(unsigned int) arg2` 鍙傛暟鏄竴涓弿杩版墍鐢ㄦ帶鍒舵ā寮忕殑浣嶆帺鐮侊細

     - `PR_TAGGED_ADDR_ENABLE`锛氬惎鐢?AArch64 甯︽爣绛惧湴鍧€ ABI銆傞粯璁ょ姸鎬佷负绂佺敤銆?
     锘垮弬鏁?`arg3`銆乣arg4` 鍜?`arg5` 蹇呴』涓?0銆?
   - `PR_GET_TAGGED_ADDR_CTRL`锛氳幏鍙栬皟鐢ㄧ嚎绋嬬殑 AArch64 甯︽爣绛惧湴鍧€ ABI 鐘舵€併€?
     锘垮弬鏁?`arg2`銆乣arg3`銆乣arg4` 鍜?`arg5` 蹇呴』涓?0銆?
   涓婅堪 ABI 灞炴€ф槸绾跨▼浣滅敤鍩熺殑锛屽湪 clone() 鍜?fork() 鏃剁户鎵匡紝鍦?exec() 鏃舵竻闄ゃ€?
   濡傛灉 AArch64 甯︽爣绛惧湴鍧€ ABI 琚?`sysctl abi.tagged_addr_disabled=1` 鍏ㄥ眬绂佺敤锛岄偅涔堣皟鐢?`prctl(PR_SET_TAGGED_ADDR_CTRL, PR_TAGGED_ADDR_ENABLE, 0, 0, 0)` 灏嗚繑鍥?`-EINVAL`銆傞粯璁ょ殑 `sysctl abi.tagged_addr_disabled` 閰嶇疆涓?0銆?
褰撲负鏌愮嚎绋嬪惎鐢ㄤ簡 AArch64 甯︽爣绛惧湴鍧€ ABI 鏃讹紝淇濊瘉浠ヤ笅琛屼负锛?
- 闄ょ 3 鑺傛彁鍒扮殑鎯呭舰澶栵紝鎵€鏈夌郴缁熻皟鐢ㄩ兘鍙互鎺ュ彈浠讳綍鏈夋晥鐨勫甫鏍囩鎸囬拡銆?
- 瀵逛簬鏃犳晥鐨勫甫鏍囩鎸囬拡锛岀郴缁熻皟鐢ㄧ殑琛屼负鏄湭瀹氫箟鐨勶細瀹冨彲鑳藉鑷磋繑鍥為敊璇爜銆佸紩鍙戯紙鑷村懡锛変俊鍙凤紝鎴栧叾浠栧け璐ユā寮忋€?
- 瀵逛簬鏈夋晥鐨勫甫鏍囩鎸囬拡锛岀郴缁熻皟鐢ㄧ殑琛屼负涓庡搴旂殑鍘绘爣绛炬寚閽堢浉鍚屻€?

AArch64 涓婂甫鏍囩鎸囬拡鍚箟鐨勫畾涔夊彲浠ュ湪 Documentation/arch/arm64/tagged-pointers.rst 涓壘鍒般€?
### 3. AArch64 甯︽爣绛惧湴鍧€ ABI 渚嬪


鏃犺 ABI 鏀惧涓庡惁锛屼互涓嬬郴缁熻皟鐢ㄥ弬鏁板繀椤诲幓鏍囩锛?
- `prctl()` 涓櫎鐩存帴闂存帴浣滀负鍐呮牳瑕佽闂殑鍙傛暟浼犻€掔殑鐢ㄦ埛鏁版嵁鎸囬拡涔嬪鐨勫叾浠栧弬鏁般€?
- `ioctl()` 涓櫎鐩存帴闂存帴浣滀负鍐呮牳瑕佽闂殑鍙傛暟浼犻€掔殑鐢ㄦ埛鏁版嵁鎸囬拡涔嬪鐨勫叾浠栧弬鏁般€?
- `shmat()` 鍜?`shmdt()`銆?
- `brk()`锛堣嚜鍐呮牳 v5.6 璧凤級銆?
- `mmap()`锛堣嚜鍐呮牳 v5.6 璧凤級銆?
- `mremap()` 鐨?`new_address` 鍙傛暟锛堣嚜鍐呮牳 v5.6 璧凤級銆?
浠讳綍浣跨敤闈為浂甯︽爣绛炬寚閽堢殑灏濊瘯閮藉彲鑳藉鑷磋繑鍥為敊璇爜銆佸紩鍙戯紙鑷村懡锛変俊鍙凤紝鎴栧叾浠栧け璐ユā寮忋€?
### 4. 姝ｇ‘鐢ㄦ硶绀轰緥


   #include <stdlib.h>
   #include <string.h>
   #include <unistd.h>
   #include <sys/mman.h>
   #include <sys/prctl.h>

   #define PR_SET_TAGGED_ADDR_CTRL	55
   #define PR_TAGGED_ADDR_ENABLE	(1UL << 0)

   #define TAG_SHIFT		56

   int main(void)
   {
   	int tbi_enabled = 0;
   	unsigned long tag = 0;
   	char *ptr;

   	/** check/enable the tagged address ABI **/
   	if (!prctl(PR_SET_TAGGED_ADDR_CTRL, PR_TAGGED_ADDR_ENABLE, 0, 0, 0))
   		tbi_enabled = 1;

   	/** memory allocation **/
   	ptr = mmap(NULL, sysconf(_SC_PAGE_SIZE), PROT_READ | PROT_WRITE,
   		   MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
   	if (ptr == MAP_FAILED)
   		return 1;

   	/** set a non-zero tag if the ABI is available **/
   	if (tbi_enabled)
   		tag = rand() & 0xff;
   	ptr = (char *)((unsigned long)ptr | (tag << TAG_SHIFT));

   	/** memory access to a tagged address **/
   	strcpy(ptr, "tagged pointer\n");

   	/** syscall with a tagged pointer **/
   	write(1, ptr, strlen(ptr));

   	return 0;
   }
