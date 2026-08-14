
## 濡備綍楠岃瘉缂洪櫡骞惰繘琛屽洖褰掔殑浜屽垎瀹氫綅


鏈枃浠嬬粛濡備綍妫€鏌ユ煇涓?Linux 鍐呮牳闂鏄惁鍑虹幇鍦ㄥ紑鍙戣€呭綋鍓嶇淮鎶ょ殑浠ｇ爜涓€斺€斿苟杩涗竴姝ヨ鏄庯紝鑻ヨ闂灞炰簬鍥炲綊锛堜緥濡傛棭鏈熺増鏈腑骞舵湭鍑虹幇锛夛紝搴斿浣曞畾浣嶅鑷磋闂鐨勫彉鏇淬€?

姝ｆ枃涓昏闈㈠悜鍦ㄦ櫘閫氱‖浠朵笂杩愯涓绘祦 Linux 鍙戣鐗堝唴鏍搞€佸苟甯屾湜鍚戜笂娓?Linux 寮€鍙戣€呮姤鍛婂唴鏍哥己闄风殑鐢ㄦ埛銆傚敖绠″姝わ紝杩欎簺璇存槑鍚屾牱閫傜敤浜庡凡缁忕啛鎮夎嚜琛屾瀯寤哄唴鏍哥殑鐢ㄦ埛锛氬畠浠湁鍔╀簬閬垮厤鍗充娇鏄粡楠屼赴瀵岀殑寮€鍙戣€呭伓灏斾篃浼氱姱涓嬬殑閿欒銆?

..
   Note: if you see this note, you are reading the text's source file. You
   might want to switch to a rendered version: it makes it a lot easier to
   read and navigate this document -- especially when you want to look something
   up in the reference section, then jump back to where you left off.
..
   Find the latest rendered version of this text here:
   https://docs.kernel.org/admin-guide/verify-bugs-and-bisect-regressions.html

## 娴佺▼鐨勬牳蹇冿紙鍗斥€淭L;DR鈥濓級


*[濡傛灉浣犳槸鍒濇鏋勫缓鍐呮牳鎴栧鍏惰繘琛屼簩鍒嗗畾浣嶏紝璇峰拷鐣ユ湰鑺傦紝鐩存帴鍓嶅線涓嬫柟鐨?'step-by-step guide <introguide_bissbs>'銆傛湰鑺備娇鐢ㄤ笌涓嬫枃鐩稿悓鐨勫懡浠わ紝鍙槸鎻忚堪鏇翠负绠€鐣ワ紱涓嶈繃杩欎簺姝ラ渚濈劧鏄撲簬閬靛惊锛屽苟涓斿弬鑰冪珷鑺備腑鐨勭浉鍏虫潯鐩繕鎻愬埌浜嗚澶氭浛浠ｆ柟妗堛€侀櫡闃变互鍙婂叾浠栨敞鎰忎簨椤癸紝鍦ㄤ綘褰撳墠鐨勬儏鍐典笅杩欎簺鍙兘閮借嚦鍏抽噸瑕併€俔*

**濡傛灉浣犳兂妫€鏌ユ煇涓己闄锋槸鍚﹀瓨鍦ㄤ簬寮€鍙戣€呭綋鍓嶇淮鎶ょ殑浠ｇ爜涓?*锛屽彧闇€鎵ц **鍑嗗宸ヤ綔锛坧reparations锛?* 涓?**绗?1 娈碉紙segment 1锛?*锛涘湪姝よ繃绋嬩腑锛屾妸浣犳棩甯镐娇鐢ㄧ殑銆佹渶鏂扮殑 Linux 鍐呮牳瑙嗕负鈥滃彲鐢紙working锛夆€濆唴鏍搞€備笅闈㈢殑绀轰緥鍋囪璇ュ唴鏍镐负 6.0锛屽洜姝ゅ皢浣跨敤瀹冪殑婧愮爜鏉ュ噯澶?.config 鏂囦欢銆?

**濡傛灉浣犻亣鍒扮殑鏄竴涓洖褰?*锛岃鑷冲皯鎵ц鍒?**绗?2 娈碉紙segment 2锛?* 缁撴潫銆傞殢鍚庝綘鍙互鎻愪氦涓€浠藉垵姝ユ姤鍛娾€斺€斾篃鍙互缁х画 **绗?3 娈碉紙segment 3锛?*锛屽叾涓鏄庝簡濡備綍鎵ц涓€浠藉畬鏁寸殑鍥炲綊鎶ュ憡鎵€闇€鐨勪簩鍒嗗畾浣嶃€備笅闈㈢殑绀轰緥鍋囪 6.0.13 涓衡€滃彲鐢紙working锛夆€濆唴鏍搞€?.1.5 涓虹涓€涓€滄崯鍧忥紙broken锛夆€濆唴鏍革紝鍥犳灏嗘妸 6.0 瑙嗕负鈥滆壇濂斤紙good锛夆€濈増鏈苟鐢ㄤ簬鍑嗗 .config 鏂囦欢銆?

```
    # * Remove any software that depends on externally maintained kernel modules
    #   or builds any automatically during bootup.
    # * Ensure Secure Boot permits booting self-compiled Linux kernels.
    # * If you are not already running the 'working' kernel, reboot into it.
    # * Install compilers and everything else needed for building Linux.
    # * Ensure to have 15 Gigabyte free space in your home directory.
    git clone -o mainline --no-checkout \
      https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git ~/linux/
    cd ~/linux/
    git remote add -t master stable \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git
    git switch --detach v6.0
    # * Hint: if you used an existing clone, ensure no stale .config is around.
    make olddefconfig
    # * Ensure the former command picked the .config of the 'working' kernel.
    # * Connect external hardware (USB keys, tokens, ...), start a VM, bring up
    #   VPNs, mount network shares, and briefly try the feature that is broken.
    yes '' | make localmodconfig
    ./scripts/config --set-str CONFIG_LOCALVERSION '-local'
    ./scripts/config -e CONFIG_LOCALVERSION_AUTO
    # * Note, when short on storage space, check the guide for an alternative:
    ./scripts/config -d DEBUG_INFO_NONE -e KALLSYMS_ALL -e DEBUG_KERNEL \
      -e DEBUG_INFO -e DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT -e KALLSYMS
    # * Hint: at this point you might want to adjust the build configuration;
    #   you'll have to, if you are running Debian.
    make olddefconfig
    cp .config ~/kernel-config-working
```
- **绗?1 娈碉紙Segment 1锛?*锛氬熀浜庢渶鏂扮殑 mainline 浠ｇ爜搴撴瀯寤轰竴涓唴鏍搞€?

  杩欓櫎浜嗗彲浠ユ鏌ラ棶棰樻槸鍚﹀凡缁忚淇涔嬪锛岃繕鑳藉憡璇夊悗缁渶瑕佺煡浼氬摢浜涘紑鍙戣€咃紱鍦ㄥ洖褰掔殑鎯呭喌涓嬶紝杩欎竴姝ヨ繕鑳芥帓闄ら棶棰樻槸鍥?.config 鍙樻洿鑰岃捣鐨勫彲鑳姐€?

```
    cd ~/linux/
    git switch --discard-changes --detach mainline/master

  b) Build, install, and boot a kernel::

    cp ~/kernel-config-working .config
    make olddefconfig
    make -j $(nproc --all)
    # * Make sure there is enough disk space to hold another kernel:
    df -h /boot/ /lib/modules/
    # * Note: on Arch Linux, its derivatives and a few other distributions
    #   the following commands will do nothing at all or only part of the
    #   job. See the step-by-step guide for further details.
    sudo make modules_install
    command -v installkernel && sudo make install
    # * Check how much space your self-built kernel actually needs, which
    #   enables you to make better estimates later:
    du -ch /boot/*$(make -s kernelrelease)* | tail -n 1
    du -sh /lib/modules/$(make -s kernelrelease)/
    # * Hint: the output of the following command will help you pick the
    #   right kernel from the boot menu:
    make -s kernelrelease | tee -a ~/kernels-built
    reboot
    # * Once booted, ensure you are running the kernel you just built by
    #   checking if the output of the next two commands matches:
    tail -n 1 ~/kernels-built
    uname -r
    cat /proc/sys/kernel/tainted

  c) Check if the problem occurs with this kernel as well.
```
- **绗?2 娈碉紙Segment 2锛?*锛氱‘淇濃€滆壇濂斤紙good锛夆€濆唴鏍稿悓鏃朵篃鏄€滃彲鐢紙working锛夆€濆唴鏍搞€?

  杩欏湪鍏朵粬鏂归潰涔熼獙璇佷簡瑁佸壀鍚庣殑 .config 鏂囦欢纭疄鑳芥甯稿伐浣滐紝鍚﹀垯鐢ㄥ畠鏉ュ仛浜屽垎瀹氫綅灏辨槸鍦ㄦ氮璐规椂闂达細

```
    cd ~/linux/
    git switch --discard-changes --detach v6.0

  b) Build, install, and boot a kernel as described earlier in *segment 1,
     section b* -- just feel free to skip the 'du' commands, as you have a rough
     estimate already.

  c) Ensure the feature that regressed with the 'broken' kernel actually works
     with this one.
```
- **绗?3 娈碉紙Segment 3锛?*锛氭墽琛屽苟楠岃瘉浜屽垎瀹氫綅銆?

```
    git remote set-branches --add stable linux-6.1.y
    git fetch stable

  b) Initialize the bisection::

    cd ~/linux/
    git bisect start
    git bisect good v6.0
    git bisect bad v6.1.5

  c) Build, install, and boot a kernel as described earlier in *segment 1,
     section b*.

     In case building or booting the kernel fails for unrelated reasons, run
     ``git bisect skip``. In all other outcomes, check if the regressed feature
     works with the newly built kernel. If it does, tell Git by executing
     ``git bisect good``; if it does not, run ``git bisect bad`` instead.

     All three commands will make Git check out another commit; then re-execute
     this step (e.g. build, install, boot, and test a kernel to then tell Git
     the outcome). Do so again and again until Git shows which commit broke
     things. If you run short of disk space during this process, check the
     section 'Complementary tasks: cleanup during and after the process'
     below.

  d) Once your finished the bisection, put a few things away::

    cd ~/linux/
    git bisect log > ~/bisect-log
    cp .config ~/bisection-config-culprit
    git bisect reset

  e) Try to verify the bisection result::

    git switch --discard-changes --detach mainline/master
    git revert --no-edit cafec0cacaca0
    cp ~/kernel-config-working .config
    ./scripts/config --set-str CONFIG_LOCALVERSION '-local-cafec0cacaca0-reverted'

    This is optional, as some commits are impossible to revert. But if the
    second command worked flawlessly, build, install, and boot one more kernel
    kernel; just this time skip the first command copying the base .config file
    over, as that already has been taken care off.
```
- **杈呭姪浠诲姟锛圕omplementary tasks锛?*锛氬湪娴佺▼杩涜鏈熼棿鍙婁箣鍚庤繘琛屾竻鐞嗐€?

  a) 涓轰簡閬垮厤鍦ㄤ簩鍒嗗畾浣嶈繃绋嬩腑鑰楀敖纾佺洏绌洪棿锛屼綘鍙兘闇€瑕佸垹闄や竴浜涗箣鍓嶆瀯寤虹殑鍐呮牳銆備綘寰堝彲鑳藉笇鏈涘皢绗?1 娈靛拰绗?2 娈垫湡闂存瀯寤虹殑鍐呮牳淇濈暀涓€娈垫椂闂达紝浣嗗湪瀹為檯浜屽垎瀹氫綅杩囩▼涓祴璇曡繃鐨勫唴鏍革紝浣犲鍗婁笉鍐嶉渶瑕佸畠浠€?

```
       ls -ltr /lib/modules/*-local*

    To then for example erase a kernel that identifies itself as
    '6.0-rc1-local-gcafec0cacaca0', use this::

       sudo rm -rf /lib/modules/6.0-rc1-local-gcafec0cacaca0
       sudo kernel-install -v remove 6.0-rc1-local-gcafec0cacaca0
       # * Note, on some distributions kernel-install is missing
       #   or does only part of the job.

  b) If you performed a bisection and successfully validated the result, feel
     free to remove all kernels built during the actual bisection (Segment 3 c);
     the kernels you built earlier and later you might want to keep around for
     a week or two.
```
```
    git fetch mainline
    git switch --discard-changes --detach mainline/master
    git apply /tmp/foobars-proposed-fix-v1.patch
    cp ~/kernel-config-working .config
    ./scripts/config --set-str CONFIG_LOCALVERSION '-local-foobars-fix-v1'

  Build, install, and boot a kernel as described in *segment 1, section b* --
  but this time omit the first command copying the build configuration over,
  as that has been taken care of already.
```
## 鍏充簬濡備綍楠岃瘉缂洪櫡骞惰繘琛屽洖褰掍簩鍒嗗畾浣嶇殑鍒嗘鎸囧崡


鏈寚鍗椾粙缁嶅浣曟惌寤轰綘鑷繁鐨?Linux 鍐呮牳锛屼互璋冩煡浣犳墦绠楁姤鍛婄殑缂洪櫡鎴栧洖褰掋€備綘鎯冲湪澶氬ぇ绋嬪害涓婇伒寰繖浜涜鏄庯紝鍙栧喅浜庝綘閬囧埌鐨勯棶棰橈細

鎵ц鍒?**绗?1 娈碉紙segment 1锛?* 缁撴潫锛屼互 **楠岃瘉浣犵殑鍐呮牳闂鏄惁鍑虹幇鍦?Linux 鍐呮牳寮€鍙戣€呯淮鎶ょ殑浠ｇ爜涓?*銆傚鏋滄槸锛屼綘灏卞彲浠ュ噯澶囨姤鍛婅缂洪櫡浜嗏€斺€旈櫎闈炲畠鍦ㄦ洿鏃╃殑鍐呮牳鐗堟湰涓苟鏈彂鐢燂紝閭ｆ牱浣犲氨鑷冲皯搴斿綋缁х画 **绗?2 娈碉紙segment 2锛?* 浠?**妫€鏌ヨ闂鏄惁绗﹀悎鍥炲綊锛坮egression锛夌殑瀹氫箟**锛屽洖褰掍細寰楀埌浼樺厛澶勭悊銆傛牴鎹粨鏋滐紝浣犲氨鍙互鍑嗗鎶ュ憡缂洪櫡鎴栨彁浜や竴浠藉垵姝ョ殑鍥炲綊鎶ュ憡锛涗笌鍏舵彁浜ゅ悗鑰咃紝浣犱篃鍙互鐩存帴缁х画 **绗?3 娈碉紙segment 3锛?* 鏉?**鎵ц浜屽垎瀹氫綅**锛屼互鑾峰緱涓€浠藉紑鍙戣€呮湁涔夊姟澶勭悊鐨勫畬鏁村洖褰掓姤鍛娿€?

 Preparations: 鎼缓涓€鍒囦互鏋勫缓浣犺嚜宸辩殑鍐呮牳 <introprep_bissbs>.

 Segment 1: 鐢ㄦ渶鏂扮殑浠ｇ爜搴撳皾璇曞鐜伴棶棰?<introlatestcheck_bissbs>.

 Segment 2: 妫€鏌ヤ綘鏋勫缓鐨勫唴鏍告槸鍚﹀伐浣滄甯?<introworkingcheck_bissbs>.

 Segment 3: 鎵ц浜屽垎瀹氫綅骞堕獙璇佺粨鏋?<introbisect_bissbs>.

 Complementary tasks: 鍦ㄩ伒寰湰鎸囧崡鏈熼棿鍙婁箣鍚庣殑娓呯悊宸ヤ綔 <introclosure_bissbs>.

 Optional tasks: 娴嬭瘯 revert銆佽ˉ涓佹垨鏇存柊鐨勭増鏈?<introoptional_bissbs>.

姣忎釜娈佃惤涓殑姝ラ璇存槑浜嗘祦绋嬬殑閲嶈鏂归潰锛岃€屼竴浠借灏界殑鍙傝€冪珷鑺備负鍑犱箮鎵€鏈夋楠ら兘鎻愪緵浜嗘洿澶氱粏鑺傘€傚弬鑰冪珷鑺傛湁鏃惰繕浼氬垪鍑烘浛浠ｆ柟妗堛€侀櫡闃憋紝浠ュ強鍦ㄨ鐗瑰畾姝ラ鍙兘鍑虹幇鐨勯棶棰樷€斺€斾互鍙婂浣曡浜嬫儏閲嶆柊鍥炲埌姝ｈ建銆?

鍏充簬濡備綍鎶ュ憡 Linux 鍐呮牳闂鎴栧洖褰掔殑鏇村缁嗚妭锛岃鍙傞槄 Documentation/admin-guide/reporting-issues.rst锛屽畠涓庢湰鏂囨。閰嶅悎浣跨敤銆傚叾涓壒鍒В閲婁簡涓轰粈涔堝嵆浣夸綘闈㈠鐨勬槸鏉ヨ嚜鈥渟table/longterm鈥濈郴鍒楋紙渚嬪 6.0.13锛夌殑鍐呮牳闂锛屼篃闇€瑕佺敤鏈€鏂扮殑鈥渕ainline鈥濆唴鏍革紙渚嬪 6.0銆?.1-rc1 鎴?6.1-rc6 绛夌増鏈級鏉ラ獙璇佺己闄枫€?

瀵逛簬閬囧埌鍥炲綊鐨勭敤鎴凤紝璇ユ枃妗ｈ繕瑙ｉ噴浜嗕负浠€涔堝湪绗?2 娈典箣鍚庢彁浜や竴浠藉垵姝ユ姤鍛婃槸鏄庢櫤鐨勶紝鍥犱负璇ュ洖褰掑強鍏?culprit 鍙兘宸茬粡琚煡鏅撱€傚叧浜庣┒绔熶粈涔堟墠绠楀洖褰掔殑鏇村缁嗚妭锛岃鍙傞槄 Documentation/admin-guide/reporting-regressions.rst銆?

濡傛灉浣犲湪閬靛惊鏈寚鍗楁椂閬囧埌浠讳綍闂锛屾垨鑰呮湁濂界偣瀛愭潵鏀硅繘瀹冿紝璇峰憡鐭ュ唴鏍稿紑鍙戣€?<submit_improvements_vbbr>銆?


### Preparations: 鎼缓涓€鍒囦互鏋勫缓浣犺嚜宸辩殑鍐呮牳


浠ヤ笅姝ラ涓烘墍鏈夊悗缁换鍔℃墦涓嬪熀纭€銆?

Note: the instructions assume you are building and testing on the same machine; if you want to compile the kernel on another system, check Build kernels on a different machine <buildhost_bis> below.


- 鍒涘缓涓€浠藉叏鏂扮殑澶囦唤锛屽苟鍑嗗濂界郴缁熶慨澶嶄笌鎭㈠宸ュ叿锛屼互闃蹭竾涓€鍑虹幇鎰忓鎯呭喌銆?

  [details <backup_bisref>]


