## BPF Type Format（BTF，BPF 类型格式）


## 1. 简介


BTF（BPF Type Format，BPF 类型格式）是一种元数据格式，用于编码与 BPF
程序/映射（map）相关的调试信息。BTF 这个名字最初用于描述数据类型。后来 BTF
被扩展为同时包含已定义子例程的函数信息，以及源文件/行号信息。

这些调试信息可用于映射的友好打印（pretty print）、函数签名等。函数签名
使得 BPF 程序/函数的内核符号显示得更好。行号信息有助于生成带源码标注的
翻译后字节码、JIT 后代码以及验证器（verifier）日志。

BTF 规范包含两个部分：
  - BTF 内核 API
  - BTF ELF 文件格式

内核 API 是用户空间与内核之间的契约。内核在使用 BTF 信息之前会先对其
进行校验。ELF 文件格式则是 ELF 文件与 libbpf 加载器之间的用户空间契约。

类型（type）和字符串（string）段属于 BTF 内核 API 的一部分，描述了被 BPF
程序引用的调试信息（主要是与类型相关的信息）。这两段在 BTF_Type_String
中有详细讨论。


## 2. BTF 类型与字符串编码


文件 `include/uapi/linux/btf.h` 提供了类型/字符串如何编码的高层定义。

```

    struct btf_header {
        __u16   magic;
        __u8    version;
        __u8    flags;
        __u32   hdr_len;

        /* 所有偏移量都以字节为单位，相对于本头部末尾 */
        __u32   type_off;       /* 类型段偏移       */
        __u32   type_len;       /* 类型段长度       */
        __u32   str_off;        /* 字符串段偏移     */
        __u32   str_len;        /* 字符串段长度     */
    };

```
magic 为 `0xeB9F`，在大端和小端系统上编码不同，可用于测试 BTF 是为大端
还是小端目标生成的。设计 `btf_header` 时保留了可扩展性，当生成数据块时
`hdr_len` 等于 `sizeof(struct btf_header)`。

### 2.1 字符串编码


字符串段中的第一个字符串必须是空字符串。字符串表的其余部分是其他
以 null 结尾的字符串的拼接。

### 2.2 类型编码


类型 id `0` 保留给 `void` 类型。类型段被顺序解析，类型 id 从 1 开始
依次分配给每个被识别出的类型。
```

    #define BTF_KIND_INT            1       /* Integer      */
    #define BTF_KIND_PTR            2       /* Pointer      */
    #define BTF_KIND_ARRAY          3       /* Array        */
    #define BTF_KIND_STRUCT         4       /* Struct       */
    #define BTF_KIND_UNION          5       /* Union        */
    #define BTF_KIND_ENUM           6       /* Enumeration up to 32-bit values */
    #define BTF_KIND_FWD            7       /* Forward      */
    #define BTF_KIND_TYPEDEF        8       /* Typedef      */
    #define BTF_KIND_VOLATILE       9       /* Volatile     */
    #define BTF_KIND_CONST          10      /* Const        */
    #define BTF_KIND_RESTRICT       11      /* Restrict     */
    #define BTF_KIND_FUNC           12      /* Function     */
    #define BTF_KIND_FUNC_PROTO     13      /* Function Proto       */
    #define BTF_KIND_VAR            14      /* Variable     */
    #define BTF_KIND_DATASEC        15      /* Section      */
    #define BTF_KIND_FLOAT          16      /* Floating point       */
    #define BTF_KIND_DECL_TAG       17      /* Decl Tag     */
    #define BTF_KIND_TYPE_TAG       18      /* Type Tag     */
    #define BTF_KIND_ENUM64         19      /* Enumeration up to 64-bit values */

```
注意类型段编码的是调试信息，而不仅仅是纯粹的类型。`BTF_KIND_FUNC` 不是
一个类型，它表示一个已定义的子程序。

```

    struct btf_type {
        __u32 name_off;
        /* "info" 位的排布
         * bits  0-15: vlen（例如 struct 的成员数）
         * bits 16-23: 未使用
         * bits 24-28: kind（例如 int、ptr、array……等）
         * bits 29-30: 未使用
         * bit     31: kind_flag，目前用于
         *             struct、union、enum、fwd、enum64、
         *             decl_tag 和 type_tag
         */
        __u32 info;
        /* "size" 用于 INT、ENUM、STRUCT、UNION 和 ENUM64。
         * "size" 表示它所描述类型的大小。
         *
         * "type" 用于 PTR、TYPEDEF、VOLATILE、CONST、RESTRICT、
         * FUNC、FUNC_PROTO、DECL_TAG 和 TYPE_TAG。
         * "type" 是一个指向另一个类型的 type_id。
         */
        union {
                __u32 size;
                __u32 type;
        };
    };

```
对于某些 kind，公共数据之后会跟随该 kind 特有的数据。`struct btf_type`
中的 `name_off` 指定了在字符串表中的偏移量。以下各小节详细说明了每种
kind 的编码。

#### 2.2.1 BTF_KIND_INT


`struct btf_type` 编码要求：
 - `name_off`：任意有效偏移
 - `info.kind_flag`：0
 - `info.kind`：BTF_KIND_INT
 - `info.vlen`：0
 - `size`：int 类型的大小（字节数）

