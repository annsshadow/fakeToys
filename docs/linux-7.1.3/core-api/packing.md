## 通用位域打包与解包函数


### 问题陈述


与硬件打交道时，必须在几种与其交互的方法之间做出选择。可以将一个指针内存映射到硬件
设备内存区域上一个精心构造的结构体，并将其字段作为结构体成员（可能被声明为位域）访问。
但以这种方式编写代码会降低可移植性，因为 CPU 与硬件设备之间可能存在字节序（endianness）
不匹配。此外，将硬件文档中的寄存器定义转换为结构体所需的位域索引时，需要格外注意。还有，
某些硬件（通常是网络设备）倾向于以违反任何合理字边界（有时甚至是 64 位边界）的方式对
其寄存器字段进行分组。这就造成了必须在结构体内定义寄存器字段“高”和“低”部分的麻烦。

比结构体字段定义更稳健的替代方案，是通过移位适当的位数来提取所需字段。但这仍无法防范
字节序不匹配——除非所有内存访问都逐字节进行。而且代码很容易变得杂乱，高层思路可能会
淹没在所需的众多位移位之中。

许多驱动采用位移位的方法，然后试图用定制的宏来减少杂乱，但这些宏多半会走捷径，仍使
代码无法真正可移植。

### 解决方案


本 API 处理两个基本操作：

  - 将 CPU 可用的数字打包（pack）进内存缓冲区（带硬件约束/怪异行为）
  - 将内存缓冲区（带硬件约束/怪异行为）解包（unpack）为 CPU 可用的数字。

该 API 对上述硬件约束和怪异行为、对 CPU 字节序，从而也对两者间可能的不匹配，提供了一层
抽象。

这些 API 函数的基本单位是 u64。从 CPU 的角度看，第 63 位始终意味着字节 7 的第 7 位偏移
（尽管仅在逻辑上）。问题是：我们在内存中把这个位放在哪里？

以下示例涵盖了打包后的 u64 字段的内存布局。打包缓冲区中的字节偏移量总是隐式地为 0、1、...、7。
示例展示的是逻辑字节和位所在的位置。

1. 正常情况（无怪异行为），我们这样处理：

```

  63 62 61 60 59 58 57 56 55 54 53 52 51 50 49 48 47 46 45 44 43 42 41 40 39 38 37 36 35 34 33 32
  7                       6                       5                        4
  31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
  3                       2                       1                        0

```

也就是说，CPU 可用的 u64 的最高字节（MSByte，7）位于内存偏移量 0 处，而 u64 的最低字节
（LSByte，0）位于内存偏移量 7 处。这对应于大多数人所说的“大端（big endian）”，其中位 i
对应数字 2^i。代码注释中也将此称为“logical（逻辑）”记法。

2. 如果设置了 QUIRK_MSB_ON_THE_RIGHT，我们这样处理：

```

  56 57 58 59 60 61 62 63 48 49 50 51 52 53 54 55 40 41 42 43 44 45 46 47 32 33 34 35 36 37 38 39
  7                       6                        5                       4
  24 25 26 27 28 29 30 31 16 17 18 19 20 21 22 23  8  9 10 11 12 13 14 15  0  1  2  3  4  5  6  7
  3                       2                        1                       0

```

也就是说，QUIRK_MSB_ON_THE_RIGHT 不影响字节定位，但会反转字节内部的位偏移。

3. 如果设置了 QUIRK_LITTLE_ENDIAN，我们这样处理：

```

  39 38 37 36 35 34 33 32 47 46 45 44 43 42 41 40 55 54 53 52 51 50 49 48 63 62 61 60 59 58 57 56
  4                       5                       6                       7
  7  6  5  4  3  2  1  0  15 14 13 12 11 10  9  8 23 22 21 20 19 18 17 16 31 30 29 28 27 26 25 24
  0                       1                       2                       3

```

因此，QUIRK_LITTLE_ENDIAN 意味着在内存区域内，每个 4 字节字（word）中的每一个字节都位于
相对于该字边界的镜像位置。

4. 如果同时设置了 QUIRK_MSB_ON_THE_RIGHT 和 QUIRK_LITTLE_ENDIAN，我们这样处理：

```

  32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63
  4                       5                       6                       7
  0  1  2  3  4  5  6  7  8   9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
  0                       1                       2                       3


```

