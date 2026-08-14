## 缺陷追踪（Bug Hunting）


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
此类栈跟踪提供了足够的信息来定位内核源代码中发生缺陷的那一行。根据问题的严重程度，

```

	BUG: unable to handle kernel NULL pointer dereference at   (null)
	IP: [<c06969d4>] iret_exc+0x7d0/0xa59
	*pdpt = 000000002258a001 *pde = 0000000000000000
	Oops: 0002 [#1] PREEMPT SMP
	...

```
尽管它是一个 **Oops** 或某种其他类型的栈跟踪，通常仍需要找到出错的那一行才能定位和处理缺陷。在本章中，我们用“Oops”来泛指所有需要分析的各类栈跟踪。

如果内核使用 `CONFIG_DEBUG_INFO` 编译，你可以通过 `scripts/decode_stacktrace.sh` 脚本来提升栈跟踪的质量。

### 已加载的模块


已被污染或正在加载/卸载的模块会以“(...)”标记，其中污染标志在
`Documentation/admin-guide/tainted-kernels.rst` 中描述；“正在加载”用“+”标注，“正在卸载”用“-”标注。


### Oops 消息位于何处？


通常 Oops 文本由 `klogd` 从内核缓冲区读出，并交给 `syslogd` 写入一个 syslog 文件，通常是
`/var/log/messages`（取决于 `/etc/syslog.conf`）。在使用 systemd 的系统上，它也可能由
`journald` 守护进程保存，并通过运行 `journalctl` 命令来访问。

有时 `klogd` 会死掉，这时你可以运行 `dmesg > file` 从内核缓冲区读出数据并保存。或者你可以
`cat /proc/kmsg > file`，不过你必须中断传输来停止它，因为 `kmsg` 是一个“永不结束的文件”。

如果机器崩溃得如此严重，以至于你无法输入命令或磁盘不可用，那么你有三种选择：

(1) 从屏幕手写抄录文本，并在机器重启后键入。这很麻烦，但如果你没有为崩溃做准备，这是唯一的办法。或者，你可以用数码相机拍下屏幕——虽然不好，但总比没有强。如果消息滚出控制台顶部，你可能会发现用更高的分辨率引导（例如 `vga=791`）可以让你读到更多文本。（注意：这需要 `vesafb`，因此对“早期”的 Oops 没有帮助。）

(2) 用串口控制台引导（参见 `Documentation/admin-guide/serial-console.rst` <serial_console>），通过零调制解调器电缆连接到另一台机器，并用你喜欢的通信程序在那里捕获输出。Minicom 效果很好。

(3) 使用 Kdump（参见 `Documentation/admin-guide/kdump/kdump.rst`），通过 `Documentation/admin-guide/kdump/gdbmacros.txt` 中的 dmesg gdb 宏从旧内存中提取内核环形缓冲区。

### 定位缺陷位置


如果你能将缺陷的位置指向内核源文件，报告缺陷的效果最好。有两种方法可以做到这一点。通常，使用
`gdb` 更简单，但内核应预先使用调试信息编译。

##### gdb


GNU 调试器（`gdb`）是确定 OOPS 在 `vmlinux` 文件中精确文件与行号的最佳方式。

在带有 `CONFIG_DEBUG_INFO` 编译的内核上，`gdb` 的使用效果最佳。

```

  $ ./scripts/config -d COMPILE_TEST -e DEBUG_KERNEL -e DEBUG_INFO

```
在带有 `CONFIG_DEBUG_INFO` 编译的内核上，你只需复制

```

 EIP:    0060:[<c021e50e>]    Not tainted VLI

```

```

  $ gdb vmlinux
  (gdb) l *0xc021e50e

```
如果你没有启用 `CONFIG_DEBUG_INFO`，你可以使用函数

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
这表明问题很可能出在 `:jbd:` 模块中。你可以加载那个模块

```

  $ gdb fs/jbd/jbd.ko
  (gdb) l *log_wait_commit+0xa3

```

     你也可以对栈跟踪中的任何函数调用做同样的事情，

```

	 [<f80bc9ca>] ? dvb_usb_adapter_frontend_exit+0x3a/0x70 [dvb_usb]

     上述调用发生的位置可以通过以下方式查看：

	$ gdb drivers/media/usb/dvb-usb/dvb-usb.o
	(gdb) l *dvb_usb_adapter_frontend_exit+0x3a

```
##### objdump


要调试内核，可以使用 objdump 并查找崩溃输出中的十六进制偏移，以找到有效的代码行/汇编行。在没有调试符号的情况下，你会看到该例程显示的汇编代码，但如果你的内核带有调试符号，C 代码也会可用。（调试符号可以通过

```

    $ objdump -r -S -l --disassemble net/ipv4/tcp.o

```

   你需要位于内核树的最顶层，这样它才能找到你的 C 文件。

如果你无法访问源代码，仍然可以使用以下方法调试一些崩溃转储（示例崩溃转储输出如

