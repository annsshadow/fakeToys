## Reed-Solomon 库编程接

:Author: Thomas Gleixner

## 简

通用Reed-Solomon 库提供了编码、解码和纠错函数
Reed-Solomon 码用于通信和存储应用中，以确保数据完整性
本文档提供给希望利用该库所提供函数的开发者
## 已知缺陷与假

无
## 用法


本章提供如何使用该库的示例
### 初始

初始化函init_rs 返回一个指rs 解码器结构的指针，该结构保存了使给定多项式进行编码、解码和纠错所必需的信息。它会使用一个已有的匹配
解码器，或者创建一个新的。在创建时，所有用于快速编解码的查找表都会创建。该函数可能会耗时较长，因此请确保在关键代码路径中不要调用它
```

    /* Reed Solomon 控制结构 */
    static struct rs_control *rs_decoder;

    /* 符号大小10（位     * 本原多项式为 x^10+x^3+1
     * 第一个连续根0
     * 用于生成根的本原元素 = 1
     * 生成多项式次数（根的数量 6
     */
    rs_decoder = init_rs (10, 0x409, 0, 1, 6);


```
### 编码


编码器在给定数据长度上计Reed-Solomon 码，并将结果存入奇偶校验（parity缓冲区。请注意，在调用编码器之前必须初始化奇偶校验缓冲区
通过提供一个非零的反转掩码，可以在计算过程中就地反转扩展后的数据。扩展后
的数据会与掩码进行异或（XOR）。例如，这用FLASH ECC，其中全 0xFF 反转为全 0x00。全 0x00 Reed-Solomon 码也是全 0x00。在存入 FLASH 之前
该码被反转，因此它也0xFF。这样可以防止读取已擦除FLASH 时产ECC
错误
数据字节会在计算过程中就地扩展为给定的符号大小。目前不支持对符号大不等8 的连续比特流进行编码。如果有必要，实现这样的功能应该不是难事
```

    /* 奇偶校验缓冲区。大= 根的数量 */
    uint16_t par[6];
    /* 初始化奇偶校验缓冲区 */
    memset(par, 0, sizeof(par));
    /* data8 中的 512 字节进行编码。将奇偶校验存入缓冲par */
    encode_rs8 (rs_decoder, data8, 512, par, 0);


```
### 解码


解码器在给定数据长度以及接收到的奇偶校验符号上计算伴随式（syndrome），
并纠正数据中的错误
如果伴随式可以从硬件解码器获得，则跳过伴随式的计算
通过向解码器提供纠错模式缓冲区和错误位置缓冲区，可以抑制对数据缓冲区纠正。解码器将计算出的错误位置和纠错位掩码存入给定的缓冲区。这对于使用
奇怪位序方案的硬件解码器很有用
数据字节会在计算过程中就地扩展为给定的符号大小。目前不支持对符号大不等8 的连续比特流进行解码。如果有必要，实现这样的功能应该不是难事
#### 带伴随式计算的解码，直接数据纠正


```

    /* 奇偶校验缓冲区。大= 根的数量 */
    uint16_t par[6];
    uint8_t  data[512];
    int numerr;
    /* 接收数据 */
    .....
    /* 接收奇偶校验 */
    .....
    /* data8 中的 512 字节进行解码/
    numerr = decode_rs8 (rs_decoder, data8, par, 512, NULL, 0, NULL, 0, NULL);


```
#### 由硬件解码器提供伴随式的解码，直接数据纠

```

    /* 奇偶校验缓冲区。大= 根的数量 */
    uint16_t par[6], syn[6];
    uint8_t  data[512];
    int numerr;
    /* 接收数据 */
    .....
    /* 接收奇偶校验 */
    .....
    /* 从硬件解码器获取伴随*/
    .....
    /* data8 中的 512 字节进行解码/
    numerr = decode_rs8 (rs_decoder, data8, par, 512, syn, 0, NULL, 0, NULL);


```
#### 由硬件解码器提供伴随式的解码，无直接数据纠正

注意：不需要向解码器提供数据和接收到的奇偶校验
```

    /* 奇偶校验缓冲区。大= 根的数量 */
    uint16_t par[6], syn[6], corr[8];
    uint8_t  data[512];
    int numerr, errpos[8];
    /* 接收数据 */
    .....
    /* 接收奇偶校验 */
    .....
    /* 从硬件解码器获取伴随*/
    .....
    /* data8 中的 512 字节进行解码/
    numerr = decode_rs8 (rs_decoder, NULL, NULL, 512, syn, 0, errpos, 0, corr);
    for (i = 0; i < numerr; i++) {
        do_error_correction_in_your_buffer(errpos[i], corr[i]);
    }


```
### 清理


如果函数 free_rs 的调用者是解码器的最后一个使用者，它将释放已分配的资源
```

    /* 释放资源 */
    free_rs(rs_decoder);


```
## 结构


本章包含 Reed-Solomon 库中供开发者使用的结构的自动生成文档
   :internal:

## 提供的公共函

本章包含导出Reed-Solomon 函数的自动生成文档
   :export:

## 致谢


用于编码和解码的库代码由 Phil Karn 编写
```

            Copyright 2002, Phil Karn, KA9Q
            May be used under the terms of the GNU General Public License (GPL)


```
包装函数和接口由 Thomas Gleixner 编写
许多用户提供bug 修复、改进以及测试方面的帮助。非常感谢
以下人员对本文档做出了贡献：

Thomas Gleixner\ tglx@kernel.org
