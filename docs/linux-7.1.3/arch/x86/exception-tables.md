
## 内核级异常处理


评论由 Joerg Pommnitz <joerg@raleigh.ibm.com> 撰写

当一个进程运行在内核模式时，它常常需要访问由不可信程序传入的用户态内存地址。
为了保护自身，内核必须校验该地址。

在较旧的 Linux 版本中，这是通过

int verify_area(int type, const void * addr, unsigned long size)

函数完成的（后来已被 access_ok() 取代）。

该函数校验从地址 'addr' 开始、大小为 'size' 的内存区域对于 type 中指定的操作
（读或写）是否可访问。为此，verify_read 必须查找包含地址 addr 的虚拟内存区域
（vma）。在正常情况（正确工作的程序）下，该测试会成功。它只对少数有 bug 的程序
失败。在某些内核性能剖析测试中，这个通常不必要的校验消耗了相当多的时间。

为了克服这种情况，Linus 决定让每个支持 Linux 的 CPU 中都存在的虚拟内存硬件来处理
这个测试。

这是如何工作的呢？

每当内核试图访问一个当前不可访问的地址时，CPU 会产生一个页错误异常，并调用
```
  void exc_page_fault(struct pt_regs *regs, unsigned long error_code)
```
arch/x86/mm/fault.c 中的函数。栈上的参数由 arch/x86/entry/entry_32.S 中的底层
汇编胶水代码设置。参数 regs 是指向栈上已保存寄存器的指针，error_code 包含了异常的
原因码。

exc_page_fault() 首先从 CPU 控制寄存器 CR2 中获取不可访问的地址。如果该地址位于
进程的虚拟地址空间内，那么故障很可能是由页面尚未换入、被写保护或类似情况引起的。
不过，我们感兴趣的是另一种情况：该地址无效，不存在包含此地址的 vma。在这种情况下，
内核跳转到 bad_area 标签。

在那里，它使用引发异常的指令地址（即 regs->eip）来查找可以从中继续执行（fixup）
的地址。如果查找成功，故障处理程序会修改返回地址（同样是 regs->eip）并返回。执行
将在 fixup 中的地址处继续。

fixup 指向哪里？

