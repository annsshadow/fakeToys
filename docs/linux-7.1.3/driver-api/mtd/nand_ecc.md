## NAND 纠错码（Error-correction Code
## 简
在研究过 Linux mtd/nand Hamming 软件 ECC 引擎驱动之后，我觉得还有优化的空间。我对代码折腾了几个小时，做了诸如查表、移除多余代码之类的技巧。之后速度提升35%0%。尽管如此我仍不太满意，因为我感觉还有进一步的改进余地
糟糕！我上瘾了。我决定在这个文件里记录下我的每一步。或许它对某个人有用，或者有人能从中有所收获

## 问题

NAND 闪存（至少是 SLC 那种）通常具有 256 字节大小的扇区。然NAND 闪存并不是极其可靠，因此需要一些错误检测（有时还需要纠错）
这通过 Hamming 码来完成。我会尝试用外行的话来解释它（如果我没有使用正确的术语，请向该领域所有的专业人士致歉，我的编码理论课几乎是在 30 年前上的，而且我必须承认那并不是我最喜欢的课）
正如我之前所说，ecc 计算是在 256 字节的扇区上进行的。这是通过计算行和列上的若干奇偶校验位来实现的。所用的是偶校验（even parity），即：如果被计算奇偶校验的数据1，则奇偶校验= 1；如果被计算奇偶校验的数据为 0，则奇偶校验= 0。因此，被计算奇偶校验的数据的位总数加上奇偶校验位为偶数。（如果跟不上，请看 wikipedia。）奇偶校验通常通过异或（exclusive or）运算来计算，有时也称为 xor。在 C 语言xor 的运算符^

回到 ecc。让我们给出一张小图：

=========  ==== ==== ==== ==== ==== ==== ==== ====   === === === === ====
byte   0:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp0 rp2 rp4 ... rp14
byte   1:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp1 rp2 rp4 ... rp14
byte   2:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp0 rp3 rp4 ... rp14
byte   3:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp1 rp3 rp4 ... rp14
byte   4:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp0 rp2 rp5 ... rp14
...
byte 254:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp0 rp3 rp5 ... rp15
byte 255:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp1 rp3 rp5 ... rp15
           cp1  cp0  cp1  cp0  cp1  cp0  cp1  cp0
           cp3  cp3  cp2  cp2  cp3  cp3  cp2  cp2
           cp5  cp5  cp5  cp5  cp4  cp4  cp4  cp4
=========  ==== ==== ==== ==== ==== ==== ==== ====   === === === === ====

这张图表示一256 字节的扇区。cp column parity（列奇偶校验）的缩写，rp row parity（行奇偶校验）的缩写
让我们开始解释列奇偶校验
- cp0 是属于所bit0、bit2、bit4、bit6 的奇偶校验
  因此所bit0、bit2、bit4 bit6 的值之和加cp0 本身为偶数
类似cp1 是所bit1、bit3、bit5 bit7 之和
- cp2 bit0、bit1、bit4 bit5 上的奇偶校验
- cp3 bit2、bit3、bit6 bit7 上的奇偶校验- cp4 bit0、bit1、bit2 bit3 上的奇偶校验- cp5 bit4、bit5、bit6 bit7 上的奇偶校验
注意 cp0 .. cp5 每一个都恰好是一位
行奇偶校验的工作方式几乎相同
- rp0 是所有偶数字节（0, 2, 4, 6, ... 252, 254）的奇偶校验
- rp1 是所有奇数字节（1, 3, 5, 7, ..., 253, 255）的奇偶校验
- rp2 是字0, 1, 4, 5, 8, 9, ... 的奇偶校  （即处理两个字节，然后跳2 个字节）- rp3 覆盖 rp2 未覆盖的那一半（字节 2, 3, 6, 7, 10, 11, ...- 对于 rp4，规则是覆盖 4 个字节，跳过 4 个字节，覆盖 4 个字节，跳过 4 个，依此类推
  所rp4 计算字节 0, 1, 2, 3, 8, 9, 10, 11, 16, ... 上的奇偶校验