5. 如果仅设置了 QUIRK_LSW32_IS_FIRST，我们这样处理：

```

  31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
  3                       2                       1                        0
  63 62 61 60 59 58 57 56 55 54 53 52 51 50 49 48 47 46 45 44 43 42 41 40 39 38 37 36 35 34 33 32
  7                       6                       5                        4

```

在这种情况下，8 字节内存区域解释如下：前 4 字节对应于最低有效 4 字节字，接下来 4 字节
对应于更高有效的 4 字节字。

6. 如果设置了 QUIRK_LSW32_IS_FIRST 和 QUIRK_MSB_ON_THE_RIGHT，我们这样处理：

```

  24 25 26 27 28 29 30 31 16 17 18 19 20 21 22 23  8  9 10 11 12 13 14 15  0  1  2  3  4  5  6  7
  3                       2                        1                       0
  56 57 58 59 60 61 62 63 48 49 50 51 52 53 54 55 40 41 42 43 44 45 46 47 32 33 34 35 36 37 38 39
  7                       6                        5                       4


```

7. 如果设置了 QUIRK_LSW32_IS_FIRST 和 QUIRK_LITTLE_ENDIAN，它看起来像这样：

```

  7  6  5  4  3  2  1  0  15 14 13 12 11 10  9  8 23 22 21 20 19 18 17 16 31 30 29 28 27 26 25 24
  0                       1                       2                       3
  39 38 37 36 35 34 33 32 47 46 45 44 43 42 41 40 55 54 53 52 51 50 49 48 63 62 61 60 59 58 57 56
  4                       5                       6                       7


```

8. 如果设置了 QUIRK_LSW32_IS_FIRST、QUIRK_LITTLE_ENDIAN 和 QUIRK_MSB_ON_THE_RIGHT，它看起来
   像这样：

```

  0  1  2  3  4  5  6  7  8   9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
  0                       1                       2                       3
  32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63
  4                       5                       6                       7


```

我们总是以假设没有任何怪异行为的方式来思考偏移量，然后在访问内存区域之前进行转换。

### 关于缓冲区长度不是 4 的倍数的说明


为了处理每组 4 字节彼此之间以“小端”布局、但组内本身以“大端”布局的内存布局怪异行为，
“4 字节组”这个概念对打包 API 是内在固有的（不过不要与内存访问混淆，后者是逐字节进行的）。

对于长度不是 4 的倍数的缓冲区，这意味着会有一组是不完整的。根据怪异行为的不同，这可能
导致缓冲区中可访问的位域出现不连续。打包 API 假定这种不连续并非内存布局的本意，因此它通过
在逻辑上有效地将最高有效的 4 字节组缩短为实际可用的字节数，来避免这种不连续。

下面给出一个 31 字节大小缓冲区的示例。物理缓冲区偏移量是隐式的，并在组内从左到右、在列内
从上到下递增。

无怪异行为：

```

            31         29         28        |   Group 7 (最高有效)
 27         26         25         24        |   Group 6
 23         22         21         20        |   Group 5
 19         18         17         16        |   Group 4
 15         14         13         12        |   Group 3
 11         10          9          8        |   Group 2
  7          6          5          4        |   Group 1
  3          2          1          0        |   Group 0 (最低有效)

```

QUIRK_LSW32_IS_FIRST：

```

  3          2          1          0        |   Group 0 (最低有效)
  7          6          5          4        |   Group 1
 11         10          9          8        |   Group 2
 15         14         13         12        |   Group 3
 19         18         17         16        |   Group 4
 23         22         21         20        |   Group 5
 27         26         25         24        |   Group 6
 30         29         28                   |   Group 7 (最高有效)

```

QUIRK_LITTLE_ENDIAN：

```

            30         28         29        |   Group 7 (最高有效)
 24         25         26         27        |   Group 6
 20         21         22         23        |   Group 5
 16         17         18         19        |   Group 4
 12         13         14         15        |   Group 3
  8          9         10         11        |   Group 2
  4          5          6          7        |   Group 1
  0          1          2          3        |   Group 0 (最低有效)

```

QUIRK_LITTLE_ENDIAN | QUIRK_LSW32_IS_FIRST：

