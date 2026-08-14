
## KSMBD - SMB3 鍐呮牳 Server


KSMBD 鏄?涓€涓?linux 鍐呮牳 server 鍏?implements SMB3 鍗忚 鍦?鍐呮牳绌洪棿
鐢ㄤ簬 sharing 鏂囦欢 鍦ㄢ€︿笂 缃戠粶.

## KSMBD architecture


The subset 鐨?鎬ц兘 related 鎿嶄綔 belong 鍦?kernelspace 鍜?
the 鍏朵粬 subset 鍏?belong 鍒?鎿嶄綔 鍏?鏄?涓?really related 涓?
鎬ц兘 鍦?userspace. 鍥犳, DCE/RPC 绠＄悊 璇?鍏锋湁 historically resulted
杩涘叆 涓€涓?鏁板瓧 鐨?缂撳啿鍖?overflow issues 鍜?dangerous 瀹夊叏 bugs 鍜?鐢ㄦ埛
account 绠＄悊 鏄?implemented 鍦?鐢ㄦ埛绌洪棿 浣滀负 ksmbd.mountd.
鏂囦欢 鎿嶄綔 璇?鏄?related 涓?鎬ц兘 (鎵撳紑/璇诲彇/鍐欏叆/鍏抽棴 绛?)
鍦?鍐呮牳绌洪棿 (ksmbd). 姝?涔?allows 鐢ㄤ簬 easier integration 涓?VFS
鎺ュ彛 鐢ㄤ簬 鍏ㄩ儴 鏂囦欢 鎿嶄綔.

### ksmbd (鍐呮牳 daemon)


褰?the server daemon 鏄?started, 瀹?starts up 涓€涓?forker 绾跨▼
(ksmbd/鎺ュ彛 name) 鍦?鍒濆鍖?time 鍜?鎵撳紑 涓€涓?dedicated 绔彛 445
鐢ㄤ簬 listening 鍒?SMB requests. Whenever 鏂?clients make 涓€涓?璇锋眰, the Forker
绾跨▼ 灏?accept the client 杩炴帴 鍜?fork 涓€涓?鏂?绾跨▼ 鐢ㄤ簬 涓€涓?dedicated
communication channel 涔嬮棿 the client 鍜?the server. 瀹?allows 鐢ㄤ簬 骞惰
processing 鐨?SMB requests(鍛戒护) 鏉ヨ嚜 clients 浠ュ強 allowing 鐢ㄤ簬 鏂?
clients 鍒?make 鏂?connections. 姣忎釜 瀹炰緥 鏄?named ksmbd/1~n(绔彛 鏁板瓧)
鍒?indicate connected clients. Depending 鍦?the SMB 璇锋眰 types, 姣忎釜 鏂?
绾跨▼ 鍙?decide 鍒?pass through the 鍛戒护 鍒?the 鐢ㄦ埛绌洪棿 (ksmbd.mountd),
currently DCE/RPC 鍛戒护 鏄?identified 鍒?涓?handled through the 鐢ㄦ埛绌洪棿.
鍒?further utilize the linux 鍐呮牳, 瀹?鍏锋湁 宸茬粡 chosen 鍒?杩涚▼ the 鍛戒护
浣滀负 workitems 鍜?鍒?涓?executed 鍦?the handlers 鐨?the ksmbd-io kworker 绾跨▼.
瀹?allows 鐢ㄤ簬 multiplexing 鐨?the handlers 浣滀负 the 鍐呮牳 takes care 鐨?initiating
extra worker 绾跨▼ 鑻?the 鍔犺浇 鏄?increased 鍜?vice versa, 鑻?the 鍔犺浇 鏄?
decreased 瀹?destroys the extra worker 绾跨▼. 鍥犳, 涔嬪悗 the 杩炴帴 鏄?
established 涓?the client. Dedicated ksmbd/1..n(绔彛 鏁板瓧) takes complete
ownership 鐨?receiving/parsing 鐨?SMB 鍛戒护. 姣忎釜 received 鍛戒护 鏄?worked
鍦?骞惰 i.e., 閭ｉ噷 鍙?涓?澶氫釜 client 鍛戒护 鍏?鏄?worked 鍦?
骞惰. 涔嬪悗 receiving 姣忎釜 鍛戒护 涓€涓?separated 鍐呮牳 workitem 鏄?prepared
鐢ㄤ簬 姣忎釜 鍛戒护 鍏?鏄?further queued 鍒?涓?handled 鐢?ksmbd-io kworkers.
鍥犳, 姣忎釜 SMB workitem 鏄?queued 鍒?the kworkers. 姝?allows the benefit 鐨?鍔犺浇
sharing 鍒?涓?managed optimally 鐢?the 榛樿 鍐呮牳 鍜?optimizing client
鎬ц兘 鐢?handling client 鍛戒护 鍦?骞惰.