- 鑰?rp5 瑕嗙洊鍙︿竴鍗婏紝鍗冲瓧鑺?4, 5, 6, 7, 12, 13, 14, 15, 20, ..

接下来的叙述就变得相当乏味了。我想你已经明白意思了
- rp6 覆盖 8 个字节然后跳8 个，依此类推
- rp7 跳过 8 个字节然后覆8 个，依此类推
- rp8 覆盖 16 个字节然后跳16 个，依此类推
- rp9 跳过 16 个字节然后覆16 个，依此类推
- rp10 覆盖 32 个字节然后跳32 个，依此类推
- rp11 跳过 32 个字节然后覆32 个，依此类推
- rp12 覆盖 64 个字节然后跳64 个，依此类推
- rp13 跳过 64 个字节然后覆64 个，依此类推
- rp14 覆盖 128 个字节然后跳128
- rp15 跳过 128 个字节然后覆128

最后，奇偶校验位被分组到三个字节中，如下所示：

=====  ===== ===== ===== ===== ===== ===== ===== =====
ECC    Bit 7 Bit 6 Bit 5 Bit 4 Bit 3 Bit 2 Bit 1 Bit 0
=====  ===== ===== ===== ===== ===== ===== ===== =====
ECC 0   rp07  rp06  rp05  rp04  rp03  rp02  rp01  rp00
ECC 1   rp15  rp14  rp13  rp12  rp11  rp10  rp09  rp08
ECC 2   cp5   cp4   cp3   cp2   cp1   cp0      1     1
=====  ===== ===== ===== ===== ===== ===== ===== =====

我在写完这些之后发现，ST 应用笔记 AN1823（http://www.st.com/stonline/）给出了一幅更漂亮的图。（不过他们line parity 这个术语，而我用的row parity。）哦算了，我画图不行，所以请陪我忍受一:-)

而且由于版权原因，我也无法复ST 的图

## 尝试 0

实现奇偶校验的计算相当简单```

  for (i = 0; i < 256; i++)
  {
    if (i & 0x01)
       rp1 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp1;
    else
       rp0 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp0;
    if (i & 0x02)
       rp3 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp3;
    else
       rp2 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp2;
    if (i & 0x04)
      rp5 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp5;
    else
      rp4 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp4;
    if (i & 0x08)
      rp7 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp7;
    else
      rp6 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp6;
    if (i & 0x10)
      rp9 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp9;
    else
      rp8 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp8;
    if (i & 0x20)
      rp11 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp11;
    else
      rp10 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp10;
    if (i & 0x40)
      rp13 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp13;
    else
      rp12 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp12;
    if (i & 0x80)
      rp15 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp15;
    else
      rp14 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp14;
    cp0 = bit6 ^ bit4 ^ bit2 ^ bit0 ^ cp0;
    cp1 = bit7 ^ bit5 ^ bit3 ^ bit1 ^ cp1;
    cp2 = bit5 ^ bit4 ^ bit1 ^ bit0 ^ cp2;
    cp3 = bit7 ^ bit6 ^ bit3 ^ bit2 ^ cp3
    cp4 = bit3 ^ bit2 ^ bit1 ^ bit0 ^ cp4
    cp5 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ cp5
  }


```
## 分析 0

C 语言确实有位运算符，但并没有能高效完成上述运算的运算符（而且大多数硬件也没有这类指令）。因此无需实现就能清楚，上面的代码不会给我带来诺贝尔奖 :-)

幸运的是，异或运算是可交换的，所以我们可以以任意顺序组合这些值。因此，与其逐个计算所有位，不如尝试重新排列一下。对于列奇偶校验这很容易。我们可以简单地对字节做 xor，最后再过滤出相关的位。这非常好，因为它会把所cp 计算移出 for 循环
类似地，我们可以先为各个xor 字节。这可以引出

