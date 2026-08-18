
## 閫氳繃 gdb 璋冭瘯鍐呮牳涓庢ā鍧?

鍐呮牳璋冭瘯鍣?kgdb銆佸儚 QEMU 杩欐牱鐨?hypervisor 鎴栧熀浜?JTAG 鐨勭‖浠舵帴鍙ｏ紝鍏佽鍦ㄨ繍琛屾椂浣跨敤 gdb 璋冭瘯 Linux 鍐呮牳鍙婂叾妯″潡銆侴db 甯︽湁涓€涓己澶х殑 python 鑴氭湰鎺ュ彛銆傚唴鏍告彁渚涗簡涓€缁勮緟鍔╄剼鏈紝鍙互绠€鍖栧吀鍨嬬殑鍐呮牳璋冭瘯姝ラ銆傝繖鏄竴涓叧浜庡浣曞惎鐢ㄥ拰浣跨敤瀹冧滑鐨勭畝鐭暀绋嬨€傚畠鑱氱劍浜庝互 QEMU/KVM 铏氭嫙鏈轰綔涓虹洰鏍囷紝浣嗙ず渚嬩篃鍙互绉绘鍒板叾浠?gdb stub 涓娿€?
### 瑕佹眰


- gdb 7.2+锛堟帹鑽愶細7.4+锛夛紝鍚敤浜?python 鏀寔锛堝浜庡彂琛岀増閫氬父涓虹湡锛?
### 璁剧疆


- 涓?QEMU/KVM 鍒涘缓涓€涓櫄鎷?Linux 鏈哄櫒锛堟洿澶氱粏鑺傝 www.linux-kvm.org 鍜?www.qemu.org锛夈€傚浜庝氦鍙夊紑鍙戯紝https://landley.net/aboriginal/bin 淇濈暀浜嗕竴鎵规満鍣ㄩ暅鍍忓拰宸ュ叿閾撅紝鍙互浣滀负璧风偣浣跨敤銆?
- 鏋勫缓鍐呮牳鏃跺惎鐢?CONFIG_GDB_SCRIPTS锛屼絾淇濇寔 CONFIG_DEBUG_INFO_REDUCED 鍏抽棴銆傚鏋滀綘鐨勬灦鏋勬敮鎸?CONFIG_FRAME_POINTER锛岃淇濇寔鍚敤銆?
- 灏嗚鍐呮牳瀹夎鍒板鎴锋満涓婏紝濡傛湁蹇呰锛岄€氳繃鍦ㄥ唴鏍稿懡浠よ娣诲姞 "nokaslr" 鍏抽棴 KASLR銆?  鎴栬€咃紝QEMU 鍏佽浣跨敤 -kernel銆?append銆?initrd 鍛戒护琛屽紑鍏崇洿鎺ュ惎鍔ㄥ唴鏍搞€傚鏋滀綘涓嶄緷璧栨ā鍧楋紝杩欓€氬父鎵嶆湁鐢ㄣ€傚叧浜庢妯″紡鐨勬洿澶氱粏鑺傝鍙傞槄 QEMU 鏂囨。銆傚湪杩欑鎯呭喌涓嬶紝濡傛灉浣犵殑鏋舵瀯鏀寔 KASLR锛屼綘搴旇鏋勫缓鏃剁鐢?CONFIG_RANDOMIZE_BASE銆?
```

    make scripts_gdb

```
- 鍚敤 QEMU/KVM 鐨?gdb stub锛屽彲浠ワ細

    - 鍦?VM 鍚姩鏃堕€氳繃鍚?QEMU 鍛戒护琛岃拷鍔?"-s"

  鎴?
    - 鍦ㄨ繍琛屾椂閫氳繃 QEMU monitor 鎺у埗鍙板彂鍑?"gdbserver"

- cd /path/to/linux-build

- 鍚姩 gdb锛歡db vmlinux

  娉ㄦ剰锛氭煇浜涘彂琛岀増鍙兘闄愬埗 gdb 鑴氭湰鑷姩鍔犺浇鍒板凡鐭ュ畨鍏ㄨ矾寰?```

    add-auto-load-safe-path /path/to/linux-build

  鍒?~/.gdbinit銆傛洿澶氱粏鑺傝鍙傞槄 gdb 甯姪銆?
```
```

    (gdb) target remote :1234


```
### 浣跨敤 Linux 鎻愪緵鐨?gdb 杈呭姪鍑芥暟鐨勭ず渚?