由于我们是跳转到 fixup 的内容，fixup 显然指向可执行的代码。这段代码隐藏在用户访问
宏内部。我选取了 arch/x86/include/asm/uaccess.h 中定义的 get_user() 宏作为例子。
其定义有些难以理解，所以让我们看一下预处理器和编译器生成的代码。我选取了
drivers/char/sysrq.c 中的 get_user() 调用进行详细分析。
```
        get_user(c, buf);
```
```
  (
    {
      long __gu_err = - 14 , __gu_val = 0;
      const __typeof__(*( (  buf ) )) *__gu_addr = ((buf));
      if (((((0 + current_set[0])->tss.segment) == 0x18 )  ||
        (((sizeof(*(buf))) <= 0xC0000000UL) &&
        ((unsigned long)(__gu_addr ) <= 0xC0000000UL - (sizeof(*(buf)))))))
        do {
          __gu_err  = 0;
          switch ((sizeof(*(buf)))) {
            case 1:
              __asm__ __volatile__(
                "1:      mov" "b" " %2,%" "b" "1\n"
                "2:\n"
                ".section .fixup,\"ax\"\n"
                "3:      movl %3,%0\n"
                "        xor" "b" " %" "b" "1,%" "b" "1\n"
                "        jmp 2b\n"
                ".section __ex_table,\"a\"\n"
                "        .align 4\n"
                "        .long 1b,3b\n"
                ".text"        : "=r"(__gu_err), "=q" (__gu_val): "m"((*(struct __large_struct *)
                              (   __gu_addr   )) ), "i"(- 14 ), "0"(  __gu_err  )) ;
                break;
            case 2:
              __asm__ __volatile__(
                "1:      mov" "w" " %2,%" "w" "1\n"
                "2:\n"
                ".section .fixup,\"ax\"\n"
                "3:      movl %3,%0\n"
                "        xor" "w" " %" "w" "1,%" "w" "1\n"
                "        jmp 2b\n"
                ".section __ex_table,\"a\"\n"
                "        .align 4\n"
                "        .long 1b,3b\n"
                ".text"        : "=r"(__gu_err), "=r" (__gu_val) : "m"((*(struct __large_struct *)
                              (   __gu_addr   )) ), "i"(- 14 ), "0"(  __gu_err  ));
                break;
            case 4:
              __asm__ __volatile__(
                "1:      mov" "l" " %2,%" "" "1\n"
                "2:\n"
                ".section .fixup,\"ax\"\n"
                "3:      movl %3,%0\n"
                "        xor" "l" " %" "" "1,%" "" "1\n"
                "        jmp 2b\n"
                ".section __ex_table,\"a\"\n"
                "        .align 4\n"        "        .long 1b,3b\n"
                ".text"        : "=r"(__gu_err), "=r" (__gu_val) : "m"((*(struct __large_struct *)
                              (   __gu_addr   )) ), "i"(- 14 ), "0"(__gu_err));
                break;
            default:
              (__gu_val) = __get_user_bad();
          }
        } while (0) ;
      ((c)) = (__typeof__(*((buf))))__gu_val;
      __gu_err;
    }
  );
```
真厉害！晦涩的 GCC/汇编魔法。这根本没法读懂，所以我们来看
```
 >         xorl %edx,%edx
 >         movl current_set,%eax
 >         cmpl $24,788(%eax)
 >         je .L1424
 >         cmpl $-1073741825,64(%esp)
 >         ja .L1423
 > .L1424:
 >         movl %edx,%eax
 >         movl 64(%esp),%ebx
 > #APP
 > 1:      movb (%ebx),%dl                /* this is the actual user access */
 > 2:
 > .section .fixup,"ax"
 > 3:      movl $-14,%eax
 >         xorb %dl,%dl
 >         jmp 2b
 > .section __ex_table,"a"
 >         .align 4
 >         .long 1b,3b
 > .text
 > #NO_APP
 > .L1423:
 >         movzbl %dl,%esi
```
优化器做得很好，给了我们一些实际能看懂的东西。是吗？实际的用户访问非常明显。多亏
统一的地址空间，我们可以直接访问用户内存中的地址。但那些 .section 的东西是干嘛的？？？？
```
 > objdump --section-headers vmlinux
 >
 > vmlinux:     file format elf32-i386
 >
 > Sections:
 > Idx Name          Size      VMA       LMA       File off  Algn
 >   0 .text         00098f40  c0100000  c0100000  00001000  2**4
 >                   CONTENTS, ALLOC, LOAD, READONLY, CODE
 >   1 .fixup        000016bc  c0198f40  c0198f40  00099f40  2**0
 >                   CONTENTS, ALLOC, LOAD, READONLY, CODE
 >   2 .rodata       0000f127  c019a5fc  c019a5fc  0009b5fc  2**2
 >                   CONTENTS, ALLOC, LOAD, READONLY, DATA
 >   3 __ex_table    000015c0  c01a9724  c01a9724  000aa724  2**2
 >                   CONTENTS, ALLOC, LOAD, READONLY, DATA
 >   4 .data         0000ea58  c01abcf0  c01abcf0  000abcf0  2**4
 >                   CONTENTS, ALLOC, LOAD, DATA
 >   5 .bss          00018e21  c01ba748  c01ba748  000ba748  2**2
 >                   ALLOC
 >   6 .comment      00000ec4  00000000  00000000  000ba748  2**0
 >                   CONTENTS, READONLY
 >   7 .note         00001068  00000ec4  00000ec4  000bb60c  2**0
 >                   CONTENTS, READONLY
```
显然，生成的 obj 文件中有 2 个非标准的 ELF 节。但首先我们想弄清楚我们的代码在
```
 > objdump --disassemble --section=.text vmlinux
 >
 > c017e785 <do_con_write+c1> xorl   %edx,%edx
 > c017e787 <do_con_write+c3> movl   0xc01c7bec,%eax
 > c017e78c <do_con_write+c8> cmpl   $0x18,0x314(%eax)
 > c017e793 <do_con_write+cf> je     c017e79f <do_con_write+db>
 > c017e795 <do_con_write+d1> cmpl   $0xbfffffff,0x40(%esp,1)
 > c017e79d <do_con_write+d9> ja     c017e7a7 <do_con_write+e3>
 > c017e79f <do_con_write+db> movl   %edx,%eax
 > c017e7a1 <do_con_write+dd> movl   0x40(%esp,1),%ebx
 > c017e7a5 <do_con_write+e1> movb   (%ebx),%dl
 > c017e7a7 <do_con_write+e3> movzbl %dl,%esi
```
整个用户内存访问被缩减为 10 条 x86 机器指令。被 .section 指令括起来的指令不再处于
正常的执行路径中。它们位于一个不同的节里
```
 > objdump --disassemble --section=.fixup vmlinux
 >
 > c0199ff5 <.fixup+10b5> movl   $0xfffffff2,%eax
 > c0199ffa <.fixup+10ba> xorb   %dl,%dl
 > c0199ffc <.fixup+10bc> jmp    c017e7a7 <do_con_write+e3>
```
```
 > objdump --full-contents --section=__ex_table vmlinux
 >
 >  c01aa7c4 93c017c0 e09f19c0 97c017c0 99c017c0  ................
 >  c01aa7d4 f6c217c0 e99f19c0 a5e717c0 f59f19c0  ................
 >  c01aa7e4 080a18c0 01a019c0 0a0a18c0 04a019c0  ................
```
```
 >  c01aa7c4 c017c093 c0199fe0 c017c097 c017c099  ................
 >  c01aa7d4 c017c2f6 c0199fe9 c017e7a5 c0199ff5  ................
                               ^^^^^^^^^^^^^^^^^
                               this is the interesting part!
 >  c01aa7e4 c0180a08 c019a001 c0180a0a c019a004  ................
```
```
  .section .fixup,"ax"
  .section __ex_table,"a"
```
告诉汇编器将后面的代码移动到指定的
```
  3:      movl $-14,%eax
          xorb %dl,%dl
          jmp 2b
```
```
        .long 1b,3b
```
最终出现在目标文件的 __ex_table 节中。1b 和 3b 是局部标签。局部标签 1b（1b 表示
向后最近的标签 1）是可能出错指令的地址，即在我们的情况中，标签 1 的地址为
c017e7a5：
原始汇编代码： > 1:      movb (%ebx),%dl
链接进 vmlinux 后： > c017e7a5 <do_con_write+e1> movb   (%ebx),%dl