- 绉婚櫎鎵€鏈変緷璧栧閮ㄥ紑鍙戠殑鍐呮牳椹卞姩銆佹垨浼氬湪鍚姩鏃惰嚜鍔ㄦ瀯寤哄畠浠殑杞欢銆傝繖鍖呮嫭浣嗕笉闄愪簬 DKMS銆乷penZFS銆乂irtualBox锛屼互鍙?Nvidia 鐨勫浘褰㈤┍鍔紙鍖呮嫭鍏?GPL 璁稿彲鐨勫唴鏍告ā鍧楋級銆?

  [details <vanilla_bisref>]


- 鍦ㄥ甫鏈夆€淪ecure Boot鈥濇垨绫讳技鏈哄埗鐨勫钩鍙颁笂锛屽噯澶囧ソ涓€鍒囷紝纭繚绯荤粺鍏佽浣犺嚜琛岀紪璇戠殑鍐呮牳鍚姩銆傚湪鏅€?x86 绯荤粺涓婏紝鏈€蹇嵎绠€渚跨殑鏂规硶鏄湪 BIOS 璁剧疆宸ュ叿涓鐢ㄦ绫绘満鍒讹紱鎴栬€咃紝閫氳繃鐢?`mokutil --disable-validation` 鍙戣捣鐨勬祦绋嬫潵瑙ｉ櫎鍏堕檺鍒躲€?

  [details <secureboot_bisref>]


- 纭畾璐┛鏈寚鍗楄瑙嗕负鈥滆壇濂斤紙good锛夆€濆拰鈥滄崯鍧忥紙bad锛夆€濈殑鍐呮牳鐗堟湰锛?

  - 浣犻伒寰湰鎸囧崡鏄兂楠岃瘉鏌愪釜缂洪櫡鏄惁鍑虹幇鍦ㄤ富瑕佸紑鍙戣€呮墍鍏虫敞鐨勪唬鐮佷腑锛熼偅涔堟妸浣犲綋鍓嶆棩甯镐娇鐢ㄧ殑鏈€鏂板唴鏍哥増鏈涓衡€滆壇濂斤紙good锛夆€濓紙渚嬪 6.0銆?.0.13 鎴?6.1-rc2锛夈€?

  - 浣犻亣鍒颁簡鍥炲綊锛屼緥濡傚湪鍒囨崲鍒拌緝鏂扮殑鍐呮牳鐗堟湰鍚庯紝鏌愪簺鍔熻兘鎹熷潖鎴栬〃鐜板彉宸紵杩欑鎯呭喌涓嬪彇鍐充簬闂鍑虹幇鏃剁殑鐗堟湰鑼冨洿锛?

    - 鍦ㄤ粠鏌愪釜 stable/longterm 鐗堟湰锛堜緥濡?6.0.13锛夋洿鏂板埌鏇存柊鐨?mainline 绯诲垪锛堝 6.1-rc7 鎴?6.1锛夛紝鎴栧熀浜庡畠鐨?stable/longterm 鐗堟湰锛堝 6.1.5锛夋椂鍙戠敓浜嗗洖褰掞紵閭ｄ箞鎶婁綘鍙敤鍐呮牳鎵€鍩轰簬鐨?mainline 鐗堟湰瑙嗕负鈥滆壇濂斤紙good锛夆€濈増鏈紙渚嬪 6.0锛夛紝骞跺皢绗竴涓崯鍧忕殑鐗堟湰瑙嗕负鈥滄崯鍧忥紙bad锛夆€濈増鏈紙渚嬪 6.1-rc7銆?.1 鎴?6.1.5锛夈€傛敞鎰忥紝姝ゆ椂浠呬粎鏄亣璁?6.0 娌￠棶棰橈紱杩欎竴鍋囪灏嗗湪绗?2 娈典腑妫€楠屻€?

    - 鍦ㄤ粠涓€涓?mainline 鐗堟湰锛堜緥濡?6.0锛夊垏鎹㈠埌鏇存柊鐨勭増鏈紙濡?6.1-rc1锛夋垨鍩轰簬瀹冪殑 stable/longterm 鐗堟湰锛堝 6.1.5锛夋椂鍙戠敓浜嗗洖褰掞紵閭ｄ箞灏嗘渶鍚庝竴涓彲鐢ㄧ増鏈紙渚嬪 6.0锛夎涓衡€滆壇濂斤紙good锛夆€濓紝灏嗙涓€涓崯鍧忕増鏈紙渚嬪 6.1-rc1 鎴?6.1.5锛夎涓衡€滄崯鍧忥紙bad锛夆€濄€?

    - 鍦?stable/longterm 绯诲垪鍐呴儴鏇存柊鏃讹紙渚嬪浠?6.0.13 鍒?6.0.15锛夊彂鐢熶簡鍥炲綊锛熼偅涔堝皢杩欎簺鐗堟湰瑙嗕负鈥滆壇濂斤紙good锛夆€濆拰鈥滄崯鍧忥紙bad锛夆€濓紙渚嬪 6.0.13 鍜?6.0.15锛夛紝鍥犱负浣犻渶瑕佸湪璇ョ郴鍒楀唴閮ㄨ繘琛屼簩鍒嗗畾浣嶃€?

  *娉ㄦ剰锛屼笉瑕佹妸鈥滆壇濂斤紙good锛夆€濈増鏈笌鈥滃彲鐢紙working锛夆€濆唴鏍告贩娣嗭紱鍚庝竴涓湳璇湪鏁寸瘒鎸囧崡涓寚鐨勬槸鏈€鍚庝竴涓竴鐩存甯稿伐浣滅潃鐨勫唴鏍搞€?

  [details <rangecheck_bisref>]


- 鍚姩杩涘叆鈥滃彲鐢紙working锛夆€濆唴鏍革紝骞剁畝鍗曡瘯鐢ㄤ竴涓嬫槑鏄惧凡鎹熷潖鐨勫姛鑳姐€?

  [details <bootworking_bisref>]


- 纭繚鏈夎冻澶熺殑鍙敤绌洪棿鏉ユ瀯寤?Linux銆備富鐩綍涓?15 GB 閫氬父宸茬粡瓒冲銆傚鏋滀綘鍙敤绌洪棿鏇村皯锛岃鍔″繀鐣欐剰鍚庣画鍏充簬鑾峰彇 Linux 婧愮爜鍜屽鐞嗚皟璇曠鍙风殑姝ラ锛氫袱鑰呴兘浠嬬粛浜嗚兘鍑忓皯绌洪棿鍗犵敤鐨勫姙娉曪紝搴斿綋鑳借浣犲湪绾?4 GB 鍙敤绌洪棿鐨勬儏鍐典笅瀹屾垚杩欎簺浠诲姟銆?

  [details <diskspace_bisref>]


- 瀹夎鏋勫缓 Linux 鍐呮牳鎵€闇€鐨勬墍鏈夎蒋浠躲€傞€氬父浣犱細闇€瑕侊細'bc'銆?binutils'锛?ld' 绛夛級銆?bison'銆?flex'銆?gcc'銆?git'銆?openssl'銆?pahole'銆?perl'锛屼互鍙?'libelf' 鍜?'openssl' 鐨勫紑鍙戝ご鏂囦欢銆傚弬鑰冪珷鑺傚睍绀轰簡濡備綍鍦ㄥ绉嶆祦琛岀殑 Linux 鍙戣鐗堜笂蹇€熷畨瑁呭畠浠€?

  [details <buildrequires_bisref>]


- 鑾峰彇 mainline Linux 婧愪唬鐮侊紱鐒跺悗杩涘叆瀛樻斁杩欎簺浠ｇ爜鐨勭洰褰曪紝鍥犱负鏈寚鍗楀悗缁墍鏈夊懡浠ら兘搴斾粠璇ョ洰褰曟墽琛屻€?

  *娉ㄦ剰锛屼笅闈粙缁嶇殑鏄€氳繃瀹屾暣鐨?mainline 鍏嬮殕鏉ヨ幏鍙栨簮浠ｇ爜锛屾埅鑷?2024 骞村垵鍏朵笅杞介噺绾︿负 2.75 GB銆? 鍙傝€冨皬鑺備粙缁嶄簡涓ょ鏇夸唬鏂规 <sources_bisref> *锛氫竴绉嶄笅杞介噺涓嶅埌 500 MB锛屽彟涓€绉嶆洿閫傚悎缃戠粶涓嶇ǔ瀹氱殑杩炴帴銆?

  鎵ц浠ヤ笅鍛戒护浠ヨ幏鍙栦竴浠藉叏鏂扮殑 mainline 浠ｇ爜搴擄細
```

    git clone -o mainline --no-checkout \
      https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git ~/linux/
    cd ~/linux/
    git remote add -t master stable \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git

  [:ref:`details <sources_bisref>`]

```

- 浣犱箣鍓嶇‘瀹氱殑鈥滆壇濂斤紙good锛夆€濇垨鈥滄崯鍧忥紙bad锛夆€濈増鏈腑锛屾槸鍚︽湁涓€涓槸 stable 鎴?longterm 鍙戣鐗堬紙濡?6.1.5锛夛紵閭ｄ箞锛岃涓嬭浇瀹冩墍灞炵郴鍒楃殑婧愪唬鐮?
```

    git remote set-branches --add stable linux-6.1.y
    git fetch stable

```

- 寮€濮嬪噯澶囧唴鏍告瀯寤洪厤缃紙鍗?'.config' 鏂囦欢锛夈€?

  鍦ㄦ涔嬪墠锛岃纭浣犱粛鍦ㄨ繍琛屾棭鍓嶆楠よ姹備綘鍚姩鐨勨€滃彲鐢紙working锛夆€濆唴鏍革紱濡傛灉涓嶇‘瀹氾紝鍙敤 `uname -r` 鏌ョ湅褰撳墠鐨?kernelrelease 鏍囪瘑绗︺€?

  涔嬪悗锛屾鍑烘棭鍓嶇‘瀹氫负鈥滆壇濂斤紙good锛夆€濈殑鐗堟湰瀵瑰簲鐨勬簮浠ｇ爜銆備笅闈㈢殑绀轰緥鍛戒护鍋囧畾璇ョ増鏈负 6.0锛涜娉ㄦ剰锛屾湰鍛戒护鍙婂悗缁墍鏈?Git 鍛戒护涓殑鐗堟湰鍙烽兘闇€瑕佸姞涓婂墠缂€
```

    git switch --discard-changes --detach v6.0

  Now create a build configuration file::

    make olddefconfig

  The kernel build scripts then will try to locate the build configuration file for the running kernel and then adjust it for the needs of the kernel sources you checked out. While doing so, it will print a few lines you need to check.

  Look out for a line starting with '# using defaults found in'. It should be followed by a path to a file in '/boot/' that contains the release identifier of your currently working kernel. If the line instead continues with something like 'arch/x86/configs/x86_64_defconfig', then the build infra failed to find the .config file for your running kernel -- in which case you have to put one there manually, as explained in the reference section.

  In case you can not find such a line, look for one containing '# configuration written to .config'. If that's the case you have a stale build configuration lying around. Unless you intend to use it, delete it; afterwards run 'make olddefconfig' again and check if it now picked up the right config file as base.

  [:ref:`details <oldconfig_bisref>`]

```

- 绂佺敤閭ｄ簺瀵逛綘鐨勯厤缃€岃█鏄庢樉澶氫綑鐨勪换鎰忓唴鏍告ā鍧椼€傝繖涓€姝ユ槸鍙€夌殑锛屼絾瀵逛簬浜屽垎瀹氫綅灏ゅ叾鏄庢櫤锛屽洜涓哄畠鑳芥瀬澶у湴鍔犲揩鏋勫缓杩囩▼鈥斺€旈櫎闈炰笂涓€姝ュ彇寰楃殑 .config 鏂囦欢宸茬粡閽堝浣犲拰浣犵殑纭欢闇€姹傚仛浜嗗畾鍒讹紝閭ｆ牱鐨勮瘽浣犲簲璺宠繃姝ゆ銆?

  涓哄噯澶囩簿绠€锛岃杩炴帴浣犲伓灏斾娇鐢ㄧ殑澶栭儴纭欢锛圲SB 瀵嗛挜銆佷护鐗岀瓑锛夛紝蹇€熷惎鍔ㄤ竴涓?VM锛屽苟鍚敤 VPN銆傚鏋滀綘鍦ㄥ紑濮嬮伒寰湰鎸囧崡鍚庨噸鍚繃锛岃纭繚宸茬粡灏濊瘯浣跨敤閭ｄ釜瀵艰嚧
```

     yes '' | make localmodconfig

  There is a catch to this, as the 'apparently' in initial sentence of this step and the preparation instructions already hinted at:

  鈥渓ocalmodconfig鈥濈洰鏍囧緢瀹规槗绂佺敤閭ｄ簺浠呭伓灏斾娇鐢ㄧ殑鍔熻兘瀵瑰簲鐨勫唴鏍告ā鍧椻€斺€斾緥濡傝嚜鍚姩浠ユ潵灏氭湭杩炴帴鐨勫閮ㄥ璁剧殑妯″潡銆佸皻鏈娇鐢ㄧ殑铏氭嫙鍖栬蒋浠躲€乂PN 闅ч亾锛屼互鍙婂叾浠栦竴浜涗笢瑗裤€傝繖鏄洜涓烘煇浜涗换鍔′緷璧栫殑鍐呮牳妯″潡鍙湁鍦ㄤ綘棣栨鎵ц杩欑被浠诲姟鏃讹紝Linux 鎵嶄細鍔犺浇銆?

  localmodconfig 鐨勮繖涓€缂虹偣骞朵笉鍊煎緱浣犲咖蹇冿紝浣嗗簲褰撹鍦ㄥ績閲岋細濡傛灉鏈寚鍗楁瀯寤虹殑鍐呮牳鍑虹幇鏌愮寮傚父琛屼负锛岃繖寰堝彲鑳藉氨鏄師鍥犮€備綘鍙互鐢ㄥ弬鑰冨皬鑺備腑鍒楀嚭鐨勬妧宸ф潵闄嶄綆鎴栧嚑涔庢秷闄よ繖涓€椋庨櫓锛涗絾濡傛灉鏄粎涓哄揩閫熸祴璇曡€屾瀯寤哄唴鏍革紝鍙瀹冭兘鍚姩骞惰浣犳甯告祴璇曞嚭闂鐨勫姛鑳斤紝閫氬父涓嶅€煎緱鍦ㄦ涓婅姳璐瑰お澶氱簿鍔涖€?

  [:ref:`details <localmodconfig_bisref>`]

```

- 纭繚浣犲皢鏋勫缓鐨勬墍鏈夊唴鏍搁兘鑳介€氳繃涓€绉嶇壒娈?
```

    ./scripts/config --set-str CONFIG_LOCALVERSION '-local'
    ./scripts/config -e CONFIG_LOCALVERSION_AUTO

  [:ref:`details <tagging_bisref>`]

```

- 鍐冲畾濡備綍澶勭悊璋冭瘯绗﹀彿銆?

  灏辨湰鏂囨。鑰岃█锛岄€氬父鏄庢櫤鐨勫仛娉曟槸鍚敤瀹冧滑锛屽洜涓轰綘寰堟湁鍙兘浼氶渶瑕佷粠涓€涓€減anic鈥濄€佲€淥ops鈥濄€?
```

    ./scripts/config -d DEBUG_INFO_NONE -e KALLSYMS_ALL -e DEBUG_KERNEL \
      -e DEBUG_INFO -e DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT -e KALLSYMS

  But if you are extremely short on storage space, you might want to disable debug symbols instead::

    ./scripts/config -d DEBUG_INFO -d DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT \
      -d DEBUG_INFO_DWARF4 -d DEBUG_INFO_DWARF5 -e CONFIG_DEBUG_INFO_NONE

  [:ref:`details <debugsymbols_bisref>`]

```

- 妫€鏌ヤ綘鏄惁鎯宠鎴栭渶瑕佽皟鏁村叾浠栦竴浜涘唴鏍搁厤缃€夐」锛?

  - 浣犱娇鐢ㄧ殑鏄?Debian 鍚楋紵閭ｄ箞锛屼綘鍙兘甯屾湜閫氳繃鎵ц鍙傝€冨皬鑺備腑浠嬬粛鐨勯澶栬皟鏁存潵閬垮厤宸茬煡闂銆?

    [details <configmods_distros_bisref>].

  - 濡傛灉浣犳兂褰卞搷閰嶇疆鐨勫叾浠栨柟闈紝鐜板湪灏辩敤浣犲枩娆㈢殑宸ュ叿鍘诲仛銆傛敞鎰忥紝瑕佷娇鐢?'menuconfig' 鎴?'nconfig' 杩欐牱鐨?make 鐩爣锛屼綘闇€瑕佸畨瑁?ncurses 鐨勫紑鍙戞枃浠讹紱瀵逛簬 'xconfig'锛屼綘鍚屾牱闇€瑕?Qt5 鎴?Qt6 鐨勫ご鏂囦欢銆?

    [details <configmods_individual_bisref>].

- 鍦ㄦ渶鏂拌皟鏁翠箣鍚庨噸鏂板鐞?.config锛屽苟灏嗗叾淇濆瓨鍦ㄥ畨鍏ㄧ殑
```

     make olddefconfig
     cp .config ~/kernel-config-working

  [:ref:`details <saveconfig_bisref>`]

```

### 绗?1 娈碉細灏濊瘯鐢ㄦ渶鏂扮殑浠ｇ爜搴撳鐜伴棶棰?


浠ヤ笅姝ラ鐢ㄤ簬纭闂鏄惁鍑虹幇鍦ㄥ紑鍙戣€呭綋鍓嶇淮鎶ょ殑浠ｇ爜涓€傚鏋滀綘閬囧埌鐨勬槸鍥炲綊闂锛屽畠杩樿兘纭闂涓嶆槸鐢辨煇浜?.config 鍙樻洿寮曡捣鐨勶紝鍚﹀垯鎶ュ憡璇ラ棶棰樺氨鏄湪娴垂鏃堕棿銆?[details <introlatestcheck_bisref>]


- 妫€鍑烘渶鏂扮殑 Linux 浠ｇ爜搴撱€?

  - 浣犵殑鈥滆壇濂斤紙good锛夆€濆拰鈥滄崯鍧忥紙bad锛夆€濈増鏈槸鍚︽潵鑷悓涓€涓?stable 鎴?longterm 绯诲垪锛熼偅涔堣鏌ョ湅 `kernel.org 棣栭〉 <https://kernel.org/>`_锛氬鏋滃畠鍒楀嚭浜嗚绯诲垪涓竴涓笉甯︹€淸EOL]鈥濇爣绛剧殑鍙戣鐗堬紝灏辨鍑鸿绯诲垪
```

      cd ~/linux/
      git switch --discard-changes --detach stable/linux-6.1.y

    Your series is unsupported, if is not listed or carrying a 'end of life' tag. In that case you might want to check if a successor series (say linux-6.2.y) or mainline (see next point) fix the bug.

  * In all other cases, run::

      cd ~/linux/
      git switch --discard-changes --detach mainline/master

  [:ref:`details <checkoutmaster_bisref>`]

```