### ksmbd.mountd (鐢ㄦ埛绌洪棿 daemon)


ksmbd.mountd 鏄?涓€涓?userspace 杩涚▼ 鍒? transfer the 鐢ㄦ埛 account 鍜?password 璇?
鏄?registered 浣跨敤 ksmbd.adduser (part 鐨?utils 鐢ㄤ簬 鐢ㄦ埛绌洪棿). Further 瀹?
allows sharing information 鍙傛暟 璇?鏄?parsed 鏉ヨ嚜 smb.conf 鍒?ksmbd 鍦?
鍐呮牳. 鐢ㄤ簬 the execution part 瀹?鍏锋湁 涓€涓?daemon 鍏?鏄?continuously 杩愯涓?
鍜?connected 鍒?the 鍐呮牳 鎺ュ彛 浣跨敤 netlink 濂楁帴瀛? 瀹?waits 鐢ㄤ簬 the
requests (dcerpc 鍜?share/鐢ㄦ埛 info). 瀹?handles RPC calls (鍦?涓€涓?鏈€灏?灏戦噺
dozen) 璇?鏄?澶у鏁?閲嶈 鐢ㄤ簬 鏂囦欢 server 鏉ヨ嚜 NetShareEnum 鍜?
NetServerGetInfo. Complete DCE/RPC 鍝嶅簲 鏄?prepared 鏉ヨ嚜 the 鐢ㄦ埛绌洪棿
鍜?passed 鍦ㄢ€︿笂 鍒?the associated 鍐呮牳 绾跨▼ 鐢ㄤ簬 the client.


## KSMBD 鐗规€?鐘舵€?


============================== =================================================
鐗规€?name                   鐘舵€?
============================== =================================================
Dialects                       鍙楁敮鎸? SMB2.1 SMB3.0, SMB3.1.1 dialects
                               (intentionally excludes 瀹夊叏 vulnerable SMB1
                               dialect).
Auto Negotiation               鍙楁敮鎸?
Compound 璇锋眰               鍙楁敮鎸?
Oplock 缂撳瓨 Mechanism         鍙楁敮鎸?
SMB2 leases(v1 lease)          鍙楁敮鎸?
Directory leases(v2 lease)     鍙楁敮鎸?
Multi-credits                  鍙楁敮鎸?
NTLM/NTLMv2                    鍙楁敮鎸?
HMAC-SHA256 Signing            鍙楁敮鎸?
Secure negotiate               鍙楁敮鎸?
Signing 鏇存柊                 鍙楁敮鎸?
Pre-authentication integrity   鍙楁敮鎸?
SMB3 encryption(CCM, GCM)      鍙楁敮鎸? (CCM/GCM128 鍜?CCM/GCM256 鍙楁敮鎸?
SMB direct(RDMA)               鍙楁敮鎸?
SMB3 Multi-channel             Partially 鍙楁敮鎸? Planned 鍒?implement
                               replay/retry mechanisms 鐢ㄤ簬 future.
Receive Side Scaling 妯″紡      鍙楁敮鎸?
SMB3.1.1 POSIX extension       鍙楁敮鎸?
ACLs                           Partially 鍙楁敮鎸? 浠?DACLs 鍙敤, SACLs
                               (auditing) 鏄?planned 鐢ㄤ簬 the future. 鐢ㄤ簬
                               ownership (SIDs) ksmbd generates random subauth
                               鍊?鐒跺悗 store 瀹?鍒?disk) 鍜?浣跨敤 uid/gid
                               get 鏉ヨ嚜 inode 浣滀负 RID 鐢ㄤ簬 鏈湴 domain SID.
                               The 鐢垫祦 acl implementation 鏄?limited 鍒?
                               standalone server, 涓?涓€涓?domain member.
                               Integration 涓?Samba tools 鏄?姝ｅ湪 worked 鍦?
                               鍒?鍏佽 future 鏀寔 鐢ㄤ簬 杩愯涓?浣滀负 涓€涓?domain
                               member.
