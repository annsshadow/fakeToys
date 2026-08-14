## 缂洪櫡杩借釜锛圔ug Hunting锛?

```

	------------[ cut here ]------------
	WARNING: CPU: 1 PID: 28102 at kernel/module.c:1108 module_put+0x57/0x70
	Modules linked in: dvb_usb_gp8psk(-) dvb_usb dvb_core nvidia_drm(PO) nvidia_modeset(PO) snd_hda_codec_hdmi snd_hda_intel snd_hda_codec snd_hwdep snd_hda_core snd_pcm snd_timer snd soundcore nvidia(PO) [last unloaded: rc_core]
	CPU: 1 PID: 28102 Comm: rmmod Tainted: P        WC O 4.8.4-build.1 #1
	Hardware name: MSI MS-7309/MS-7309, BIOS V1.12 02/23/2009
	 00000000 c12ba080 00000000 00000000 c103ed6a c1616014 00000001 00006dc6
	 c1615862 00000454 c109e8a7 c109e8a7 00000009 ffffffff 00000000 f13f6a10
	 f5f5a600 c103ee33 00000009 00000000 00000000 c109e8a7 f80ca4d0 c109f617
	Call Trace:
	 [<c12ba080>] ? dump_stack+0x44/0x64
	 [<c103ed6a>] ? __warn+0xfa/0x120
	 [<c109e8a7>] ? module_put+0x57/0x70
	 [<c109e8a7>] ? module_put+0x57/0x70
	 [<c103ee33>] ? warn_slowpath_null+0x23/0x30
	 [<c109e8a7>] ? module_put+0x57/0x70
	 [<f80ca4d0>] ? gp8psk_fe_set_frontend+0x460/0x460 [dvb_usb_gp8psk]
	 [<c109f617>] ? symbol_put_addr+0x27/0x50
	 [<f80bc9ca>] ? dvb_usb_adapter_frontend_exit+0x3a/0x70 [dvb_usb]
	 [<f80bb3bf>] ? dvb_usb_exit+0x2f/0xd0 [dvb_usb]
	 [<c13d03bc>] ? usb_disable_endpoint+0x7c/0xb0
	 [<f80bb48a>] ? dvb_usb_device_exit+0x2a/0x50 [dvb_usb]
	 [<c13d2882>] ? usb_unbind_interface+0x62/0x250
	 [<c136b514>] ? __pm_runtime_idle+0x44/0x70
	 [<c13620d8>] ? __device_release_driver+0x78/0x120
	 [<c1362907>] ? driver_detach+0x87/0x90
	 [<c1361c48>] ? bus_remove_driver+0x38/0x90
	 [<c13d1c18>] ? usb_deregister+0x58/0xb0
	 [<c109fbb0>] ? SyS_delete_module+0x130/0x1f0
	 [<c1055654>] ? task_work_run+0x64/0x80
	 [<c1000fa5>] ? exit_to_usermode_loop+0x85/0x90
	 [<c10013f0>] ? do_fast_syscall_32+0x80/0x130
	 [<c1549f43>] ? sysenter_past_esp+0x40/0x6a
	---[ end trace 6ebc60ef3981792f ]---

```
姝ょ被鏍堣窡韪彁渚涗簡瓒冲鐨勪俊鎭潵瀹氫綅鍐呮牳婧愪唬鐮佷腑鍙戠敓缂洪櫡鐨勯偅涓€琛屻€傛牴鎹棶棰樼殑涓ラ噸绋嬪害锛?
```

	BUG: unable to handle kernel NULL pointer dereference at   (null)
	IP: [<c06969d4>] iret_exc+0x7d0/0xa59
	*pdpt = 000000002258a001 *pde = 0000000000000000
	Oops: 0002 [#1] PREEMPT SMP
	...

```
灏界瀹冩槸涓€涓?**Oops** 鎴栨煇绉嶅叾浠栫被鍨嬬殑鏍堣窡韪紝閫氬父浠嶉渶瑕佹壘鍒板嚭閿欑殑閭ｄ竴琛屾墠鑳藉畾浣嶅拰澶勭悊缂洪櫡銆傚湪鏈珷涓紝鎴戜滑鐢ㄢ€淥ops鈥濇潵娉涙寚鎵€鏈夐渶瑕佸垎鏋愮殑鍚勭被鏍堣窡韪€?
濡傛灉鍐呮牳浣跨敤 `CONFIG_DEBUG_INFO` 缂栬瘧锛屼綘鍙互閫氳繃 `scripts/decode_stacktrace.sh` 鑴氭湰鏉ユ彁鍗囨爤璺熻釜鐨勮川閲忋€?
### 宸插姞杞界殑妯″潡