- 浣跨敤浣犲噯澶囩殑閰嶇疆鏂囦欢鏋勫缓绗竴涓唴鏍哥殑闀滃儚涓庢ā鍧楋細
```

    cp ~/kernel-config-working .config
    make olddefconfig
    make -j $(nproc --all)

  If you want your kernel packaged up as deb, rpm, or tar file, see the reference section for alternatives, which obviously will require other steps to install as well.

  [:ref:`details <build_bisref>`]

```

- 瀹夎浣犳柊鏋勫缓鐨勫唴鏍搞€?
```

    df -h /boot/ /lib/modules/

  For now assume 150 MByte in /boot/ and 200 in /lib/modules/ will suffice; how much your kernels actually require will be determined later during this guide.

  Now install the kernel's modules and its image, which will be stored in parallel to the your Linux distribution's kernels::

    sudo make modules_install
    command -v installkernel && sudo make install

  The second command ideally will take care of three steps required at this point: copying the kernel's image to /boot/, generating an initramfs, and adding an entry for both to the boot loader's configuration.

  Sadly some distributions (among them Arch Linux, its derivatives, and many immutable Linux distributions) will perform none or only some of those tasks. You therefore want to check if all of them were taken care of and manually perform those that were not. The reference section provides further details on that; your distribution's documentation might help, too.

  Once you figured out the steps needed at this point, consider writing them down: if you will build more kernels as described in segment 2 and 3, you will have to perform those again after executing ``command -v installkernel [...]``.

  [:ref:`details <install_bisref>`]

```

- 濡傛灉浣犳墦绠楃户缁伒寰湰鎸囧崡锛岃妫€鏌ラ渶瑕佸灏戝瓨鍌ㄧ┖闂?
```

    du -ch /boot/*$(make -s kernelrelease)* | tail -n 1
    du -sh /lib/modules/$(make -s kernelrelease)/

  Write down or remember those two values for later: they enable you to prevent running out of disk space accidentally during a bisection.

  [:ref:`details <storagespace_bisref>`]

```

```

    make -s kernelrelease | tee -a ~/kernels-built

  Remember the identifier momentarily, as it will help you pick the right kernel from the boot menu upon restarting.

```

- 閲嶅惎杩涘叆浣犳柊鏋勫缓鐨勫唴鏍搞€備负纭繚浣犲惎鍔ㄧ殑纭疄鏄綘鍒氭瀯寤虹殑閭ｄ釜锛屼綘鍙兘鎯抽獙璇佽繖浜涘懡浠ょ殑杈撳嚭
```

    tail -n 1 ~/kernels-built
    uname -r

```

```

    cat /proc/sys/kernel/tainted

  If that command does not return '0', check the reference section, as the cause for this might interfere with your testing.

  [:ref:`details <tainted_bisref>`]

```

- 楠岃瘉浣犳瀯寤虹殑鏂板唴鏍告槸鍚﹀嚭鐜颁簡璇ョ己闄枫€傚鏋滄病鏈夛紝璇锋煡闃呭弬鑰冨皬鑺備腑鐨勮鏄庯紝浠ョ‘淇濅綘鐨勬祴璇曡繃绋嬩腑娌℃湁鍑哄矓瀛愩€?

  [details <recheckbroken_bisref>]


- 浣犲垰鏋勫缓鐨勬槸 stable 鎴?longterm 鍐呮牳鍚楋紵骞朵笖浣犺兘鍚︾敤瀹冨鐜拌鍥炲綊锛熼偅涔堜綘涔熷簲褰撴祴璇曟渶鏂扮殑 mainline 浠ｇ爜搴擄紝鍥犱负缁撴灉鍐冲畾浜嗚缂洪櫡蹇呴』鎻愪氦缁欏摢浜涘紑鍙戣€呫€?
```

    cd ~/linux/
    git switch --discard-changes --detach mainline/master

  Now use the checked out code to build and install another kernel using the commands the earlier steps already described in more detail::

    cp ~/kernel-config-working .config
    make olddefconfig
    make -j $(nproc --all)
    # * Check if the free space suffices holding another kernel:
    df -h /boot/ /lib/modules/
    sudo make modules_install
    command -v installkernel && sudo make install
    make -s kernelrelease | tee -a ~/kernels-built
    reboot

  Confirm you booted the kernel you intended to start and check its tainted status::

    tail -n 1 ~/kernels-built
    uname -r
    cat /proc/sys/kernel/tainted

  Now verify if this kernel is showing the problem. If it does, then you need to report the bug to the primary developers; if it does not, report it to the stable team. See Documentation/admin-guide/reporting-issues.rst for details.

  [:ref:`details <recheckstablebroken_bisref>`]

```

浣犻伒寰湰鎸囧崡鏄负浜嗛獙璇佹煇涓棶棰樻槸鍚﹀瓨鍦ㄤ簬 Linux 鍐呮牳寮€鍙戣€呭綋鍓嶇淮鎶ょ殑浠ｇ爜涓悧锛熼偅涔堝埌姝や綘灏卞畬鎴愪簡銆傚鏋滀綘涔嬪悗鎯冲垹闄ゅ垰鏋勫缓鐨勫唴鏍革紝璇峰弬闃呪€滆ˉ鍏呬换鍔★細閬靛惊鏈寚鍗楁湡闂村強涔嬪悗鐨勬竻鐞嗗伐浣?<introclosure_bissbs>鈥濄€?

濡傛灉浣犻亣鍒扮殑鏄洖褰掗棶棰橈紝璇风户缁苟鑷冲皯鎵ц涓嬩竴娈点€?


### 绗?2 娈碉細妫€鏌ヤ綘鏋勫缓鐨勫唴鏍告槸鍚﹀伐浣滄甯?


濡傛灉鏄洖褰掗棶棰橈紝浣犵幇鍦ㄩ渶瑕佺‘淇濇棭鍓嶅垱寤虹殑绮剧畝閰嶇疆鏂囦欢鎸夐鏈熷伐浣滐紱鍚﹀垯鐢ㄥ畠杩涜浜屽垎瀹氫綅灏辨槸鍦ㄦ氮璐规椂闂淬€?[details <introworkingcheck_bisref>]


- 鏋勫缓浣犺嚜宸辩殑鈥滃彲鐢紙working锛夆€濆唴鏍稿彉浣擄紝骞舵鏌ラ偅涓彂鐢熷洖褰掔殑鍔熻兘鍦ㄥ畠涓婇潰鏄惁鎸夐鏈熷伐浣溿€?

  棣栧厛妫€鍑烘棭鍓嶇‘瀹氫负
```

    cd ~/linux/
    git switch --discard-changes --detach v6.0

  Now use the checked out code to configure, build, and install another kernel using the commands the previous subsection explained in more detail::

    cp ~/kernel-config-working .config
    make olddefconfig
    make -j $(nproc --all)
    # * Check if the free space suffices holding another kernel:
    df -h /boot/ /lib/modules/
    sudo make modules_install
    command -v installkernel && sudo make install
    make -s kernelrelease | tee -a ~/kernels-built
    reboot

  When the system booted, you may want to verify once again that the kernel you started is the one you just built::

    tail -n 1 ~/kernels-built
    uname -r

  Now check if this kernel works as expected; if not, consult the reference section for further instructions.

  [:ref:`details <recheckworking_bisref>`]

```

### 绗?3 娈碉細鎵ц浜屽垎瀹氫綅骞堕獙璇佺粨鏋?


鍦ㄥ畬鎴愪簡鎵€鏈夊噯澶囧伐浣滃拰棰勯槻鎬ф瀯寤轰箣鍚庯紝浣犵幇鍦ㄥ彲浠ュ紑濮嬩簩鍒嗗畾浣嶄簡銆傝繖浼氳浣犳瀯寤虹浉褰撳鐨勫唴鏍糕€斺€旈€氬父绾︿负 15 涓紝濡傛灉浣犳槸鍦ㄦ洿鏂板埌杈冩柊绯诲垪锛堝浠?6.0.13 鍒?6.1.5锛夋椂閬囧埌鐨勫洖褰掋€備絾涓嶇敤鎷呭績锛岀敱浜庢棭鍓嶅垱寤虹殑绮剧畝鏋勫缓閰嶇疆锛岃繖涓繃绋嬫瘮璁稿浜烘兂璞＄殑瑕佸揩寰楀锛氬湪鏅€?x86 鏈哄櫒涓婏紝骞冲潎鏉ヨ缂栬瘧姣忎釜鍐呮牳閫氬父鍙渶绾?10 鍒?15 鍒嗛挓銆?


- 寮€濮嬩簩鍒嗗畾浣嶏紝骞跺憡鐭?Git 鏃╁墠纭畾鐨勭増鏈?
```

    cd ~/linux/
    git bisect start
    git bisect good v6.0
    git bisect bad v6.1.5

  [:ref:`details <bisectstart_bisref>`]

```

- 鐜板湪鐢?Git 妫€鍑虹殑浠ｇ爜锛屽€熷姪
```

    cp ~/kernel-config-working .config
    make olddefconfig
    make -j $(nproc --all)
    # * Check if the free space suffices holding another kernel:
    df -h /boot/ /lib/modules/
    sudo make modules_install
    command -v installkernel && sudo make install
    make -s kernelrelease | tee -a ~/kernels-built
    reboot

  If compilation fails for some reason, run ``git bisect skip`` and restart executing the stack of commands from the beginning.

  In case you skipped the 'test latest codebase' step in the guide, check its description as for why the 'df [...]' and 'make -s kernelrelease [...]' commands are here.

  Important note: the latter command from this point on will print release identifiers that might look odd or wrong to you -- which they are not, as it's totally normal to see release identifiers like '6.0-rc1-local-gcafec0cacaca0' if you bisect between versions 6.1 and 6.2 for example.

  [:ref:`details <bisectbuild_bisref>`]

```

- 鐜板湪妫€鏌ラ偅涓彂鐢熷洖褰掔殑鍔熻兘鍦ㄤ綘鍒氭瀯寤虹殑鍐呮牳涓槸鍚﹀伐浣滄甯搞€?

   浣犲彲鑳借繕鏄兂鍏堢‘璁や綘鍚姩鐨勫唴鏍告鏄綘鏋勫缓鐨勯偅涓?
```

    cd ~/linux/
    tail -n 1 ~/kernels-built
    uname -r

  Now verify if the feature that regressed works at this kernel bisection point.
  If it does, run this::

    git bisect good

  If it does not, run this::

    git bisect bad

  Be sure about what you tell Git, as getting this wrong just once will send the rest of the bisection totally off course.

  While the bisection is ongoing, Git will use the information you provided to find and check out another bisection point for you to test. While doing so, it will print something like 'Bisecting: 675 revisions left to test after this (roughly 10 steps)' to indicate how many further changes it expects to be tested. Now build and install another kernel using the instructions from the previous step; afterwards follow the instructions in this step again.

  Repeat this again and again until you finish the bisection -- that's the case when Git after tagging a change as 'good' or 'bad' prints something like 'cafecaca0c0dacafecaca0c0dacafecaca0c0da is the first bad commit'; right afterwards it will show some details about the culprit including the patch description of the change. The latter might fill your terminal screen, so you might need to scroll up to see the message mentioning the culprit; alternatively, run ``git bisect log > ~/bisection-log``.

  [:ref:`details <bisecttest_bisref>`]

```

- 鍦ㄤ互涓嬫搷浣滀箣鍓嶏紝鍏堝皢 Git 鐨勪簩鍒嗘棩蹇椾笌褰撳墠鐨?.config 鏂囦欢淇濆瓨鍦ㄥ畨鍏ㄧ殑鍦版柟
```

    cd ~/linux/
    git bisect log > ~/bisection-log
    cp .config ~/bisection-config-culprit
    git bisect reset

  [:ref:`details <bisectlog_bisref>`]

```

- 灏濊瘯鍦ㄦ渶鏂?mainline 涔嬩笂鍥為€€缃瓉绁搁锛岀湅鏄惁鑳戒慨澶嶄綘鐨勫洖褰掋€?

  This is optional, as it might be impossible or hard to realize. The former is the case, if the bisection determined a merge commit as the culprit; the latter happens if other changes depend on the culprit. But if the revert succeeds, it is worth building another kernel, as it validates the result of a bisection, which can easily deroute; it furthermore will let kernel developers know, if they can resolve the regression with a quick revert.

  Begin by checking out the latest codebase depending on the range you bisected:

  - Did you face a regression within a stable/longterm series (say between 6.0.13 and 6.0.15) that does not happen in mainline? Then check out the
```

      git fetch stable
      git switch --discard-changes --detach linux-6.0.y

  * In all other cases check out latest mainline::

      git fetch mainline
      git switch --discard-changes --detach mainline/master

    If you bisected a regression within a stable/longterm series that also happens in mainline, there is one more thing to do: look up the mainline commit-id. To do so, use a command like ``git show abcdcafecabcd`` to view the patch description of the culprit. There will be a line near the top which looks like 'commit cafec0cacaca0 upstream.' or 'Upstream commit cafec0cacaca0'; use that commit-id in the next command and not the one the bisection blamed.

  Now try reverting the culprit by specifying its commit id::

    git revert --no-edit cafec0cacaca0

  If that fails, give up trying and move on to the next step; if it works, adjust the tag to facilitate the identification and prevent accidentally overwriting another kernel::

    cp ~/kernel-config-working .config
    ./scripts/config --set-str CONFIG_LOCALVERSION '-local-cafec0cacaca0-reverted'

  Build a kernel using the familiar command sequence, just without copying the the base .config over::

    make olddefconfig &&
    make -j $(nproc --all)
    # * Check if the free space suffices holding another kernel:
    df -h /boot/ /lib/modules/
    sudo make modules_install
    command -v installkernel && sudo make install
    make -s kernelrelease | tee -a ~/kernels-built
    reboot

  Now check one last time if the feature that made you perform a bisection works with that kernel: if everything went well, it should not show the regression.

  [:ref:`details <revert_bisref>`]

```

### 琛ュ厖浠诲姟锛氫簩鍒嗗畾浣嶆湡闂村強涔嬪悗鐨勬竻鐞嗗伐浣?


鍦ㄩ伒寰湰鎸囧崡鏈熼棿鍙婁箣鍚庯紝浣犲彲鑳芥兂瑕佹垨闇€瑕佸垹闄や竴浜涘凡瀹夎鐨勫唴鏍革細鍚﹀垯鍚姩鑿滃崟浼氬彉寰楁贩涔憋紝鎴栬€呯┖闂村彲鑳借€楀敖銆?


- 瑕佸垹闄ゆ煇涓凡瀹夎鐨勫唴鏍革紝鍏堟煡瀹冪殑鈥渒ernelrelease鈥濇爣璇嗙銆傛湰鎸囧崡灏嗗畠浠繚瀛樺湪鈥渵/kernels-built鈥濅腑锛屼絾涔熷彲浠ュ€熷姪浠ヤ笅
```

    ls -ltr /lib/modules/*-local*

  You in most situations want to remove the oldest kernels built during the actual bisection (e.g. segment 3 of this guide). The two ones you created beforehand (e.g. to test the latest codebase and the version considered 'good') might become handy to verify something later -- thus better keep them around, unless you are really short on storage space.

  To remove the modules of a kernel with the kernelrelease identifier '*6.0-rc1-local-gcafec0cacaca0*', start by removing the directory holding its modules::

    sudo rm -rf /lib/modules/6.0-rc1-local-gcafec0cacaca0

  Afterwards try the following command::

    sudo kernel-install -v remove 6.0-rc1-local-gcafec0cacaca0

  On quite a few distributions this will delete all other kernel files installed while also removing the kernel's entry from the boot menu. But on some distributions kernel-install does not exist or leaves boot-loader entries or kernel image and related files behind; in that case remove them as described in the reference section.

  [:ref:`details <makeroom_bisref>`]

```

- 涓€鏃﹀畬鎴愪簩鍒嗗畾浣嶏紝涓嶈绔嬪嵆鍒犻櫎浣犳惌寤虹殑浠讳綍涓滆タ锛屽洜涓轰綘鍙兘杩橀渶瑕佺敤鍒板叾涓竴浜涖€傚摢浜涘彲浠ュ畨鍏ㄥ垹闄わ紝鍙栧喅浜庝簩鍒嗗畾浣嶇殑缁撴灉锛?

  - 浣犳渶鍒濊兘鍚︾敤鏈€鏂颁唬鐮佸簱澶嶇幇璇ュ洖褰掞紝骞朵笖鍦ㄤ簩鍒嗗畾浣嶅悗閫氳繃鍥為€€鏈€鏂颁唬鐮佸簱涔嬩笂鐨勭姜榄佺ジ棣栦慨澶嶄簡闂锛熼偅涔堜綘鍙兘鎯虫妸閭ｄ袱涓唴鏍镐繚鐣欎竴娈垫椂闂达紝浣嗗畨鍏ㄥ湴鍒犻櫎鎵€鏈夊彂琛屾爣璇嗙涓寘鍚€?local鈥濈殑鍏朵粬鍐呮牳銆?

  - 浜屽垎瀹氫綅鏄惁缁堢粨浜庝竴涓悎骞舵彁浜わ紝鎴栧洜鍏朵粬鍘熷洜鏄惧緱鍙枒锛熼偅涔堜綘鍙兘鎯冲敖鍙兘澶氬湴淇濈暀鍐呮牳鍑犲ぉ锛氫綘寰堝彲鑳戒細琚姹傞噸鏂版鏌ユ煇浜涗笢瑗裤€?

  - 鍦ㄥ叾浠栨儏鍐典笅锛屾渶濂藉皢浠ヤ笅鍐呮牳淇濈暀涓€娈垫椂闂达細浠庢渶鏂颁唬鐮佸簱鏋勫缓鐨勯偅涓€佺敤琚涓衡€滆壇濂斤紙good锛夆€濈殑鐗堟湰鍒涘缓鐨勯偅涓€涓紝浠ュ強浣犲湪瀹為檯浜屽垎杩囩▼涓紪璇戠殑鏈€鍚庝笁鍥涗釜銆?

  [details <finishingtouch_bisref>]


### 鍙€夛細娴嬭瘯鍥為€€銆佽ˉ涓佹垨鏇撮珮鐗堟湰


鍦ㄦ姤鍛婄己闄锋湡闂存垨涔嬪悗锛屼綘鍙兘鎯宠銆佷篃鍙兘浼氳瑕佹眰鍘绘祴璇曞洖閫€銆佽皟璇曡ˉ涓併€佹彁璁殑淇锛屾垨鍏朵粬鐗堟湰銆傝繖绉嶆儏鍐典笅锛岃閬靛惊浠ヤ笅璇存槑銆?

- 鏇存柊浣犵殑 Git 鍏嬮殕骞舵鍑烘渶鏂颁唬鐮併€?

  - 濡傛灉浣犳兂娴嬭瘯 mainline锛岃鍦ㄦ鍑哄墠鍏堣幏鍙栧叾鏈€鏂板彉鏇?
```

      git fetch mainline
      git switch --discard-changes --detach mainline/master

  * In case you want to test a stable or longterm kernel, first add the branch holding the series you are interested in (6.2 in the example), unless you already did so earlier::

      git remote set-branches --add stable linux-6.2.y

    Then fetch the latest changes and check out the latest version from the series::

      git fetch stable
      git switch --discard-changes --detach stable/linux-6.2.y

```