```

  0          1          2          3        |   Group 0 (最低有效)
  4          5          6          7        |   Group 1
  8          9         10         11        |   Group 2
 12         13         14         15        |   Group 3
 16         17         18         19        |   Group 4
 20         21         22         23        |   Group 5
 24         25         26         27        |   Group 6
 28         29         30                   |   Group 7 (最高有效)

```

### 预期用途


选择使用本 API 的驱动首先需要确定上述 8 种怪异行为组合（共 8 种）中哪一种与硬件文档描述
相符。

共有 3 种受支持的使用模式，详述如下。

##### packing()


此 API 函数已废弃。

packing() 函数返回一个以 int 编码的错误码，保护程序员避免错误的 API 使用。这些错误预期
不会在运行时发生，因此将 packing() 包装到一个返回 void 并吞掉这些错误的自定义函数中是合理
的。可选地，它可以打印栈回溯或打印错误描述。

  void my_packing(void **buf, u64 **val, int startbit, int endbit,
                  size_t len, enum packing_op op)
  {
          int err;

          /** 相应调整怪异行为 **/
          err = packing(buf, val, startbit, endbit, len, op, QUIRK_LSW32_IS_FIRST);
          if (likely(!err))
                  return;

          if (err == -EINVAL) {
                  pr_err("Start bit (%d) expected to be larger than end (%d)\n",
                         startbit, endbit);
          } else if (err == -ERANGE) {
                  if ((startbit - endbit + 1) > 64)
                          pr_err("Field %d-%d too large for 64 bits!\n",
                                 startbit, endbit);
                  else
                          pr_err("Cannot store %llx inside bits %d-%d (would truncate)\n",
                                 *val, startbit, endbit);
          }
          dump_stack();
  }

##### pack() 和 unpack()


这些是 packing() 的常量正确性（const-correct）变体，并去掉了最后的 “enum packing_op op”
参数。

调用 pack(...) 等价于（且更受推荐）调用 packing(..., PACK)。

调用 unpack(...) 等价于（且更受推荐）调用 packing(..., UNPACK)。

##### pack_fields() 和 unpack_fields()


该库针对缓冲区中存在许多字段的场景暴露了优化的函数，并鼓励消费驱动避免对每个字段重复调用
pack() 和 unpack()，而是使用 pack_fields() 和 unpack_fields()，这能减少代码体积。

这些 API 使用 `struct packed_field_u8` 或 `struct packed_field_u16` 数组中的字段定义，允许
消费驱动根据自身定制需求最小化这些数组的大小。

pack_fields() 和 unpack_fields() API 函数实际上是在编译时根据传入的字段数组类型自动选择
适当函数的宏。

相较于 pack() 和 unpack() 的额外好处是，对字段定义的健全性检查在编译时通过 `BUILD_BUG_ON`
处理，而不仅仅是在违规代码执行时。这些函数返回 void，无需包装它们来处理意外错误。

建议（但非强制）将你的打包缓冲区包装进一个具有固定大小的结构化类型。这通常使编译器更容易
强制使用正确大小的缓冲区。

以下是如何使用字段 API 的示例：

   /* 解包结构体内部的排序是灵活的，可以与打包缓冲区不同。
    - 此处为减少填充而进行优化。
    */
   struct data {
        u64 field3;
        u32 field4;
        u16 field1;
        u8 field2;
   };

   #define SIZE 13

   typedef struct __packed { u8 buf[SIZE]; } packed_buf_t;

   static const struct packed_field_u8 fields[] = {
           PACKED_FIELD(100, 90, struct data, field1),
           PACKED_FIELD(90, 87, struct data, field2),
           PACKED_FIELD(86, 30, struct data, field3),
           PACKED_FIELD(29, 0, struct data, field4),
   };

   void unpack_your_data(const packed_buf_t **buf, struct data **unpacked)
   {
           BUILD_BUG_ON(sizeof(*buf) != SIZE;

           unpack_fields(buf, sizeof(*buf), unpacked, fields,
                         QUIRK_LITTLE_ENDIAN);
   }

   void pack_your_data(const struct data **unpacked, packed_buf_t **buf)
   {
           BUILD_BUG_ON(sizeof(*buf) != SIZE;

           pack_fields(buf, sizeof(*buf), unpacked, fields,
                       QUIRK_LITTLE_ENDIAN);
   }