宸茶姹℃煋鎴栨鍦ㄥ姞杞?鍗歌浇鐨勬ā鍧椾細浠モ€?...)鈥濇爣璁帮紝鍏朵腑姹℃煋鏍囧織鍦?`Documentation/admin-guide/tainted-kernels.rst` 涓弿杩帮紱鈥滄鍦ㄥ姞杞解€濈敤鈥?鈥濇爣娉紝鈥滄鍦ㄥ嵏杞解€濈敤鈥?鈥濇爣娉ㄣ€?

### Oops 娑堟伅浣嶄簬浣曞锛?

閫氬父 Oops 鏂囨湰鐢?`klogd` 浠庡唴鏍哥紦鍐插尯璇诲嚭锛屽苟浜ょ粰 `syslogd` 鍐欏叆涓€涓?syslog 鏂囦欢锛岄€氬父鏄?`/var/log/messages`锛堝彇鍐充簬 `/etc/syslog.conf`锛夈€傚湪浣跨敤 systemd 鐨勭郴缁熶笂锛屽畠涔熷彲鑳界敱
`journald` 瀹堟姢杩涚▼淇濆瓨锛屽苟閫氳繃杩愯 `journalctl` 鍛戒护鏉ヨ闂€?
鏈夋椂 `klogd` 浼氭鎺夛紝杩欐椂浣犲彲浠ヨ繍琛?`dmesg > file` 浠庡唴鏍哥紦鍐插尯璇诲嚭鏁版嵁骞朵繚瀛樸€傛垨鑰呬綘鍙互
`cat /proc/kmsg > file`锛屼笉杩囦綘蹇呴』涓柇浼犺緭鏉ュ仠姝㈠畠锛屽洜涓?`kmsg` 鏄竴涓€滄案涓嶇粨鏉熺殑鏂囦欢鈥濄€?
濡傛灉鏈哄櫒宕╂簝寰楀姝や弗閲嶏紝浠ヨ嚦浜庝綘鏃犳硶杈撳叆鍛戒护鎴栫鐩樹笉鍙敤锛岄偅涔堜綘鏈変笁绉嶉€夋嫨锛?
(1) 浠庡睆骞曟墜鍐欐妱褰曟枃鏈紝骞跺湪鏈哄櫒閲嶅惎鍚庨敭鍏ャ€傝繖寰堥夯鐑︼紝浣嗗鏋滀綘娌℃湁涓哄穿婧冨仛鍑嗗锛岃繖鏄敮涓€鐨勫姙娉曘€傛垨鑰咃紝浣犲彲浠ョ敤鏁扮爜鐩告満鎷嶄笅灞忓箷鈥斺€旇櫧鐒朵笉濂斤紝浣嗘€绘瘮娌℃湁寮恒€傚鏋滄秷鎭粴鍑烘帶鍒跺彴椤堕儴锛屼綘鍙兘浼氬彂鐜扮敤鏇撮珮鐨勫垎杈ㄧ巼寮曞锛堜緥濡?`vga=791`锛夊彲浠ヨ浣犺鍒版洿澶氭枃鏈€傦紙娉ㄦ剰锛氳繖闇€瑕?`vesafb`锛屽洜姝ゅ鈥滄棭鏈熲€濈殑 Oops 娌℃湁甯姪銆傦級

