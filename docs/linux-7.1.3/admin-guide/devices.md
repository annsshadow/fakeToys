
## Linux allocated 璁惧 (4.x+ 鐗堟湰)


姝?鍒楀嚭 鏄?the Linux 璁惧 鍒楀嚭, the official registry 鐨?allocated
璁惧 numbers 鍜?`/dev` directory nodes 鐢ㄤ簬 the Linux operating
绯荤粺.

The 鐗堟湰 鐨?姝?document 鍦?lanana.org 鏄?鏃?longer maintained.  姝?
鐗堟湰 鍦?the mainline Linux 鍐呮牳 鏄?the master document.  Updates
shall 涓?sent 浣滀负 patches 鍒?the 鍐呮牳 maintainers (鍙傝 the
Documentation/杩涚▼/submitting-patches.rst <submittingpatches> document).
Specifically explore the sections titled "CHAR 鍜?MISC 椹卞姩", 鍜?
"鍧?LAYER" 鍦?the MAINTAINERS 鏂囦欢 鍒?find the right maintainers
鍒?involve 鐢ㄤ簬 character 鍜?鍧?璁惧.

姝?document 鏄?included 鐢?鍙傝€?杩涘叆 the 鏂囦欢绯荤粺 Hierarchy
鏍囧噯 (FHS).	 The FHS 鏄?鍙敤 鏉ヨ嚜 https://www.pathname.com/fhs/.

Allocations marked (68k/Amiga) apply 鍒?Linux/68k 鍦?the Amiga
platform 浠?	Allocations marked (68k/Atari) apply 鍒?Linux/68k 鍦?
the Atari platform 浠?

姝?document 鏄?鍦?the 鍏叡 domain.	The authors requests, 鐒惰€?
璇?semantically altered versions 鏄?涓?distributed 鏃?
permission 鐨?the authors, assuming the authors 鍙?涓?contacted 鏃?
涓€涓?unreasonable effort.



  璁惧 椹卞姩 AUTHORS 璇?璇诲彇 姝?

  Linux 鐜板湪 鍏锋湁 extensive 鏀寔 鐢ㄤ簬 鍔ㄦ€?鍒嗛厤 鐨?璁惧 numbering
  鍜?鍙?浣跨敤 `sysfs` 鍜?`udev` (`systemd`) 鍒?handle the naming needs.
  瀛樺湪 浠嶇劧 涓€浜?exceptions 鍦?the 涓茶 鍜?boot 璁惧 area. 涔嬪墠
  asking   鐢ㄤ簬 涓€涓?璁惧 鏁板瓧 纭繚 鎮?actually 闇€瑕?one.

  鍒?鍏锋湁 涓€涓?涓昏 鏁板瓧 allocated, 鎴?涓€涓?娆¤ 鏁板瓧 鍦?situations
  浣曞 璇?applies (e.g. busmice), 璇?submit 涓€涓?patch 鍜?send 鍒?
  the authors 浣滀负 indicated 涓婃枃.

  Keep the description 鐨?the 璁惧 *鍦?the 鐩稿悓 鏍煎紡
  浣滀负 姝?鍒楀嚭*. The reason 鐢ㄤ簬 杩欐槸 璇?瀹冩槸 the 浠?way 鎴戜滑 鍏锋湁
  found 鍒?ensure 鎴戜滑 鍏锋湁 鍏ㄩ儴 the requisite information 鍒?publish 鎮ㄧ殑
  璁惧 鍜?avoid conflicts.

  Finally, 鏈夋椂 鎴戜滑 鍏锋湁 鍒?play "namespace police."  璇?don't 涓?
  offended.  鎴戜滑 閫氬父 get submissions 鐢ㄤ簬 `/dev` names 璇?灏嗕細 涓?bound
  鍒?cause conflicts down the road.  鎴戜滑 鏄?trying 鍒?avoid getting 鍦?涓€涓?
  situation 浣曞 鎴戜滑 灏嗕細 鍏锋湁 鍒?suffer 涓€涓?incompatible forward
  change.  鍥犳, 璇?consult 涓?us **涔嬪墠** 鎮?make 鎮ㄧ殑
  璁惧 names 鍜?numbers 鍦?浠讳綍 way 鍏叡, 鑷冲皯 鍒?the point
  浣曞 瀹?灏嗕細 涓?鍦?鍏ㄩ儴 difficult 鍒?get them changed.

  鎮ㄧ殑 cooperation 鏄?appreciated.

   :literal:

