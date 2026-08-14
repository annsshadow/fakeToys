
## 通过 gdb 调试内核与模块


内核调试器 kgdb、像 QEMU 这样的 hypervisor 或基于 JTAG 的硬件接口，允许在运行时使用 gdb 调试 Linux 内核及其模块。Gdb 带有一个强大的 python 脚本接口。内核提供了一组辅助脚本，可以简化典型的内核调试步骤。这是一个关于如何启用和使用它们的简短教程。它聚焦于以 QEMU/KVM 虚拟机作为目标，但示例也可以移植到其他 gdb stub 上。

### 要求


- gdb 7.2+（推荐：7.4+），启用了 python 支持（对于发行版通常为真）

### 设置


- 为 QEMU/KVM 创建一个虚拟 Linux 机器（更多细节见 www.linux-kvm.org 和 www.qemu.org）。对于交叉开发，https://landley.net/aboriginal/bin 保留了一批机器镜像和工具链，可以作为起点使用。

- 构建内核时启用 CONFIG_GDB_SCRIPTS，但保持 CONFIG_DEBUG_INFO_REDUCED 关闭。如果你的架构支持 CONFIG_FRAME_POINTER，请保持启用。

- 将该内核安装到客户机上，如有必要，通过在内核命令行添加 "nokaslr" 关闭 KASLR。
  或者，QEMU 允许使用 -kernel、-append、-initrd 命令行开关直接启动内核。如果你不依赖模块，这通常才有用。关于此模式的更多细节请参阅 QEMU 文档。在这种情况下，如果你的架构支持 KASLR，你应该构建时禁用 CONFIG_RANDOMIZE_BASE。

```

    make scripts_gdb

```
- 启用 QEMU/KVM 的 gdb stub，可以：

    - 在 VM 启动时通过向 QEMU 命令行追加 "-s"

  或

    - 在运行时通过 QEMU monitor 控制台发出 "gdbserver"

- cd /path/to/linux-build

- 启动 gdb：gdb vmlinux

  注意：某些发行版可能限制 gdb 脚本自动加载到已知安全路径
```

    add-auto-load-safe-path /path/to/linux-build

  到 ~/.gdbinit。更多细节请参阅 gdb 帮助。

```
```

    (gdb) target remote :1234


```
### 使用 Linux 提供的 gdb 辅助函数的示例


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
- 在目标上加载模块，并观察符号被加载，以及
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
### 命令与函数列表


命令和便捷函数的数量可能会随时间演变，
```

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
可以通过 "help <命令名>" 获取命令的详细帮助，通过 "help function <函数名>" 获取便捷函数的详细帮助。

### 调试 GDB 脚本


GDB 默认不启用完整的 Python 回溯，这可能使调试 GDB 脚本比必要的更困难。以下内容将允许打印
```

 (gdb) set python print-stack full

```