```

    cp ~/kernel-config-working .config

```

- 浣犵殑涓嬩竴姝ュ彇鍐充簬浣犳兂鍋氫粈涔堬細

  - 濡傛灉浣犲彧鏄兂娴嬭瘯鏈€鏂颁唬鐮佸簱锛岀洿鎺ヨ繘鍏ヤ笅涓€姝ュ嵆鍙紝浣犲凡缁忓噯澶囧氨缁€?

  - 濡傛灉浣犳兂娴嬭瘯鍥為€€鏄惁鑳戒慨澶嶆煇涓棶棰橈紝璇峰洖閫€涓€涓垨澶氫釜
```

      git revert --no-edit cafec0cacaca0

    Now give that kernel a special tag to facilitates its identification and prevent accidentally overwriting another kernel::

      ./scripts/config --set-str CONFIG_LOCALVERSION '-local-cafec0cacaca0-reverted'

  * In case you want to test a patch, store the patch in a file like '/tmp/foobars-proposed-fix-v1.patch' and apply it like this::

      git apply /tmp/foobars-proposed-fix-v1.patch

    In case of multiple patches, repeat this step with the others.

    Now give that kernel a special tag to facilitates its identification and prevent accidentally overwriting another kernel::

    ./scripts/config --set-str CONFIG_LOCALVERSION '-local-foobars-fix-v1'

```

- 浣跨敤鐔熸倝鐨勫懡浠ゆ瀯寤哄唴鏍革紝鍙槸涓嶈澶嶅埗鍐呮牳
```

    make olddefconfig &&
    make -j $(nproc --all)
    # * Check if the free space suffices holding another kernel:
    df -h /boot/ /lib/modules/
    sudo make modules_install
    command -v installkernel && sudo make install
    make -s kernelrelease | tee -a ~/kernels-built
    reboot

```

- 鐜板湪纭浣犲惎鍔ㄧ殑鏄柊鏋勫缓鐨勫唴鏍稿苟妫€鏌ュ畠銆?

[details <introoptional_bisref>]


### 缁撹


浣犲凡鍒拌揪鍒嗘鎸囧崡鐨勭粨灏俱€?

浣犲湪閬靛惊鍒嗘鎸囧崡鏃舵槸鍚﹂亣鍒颁簡鍙傝€冨皬鑺傛湭鑳借В鍐崇殑楹荤儲锛熸槸鍚﹀彂鐜颁簡閿欒锛熸垨鑰呮槸鍚︽湁鏀硅繘鎸囧崡鐨勬兂娉曪紵

濡傛灉鏈変笂杩颁换浣曟儏鍐碉紝璇烽€氳繃鍙戦€佺畝鐭鏄庢垨琛ヤ竵缁?Thorsten Leemhuis <linux@leemhuis.info>锛屽苟鏈€濂芥妱閫佸叕寮€鐨?Linux 鏂囨。閭欢鍒楄〃 <linux-doc@vger.kernel.org>锛岃寮€鍙戣€呯煡鏅撱€傝繖鏍风殑鍙嶉瀵硅繘涓€姝ユ敼杩涙湰鏂囪嚦鍏抽噸瑕侊紝涔熺鍚堟墍鏈変汉鐨勫埄鐩婏紝鍥犱负瀹冭兘璁╂洿澶氫汉鎺屾彙姝ゅ鎻忚堪鐨勪换鍔°€?


## 鍒嗘鎸囧崡鐨勫弬鑰冨皬鑺?


鏈妭鍖呭惈瀵逛笂杩板垎姝ユ寚鍗椾腑鍑犱箮姣忎竴椤瑰唴瀹圭殑琛ュ厖淇℃伅銆?

### 鏋勫缓鑷湁鍐呮牳鐨勫噯澶囧伐浣?


  **鏈妭涓殑姝ラ涓烘墍鏈夊悗缁祴璇曞瀹氬熀纭€銆?*
  [... <introprep_bissbs>]

鏈寚鍗楁墍鏈夊悗缁皬鑺備腑鐨勬楠ら兘渚濊禆浜庢澶勬弿杩扮殑鍐呭銆?

[back to step-by-step guide <introprep_bissbs>].


#### 涓虹揣鎬ユ儏鍐靛仛鍑嗗


  **鍒涘缓涓€浠藉叏鏂板浠斤紝骞跺皢绯荤粺淇涓庢仮澶嶅伐鍏锋斁鍦ㄦ墜杈广€?*
  [... <backup_bissbs>]

璇疯浣忥紝浣犻潰瀵圭殑鏄绠楁満锛屽畠浠湁鏃朵細鍙戠敓鎰忔兂涓嶅埌鐨勭姸鍐碘€斺€斿挨鍏舵槸褰撲綘鎽嗗紕鍍忔搷浣滅郴缁熷唴鏍歌繖鏍峰叧閿殑閮ㄥ垎鏃躲€傝€岃繖姝ｆ槸浣犲湪姝よ繃绋嬩腑瑕佸仛鐨勪簨銆傚洜姝わ紝鍗充究涓嶅お鍙兘鍙戠敓锛屼篃鏈€濂戒负鍑虹幇宸敊鍋氬ソ鍑嗗銆?

[back to step-by-step guide <backup_bissbs>]


#### 绉婚櫎浠讳綍涓庡閮ㄧ淮鎶ょ殑鍐呮牳妯″潡鐩稿叧鐨勪笢瑗?


  *绉婚櫎鎵€鏈変緷璧栧閮ㄥ紑鍙戠殑鍐呮牳椹卞姩銆佹垨浼氳嚜鍔ㄦ瀯寤鸿繖绫婚┍鍔ㄧ殑绋嬪簭銆? [...<vanilla_bissbs>]

澶栭儴寮€鍙戠殑鍐呮牳妯″潡寰堝鏄撳湪浜屽垎瀹氫綅杩囩▼涓紩鍙戦夯鐑︺€?

浣嗘湰鎸囧崡鍖呭惈杩欎竴姝ヨ繕鏈変竴涓洿閲嶈鐨勫師鍥狅細澶у鏁板唴鏍稿紑鍙戣€呬笉浼氱悊浼氶偅浜涗娇鐢ㄤ簡姝ょ被妯″潡鐨勫唴鏍告墍鍑虹幇鐨勫洖褰掓姤鍛娿€傝繖鏄洜涓鸿繖绫诲唴鏍镐笉鍐嶈瑙嗕负鈥渧anilla锛堝師鐗堬級鈥濓紝姝ｅ Documentation/admin-guide/reporting-issues.rst 涓洿璇︾粏鍦拌В閲婄殑閭ｆ牱銆?

[back to step-by-step guide <vanilla_bissbs>]


#### 搴斿 Secure Boot 涔嬬被鐨勬満鍒?


  *鍦ㄥ惎鐢ㄤ簡鈥淪ecure Boot鈥濇垨绫讳技鏈哄埗鐨勫钩鍙帮紙濡?commodity x86锛変笂锛岃纭繚绯荤粺绋嶅悗鍏佽浣犺嚜琛岀紪璇戠殑鍐呮牳鍚姩锛屽苟鍋氬ソ涓€鍒囧噯澶囥€? [... <secureboot_bissbs>]

璁稿鐜颁唬绯荤粺鍙厑璁告煇浜涚壒瀹氱殑鎿嶄綔绯荤粺鍚姩锛涜繖姝ｆ槸瀹冧滑榛樿鎷掔粷鍚姩鑷缂栬瘧鐨勫唴鏍哥殑鍘熷洜銆?

鐞嗘兂鎯呭喌涓嬶紝浣犲簲閫氳繃璇佷功璁╀綘鐨勫钩鍙颁俊浠讳綘鑷鏋勫缓鐨勫唴鏍革紝浠庤€岃В鍐宠繖涓棶棰樸€傚叿浣撳仛娉曟澶勪笉灞曞紑锛屽洜涓洪偅闇€瑕佸涓楠わ紝浼氫娇鏈枃鍋忕涓婚锛?Documentation/admin-guide/module-signing.rst' 浠ュ強鍚勭缃戦〉 already 鏇磋缁嗗湴璇存槑浜嗘墍闇€鐨勪竴鍒囥€?

涓存椂绂佺敤 Secure Boot 涔嬬被鐨勬満鍒讹紝鏄浣犺嚜宸辩殑 Linux 鍚姩鐨勫彟涓€绉嶅姙娉曘€傚湪鏅€?x86 绯荤粺涓婏紝鍙互鍦?BIOS 璁剧疆涓畬鎴愶紱鎵€闇€姝ラ鍥犳満鍣ㄨ€屽紓锛屽洜姝ゆ澶勬棤娉曡杩般€?

鍦ㄤ富娴?x86 Linux 鍙戣鐗堜笂锛岃繕鏈夌涓夌閫氱敤鐨勫姙娉曪細涓轰綘鐨?Linux 鐜绂佺敤鎵€鏈?Secure Boot 闄愬埗銆備綘鍙互閫氳繃杩愯 `mokutil --disable-validation` 鏉ュ惎鍔ㄨ繖涓€杩囩▼锛涘畠浼氭彁绀轰綘鍒涘缓涓€涓竴娆℃€у瘑鐮侊紝璁颁笅鏉ユ槸瀹夊叏鐨勩€傜幇鍦ㄩ噸鍚紱鍦?BIOS 瀹屾垚鎵€鏈夎嚜妫€鍚庯紝寮曞鍔犺浇绋嬪簭 Shim 浼氭樉绀轰竴涓摑鑹叉柟妗嗭紝鎻愮ず鈥淧ress any key to perform MOK management鈥濄€傚湪鍊掕鏃剁粨鏉熷墠鎸変笅浠绘剰閿紝鍗冲彲鎵撳紑涓€涓彍鍗曘€傞€夋嫨鈥淐hange Secure Boot state鈥濄€係him 鐨勨€淢okManager鈥濅細瑕佹眰浣犺緭鍏ヤ箣鍓嶈瀹氱殑涓€娆℃€у瘑鐮佷腑鐨勪笁涓殢鏈哄瓧绗︺€傝緭鍏ュ悗锛岀‘璁や綘纭疄鎯宠绂佺敤璇ラ獙璇併€備箣鍚庯紝鍏佽 MokManager 閲嶅惎鏈哄櫒銆?

[back to step-by-step guide <secureboot_bissbs>]


#### 鍚姩鏈€鍚庝竴涓伐浣滄甯哥殑鍐呮牳


  *鍚姩杩涘叆鏈€鍚庝竴涓伐浣滄甯哥殑鍐呮牳锛屽苟绠€瑕侀噸鏂扮‘璁ら偅涓彂鐢熷洖褰掔殑鍔熻兘鏄惁鐪熺殑姝ｅ父宸ヤ綔銆? [...<bootworking_bissbs>]

杩欒兘璁╁悗缁秹鍙婂垱寤哄拰绮剧畝閰嶇疆鐨勬楠ゅ仛鍑烘纭殑浜嬫儏銆?

[back to step-by-step guide <bootworking_bissbs>]


#### 绌洪棿闇€姹?


  **纭繚鏈夎冻澶熺殑绌洪棽绌洪棿鐢ㄤ簬鏋勫缓 Linux銆?*
  [... <diskspace_bissbs>]

涓婅堪鏁板瓧鍙槸绮楃暐浼拌锛屽苟棰勭暀浜嗚緝澶х殑浣欓噺浠ョ‘淇濆畨鍏紝鍥犳浣犲疄闄呴渶瑕佺殑寰€寰€鏇村皯銆?

If you have space constraints, be sure to hay attention to the :ref:`鍏充簬璋冭瘯绗﹀彿鐨勬楠?<debugsymbols_bissbs>` and its :ref:` accompanying 鍙傝€冨皬鑺?<debugsymbols_bisref>`, as disabling then will reduce the consumed disk space by quite a few gigabytes.

[back to step-by-step guide <diskspace_bissbs>]


#### 浜屽垎鑼冨洿


  *纭畾璐┛鏈寚鍗椼€佽瑙嗕负鈥滆壇濂斤紙good锛夆€濅笌鈥滄崯鍧忥紙bad锛夆€濈殑鍐呮牳鐗堟湰銆? [...<rangecheck_bissbs>]

纭畾寰呮鏌ョ殑鎻愪氦鑼冨洿閫氬父寰堢洿鎺ワ紝闄ら潪鍥炲綊鍙戠敓鍦ㄤ粠涓€涓?stable 绯诲垪鐨勫彂琛岀増鍒囨崲鍒拌緝鏂扮郴鍒楃殑鍙戣鐗堟椂锛堝浠?6.0.13 鍒?6.1.5锛夈€傝繖绉嶆儏鍐典笅 Git 闇€瑕佷竴浜涘紩瀵硷紝鍥犱负娌℃湁涓€鏉＄洿绯荤殑缁ф壙绾裤€?

杩欐槸鍥犱负闅忕潃 6.0 鐨勫彂甯冿紝mainline 鎺ㄨ繘鍒颁簡 6.1锛岃€?stable 绯诲垪 6.0.y 鍒欏垎鍙夊埌浜嗕竴鏃併€傚洜姝や粠鐞嗚涓婅锛屼綘鍦?6.1.5 涓婇亣鍒扮殑闂鍙兘鍙湪 6.0.13 涓甯革紝鍥犱负瀹冩槸鐢辫繘鍏ユ煇涓?6.0.y 鍙戣鐗堢殑鎻愪氦淇鐨勶紝浣嗕粠鏈繘鍏?mainline 鎴?6.1.y 绯诲垪銆傛墍骞革紝鐢变簬 stable/longterm 缁存姢鑰呯淮鎶や唬鐮佺殑鏂瑰紡锛岃繖绉嶆儏鍐甸€氬父涓嶄細鍙戠敓銆傚洜姝わ紝灏?6.0 鍋囪涓衡€滆壇濂斤紙good锛夆€濆唴鏍告槸鐩稿綋瀹夊叏鐨勩€備笉杩囪繖涓亣璁炬棤璁哄浣曢兘浼氳妫€楠岋紝鍥犱负璇ュ唴鏍稿皢鍦ㄦ湰鎸囧崡鐨勭 2 娈典腑琚瀯寤哄拰娴嬭瘯锛涘鏋滀綘灏濊瘯鍦?6.0.13 涓?6.1.15 涔嬮棿杩涜浜屽垎锛孏it 涔熶細寮哄埗浣犺繖鏍峰仛銆?

[back to step-by-step guide <rangecheck_bissbs>]


#### 瀹夎鏋勫缓鎵€闇€鐨勪緷璧?


  **瀹夎鏋勫缓 Linux 鍐呮牳鎵€闇€鐨勫叏閮ㄨ蒋浠躲€?*
  [...<buildrequires_bissbs>]

鍐呮牳鐩稿綋鐙珛锛屼絾闄や簡缂栬瘧鍣ㄤ箣绫荤殑宸ュ叿澶栵紝鏈夋椂浣犺繕闇€瑕佸嚑涓簱鎵嶈兘鏋勫缓瀹冦€傚浣曞畨瑁呮墍闇€鐨勪竴鍒囷紝鍙栧喅浜庝綘鐨?Linux 鍙戣鐗堜互鍙婁綘鍗冲皢鏋勫缓鐨勫唴鏍哥殑閰嶇疆銆?

浠ヤ笅鏄竴浜涗富娴佸彂琛岀増涓婁綘閫氬父闇€瑕佺殑绀轰緥锛?

```

    sudo pacman --needed -S bc binutils bison flex gcc git kmod libelf openssl \
      pahole perl zlib ncurses qt6-base

```

```

    sudo apt install bc binutils bison dwarves flex gcc git kmod libelf-dev \
      libssl-dev make openssl pahole perl-base pkg-config zlib1g-dev \
      libncurses-dev qt6-base-dev g++

```

```

    sudo dnf install binutils \
      /usr/bin/{bc,bison,flex,gcc,git,openssl,make,perl,pahole,rpmbuild} \
      /usr/include/{libelf.h,openssl/pkcs7.h,zlib.h,ncurses.h,qt6/QtGui/QAction}

```

```

    sudo zypper install bc binutils bison dwarves flex gcc git \
      kernel-install-tools libelf-devel make modutils openssl openssl-devel \
      perl-base zlib-devel rpm-build ncurses-devel qt6-base-devel

```

杩欎簺鍛戒护浼氬畨瑁呬竴浜涢€氬父锛堜絾骞堕潪鎬绘槸锛夐渶瑕佺殑杞欢鍖呫€備緥濡傦紝浣犲彲鑳芥兂璺宠繃瀹夎 ncurses 鐨勫紑鍙戝ご鏂囦欢锛屽洜涓哄彧鏈夊湪浠ュ悗鎯崇敤 'menuconfig' 鎴?'nconfig' 杩欎簺 make 鐩爣鏉ヨ皟鏁村唴鏍告瀯寤洪厤缃椂鎵嶉渶瑕佸畠浠紱鍚屾牱锛屽鏋滀綘涓嶆墦绠楃敤 'xconfig' 璋冩暣 .config锛屼篃鍙互鐪佺暐 Qt6 鐨勫ご鏂囦欢銆?

姝ゅ锛屽浜庢湰鎸囧崡鏈兜鐩栫殑浠诲姟鈥斺€斾緥濡備粠鍐呮牳鐨?tools/ 鐩綍鏋勫缓宸ュ叿鏃垛€斺€斾綘鍙兘杩橀渶瑕侀澶栫殑搴撳強鍏跺紑鍙戝ご鏂囦欢銆?

[back to step-by-step guide <buildrequires_bissbs>]


#### 浣跨敤 Git 涓嬭浇婧愪唬鐮?


  **鑾峰彇 Linux mainline 婧愪唬鐮併€?*
  [...<sources_bissbs>]

鍒嗘鎸囧崡浠嬬粛浜嗗浣曢€氳繃 Linus 鐨?mainline 浠撳簱鐨勫畬鏁?Git 鍏嬮殕鏉ヤ笅杞?Linux 婧愪唬鐮併€傚叧浜庤繖涓€鐐规病浠€涔堟洿澶氬彲璇寸殑鈥斺€斾絾杩樻湁涓ょ鏇夸唬鐨勮幏鍙栨柟寮忥紝鍙兘瀵逛綘鏇村悎閫傦細

- 濡傛灉浣犵殑缃戠粶杩炴帴涓嶇ǔ瀹氾紝鍙互鑰冭檻浣跨敤 'Git bundle'<sources_bundle_bisref>銆?

- 濡傛灉涓嬭浇瀹屾暣鐨勪粨搴撹€楁椂澶箙鎴栭渶瑕佽繃澶氬瓨鍌ㄧ┖闂达紝鍙互鑰冭檻 :ref:`浣跨敤 'shallow clone'锛堟祬鍏嬮殕锛?sources_shallow_bisref>`銆?


###### 浣跨敤 bundle 涓嬭浇 Linux mainline 婧愪唬鐮?


