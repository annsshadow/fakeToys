
Minimal requirements to compile the Kernel
++++++++++++++++++++++++++++++++++++++++++

## 绠€浠?

鏈枃妗ｆ棬鍦ㄦ彁渚涜繍琛屽綋鍓嶅唴鏍哥増鏈墍闇€鐨勬渶浣庤蒋浠剁骇鍒垪琛ㄣ€?
鏈枃妗ｆ渶鍒濆熀浜庢垜閽堝 2.0.x 鍐呮牳鎵€鍐欑殑鈥淐hanges鈥濇枃浠讹紝鍥犳鍚戜笌閭ｄ唤鏂囦欢鐩稿悓鐨勪汉鑷磋阿锛圝ared Mauch銆丄xel Boldt銆丄lessandro Sigala锛屼互鍙婄綉缁滀笂鏃犳暟鍏朵粬鐢ㄦ埛锛夈€?
######## 褰撳墠鏈€浣庤姹?

鍦ㄨ涓轰綘閬囧埌浜嗙己闄蜂箣鍓嶏紝璇?*鑷冲皯**鍗囩骇鍒颁互涓嬭蒋浠剁増鏈紒濡傛灉浣犱笉纭畾褰撳墠杩愯鐨勬槸鍝釜鐗堟湰锛屽缓璁殑鍛戒护浼氬憡璇変綘銆傝鍒楀嚭绯荤粺涓寘鍚叾鐗堟湰鐨勬墍鏈夌▼搴忥紝璇锋墽琛?./scripts/ver_linux

鍐嶆鎻愰啋锛屾鍒楄〃鍋囪浣犲凡缁忓湪姝ｅ父杩愯涓€涓?Linux 鍐呮牳銆傛澶栵紝骞堕潪鎵€鏈夊伐鍏峰湪鎵€鏈夌郴缁熶笂閮芥槸蹇呴渶鐨勶紱鏄剧劧锛屼緥濡傚鏋滀綘娌℃湁浠讳綍 PC Card 纭欢锛屼綘鍙兘涓嶉渶瑕佸叧蹇?pcmciautils銆?
====================== ===============  ========================================
        绋嬪簭            鏈€浣庣増鏈?             妫€鏌ョ増鏈殑鍛戒护