```

  #define BTF_INT_ENCODING(VAL)   (((VAL) & 0x0f000000) >> 24)
  #define BTF_INT_OFFSET(VAL)     (((VAL) & 0x00ff0000) >> 16)
  #define BTF_INT_BITS(VAL)       ((VAL)  & 0x000000ff)

```
```

  #define BTF_INT_SIGNED  (1 << 0)
  #define BTF_INT_CHAR    (1 << 1)
  #define BTF_INT_BOOL    (1 << 2)

```
`BTF_INT_ENCODING()` 提供额外信息：有符号性（signedness）、char 或
bool，针对 int 类型。char 和 bool 编码主要用于友好打印。int 类型
最多只能指定一种编码。

`BTF_INT_BITS()` 指定该 int 类型所持有的实际位数。例如，一个 4 位的位域
编码为 `BTF_INT_BITS()` 等于 4。`btf_type.size * 8` 必须
大于或等于该类型的 `BTF_INT_BITS()`。`BTF_INT_BITS()` 的最大值为 128。

`BTF_INT_OFFSET()` 指定计算该 int 值时的起始位偏移。例如，一个位域
struct 成员具有：

 - btf 成员相对结构体起始的位偏移为 100，
 - btf 成员指向一个 int 类型，
 - 该 int 类型的 `BTF_INT_OFFSET() = 2` 且 `BTF_INT_BITS() = 4`

那么在结构体内存布局中，该成员将占据从位 `100 + 2 = 102` 开始的 `4` 位。

另一种方式是，下面的位域 struct 成员可以访问与上述相同的位：

 - btf 成员位偏移为 102，
 - btf 成员指向一个 int 类型，
 - 该 int 类型的 `BTF_INT_OFFSET() = 0` 且 `BTF_INT_BITS() = 4`

`BTF_INT_OFFSET()` 的原始意图是为了提供位域编码的灵活性。目前，llvm 和
pahole 对所有 int 类型都生成 `BTF_INT_OFFSET() = 0`。

#### 2.2.2 BTF_KIND_PTR


`struct btf_type` 编码要求：
  - `name_off`：0
  - `info.kind_flag`：0
  - `info.kind`：BTF_KIND_PTR
  - `info.vlen`：0
  - `type`：该指针所指向的类型

`btf_type` 之后没有额外的类型数据。

#### 2.2.3 BTF_KIND_ARRAY


`struct btf_type` 编码要求：
  - `name_off`：0
  - `info.kind_flag`：0
  - `info.kind`：BTF_KIND_ARRAY
  - `info.vlen`：0
  - `size/type`：0，未使用

```

    struct btf_array {
        __u32   type;
        __u32   index_type;
        __u32   nelems;
    };

```
`struct btf_array` 的编码：
  - `type`：元素类型
  - `index_type`：索引类型
  - `nelems`：该数组的元素个数（`0` 也允许）

`index_type` 可以是任意常规 int 类型（`u8`、`u16`、`u32`、`u64`、
`unsigned __int128`）。包含 `index_type` 的原始设计遵循 DWARF，因为
DWARF 的数组类型也带有一个 `index_type`。目前在 BTF 中，除了类型校验之外，
`index_type` 并未被使用。

`struct btf_array` 通过元素类型链化以支持多维数组。例如，对于
`int a[^5^][^6^]`，下面类型信息展示了链化过程：

  - [^1^]：int
  - [^2^]：array，`btf_array.type = [^1^]`，`btf_array.nelems = 6`
  - [^3^]：array，`btf_array.type = [^2^]`，`btf_array.nelems = 5`

目前，pahole 和 llvm 都会把多维数组折叠成一维数组，例如对于
`a[^5^][^6^]`，`btf_array.nelems` 等于 `30`。这是因为最初的使用场景是
map 友好打印，在那里整个数组都被 dump 出来，所以一维数组就足够了。随着
更多 BTF 用途被挖掘，pahole 和 llvm 可以改为生成针对多维数组的恰当链化
表示。

#### 2.2.4 BTF_KIND_STRUCT

#### 2.2.5 BTF_KIND_UNION


`struct btf_type` 编码要求：
  - `name_off`：0 或指向一个有效 C 标识符的偏移
  - `info.kind_flag`：0 或 1
  - `info.kind`：BTF_KIND_STRUCT 或 BTF_KIND_UNION
  - `info.vlen`：struct/union 成员的个数
  - `info.size`：struct/union 的大小（字节数）

```

    struct btf_member {
        __u32   name_off;
        __u32   type;
        __u32   offset;
    };

```
`struct btf_member` 编码：
  - `name_off`：指向一个有效 C 标识符的偏移
  - `type`：成员类型
  - `offset`：<见下文>

如果类型信息 `kind_flag` 未置位，offset 只含有该成员的位偏移。注意位域的
基类型只能是 int 或 enum 类型。如果位域大小为 32，基类型可以是 int 或
enum 类型。如果位域大小不为 32，基类型必须是 int，且 int 类型的
`BTF_INT_BITS()` 编码了位域大小。

如果 `kind_flag` 被置位，`btf_member.offset` 同时包含成员的位域大小和位偏移。
位域大小和位偏移按如下方式计算：
```

  #define BTF_MEMBER_BITFIELD_SIZE(val)   ((val) >> 24)
  #define BTF_MEMBER_BIT_OFFSET(val)      ((val) & 0xffffff)

```
在这种情况下，如果基类型是 int 类型，它必须是常规 int 类型：

  - `BTF_INT_OFFSET()` 必须为 0。
  - `BTF_INT_BITS()` 必须等于 `{1,2,4,8,16} * 8`。