## 尝试 1

```
  const char parity[256] = {
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0
  };

  void ecc1(const unsigned char *buf, unsigned char *code)
  {
      int i;
      const unsigned char *bp = buf;
      unsigned char cur;
      unsigned char rp0, rp1, rp2, rp3, rp4, rp5, rp6, rp7;
      unsigned char rp8, rp9, rp10, rp11, rp12, rp13, rp14, rp15;
      unsigned char par;

      par = 0;
      rp0 = 0; rp1 = 0; rp2 = 0; rp3 = 0;
      rp4 = 0; rp5 = 0; rp6 = 0; rp7 = 0;
      rp8 = 0; rp9 = 0; rp10 = 0; rp11 = 0;
      rp12 = 0; rp13 = 0; rp14 = 0; rp15 = 0;

      for (i = 0; i < 256; i++)
      {
          cur = *bp++;
          par ^= cur;
          if (i & 0x01) rp1 ^= cur; else rp0 ^= cur;
          if (i & 0x02) rp3 ^= cur; else rp2 ^= cur;
          if (i & 0x04) rp5 ^= cur; else rp4 ^= cur;
          if (i & 0x08) rp7 ^= cur; else rp6 ^= cur;
          if (i & 0x10) rp9 ^= cur; else rp8 ^= cur;
          if (i & 0x20) rp11 ^= cur; else rp10 ^= cur;
          if (i & 0x40) rp13 ^= cur; else rp12 ^= cur;
          if (i & 0x80) rp15 ^= cur; else rp14 ^= cur;
      }
      code[0] =
          (parity[rp7] << 7) |
          (parity[rp6] << 6) |
          (parity[rp5] << 5) |
          (parity[rp4] << 4) |
          (parity[rp3] << 3) |
          (parity[rp2] << 2) |
          (parity[rp1] << 1) |
          (parity[rp0]);
      code[1] =
          (parity[rp15] << 7) |
          (parity[rp14] << 6) |
          (parity[rp13] << 5) |
          (parity[rp12] << 4) |
          (parity[rp11] << 3) |
          (parity[rp10] << 2) |
          (parity[rp9]  << 1) |
          (parity[rp8]);
      code[2] =
          (parity[par & 0xf0] << 7) |
          (parity[par & 0x0f] << 6) |
          (parity[par & 0xcc] << 5) |
          (parity[par & 0x33] << 4) |
          (parity[par & 0xaa] << 3) |
          (parity[par & 0x55] << 2);
      code[0] = ~code[0];
      code[1] = ~code[1];
      code[2] = ~code[2];
  }

```
仍然相当直观。最后三invert（取反）语句是为了让空闪存得0xff 0xff 0xff 的校验和。在空闪存中所有数据都0xff，因此校验和随之匹配
我还引入parity 查表。我原本期望这是计算奇偶校验最快的方式，但我稍后会研究替代方案

## 分析 1

代码能用，但效率并不高得惊人。在我的系统上它耗费的时间几乎是 Linux 驱动代码4 倍。不过，如果真那么容易，这早就被人做过了没有付出，就没有收获
幸运的是仍有大量改进空间
在第 1 步中我们从逐位计算转移到了逐字节计算。然而在 C 中我们也可以使用 unsigned long 数据类型，而且几乎每个现代微处理器都支32 位操作，那么为何不尝试把代码写成32 位块来处理数据的方式呢
当然，这意味着一些修改，因为行奇偶校验是逐字节的。一个快速分析：
对于列奇偶校验我们使par 变量。当扩展32 位时，我们最终可以轻松地从它计算rp0 rp1（因par 现在4 个字节组成，分别MSB LSB 贡献rp1、rp0、rp1、rp0同样 rp2 rp3 也可以轻松地par 取得，因rp3 覆盖前两MSB，rp2 覆盖后两LSB
注意现在循环只执64 次（256/4）。并且注意必须小心字节序（byte ordering）。字节在 long 中的排列顺序是机器相关的，可能会影响到我们。无论如何，如果有问题：这段代码是在 x86 上开发的（确切地说：是一台带D920 Intel CPU DELL PC
当然，性能可能也取决于对齐，但我预nand 驱动中的 I/O 缓冲区是对齐良好的（否则应当修复它以获得最大性能）
让我们试一试…

