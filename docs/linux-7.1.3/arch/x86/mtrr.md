
## MTRR（Memory Type Range Register，内存类型范围寄存器）控制

:Authors: - Richard Gooch <rgooch@atnf.csiro.au> - 1999 年 6 月 3 日
          - Luis R. Rodriguez <mcgrof@do-not-panic.com> - 2015 年 4 月 9 日

## 逐步淘汰 MTRR 的使用

在现代 x86 硬件上，MTRR 的使用已被 PAT 取代。Linux 中驱动直接对 MTRR 的使用现已完全淘汰，设备驱动应当使用 `arch_phys_wc_add()` 配合 `ioremap_wc()`，以在非 PAT 系统上使 MTRR 生效，而在 PAT 启用系统上则为一个空操作但同样生效。

即使 Linux 不直接使用 MTRR，一些 x86 平台固件仍可能在启动操作系统之前尽早设置好 MTRR。它们这样做是因为部分平台固件可能仍然实现了对 MTRR 的访问，而这些将由平台固件直接控制和处理。平台使用 MTRR 的一个例子是通过 SMI 处理程序，一种情况可能是用于风扇控制，平台代码需要对其部分风扇控制寄存器进行不可缓存（uncachable）访问。此类平台访问除了 `mtrr_type_lookup()` 之外，不需要任何操作系统 MTRR 代码存在，以确保任何 OS 特定的映射请求与平台 MTRR 设置一致。不过，如果 MTRR 仅由平台固件代码设置，而操作系统不做任何特定的 MTRR 映射请求，则 `mtrr_type_lookup()` 应始终返回 `MTRR_TYPE_INVALID`。

详情请参阅 Documentation/arch/x86/pat.rst。

  在 Intel P6 系列处理器（Pentium Pro、Pentium II 及以后的型号）上，
  Memory Type Range Registers（MTRRs，内存类型范围寄存器）可用于控制
  处理器对内存范围的访问。当你在 PCI 或 AGP 总线上有一块视频（VGA）卡时，
  这最为有用。启用写合并（write-combining）可以在通过 PCI/AGP 总线
  突发传输之前，将总线写入传输合并为更大的传输。这可以将图像写入
  操作的性能提升 2.5 倍或更多。

  Cyrix 6x86、6x86MX 和 M II 处理器具有 Address Range Registers（ARRs，
  地址范围寄存器），提供与 MTRR 类似的功能。对于这些处理器，使用 ARR 来
  模拟 MTRR。

  AMD K6-2（stepping 8 及以上）和 K6-3 处理器有两个 MTRR。它们受支持。
  AMD Athlon 系列提供 8 个 Intel 风格的 MTRR。

  Centaur C6（WinChip）有 8 个 MCR，允许写合并。它们受支持。

  VIA Cyrix III 和 VIA C3 CPU 提供 8 个 Intel 风格的 MTRR。

  CONFIG_MTRR 选项会创建一个 /proc/mtrr 文件，可用于操作你的 MTRR。
  通常 X 服务器应当使用它。它应具有一个相当通用的接口，以便其他处理器上
  类似的 control registers 也能被轻松支持。

`/proc/mtrr` 有两种接口：一种是 ASCII 接口，允许你读取和写入；另一种是 `ioctl()` 接口。ASCII 接口用于管理，而 `ioctl()` 接口面向 C 程序（即 X 服务器）。下面以示例命令和 C 代码描述这些接口。

## 从 shell 读取 MTRR

```

  % cat /proc/mtrr
  reg00: base=0x00000000 (   0MB), size= 128MB: write-back, count=1
  reg01: base=0x08000000 ( 128MB), size=  64MB: write-back, count=1

```
```

  # echo "base=0xf8000000 size=0x400000 type=write-combining" >! /proc/mtrr

```
```

  # echo "base=0xf8000000 size=0x400000 type=write-combining" >| /proc/mtrr

```
```

  % cat /proc/mtrr
  reg00: base=0x00000000 (   0MB), size= 128MB: write-back, count=1
  reg01: base=0x08000000 ( 128MB), size=  64MB: write-back, count=1
  reg02: base=0xf8000000 (3968MB), size=   4MB: write-combining, count=1

```
这对应于基地址 0xf8000000、大小 4 兆字节的视频 RAM。要找出你的基地址，你需要查看 X 服务器的输出，它会告诉你线性帧缓冲区的地址在哪里。
```

  (--) S3: PCI: 968 rev 0, Linear FB @ 0xf8000000

```
注意你应当只使用来自 X 服务器的值，因为它可能会移动帧缓冲区基地址，所以你唯一可以信任的值就是 X 服务器报告的那个。