```

     EIP is at 	+0x14/0x4c0
      ...
     Code: 44 24 04 e8 6f 05 00 00 e9 e8 fe ff ff 8d 76 00 8d bc 27 00 00
     00 00 55 57  56 53 81 ec bc 00 00 00 8b ac 24 d0 00 00 00 8b 5d 08
     <8b> 83 3c 01 00 00 89 44  24 14 8b 45 28 85 c0 89 44 24 18 0f 85

     将字节放入一个“foo.s”文件中，如下所示：

            .text
            .globl foo
     foo:
            .byte  .... /* bytes from Code: part of OOPS dump */

     用 "gcc -c -o foo.o foo.s" 编译它，然后查看 "objdump --disassemble foo.o" 的输出。

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
`scripts/decodecode` 可用于自动化其中大部分工作，具体取决于正在调试的 CPU 架构。

### 报告缺陷


一旦通过检查位置确定了缺陷发生的位置，你既可以选择自己尝试修复它，也可以将其报告给上游。

为了将其报告给上游，你应该确定受影响代码的缺陷跟踪器（如果有的话）或邮件列表。这可以通过使用
`get_maintainer.pl` 脚本来完成。

例如，如果你在 gspca 的 sonixj.c 文件中发现了一个缺陷，你可以得到

```

	$ ./scripts/get_maintainer.pl --bug -f drivers/media/usb/gspca/sonixj.c
	Hans Verkuil <hverkuil@kernel.org> (odd fixer:GSPCA USB WEBCAM DRIVER,commit_signer:1/1=100%)
	Mauro Carvalho Chehab <mchehab@kernel.org> (maintainer:MEDIA INPUT INFRASTRUCTURE (V4L/DVB),commit_signer:1/1=100%)
	Tejun Heo <tj@kernel.org> (commit_signer:1/1=100%)
	Bhaktipriya Shridhar <bhaktipriya96@gmail.com> (commit_signer:1/1=100%,authored:1/1=100%,added_lines:4/4=100%,removed_lines:9/9=100%)
	linux-media@vger.kernel.org (open list:GSPCA USB WEBCAM DRIVER)
	linux-kernel@vger.kernel.org (open list)

```
请注意，它会指向：

- 最后修改过源代码的开发者（如果这是在 git 树内完成的）。在上面例子中是 Tejun 和 Bhaktipriya（在这个具体情况下，他们都没有真正参与该文件的开发）；
- 驱动维护者（Hans Verkuil）；
- 子系统维护者（Mauro Carvalho Chehab）；
- 驱动和/或子系统邮件列表（linux-media@vger.kernel.org）；
- Linux 内核邮件列表（linux-kernel@vger.kernel.org）；
- 驱动/子系统的缺陷报告 URI（上例中为空）。

如果列表中末尾包含缺陷报告 URI，请优先使用它们而不是电子邮件。否则，请将缺陷报告给用于该代码开发的邮件列表（linux-media ML），并抄送给驱动维护者（Hans）。

如果你完全不知道该把报告发给谁，并且 `get_maintainer.pl` 也没有提供任何有用的信息，请将其发送到
linux-kernel@vger.kernel.org。

感谢你为让 Linux 尽可能稳定所提供的帮助。

### 修复缺陷


如果你懂编程，你可以通过不仅报告缺陷、还提供解决方案的方式来帮助我们。毕竟，开源的意义在于分享你所做的，你难道不想因自己的才华而获得认可吗？

如果你决定走这条路，一旦你想出了修复方案，请将其提交给上游。

不过，请务必阅读
`Documentation/process/submitting-patches.rst` <submittingpatches>，以帮助你提交的代码被接受。


---------------------------------------------------------------------------

### 关于使用 ``klogd`` 进行 Oops 追踪的说明


为了帮助 Linus 和其他内核开发者，`klogd` 中加入了大量用于处保护故障的支持。为了获得对地址解析的
完整支持，至少应使用 `sysklogd` 软件包 1.3-pl3 版本。

当发生保护故障时，`klogd` 守护进程会自动将内核日志消息中的重要地址转换为它们的符号等价形式。这个
转换后的内核消息随后通过 `klogd` 正在使用的任何报告机制转发。保护故障消息可以简单地从消息文件中
截取并转发给内核开发者。

`klogd` 执行两种类型的地址解析。第一种是静态转换，第二种是动态转换。静态转换使用 System.map 文件。
为了进行静态转换，`klogd` 守护进程必须能够在守护进程初始化时找到一个系统映射文件。有关 `klogd` 如何
搜索映射文件的信息，请参阅 klogd 手册页。

当使用内核可加载模块时，动态地址解析很重要。由于内核模块的内存是从内核的动态内存池中分配的，因此
无论是模块的起始位置还是模块中的函数和符号都没有固定的位置。

内核支持一些系统调用，允许程序确定加载了哪些模块以及它们在内存中的位置。通过对这些系统调用的使用，
`klogd` 守护进程构建了一张符号表，可用于调试可加载内核模块中发生的保护故障。

至少，`klogd` 会提供生成保护故障的模块名称。如果可加载模块的开发者选择从模块中导出符号信息，可能还会有
额外的符号信息可用。

由于内核模块环境是动态的，因此必须有一种机制能在模块环境发生变化时通知 `klogd` 守护进程。有一些命令行
选项可供使用，它们允许 klogd 向当前正在执行的守护进程发信号，表示应该刷新符号信息。有关更多信息，请参阅
`klogd` 手册页。

sysklogd 发行版中包含一个补丁，它修改 `modules-2.0.0` 软件包，使其在每次加载或卸载模块时自动向 klogd
发信号。应用此补丁基本上可以为调试内核可加载模块中发生的保护故障提供无缝支持。

以下是可加载模块中保护故障的一个示例

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