## 尝试 2

```
  extern const char parity[256];

  void ecc2(const unsigned char *buf, unsigned char *code)
  {
      int i;
      const unsigned long *bp = (unsigned long *)buf;
      unsigned long cur;
      unsigned long rp0, rp1, rp2, rp3, rp4, rp5, rp6, rp7;
      unsigned long rp8, rp9, rp10, rp11, rp12, rp13, rp14, rp15;
      unsigned long par;

      par = 0;
      rp0 = 0; rp1 = 0; rp2 = 0; rp3 = 0;
      rp4 = 0; rp5 = 0; rp6 = 0; rp7 = 0;
      rp8 = 0; rp9 = 0; rp10 = 0; rp11 = 0;
      rp12 = 0; rp13 = 0; rp14 = 0; rp15 = 0;

      for (i = 0; i < 64; i++)
      {
          cur = *bp++;
          par ^= cur;
          if (i & 0x01) rp5 ^= cur; else rp4 ^= cur;
          if (i & 0x02) rp7 ^= cur; else rp6 ^= cur;
          if (i & 0x04) rp9 ^= cur; else rp8 ^= cur;
          if (i & 0x08) rp11 ^= cur; else rp10 ^= cur;
          if (i & 0x10) rp13 ^= cur; else rp12 ^= cur;
          if (i & 0x20) rp15 ^= cur; else rp14 ^= cur;
      }
      /*
         we need to adapt the code generation for the fact that rp vars are now
         long; also the column parity calculation needs to be changed.
         we'll bring rp4 to 15 back to single byte entities by shifting and
         xoring
      */
      rp4 ^= (rp4 >> 16); rp4 ^= (rp4 >> 8); rp4 &= 0xff;
      rp5 ^= (rp5 >> 16); rp5 ^= (rp5 >> 8); rp5 &= 0xff;
      rp6 ^= (rp6 >> 16); rp6 ^= (rp6 >> 8); rp6 &= 0xff;
      rp7 ^= (rp7 >> 16); rp7 ^= (rp7 >> 8); rp7 &= 0xff;
      rp8 ^= (rp8 >> 16); rp8 ^= (rp8 >> 8); rp8 &= 0xff;
      rp9 ^= (rp9 >> 16); rp9 ^= (rp9 >> 8); rp9 &= 0xff;
      rp10 ^= (rp10 >> 16); rp10 ^= (rp10 >> 8); rp10 &= 0xff;
      rp11 ^= (rp11 >> 16); rp11 ^= (rp11 >> 8); rp11 &= 0xff;
      rp12 ^= (rp12 >> 16); rp12 ^= (rp12 >> 8); rp12 &= 0xff;
      rp13 ^= (rp13 >> 16); rp13 ^= (rp13 >> 8); rp13 &= 0xff;
      rp14 ^= (rp14 >> 16); rp14 ^= (rp14 >> 8); rp14 &= 0xff;
      rp15 ^= (rp15 >> 16); rp15 ^= (rp15 >> 8); rp15 &= 0xff;
      rp3 = (par >> 16); rp3 ^= (rp3 >> 8); rp3 &= 0xff;
      rp2 = par & 0xffff; rp2 ^= (rp2 >> 8); rp2 &= 0xff;
      par ^= (par >> 16);
      rp1 = (par >> 8); rp1 &= 0xff;
      rp0 = (par & 0xff);
      par ^= (par >> 8); par &= 0xff;

      code[0] =
          (parity[rp7] << 7) |
          (parity[rp6] << 6) |
          (parity[rp5] << 5) |
          (parity[rp4] << 4) |
          (parity[rp3] << 3) |
          (parity[rp2] << 2) |
          (parity[rp1] << 1) |
          (parity[rp0]);
      code[1] =
          (parity[rp15] << 7) |
          (parity[rp14] << 6) |
          (parity[rp13] << 5) |
          (parity[rp12] << 4) |
          (parity[rp11] << 3) |
          (parity[rp10] << 2) |
          (parity[rp9]  << 1) |
          (parity[rp8]);
      code[2] =
          (parity[par & 0xf0] << 7) |
          (parity[par & 0x0f] << 6) |
          (parity[par & 0xcc] << 5) |
          (parity[par & 0x33] << 4) |
          (parity[par & 0xaa] << 3) |
          (parity[par & 0x55] << 2);
      code[0] = ~code[0];
      code[1] = ~code[1];
      code[2] = ~code[2];
  }

```
parity 数组不再展示了。还要注意，对于这些示例，我有意偏离了我平常的编程风格，允许一行多条语句、在只有单条语句then else 块中不使{ }，并使用^= 这类运算符