要找出你的帧缓冲区大小（怎么，你竟然不
```

  (--) S3: videoram:  4096k

```
那就是 4 兆字节，即 0x400000 字节（十六进制）。
XFree86 正在编写一个补丁以使这一切自动化：换句话说，X 服务器将使用 `ioctl()` 接口操作 /proc/mtrr，这样用户就无需做任何事。如果你使用商业 X 服务器，请游说你的供应商添加对 MTRR 的支持。

## 创建重叠的 MTRR

```

  %echo "base=0xfb000000 size=0x1000000 type=write-combining" >/proc/mtrr
  %echo "base=0xfb000000 size=0x1000 type=uncachable" >/proc/mtrr

```
```

  % cat /proc/mtrr
  reg00: base=0x00000000 (   0MB), size=  64MB: write-back, count=1
  reg01: base=0xfb000000 (4016MB), size=  16MB: write-combining, count=1
  reg02: base=0xfb000000 (4016MB), size=   4kB: uncachable, count=1

```
某些显卡（尤其是 Voodoo Graphics 板卡）需要从这个区域的起始处排除这 4 kB 区域，因为它被用作寄存器。

注意：只有当第一个你创建的区域的类型是 write-combining 时，你才能创建 type=uncachable 的区域。

## 从 C shell 移除 MTRR

```

  % echo "disable=2" >! /proc/mtrr

```
```

  % echo "disable=2" >| /proc/mtrr


```
## 从 C 程序使用 ioctl() 读取 MTRR

```

  /*  mtrr-show.c

      Source file for mtrr-show (example program to show MTRRs using ioctl()'s)

      Copyright (C) 1997-1998  Richard Gooch

      This program is free software; you can redistribute it and/or modify
      it under the terms of the GNU General Public License as published by
      the Free Software Foundation; either version 2 of the License, or
      (at your option) any later version.

      This program is distributed in the hope that it will be useful,
      but WITHOUT ANY WARRANTY; without even the implied warranty of
      MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
      GNU General Public License for more details.

      You should have received a copy of the GNU General Public License
      along with this program; if not, write to the Free Software
      Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.

      Richard Gooch may be reached by email at  rgooch@atnf.csiro.au
      The postal address is:
        Richard Gooch, c/o ATNF, P. O. Box 76, Epping, N.S.W., 2121, Australia.
  */

  /*
      This program will use an ioctl() on /proc/mtrr to show the current MTRR
      settings. This is an alternative to reading /proc/mtrr.


      Written by      Richard Gooch   17-DEC-1997

      Last updated by Richard Gooch   2-MAY-1998


  */
  #include <stdio.h>
  #include <stdlib.h>
  #include <string.h>
  #include <sys/types.h>
  #include <sys/stat.h>
  #include <fcntl.h>
  #include <sys/ioctl.h>
  #include <errno.h>
  #include <asm/mtrr.h>

  #define TRUE 1
  #define FALSE 0
  #define ERRSTRING strerror (errno)

  static char *mtrr_strings[MTRR_NUM_TYPES] =
  {
      "uncachable",               /* 0 */
      "write-combining",          /* 1 */
      "?",                        /* 2 */
      "?",                        /* 3 */
      "write-through",            /* 4 */
      "write-protect",            /* 5 */
      "write-back",               /* 6 */
  };

  int main ()
  {
      int fd;
      struct mtrr_gentry gentry;

      if ( ( fd = open ("/proc/mtrr", O_RDONLY, 0) ) == -1 )
      {
    if (errno == ENOENT)
    {
        fputs ("/proc/mtrr not found: not supported or you don't have a PPro?\n",
        stderr);
        exit (1);
    }
    fprintf (stderr, "Error opening /proc/mtrr\t%s\n", ERRSTRING);
    exit (2);
      }
      for (gentry.regnum = 0; ioctl (fd, MTRRIOC_GET_ENTRY, &gentry) == 0;
    ++gentry.regnum)
      {
    if (gentry.size < 1)
    {
        fprintf (stderr, "Register: %u disabled\n", gentry.regnum);
        continue;
    }
    fprintf (stderr, "Register: %u base: 0x%lx size: 0x%lx type: %s\n",
      gentry.regnum, gentry.base, gentry.size,
      mtrr_strings[gentry.type]);
      }
      if (errno == EINVAL) exit (0);
      fprintf (stderr, "Error doing ioctl(2) on /dev/mtrr\t%s\n", ERRSTRING);
      exit (3);
  }   /*  End Function main  */


```
## 从 C 程序使用 ioctl() 创建 MTRR