commit 9d5f9f701b18 引入了 `kind_flag`，并解释了为何两种模式并存。

#### 2.2.6 BTF_KIND_ENUM


`struct btf_type` 编码要求：
  - `name_off`：0 或指向一个有效 C 标识符的偏移
  - `info.kind_flag`：无符号为 0，有符号为 1
  - `info.kind`：BTF_KIND_ENUM
  - `info.vlen`：enum 值的个数
  - `size`：1/2/4/8

```

    struct btf_enum {
        __u32   name_off;
        __s32   val;
    };

```
`btf_enum` 编码：
  - `name_off`：指向一个有效 C 标识符的偏移
  - `val`：任意值

如果原始 enum 值是有符号的且大小小于 4，该值会被符号扩展为 4 字节。如果
大小为 8，该值会被截断为 4 字节。

#### 2.2.7 BTF_KIND_FWD


`struct btf_type` 编码要求：
  - `name_off`：指向一个有效 C 标识符的偏移
  - `info.kind_flag`：struct 为 0，union 为 1
  - `info.kind`：BTF_KIND_FWD
  - `info.vlen`：0
  - `type`：0

`btf_type` 之后没有额外的类型数据。

#### 2.2.8 BTF_KIND_TYPEDEF


`struct btf_type` 编码要求：
  - `name_off`：指向一个有效 C 标识符的偏移
  - `info.kind_flag`：0
  - `info.kind`：BTF_KIND_TYPEDEF
  - `info.vlen`：0
  - `type`：`name_off` 处名字所能指代的类型

`btf_type` 之后没有额外的类型数据。

#### 2.2.9 BTF_KIND_VOLATILE


`struct btf_type` 编码要求：
  - `name_off`：0
  - `info.kind_flag`：0
  - `info.kind`：BTF_KIND_VOLATILE
  - `info.vlen`：0
  - `type`：带有 `volatile` 限定的类型

`btf_type` 之后没有额外的类型数据。

#### 2.2.10 BTF_KIND_CONST


`struct btf_type` 编码要求：
  - `name_off`：0
  - `info.kind_flag`：0
  - `info.kind`：BTF_KIND_CONST
  - `info.vlen`：0
  - `type`：带有 `const` 限定的类型

`btf_type` 之后没有额外的类型数据。

#### 2.2.11 BTF_KIND_RESTRICT


`struct btf_type` 编码要求：
  - `name_off`：0
  - `info.kind_flag`：0
  - `info.kind`：BTF_KIND_RESTRICT
  - `info.vlen`：0
  - `type`：带有 `restrict` 限定的类型

`btf_type` 之后没有额外的类型数据。

#### 2.2.12 BTF_KIND_FUNC


`struct btf_type` 编码要求：
  - `name_off`：指向一个有效 C 标识符的偏移
  - `info.kind_flag`：0
  - `info.kind`：BTF_KIND_FUNC
  - `info.vlen`：链接信息（BTF_FUNC_STATIC、BTF_FUNC_GLOBAL
                   或 BTF_FUNC_EXTERN —— 见 BTF_Function_Linkage_Constants）
  - `type`：一个 BTF_KIND_FUNC_PROTO 类型

`btf_type` 之后没有额外的类型数据。

BTF_KIND_FUNC 定义的不是一个类型，而是一个子程序（函数），其签名由 `type`
定义。因此该子程序是那个类型的一个实例。BTF_KIND_FUNC 反过来又可能被
BTF_Ext_Section（ELF）中的 func_info 或 BPF_Prog_Load 的参数（ABI）所
引用。

目前，内核只支持 BTF_FUNC_STATIC 和 BTF_FUNC_GLOBAL 这两种链接值。

#### 2.2.13 BTF_KIND_FUNC_PROTO


`struct btf_type` 编码要求：
  - `name_off`：0
  - `info.kind_flag`：0
  - `info.kind`：BTF_KIND_FUNC_PROTO
  - `info.vlen`：参数个数
  - `type`：返回类型

```

    struct btf_param {
        __u32   name_off;
        __u32   type;
    };

```
如果一个 BTF_KIND_FUNC_PROTO 类型被某个 BTF_KIND_FUNC 类型引用，那么
`btf_param.name_off` 必须指向一个有效的 C 标识符，可能的最后一个表示可变
参数的参数除外。`btf_param.type` 指向参数类型。

如果函数带有可变参数，最后一个参数编码为 `name_off = 0` 且 `type = 0`。

#### 2.2.14 BTF_KIND_VAR


`struct btf_type` 编码要求：
  - `name_off`：指向一个有效 C 标识符的偏移
  - `info.kind_flag`：0
  - `info.kind`：BTF_KIND_VAR
  - `info.vlen`：0
  - `type`：变量的类型

`btf_type` 之后跟随一个单独的 `struct btf_variable`，其
```

    struct btf_var {
        __u32   linkage;
    };

```
`btf_var.linkage` 可取以下值：BTF_VAR_STATIC、BTF_VAR_GLOBAL_ALLOCATED 或
BTF_VAR_GLOBAL_EXTERN —— 见 BTF_Var_Linkage_Constants。

目前 LLVM 并非支持所有类型的全局变量。当前可用的是：

  - 带或不带 section 属性的静态变量
  - 带 section 属性的全局变量

后者用于将来从 map 定义中抽取 map 键/值类型 id。

#### 2.2.15 BTF_KIND_DATASEC