(2) 鐢ㄤ覆鍙ｆ帶鍒跺彴寮曞锛堝弬瑙?`Documentation/admin-guide/serial-console.rst` <serial_console>锛夛紝閫氳繃闆惰皟鍒惰В璋冨櫒鐢电紗杩炴帴鍒板彟涓€鍙版満鍣紝骞剁敤浣犲枩娆㈢殑閫氫俊绋嬪簭鍦ㄩ偅閲屾崟鑾疯緭鍑恒€侻inicom 鏁堟灉寰堝ソ銆?
(3) 浣跨敤 Kdump锛堝弬瑙?`Documentation/admin-guide/kdump/kdump.rst`锛夛紝閫氳繃 `Documentation/admin-guide/kdump/gdbmacros.txt` 涓殑 dmesg gdb 瀹忎粠鏃у唴瀛樹腑鎻愬彇鍐呮牳鐜舰缂撳啿鍖恒€?
### 瀹氫綅缂洪櫡浣嶇疆


濡傛灉浣犺兘灏嗙己闄风殑浣嶇疆鎸囧悜鍐呮牳婧愭枃浠讹紝鎶ュ憡缂洪櫡鐨勬晥鏋滄渶濂姐€傛湁涓ょ鏂规硶鍙互鍋氬埌杩欎竴鐐广€傞€氬父锛屼娇鐢?`gdb` 鏇寸畝鍗曪紝浣嗗唴鏍稿簲棰勫厛浣跨敤璋冭瘯淇℃伅缂栬瘧銆?
##### gdb


GNU 璋冭瘯鍣紙`gdb`锛夋槸纭畾 OOPS 鍦?`vmlinux` 鏂囦欢涓簿纭枃浠朵笌琛屽彿鐨勬渶浣虫柟寮忋€?
鍦ㄥ甫鏈?`CONFIG_DEBUG_INFO` 缂栬瘧鐨勫唴鏍镐笂锛宍gdb` 鐨勪娇鐢ㄦ晥鏋滄渶浣炽€?
```

  $ ./scripts/config -d COMPILE_TEST -e DEBUG_KERNEL -e DEBUG_INFO

```
鍦ㄥ甫鏈?`CONFIG_DEBUG_INFO` 缂栬瘧鐨勫唴鏍镐笂锛屼綘鍙渶澶嶅埗

```

 EIP:    0060:[<c021e50e>]    Not tainted VLI

```

```

  $ gdb vmlinux
  (gdb) l *0xc021e50e

```
濡傛灉浣犳病鏈夊惎鐢?`CONFIG_DEBUG_INFO`锛屼綘鍙互浣跨敤鍑芥暟

```

 EIP is at vt_ioctl+0xda8/0x1482

