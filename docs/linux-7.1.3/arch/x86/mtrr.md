
## MTRR锛圡emory Type Range Register锛屽唴瀛樼被鍨嬭寖鍥村瘎瀛樺櫒锛夋帶鍒?
:Authors: - Richard Gooch <rgooch@atnf.csiro.au> - 1999 骞?6 鏈?3 鏃?          - Luis R. Rodriguez <mcgrof@do-not-panic.com> - 2015 骞?4 鏈?9 鏃?
## 閫愭娣樻卑 MTRR 鐨勪娇鐢?
鍦ㄧ幇浠?x86 纭欢涓婏紝MTRR 鐨勪娇鐢ㄥ凡琚?PAT 鍙栦唬銆侺inux 涓┍鍔ㄧ洿鎺ュ MTRR 鐨勪娇鐢ㄧ幇宸插畬鍏ㄦ窐姹帮紝璁惧椹卞姩搴斿綋浣跨敤 `arch_phys_wc_add()` 閰嶅悎 `ioremap_wc()`锛屼互鍦ㄩ潪 PAT 绯荤粺涓婁娇 MTRR 鐢熸晥锛岃€屽湪 PAT 鍚敤绯荤粺涓婂垯涓轰竴涓┖鎿嶄綔浣嗗悓鏍风敓鏁堛€?
鍗充娇 Linux 涓嶇洿鎺ヤ娇鐢?MTRR锛屼竴浜?x86 骞冲彴鍥轰欢浠嶅彲鑳藉湪鍚姩鎿嶄綔绯荤粺涔嬪墠灏芥棭璁剧疆濂?MTRR銆傚畠浠繖鏍峰仛鏄洜涓洪儴鍒嗗钩鍙板浐浠跺彲鑳戒粛鐒跺疄鐜颁簡瀵?MTRR 鐨勮闂紝鑰岃繖浜涘皢鐢卞钩鍙板浐浠剁洿鎺ユ帶鍒跺拰澶勭悊銆傚钩鍙颁娇鐢?MTRR 鐨勪竴涓緥瀛愭槸閫氳繃 SMI 澶勭悊绋嬪簭锛屼竴绉嶆儏鍐靛彲鑳芥槸鐢ㄤ簬椋庢墖鎺у埗锛屽钩鍙颁唬鐮侀渶瑕佸鍏堕儴鍒嗛鎵囨帶鍒跺瘎瀛樺櫒杩涜涓嶅彲缂撳瓨锛坲ncachable锛夎闂€傛绫诲钩鍙拌闂櫎浜?`mtrr_type_lookup()` 涔嬪锛屼笉闇€瑕佷换浣曟搷浣滅郴缁?MTRR 浠ｇ爜瀛樺湪锛屼互纭繚浠讳綍 OS 鐗瑰畾鐨勬槧灏勮姹備笌骞冲彴 MTRR 璁剧疆涓€鑷淬€備笉杩囷紝濡傛灉 MTRR 浠呯敱骞冲彴鍥轰欢浠ｇ爜璁剧疆锛岃€屾搷浣滅郴缁熶笉鍋氫换浣曠壒瀹氱殑 MTRR 鏄犲皠璇锋眰锛屽垯 `mtrr_type_lookup()` 搴斿缁堣繑鍥?`MTRR_TYPE_INVALID`銆?
璇︽儏璇峰弬闃?Documentation/arch/x86/pat.rst銆?
  鍦?Intel P6 绯诲垪澶勭悊鍣紙Pentium Pro銆丳entium II 鍙婁互鍚庣殑鍨嬪彿锛変笂锛?  Memory Type Range Registers锛圡TRRs锛屽唴瀛樼被鍨嬭寖鍥村瘎瀛樺櫒锛夊彲鐢ㄤ簬鎺у埗
  澶勭悊鍣ㄥ鍐呭瓨鑼冨洿鐨勮闂€傚綋浣犲湪 PCI 鎴?AGP 鎬荤嚎涓婃湁涓€鍧楄棰戯紙VGA锛夊崱鏃讹紝
  杩欐渶涓烘湁鐢ㄣ€傚惎鐢ㄥ啓鍚堝苟锛坵rite-combining锛夊彲浠ュ湪閫氳繃 PCI/AGP 鎬荤嚎
  绐佸彂浼犺緭涔嬪墠锛屽皢鎬荤嚎鍐欏叆浼犺緭鍚堝苟涓烘洿澶х殑浼犺緭銆傝繖鍙互灏嗗浘鍍忓啓鍏?  鎿嶄綔鐨勬€ц兘鎻愬崌 2.5 鍊嶆垨鏇村銆?
  Cyrix 6x86銆?x86MX 鍜?M II 澶勭悊鍣ㄥ叿鏈?Address Range Registers锛圓RRs锛?  鍦板潃鑼冨洿瀵勫瓨鍣級锛屾彁渚涗笌 MTRR 绫讳技鐨勫姛鑳姐€傚浜庤繖浜涘鐞嗗櫒锛屼娇鐢?ARR 鏉?  妯℃嫙 MTRR銆?
  AMD K6-2锛坰tepping 8 鍙婁互涓婏級鍜?K6-3 澶勭悊鍣ㄦ湁涓や釜 MTRR銆傚畠浠彈鏀寔銆?  AMD Athlon 绯诲垪鎻愪緵 8 涓?Intel 椋庢牸鐨?MTRR銆?
  Centaur C6锛圵inChip锛夋湁 8 涓?MCR锛屽厑璁稿啓鍚堝苟銆傚畠浠彈鏀寔銆?
  VIA Cyrix III 鍜?VIA C3 CPU 鎻愪緵 8 涓?Intel 椋庢牸鐨?MTRR銆?
  CONFIG_MTRR 閫夐」浼氬垱寤轰竴涓?/proc/mtrr 鏂囦欢锛屽彲鐢ㄤ簬鎿嶄綔浣犵殑 MTRR銆?  閫氬父 X 鏈嶅姟鍣ㄥ簲褰撲娇鐢ㄥ畠銆傚畠搴斿叿鏈変竴涓浉褰撻€氱敤鐨勬帴鍙ｏ紝浠ヤ究鍏朵粬澶勭悊鍣ㄤ笂
  绫讳技鐨?control registers 涔熻兘琚交鏉炬敮鎸併€?