`struct btf_type` 编码要求：
  - `name_off`：指向与某个变量相关联的有效名字的偏移，或为
                  .data/.bss/.rodata 之一
  - `info.kind_flag`：0
  - `info.kind`：BTF_KIND_DATASEC
  - `info.vlen`：变量个数
  - `size`：段总大小（字节数，编译时为 0，由 libbpf 等 BPF 加载器
              补丁为实际大小）

```

    struct btf_var_secinfo {
        __u32   type;
        __u32   offset;
        __u32   size;
    };

```
`struct btf_var_secinfo` 编码：
  - `type`：BTF_KIND_VAR 变量的类型
  - `offset`：变量在段内的偏移
  - `size`：变量大小（字节数）

#### 2.2.16 BTF_KIND_FLOAT


`struct btf_type` 编码要求：
 - `name_off`：任意有效偏移
 - `info.kind_flag`：0
 - `info.kind`：BTF_KIND_FLOAT
 - `info.vlen`：0
 - `size`：float 类型的大小（字节数）：2、4、8、12 或 16。

`btf_type` 之后没有额外的类型数据。

#### 2.2.17 BTF_KIND_DECL_TAG


`struct btf_type` 编码要求：
 - `name_off`：指向一个非空字符串的偏移
 - `info.kind_flag`：0 或 1
 - `info.kind`：BTF_KIND_DECL_TAG
 - `info.vlen`：0
 - `type`：`struct`、`union`、`func`、`var` 或 `typedef`

```

    struct btf_decl_tag {
        __u32   component_idx;
    };

```
`type` 应为 `struct`、`union`、`func`、`var` 或 `typedef`。对于 `var` 或
`typedef` 类型，`btf_decl_tag.component_idx` 必须为 `-1`。对于另外三种类型，
如果 btf_decl_tag 属性应用于 `struct`、`union` 或 `func` 自身，
`btf_decl_tag.component_idx` 必须为 `-1`。否则，该属性应用于某个
`struct`/`union` 成员或某个 `func` 参数，`btf_decl_tag.component_idx` 应为
一个有效索引（从 0 开始），指向某个成员或参数。

如果 `info.kind_flag` 为 0，则这是一个普通的 decl tag，`name_off` 编码的是
btf_decl_tag 属性字符串。

如果 `info.kind_flag` 为 1，则该 decl tag 表示任意的 `__attribute__`。在这种
情况下，`name_off` 编码的是一个代表属性说明符（attribute specifier）属性列表
的字符串。例如，对于 `__attribute__((aligned(4)))`，字符串内容为 `aligned(4)`。

#### 2.2.18 BTF_KIND_TYPE_TAG


`struct btf_type` 编码要求：
 - `name_off`：指向一个非空字符串的偏移
 - `info.kind_flag`：0 或 1
 - `info.kind`：BTF_KIND_TYPE_TAG
 - `info.vlen`：0
 - `type`：带有 `btf_type_tag` 属性的类型

目前，`BTF_KIND_TYPE_TAG` 仅针对指针类型生成。它具有如下 btf 类型链：
```

  ptr -> [type_tag]*
      -> [const | volatile | restrict | typedef]*
      -> base_type

```
基本上，一个指针类型指向零个或多个 type_tag，然后是零个或多个
const/volatile/restrict/typedef，最后是基类型。基类型是 int、ptr、
array、struct、union、enum、func_proto 和 float 类型之一。

与 decl tag 类似，如果 `info.kind_flag` 为 0，则这是一个普通的 type tag，
`name_off` 编码的是 btf_type_tag 属性字符串。

如果 `info.kind_flag` 为 1，则该 type tag 表示任意的 `__attribute__`，
`name_off` 编码的是一个代表属性说明符属性列表的字符串。

#### 2.2.19 BTF_KIND_ENUM64


`struct btf_type` 编码要求：
  - `name_off`：0 或指向一个有效 C 标识符的偏移
  - `info.kind_flag`：无符号为 0，有符号为 1
  - `info.kind`：BTF_KIND_ENUM64
  - `info.vlen`：enum 值的个数
  - `size`：1/2/4/8

```

    struct btf_enum64 {
        __u32   name_off;
        __u32   val_lo32;
        __u32   val_hi32;
    };

```
`btf_enum64` 编码：
  - `name_off`：指向一个有效 C 标识符的偏移
  - `val_lo32`：64 位值的低 32 位
  - `val_hi32`：64 位值的高 32 位

如果原始 enum 值是有符号的且大小小于 8，该值会被符号扩展为 8 字节。

### 2.3 常量值


#### 2.3.1 函数链接常量值


  ===================  =====  ===========
  kind                 value  description
  ===================  =====  ===========
  `BTF_FUNC_STATIC`  0x0    子程序定义，在所属编译单元之外不可见
  `BTF_FUNC_GLOBAL`  0x1    子程序定义，在所属编译单元之外可见
  `BTF_FUNC_EXTERN`  0x2    子程序声明，其定义在所属编译单元之外
  ===================  =====  ===========

#### 2.3.2 变量链接常量值


  ============================  =====  ===========
  kind                          value  description
  ============================  =====  ===========
  `BTF_VAR_STATIC`            0x0    全局变量定义，在所属编译单元之外不可见
  `BTF_VAR_GLOBAL_ALLOCATED`  0x1    全局变量定义，在所属编译单元之外可见
  `BTF_VAR_GLOBAL_EXTERN`     0x2    全局变量声明，其定义在所属编译单元之外
  ============================  =====  ===========

## 3. BTF 内核 API