浣跨敤浠ヤ笅鍛戒护閫氳繃
```

    wget -c \
      https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/clone.bundle
    git clone --no-checkout clone.bundle ~/linux/
    cd ~/linux/
    git remote remove origin
    git remote add mainline \
      https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git
    git fetch mainline
    git remote add -t master stable \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git

```
濡傛灉 'wget' 鍛戒护澶辫触锛屽彧闇€閲嶆柊鎵ц瀹冿紝瀹冧細浠庢柇鐐瑰缁х画銆?

[back to step-by-step guide <sources_bissbs>]
[back to section intro <sources_bisref>]


#### 浣跨敤娴呭厠闅嗕笅杞?Linux mainline 婧愪唬鐮?


```

    git clone -o mainline --no-checkout --depth 1 -b master \
      https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git ~/linux/
    cd ~/linux/
    git remote add -t master stable \
      https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git

```
鐜板湪灏嗗厠闅嗙殑鍘嗗彶鍔犳繁鍒颁綘鈥滆壇濂斤紙good锛夆€濈増鏈搴?mainline 鍙戣鐗堢殑涓婁笂涓増鏈€傚鏋滃悗鑰呮槸 6.0 鎴?6.0.13锛岄偅涔?5.19 鏄笂涓€涓増鏈€?.18 鏄笂涓婁釜鐗堟湰鈥斺€斿洜姝ゅ皢鍘嗗彶鍔犳繁鍒?
```

    git fetch --shallow-exclude=v5.18 mainline

```
涔嬪悗锛屾寜鐓у垎姝ユ寚鍗椾腑鐨勮鏄庯紝灏?stable Git 浠撳簱娣诲姞涓鸿繙绋嬶紝骞舵坊鍔犳墍鏈夐渶瑕佺殑 stable 鍒嗘敮銆?

娉ㄦ剰锛屾祬鍏嬮殕鏈夊嚑涓壒娈婁箣澶勶細

- 瀵逛簬浜屽垎瀹氫綅锛屽巻鍙查渶瑕佹瘮鐪嬭捣鏉ュ繀瑕佺殑绋嬪害鍐嶅姞娣卞嚑涓?mainline 鐗堟湰锛屽鍓嶆墍杩般€傝繖鏄洜涓哄惁鍒?Git 灏嗘棤娉曞洖閫€鎴栨弿杩版煇涓寖鍥村唴鐨勫鏁版彁浜わ紙濡?6.1..6.2锛夛紝鍥犱负瀹冧滑鍦ㄥ唴閮ㄥ熀浜庢洿鏃╃殑鍐呮牳鍙戣鐗堬紙濡?6.0-rc2 鎴?5.19-rc3锛夈€?

- 鏈枃妗ｅ湪澶у鏁板湴鏂逛娇鐢ㄥ甫鏈?`--shallow-exclude=` 鐨?`git fetch` 鏉ユ寚瀹氫綘鍏冲績鐨勬渶鏃╃増鏈紙鍑嗙‘鍦拌锛氭槸瀹冪殑 git 鏍囩锛夈€備綘涔熷彲浠ユ敼鐢?`--shallow-since=` 鍙傛暟锛屾寚瀹氫竴涓粷瀵规棩鏈燂紙濡?`'2023-07-15'`锛夋垨鐩稿鏃ユ湡锛堝 `'12 months'`锛夋潵瀹氫箟鎯宠涓嬭浇鐨勫巻鍙叉繁搴︺€傚湪瀵?mainline 杩涜浜屽垎鏃讹紝璇风‘淇濆皢鍘嗗彶鑷冲皯鍔犳繁鍒颁綘鈥滆壇濂斤紙good锛夆€濆唴鏍告墍鍩轰簬鐨?mainline 鍙戣鐗堝彂甯冨墠 7 涓湀銆?

- 璀﹀憡锛氬湪鍔犳繁鍏嬮殕鏃讹紝浣犲彲鑳戒細閬囧埌绫讳技鈥渇atal: error in object: unshallow cafecaca0c0dacafecaca0c0dacafecaca0c0da鈥濈殑閿欒銆傝繖绉嶆儏鍐典笅璇疯繍琛?`git repack -d` 骞堕噸璇曘€?

[back to step-by-step guide <sources_bissbs>]
[back to section intro <sources_bisref>]


#### 寮€濮嬪畾涔夊唴鏍哥殑鏋勫缓閰嶇疆


  **寮€濮嬪噯澶囧唴鏍告瀯寤洪厤缃紙鍗?'.config' 鏂囦欢锛夈€?*
  [... <oldconfig_bissbs>]

*娉ㄦ剰锛岃繖鏄湰鎸囧崡涓垱寤烘垨淇敼鏋勫缓浜х墿鐨勫涓楠や腑鐨勭涓€姝ャ€傛湰鎸囧崡涓殑鍛戒护涓轰簡绠€鍗曡捣瑙侊紝灏嗚繖浜涗骇鐗╃洿鎺ュ瓨鏀惧湪婧愪唬鐮佹爲涓€傚鏋滀綘鏇存効鎰忔妸鏋勫缓浜х墿鍗曠嫭瀛樻斁锛屽彲浠ュ垱寤轰竴涓被浼尖€渵/linux-builddir/鈥濈殑鐩綍锛屽苟鍦ㄦ湰鎸囧崡鎵€鏈?make 璋冪敤涓姞鍏ュ弬鏁?`O=~/linux-builddir/`銆備綘杩橀渶瑕佽鍏朵粬鍛戒护涔熸寚鍚戣鐩綍鈥斺€斿叾涓寘鎷?``./scripts/config [...]`` 鍛戒护锛屽畠浠渶瑕?`--file ~/linux-builddir/.config`` 鎵嶈兘鎵惧埌姝ｇ‘鐨勬瀯寤洪厤缃€?

鎸夌収涓婅堪寤鸿鍒涘缓 .config 鏂囦欢鏃讹紝鏈変袱浠朵簨寰堝鏄撳嚭閿欙細

- 濡傛灉鏋勫缓鐩綍涓凡瀛樺湪 .config 鏂囦欢锛堝鈥渵/linux/.config鈥濓級锛宱ldconfig 鐩爣浼氫娇鐢ㄥ畠銆傚鏋滀綘姝ｆ槸杩欎釜鎰忓浘锛堣涓嬩竴姝ワ級锛岄偅瀹屽叏娌￠棶棰橈紱浣嗗湪鎵€鏈夊叾浠栨儏鍐典笅锛屼綘閮藉簲鍒犻櫎瀹冦€備緥濡傦紝濡傛灉浣犲湪閬靛惊鏈寚鍗楁椂璧板緱鏇磋繙锛屽悗鏉ュ洜閬囧埌闂鍥炲埌杩欓噷浠庡ご閲嶆柊閰嶇疆锛岃繖涓€鐐瑰氨寰堥噸瑕併€?

- 鏈夋椂 olddefconfig 鏃犳硶瀹氫綅浣犳鍦ㄨ繍琛屽唴鏍哥殑 .config 鏂囦欢锛屼粠鑰屼細浣跨敤榛樿鍊硷紝姝ｅ鎸囧崡涓畝瑕佹彁鍒扮殑閭ｆ牱銆傝繖绉嶆儏鍐典笅锛岃妫€鏌ヤ綘鐨勫彂琛岀増鏄惁鍦ㄦ煇澶勬彁渚涗簡璇ラ厤缃枃浠讹紱濡傛灉鏈夛紝灏辨墜鍔ㄦ妸瀹冩斁鍒版纭殑浣嶇疆锛堝鈥渵/linux/.config鈥濓級銆傚湪鏌愪簺鍙戣鐗堜笂
```

    zcat /proc/config.gz > .config

  Once you put it there, run ``make olddefconfig`` again to adjust it to the needs of the kernel about to be built.

```

娉ㄦ剰锛宱lddefconfig 鐩爣浼氭妸浠讳綍鏈畾涔夌殑鏋勫缓閫夐」璁剧疆涓洪粯璁ゅ€笺€傚鏋滀綘鏇存効鎰忔墜鍔ㄨ缃繖浜涢厤缃€夐」锛岃鏀圭敤 `make oldconfig`銆傝繖鏍凤紝瀵逛簬姣忎釜鏈畾涔夌殑閰嶇疆閫夐」锛岄兘浼氳闂綘濡備綍缁х画锛涘鏋滀笉纭畾濡備綍鍥炵瓟锛岀洿鎺ユ寜鈥渆nter鈥濆簲鐢ㄩ粯璁ゅ€煎嵆鍙€備笉杩囪娉ㄦ剰锛屽浜庝簩鍒嗗畾浣嶏紝浣犻€氬父搴斾娇鐢ㄩ粯璁ゅ€硷紝鍚﹀垯鍙兘浼氬惎鐢ㄦ煇涓柊鍔熻兘锛屽紩鍙戠湅璧锋潵鍍忓洖褰掔殑闂锛堜緥濡傜敱浜庡畨鍏ㄩ檺鍒讹級銆?

鏈夋椂锛屾妸涓€涓负鏌愪釜鍐呮牳锛堝 6.1锛夊噯澶囩殑閰嶇疆鏂囦欢鐢ㄤ簬鏇存棫鐨?mainline 鍙戣鐗堟椂锛屼細鍙戠敓濂囨€殑浜嬫儏鈥斺€斿挨鍏舵槸褰撳悗鑰呮棫寰楀鏃讹紙濡?5.15锛夈€傝繖涔熸槸鎸囧崡涓笂涓€姝ヨ浣犲惎鍔ㄤ竴鍒囨甯哥殑鍐呮牳鐨勫師鍥犱箣涓€銆傚洜姝わ紝濡傛灉浣犳墜鍔ㄦ坊鍔?.config 鏂囦欢锛屽姟蹇呯‘淇濆畠鏉ヨ嚜宸ヤ綔姝ｅ父鐨勫唴鏍革紝鑰屼笉鏄潵鑷嚭鐜板洖褰掔殑閭ｄ釜銆?

濡傛灉浣犳兂涓哄彟涓€鍙版満鍣ㄦ瀯寤哄唴鏍革紝璇锋壘鍒板畠鐨勫唴鏍告瀯寤洪厤缃紱閫氬父 `ls /boot/config-$(uname -r)` 浼氭墦鍗板嚭瀹冪殑鍚嶇О銆傚皢璇ユ枃浠跺鍒跺埌鏋勫缓鏈哄櫒涓婏紝骞朵繚瀛樹负 ~/linux/.config锛涗箣鍚庤繍琛?`make olddefconfig` 杩涜璋冩暣銆?

[back to step-by-step guide <oldconfig_bissbs>]


#### 绮剧畝鍐呮牳鐨勬瀯寤洪厤缃?


  **绂佺敤閭ｄ簺瀵逛綘鐨勯厤缃€岃█鏄庢樉澶氫綑鐨勪换鎰忓唴鏍告ā鍧椼€?*
  [... <localmodconfig_bissbs>]

姝ｅ鍒嗘鎸囧崡涓凡缁忕畝瑕佽鏄庣殑锛氫娇鐢?localmodconfig 鏃讹紝寰堝鏄撳嚭鐜颁綘鑷鏋勫缓鐨勫唴鏍哥己灏戞煇浜涙ā鍧楃殑鎯呭喌锛岃繖浜涙ā鍧楀搴旂殑浠诲姟浣犲湪浣跨敤璇?make 鐩爣鍓嶈嚦灏戞墽琛岃繃涓€娆°€傝繖鏄洜涓烘煇浜涗换鍔′緷璧栫殑鍐呮牳妯″潡鍙湁鍦ㄤ綘棣栨鎵ц璇ヤ换鍔℃椂鎵嶄細鑷姩鍔犺浇銆傛墍浠ワ紝濡傛灉浣犺嚜鍚姩鍐呮牳浠ユ潵浠庢湭鎵ц杩囪浠诲姟锛岃繖浜涙ā鍧楀氨涓嶄細琚姞杞解€斺€斿湪 localmodconfig 鐪嬫潵瀹冧滑鏄惧緱澶氫綑锛屼簬鏄究琚鐢紝浠庤€屽噺灏戦渶瑕佺紪璇戠殑浠ｇ爜閲忋€?

浣犲彲浠ラ€氳繃鎵ц閭ｄ簺閫氬父浼氳嚜鍔ㄥ姞杞介澶栧唴鏍告ā鍧楃殑鍏稿瀷浠诲姟鏉ュ敖閲忛伩鍏嶈繖涓€闂锛氬惎鍔ㄤ竴涓?VM銆佸缓绔?VPN 杩炴帴銆佸洖鐜寕杞?CD/DVD 鐨?ISO銆佹寕杞界綉缁滃叡浜紙CIFS銆丯FS 绛夛級锛屽苟杩炴帴鎵€鏈夊閮ㄨ澶囷紙2FA 瀵嗛挜銆佸ご鎴磋澶囥€佺綉缁滄憚鍍忓ご绛夛級浠ュ強浣犲钩鏃朵笉浣跨敤鐨勬枃浠剁郴缁熺殑瀛樺偍璁惧锛坆trfs銆乪xt4銆丗AT銆丯TFS銆乆FS 绛夛級銆備絾寰堥毦鎯冲埌鎵€鏈夊彲鑳介渶瑕佺殑鍏ㄩ儴涓滆タ鈥斺€斿嵆渚垮唴鏍稿紑鍙戣€呭湪杩欎竴姝ヤ篃甯稿父蹇樻帀杩欐垨閭ｃ€?

涓嶈琚繖绉嶉闄╁洶鎵帮紝灏ゅ叾鏄粎涓烘祴璇曠洰鐨勮€岀紪璇戝唴鏍告椂锛氭墍鏈夐€氬父鍏抽敭鐨勪笢瑗块兘浼氬湪閭ｉ噷銆傝€屼笖濡傛灉浣犲繕浜嗘煇浜涢噸瑕佸唴瀹癸紝浠ュ悗鍙互鎵嬪姩寮€鍚己澶辩殑鍔熻兘锛屽苟蹇€熼噸鏂拌繍琛屽懡浠わ紝缂栬瘧骞跺畨瑁呬竴涓叿澶囦綘鎵€闇€涓€鍒囩殑鍐呮牳銆?

浣嗗鏋滀綘鎵撶畻瀹氭湡鏋勫缓骞朵娇鐢ㄨ嚜琛岀紪璇戠殑鍐呮牳锛屽彲浠ラ€氳繃璁板綍浣犵殑绯荤粺鍦ㄥ嚑鍛ㄥ唴鍔犺浇浜嗗摢浜涙ā鍧楁潵闄嶄綆椋庨櫓銆備綘鍙互鐢?`modprobed-db <https://github.com/graysky2/modprobed-db>`_ 灏嗚繖涓€杩囩▼鑷姩鍖栥€備箣鍚庝娇鐢?`LSMOD=<path>` 鏉?
```

  yes '' | make LSMOD='${HOME}'/.config/modprobed.db localmodconfig

```
濡傛灉浣犲鍒朵簡涓€浠藉悎閫傜殑 .config 浣滀负鍩虹锛堣涓婁竴姝ワ級锛岃鍙傛暟涔熷厑璁镐綘涓哄彟涓€鍙版満鍣ㄦ瀯寤虹簿绠€鍐呮牳銆傚彧闇€鍦ㄩ偅鍙扮郴缁熶笂杩愯 `lsmod > lsmod_foo-machine`锛屽苟灏嗙敓鎴愮殑鏂囦欢澶嶅埗鍒颁綘鐨勬瀯寤轰富鏈虹殑涓荤洰褰曘€傜劧鍚庤繍琛屼互涓嬪懡浠わ紝鑰屼笉鏄?
```

  yes '' | make LSMOD=~/lsmod_foo-machine localmodconfig

```

[back to step-by-step guide <localmodconfig_bissbs>]


#### 涓哄嵆灏嗘瀯寤虹殑鍐呮牳鎵撲笂鏍囩


  *纭繚浣犲皢鏋勫缓鐨勬墍鏈夊唴鏍搁兘鑳介€氳繃涓€绉嶇壒娈婃爣绛惧拰鍞竴鐨勭増鏈爣璇嗙琚竻鏅拌瘑鍒€? [... <tagging_bissbs>]

杩欒兘璁╀綘灏嗚嚜宸卞彂琛岀増鐨勫唴鏍镐笌鏈繃绋嬩腑鍒涘缓鐨勫唴鏍稿尯鍒嗗紑鏉ワ紝鍥犱负鍚庤€呯殑鏂囦欢鎴栫洰褰曞悕绉颁腑浼氬寘鍚€?local鈥濓紱瀹冭繕鏈夊姪浜庡湪鍚姩鑿滃崟涓寫閫夋纭殑鏉＄洰锛屽苟閬垮厤娣锋穯浣犳瀯寤虹殑鍐呮牳鈥斺€斿洜涓哄湪浜屽垎瀹氫綅鏈熼棿锛屽畠浠殑鐗堟湰鍙风湅璧锋潵浼氭湁浜涙贩涔便€?

[back to step-by-step guide <tagging_bissbs>]


#### 鍐冲畾鏄惁鍚敤璋冭瘯绗﹀彿


  **鍐冲畾濡備綍澶勭悊璋冭瘯绗﹀彿銆?* [... <debugsymbols_bissbs>]

褰撲綘鐨勫唴鏍稿湪鍚庣画杩愯涓姏鍑衡€減anic鈥濄€佲€淥ops鈥濄€佲€渨arning鈥濇垨鈥淏UG鈥濇椂锛屾嫢鏈夎皟璇曠鍙峰彲鑳藉緢閲嶈锛屽洜涓洪偅鏍蜂綘灏辫兘鎵惧埌闂涓唬鐮佺‘鍒囧彂鐢熺殑浣嶇疆銆備絾鏀堕泦鍜屽祵鍏ユ墍闇€鐨勮皟璇曚俊鎭渶瑕佹椂闂达紝骞朵笖浼氭秷鑰楃浉褰撳鐨勭┖闂达細鍦?2022 骞村簳锛岀敤 localmodconfig 绮剧畝鐨勫吀鍨?x86 鍐呮牳锛屽惎鐢ㄨ皟璇曠鍙锋椂鏋勫缓浜х墿绾︿负 5 GB锛岃€岀鐢ㄦ椂涓嶅埌 1 GB銆傜敓鎴愮殑鍐呮牳闀滃儚涓庢ā鍧椾篃浼氭洿澶э紝浠庤€屽鍔?/boot/ 鐨勫瓨鍌ㄩ渶姹傚拰鍔犺浇鏃堕棿銆?

鍥犳锛屽鏋滀綘鎯宠涓€涓緝灏忕殑鍐呮牳锛屽苟涓斾互鍚庝笉澶彲鑳藉幓瑙ｇ爜鏍堝洖婧紝灏卞彲鑳介渶瑕佺鐢ㄨ皟璇曠鍙蜂互閬垮厤杩欎簺寮婄銆傚鏋滃悗鏉ュ彂鐜扮‘瀹為渶瑕佸畠浠紝鍙渶鎸変笂杩版柟寮忓惎鐢ㄥ苟閲嶆柊鏋勫缓鍐呮牳鍗冲彲銆?