Kerberos                       鍙楁敮鎸?
Durable handle v1,v2           Planned 鐢ㄤ簬 future.
Persistent handle              Planned 鐢ㄤ簬 future.
SMB2 notify                    Planned 鐢ㄤ簬 future.
Sparse 鏂囦欢 鏀寔            鍙楁敮鎸?
DCE/RPC 鏀寔                Partially 鍙楁敮鎸? 涓€涓?灏戦噺 calls(NetShareEnumAll,
                               NetServerGetInfo, SAMR, LSARPC) 璇?鏄?needed
                               鐢ㄤ簬 鏂囦欢 server handled 閫氳繃 netlink 鎺ュ彛
                               鏉ヨ嚜 ksmbd.mountd. 棰濆 integration 涓?
                               Samba tools 鍜?搴?閫氳繃 upcall 鏄?姝ｅ湪
                               investigated 鍒?鍏佽 鏀寔 鐢ㄤ簬 棰濆
                               DCE/RPC 绠＄悊 calls (鍜?future 鏀寔
                               鐢ㄤ簬 Witness 鍗忚 e.g.)
ksmbd/nfsd interoperability    Planned 鐢ㄤ簬 future. The 鐗规€?璇?ksmbd
                               鏀寔 鏄?Leases, Notify, ACLs 鍜?Share modes.
SMB3.1.1 Compression           Planned 鐢ㄤ簬 future.
SMB3.1.1 鍦ㄢ€︿笂 QUIC             Planned 鐢ㄤ簬 future.
Signing/Encryption 鍦ㄢ€︿笂 RDMA   Planned 鐢ㄤ簬 future.
SMB3.1.1 GMAC signing 鏀寔  Planned 鐢ㄤ簬 future.
============================== =================================================


## 濡備綍 鍒?杩愯


1. Download ksmbd-tools(https://github.com/cifsd-team/ksmbd-tools/releases) 鍜?
   compile them.

   - 鍙傝€?鍒?README(https://github.com/cifsd-team/ksmbd-tools/blob/master/README.md)
     鍒?know 濡備綍 鍒?浣跨敤 ksmbd.mountd/adduser/addshare/control utils

     $ ./autogen.sh
     $ ./configure --with-rundir=/杩愯
     $ make && sudo make install

2. 鍒涘缓 /usr/鏈湴/绛?ksmbd/ksmbd.conf 鏂囦欢, add SMB share 鍦?ksmbd.conf 鏂囦欢.

   - 鍙傝€?鍒?ksmbd.conf.绀轰緥 鍦?ksmbd-utils, 鍙傝 ksmbd.conf manpage
     鐢ㄤ簬 details 鍒?configure shares.

        $ man ksmbd.conf

3. 鍒涘缓 鐢ㄦ埛/password 鐢ㄤ簬 SMB share.

   - 鍙傝 ksmbd.adduser manpage.

     $ man ksmbd.adduser
     $ sudo ksmbd.adduser -涓€涓?<Enter USERNAME 鐢ㄤ簬 SMB share access>

4. Insert the ksmbd.ko 妯″潡 涔嬪悗 鎮?build 鎮ㄧ殑 鍐呮牳. 鏃?闇€瑕?鍒?鍔犺浇 the 妯″潡
   鑻?ksmbd 鏄?built 杩涘叆 the 鍐呮牳.

   - Set ksmbd 鍦?menuconfig(e.g. $ make menuconfig)
       [*] 缃戠粶 鏂囦欢 绯荤粺  --->
           <M> SMB3 server 鏀寔 (EXPERIMENTAL)

	$ sudo modprobe ksmbd.ko

5. 鍚姩 ksmbd 鐢ㄦ埛绌洪棿 daemon

	$ sudo ksmbd.mountd

6. Access share 鏉ヨ嚜 Windows 鎴?Linux 浣跨敤 SMB3 client (cifs.ko 鎴?smbclient 鐨?samba)

## Shutdown KSMBD


1. kill 鐢ㄦ埛 鍜?鍐呮牳绌洪棿 daemon
	# sudo ksmbd.control -s

## 濡備綍 鍒?turn debug print 鍦?


姣忎釜 layer
/sys/绫?ksmbd-control/debug

1. 鍚敤 鍏ㄩ儴 component prints
	# sudo ksmbd.control -d "鍏ㄩ儴"

2. 鍚敤 one 鐨?the components (smb, auth, vfs, oplock, ipc, conn, rdma)
	# sudo ksmbd.control -d "smb"

3. 鏄剧ず 浠€涔?prints 鏄?宸插惎鐢?
	# cat /sys/绫?ksmbd-control/debug
	  [smb] auth vfs oplock ipc conn [rdma]

4. 绂佺敤 prints:
	鑻?鎮?try the selected component 涓€鏃?鏇村, 瀹冩槸 宸茬鐢?鏃?brackets.