```

    (gdb) lx-symbols
    loading vmlinux
    scanning for modules in /home/user/linux/build
    loading @0xffffffffa0020000: /home/user/linux/build/net/netfilter/xt_tcpudp.ko
    loading @0xffffffffa0016000: /home/user/linux/build/net/netfilter/xt_pkttype.ko
    loading @0xffffffffa0002000: /home/user/linux/build/net/netfilter/xt_limit.ko
    loading @0xffffffffa00ca000: /home/user/linux/build/net/packet/af_packet.ko
    loading @0xffffffffa003c000: /home/user/linux/build/fs/fuse/fuse.ko
    ...
    loading @0xffffffffa0000000: /home/user/linux/build/drivers/ata/ata_generic.ko

```
```

    (gdb) b btrfs_init_sysfs
    Function "btrfs_init_sysfs" not defined.
    Make breakpoint pending on future shared library load? (y or [n]) y
    Breakpoint 1 (btrfs_init_sysfs) pending.

```
```

    (gdb) c

```
- 鍦ㄧ洰鏍囦笂鍔犺浇妯″潡锛屽苟瑙傚療绗﹀彿琚姞杞斤紝浠ュ強
```

    loading @0xffffffffa0034000: /home/user/linux/build/lib/libcrc32c.ko
    loading @0xffffffffa0050000: /home/user/linux/build/lib/lzo/lzo_compress.ko
    loading @0xffffffffa006e000: /home/user/linux/build/lib/zlib_deflate/zlib_deflate.ko
    loading @0xffffffffa01b1000: /home/user/linux/build/fs/btrfs/btrfs.ko

    Breakpoint 1, btrfs_init_sysfs () at /home/user/linux/fs/btrfs/sysfs.c:36
    36              btrfs_kset = kset_create_and_add("btrfs", NULL, fs_kobj);

```
```

    (gdb) lx-dmesg
    [     0.000000] Initializing cgroup subsys cpuset
    [     0.000000] Initializing cgroup subsys cpu
    [     0.000000] Linux version 3.8.0-rc4-dbg+ (...
    [     0.000000] Command line: root=/dev/sda2 resume=/dev/sda1 vga=0x314
    [     0.000000] e820: BIOS-provided physical RAM map:
    [     0.000000] BIOS-e820: [mem 0x0000000000000000-0x000000000009fbff] usable
    [     0.000000] BIOS-e820: [mem 0x000000000009fc00-0x000000000009ffff] reserved
    ....

```
```

    (gdb) p $lx_current().pid
    $1 = 4998
    (gdb) p $lx_current().comm
    $2 = "modprobe\000\000\000\000\000\000\000"

```
```

    (gdb) p $lx_per_cpu(runqueues).nr_running
    $3 = 1
    (gdb) p $lx_per_cpu(runqueues, 2).nr_running
    $4 = 0

```
```

    (gdb) set $leftmost = $lx_per_cpu(hrtimer_bases).clock_base[0].active.rb_root.rb_leftmost
    (gdb) p *$container_of($leftmost, "struct hrtimer", "node")
    $5 = {
      node = {
        node = {
          __rb_parent_color = 18446612686384860673,
          rb_right = 0xffff888231da8b00,
          rb_left = 0x0
        },
        expires = 1228461000000
      },
      _softexpires = 1228461000000,
      function = 0xffffffff8137ab20 <tick_nohz_handler>,
      base = 0xffff888231d9b4c0,
      state = 1 '\001',
      is_rel = 0 '\000',
      is_soft = 0 '\000',
      is_hard = 1 '\001'
    }


```
### 鍛戒护涓庡嚱鏁板垪琛?

鍛戒护鍜屼究鎹峰嚱鏁扮殑鏁伴噺鍙兘浼氶殢鏃堕棿婕斿彉锛?```

 (gdb) apropos lx
 function lx_current -- Return current task
 function lx_module -- Find module by name and return the module variable
 function lx_per_cpu -- Return per-cpu variable
 function lx_task_by_pid -- Find Linux task by PID and return the task_struct variable
 function lx_thread_info -- Calculate Linux thread_info from task variable
 lx-dmesg -- Print Linux kernel log buffer
 lx-lsmod -- List currently loaded modules
 lx-symbols -- (Re-)load symbols of Linux kernel and currently loaded modules

```
鍙互閫氳繃 "help <鍛戒护鍚?" 鑾峰彇鍛戒护鐨勮缁嗗府鍔╋紝閫氳繃 "help function <鍑芥暟鍚?" 鑾峰彇渚挎嵎鍑芥暟鐨勮缁嗗府鍔┿€?
### 璋冭瘯 GDB 鑴氭湰


GDB 榛樿涓嶅惎鐢ㄥ畬鏁寸殑 Python 鍥炴函锛岃繖鍙兘浣胯皟璇?GDB 鑴氭湰姣斿繀瑕佺殑鏇村洶闅俱€備互涓嬪唴瀹瑰皢鍏佽鎵撳嵃
```

 (gdb) set python print-stack full

```