```

  /*  mtrr-add.c

      Source file for mtrr-add (example programme to add an MTRRs using ioctl())

      Copyright (C) 1997-1998  Richard Gooch

      This program is free software; you can redistribute it and/or modify
      it under the terms of the GNU General Public License as published by
      the Free Software Foundation; either version 2 of the License, or
      (at your option) any later version.

      This program is distributed in the hope that it will be useful,
      but WITHOUT ANY WARRANTY; without even the implied warranty of
      MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
      GNU General Public License for more details.

      You should have received a copy of the GNU General Public License
      along with this program; if not, write to the Free Software
      Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.

      Richard Gooch may be reached by email at  rgooch@atnf.csiro.au
      The postal address is:
        Richard Gooch, c/o ATNF, P. O. Box 76, Epping, N.S.W., 2121, Australia.
  */

  /*
      This programme will use an ioctl() on /proc/mtrr to add an entry. The first
      available mtrr is used. This is an alternative to writing /proc/mtrr.


      Written by      Richard Gooch   17-DEC-1997

      Last updated by Richard Gooch   2-MAY-1998


  */
  #include <stdio.h>
  #include <string.h>
  #include <stdlib.h>
  #include <unistd.h>
  #include <sys/types.h>
  #include <sys/stat.h>
  #include <fcntl.h>
  #include <sys/ioctl.h>
  #include <errno.h>
  #include <asm/mtrr.h>

  #define TRUE 1
  #define FALSE 0
  #define ERRSTRING strerror (errno)

  static char *mtrr_strings[MTRR_NUM_TYPES] =
  {
      "uncachable",               /* 0 */
      "write-combining",          /* 1 */
      "?",                        /* 2 */
      "?",                        /* 3 */
      "write-through",            /* 4 */
      "write-protect",            /* 5 */
      "write-back",               /* 6 */
  };

  int main (int argc, char **argv)
  {
      int fd;
      struct mtrr_sentry sentry;

      if (argc != 4)
      {
    fprintf (stderr, "Usage:\tmtrr-add base size type\n");
    exit (1);
      }
      sentry.base = strtoul (argv[1], NULL, 0);
      sentry.size = strtoul (argv[2], NULL, 0);
      for (sentry.type = 0; sentry.type < MTRR_NUM_TYPES; ++sentry.type)
      {
    if (strcmp (argv[3], mtrr_strings[sentry.type]) == 0) break;
      }
      if (sentry.type >= MTRR_NUM_TYPES)
      {
    fprintf (stderr, "Illegal type: \"%s\"\n", argv[3]);
    exit (2);
      }
      if ( ( fd = open ("/proc/mtrr", O_WRONLY, 0) ) == -1 )
      {
    if (errno == ENOENT)
    {
        fputs ("/proc/mtrr not found: not supported or you don't have a PPro?\n",
        stderr);
        exit (3);
    }
    fprintf (stderr, "Error opening /proc/mtrr\t%s\n", ERRSTRING);
    exit (4);
      }
      if (ioctl (fd, MTRRIOC_ADD_ENTRY, &sentry) == -1)
      {
    fprintf (stderr, "Error doing ioctl(2) on /dev/mtrr\t%s\n", ERRSTRING);
    exit (5);
      }
      fprintf (stderr, "Sleeping for 5 seconds so you can see the new entry\n");
      sleep (5);
      close (fd);
      fputs ("I've just closed /proc/mtrr so now the new entry should be gone\n",
      stderr);
  }   /*  End Function main  */

```