## 分析 2

代码（当然）能用，并且好耶：我们Linux 驱动代码快了一点点（约 15%）。不过等等，别高兴得太早。还有更多可提升的空间如果我们看例rp14 rp15，会发现我们要么rp14 异或数据，要么用 rp15 异或数据。然而我们还有遍历所有数据的 par。这意味着无需计算 rp14，因为它可以通过 rp14 = par ^ rp15 rp15 算出来，因为 par = rp14 ^ rp15（或者如果愿意，我们可以避免计算 rp15，而从 rp14 算出来）。这就是为什么有些地方提到了 inverse parity（逆奇偶校验）当然，同样的情况适用rp4/5、rp6/7、rp8/9、rp10/11 rp12/13实际上这意味着我们可以if 语句中去else 子句。而且我们还可以通过先从 long 回到 byte 来在最后稍微优化一下计算。事实上我们甚至可以不用查表

## 尝试 3

```
          if (i & 0x01) rp5 ^= cur; else rp4 ^= cur;
          if (i & 0x02) rp7 ^= cur; else rp6 ^= cur;
          if (i & 0x04) rp9 ^= cur; else rp8 ^= cur;
          if (i & 0x08) rp11 ^= cur; else rp10 ^= cur;
          if (i & 0x10) rp13 ^= cur; else rp12 ^= cur;
          if (i & 0x20) rp15 ^= cur; else rp14 ^= cur;

```
```
          if (i & 0x01) rp5 ^= cur;
          if (i & 0x02) rp7 ^= cur;
          if (i & 0x04) rp9 ^= cur;
          if (i & 0x08) rp11 ^= cur;
          if (i & 0x10) rp13 ^= cur;
          if (i & 0x20) rp15 ^= cur;

```
```
          rp4  = par ^ rp5;
          rp6  = par ^ rp7;
          rp8  = par ^ rp9;
          rp10  = par ^ rp11;
          rp12  = par ^ rp13;
          rp14  = par ^ rp15;

```
此后代码耗时增加了约 30%，尽管语句数量减少了。汇编代码也反映了这一点

## 分析 3

很奇怪。我猜这与缓存或指令并行之类有关。我也在 eeePC（Celeron，主900 Mhz）上试过。一个有趣的观察是，在执行这段代码时，它只比3Ghz D920 处理器慢 30%（根time 测量）
嗯，本来就知道不会容易，所以也许该换条路：让我们回到尝2 的代码并做一点循环展开（loop unrolling）。这会消除几if 语句。我会尝试不同展开量，看看哪个效果最好

## 尝试 4