鍙︿竴鏂归潰锛屽鏋滀綘涔嬪悗寰堝彲鑳介渶瑕佽В鐮佹爤鍥炴函锛岄偅涔堝湪杩欎釜杩囩▼涓氨涓€瀹氳鍚敤瀹冧滑銆侱ocumentation/admin-guide/reporting-issues.rst 涓殑鈥淒ecode failure messages锛堣В鐮佸け璐ヤ俊鎭級鈥濅竴鑺傚姝よ繃绋嬫湁鏇磋缁嗙殑璇存槑銆?

[back to step-by-step guide <debugsymbols_bissbs>]


#### 璋冩暣鏋勫缓閰嶇疆


  *妫€鏌ヤ綘鏄惁鎯宠鎴栭渶瑕佽皟鏁村叾浠栦竴浜涘唴鏍搁厤缃€夐」锛?

鏍规嵁浣犵殑闇€瑕侊紝姝ゆ椂浣犲彲鑳芥兂瑕佹垨蹇呴』璋冩暣涓€浜涘唴鏍搁厤缃€夐」銆?


###### 鍙戣鐗堢壒瀹氱殑璋冩暣


  **Are you running** [... <configmods_bissbs>]

浠ヤ笅灏忚妭鏈夊姪浜庝綘閬垮厤鍦ㄦ湰鎸囧崡涓彁鍒扮殑鍑犱釜鏅€氬彂琛岀増涓婃瀯寤烘椂鍑虹幇鐨勫凡鐭ラ棶棰樸€?

**Debian:**

- 鍒犻櫎瀵瑰凡澶辨晥璇佷功鏂囦欢鐨勫紩鐢紝鍚﹀垯瀹冧細瀵艰嚧浣犵殑鏋勫缓
```

   ./scripts/config --set-str SYSTEM_TRUSTED_KEYS ''

  Alternatively, download the needed certificate and make that configuration option point to it, as `Debian 鎵嬪唽涓湁鏇磋缁嗙殑璇存槑 <https://debian-handbook.info/browse/stable/sect.kernel-compilation.html>`_ -- or generate your own, as explained in Documentation/admin-guide/module-signing.rst.

```

[back to step-by-step guide <configmods_bissbs>]


###### 涓汉鍖栬皟鏁?


  *濡傛灉浣犳兂褰卞搷閰嶇疆鐨勫叾浠栨柟闈紝鐜板湪灏卞幓鍋氥€? [... <configmods_bissbs>]

姝ゆ椂浣犲彲浠ヤ娇鐢?`make menuconfig` 鎴?`make nconfig` 杩欐牱鐨勫懡浠わ紝閫氳繃鍩轰簬鏂囨湰鐨勭敤鎴风晫闈㈡潵鍚敤鎴栫鐢ㄦ煇浜涘姛鑳斤紱鑻ヨ浣跨敤鍥惧舰鍖栭厤缃伐鍏凤紝鍒欒繍琛?`make xconfig`銆備袱鑰呴兘闇€瑕佸叾鎵€渚濊禆宸ュ叿鍖咃紙鍒嗗埆鏄?ncurses 浠ュ強 Qt5 鎴?Qt6锛夌殑寮€鍙戝簱锛涘鏋滅己灏戞墍闇€鍐呭锛屼細鍑虹幇閿欒娑堟伅鎻愮ず浣犮€?

[back to step-by-step guide <configmods_bissbs>]


#### 灏?.config 鏂囦欢濡ュ杽鏀跺ソ


  **鍦ㄦ渶鏂版洿鏀逛箣鍚庨噸鏂板鐞?.config锛屽苟灏嗗叾淇濆瓨鍦ㄥ畨鍏ㄧ殑鍦版柟銆?*
  [... <saveconfig_bissbs>]

鎶婁綘鍑嗗濂界殑 .config 鏀惧湪涓€鏃侊紝鍥犱负鍦ㄦ湰鎸囧崡鍚庣画姣忔寮€濮嬫瀯寤哄彟涓€涓唴鏍镐箣鍓嶏紝浣犻兘鎯宠鎶婂畠澶嶅埗鍥炴瀯寤虹洰褰曘€傝繖鏄洜涓哄湪涓嶅悓鐗堟湰涔嬮棿鏉ュ洖鍒囨崲鍙兘浼氫互濂囨€殑鏂瑰紡鏀瑰姩 .config 鏂囦欢锛涜繖浜涙敼鍔ㄥ伓灏斾細寮曞彂鍓綔鐢紝鍙兘鎵颁贡娴嬭瘯锛屾垨鍦ㄦ湁浜涙儏鍐典笅浣夸綘浜屽垎瀹氫綅鐨勭粨鏋滃彉寰楁鏃犳剰涔夈€?

[back to step-by-step guide <saveconfig_bissbs>]


### 灏濊瘯鐢ㄦ渶鏂颁唬鐮佸簱澶嶇幇闂


  *纭璇ュ洖褰掍笉鏄敱鏌愪簺 .config 鍙樻洿寮曡捣鐨勶紝骞舵鏌ュ畠鍦ㄦ渶鏂颁唬鐮佸簱涓槸鍚︿緷鐒跺瓨鍦ㄣ€? [... <introlatestcheck_bissbs>]

瀵规煇浜涜鑰呮潵璇达紝姝ゆ椂妫€鏌ユ渶鏂颁唬鐮佸簱鍙兘鏄惧緱娌℃湁蹇呰锛屽挨鍏舵槸濡傛灉浣犲凡缁忕敤鍙戣鐗堟彁渚涚殑鍐呮牳鍋氳繃锛屾垨鑰呴亣鍒扮殑鏄?stable/longterm 绯诲垪鍐呴儴鐨勫洖褰掋€備絾鍦ㄤ互涓嬭繖浜涚悊鐢变笅锛屾垜浠己鐑堝缓璁繖鏍峰仛锛?

- 浣犱細鍦ㄧ湡姝ｅ紑濮嬩簩鍒嗗畾浣嶄箣鍓嶏紝灏遍亣鍒扮敱浣犵殑鐜寮曡捣鐨勪换浣曢棶棰樸€傝繖灏嗚浣犲緢瀹规槗鍖哄垎鈥滆繖寰堝彲鑳芥槸鎴戠幆澧冮噷鐨勬煇涓棶棰樷€濅笌鈥滆繖娆″彉鏇撮渶瑕佸湪浜屽垎杩囩▼涓烦杩囷紝鍥犱负璇ラ樁娈电殑婧愪唬鐮佸惈鏈変竴涓笉鐩稿叧鐨勯棶棰橈紝瀵艰嚧鏋勫缓鎴栧惎鍔ㄥけ璐モ€濄€?

- 杩欎簺姝ラ鑳芥帓闄や綘鐨勯棶棰樻槸鍚︾敱鈥滃彲鐢紙working锛夆€濆唴鏍镐笌鈥滄崯鍧忥紙broken锛夆€濆唴鏍镐箣闂存瀯寤洪厤缃殑鏌愪簺鍙樻洿寮曡捣銆備緥濡傦紝褰撲綘鐨勫彂琛岀増鍦ㄦ柊鍐呮牳涓惎鐢ㄤ簡鏌愪釜棰濆鐨勫畨鍏ㄧ壒鎬э紝鑰屾棫鍐呮牳涓鐗规€ц绂佺敤鎴栧皻涓嶆敮鎸佹椂锛屽氨鍙兘鍑虹幇杩欑鎯呭喌銆傝瀹夊叏鐗规€у彲鑳戒細濡ㄧ浣犲仛鐨勬煇浜涗簨鎯呪€斺€旇繖绉嶆儏鍐典笅锛屼粠 Linux 鍐呮牳涓婃父寮€鍙戣€呯殑瑙掑害鐪嬶紝浣犵殑闂骞朵笉鏋勬垚鍥炲綊锛屾濡?Documentation/admin-guide/reporting-regressions.rst 涓洿璇︾粏瑙ｉ噴鐨勯偅鏍枫€傚洜姝わ紝濡傛灉浣犲幓浜屽垎瀹冿紝灏辨槸鍦ㄦ氮璐规椂闂淬€?

- 濡傛灉浣犲洖褰掔殑鎴愬洜鍦ㄦ渶鏂?mainline 浠ｇ爜搴撲腑宸茬粡琚慨澶嶏紝閭ｄ箞浣犵殑浜屽垎瀹氫綅灏辩櫧鍋氫簡銆傝繖涓€鐐瑰浜庝綘鍦?stable/longterm 鍙戣鐗堜腑閬囧埌鐨勫洖褰掑悓鏍锋垚绔嬶紝鍥犱负瀹冧滑寰€寰€鏄敱琚洖绉绘锛坆ackport锛夌殑 mainline 鍙樻洿涓殑闂寮曡捣鐨勨€斺€旇繖绉嶆儏鍐典笅锛岄棶棰樺繀椤诲厛鍦?mainline 涓慨澶嶃€備篃璁稿畠宸茬粡鍦ㄩ偅閲岃淇锛屽苟涓斾慨澶嶆鍦ㄨ鍥炵Щ妞嶇殑杩囩▼涓€?

- 姝ゅ锛屽浜?stable/longterm 绯诲垪鍐呴儴鐨勫洖褰掞紝鑷冲叧閲嶈鐨勬槸寮勬竻璇ラ棶棰樻槸鍚︾壒瀹氫簬璇ョ郴鍒楋紝杩樻槸鍦?mainline 鍐呮牳涓篃浼氬嚭鐜帮紝鍥犱负鎶ュ憡闇€瑕佸彂閫佺粰涓嶅悓鐨勪汉锛?

  - 鐗瑰畾浜庢煇涓?stable/longterm 绯诲垪鐨勫洖褰掔敱 stable 鍥㈤槦璐熻矗锛沵ainline 鐨?Linux 寮€鍙戣€呭彲鑳戒細鍦ㄦ剰锛屼篃鍙兘涓嶄細銆?

  - 鍦?mainline 涓篃鍑虹幇鐨勫洖褰掞紝鍒欐槸鐢卞父瑙勭殑 Linux 寮€鍙戣€呬笌缁存姢鑰呰礋璐ｅ鐞嗭紱stable 鍥㈤槦涓嶅叧蹇冿紝涔熶笉闇€瑕佸弬涓庢姤鍛婏紝鍙渶瑕佸湪淇灏辩华鏃惰鍛婄煡鍘诲洖绉绘瀹冦€?

  濡傛灉浣犳妸鎶ュ憡鍙戦敊浜嗗璞★紝瀹冨彲鑳戒細琚拷鐣モ€斺€斿嵆渚垮緱鍒板洖澶嶏紝寮€鍙戣€呬篃寰堝彲鑳戒細璁╀綘鍏堝垽鏂睘浜庝笂杩板摢绉嶆儏鍐碉紝鍐嶈繘琛屾繁鍏ユ煡鐪嬨€?

[back to step-by-step guide <introlatestcheck_bissbs>]


#### 妫€鍑烘渶鏂扮殑 Linux 浠ｇ爜搴?


  **妫€鍑烘渶鏂扮殑 Linux 浠ｇ爜搴撱€?*
  [... <checkoutmaster_bissbs>]

濡傛灉浣犱互鍚庢兂鍐嶆妫€鏌ユ槸鍚︽湁涓€涓洿鏂扮殑浠ｇ爜搴撹兘淇璇ラ棶棰橈紝璇疯寰楀啀娆¤繍琛屽墠闈㈡彁鍒扮殑閭ｆ潯 `git fetch --shallow-exclude [...]` 鍛戒护锛屼互鏇存柊浣犵殑鏈湴 Git 浠撳簱銆?

[back to step-by-step guide <checkoutmaster_bissbs>]


#### 鏋勫缓浣犵殑鍐呮牳


  *浣跨敤浣犲噯澶囧ソ鐨勯厤缃枃浠讹紝鏋勫缓绗竴涓唴鏍哥殑闀滃儚涓庢ā鍧椼€? [... <build_bissbs>]

鍦ㄨ繖涓樁娈靛緢澶氫簨鎯呴兘鍙兘鍑洪敊锛屼絾涓嬮潰鐨勮鏄庤兘甯綘鑷姪瑙ｅ喅銆傚彟涓€涓皬鑺備粙缁嶄簡濡備綍鐩存帴灏嗗唴鏍告墦鍖呮垚 deb銆乺pm 鎴?tar 鏂囦欢銆?

###### 澶勭悊鏋勫缓閿欒


褰撴瀯寤洪敊璇彂鐢熸椂锛屽畠鍙兘鏄敱浣犳満鍣ㄧ幆澧冪殑鏌愪簺鏂归潰寮曡捣鐨勶紝杩欑鎯呭喌閫氬父鑳藉揩閫熶慨澶嶏紱浣嗘湁鏃堕棶棰樺嚭鍦ㄤ唬鐮佷腑锛屽彧鑳界敱寮€鍙戣€呬慨澶嶃€備粩缁嗘煡鐪嬪け璐ヤ俊鎭紝鍐嶇粨鍚堝湪缃戜笂鍋氫竴浜涜皟鐮旓紝閫氬父鑳藉憡璇変綘灞炰簬鍝竴绉嶆儏鍐点€傝杩涜杩欐牱鐨勮皟鏌ワ紝璇烽噸鏂板惎鍔ㄦ瀯寤?
```

  make V=1

```
`V=1` 浼氬惎鐢ㄨ缁嗚緭鍑猴紝杩欏彲鑳芥槸鏌ョ湅鐪熷疄閿欒鎵€蹇呴渶鐨勩€備负浜嗚閿欒鏇村鏄撹鍙戠幇锛岃繖鏉″懡浠よ繕鐪佺暐浜嗘棭鍓嶇敤浜庤绯荤粺鎵€鏈?CPU 鏍稿績閮藉弬涓庤浠诲姟鐨?``-j $(nproc --all)``鈥斺€斾絾杩欑骞惰鎬у湪鍑洪敊鏃朵篃浼氬甫鏉ヤ竴浜涙贩涔便€?

鍑犵閽熷悗锛屾瀯寤鸿繃绋嬪簲璇ヤ細鍐嶆閬囧埌璇ラ敊璇€傜幇鍦ㄨ瘯鐫€鎵惧嚭鎻忚堪璇ラ棶棰樻渶鍏抽敭鐨勯偅涓€琛屻€傜劧鍚庡湪缃戜笂鎼滅储璇ヨ涓渶閲嶈銆佹渶涓嶉€氱敤鐨勪竴娈碉紙姣斿 4 鍒?8 涓崟璇嶏級锛涢伩鍏嶆垨鍘绘帀浠讳綍鐪嬭捣鏉ヤ笌鐗瑰畾绯荤粺鐩稿叧鐨勫唴瀹癸紝姣斿浣犵殑鐢ㄦ埛鍚嶆垨鍍?`/home/username/linux/` 杩欐牱鐨勬湰鍦拌矾寰勫悕銆傚厛鐢ㄤ綘甯哥敤鐨勬悳绱㈠紩鎿庢悳杩欎釜瀛楃涓诧紝鐒跺悗鍐嶉€氳繃 `lore.kernel.org/all/ <https://lore.kernel.org/all/>`_ 鎼滅储 Linux 鍐呮牳閭欢鍒楄〃銆?

澶у鏁版椂鍊欙紝杩欐牱鑳芥壘鍒拌В閲婇棶棰樻墍鍦ㄧ殑鍐呭锛涜€屼笖寰€寰€鍏朵腑涓€鏉＄粨鏋滆繕浼氫负浣犵殑鐨勯棶棰樻彁渚涜В鍐虫柟妗堛€傚鏋滄壘涓嶅埌涓庝綘鐨勯棶棰樺尮閰嶇殑鍐呭锛屽氨鎹釜瑙掑害鍐嶈瘯锛屾瘮濡備慨鏀规悳绱㈣瘝锛屾垨鏀圭敤閿欒淇℃伅涓殑鍙︿竴琛屻€?

褰掓牴缁撳簳锛屼綘閬囧埌鐨勫ぇ澶氭暟闂寰堝彲鑳藉凡缁忚鍒汉閬囧埌骞舵姤鍛婅繃浜嗐€傝繖鍏朵腑鍖呮嫭閭ｄ簺鎴愬洜涓嶅湪浣犵殑绯荤粺銆佽€屽湪浠ｇ爜涓殑闂銆傚鏋滀綘閬囧埌鐨勬槸鍚庝竴绫伙紝閭ｄ箞涔熷緢鍙兘鑳戒负浣犵殑涓洪鎵惧埌瑙ｅ喅鏂规锛堝琛ヤ竵锛夋垨鍙橀€氬姙娉曘€?

###### 灏嗗唴鏍告墦鍖?


鍒嗘鎸囧崡浣跨敤榛樿鐨?make 鐩爣锛堝湪 x86 涓婁负 'bzImage' 鍜?'modules'锛夋潵鏋勫缓鍐呮牳鐨勯暅鍍忎笌妯″潡锛岄殢鍚庣敱鎸囧崡涓殑鍚庣画姝ラ瀹夎銆備綘涔熷彲浠ユ敼鐢ㄤ互涓嬬洰鏍囦箣涓€锛岀洿鎺ユ瀯寤烘墍鏈夊唴瀹瑰苟鐩存帴鎵撳寘锛?

- `make -j $(nproc --all) bindeb-pkg` 浠ョ敓鎴?deb 鍖?

- `make -j $(nproc --all) binrpm-pkg` 浠ョ敓鎴?rpm 鍖?

- `make -j $(nproc --all) tarbz2-pkg` 浠ョ敓鎴?bz2 鍘嬬缉鐨?tar 鍖?

杩欓噷鍙槸涓烘鐩殑鎻愪緵鐨勯儴鍒?make 鐩爣锛屽叾浠栫洰鏍囪瑙?`make help`銆備綘涔熷彲浠ュ湪杩愯 `make -j $(nproc --all)` 涔嬪悗鍐嶄娇鐢ㄨ繖浜涚洰鏍囷紝鍥犱负瀹冧滑浼氭嬀鍙栧凡缁忔瀯寤哄ソ鐨勬墍鏈夊唴瀹广€?

濡傛灉浣犱娇鐢ㄨ繖浜涚洰鏍囨潵鐢熸垚 deb 鎴?rpm 鍖咃紝璇峰拷鐣ュ垎姝ユ寚鍗椾腑鍏充簬瀹夎鍜屽嵏杞藉唴鏍哥殑璇存槑锛涙敼涓轰娇鐢ㄥ搴旀牸寮忕殑鍖呯鐞嗗伐鍏凤紙濡?dpkg 鍜?rpm锛夛紝鎴栨瀯寤哄湪瀹冧滑涔嬩笂鐨勫寘绠＄悊宸ュ叿锛坅pt銆乤ptitude銆乨nf/yum銆亃ypper 绛夛級鏉ュ畨瑁呭拰鍗歌浇杩欎簺鍖呫€傝娉ㄦ剰锛岀敤杩欎袱涓?make 鐩爣鐢熸垚鐨勫寘鏃ㄥ湪閫傜敤浜庝娇鐢ㄨ繖浜涙牸寮忕殑鍚勭鍙戣鐗堬紝鍥犳鏈夋椂瀹冧滑鐨勮涓轰細涓庝綘鍙戣鐗堢殑鍐呮牳鍖呮湁鎵€涓嶅悓銆?

[back to step-by-step guide <build_bissbs>]