局部标签 3（同样向后）是处理故障的代码的地址，在我们的情况中实际值为 c0199ff5：
原始汇编代码： > 3:      movl $-14,%eax
链接进 vmlinux 后： > c0199ff5 <.fixup+10b5> movl   $0xfffffff2,%eax

如果 fixup 能够处理该异常，控制流可以返回到触发故障的那条指令之后的指令，即局部
标签 2b。
```
 > .section __ex_table,"a"
 >         .align 4
 >         .long 1b,3b
```
```
 >  c01aa7d4 c017c2f6 c0199fe9 c017e7a5 c0199ff5  ................
                               ^this is ^this is
                               1b       3b
```
c017e7a5，c0199ff5 位于内核的异常表中。

那么，如果发生了来自内核模式、且没有合适 vma 的故障，实际会发生什么呢？
```
    > c017e7a5 <do_con_write+e1> movb   (%ebx),%dl
```
#. MMU 产生异常
#. CPU 调用 exc_page_fault()
#. exc_page_fault() 调用 do_user_addr_fault()
#. do_user_addr_fault() 调用 kernelmode_fixup_or_oops()
#. kernelmode_fixup_or_oops() 调用 fixup_exception()（regs->eip == c017e7a5）；
#. fixup_exception() 调用 search_exception_tables()
#. search_exception_tables() 在异常表中查找地址 c017e7a5（即 ELF 节 __ex_table
   的内容），并返回相关联的故障处理代码的地址 c0199ff5。
#. fixup_exception() 修改自身的返回地址以指向故障处理代码并返回。
#. 执行在故障处理代码中继续。
#. a) EAX 变为 -EFAULT（== -14）
   b) DL 变为零（我们从用户空间“读取”的值）
   c) 执行在局部标签 2 处继续（即紧接在引发故障的用户访问指令之后的指令的地址）。

   上面 a 到 c 的步骤在某种意义上模拟了那条出错的指令。

大体就是这样了。如果你看我们的例子，可能会问为什么我们在异常处理代码中把 EAX 设
为 -EFAULT。嗯，get_user() 宏实际上返回一个值：用户访问成功时为 0，失败时
为 -EFAULT。我们原来的代码没有测试这个返回值，但 get_user() 中的内联汇编代码
尝试返回 -EFAULT。GCC 选择了 EAX 来返回这个值。

注意：
由于异常表的构建方式以及需要保持有序，只对 .text 节中的代码使用异常。任何其它
节都会导致异常表无法被正确排序，从而使异常处理失败。

当 64 位支持被加入 x86 Linux 时，情况发生了变化。与其通过将两个条目从 32 位扩展
到 64 位来使异常表大小翻倍，不如使用了一个巧妙的技巧：将地址存储为相对于表本身的
偏移量。汇编代码从
```
    .long 1b,3b
  to:
          .long (from) - .
          .long (to) - .
```
改为了上面的形式，而使用这些值的 C 代码将其转换回绝对地址
```
	ex_insn_addr(const struct exception_table_entry *x)
	{
		return (unsigned long)&x->insn + x->insn;
	}
```
在 v4.6 中，异常表条目被扩展了一个新的字段 "handler"。它同样是 32 位宽，包含一个
第三个相对函数指针，指向以下之一：

1) `int ex_handler_default(const struct exception_table_entry *fixup)`
     这是传统的情形，只是跳转到 fixup 代码

2) `int ex_handler_fault(const struct exception_table_entry *fixup)`
     这种情形提供在 entry->insn 处发生的陷阱的故障号。它用于区分页错误与机器检查。

可以很容易地添加更多函数。

CONFIG_BUILDTIME_TABLE_SORT 允许通过主机工具 scripts/sorttable 在内核镜像链接之后
对 __ex_table 节进行排序。它会将符号 main_extable_sort_needed 设为 0，从而避免
在启动时对 __ex_table 节进行排序。有了排序后的异常表，在运行时发生异常时，我们
可以通过二分查找快速定位 __ex_table 条目。

这不仅仅是一个启动时的优化，某些架构要求该表是有序的，以便在启动过程中相当早的
阶段就能处理异常。例如，i386 甚至在分页支持尚未启用之前就使用了这种形式的异常
处理！