将循环展开 1 4 次```

    for (i = 0; i < 4; i++)
    {
        cur = *bp++;
        par ^= cur;
        rp4 ^= cur;
        rp6 ^= cur;
        rp8 ^= cur;
        rp10 ^= cur;
        if (i & 0x1) rp13 ^= cur; else rp12 ^= cur;
        if (i & 0x2) rp15 ^= cur; else rp14 ^= cur;
        cur = *bp++;
        par ^= cur;
        rp5 ^= cur;
        rp6 ^= cur;
        ...


```
## 分析 4

展开一次获得约 15% 的提
展开两次将提升保持在15%

展开三次相比尝试 2 获得 30% 的提升
展开四次相比展开三次只有边际改进
我决定无论如何还是继续采用展开四次的循环。我的直觉是，在接下来的步骤中我会从中获得额外的收益
下一步的触发点是：par 包含了所有字节的 xor，rp4 rp5 各自包含了一半字节的 xor所以实际上 par = rp4 ^ rp5。但由于 xor 是可交换的，我们也可以说 rp5 = par ^ rp4。因此无需同时保留 rp4 rp5。我们可以去rp5（或 rp4，但我已经预见到了另一个优化）同样的情况适用rp6/7、rp8/9、rp10/11、rp12/13 rp14/15

## 尝试 5

实际上，循环中所有的奇数 rp 赋值都被移除了这包if 语句else 子句```

    rp5 = par ^ rp4;

```
同时，初始赋值（rp5 = 0; 等）也可以移除沿此思路，我也移除了 rp0/1/2/3 的初始化

## 分析 5

测量表明这是一个好举措。相比展开四次的尝4，运行时间大约减半，而且相比 Linux 内核中当前的代码，我们只需1/3 的处理器时间
然而，我仍觉得还有空间。我不喜欢那if 语句。为什么不保持一个运行中的奇偶校验，只保留最后一if 语句。是时候再来一个版本了

## 尝试 6

```

    for (i = 0; i < 4; i++)
    {
        cur = *bp++; tmppar  = cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= tmppar;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur; rp8 ^= tmppar;

        cur = *bp++; tmppar ^= cur; rp4 ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur; rp10 ^= tmppar;

        cur = *bp++; tmppar ^= cur; rp4 ^= cur; rp6 ^= cur; rp8 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= cur; rp8 ^= cur;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur; rp8 ^= cur;
        cur = *bp++; tmppar ^= cur; rp8 ^= cur;

        cur = *bp++; tmppar ^= cur; rp4 ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur;

        par ^= tmppar;
        if ((i & 0x1) == 0) rp12 ^= tmppar;
        if ((i & 0x2) == 0) rp14 ^= tmppar;
    }

```
如你所见，tmppar 用于for 循环的一次迭代内累积奇偶校验。在最后的 3 条语句中，它被加par 上，并在需要时加到 rp12 rp14 上
在做这些改动的同时，我还发现可以利用 tmppar 包含本次迭代运行中奇偶校验这一点。所以与其写rp4 ^= cur; rp6 ^= cur;
我去掉了 rp6 ^= cur; 语句，并在下一条语句中rp6 ^= tmppar;。对 rp8 rp10 也做了类似的改动

## 分析 6

再次测量这段代码显示了巨大的收益。当执行原始Linux 代码 100 万次时，在我的系统上大约需1 秒（使time 来测量性能）。经过这次迭代后我回到了 0.075 秒。实际上我不得不决定在超1000 万次迭代上测量，以免损失过多精度。这绝对看上去是中大奖了
不过还有一点改进空间。在循环中有三处
```

	rp4 ^= cur; rp6 ^= cur;

```
维护一个变rp4_6 似乎更高效；这每次循环消3 条语句。当然在循环之后我们
```

	rp4 ^= rp4_6;
	rp6 ^= rp4_6

```
此外4 条顺序的赋值给 rp8。这可以用略微更高效的方式编码：在那 4 行之前保tmppar，之后再rp8 = rp8 ^ tmppar ^ notrp8;
（其notrp8 是那 4 行之rp8 的值）这再次利用了 xor 的可交换性质。是时候做新测试了

## 尝试 7