以下 bpf 系统调用命令涉及 BTF：
   - BPF_BTF_LOAD：将一块 BTF 数据加载进内核
   - BPF_MAP_CREATE：创建带 btf 键和值类型信息的 map
   - BPF_PROG_LOAD：带 btf 函数和行号信息加载程序
   - BPF_BTF_GET_FD_BY_ID：获取一个 btf 文件描述符（fd）
   - BPF_OBJ_GET_INFO_BY_FD：返回 btf、func_info、line_info
     及其他 btf 相关信息

典型的工作流程如下：
```

  Application:
      BPF_BTF_LOAD
          |
          v
      BPF_MAP_CREATE and BPF_PROG_LOAD
          |
          V
      ......

  Introspection tool:
      ......
      BPF_{PROG,MAP}_GET_NEXT_ID (获取 prog/map 的 id)
          |
          V
      BPF_{PROG,MAP}_GET_FD_BY_ID (获取一个 prog/map 的 fd)
          |
          V
      BPF_OBJ_GET_INFO_BY_FD (用 btf_id 获取 bpf_prog_info/bpf_map_info)
          |                                     |
          V                                     |
      BPF_BTF_GET_FD_BY_ID (获取 btf_fd)         |
          |                                     |
          V                                     |
      BPF_OBJ_GET_INFO_BY_FD (获取 btf)          |
          |                                     |
          V                                     V
      pretty print 类型、dump 函数签名与行号信息等

```
### 3.1 BPF_BTF_LOAD


将一块 BTF 数据加载进内核。一块数据（如 BTF_Type_String 所述）可以直接加载
进内核。会向用户空间返回一个 `btf_fd`。

### 3.2 BPF_MAP_CREATE


```

    __u32   btf_fd;         /* 指向 BTF 类型数据的 fd */
    __u32   btf_key_type_id;        /* 键的 BTF type_id */
    __u32   btf_value_type_id;      /* 值的 BTF type_id */

```
在 libbpf 中，可以像下面这样用额外注解来定义 map：
```

    struct {
        __uint(type, BPF_MAP_TYPE_ARRAY);
        __type(key, int);
        __type(value, struct ipv_counts);
        __uint(max_entries, 4);
    } btf_map SEC(".maps");

```
在 ELF 解析期间，libbpf 能够抽出键/值 type_id 并自动赋值给 BPF_MAP_CREATE
的属性。


### 3.3 BPF_PROG_LOAD


在 prog_load 期间，可以将 func_info 和 line_info 连同以下属性的恰当取值
传入内核：
```

    __u32           insn_cnt;
    __aligned_u64   insns;
    ......
    __u32           prog_btf_fd;    /* 指向 BTF 类型数据的 fd */
    __u32           func_info_rec_size;     /* 用户空间 bpf_func_info 大小 */
    __aligned_u64   func_info;      /* func 信息 */
    __u32           func_info_cnt;  /* bpf_func_info 记录数 */
    __u32           line_info_rec_size;     /* 用户空间 bpf_line_info 大小 */
    __aligned_u64   line_info;      /* line 信息 */
    __u32           line_info_cnt;  /* bpf_line_info 记录数 */

```
```

    struct bpf_func_info {
        __u32   insn_off; /* [0, insn_cnt - 1] */
        __u32   type_id;  /* 指向一个 BTF_KIND_FUNC 类型 */
    };
    struct bpf_line_info {
        __u32   insn_off; /* [0, insn_cnt - 1] */
        __u32   file_name_off; /* 指向文件名的字符串表偏移 */
        __u32   line_off; /* 指向源码行的字符串表偏移 */
        __u32   line_col; /* 行号与列号 */
    };

```
func_info_rec_size 是每条 func_info 记录的大小，line_info_rec_size 是每条
line_info 记录的大小。将记录大小传给内核，使得将来扩展记录本身成为可能。

以下是 func_info 的要求：
  - func_info[^0^].insn_off 必须为 0。
  - func_info 的 insn_off 必须严格递增，并且与 bpf 函数边界匹配。

以下是 line_info 的要求：
  - 每个函数中的第一条指令必须有一条指向它的 line_info 记录。
  - line_info 的 insn_off 必须严格递增。

对于 line_info，行号和列号定义如下：
```

    #define BPF_LINE_INFO_LINE_NUM(line_col)        ((line_col) >> 10)
    #define BPF_LINE_INFO_LINE_COL(line_col)        ((line_col) & 0x3ff)

```
### 3.4 BPF_{PROG,MAP}_GET_NEXT_ID


在内核中，每个被加载的程序、map 或 btf 都有一个唯一 id。该 id 在程序、map
或 btf 的生命周期内不会改变。

bpf 系统调用命令 BPF_{PROG,MAP}_GET_NEXT_ID 会分别返回 bpf 程序或 map 的所有
id（每个命令一个），交给用户空间，以便一个内省工具可以检查所有的程序和 map。

### 3.5 BPF_{PROG,MAP}_GET_FD_BY_ID


内省工具无法直接使用 id 来获取程序或 map 的详细信息。需要先获取一个文件
描述符，以便进行引用计数。

### 3.6 BPF_OBJ_GET_INFO_BY_FD


一旦拿到程序/map 的 fd，内省工具就可以从内核获取关于该 fd 的详细信息，其中
一些与 BTF 相关。例如，`bpf_map_info` 返回 `btf_id` 以及键/值类型 id。
`bpf_prog_info` 返回 `btf_id`、func_info，以及翻译后的 bpf 字节码的 line info
和 jited_line_info。