#### 灏嗗唴鏍稿畨瑁呭埌浣?


  **Install the kernel you just built.** [... <install_bissbs>]

鍦ㄥ垎姝ユ寚鍗椾腑鎵ц鍛戒护涔嬪悗浣犻渶瑕佸仛浠€涔堬紝鍙栧喅浜庝綘鐨勫彂琛岀増涓婃槸鍚﹀瓨鍦?`/sbin/installkernel` 鍙墽琛屾枃浠讹紝浠ュ強瀹冪殑瀹炵幇鏂瑰紡銆?

濡傛灉鎵惧埌浜?installkernel锛屽唴鏍哥殑鏋勫缓绯荤粺浼氭妸鍐呮牳闀滃儚鐨勫疄闄呭畨瑁呭伐浣滃鎵樼粰杩欎釜鍙墽琛屾枃浠讹紝瀹冧細鎵ц浠ヤ笅閮ㄥ垎鎴栧叏閮ㄤ换鍔★細

- 鍦ㄥ嚑涔庢墍鏈?Linux 鍙戣鐗堜笂锛宨nstallkernel 閮戒細鎶婁綘鐨勫唴鏍搁暅鍍忓瓨鍏?/boot/锛岄€氬父鍚嶄负鈥?boot/vmlinuz-<kernelrelease_id>鈥濓紱閫氬父瀹冭繕浼氬湪鏃佽竟鏀句竴涓€淪ystem.map-<kernelrelease_id>鈥濄€?

- 鍦ㄥぇ澶氭暟鍙戣鐗堜笂锛宨nstallkernel 闅忓悗浼氱敓鎴愪竴涓€渋nitramfs鈥濓紙鏈夋椂涔熷彨鈥渋nitrd鈥濓級锛岄€氬父瀛樺偍涓衡€?boot/initramfs-<kernelrelease_id>.img鈥濇垨鈥?boot/initrd-<kernelrelease_id>鈥濄€傛櫘閫氬彂琛岀増渚濊禆杩欎釜鏂囦欢鏉ュ惎鍔紝鍥犳鍔″繀鍏堟墽琛?make 鐩爣鈥渕odules_install鈥濓紝鍚﹀垯浣犲彂琛岀増鐨?initramfs 鐢熸垚鍣ㄥ皢鏃犳硶鎵惧埌鎵撳寘杩涢暅鍍忔墍闇€鐨勬ā鍧椼€?

- 鍦ㄦ煇浜涘彂琛岀増涓婏紝installkernel 杩樹細涓轰綘鐨勫唴鏍稿湪寮曞鍔犺浇绋嬪簭鐨勯厤缃腑娣诲姞涓€涓潯鐩€?

濡傛灉浣犵殑鍙戣鐗堢己灏?installkernel 鑴氭湰锛屾垨鍙鐞嗕簡鍏朵腑涓€閮ㄥ垎浠诲姟锛屼綘灏卞繀椤昏嚜宸卞畬鎴愰儴鍒嗘垨鍏ㄩ儴浠诲姟銆傝鎯呰鏌ラ槄鍙戣鐗堢殑鏂囨。銆傚鏋滄嬁涓嶅噯锛屽彲浠ュ畨瑁?
```

   sudo install -m 0600 $(make -s image_name) /boot/vmlinuz-$(make -s kernelrelease)
   sudo install -m 0600 System.map /boot/System.map-$(make -s kernelrelease)

```
鐜板湪浣跨敤浣犵殑鍙戣鐗堜负姝ゆ彁渚涚殑宸ュ叿鐢熸垚 initramfs銆備箣鍚庡皢浣犵殑鍐呮牳娣诲姞鍒板紩瀵煎姞杞界▼搴忛厤缃腑锛屽苟閲嶅惎銆?

[back to step-by-step guide <install_bissbs>]


#### 姣忎釜鍐呮牳鐨勫瓨鍌ㄩ渶姹?


  *妫€鏌ュ唴鏍搞€佸叾妯″潡浠ュ強 initramfs 绛夊叾浠栫浉鍏虫枃浠舵秷鑰椾簡澶氬皯瀛樺偍绌洪棿銆? [... <storagespace_bissbs>]

浜屽垎瀹氫綅杩囩▼涓瀯寤虹殑鍐呮牳浼氬湪 /boot/ 鍜?/lib/modules/ 涓嬪崰鐢ㄧ浉褰撳鐨勭┖闂达紝灏ゅ叾鏄綋浣犲惎鐢ㄤ簡璋冭瘯绗﹀彿鏃躲€傝繖浣垮緱鍦ㄤ簩鍒嗚繃绋嬩腑寰堝鏄撴妸鍗峰～婊♀€斺€斾互鑷充簬杩炴棭鍏堣繕鑳芥甯稿伐浣滅殑鍐呮牳閮藉彲鑳芥棤娉曞惎鍔ㄣ€備负閬垮厤杩欑鎯呭喌锛屼綘闇€瑕佺煡閬撴瘡涓凡瀹夎鍐呮牳閫氬父闇€瑕佸澶х┖闂淬€?