```

    for (i = 0; i < 4; i++)
    {
        cur = *bp++; tmppar  = cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= tmppar;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur; rp8 ^= tmppar;

        cur = *bp++; tmppar ^= cur; rp4_6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur; rp10 ^= tmppar;

        notrp8 = tmppar;
        cur = *bp++; tmppar ^= cur; rp4_6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur;
        rp8 = rp8 ^ tmppar ^ notrp8;

        cur = *bp++; tmppar ^= cur; rp4_6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur;

        par ^= tmppar;
        if ((i & 0x1) == 0) rp12 ^= tmppar;
        if ((i & 0x2) == 0) rp14 ^= tmppar;
    }
    rp4 ^= rp4_6;
    rp6 ^= rp4_6;


```
改动不大，但积少成多 :-)


## 分析 7

实际上这让事情变糟了。不太多，但我不想往错误的方向走。也许以后可以研究一下。可能又和缓存有关
我想循环内能赢的也就这些了。再多展开一次也许有帮助。我暂时保留来自尝试 7 的优化

## 尝试 8

将循环再展开一次

## 分析 8

这让事情变糟了。让我们坚持尝试 6，并从那里继续。虽然循环内的代码似乎无法进一步优化，但生ecc 码仍有优化空间我们可以简单地计算总奇偶校验。如果它0，那rp4 = rp5 等等。如果奇偶校验是 1，那rp4 = !rp5
但如rp4 = rp5 我们就不需rp5 等等。我们可以直接写入偶数位
```

    code[0] |= (code[0] << 1);

```
让我们测试一下

## 尝试 9

改了代码，但同样这略微降低了性能。试过各种其他办法，比如使用专用的奇偶校验数组以避免 parity[rp7] << 7 之后的移位。没有收益使用移位运算符（例如）来替换 parity 数组的查找：
```

	rp7 ^= (rp7 << 4);
	rp7 ^= (rp7 << 2);
	rp7 ^= (rp7 << 1);
	rp7 &= 0x80;

```
没有收益
唯一的边际改动是反转奇偶校验位，这样我们可以去掉最后三invert 语句
啊，真可惜这没有带来更多收益。话说回来，使用 Linux 驱动代码执行 1000 万次迭代需13 13.5 秒，而我的代码现在对这些 1000 万次迭代大约只需 0.73 秒。所以基本上我在我的系统上将性能提升18 倍。还不赖。当然在不同的硬件上你会得到不同的结果。不提供任何保证
但当然天下没有免费的午餐。代码大小几乎翻了三倍（562 字节1434 字节）。话又说回来，也没那么大

## 纠正错误

对于纠正错误，我再次ST 应用笔记为起点，但我也瞄了一眼现有代码
算法本身相当直接。只需 xor 给定ecc 与计算出ecc。如果所有字节都0 就没有问题。如果有 11 位是 1，我们就有一个可纠正的位错误。如果只1 位是 1，则给定ecc 码中有错误
证明最快的方法是做几次查表。当需要做修复时，由此引入的性能提升在我的系统上约为 2 倍；而当无需修复时则1% 左右
该函数的代码大小330 字节增加686 字节（gcc 4.2, -O3

## 结论

计算 ecc 时的收益是巨大的。在我的开发硬件上获得ecc 计算 18 倍的加速。在一个带MIPS 核心的嵌入式系统测试中获得了 7 倍
Linksys NSLU2（ARMv5TE 处理器）的测试中加速为 5 倍（大端模式，gcc 4.1.2, -O3
对于纠正则得不到太多收益（因为位翻转很罕见）。话说回来，那里花费的周期也要少得多
似乎在这方面没有太多可赢的空间了，至少在C 编程的情况下。当然用汇编程序也许能从它里面再榨出一点，但由于流水线行为等原因，这非常棘手（至少intel 硬件而言）
Author: Frans Meulenbroeks

Copyright (C) 2008 Koninklijke Philips Electronics NV.