### 3.7 BPF_BTF_GET_FD_BY_ID


借助在 `bpf_map_info` 和 `bpf_prog_info` 中获取的 `btf_id`，bpf 系统调用命令
BPF_BTF_GET_FD_BY_ID 可以取出一个 btf fd。然后，通过命令 BPF_OBJ_GET_INFO_BY_FD，
可以把最初用 BPF_BTF_LOAD 加载进内核的 btf 数据块取回。

拥有了 btf 数据块、`bpf_map_info` 和 `bpf_prog_info`，内省工具就掌握了完整的
btf 知识，能够友好打印 map 的键/值、dump 函数签名与行号信息，以及字节码/JIT
代码。

## 4. ELF 文件格式接口


### 4.1 .BTF 段


.BTF 段包含类型和字符串数据。该段的格式与 BTF_Type_String 中描述的相同。


### 4.2 .BTF.ext 段


.BTF.ext 段编码 func_info、line_info 以及 CO-RE 重定位信息，这些内容在加载进
内核之前需要加载器进行处理。

.BTF.ext 段的规范定义于 `tools/lib/bpf/btf.h` 和 `tools/lib/bpf/btf.c`。

```

    struct btf_ext_header {
        __u16   magic;
        __u8    version;
        __u8    flags;
        __u32   hdr_len;

        /* 所有偏移量都以字节为单位，相对于本头部末尾 */
        __u32   func_info_off;
        __u32   func_info_len;
        __u32   line_info_off;
        __u32   line_info_len;

        /* .BTF.ext 头部的可选部分 */
        __u32   core_relo_off;
        __u32   core_relo_len;
    };

```
它与 .BTF 段非常相似。它不包含类型/字符串段，而是包含 func_info、line_info
和 core_relo 子段。关于 func_info 和 line_info 记录格式的详情，见 BPF_Prog_Load。

```

     func_info_rec_size              /* __u32 值 */
     btf_ext_info_sec for section #1 /* section #1 的 func_info */
     btf_ext_info_sec for section #2 /* section #2 的 func_info */
     ...

```
`func_info_rec_size` 指定生成 .BTF.ext 时 `bpf_func_info` 结构的大小。
`btf_ext_info_sec`（定义如下）是一个集合：
```

     struct btf_ext_info_sec {
        __u32   sec_name_off; /* 段名偏移 */
        __u32   num_info;
        /* 紧跟着 num_info * record_size 个字节 */
        __u8    data[0];
     };

```
此处 num_info 必须大于 0。

```

     line_info_rec_size              /* __u32 值 */
     btf_ext_info_sec for section #1 /* section #1 的 line_info */
     btf_ext_info_sec for section #2 /* section #2 的 line_info */
     ...

```
`line_info_rec_size` 指定生成 .BTF.ext 时 `bpf_line_info` 结构的大小。

`bpf_func_info->insn_off` 和 `bpf_line_info->insn_off` 在“内核 API”与“ELF API”
中的解释不同。对于内核 API，`insn_off` 是以 ``struct bpf_insn` 为单位的指令
偏移。对于 ELF API，`insn_off` 是从段开头算起的字节偏移
（`btf_ext_info_sec->sec_name_off`）。

```

     core_relo_rec_size              /* __u32 值 */
     btf_ext_info_sec for section #1 /* section #1 的 core_relo */
     btf_ext_info_sec for section #2 /* section #2 的 core_relo */

```
`core_relo_rec_size` 指定生成 .BTF.ext 时 `bpf_core_relo` 结构的大小。单个
`btf_ext_info_sec` 内的所有 `bpf_core_relo` 结构描述应用于由
`btf_ext_info_sec->sec_name_off` 命名的段上的重定位。

详见 Documentation/bpf/llvm_reloc.rst <btf-co-re-relocations>
了解关于 CO-RE 重定位的更多信息。

### 4.3 .BTF_ids 段


.BTF_ids 段编码内核中使用的 BTF ID 值。

该段在内核编译期间借助 `include/linux/btf_ids.h` 头文件中定义的宏创建。内核
代码可以用它们来创建 BTF ID 值的列表和集合（有序列表）。

`BTF_ID_LIST` 和 `BTF_ID` 宏定义无序的 BTF ID 值列表，
```

  BTF_ID_LIST(list)
  BTF_ID(type1, name1)
  BTF_ID(type2, name2)

```
```

  __BTF_ID__type1__name1__1:
  .zero 4
  __BTF_ID__type2__name2__2:
  .zero 4

```
定义了 `u32 list[];` 变量来访问该列表。

`BTF_ID_UNUSED` 宏定义 4 个零字节。当我们需要占位时使用它，例如
```

      BTF_ID_LIST(bpf_skb_output_btf_ids)
      BTF_ID(struct, sk_buff)
      BTF_ID_UNUSED
      BTF_ID(struct, task_struct)

```
`BTF_SET_START/END` 宏对定义有序的 BTF ID 值集合
```

  BTF_SET_START(set)
  BTF_ID(type1, name1)
  BTF_ID(type2, name2)
  BTF_SET_END(set)

```
```

  __BTF_ID__set__set:
  .zero 4
  __BTF_ID__type1__name1__3:
  .zero 4
  __BTF_ID__type2__name2__4:
  .zero 4

```
定义了 `struct btf_id_set set;` 变量来访问该列表。

```

   struct, union, typedef, func