娉ㄦ剰锛屽鏁版儏鍐典笅鏈寚鍗椾腑浣跨敤鐨勬ā寮忊€?boot/**$(make -s kernelrelease)**鈥濅細鍖归厤鍚姩鍐呮牳鎵€闇€鐨勬墍鏈夋枃浠垛€斺€斾絾璺緞鍜屽懡鍚嶆柟妗堥兘涓嶆槸寮哄埗鎬х殑銆傚洜姝ゅ湪鏌愪簺鍙戣鐗堜笂锛屼綘闇€瑕佸埌涓嶅悓鐨勪綅缃幓鏌ユ壘銆?

[back to step-by-step guide <storagespace_bissbs>]


#### 妫€鏌ヤ綘鏂版瀯寤虹殑鍐呮牳鏄惁璁や负鑷繁鈥渢ainted锛堣姹℃煋锛夆€?


  **妫€鏌ュ唴鏍告槸鍚﹀皢鑷繁鏍囪涓衡€渢ainted锛堣姹℃煋锛夆€濄€?*
  [... <tainted_bissbs>]

褰撳彂鐢熸煇浜涘彲鑳藉鑷村悗缁湅浼煎畬鍏ㄦ棤鍏崇殑閿欒鐨勪簨鎯呮椂锛孡inux 浼氬皢鑷繁鏍囪涓?tainted锛堣姹℃煋锛夈€傝繖灏辨槸涓轰粈涔堝紑鍙戣€呭彲鑳戒細蹇界暐鎴栬崏鐜囧洖搴旀潵鑷姹℃煋鍐呮牳鐨勬姤鍛娾€斺€斿綋鐒讹紝闄ら潪鍐呮牳姝ｆ槸鍦ㄦ墍鎶ュ憡缂洪櫡鍙戠敓鐨勯偅涓€鍒昏缃簡璇ユ爣蹇椼€?

鍥犳锛屼綘搴斿弬鐓?Documentation/admin-guide/tainted-kernels.rst 涓殑璇存槑锛屽幓鏌ユ槑鍐呮牳涓轰綍琚薄鏌擄紱杩欐牱鍋氫篃绗﹀悎浣犺嚜宸辩殑鍒╃泭锛屽惁鍒欎綘鐨勬祴璇曞彲鑳芥湁闂銆?

[back to step-by-step guide <tainted_bissbs>]


#### 妫€鏌ュ熀浜庢渶鏂?mainline 浠ｇ爜搴撴瀯寤虹殑鍐呮牳


  **楠岃瘉浣犳瀯寤虹殑鏂板唴鏍告槸鍚﹀嚭鐜颁簡璇ョ己闄枫€?*
  [... <recheckbroken_bissbs>]

浣犵殑缂洪櫡鎴栧洖褰掓病鏈夊湪浣犵敤鏈€鏂颁唬鐮佸簱鏋勫缓鐨勫唴鏍镐笂鍑虹幇锛屽彲鑳芥湁鍑犱釜鍘熷洜銆備互涓嬫槸鏈€甯歌鐨勶細

- 璇ョ己闄峰綋鏃跺凡缁忚淇銆?

- 浣犳€€鐤戞槸鍥炲綊鐨勯棶棰橈紝鍏跺疄鏄敱浣犵殑鍐呮牳鎻愪緵鍟嗘墍鍋氱殑鏋勫缓閰嶇疆鍙樻洿寮曡捣鐨勩€?

- 浣犵殑闂鍙兘鏄竴涓珵鎬佹潯浠讹紝鍦ㄤ綘鐨勫唴鏍镐笂涓嶄細鏄剧幇锛涚簿绠€鍚庣殑鏋勫缓閰嶇疆銆佷笉鍚岀殑璋冭瘯绗﹀彿璁剧疆銆佹墍浣跨敤鐨勭紪璇戝櫒锛屼互鍙婂叾浠栧悇绉嶅洜绱犻兘鍙兘瀵艰嚧杩欑鎯呭喌銆?

- 濡傛灉浣犳槸鐢?stable/longterm 鍐呮牳閬囧埌鐨勮鍥炲綊锛岄偅涔堝畠鍙兘鏄壒瀹氫簬璇ョ郴鍒楃殑闂锛涙湰鎸囧崡鐨勪笅涓€姝ヤ細瀵规杩涜妫€鏌ャ€?

[back to step-by-step guide <recheckbroken_bissbs>]


#### 妫€鏌ュ熀浜庢渶鏂?stable/longterm 浠ｇ爜搴撴瀯寤虹殑鍐呮牳


  *浣犳槸鍚︽闈复鏌愪釜 stable/longterm 鍙戣鐗堝唴閮ㄧ殑鍥炲綊锛屽嵈鏈兘鐢ㄤ綘鍒氱敤鏈€鏂?mainline 婧愪唬鐮佹瀯寤虹殑鍐呮牳澶嶇幇瀹冿紵閭ｄ箞璇锋鏌ヨ鐗瑰畾绯诲垪鐨勬渶鏂颁唬鐮佸簱鏄惁宸茬粡淇浜嗚繖涓棶棰樸€? [... <recheckstablebroken_bissbs>]

濡傛灉杩欎釜鍐呮牳涔熸病鏈夊嚭鐜拌鍥炲綊锛岄偅涔堝ぇ姒傜巼灏变笉闇€瑕佽繘琛屼簩鍒嗗畾浣嶄簡銆?

[back to step-by-step guide <recheckstablebroken_bissbs>]


### 纭繚鈥滆壇濂斤紙good锛夆€濈増鏈‘瀹炲伐浣滆壇濂?


  **妫€鏌ヤ綘鏋勫缓鐨勫唴鏍告槸鍚﹀伐浣滄甯搞€?*
  [... <introworkingcheck_bissbs>]

鏈妭灏嗛噸鏂扮‘绔嬩竴涓凡鐭ュ彲宸ヤ綔鐨勫熀纭€銆傝烦杩囧畠涔熻寰堣浜猴紝浣嗛€氬父鏄釜鍧忎富鎰忥紝鍥犱负瀹冨仛浜嗕竴浠堕噸瑕佺殑浜嬶細

瀹冭兘纭繚浣犳棭鍓嶅噯澶囩殑 .config 鏂囦欢纭疄鎸夐鏈熷伐浣溿€傝繖涔熺鍚堜綘鑷繁鐨勫埄鐩婏紝鍥犱负绮剧畝閰嶇疆骞堕潪涓囨棤涓€澶扁€斺€斿湪鎬€鐤戞瀯寤洪厤缃彲鑳藉嚭浜嗛棶棰樹箣鍓嶏紝浣犲彲鑳戒細鐧界櫧鏋勫缓鍜屾祴璇曞崄涓垨鏇村鍐呮牳銆?

浠呰繖涓€鐐瑰氨瓒充互鎴愪负鍦ㄦ鑺辫垂鏃堕棿鐨勭悊鐢憋紝浣嗚繖骞堕潪鍞竴鐨勭悊鐢便€?

鏈寚鍗楃殑璁稿璇昏€呴€氬父杩愯鐨勬槸鎵撲簡琛ヤ竵鐨勫唴鏍革紝鎴栦娇鐢ㄤ簡闄勫姞妯″潡锛屾垨涓よ€呭吋鏈夈€傚洜姝よ繖浜涘唴鏍镐笉琚涓衡€渧anilla锛堝師鐗堬級鈥濃€斺€旇繖鏍蜂竴鏉ワ紝閭ｄ釜鍙戠敓鍥炲綊鐨勪笢瑗垮彲鑳戒粠涓€寮€濮嬪湪鈥滆壇濂斤紙good锛夆€濈増鏈殑 vanilla 鏋勫缓涓氨浠庢湭姝ｅ父宸ヤ綔杩囥€?

瀵逛簬閭ｄ簺娉ㄦ剰鍒颁笉鍚岀郴鍒楃殑 stable/longterm 鍐呮牳涔嬮棿鍑虹幇鍥炲綊锛堝 6.0.13..6.1.5锛夌殑浜猴紝杩樻湁绗笁涓悊鐢憋細瀹冭兘纭繚浣犲湪杩囩▼涓棭浜涙椂鍊欏亣璁句负鈥滆壇濂斤紙good锛夆€濈殑鍐呮牳鐗堟湰锛堝 6.0锛夌‘瀹炲湪姝ｅ父宸ヤ綔銆?

[back to step-by-step guide <introworkingcheck_bissbs>]


#### 鏋勫缓浣犺嚜宸辩殑鈥滆壇濂斤紙good锛夆€濆唴鏍哥増鏈?


  *鏋勫缓浣犺嚜宸辩殑鍙敤锛坵orking锛夊唴鏍稿彉浣擄紝骞舵鏌ラ偅涓彂鐢熷洖褰掔殑鍔熻兘鍦ㄥ畠涓婇潰鏄惁鎸夐鏈熷伐浣溿€? [... <recheckworking_bisref>]

濡傛灉闅忕潃鏂板唴鏍告崯鍧忕殑閭ｄ釜鍔熻兘锛屽湪浣犵涓€涓嚜琛屾瀯寤虹殑鍐呮牳涓婁篃涓嶅伐浣滐紝璇峰湪缁х画涔嬪墠鎵惧嚭骞惰В鍐冲師鍥犮€傚嚭鐜拌繖绉嶆儏鍐电殑鍘熷洜鏈夊緢澶氥€備互涓嬫槸涓€浜涙帓鏌ユ€濊矾锛?

- 妫€鏌?taint 鐘舵€佷互鍙?`dmesg` 鐨勮緭鍑猴紝涔熻鏄煇涓笉鐩稿叧鐨勯棶棰樺嚭閿欎簡銆?

- 涔熻 localmodconfig 鍋氫簡浜涘鎬殑浜嬶紝绂佺敤浜嗘祴璇曡鍔熻兘鎵€闇€鐨勬ā鍧楋紵閭ｄ綘鍙兘闇€瑕佸熀浜庢渶鍚庝竴涓伐浣滄甯哥殑鍐呮牳鐨?.config 閲嶆柊鍒涘缓涓€涓厤缃枃浠讹紝骞惰烦杩囩簿绠€锛涘湪 .config 涓墜鍔ㄧ鐢ㄦ煇浜涘姛鑳藉悓鏍峰彲鑳藉鏁堬紝骞惰兘鍑忓皯鏋勫缓鏃堕棿銆?

- 涔熻杩欐牴鏈笉鏄唴鏍稿洖褰掞紝鑰屾槸鐢辨煇浜涘伓鐒跺洜绱犮€佹崯鍧忕殑 initramfs锛堜篃鍙?initrd锛夈€佹柊鐨勫浐浠舵枃浠讹紝鎴栨洿鏂板悗鐨勭敤鎴锋€佽蒋浠跺紩璧风殑锛?

- 涔熻閭ｆ槸浣犲彂琛岀増鍐呮牳涓坊鍔犵殑鏌愪釜鍔熻兘锛岃€屽綋鏃剁殑 vanilla Linux 浠庢湭鏀寔杩囷紵

娉ㄦ剰锛屽鏋滀綘鍙戠幇骞朵慨澶嶄簡 .config 鏂囦欢鐨勯棶棰橈紝浣犱細鎯崇敤瀹冧粠鏈€鏂颁唬鐮佸簱鍐嶆瀯寤轰竴涓唴鏍革紝鍥犱负浣犳棭鍓嶅 mainline 浠ュ強鏌愪釜鍙楀奖鍝?stable/longterm 绯诲垪鐨勬渶鏂扮増鏈墍鍋氱殑娴嬭瘯锛屽緢鍙兘閮芥槸鏈夐棶棰樼殑銆?

[back to step-by-step guide <recheckworking_bisref>]


### 鎵ц浜屽垎瀹氫綅骞堕獙璇佺粨鏋?


  *鍦ㄥ畬鎴愪簡鎵€鏈夊噯澶囧伐浣滃拰棰勯槻鎬ф瀯寤轰箣鍚庯紝浣犵幇鍦ㄥ彲浠ュ紑濮嬩簩鍒嗗畾浣嶄簡銆? [... <introbisect_bissbs>]

鏈涓殑姝ラ鎵ц骞堕獙璇佷簩鍒嗗畾浣嶃€?

[back to step-by-step guide <introbisect_bissbs>].


#### 寮€濮嬩簩鍒嗗畾浣?


  *寮€濮嬩簩鍒嗗畾浣嶏紝骞跺憡鐭?Git 鏃╁墠纭畾鐨勨€滆壇濂斤紙good锛夆€濅笌鈥滄崯鍧忥紙bad锛夆€濈増鏈€? [... <bisectstart_bissbs>]

杩欏皢鍚姩浜屽垎瀹氫綅杩囩▼锛涙渶鍚庝竴鏉″懡浠や細璁?Git 妫€鍑轰綅浜庘€滆壇濂斤紙good锛夆€濅笌鈥滄崯鍧忥紙bad锛夆€濆彉鏇翠箣闂村ぇ绾︿腑鐐瑰鐨勬煇涓彁浜や緵浣犳祴璇曘€?

[back to step-by-step guide <bisectstart_bissbs>]


#### 浠庝簩鍒嗙偣鏋勫缓鍐呮牳


  *浣跨敤浣犳棭鍓嶇敤杩囩殑鐩稿悓鍛戒护锛屼粠 Git 妫€鍑虹殑浠ｇ爜鏋勫缓銆佸畨瑁呭苟鍚姩涓€涓唴鏍搞€? [... <bisectbuild_bissbs>]

杩欓噷鏈変袱浠朵簨鍊煎緱娉ㄦ剰锛?

- 鍋跺皵锛屾瀯寤哄唴鏍镐細澶辫触锛屾垨鑰呯敱浜庢煇浜?
```

    git bisect skip

  Git 闅忓悗浼氭鍑洪檮杩戝彟涓€涓彁浜わ紝杩愭皵濂界殑璇濆畠搴旇鑳芥洿濂藉湴宸ヤ綔銆備箣鍚庨噸鏂版墽琛岃繖涓€姝ャ€?

```
- 浜屽垎杩囩▼涓彲鑳戒細鍑虹幇閭ｄ簺鐪嬭捣鏉ユ湁鐐瑰鎬殑鐗堟湰鏍囪瘑绗︼紝杩欐槸鍥犱负 Linux 鍐呮牳鐨勫悇涓瓙绯荤粺浼氬湪鍏跺墠涓€涓増鏈紙濡?6.1锛夊畬鎴愪箣鍓嶏紝灏变负鏂扮殑 mainline 鍙戣鐗堬紙濡?6.2锛夊噯澶囧彉鏇淬€傚洜姝ゅ畠浠細鍩轰簬绋嶆棭涓€浜涚殑鐐癸紙濡?6.1-rc1 鐢氳嚦 6.0锛夎繘琛屽紑鍙戔€斺€旂劧鍚庡湪 6.1 鍙戝竷鍚庯紝鏈粡鍙樺熀鎴栧帇缂╁氨鍚堝苟杩?6.2銆傝繖灏卞鑷翠簡浜屽垎杩囩▼涓細鍑虹幇閭ｄ簺鐪嬭捣鏉ユ湁鐐瑰鎬殑鐗堟湰鏍囪瘑绗︺€?

[back to step-by-step guide <bisectbuild_bissbs>]


#### 浜屽垎妫€鏌ョ偣


  **妫€鏌ラ偅涓彂鐢熷洖褰掔殑鍔熻兘鍦ㄤ綘鍒氭瀯寤虹殑鍐呮牳涓槸鍚﹀伐浣滄甯搞€?*
  [... <bisecttest_bisref>]

纭繚浣犲憡璇?Git 鐨勫唴瀹瑰噯纭棤璇細鍙閿欎竴娆★紝灏变細璁╁悗缁殑浜屽垎瀹氫綅瀹屽叏鍋忕姝ｈ建锛屽洜姝ら偅涔嬪悗鐨勬墍鏈夋祴璇曢兘灏嗙櫧璐广€?

[back to step-by-step guide <bisecttest_bisref>]


#### 鏀跺ソ浜屽垎鏃ュ織


  **灏?Git 鐨勪簩鍒嗘棩蹇椾笌褰撳墠鐨?.config 鏂囦欢淇濆瓨鍦ㄥ畨鍏ㄧ殑鍦版柟銆?*
  [... <bisectlog_bisref>]

濡備笂鎵€杩帮細鍙鎶婃煇涓€涓唴鏍搁敊璇湴鏍囪涓衡€済ood鈥濇垨鈥渂ad鈥濓紝灏变細璁╀簩鍒嗗畾浣嶇殑鏈€缁堢粨鏋滃彉寰楁鏃犵敤澶勩€傝繖绉嶆儏鍐典笅锛屼綘閫氬父涓嶅緱涓嶄粠澶撮噸鏂板紑濮嬩簩鍒嗗畾浣嶃€傝€屾棩蹇楀彲浠ラ槻姝㈣繖绉嶆儏鍐碉紝鍥犱负瀹冨彲鑳借鍒汉鎸囧嚭浜屽垎澶ф鏄湪鍝噷璺戝亸浜嗏€斺€旇繖鏍蜂竴鏉ワ紝浣犱篃璁稿彧闇€鏋勫缓鍑犱釜鍐呮牳锛岃€屼笉鏄崄涓垨鏇村锛屽氨鑳借В鍐抽棶棰樸€?

鎶?.config 鏂囦欢鏀跺ソ锛屾槸鍥犱负鍦ㄤ綘鎶ュ憡鍥炲綊涔嬪悗锛屽紑鍙戣€呭緢鏈夊彲鑳戒細鍚戜綘瑕佸畠銆?

[back to step-by-step guide <bisectlog_bisref>]


#### 灏濊瘯鍥為€€缃瓉绁搁


  *灏濊瘯鍦ㄦ渶鏂颁唬鐮佸簱涔嬩笂鍥為€€缃瓉绁搁锛岀湅鏄惁鑳戒慨澶嶄綘鐨勫洖褰掋€? [... <revert_bissbs>]

杩欐槸涓€涓彲閫夋楠わ紝浣嗗彧瑕佹湁鍙兘浣犲氨搴斿綋灏濊瘯锛氬綋浣犳彁鍑轰簩鍒嗗畾浣嶇粨鏋滄椂锛屽紑鍙戣€呭緢鏈夊彲鑳戒細瑕佹眰浣犳墽琛岃繖涓€姝ャ€傛棦鐒朵綘宸茬粡杩涘叆鐘舵€侊紝姝ゆ椂鍐嶆瀯寤轰竴涓唴鏍稿簲璇ヤ笉鎴愰棶棰橈紝涓嶅Θ涓€璇曘€?

鍒嗘鎸囧崡宸茬粡娑电洊浜嗘墍鏈夌浉鍏冲唴瀹癸紝鍙湁涓€浠剁暐鏄惧皯瑙佺殑鎯呭喌闄ゅ锛氫綘鏄惁鐢ㄦ煇涓?stable/longterm 绯诲垪瀵逛竴涓悓鏍峰嚭鐜板湪 mainline 涓殑鍥炲綊鍋氫簡浜屽垎瀹氫綅锛屼絾 Git 鏃犳硶鍦?mainline 涓?revert 璇ユ彁浜わ紵閭ｄ箞灏濊瘯鍦ㄥ彈褰卞搷鐨?stable/longterm 绯诲垪涓?revert 璇?culprit鈥斺€斿鏋滄垚鍔燂紝灏辨敼涓烘祴璇曡鍐呮牳鐗堟湰銆?

[back to step-by-step guide <revert_bissbs>]

### 鍦ㄩ伒寰湰鎸囧崡鏈熼棿鍙婁箣鍚庣殑娓呯悊姝ラ


  *During and after following this guide you might want or need to remove some
  of the kernels you installed.* [... <introclosure_bissbs>]

鏈妭涓殑姝ラ鎻忚堪浜嗘竻鐞嗘祦绋嬨€?

[back to step-by-step guide <introclosure_bissbs>].

#### 鍦ㄤ簩鍒嗗畾浣嶈繃绋嬩腑鐨勬竻鐞?


  *To remove one of the kernels you installed, look up its 'kernelrelease'
  identifier.* [... <makeroom_bissbs>]

浣犲湪姝よ繃绋嬩腑瀹夎鐨勫唴鏍镐互鍚庡緢瀹规槗鍒犻櫎锛屽洜涓哄畠鐨勫悇涓儴鍒嗗彧瀛樺偍鍦ㄤ袱涓綅缃紝涓旀爣璇嗘竻鏅般€傚洜姝わ紝褰撲綘鎵嬪姩瀹夎鍐呮牳锛堜粠鑰岀粫杩囦簡鍙戣鐗堢殑鎵撳寘绯荤粺锛夋椂锛屾棤闇€鎷呭績浼氭妸鏈哄櫒鎼炰贡锛氫綘鐨勫唴鏍哥殑鍚勪釜閮ㄥ垎浠ュ悗閮界浉瀵瑰鏄撳垹闄ゃ€?

杩欎袱涓綅缃箣涓€鏄?/lib/modules/ 涓嬬殑涓€涓洰褰曪紝鍏朵腑淇濆瓨浜嗘瘡涓凡瀹夎鍐呮牳鐨勬ā鍧椼€傝鐩綍浠ュ唴鏍哥殑 release 鏍囪瘑绗﹀懡鍚嶏紱鍥犳锛岃鍒犻櫎浣犳瀯寤虹殑鏌愪釜鍐呮牳鐨勬墍鏈夋ā鍧楋紝鍙渶鍒犻櫎瀹冨湪 /lib/modules/ 涓殑妯″潡鐩綍鍗冲彲銆?

鍙︿竴涓綅缃槸 /boot/锛屽畨瑁呭唴鏍告椂閫氬父浼氬湪鍏朵腑鏀剧疆涓ゅ埌浜斾釜鏂囦欢銆傚畠浠殑鏂囦欢鍚嶄腑閫氬父閮藉寘鍚?release 鍚嶇О锛屼絾鍏蜂綋鏂囦欢鏁伴噺鍜岀‘鍒囧悕绉板湪涓€瀹氱▼搴︿笂鍙栧喅浜庝綘鍙戣鐗堢殑 installkernel 鍙墽琛屾枃浠跺強鍏?initramfs 鐢熸垚鍣ㄣ€傚湪鏌愪簺鍙戣鐗堜笂锛屽垎姝ユ寚鍗椾腑鎻愬埌鐨?`kernel-install remove...` 鍛戒护浼氭浛浣犲垹闄ゆ墍鏈夎繖浜涙枃浠讹紝鍚屾椂杩樹細浠庝綘鐨?bootloader 閰嶇疆涓Щ闄よ鍐呮牳鐨勮彍鍗曢」銆傚湪鍏朵粬鍙戣鐗堜笂锛岃繖涓ら」浠诲姟闇€瑕佷綘鑷繁瀹屾垚銆備互涓嬪懡浠ゅ簲褰撹兘浠ヤ氦浜掓柟寮忓垹闄ゆ煇涓叿鏈夎 release 鍚嶇О鐨勫唴鏍哥殑涓変釜涓昏鏂囦欢锛?

```
  rm -i /boot/{System.map,vmlinuz,initr}-6.0-rc1-local-gcafec0cacaca0

```
涔嬪悗锛屾鏌?/boot/ 涓槸鍚﹁繕鏈夊叾浠栨枃浠跺悕鍖呭惈 '6.0-rc1-local-gcafec0cacaca0' 鐨勬枃浠讹紝骞惰€冭檻涔熷皢鍏跺垹闄ゃ€傜幇鍦ㄤ粠浣犵殑 bootloader 閰嶇疆涓Щ闄よ鍐呮牳鐨勫惎鍔ㄩ」锛涘叿浣撴楠ゅ湪涓嶅悓鐨?Linux 鍙戣鐗堜箣闂村樊寮傚緢澶с€?

娉ㄦ剰锛屾墜鍔ㄥ垹闄ゅ唴鏍哥殑鏂囦欢鎴栫洰褰曟椂瑕佸皬蹇冨儚 '*' 杩欐牱鐨勯€氶厤绗︼細浣犲彲鑳芥湰鎯冲垹闄?6.0 鎴?6.0.1锛屽嵈涓嶅皬蹇冨垹闄や簡 6.0.13 鍐呮牳鐨勬枃浠躲€?

[back to step-by-step guide <makeroom_bissbs>]

#### 鍦ㄤ簩鍒嗗畾浣嶄箣鍚庣殑娓呯悊


  *Once you have finished the bisection, do not immediately remove anything
  you set up, as you might need a few things again.*
  [... <finishingtouch_bissbs>]

褰撲綘纭疄瀛樺偍绌洪棿绱у紶鏃讹紝鎸夊垎姝ユ寚鍗楁墍杩板垹闄ゅ唴鏍稿彲鑳介噴鏀句笉浜嗕綘鏈熸湜鐨勯偅涔堝绌洪棿銆傝繖绉嶆儏鍐典笅锛岀幇鍦ㄤ篃鍙互鑰冭檻涓€骞惰繍琛?`rm -rf ~/linux/*`銆傝繖浼氬垹闄ゆ瀯寤轰骇鐗╁拰 Linux 婧愮爜锛屼絾浼氫繚鐣?Git 浠撳簱锛垀/linux/.git/锛夆€斺€斿洜姝や竴鏉＄畝鍗曠殑 `git reset --hard` 灏辫兘鎶婃簮鐮佹仮澶嶅洖鏉ャ€?

姝ゆ椂杩炰粨搴撲竴骞跺垹闄ゅ彲鑳藉苟涓嶆槑鏅猴細寮€鍙戣€呭緢鏈夊彲鑳戒細瑕佹眰浣犲啀鏋勫缓涓€涓唴鏍告潵鎵ц棰濆鐨勬祴璇曗€斺€斾緥濡傛祴璇曚竴涓皟璇曡ˉ涓佹垨鎻愯鐨勪慨澶嶃€傚叧浜庡浣曟墽琛岃繖浜涙搷浣滅殑缁嗚妭锛屽彲浠ュ湪 :ref:`Optional tasks: test reverts, patches, or later versions <introoptional_bissbs>` 涓€鑺備腑鎵惧埌銆?

浣犱箣鎵€浠ユ兂鎶?~/kernel-config-working 鏂囦欢淇濈暀鍑犲懆锛屼篃鏄嚭浜庤繖浜涢澶栫殑娴嬭瘯銆?

[back to step-by-step guide <finishingtouch_bissbs>]

### 娴嬭瘯 revert銆佽ˉ涓佹垨鏇存柊鐨勭増鏈?


  *While or after reporting a bug, you might want or potentially will be asked
  to test reverts, patches, proposed fixes, or other versions.*
  [... <introoptional_bissbs>]

鏈妭涓娇鐢ㄧ殑鎵€鏈夊懡浠ら兘搴斿綋鐩稿綋鐩寸櫧锛屽洜姝ら櫎浜嗘湁涓€鐐逛箣澶栨病鏈夊お澶氬彲琛ュ厖鐨勶細鎸夌収璇存槑璁剧疆鍐呮牳 tag 鏃讹紝纭繚瀹冧笉瑕佹瘮绀轰緥涓敤鐨勯偅涓暱澶锛屽洜涓哄鏋?kernelrelease 鏍囪瘑绗﹁秴杩?63 涓瓧绗﹀氨浼氬嚭闂銆?

[back to step-by-step guide <introoptional_bissbs>].

## 闄勫姞淇℃伅

### 鍦ㄥ彟涓€鍙版満鍣ㄤ笂鏋勫缓鍐呮牳


瑕佸湪鍙︿竴鍙扮郴缁熶笂缂栬瘧鍐呮牳锛屽彧闇€瀵瑰垎姝ユ寚鍗楃殑璇存槑绋嶄綔鏀瑰姩锛?

- 鍦ㄤ綘涔嬪悗鎯宠瀹夎骞舵祴璇曞唴鏍哥殑閭ｅ彴鏈哄櫒涓婂紑濮嬮伒寰湰鎸囧崡銆?

- 鍦ㄦ墽琛?':ref:`Boot into the working kernel and briefly use the apparently broken feature <bootworking_bissbs>`' 涔嬪悗锛屼娇鐢?`lsmod > ~/test-machine-lsmod` 灏嗗凡鍔犺浇妯″潡鐨勫垪琛ㄤ繚瀛樺埌涓€涓枃浠躲€傜劧鍚庢壘鍒版鍦ㄨ繍琛岀殑鍐呮牳鐨勬瀯寤洪厤缃紙鍏充簬鍦ㄥ摢閲屽彲浠ユ壘鍒板畠锛岃鍙傞槄 ':ref:`Start defining the build configuration for your kernel <oldconfig_bisref>`'锛夛紝骞跺皢鍏朵繚瀛樹负 '~/test-machine-config-working'銆傚皢杩欎袱涓枃浠朵紶杈撳埌浣犵殑鏋勫缓涓绘満鐨勫鐩綍銆?

- 鍦ㄦ瀯寤轰富鏈轰笂缁х画閬靛惊鏈寚鍗楋紙渚嬪浠?':ref:`Ensure to have enough free space for building [...] <diskspace_bisref>`' 寮€濮嬶級銆?

- 褰撲綘鍒拌揪 ':ref:`Start preparing a kernel build configuration[...] <oldconfig_bissbs>`' 鏃讹細鍦ㄧ涓€娆¤繍琛?`make olddefconfig` 涔嬪墠锛屾墽琛屼互涓嬪懡浠わ紝灏嗕綘鐨勯厤缃熀浜庢潵鑷?

```
    cp ~/test-machine-config-working ~/linux/.config

```
- 鍦ㄦ帴涓嬫潵 ':ref:`disable any apparently superfluous kernel

```
    yes '' | make localmodconfig LSMOD=~/lsmod_foo-machine localmodconfig

```
- 缁х画閬靛惊鏈寚鍗楋紝浣嗗拷鐣ラ偅浜涜鏄庢瘡娆￠兘瑕佸浣曠紪璇戙€佸畨瑁呭苟閲嶅惎杩涘叆鏌愪釜鍐呮牳鐨勬寚绀恒€傛敼涓烘瀯寤?

```
    cp ~/kernel-config-working .config
    make olddefconfig &&
    make -j $(nproc --all) targz-pkg

  This will generate a gzipped tar file whose name is printed in the last
  line shown; for example, a kernel with the kernelrelease identifier
  '6.0.0-rc1-local-g928a87efa423' built for x86 machines usually will
  be stored as '~/linux/linux-6.0.0-rc1-local-g928a87efa423-x86.tar.gz'.

  Copy that file to your test machine's home directory.

```
- 鍒囨崲鍒版祴璇曟満鍣紝妫€鏌ユ槸鍚︽湁瓒冲绌洪棿瀹圭撼鍙︿竴涓?

```
    sudo tar -xvzf ~/linux-6.0.0-rc1-local-g928a87efa423-x86.tar.gz -C /

  Afterwards :ref:`generate the initramfs and add the kernel to your boot
  loader's configuration <install_bisref>`; on some distributions the following
  command will take care of both these tasks::

    sudo /sbin/installkernel 6.0.0-rc1-local-g928a87efa423 /boot/vmlinuz-6.0.0-rc1-local-g928a87efa423

  Now reboot and ensure you started the intended kernel.

```
杩欑鏂瑰紡鍦ㄤ负鍙︿竴绉嶆灦鏋勬瀯寤烘椂涔熷悓鏍锋湁鏁堬細鍙渶瀹夎浜ゅ弶缂栬瘧鍣紝骞跺湪姣忔璋冪敤 make 鏃跺姞涓婇€傚綋鐨勫弬鏁帮紙渚嬪 `make ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- [...]`锛夈€?

### 棰濆鐨勯槄璇绘潗鏂?


- The `man page for 'git bisect' <https://git-scm.com/docs/git-bisect>`_ and
  `fighting regressions with 'git bisect' <https://git-scm.com/docs/git-bisect-lk2009.html>`_
  in the Git documentation.
- `Working with git bisect <https://nathanchance.dev/posts/working-with-git-bisect/>`_
  from kernel developer Nathan Chancellor.
- `Using Git bisect to figure out when brokenness was introduced <http://webchick.net/node/99>`_.
- `Fully automated bisecting with 'git bisect run' <https://lwn.net/Articles/317154>`_.

..
   end-of-content
..
   This document is maintained by Thorsten Leemhuis <linux@leemhuis.info>. If
   you spot a typo or small mistake, feel free to let him know directly and
   he'll fix it. You are free to do the same in a mostly informal way if you
   want to contribute changes to the text -- but for copyright reasons please CC
   linux-doc@vger.kernel.org and 'sign-off' your contribution as
   Documentation/process/submitting-patches.rst explains in the section 'Sign
   your work - the Developer's Certificate of Origin'.
..
   This text is available under GPL-2.0+ or CC-BY-4.0, as stated at the top
   of the file. If you want to distribute this text under CC-BY-4.0 only,
   please use 'The Linux kernel development community' for author attribution
   and link this as source:
   https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/Documentation/admin-guide/verify-bugs-and-bisect-regressions.rst
..
   Note: Only the content of this RST file as found in the Linux kernel sources
   is available under CC-BY-4.0, as versions of this text that were processed
   (for example by the kernel's build system) might contain content taken from
   files which use a more restrictive license.