```

```

  $ ./scripts/config -d COMPILE_TEST -e DEBUG_KERNEL -e DEBUG_INFO
  $ make vmlinux
  $ gdb vmlinux
  (gdb) l *vt_ioctl+0xda8
  0x1888 is in vt_ioctl (drivers/tty/vt/vt_ioctl.c:293).
  288	{
  289		struct vc_data *vc = NULL;
  290		int ret = 0;
  291
  292		console_lock();
  293		if (VT_BUSY(vc_num))
  294			ret = -EBUSY;
  295		else if (vc_num)
  296			vc = vc_deallocate(vc_num);
  297		console_unlock();

```

```

  (gdb) p vt_ioctl
  $1 = {int (struct tty_struct *, unsigned int, unsigned long)} 0xae0 <vt_ioctl>
  (gdb) l *0xae0+0xda8

```

```

  $ make drivers/tty/
  $ gdb drivers/tty/vt/vt_ioctl.o
  (gdb) l *vt_ioctl+0xda8

```

```

     Call Trace:
      [<ffffffff8802c8e9>] :jbd:log_wait_commit+0xa3/0xf5
      [<ffffffff810482d9>] autoremove_wake_function+0x0/0x2e
      [<ffffffff8802770b>] :jbd:journal_stop+0x1be/0x1ee
      ...

```
杩欒〃鏄庨棶棰樺緢鍙兘鍑哄湪 `:jbd:` 妯″潡涓€備綘鍙互鍔犺浇閭ｄ釜妯″潡

```

  $ gdb fs/jbd/jbd.ko
  (gdb) l *log_wait_commit+0xa3

```

     浣犱篃鍙互瀵规爤璺熻釜涓殑浠讳綍鍑芥暟璋冪敤鍋氬悓鏍风殑浜嬫儏锛?
```

	 [<f80bc9ca>] ? dvb_usb_adapter_frontend_exit+0x3a/0x70 [dvb_usb]

     涓婅堪璋冪敤鍙戠敓鐨勪綅缃彲浠ラ€氳繃浠ヤ笅鏂瑰紡鏌ョ湅锛?
	$ gdb drivers/media/usb/dvb-usb/dvb-usb.o
	(gdb) l *dvb_usb_adapter_frontend_exit+0x3a

```
##### objdump


瑕佽皟璇曞唴鏍革紝鍙互浣跨敤 objdump 骞舵煡鎵惧穿婧冭緭鍑轰腑鐨勫崄鍏繘鍒跺亸绉伙紝浠ユ壘鍒版湁鏁堢殑浠ｇ爜琛?姹囩紪琛屻€傚湪娌℃湁璋冭瘯绗﹀彿鐨勬儏鍐典笅锛屼綘浼氱湅鍒拌渚嬬▼鏄剧ず鐨勬眹缂栦唬鐮侊紝浣嗗鏋滀綘鐨勫唴鏍稿甫鏈夎皟璇曠鍙凤紝C 浠ｇ爜涔熶細鍙敤銆傦紙璋冭瘯绗﹀彿鍙互閫氳繃

```

    $ objdump -r -S -l --disassemble net/ipv4/tcp.o

```

   浣犻渶瑕佷綅浜庡唴鏍告爲鐨勬渶椤跺眰锛岃繖鏍峰畠鎵嶈兘鎵惧埌浣犵殑 C 鏂囦欢銆?
濡傛灉浣犳棤娉曡闂簮浠ｇ爜锛屼粛鐒跺彲浠ヤ娇鐢ㄤ互涓嬫柟娉曡皟璇曚竴浜涘穿婧冭浆鍌紙绀轰緥宕╂簝杞偍杈撳嚭濡?
```

     EIP is at 	+0x14/0x4c0
      ...
     Code: 44 24 04 e8 6f 05 00 00 e9 e8 fe ff ff 8d 76 00 8d bc 27 00 00
     00 00 55 57  56 53 81 ec bc 00 00 00 8b ac 24 d0 00 00 00 8b 5d 08
     <8b> 83 3c 01 00 00 89 44  24 14 8b 45 28 85 c0 89 44 24 18 0f 85

     灏嗗瓧鑺傛斁鍏ヤ竴涓€渇oo.s鈥濇枃浠朵腑锛屽涓嬫墍绀猴細

            .text
            .globl foo
     foo:
            .byte  .... /* bytes from Code: part of OOPS dump */

     鐢?"gcc -c -o foo.o foo.s" 缂栬瘧瀹冿紝鐒跺悗鏌ョ湅 "objdump --disassemble foo.o" 鐨勮緭鍑恒€?
     Output:

     ip_queue_xmit:
         push       %ebp
         push       %edi
         push       %esi
         push       %ebx
         sub        $0xbc, %esp
         mov        0xd0(%esp), %ebp        ! %ebp = arg0 (skb)
         mov        0x8(%ebp), %ebx         ! %ebx = skb->sk
         mov        0x13c(%ebx), %eax       ! %eax = inet_sk(sk)->opt

```
`scripts/decodecode` 鍙敤浜庤嚜鍔ㄥ寲鍏朵腑澶ч儴鍒嗗伐浣滐紝鍏蜂綋鍙栧喅浜庢鍦ㄨ皟璇曠殑 CPU 鏋舵瀯銆?
### 鎶ュ憡缂洪櫡


涓€鏃﹂€氳繃妫€鏌ヤ綅缃‘瀹氫簡缂洪櫡鍙戠敓鐨勪綅缃紝浣犳棦鍙互閫夋嫨鑷繁灏濊瘯淇瀹冿紝涔熷彲浠ュ皢鍏舵姤鍛婄粰涓婃父銆?
涓轰簡灏嗗叾鎶ュ憡缁欎笂娓革紝浣犲簲璇ョ‘瀹氬彈褰卞搷浠ｇ爜鐨勭己闄疯窡韪櫒锛堝鏋滄湁鐨勮瘽锛夋垨閭欢鍒楄〃銆傝繖鍙互閫氳繃浣跨敤
`get_maintainer.pl` 鑴氭湰鏉ュ畬鎴愩€?
渚嬪锛屽鏋滀綘鍦?gspca 鐨?sonixj.c 鏂囦欢涓彂鐜颁簡涓€涓己闄凤紝浣犲彲浠ュ緱鍒?
```

	$ ./scripts/get_maintainer.pl --bug -f drivers/media/usb/gspca/sonixj.c
	Hans Verkuil <hverkuil@kernel.org> (odd fixer:GSPCA USB WEBCAM DRIVER,commit_signer:1/1=100%)
	Mauro Carvalho Chehab <mchehab@kernel.org> (maintainer:MEDIA INPUT INFRASTRUCTURE (V4L/DVB),commit_signer:1/1=100%)
	Tejun Heo <tj@kernel.org> (commit_signer:1/1=100%)
	Bhaktipriya Shridhar <bhaktipriya96@gmail.com> (commit_signer:1/1=100%,authored:1/1=100%,added_lines:4/4=100%,removed_lines:9/9=100%)
	linux-media@vger.kernel.org (open list:GSPCA USB WEBCAM DRIVER)
	linux-kernel@vger.kernel.org (open list)

```
璇锋敞鎰忥紝瀹冧細鎸囧悜锛?
- 鏈€鍚庝慨鏀硅繃婧愪唬鐮佺殑寮€鍙戣€咃紙濡傛灉杩欐槸鍦?git 鏍戝唴瀹屾垚鐨勶級銆傚湪涓婇潰渚嬪瓙涓槸 Tejun 鍜?Bhaktipriya锛堝湪杩欎釜鍏蜂綋鎯呭喌涓嬶紝浠栦滑閮芥病鏈夌湡姝ｅ弬涓庤鏂囦欢鐨勫紑鍙戯級锛?- 椹卞姩缁存姢鑰咃紙Hans Verkuil锛夛紱
- 瀛愮郴缁熺淮鎶よ€咃紙Mauro Carvalho Chehab锛夛紱
- 椹卞姩鍜?鎴栧瓙绯荤粺閭欢鍒楄〃锛坙inux-media@vger.kernel.org锛夛紱
- Linux 鍐呮牳閭欢鍒楄〃锛坙inux-kernel@vger.kernel.org锛夛紱
- 椹卞姩/瀛愮郴缁熺殑缂洪櫡鎶ュ憡 URI锛堜笂渚嬩腑涓虹┖锛夈€?
濡傛灉鍒楄〃涓湯灏惧寘鍚己闄锋姤鍛?URI锛岃浼樺厛浣跨敤瀹冧滑鑰屼笉鏄數瀛愰偖浠躲€傚惁鍒欙紝璇峰皢缂洪櫡鎶ュ憡缁欑敤浜庤浠ｇ爜寮€鍙戠殑閭欢鍒楄〃锛坙inux-media ML锛夛紝骞舵妱閫佺粰椹卞姩缁存姢鑰咃紙Hans锛夈€?
濡傛灉浣犲畬鍏ㄤ笉鐭ラ亾璇ユ妸鎶ュ憡鍙戠粰璋侊紝骞朵笖 `get_maintainer.pl` 涔熸病鏈夋彁渚涗换浣曟湁鐢ㄧ殑淇℃伅锛岃灏嗗叾鍙戦€佸埌
linux-kernel@vger.kernel.org銆?
鎰熻阿浣犱负璁?Linux 灏藉彲鑳界ǔ瀹氭墍鎻愪緵鐨勫府鍔┿€?
### 淇缂洪櫡


濡傛灉浣犳噦缂栫▼锛屼綘鍙互閫氳繃涓嶄粎鎶ュ憡缂洪櫡銆佽繕鎻愪緵瑙ｅ喅鏂规鐨勬柟寮忔潵甯姪鎴戜滑銆傛瘯绔燂紝寮€婧愮殑鎰忎箟鍦ㄤ簬鍒嗕韩浣犳墍鍋氱殑锛屼綘闅鹃亾涓嶆兂鍥犺嚜宸辩殑鎵嶅崕鑰岃幏寰楄鍙悧锛?
濡傛灉浣犲喅瀹氳蛋杩欐潯璺紝涓€鏃︿綘鎯冲嚭浜嗕慨澶嶆柟妗堬紝璇峰皢鍏舵彁浜ょ粰涓婃父銆?
涓嶈繃锛岃鍔″繀闃呰
`Documentation/process/submitting-patches.rst` <submittingpatches>锛屼互甯姪浣犳彁浜ょ殑浠ｇ爜琚帴鍙椼€?

---------------------------------------------------------------------------

### 鍏充簬浣跨敤 ``klogd`` 杩涜 Oops 杩借釜鐨勮鏄?

涓轰簡甯姪 Linus 鍜屽叾浠栧唴鏍稿紑鍙戣€咃紝`klogd` 涓姞鍏ヤ簡澶ч噺鐢ㄤ簬澶勪繚鎶ゆ晠闅滅殑鏀寔銆備负浜嗚幏寰楀鍦板潃瑙ｆ瀽鐨?瀹屾暣鏀寔锛岃嚦灏戝簲浣跨敤 `sysklogd` 杞欢鍖?1.3-pl3 鐗堟湰銆?
褰撳彂鐢熶繚鎶ゆ晠闅滄椂锛宍klogd` 瀹堟姢杩涚▼浼氳嚜鍔ㄥ皢鍐呮牳鏃ュ織娑堟伅涓殑閲嶈鍦板潃杞崲涓哄畠浠殑绗﹀彿绛変环褰㈠紡銆傝繖涓?杞崲鍚庣殑鍐呮牳娑堟伅闅忓悗閫氳繃 `klogd` 姝ｅ湪浣跨敤鐨勪换浣曟姤鍛婃満鍒惰浆鍙戙€備繚鎶ゆ晠闅滄秷鎭彲浠ョ畝鍗曞湴浠庢秷鎭枃浠朵腑
鎴彇骞惰浆鍙戠粰鍐呮牳寮€鍙戣€呫€?
`klogd` 鎵ц涓ょ绫诲瀷鐨勫湴鍧€瑙ｆ瀽銆傜涓€绉嶆槸闈欐€佽浆鎹紝绗簩绉嶆槸鍔ㄦ€佽浆鎹€傞潤鎬佽浆鎹娇鐢?System.map 鏂囦欢銆?涓轰簡杩涜闈欐€佽浆鎹紝`klogd` 瀹堟姢杩涚▼蹇呴』鑳藉鍦ㄥ畧鎶よ繘绋嬪垵濮嬪寲鏃舵壘鍒颁竴涓郴缁熸槧灏勬枃浠躲€傛湁鍏?`klogd` 濡備綍
鎼滅储鏄犲皠鏂囦欢鐨勪俊鎭紝璇峰弬闃?klogd 鎵嬪唽椤点€?
褰撲娇鐢ㄥ唴鏍稿彲鍔犺浇妯″潡鏃讹紝鍔ㄦ€佸湴鍧€瑙ｆ瀽寰堥噸瑕併€傜敱浜庡唴鏍告ā鍧楃殑鍐呭瓨鏄粠鍐呮牳鐨勫姩鎬佸唴瀛樻睜涓垎閰嶇殑锛屽洜姝?鏃犺鏄ā鍧楃殑璧峰浣嶇疆杩樻槸妯″潡涓殑鍑芥暟鍜岀鍙烽兘娌℃湁鍥哄畾鐨勪綅缃€?
鍐呮牳鏀寔涓€浜涚郴缁熻皟鐢紝鍏佽绋嬪簭纭畾鍔犺浇浜嗗摢浜涙ā鍧椾互鍙婂畠浠湪鍐呭瓨涓殑浣嶇疆銆傞€氳繃瀵硅繖浜涚郴缁熻皟鐢ㄧ殑浣跨敤锛?`klogd` 瀹堟姢杩涚▼鏋勫缓浜嗕竴寮犵鍙疯〃锛屽彲鐢ㄤ簬璋冭瘯鍙姞杞藉唴鏍告ā鍧椾腑鍙戠敓鐨勪繚鎶ゆ晠闅溿€?
鑷冲皯锛宍klogd` 浼氭彁渚涚敓鎴愪繚鎶ゆ晠闅滅殑妯″潡鍚嶇О銆傚鏋滃彲鍔犺浇妯″潡鐨勫紑鍙戣€呴€夋嫨浠庢ā鍧椾腑瀵煎嚭绗﹀彿淇℃伅锛屽彲鑳借繕浼氭湁
棰濆鐨勭鍙蜂俊鎭彲鐢ㄣ€?
鐢变簬鍐呮牳妯″潡鐜鏄姩鎬佺殑锛屽洜姝ゅ繀椤绘湁涓€绉嶆満鍒惰兘鍦ㄦā鍧楃幆澧冨彂鐢熷彉鍖栨椂閫氱煡 `klogd` 瀹堟姢杩涚▼銆傛湁涓€浜涘懡浠よ
閫夐」鍙緵浣跨敤锛屽畠浠厑璁?klogd 鍚戝綋鍓嶆鍦ㄦ墽琛岀殑瀹堟姢杩涚▼鍙戜俊鍙凤紝琛ㄧず搴旇鍒锋柊绗﹀彿淇℃伅銆傛湁鍏虫洿澶氫俊鎭紝璇峰弬闃?`klogd` 鎵嬪唽椤点€?
sysklogd 鍙戣鐗堜腑鍖呭惈涓€涓ˉ涓侊紝瀹冧慨鏀?`modules-2.0.0` 杞欢鍖咃紝浣垮叾鍦ㄦ瘡娆″姞杞芥垨鍗歌浇妯″潡鏃惰嚜鍔ㄥ悜 klogd
鍙戜俊鍙枫€傚簲鐢ㄦ琛ヤ竵鍩烘湰涓婂彲浠ヤ负璋冭瘯鍐呮牳鍙姞杞芥ā鍧椾腑鍙戠敓鐨勪繚鎶ゆ晠闅滄彁渚涙棤缂濇敮鎸併€?
浠ヤ笅鏄彲鍔犺浇妯″潡涓繚鎶ゆ晠闅滅殑涓€涓ず渚?
```

	Aug 29 09:51:01 blizard kernel: Unable to handle kernel paging request at virtual address f15e97cc
	Aug 29 09:51:01 blizard kernel: current->tss.cr3 = 0062d000, %cr3 = 0062d000
	Aug 29 09:51:01 blizard kernel: *pde = 00000000
	Aug 29 09:51:01 blizard kernel: Oops: 0002
	Aug 29 09:51:01 blizard kernel: CPU:    0
	Aug 29 09:51:01 blizard kernel: EIP:    0010:[oops:_oops+16/3868]
	Aug 29 09:51:01 blizard kernel: EFLAGS: 00010212
	Aug 29 09:51:01 blizard kernel: eax: 315e97cc   ebx: 003a6f80   ecx: 001be77b   edx: 00237c0c
	Aug 29 09:51:01 blizard kernel: esi: 00000000   edi: bffffdb3   ebp: 00589f90   esp: 00589f8c
	Aug 29 09:51:01 blizard kernel: ds: 0018   es: 0018   fs: 002b   gs: 002b   ss: 0018
	Aug 29 09:51:01 blizard kernel: Process oops_test (pid: 3374, process nr: 21, stackpage=00589000)
	Aug 29 09:51:01 blizard kernel: Stack: 315e97cc 00589f98 0100b0b4 bffffed4 0012e38e 00240c64 003a6f80 00000001
	Aug 29 09:51:01 blizard kernel:        00000000 00237810 bfffff00 0010a7fa 00000003 00000001 00000000 bfffff00
	Aug 29 09:51:01 blizard kernel:        bffffdb3 bffffed4 ffffffda 0000002b 0007002b 0000002b 0000002b 00000036
	Aug 29 09:51:01 blizard kernel: Call Trace: [oops:_oops_ioctl+48/80] [_sys_ioctl+254/272] [_system_call+82/128]
	Aug 29 09:51:01 blizard kernel: Code: c7 00 05 00 00 00 eb 08 90 90 90 90 90 90 90 90 89 ec 5d c3

```