### 棰濆 ``/dev/`` directory 鏉＄洰


姝?section details 棰濆 鏉＄洰 璇?搴斿綋 鎴?鍙?exist 鍦?
the /dev directory.  瀹冩槸 preferred 璇?symbolic links 浣跨敤 the 鐩稿悓
form (absolute 鎴?relative) 浣滀负 鏄?indicated 姝ゅ.  Links 鏄?
classified 浣滀负 "hard" 鎴?"symbolic" depending 鍦?the preferred 绫诲瀷 鐨?
link; 鑻?鍙兘, the indicated 绫诲瀷 鐨?link 搴斿綋 涓?浣跨敤.

Compulsory links
++++++++++++++++

杩欎簺 links 搴斿綋 exist 鍦?鍏ㄩ儴 绯荤粺:

=============== =============== =============== ===============================
/dev/fd		/proc/self/fd	symbolic	鏂囦欢 鎻忚堪绗?
/dev/stdin	fd/0		symbolic	stdin 鏂囦欢 鎻忚堪绗?
/dev/stdout	fd/1		symbolic	stdout 鏂囦欢 鎻忚堪绗?
/dev/stderr	fd/2		symbolic	stderr 鏂囦欢 鎻忚堪绗?
/dev/nfsd	socksys		symbolic	蹇呴渶 鐢?iBCS-2
/dev/X0R	null		symbolic	蹇呴渶 鐢?iBCS-2
=============== =============== =============== ===============================

娉ㄦ剰: `/dev/X0R` 鏄?<letter X>-<digit 0>-<letter R>.

Recommended links
+++++++++++++++++

瀹冩槸 recommended 璇?杩欎簺 links exist 鍦?鍏ㄩ儴 绯荤粺:


=============== =============== =============== ===============================
/dev/鏍稿績	/proc/kcore	symbolic	Backward compatibility
/dev/ramdisk	ram0		symbolic	Backward compatibility
/dev/ftape	qft0		symbolic	Backward compatibility
/dev/bttv0	瑙嗛0		symbolic	Backward compatibility
/dev/radio	radio0		symbolic	Backward compatibility
/dev/i2o**	/dev/i2o/**	symbolic	Backward compatibility
=============== =============== =============== ===============================

Suggested 鏇存棭 `/dev/scd?` alternative names 鐢ㄤ簬 `/dev/sr?`
CD-ROM 鍜?鍏朵粬 optical drives (浣跨敤 SCSI 鍛戒护) 鏇炬槸 removed
鍦?`udev` 鐗堟湰 174 璇?鏇炬槸 released 鍦?2011.

Locally 瀹氫箟 links
+++++++++++++++++++++

The 浠ヤ笅 links 鍙?涓?established locally 鍒?conform 鍒?the
閰嶇疆 鐨?the 绯荤粺.  杩欐槸 merely 涓€涓?tabulation 鐨?existing
practice, 鍜?鎵ц 涓?constitute 涓€涓?recommendation.  鐒惰€? 鑻?瀹冧滑
exist, 瀹冧滑 搴斿綋 鍏锋湁 the 浠ヤ笅 uses.

=============== =============== =============== ===============================
/dev/榧犳爣	榧犳爣 绔彛	symbolic	鐢垫祦 榧犳爣 璁惧
/dev/tape	tape 璁惧	symbolic	鐢垫祦 tape 璁惧
/dev/cdrom	CD-ROM 璁惧	symbolic	鐢垫祦 CD-ROM 璁惧
/dev/鎵弿浠?鎵弿浠?	symbolic	鐢垫祦 鎵弿浠?璁惧
/dev/modem	modem 绔彛	symbolic	鐢垫祦 dialout 璁惧
/dev/root	root 璁惧	symbolic	鐢垫祦 root 鏂囦欢绯荤粺
/dev/swap	swap 璁惧	symbolic	鐢垫祦 swap 璁惧
=============== =============== =============== ===============================

`/dev/modem` 搴斿綋 涓?涓?浣跨敤 鐢ㄤ簬 涓€涓?modem 鍏?supports dialin 浣滀负
well 浣滀负 dialout, 浣滀负 瀹?tends 鍒?cause 閿?鏂囦欢 problems.  鑻?瀹?
exists, `/dev/modem` 搴斿綋 point 鍒?the appropriate primary TTY 璁惧
(the 浣跨敤 鐨?the alternate callout 璁惧 鏄?宸插簾寮?.

鐢ㄤ簬 SCSI 璁惧, `/dev/tape` 鍜?`/dev/cdrom` 搴斿綋 point 鍒?the
**cooked** 璁惧 (`/dev/st**` 鍜?`/dev/sr**`, respectively), whereas
`/dev/scanner` 搴斿綋 point 鍒?the appropriate generic
SCSI 璁惧 (`/dev/sg*`).

`/dev/mouse` 鍙?point 鍒?涓€涓?primary 涓茶 TTY 璁惧, 涓€涓?纭欢 榧犳爣
璁惧, 鎴?涓€涓?濂楁帴瀛?鐢ㄤ簬 涓€涓?榧犳爣 椹卞姩 program (e.g. `/dev/gpmdata`).

Sockets 鍜?pipes
+++++++++++++++++

Non-transient sockets 鍜?named pipes 鍙?exist 鍦?/dev.  閫氱敤 鏉＄洰 鏄?

=============== =============== ===============================================
/dev/鎵撳嵃鏈?濂楁帴瀛?	lpd 鏈湴 濂楁帴瀛?
/dev/log	濂楁帴瀛?	syslog 鏈湴 濂楁帴瀛?
/dev/gpmdata	濂楁帴瀛?	gpm 榧犳爣 multiplexer
=============== =============== ===============================================

Mount points
++++++++++++

The 浠ヤ笅 names 鏄?reserved 鐢ㄤ簬 mounting 鐗规畩 鏂囦欢绯荤粺
鍦ㄢ€︿笅 /dev.  杩欎簺 鐗规畩 鏂囦欢绯荤粺 鎻愪緵 鍐呮牳 interfaces 璇?
cannot 涓?provided 涓?鏍囧噯 璁惧 nodes.

=============== =============== ===============================================
/dev/pts	devpts		PTY slave 鏂囦欢绯荤粺
/dev/shm	tmpfs		POSIX shared 鍐呭瓨 maintenance access
=============== =============== ===============================================

### Terminal 璁惧


Terminal, 鎴?TTY 璁惧 鏄?涓€涓?鐗规畩 绫?鐨?character 璁惧.  涓€涓?
terminal 璁惧 鏄?浠讳綍 璁惧 璇?鍙互 act 浣滀负 涓€涓?controlling terminal
鐢ㄤ簬 涓€涓?浼氳瘽; 姝?鍖呭惈 铏氭嫙 consoles, 涓茶 ports, 鍜?
pseudoterminals (PTYs).

鍏ㄩ儴 terminal 璁惧 share 涓€涓?閫氱敤 set 鐨?capabilities known 浣滀负 line
disciplines; 杩欎簺 鍖呭惈 the 閫氱敤 terminal line discipline 浣滀负 well
浣滀负 SLIP 鍜?PPP modes.

鍏ㄩ儴 terminal 璁惧 鏄?named similarly; 姝?section explains the
naming 鍜?浣跨敤 鐨?the 鍚勭 types 鐨?TTYs.  娉ㄦ剰 璇?the naming
conventions 鍖呭惈 鑻ュ共 historical warts; 涓€浜?鐨?杩欎簺 鏄?
Linux-specific, 涓€浜?鏇炬槸 inherited 鏉ヨ嚜 鍏朵粬 绯荤粺, 鍜?涓€浜?
reflect Linux outgrowing 涓€涓?borrowed convention.

涓€涓?hash mark (`#`) 鍦?涓€涓?璁惧 name 鏄?浣跨敤 姝ゅ 鍒?indicate 涓€涓?decimal
鏁板瓧 鏃?leading zeroes.

铏氭嫙 consoles 鍜?the console 璁惧
+++++++++++++++++++++++++++++++++++++++

铏氭嫙 consoles 鏄?full-screen terminal displays 鍦?the 绯荤粺 瑙嗛
鐩戣鍣?  铏氭嫙 consoles 鏄?named `/dev/tty#`, 涓?numbering
starting 鍦?`/dev/tty1`; `/dev/tty0` 鏄?the 鐢垫祦 铏氭嫙 console.
`/dev/tty0` 鏄?the 璁惧 璇?搴斿綋 涓?浣跨敤 鍒?access the 绯荤粺 瑙嗛
鍗?鍦?閭ｄ簺 architectures 鐢ㄤ簬 鍏?the 甯?缂撳啿鍖?璁惧
(`/dev/fb*`) 鏄?涓?applicable. 鎵ц 涓?浣跨敤 `/dev/console`
鐢ㄤ簬 姝?purpose.

The console 璁惧, `/dev/console`, 鏄?the 璁惧 鍒?鍏?绯荤粺
messages 搴斿綋 涓?sent, 鍜?鍦?鍏?logins 搴斿綋 涓?permitted 鍦?
single-user 妯″紡.  Starting 涓?Linux 2.1.71, `/dev/console` 鏄?managed
鐢?the 鍐呮牳; 鐢ㄤ簬 鍓嶄竴涓?versions 瀹?搴斿綋 涓?涓€涓?symbolic link 鍒?
浠讳竴涓?`/dev/tty0`, 涓€涓?鐗瑰畾 铏氭嫙 console 渚嬪 `/dev/tty1`, 鎴?鍒?
涓€涓?涓茶 绔彛 primary (`tty**`, 涓?`cu**`) 璁惧, depending 鍦?the
閰嶇疆 鐨?the 绯荤粺.

涓茶 ports
++++++++++++

涓茶 ports 鏄?RS-232 涓茶 ports 鍜?浠讳綍 璁惧 鍏?simulates
one, 浠讳竴涓?鍦?纭欢 (渚嬪 鍐呴儴 modems) 鎴?鍦?杞欢 (姝ょ被
浣滀负 the ISDN 椹卞姩.)  鍦ㄢ€︿笅 Linux, 姣忎釜 涓茶 ports 鍏锋湁 two 璁惧
names, the primary 鎴?callin 璁惧 鍜?the alternate 鎴?callout one.
姣忎釜 kind 鐨?璁惧 鏄?indicated 鐢?涓€涓?涓嶅悓 letter.	 鐢ㄤ簬 浠讳綍
letter X, the names 鐨?the 璁惧 鏄?`/dev/ttyX#` 鍜?`/dev/cux#`,
respectively; 鐢ㄤ簬 historical reasons, `/dev/ttyS#` 鍜?`/dev/ttyC#`
correspond 鍒?`/dev/cua#` 鍜?`/dev/cub#`. 鍦?the future, 瀹?搴斿綋 涓?
expected 璇?澶氫釜 letters 灏?涓?浣跨敤; 鍏ㄩ儴 letters 灏?涓?upper
case 鐢ㄤ簬 the "tty" 璁惧 (e.g. `/dev/ttyDP#`) 鍜?lower case 鐢ㄤ簬 the
"cu" 璁惧 (e.g. `/dev/cudp#`).

The names `/dev/ttyQ#` 鍜?`/dev/cuq#` 鏄?reserved 鐢ㄤ簬 鏈湴 浣跨敤.

The alternate 璁惧 鎻愪緵 鐢ㄤ簬 kernel-based exclusion 鍜?somewhat
涓嶅悓 defaults 姣?the primary 璁惧.  瀹冧滑鐨?涓昏 purpose 鏄?鍒?
鍏佽 the 浣跨敤 鐨?涓茶 ports 涓?programs 涓?鏃?inherent 鎴?broken
鏀寔 鐢ㄤ簬 涓茶 ports.  瀹冧滑鐨?浣跨敤 鏄?宸插簾寮? 鍜?瀹冧滑 鍙?涓?
removed 鏉ヨ嚜 涓€涓?future 鐗堟湰 鐨?Linux.

Arbitration 鐨?涓茶 ports 鏄?provided 鐢?the 浣跨敤 鐨?閿?鏂囦欢 涓?
the names `/var/lock/LCK..ttyX#`. The contents 鐨?the 閿?鏂囦欢 搴斿綋
涓?the PID 鐨?the locking 杩涚▼ 浣滀负 涓€涓?ASCII 鏁板瓧.

瀹冩槸 閫氱敤 practice 鍒?install links 渚嬪 /dev/modem
鍏?point 鍒?涓茶 ports.  涓轰簡 ensure proper locking 鍦?the
presence 鐨?杩欎簺 links, 瀹冩槸 recommended 璇?杞欢 chase
symlinks 鍜?閿?鍏ㄩ儴 鍙兘 names; additionally, 瀹冩槸 recommended
璇?涓€涓?閿?鏂囦欢 涓?installed 涓?the corresponding alternate
璁惧.	 涓轰簡 avoid deadlocks, 瀹冩槸 recommended 璇?the 閿?
鏄?acquired 鍦?the 浠ヤ笅 order, 鍜?released 鍦?the reverse:

 1. The symbolic link name, 鑻?浠讳綍 (`/var/lock/LCK..modem`)
 2. The "tty" name (`/var/lock/LCK..ttyS2`)
 3. The alternate 璁惧 name (`/var/lock/LCK..cua2`)

鍦ㄨ鎯呭喌涓?鐨?nested symbolic links, the 閿?鏂囦欢 搴斿綋 涓?
installed 鍦?the order the symlinks 鏄?resolved.

鍦ㄢ€︿笅 鏃?circumstances 搴斿綋 涓€涓?搴旂敤绋嬪簭 hold 涓€涓?閿?鍚屾椂 waiting
鐢ㄤ簬 another 鍒?涓?released.  姝ゅ, applications 鍏?attempt
鍒?鍒涘缓 閿?鏂囦欢 鐢ㄤ簬 the corresponding alternate 璁惧 names
搴斿綋 take 杩涘叆 account the possibility 鐨?姝ｅ湪 浣跨敤 鍦?涓€涓?non-serial
绔彛 TTY, 鐢ㄤ簬 鍏?鏃?alternate 璁惧 灏嗕細 exist.

Pseudoterminals (PTYs)
++++++++++++++++++++++

Pseudoterminals, 鎴?PTYs, 鏄?浣跨敤 鍒?鍒涘缓 login sessions 鎴?鎻愪緵
鍏朵粬 capabilities requiring 涓€涓?TTY line discipline (including SLIP 鎴?
PPP capability) 鍒?arbitrary data-generation 杩涚▼.	 姣忎釜 PTY 鍏锋湁
涓€涓?master side, named `/dev/pty[p-za-e][0-9a-f]`, 鍜?涓€涓?slave side, named
`/dev/tty[p-za-e][0-9a-f]`.  The 鍐呮牳 arbitrates the 浣跨敤 鐨?PTYs 鐢?
allowing 姣忎釜 master side 鍒?涓?opened 浠?涓€鏃?

涓€鏃?the master side 鍏锋湁 宸茬粡 opened, the corresponding slave 璁惧
鍙?涓?浣跨敤 鍦?the 鐩稿悓 manner 浣滀负 浠讳綍 TTY 璁惧.  The master 鍜?
slave 璁惧 鏄?connected 鐢?the 鍐呮牳, generating the equivalent
鐨?涓€涓?bidirectional pipe 涓?TTY capabilities.

Recent versions 鐨?the Linux kernels 鍜?GNU libc 鍖呭惈 鏀寔 鐢ㄤ簬
the 绯荤粺 V/Unix98 naming scheme 鐢ㄤ簬 PTYs, 鍏?assigns 涓€涓?閫氱敤
璁惧, `/dev/ptmx`, 鍒?鍏ㄩ儴 the masters (opening 瀹?灏?automatically
give 鎮?涓€涓?previously unassigned PTY) 鍜?涓€涓?subdirectory, `/dev/pts`,
鐢ㄤ簬 the slaves; the slaves 鏄?named 涓?decimal integers (`/dev/pts/#`
鍦?鎴戜滑鐨?notation).  姝?removes the problem 鐨?exhausting the
namespace 鍜?enables the 鍐呮牳 鍒?automatically 鍒涘缓 the 璁惧
nodes 鐢ㄤ簬 the slaves 鍦?demand 浣跨敤 the "devpts" 鏂囦欢绯荤粺.