```
并在解析 BTF ID 值时作为过滤器使用。

所有的 BTF ID 列表和集合都被编译进 .BTF_ids 段，并在内核构建的链接阶段由
`resolve_btfids` 工具解析。

### 4.4 .BTF.base 段


拆分 BTF（Split BTF）——其中 .BTF 段只包含不在关联的基础 .BTF 段中的类型——是
编码内核模块类型信息的一种极其高效的方式，因为内核模块通常由少量模块专属类型
加上大量共享的内核类型组成。前者编码在拆分 BTF 中，而后者编码在基础 BTF 中，
从而得到更紧凑的表示。拆分 BTF 中指向基础 BTF 中某个类型的类型，使用其基础 BTF
ID 来引用它，而拆分 BTF 的 ID 从 last_base_BTF_ID + 1 开始。

然而这种做法的缺点是让拆分 BTF 有点脆弱——当基础 BTF 发生变化时，基础 BTF ID
引用就不再有效，拆分 BTF 本身也就毫无用处了。.BTF.base 段的作用就是让拆分 BTF
在面对基础 BTF 可能变化的情况下更具韧性，内核模块并非每次都随内核一起构建的
情形正是如此。.BTF.base 包含有名字的基础类型：INT、FLOAT、STRUCT、UNION、
ENUM[^64^] 和 FWD。INT 和 FLOAT 在 .BTF.base 段中被完整描述，而像 struct 和
union 这样的复合类型则未被完整定义——.BTF.base 类型仅作为拆分 BTF 所指类型的
描述，因此 struct/union 在 .BTF.base 段中有 0 个成员。ENUM[^64^] 同样以 0 个
成员记录。任何其他类型都被加入拆分 BTF。这一“蒸馏”过程最终得到一个带有此类
最小化基础类型描述的 .BTF.base 段，以及一个引用那些基础类型的 .BTF 拆分段。之后，
我们可以结合 .BTF.base 段中存储的信息和新的 .BTF 基础段来对拆分 BTF 进行重定位；
.BTF.base 段中的类型信息让我们能够更新拆分 BTF 的引用，使其指向对应的新基础 BTF
ID。

BTF 重定位在内核模块加载时发生（当内核模块带有 .BTF.base 段时），libbpf 也提供
了 btf__relocate() API 来完成此事。

```

      [1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED
      [2] STRUCT 'foo' size=8 vlen=2
              'f1' type_id=1 bits_offset=0
              'f2' type_id=1 bits_offset=32

```
```

      [3] PTR '(anon)' type_id=2

```
即拆分 BTF 描述了一个指向 struct foo { int f1; int f2 }; 的指针

```

      [1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED
      [2] STRUCT 'foo' size=8 vlen=0

```
```

      [1] INT 'long unsigned int' size=8 bits_offset=0 nr_bits=64 encoding=(none)
      [2] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED
      [3] STRUCT 'foo' size=8 vlen=2
              'f1' type_id=2 bits_offset=0
              'f2' type_id=2 bits_offset=32

```
……我们可以利用我们的 .BTF.base 描述来知道该拆分 BTF 引用
```

      [4] PTR '(anon)' type_id=3

```
注意我们不得不更新拆分 BTF 的 BTF ID 和起始 BTF ID。

由此可见 .BTF.base 如何起到促进后续重定位的作用，从而带来更具韧性的拆分 BTF。

.BTF.base 段会在树外（out-of-tree）内核模块构建时自动生成——即设置了 KBUILD_EXTMOD
的情形（就像 "make M=path/2/mod" 那样）。.BTF.base 的生成需要 pahole 对
"distilled_base" BTF 特性的支持；这在 pahole v1.28 及更高版本中可用。

## 5. 使用 BTF


### 5.1 bpftool map 友好打印


借助 BTF，map 的键/值可以基于字段打印，而非简单地按原始字节打印。这对于大型
结构体或者当你的数据
```

      enum A { A1, A2, A3, A4, A5 };
      typedef enum A ___A;
      struct tmp_t {
           char a1:4;
           int  a2:4;
           int  :4;
           __u32 a3:4;
           int b;
           ___A b1:4;
           enum A b2:4;
      };
      struct {
           __uint(type, BPF_MAP_TYPE_ARRAY);
           __type(key, int);
           __type(value, struct tmp_t);
           __uint(max_entries, 1);
      } tmpmap SEC(".maps");

```
bpftool 能够像下面这样友好打印：
```

      [{
            "key": 0,
            "value": {
                "a1": 0x2,
                "a2": 0x4,
                "a3": 0x6,
                "b": 7,
                "b1": 0x8,
                "b2": 0xa
            }
        }
      ]

```
### 5.2 bpftool prog dump


下面是一个示例，展示 func_info 和 line_info 如何借助更好的内核符号名、函数原型
和行号信息来帮助 prog dump
```

    $ bpftool prog dump jited pinned /sys/fs/bpf/test_btf_haskv
    [...]
    int test_long_fname_2(struct dummy_tracepoint_args * arg):
    bpf_prog_44a040bf25481309_test_long_fname_2:
    ; static int test_long_fname_2(struct dummy_tracepoint_args *arg)
       0:   push   %rbp
       1:   mov    %rsp,%rbp
       4:   sub    $0x30,%rsp
       b:   sub    $0x28,%rbp
       f:   mov    %rbx,0x0(%rbp)
      13:   mov    %r13,0x8(%rbp)
      17:   mov    %r14,0x10(%rbp)
      1b:   mov    %r15,0x18(%rbp)
      1f:   xor    %eax,%eax
      21:   mov    %rax,0x20(%rbp)
      25:   xor    %esi,%esi
    ; int key = 0;
      27:   mov    %esi,-0x4(%rbp)
    ; if (!arg->sock)
      2a:   mov    0x8(%rdi),%rdi
    ; if (!arg->sock)
      2e:   cmp    $0x0,%rdi
      32:   je     0x0000000000000070
      34:   mov    %rbp,%rsi
    ; counts = bpf_map_lookup_elem(&btf_map, &key);
    [...]

```
### 5.3 验证器日志


下面是一个示例，展示 line_info 如何帮助调试验证过程
```

       /* tools/testing/selftests/bpf/test_xdp_noinline.c 中的代码
        * 被修改如下。
        */
       data = (void *)(long)xdp->data;
       data_end = (void *)(long)xdp->data_end;
       /*
       if (data + 4 > data_end)
               return XDP_DROP;
       */
       *(u32 *)data = dst->dst;

    $ bpftool prog load ./test_xdp_noinline.o /sys/fs/bpf/test_xdp_noinline type xdp
        ; data = (void *)(long)xdp->data;
        224: (79) r2 = *(u64 *)(r10 -112)
        225: (61) r2 = *(u32 *)(r2 +0)
        ; *(u32 *)data = dst->dst;
        226: (63) *(u32 *)(r2 +0) = r1
        invalid access to packet, off=0 size=4, R2(id=0,off=0,r=0)
        R2 offset is outside of the packet

```
## 6. BTF 生成


你需要最新版本的 pahole

  https://git.kernel.org/pub/scm/devel/pahole/pahole.git/

或 llvm（8.0 或更高版本）。pahole 充当 dwarf2btf 转换器。它不
```

      -bash-4.4$ cat t.c
      struct t {
        int a:2;
        int b:3;
        int c:2;
      } g;
      -bash-4.4$ gcc -c -O2 -g t.c
      -bash-4.4$ pahole -JV t.o
      File t.o:
      [1] STRUCT t kind_flag=1 size=4 vlen=3
              a type_id=2 bitfield_size=2 bits_offset=0
              b type_id=2 bitfield_size=3 bits_offset=2
              c type_id=2 bitfield_size=2 bits_offset=5
      [2] INT int size=4 bit_offset=0 nr_bits=32 encoding=SIGNED

```
llvm 能够直接用 -g 为 bpf 目标生成 .BTF 和 .BTF.ext（仅限 bpf 目标）。汇编代码
（-S）能够展示 BTF 在汇编中的编码
```

    -bash-4.4$ cat t2.c
    typedef int __int32;
    struct t2 {
      int a2;
      int (*f2)(char q1, __int32 q2, ...);
      int (*f3)();
    } g2;
    int main() { return 0; }
    int test() { return 0; }
    -bash-4.4$ clang -c -g -O2 --target=bpf t2.c
    -bash-4.4$ readelf -S t2.o
      ......
      [ 8] .BTF              PROGBITS         0000000000000000  00000247
           000000000000016e  0000000000000000           0     0     1
      [ 9] .BTF.ext          PROGBITS         0000000000000000  000003b5
           0000000000000060  0000000000000000           0     0     1
      [10] .rel.BTF.ext      REL              0000000000000000  000007e0
           0000000000000040  0000000000000010          16     9     8
      ......
    -bash-4.4$ clang -S -g -O2 --target=bpf t2.c
    -bash-4.4$ cat t2.s
      ......
            .section        .BTF,"",@progbits
            .short  60319                   # 0xeb9f
            .byte   1
            .byte   0
            .long   24
            .long   0
            .long   220
            .long   220
            .long   122
            .long   0                       # BTF_KIND_FUNC_PROTO(id = 1)
            .long   218103808               # 0xd000000
            .long   2
            .long   83                      # BTF_KIND_INT(id = 2)
            .long   16777216                # 0x1000000
            .long   4
            .long   16777248                # 0x1000020
      ......
            .byte   0                       # string offset=0
            .ascii  ".text"                 # string offset=1
            .byte   0
            .ascii  "/home/yhs/tmp-pahole/t2.c" # string offset=7
            .byte   0
            .ascii  "int main() { return 0; }" # string offset=33
            .byte   0
            .ascii  "int test() { return 0; }" # string offset=58
            .byte   0
            .ascii  "int"                   # string offset=83
      ......
            .section        .BTF.ext,"",@progbits
            .short  60319                   # 0xeb9f
            .byte   1
            .byte   0
            .long   24
            .long   0
            .long   28
            .long   28
            .long   44
            .long   8                       # FuncInfo
            .long   1                       # FuncInfo section string offset=1
            .long   2
            .long   .Lfunc_begin0
            .long   3
            .long   .Lfunc_begin1
            .long   5
            .long   16                      # LineInfo
            .long   1                       # LineInfo section string offset=1
            .long   2
            .long   .Ltmp0
            .long   7
            .long   33
            .long   7182                    # Line 7 Col 14
            .long   .Ltmp3
            .long   7
            .long   58
            .long   8206                    # Line 8 Col 14

```
## 7. 测试


内核 BPF 自测试 `tools/testing/selftests/bpf/prog_tests/btf.c`_
提供了一套广泛的 BTF 相关测试。

   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/tools/testing/selftests/bpf/prog_tests/btf.c