`/proc/mtrr` 鏈変袱绉嶆帴鍙ｏ細涓€绉嶆槸 ASCII 鎺ュ彛锛屽厑璁镐綘璇诲彇鍜屽啓鍏ワ紱鍙︿竴绉嶆槸 `ioctl()` 鎺ュ彛銆侫SCII 鎺ュ彛鐢ㄤ簬绠＄悊锛岃€?`ioctl()` 鎺ュ彛闈㈠悜 C 绋嬪簭锛堝嵆 X 鏈嶅姟鍣級銆備笅闈互绀轰緥鍛戒护鍜?C 浠ｇ爜鎻忚堪杩欎簺鎺ュ彛銆?
## 浠?shell 璇诲彇 MTRR

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
杩欏搴斾簬鍩哄湴鍧€ 0xf8000000銆佸ぇ灏?4 鍏嗗瓧鑺傜殑瑙嗛 RAM銆傝鎵惧嚭浣犵殑鍩哄湴鍧€锛屼綘闇€瑕佹煡鐪?X 鏈嶅姟鍣ㄧ殑杈撳嚭锛屽畠浼氬憡璇変綘绾挎€у抚缂撳啿鍖虹殑鍦板潃鍦ㄥ摢閲屻€?```

  (--) S3: PCI: 968 rev 0, Linear FB @ 0xf8000000

```
娉ㄦ剰浣犲簲褰撳彧浣跨敤鏉ヨ嚜 X 鏈嶅姟鍣ㄧ殑鍊硷紝鍥犱负瀹冨彲鑳戒細绉诲姩甯х紦鍐插尯鍩哄湴鍧€锛屾墍浠ヤ綘鍞竴鍙互淇′换鐨勫€煎氨鏄?X 鏈嶅姟鍣ㄦ姤鍛婄殑閭ｄ釜銆?
瑕佹壘鍑轰綘鐨勫抚缂撳啿鍖哄ぇ灏忥紙鎬庝箞锛屼綘绔熺劧涓?```

  (--) S3: videoram:  4096k

```
閭ｅ氨鏄?4 鍏嗗瓧鑺傦紝鍗?0x400000 瀛楄妭锛堝崄鍏繘鍒讹級銆?XFree86 姝ｅ湪缂栧啓涓€涓ˉ涓佷互浣胯繖涓€鍒囪嚜鍔ㄥ寲锛氭崲鍙ヨ瘽璇达紝X 鏈嶅姟鍣ㄥ皢浣跨敤 `ioctl()` 鎺ュ彛鎿嶄綔 /proc/mtrr锛岃繖鏍风敤鎴峰氨鏃犻渶鍋氫换浣曚簨銆傚鏋滀綘浣跨敤鍟嗕笟 X 鏈嶅姟鍣紝璇锋父璇翠綘鐨勪緵搴斿晢娣诲姞瀵?MTRR 鐨勬敮鎸併€?
## 鍒涘缓閲嶅彔鐨?MTRR

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
鏌愪簺鏄惧崱锛堝挨鍏舵槸 Voodoo Graphics 鏉垮崱锛夐渶瑕佷粠杩欎釜鍖哄煙鐨勮捣濮嬪鎺掗櫎杩?4 kB 鍖哄煙锛屽洜涓哄畠琚敤浣滃瘎瀛樺櫒銆?
娉ㄦ剰锛氬彧鏈夊綋绗竴涓綘鍒涘缓鐨勫尯鍩熺殑绫诲瀷鏄?write-combining 鏃讹紝浣犳墠鑳藉垱寤?type=uncachable 鐨勫尯鍩熴€?
## 浠?C shell 绉婚櫎 MTRR

```

  % echo "disable=2" >! /proc/mtrr

```
```

  % echo "disable=2" >| /proc/mtrr


```
## 浠?C 绋嬪簭浣跨敤 ioctl() 璇诲彇 MTRR

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
## 浠?C 绋嬪簭浣跨敤 ioctl() 鍒涘缓 MTRR

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