====================== ===============  ========================================
bash                   4.2              bash --version
bc                     1.06.95          bc --version
bindgen (鍙€?          0.71.1           bindgen --version
binutils               2.30             ld -v
bison                  2.0              bison --version
btrfs-progs            0.18             btrfs --version
Clang/LLVM (鍙€?       15.0.0           clang --version
e2fsprogs              1.41.4           e2fsck -V
flex                   2.5.35           flex --version
gdb                    7.2              gdb --version
GNU awk (鍙€?          5.1.0            gawk --version
GNU C                  8.1              gcc --version
GNU make               4.0              make --version
GNU tar                1.28             tar --version
GRUB                   0.93             grub --version || grub-install --version
gtags (鍙€?            6.6.5            gtags --version
iptables               1.4.2            iptables -V
jfsutils               1.1.3            fsck.jfs -V
kmod                   13               kmod -V
mcelog                 0.6              mcelog --version
mkimage (鍙€?          2017.01          mkimage --version
nfs-utils              1.0.5            showmount --version
openssl & libcrypto    1.0.0            openssl version
pahole                 1.22             pahole --version
pcmciautils            004              pccardctl -V
PPP                    2.4.0            pppd --version
procps                 3.2.0            ps --version
Python                 3.9.x            python3 --version
quota-tools            3.09             quota -V
Rust (鍙€?            1.85.0           rustc --version
Sphinx\ [#f1]_         3.4.3            sphinx-build --version
squashfs-tools         4.0              mksquashfs -version
udev                   081              udevadm --version
util-linux             2.10o            mount --version
xfsprogs               2.6.0            xfs_db -V
====================== ===============  ========================================


######## 鍐呮牳缂栬瘧


### GCC


gcc 鐨勭増鏈姹傚彲鑳藉洜浣犺绠楁満涓?CPU 鐨勭被鍨嬭€屽紓銆?
### Clang/LLVM (鍙€?


clang 鍜?LLVM 宸ュ叿鐨勬渶鏂版寮忓彂甯冪増锛堟牴鎹?`releases.llvm.org <https://releases.llvm.org>`_锛夐兘鍙楁敮鎸佺敤浜庢瀯寤哄唴鏍搞€傝緝鏃х殑鍙戝竷鐗堜笉淇濊瘉鍙敤锛屽苟涓旀垜浠彲鑳戒細浠庡唴鏍镐腑绉婚櫎鐢ㄤ簬鏀寔鏃х増鏈殑鍙橀€氫唬鐮併€傝鍙傞槄鍏充簬浣跨敤 Clang/LLVM 鏋勫缓 Linux 鐨勯澶栨枃妗?<kbuild_llvm>銆?
### Rust (鍙€?


闇€瑕佽緝鏂扮増鏈殑 Rust 缂栬瘧鍣ㄣ€?
璇峰弬闃?Documentation/rust/quick-start.rst 浜嗚В濡備綍婊¤冻 Rust 鏀寔鐨勬瀯寤鸿姹傘€傜壒鍒槸 `Makefile` 鐩爣 `rustavailable` 瀵逛簬鎺掓煡 Rust 宸ュ叿閾炬湭琚娴嬪埌鐨勫師鍥犲緢鏈夌敤銆?
### bindgen (鍙€?


`bindgen` 鐢ㄤ簬涓哄唴鏍哥殑 C 渚х敓鎴?Rust 缁戝畾銆傚畠渚濊禆浜?`libclang`銆?
### Make


鏋勫缓鍐呮牳闇€瑕?GNU make 4.0 鎴栨洿楂樼増鏈€?
### Bash


鍐呮牳鏋勫缓涓細浣跨敤涓€浜?bash 鑴氭湰銆傞渶瑕?Bash 4.2 鎴栨洿楂樼増鏈€?
### Binutils


鏋勫缓鍐呮牳闇€瑕?Binutils 2.30 鎴栨洿楂樼増鏈€?
### pkg-config


鑷?4.18 璧凤紝鏋勫缓绯荤粺闇€瑕?pkg-config 鏉ユ鏌ュ凡瀹夎鐨?kconfig 宸ュ叿锛屽苟纭畾鐢ㄤ簬 'make {g,x}config' 鐨勬爣蹇楄缃€傛鍓?pkg-config 铏借浣跨敤锛屼絾鏈楠岃瘉鎴栬褰曘€?
### Flex


鑷?Linux 4.16 璧凤紝鏋勫缓绯荤粺鍦ㄦ瀯寤烘湡闂寸敓鎴愯瘝娉曞垎鏋愬櫒銆傝繖闇€瑕?flex 2.5.35 鎴栨洿楂樼増鏈€?

### Bison


鑷?Linux 4.16 璧凤紝鏋勫缓绯荤粺鍦ㄦ瀯寤烘湡闂寸敓鎴愯В鏋愬櫒銆傝繖闇€瑕?bison 2.0 鎴栨洿楂樼増鏈€?
### pahole


鑷?Linux 5.2 璧凤紝濡傛灉閫夋嫨浜?CONFIG_DEBUG_INFO_BTF锛屾瀯寤虹郴缁熶細浠?vmlinux 涓殑 DWARF 鐢熸垚 BTF锛圔PF Type Format锛夛紝绋嶅悗涔熶細浠庡唴鏍告ā鍧楃敓鎴愩€傝繖闇€瑕?pahole v1.22 鎴栨洿楂樼増鏈€?
瀹冨彲鍦?'dwarves' 鎴?'pahole' 鍙戣鐗堣蒋浠跺寘涓壘鍒帮紝鎴栨潵鑷?https://fedorapeople.org/~acme/dwarves/銆?
### Perl


**鏋勫缓鍐呮牳闇€瑕?perl 5 浠ュ強浠ヤ笅妯″潡锛?``Getopt**
: Long``,
**``Getopt**
: Std`銆乣File::Basename` 鍜?`File::Find``銆?
### Python


鑻ュ共閰嶇疆閫夐」闇€瑕佸畠锛歛rm/arm64 鐨勯粯璁ら厤缃€丆ONFIG_LTO_CLANG銆佷竴浜涘彲閫夌殑 DRM 閰嶇疆銆乲ernel-doc 宸ュ叿浠ュ強鏂囨。鏋勫缓锛圫phinx锛夌瓑閮介渶瑕佸畠銆?
### BC


鏋勫缓 3.10 鍙婃洿楂樼増鏈殑鍐呮牳闇€瑕?bc銆?

### OpenSSL


妯″潡绛惧悕鍜屽閮ㄨ瘉涔﹀鐞嗕娇鐢?OpenSSL 绋嬪簭鍜屽姞瀵嗗簱鏉ヨ繘琛屽瘑閽ュ垱寤哄拰绛惧悕鐢熸垚銆?
濡傛灉鍚敤浜嗘ā鍧楃鍚嶏紝鏋勫缓 3.7 鍙婃洿楂樼増鏈殑鍐呮牳闇€瑕?openssl銆傛瀯寤?4.3 鍙婃洿楂樼増鏈殑鍐呮牳杩橀渶瑕?openssl 寮€鍙戝寘銆?
### Tar


濡傛灉鎯宠閫氳繃 sysfs 鍚敤瀵瑰唴鏍稿ご鏂囦欢鐨勮闂紙CONFIG_IKHEADERS锛夛紝鍒欓渶瑕?GNU tar銆?
### gtags / GNU GLOBAL (鍙€?


鍐呮牳鏋勫缓闇€瑕侀€氳繃 `make gtags` 鐢熸垚鏍囩鏂囦欢锛岃繖闇€瑕?GNU GLOBAL 6.6.5 鎴栨洿楂樼増鏈€傝繖鏄洜涓哄畠浣跨敤浜?gtags 鐨?`-C (--directory)` 鏍囧織銆?
### mkimage


璇ュ伐鍏峰湪鏋勫缓鎵佸钩闀滃儚鏍戯紙FIT锛孎lat Image Tree锛夋椂浣跨敤锛屽父瑙佷簬 ARM 骞冲彴銆傝宸ュ叿鍙€氳繃 `u-boot-tools` 杞欢鍖呰幏鍙栵紝涔熷彲浠?U-Boot 婧愪唬鐮佹瀯寤恒€傝鍙傞槄 https://docs.u-boot.org/en/latest/build/tools.html#building-tools-for-linux 涓殑璇存槑銆?
### GNU AWK


濡傛灉甯屾湜鍐呮牳鏋勫缓涓哄唴缃ā鍧楃敓鎴愬湴鍧€鑼冨洿鏁版嵁锛圕ONFIG_BUILTIN_MODULE_RANGES锛夛紝鍒欓渶瑕?GNU AWK銆?
######## 绯荤粺宸ュ叿


### 鏋舵瀯鐩稿叧鍙樻洿


DevFS 宸茶寮冪敤锛屾敼鐢?udev锛坔ttps://www.kernel.org/pub/linux/utils/kernel/hotplug/锛夈€?
32 浣?UID 鏀寔鐜板凡灏变綅銆傚敖鎯呬韩鐢ㄥ惂锛?
鍐呮牳鍑芥暟鐨勬枃妗ｆ閫愭杩囨浮鍒伴€氳繃婧愪唬鐮佷腑鍏跺畾涔夐檮杩戙€侀噰鐢ㄧ壒娈婃牸寮忕紪鍐欑殑娉ㄩ噴鏉ヨ繘琛屽唴鑱旀枃妗ｃ€傝繖浜涙敞閲婂彲浠ヤ笌 Documentation/ 鐩綍涓殑 ReST 鏂囦欢缁撳悎锛岀敓鎴愬瘜鏂囨。锛岄殢鍚庡彲杞崲涓?PostScript銆丠TML銆丩aTex銆乪PUB 鍜?PDF 鏂囦欢銆備负浜嗕粠 ReST 鏍煎紡杞崲涓轰綘閫夋嫨鐨勬牸寮忥紝浣犻渶瑕?Sphinx銆?
### Util-linux


鏂扮増鏈殑 util-linux 鎻愪緵浜嗗鏇村ぇ纾佺洏鐨?`fdisk` 鏀寔銆佹敮鎸?mount 鐨勬柊閫夐」銆佽瘑鍒洿澶氬彈鏀寔鐨勫垎鍖虹被鍨嬶紝浠ュ強绫讳技鐨勫ソ涓滆タ銆備綘鍙兘鎯宠鍗囩骇銆?
### Ksymoops


濡傛灉鍙戠敓浜嗕笉鍙兂璞＄殑浜嬫儏锛屼綘鐨勫唴鏍稿彂鐢熶簡 oops锛屼綘鍙兘闇€瑕?ksymoops 宸ュ叿鏉ヨВ鐮佸畠锛屼絾鍦ㄥぇ澶氭暟鎯呭喌涓嬩綘涓嶉渶瑕併€傞€氬父鏇村€惧悜浜庝娇鐢?`CONFIG_KALLSYMS` 鏋勫缓鍐呮牳锛岃繖鏍峰畠浼氫骇鐢熷彲鐩存帴浣跨敤鐨勫彲璇昏浆鍌紙杩欎篃浼氫骇鐢熸瘮 ksymoops 鏇村ソ鐨勮緭鍑猴級銆傚鏋滃嚭浜庢煇绉嶅師鍥犱綘鐨勫唴鏍告湭浣跨敤 `CONFIG_KALLSYMS` 鏋勫缓锛屽苟涓斾綘鏃犳硶閲嶆柊鏋勫缓骞剁敤璇ラ€夐」澶嶇幇 Oops锛岄偅涔堜綘浠嶇劧鍙互鐢?ksymoops 瑙ｇ爜璇?Oops銆?
### Mkinitrd


`/lib/modules` 鏂囦欢鏍戝竷灞€鐨勮繖浜涘彉鏇翠篃瑕佹眰鍗囩骇 mkinitrd銆?
### E2fsprogs


鏈€鏂扮増鏈殑 `e2fsprogs` 淇浜?fsck 鍜?debugfs 涓殑鑻ュ共缂洪櫡銆傛樉鐒讹紝鍗囩骇鏄釜濂戒富鎰忋€?
### JFSutils


`jfsutils` 杞欢鍖呭寘鍚鏂囦欢绯荤粺鐨勫伐鍏枫€傚彲鐢ㄥ伐鍏锋湁锛?
- `fsck.jfs` - 鍚姩浜嬪姟鏃ュ織閲嶆斁锛屽苟妫€鏌ャ€佷慨澶?JFS 鏍煎紡鐨勫垎鍖恒€?
- `mkfs.jfs` - 鍒涘缓 JFS 鏍煎紡鐨勫垎鍖恒€?
- 璇ヨ蒋浠跺寘涓繕鎻愪緵鍏朵粬鏂囦欢绯荤粺宸ュ叿銆?
### Xfsprogs


鏈€鏂扮増鏈殑 `xfsprogs` 鍖呭惈 `mkfs.xfs`銆乣xfs_db` 浠ュ強 `xfs_repair` 宸ュ叿绛夛紝鐢ㄤ簬 XFS 鏂囦欢绯荤粺銆傚畠涓庢灦鏋勬棤鍏筹紝2.0.0 鍙婁箣鍚庣殑浠讳綍鐗堟湰閮藉簲鑳戒笌姝ょ増鏈殑 XFS 鍐呮牳浠ｇ爜姝ｅ父宸ヤ綔锛堢敱浜庝竴浜涙樉钁楃殑鏀硅繘锛屽缓璁娇鐢?2.6.0 鎴栨洿楂樼増鏈級銆?
### PCMCIAutils


PCMCIAutils 鍙栦唬浜?`pcmcia-cs`銆傚畠鍦ㄧ郴缁熷惎鍔ㄦ椂姝ｇ‘璁剧疆 PCMCIA 鎻掓Ы锛屽苟鍦ㄥ唴鏍歌妯″潡鍖栧苟涓斾娇鐢ㄤ簡 hotplug 瀛愮郴缁熸椂锛屼负 16 浣?PCMCIA 璁惧鍔犺浇鐩稿簲鐨勬ā鍧椼€?
### Quota-tools


濡傛灉浣犳兂浣跨敤杈冩柊鐗堟湰 2 鐨勯厤棰濇牸寮忥紝鍒欓渶瑕佹敮鎸?32 浣?uid 鍜?gid銆俀uota-tools 3.07 鍙婃洿楂樼増鏈敮鎸佹鍔熻兘銆傝浣跨敤涓婅〃涓帹鑽愭垨鏇撮珮鐨勭増鏈€?
### Intel IA32 寰爜


娣诲姞浜嗕竴涓┍鍔紝鍏佽鏇存柊 Intel IA32 寰爜锛屽畠浣滀负鏅€氾紙misc锛夊瓧绗﹁澶囧彲璁块棶銆傚鏋滀綘娌℃湁浣跨敤
```
  mkdir /dev/cpu
  mknod /dev/cpu/microcode c 10 184
  chmod 0644 /dev/cpu/microcode

```

浣滀负 root 鎵嶈兘浣跨敤瀹冦€備綘鍙兘杩樻兂瑕佽幏鍙栫敤鎴风┖闂寸殑 microcode_ctl 宸ュ叿鏉ラ厤鍚堜娇鐢ㄣ€?
### udev


`udev` 鏄竴涓敤鎴风┖闂村簲鐢ㄧ▼搴忥紝鐢ㄤ簬浠呯敤瀹為檯瀛樺湪鐨勮澶囨潯鐩姩鎬佸～鍏?`/dev`銆俙udev` 鍙栦唬浜?devfs 鐨勫熀鏈姛鑳斤紝鍚屾椂鍏佽瀵硅澶囦娇鐢ㄦ寔涔呭寲鍛藉悕銆?
### FUSE


闇€瑕?libfuse 2.4.0 鎴栨洿楂樼増鏈€傜粷瀵规渶浣庝负 2.3.0锛屼絾鎸傝浇閫夐」 `direct_io` 鍜?`kernel_cache` 灏嗕笉璧蜂綔鐢ㄣ€?
######## 缃戠粶


### 鎬讳綋鍙樻洿


濡傛灉浣犳湁楂樼骇鐨勭綉缁滈厤缃渶姹傦紝浣犲彲鑳藉簲璇ヨ€冭檻浣跨敤 ip-route2 涓殑缃戠粶宸ュ叿銆?
### 鍖呰繃婊?/ NAT


鍖呰繃婊ゅ拰 NAT 浠ｇ爜浣跨敤涓庝箣鍓?2.4.x 鍐呮牳绯诲垪鐩稿悓鐨勫伐鍏凤紙iptables锛夈€傚畠浠嶇劧鍖呭惈閽堝 2.2.x 椋庢牸 ipchains 鍜?2.0.x 椋庢牸 ipfwadm 鐨勫悜鍚庡吋瀹规ā鍧椼€?
### PPP


PPP 椹卞姩宸茶閲嶆瀯浠ユ敮鎸佸閾捐矾锛坢ultilink锛夛紝骞朵娇鍏惰兘澶熷湪澶氭牱鍖栫殑濯掍綋灞備笂杩愯銆傚鏋滀綘浣跨敤 PPP锛岃灏?pppd 鍗囩骇鍒拌嚦灏?2.4.0銆?
濡傛灉浣犳病鏈変娇鐢?udev锛屽垯蹇呴』鏈夎澶囨枃浠?/dev/ppp
```
  mknod /dev/ppp c 108 0

```

浣滀负 root銆?
### NFS-utils


鍦ㄥ彜鑰佺殑锛?.4 鍙婃洿鏃╋級鍐呮牳涓紝nfs 鏈嶅姟鍣ㄩ渶瑕佺煡閬撲换浣曟湡鏈涜兘澶熼€氳繃 NFS 璁块棶鏂囦欢鐨勫鎴风銆傚綋瀹㈡埛绔寕杞芥枃浠剁郴缁熸椂锛岃繖浜涗俊鎭細鐢?`mountd` 鎻愪緵缁欏唴鏍革紝鎴栬€呭湪绯荤粺鍚姩鏃剁敱 `exportfs` 鎻愪緵銆俙exportfs` 浼氫粠 `/var/lib/nfs/rmtab` 鑾峰彇鍏充簬娲昏穬瀹㈡埛绔殑淇℃伅銆?
杩欑鏂瑰紡鐩稿綋鑴嗗急锛屽洜涓哄畠渚濊禆浜?rmtab 鐨勬纭€э紝鑰岃繖骞朵笉鎬绘槸瀹规槗淇濊瘉锛岀壒鍒槸鍦ㄥ皾璇曞疄鐜版晠闅滆浆绉绘椂銆傚嵆浣跨郴缁熻繍琛岃壇濂斤紝`rmtab` 涔熶細绉疮澶ч噺姘歌繙涓嶄細琚垹闄ょ殑鏃ф潯鐩€?
鍦ㄧ幇浠ｅ唴鏍镐腑锛屾垜浠彲浠ラ€夋嫨璁╁唴鏍稿湪鏀跺埌鏉ヨ嚜鏈煡涓绘満鐨勮姹傛椂閫氱煡 mountd锛岃€?mountd 鍙互鍚戝唴鏍告彁渚涚浉搴旂殑瀵煎嚭淇℃伅銆傝繖娑堥櫎浜嗗 `rmtab` 鐨勪緷璧栵紝鎰忓懗鐫€鍐呮牳鍙渶瑕佺煡閬撳綋鍓嶆椿璺冪殑瀹㈡埛绔€?
```
  mount -t nfsd nfsd /proc/fs/nfsd

```

鍦ㄨ繍琛?exportfs 鎴?mountd 涔嬪墠銆傚缓璁湪鍙兘鐨勬儏鍐典笅锛岀敤闃茬伀澧欏皢鎵€鏈?NFS 鏈嶅姟涓庢暣涓簰鑱旂綉闅旂銆?
### mcelog


鍦?x86 鍐呮牳涓婏紝褰撳惎鐢?`CONFIG_X86_MCE` 鏃讹紝闇€瑕?mcelog 宸ュ叿鏉ュ鐞嗗拰璁板綍鏈哄櫒妫€鏌ワ紙machine check锛変簨浠躲€傛満鍣ㄦ鏌ヤ簨浠舵槸鐢?CPU 鎶ュ憡鐨勯敊璇€傚己鐑堝缓璁鍏惰繘琛屽鐞嗐€?
######## 鍐呮牳鏂囨。


### Sphinx


鏈夊叧 Sphinx 瑕佹眰鐨勮缁嗕俊鎭紝璇峰弬闃?Documentation/doc-guide/sphinx.rst 涓殑 sphinx_install銆?
### rustdoc


`rustdoc` 鐢ㄤ簬鐢熸垚 Rust 浠ｇ爜鐨勬枃妗ｃ€傛洿澶氫俊鎭鍙傞槄 Documentation/rust/general-information.rst銆?
## 鑾峰彇鏇存柊鐨勮蒋浠?

######## 鍐呮牳缂栬瘧


### gcc


- <ftp://ftp.gnu.org/gnu/gcc/>

### Clang/LLVM


- Getting LLVM <getting_llvm>銆?
### Rust


- Documentation/rust/quick-start.rst銆?
### bindgen


- Documentation/rust/quick-start.rst銆?
### Make


- <ftp://ftp.gnu.org/gnu/make/>

### Bash


- <ftp://ftp.gnu.org/gnu/bash/>

### Binutils


- <https://www.kernel.org/pub/linux/devel/binutils/>

### Flex


- <https://github.com/westes/flex/releases>

### Bison


- <ftp://ftp.gnu.org/gnu/bison/>

### OpenSSL


- <https://www.openssl.org/>

######## 绯荤粺宸ュ叿


### Util-linux


- <https://www.kernel.org/pub/linux/utils/util-linux/>

### Kmod


- <https://www.kernel.org/pub/linux/utils/kernel/kmod/>
- <https://git.kernel.org/pub/scm/utils/kernel/kmod/kmod.git>

### Ksymoops


- <https://www.kernel.org/pub/linux/utils/kernel/ksymoops/v2.4/>

### Mkinitrd


- <https://code.launchpad.net/initrd-tools/main>

### E2fsprogs


- <https://www.kernel.org/pub/linux/kernel/people/tytso/e2fsprogs/>
- <https://git.kernel.org/pub/scm/fs/ext2/e2fsprogs.git/>

### JFSutils


- <https://jfs.sourceforge.net/>

### Xfsprogs


- <https://git.kernel.org/pub/scm/fs/xfs/xfsprogs-dev.git>
- <https://www.kernel.org/pub/linux/utils/fs/xfs/xfsprogs/>

### Pcmciautils


- <https://www.kernel.org/pub/linux/utils/kernel/pcmcia/>

### Quota-tools


- <https://sourceforge.net/projects/linuxquota/>


### Intel P6 microcode


- <https://downloadcenter.intel.com/>

### udev


- <https://www.freedesktop.org/software/systemd/man/udev.html>

### FUSE


- <https://github.com/libfuse/libfuse/releases>

### mcelog


- <https://www.mcelog.org/>

######## 缃戠粶


### PPP


- <https://download.samba.org/pub/ppp/>
- <https://git.ozlabs.org/?p=ppp.git>
- <https://github.com/paulusmack/ppp/>

### NFS-utils


- <https://sourceforge.net/project/showfiles.php?group_id=14>
- <https://nfs.sourceforge.net/>

### Iptables


- <https://netfilter.org/projects/iptables/index.html>

### Ip-route2


- <https://www.kernel.org/pub/linux/utils/net/iproute2/>

### OProfile


- <https://oprofile.sf.net/download/>

######## 鍐呮牳鏂囨。


### Sphinx


- <https://www.sphinx-doc.org/>
