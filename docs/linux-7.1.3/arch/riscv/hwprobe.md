
### RISC-V 硬件探测接口


RISC-V 硬件探测接口围绕一个单独的 syscall 构建，其
```

    struct riscv_hwprobe {
        __s64 key;
        __u64 value;
    };

    long sys_riscv_hwprobe(struct riscv_hwprobe *pairs, size_t pair_count,
                           size_t cpusetsize, cpu_set_t *cpus,
                           unsigned int flags);

```
   参数分为三组：一组键值对数组、一CPU 集合，以及一些标志。键值对带有数量提供。用户空间必须为每个元素key 字段预先填值，如果 key 被识别，内核会填入其 value。如果内核不认识某个 key，其 key 字段会被清为 -1，value 设为 0。CPU 集合CPU_SET(3) 定义，大小为 `cpusetsize` 字节。对于类值的 key（例vendor、arch、impl），仅当给定集合中的所CPU 具有相同值时，返回值才有效。否则将返回 -1。对于类布尔key，返回值是所指定 CPU 值的逻辑与。用户模式可以将 `cpus` 设为 NULL、将 `cpusetsize` 设为 0，作为所有在CPU 的简写。当前支持的标志如下
- `RISCV_HWPROBE_WHICH_CPUS`：该标志基本上反转了 sys_riscv_hwprobe() 的行为。它不是为给定的 CPU 集合填充 key 的值，而是给出每个 key 的值，并由 sys_riscv_hwprobe() CPU 集合缩减为仅那些与每个键值对都匹配的 CPU。如何匹配取决于 key 的类型。对于类值的 key，匹配意味着完全等于该值。对于类布尔key，匹配意味着该对的值与 CPU 的值的逻辑与结果完全等于该对的值。此外，`cpus` 为空集合时，它被初始化为其中能容纳的所有在CPU，即返回CPU 集合是用大小`cpusetsize` CPU 集合所能表示的所有在CPU 的缩减结果
所有其他标志保留供将来兼容使用，必须为零
成功时返0，失败时返回负的错误码
定义了以key
- `RISCV_HWPROBE_KEY_MVENDORID`: 包含 `mvendorid` 的值，定义RISC-V 特权架构规范
- `RISCV_HWPROBE_KEY_MARCHID`: 包含 `marchid` 的值，定义RISC-V 特权架构规范
- `RISCV_HWPROBE_KEY_MIMPID`: 包含 `mimpid` 的值，定义RISC-V 特权架构规范
- `RISCV_HWPROBE_KEY_BASE_BEHAVIOR`: 一个位掩码，包含本内核所支持的基本用户可见行为。定义了以下基本用户 ABI
  - `RISCV_HWPROBE_BASE_BEHAVIOR_IMA`: 支持 rv32ima rv64ima，定义见用户 ISA 2.2 版和特权 ISA 1.10 版，并具有以下已知例外（可能会添加更多例外，但前提是要能证明用户 ABI 未被破坏）：

    - 用户空间程序不能直接执行 `fence.i` 指令（仍可通过 vDSO 等内核控制的机制在用户空间执行）
- `RISCV_HWPROBE_KEY_IMA_EXT_0`: 一个位掩码，包含与 `RISCV_HWPROBE_BASE_BEHAVIOR_IMA`：基本系统行为兼容的扩展
  - `RISCV_HWPROBE_IMA_FD`: 支持 F D 扩展，定义见 RISC-V ISA 手册commit cd20ceeFMIN/FMAX 现在实现 minimumNumber/maximumNumber，而非 minNum/maxNum"）
  - `RISCV_HWPROBE_IMA_C`: 支持 C 扩展，定义见 RISC-V ISA 手册 2.2 版
  - `RISCV_HWPROBE_IMA_V`: 支持 V 扩展，定义见 RISC-V 向量扩展手册 1.0 版
  - `RISCV_HWPROBE_EXT_ZBA`: 支持 Zba 地址生成扩展，定义见位操ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZBB`: 支持 Zbb 扩展，定义见位操ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZBS`: 支持 Zbs 扩展，定义见位操ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZICBOZ`: 支持 Zicboz 扩展，于 riscv-CMOs commit 3dd606fCreate cmobase-v1.0.pdf"）中被批准
  - `RISCV_HWPROBE_EXT_ZBC`: 支持 Zbc 扩展，定义见位操ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZBKB`: 支持 Zbkb 扩展，定义见标量加密 ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZBKC`: 支持 Zbkc 扩展，定义见标量加密 ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZBKX`: 支持 Zbkx 扩展，定义见标量加密 ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZKND`: 支持 Zknd 扩展，定义见标量加密 ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZKNE`: 支持 Zkne 扩展，定义见标量加密 ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZKNH`: 支持 Zknh 扩展，定义见标量加密 ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZKSED`: 支持 Zksed 扩展，定义见标量加密 ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZKSH`: 支持 Zksh 扩展，定义见标量加密 ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZKT`: 支持 Zkt 扩展，定义见标量加密 ISA 扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZVBB`: 支持 Zvbb 扩展，定义见 RISC-V 加密扩展 第二1.0 版
  - `RISCV_HWPROBE_EXT_ZVBC`: 支持 Zvbc 扩展，定义见 RISC-V 加密扩展 第二1.0 版
  - `RISCV_HWPROBE_EXT_ZVKB`: 支持 Zvkb 扩展，定义见 RISC-V 加密扩展 第二1.0 版
  - `RISCV_HWPROBE_EXT_ZVKG`: 支持 Zvkg 扩展，定义见 RISC-V 加密扩展 第二1.0 版
  - `RISCV_HWPROBE_EXT_ZVKNED`: 支持 Zvkned 扩展，定义见 RISC-V 加密扩展 第二1.0 版
  - `RISCV_HWPROBE_EXT_ZVKNH... Zvknha`: 支持 Zvknha 扩展，定义见 RISC-V 加密扩展 第二1.0 版
  - `RISCV_HWPROBE_EXT_ZVKNHB`: 支持 Zvknhb 扩展，定义见 RISC-V 加密扩展 第二1.0 版
  - `RISCV_HWPROBE_EXT_ZVKSED`: 支持 Zvksed 扩展，定义见 RISC-V 加密扩展 第二1.0 版
  - `RISCV_HWPROBE_EXT_ZVKSH`: 支持 Zvksh 扩展，定义见 RISC-V 加密扩展 第二1.0 版
  - `RISCV_HWPROBE_EXT_ZVKT`: 支持 Zvkt 扩展，定义见 RISC-V 加密扩展 第二1.0 版
  - `RISCV_HWPROBE_EXT_ZFH`: 支持 Zfh 扩展 1.0 版，定义RISC-V ISA 手册
  - `RISCV_HWPROBE_EXT_ZFHMIN`: 支持 Zfhmin 扩展 1.0 版，定义RISC-V ISA 手册
  - `RISCV_HWPROBE_EXT_ZIHINTNTL`: 支持 Zihintntl 扩展 1.0 版，定义RISC-V ISA 手册
  - `RISCV_HWPROBE_EXT_ZVFH`: 支持 Zvfh 扩展，定义见 RISC-V 向量手册，自 commit e2ccd0548d6cRemove draft warnings from Zvfh[min]"）起
  - `RISCV_HWPROBE_EXT_ZVFHMIN`: 支持 Zvfhmin 扩展，定义见 RISC-V 向量手册，自 commit e2ccd0548d6cRemove draft warnings from Zvfh[min]"）起
  - `RISCV_HWPROBE_EXT_ZFA`: 支持 Zfa 扩展，定义见 RISC-V ISA 手册，自 commit 056b6ff467c7Zfa is ratified"）起
  - `RISCV_HWPROBE_EXT_ZTSO`: 支持 Ztso 扩展，定义见 RISC-V ISA 手册，自 commit 5618fb5a216bZtso is now ratified."）起
  - `RISCV_HWPROBE_EXT_ZACAS`: 支持 Zacas 扩展，定义见原子比较并交换（CAS）指令手册，commit 5059e0ca641cupdate to ratified"）起
  - `RISCV_HWPROBE_EXT_ZICNTR`: 支持 Zicntr 扩展 2.0 版，定义RISC-V ISA 手册
  - `RISCV_HWPROBE_EXT_ZICOND`: 支持 Zicond 扩展，定义见 RISC-V 整数条件（Zicond）操作扩展手册，commit 95cf1f9Add changes requested by Ved during signoff"）起
  - `RISCV_HWPROBE_EXT_ZIHINTPAUSE`: 支持 Zihintpause 扩展，定义见 RISC-V ISA 手册，自 commit d8ab5c78c207Zihintpause is ratified"）起
  - `RISCV_HWPROBE_EXT_ZIHPM`: 支持 Zihpm 扩展 2.0 版，定义RISC-V ISA 手册
  - `RISCV_HWPROBE_EXT_ZVE32X`: 支持向量子扩Zve32x，定义见 RISC-V 向量扩展手册 1.0 版
  - `RISCV_HWPROBE_EXT_ZVE32F`: 支持向量子扩Zve32f，定义见 RISC-V 向量扩展手册 1.0 版
  - `RISCV_HWPROBE_EXT_ZVE64X`: 支持向量子扩Zve64x，定义见 RISC-V 向量扩展手册 1.0 版
  - `RISCV_HWPROBE_EXT_ZVE64F`: 支持向量子扩Zve64f，定义见 RISC-V 向量扩展手册 1.0 版
  - `RISCV_HWPROBE_EXT_ZVE64D`: 支持向量子扩Zve64d，定义见 RISC-V 向量扩展手册 1.0 版
  - `RISCV_HWPROBE_EXT_ZIMOP`: 支持 Zimop（可能为操作，May-Be-Operations）扩展，定义RISC-V ISA 手册，自 commit 58220614a5fZimop is ratified/1.0"）起
  - `RISCV_HWPROBE_EXT_ZCA`: 支持 Zca 扩展，它是用于减小代码大小的 Zc* 标准扩展的一部分，于 riscv-code-size-reduction commit 8be3419c1c0Zcf doesn't exist on RV64 as it contains no instructions"）中被批准
  - `RISCV_HWPROBE_EXT_ZCB`: 支持 Zcb 扩展，它是用于减小代码大小的 Zc* 标准扩展的一部分，于 riscv-code-size-reduction commit 8be3419c1c0 中被批准
  - `RISCV_HWPROBE_EXT_ZCD`: 支持 Zcd 扩展，它是用于减小代码大小的 Zc* 标准扩展的一部分，于 riscv-code-size-reduction commit 8be3419c1c0 中被批准
  - `RISCV_HWPROBE_EXT_ZCF`: 支持 Zcf 扩展，它是用于减小代码大小的 Zc* 标准扩展的一部分，于 riscv-code-size-reduction commit 8be3419c1c0 中被批准
  - `RISCV_HWPROBE_EXT_ZCMOP`: 支持 Zcmop（可能为操作）扩展，定义RISC-V ISA 手册，自 commit c732a4f39a4Zcmop is ratified/1.0"）起
  - `RISCV_HWPROBE_EXT_ZAWRS`: 支持 Zawrs 扩展，于 riscv-isa-manual commit 98918c844281Merge pull request #1217 from riscv/zawrs"）中被批准
  - `RISCV_HWPROBE_EXT_ZAAMO`: 支持 Zaamo 扩展，定义见 RISC-V ISA 手册，自 commit e87412e621f1integrate Zaamo and Zalrsc text (#1304)"）起
  - `RISCV_HWPROBE_EXT_ZALASR`: 支持 Zalasr 扩展，于 riscv-zalasr commit 194f0094Version 0.9 for freeze"）处冻结
  - `RISCV_HWPROBE_EXT_ZALRSC`: 支持 Zalrsc 扩展，定义见 RISC-V ISA 手册，自 commit e87412e621f1integrate Zaamo and Zalrsc text (#1304)"）起
  - `RISCV_HWPROBE_EXT_SUPM`: 支持 Supm 扩展，定义见 RISC-V 指针掩码扩展 1.0 版
  - `RISCV_HWPROBE_EXT_ZFBFMIN`: 支持 Zfbfmin 扩展，定义见 RISC-V ISA 手册，自 commit 4dc23d6229deAdded Chapter title to BF16"）起
  - `RISCV_HWPROBE_EXT_ZVFBFMIN`: 支持 Zvfbfmin 扩展，定义见 RISC-V ISA 手册，自 commit 4dc23d6229deAdded Chapter title to BF16"）起
  - `RISCV_HWPROBE_EXT_ZVFBFWMA`: 支持 Zvfbfwma 扩展，定义见 RISC-V ISA 手册，自 commit 4dc23d6229deAdded Chapter title to BF16"）起
  - `RISCV_HWPROBE_EXT_ZICBOM`: 支持 Zicbom 扩展，于 riscv-CMOs commit 3dd606fCreate cmobase-v1.0.pdf"）中被批准
  - `RISCV_HWPROBE_EXT_ZABHA`: 支持 Zabha 扩展，于 riscv-zabha commit 49f49c842ff9Update to Rafified state"）中被批准
  - `RISCV_HWPROBE_EXT_ZICBOP`: 支持 Zicbop 扩展，于 riscv-CMOs commit 3dd606fCreate cmobase-v1.0.pdf"）中被批准
  - `RISCV_HWPROBE_EXT_ZILSD`: 支持 Zilsd 扩展，定义见 RISC-V ISA 手册，自 riscv-isa-manual commit f88abf1Integrating load/store pair for RV32 with the main manual"）起
  - `RISCV_HWPROBE_EXT_ZCLSD`: 支持 Zclsd 扩展，定义见 RISC-V ISA 手册，自 riscv-isa-manual commit f88abf1Integrating load/store pair for RV32 with the main manual"）起
- `RISCV_HWPROBE_KEY_CPUPERF_0`: 已弃用。返回与 `RISCV_HWPROBE_KEY_MISALIGNED_SCALAR_PERF` 类似的值，但该 key 被错误地归类为位掩码而非值
- `RISCV_HWPROBE_KEY_MISALIGNED_SCALAR_PERF`: 一个枚举值，描述所选处理器集合上未对齐标量本机字访问的性能
  - `RISCV_HWPROBE_MISALIGNED_SCALAR_UNKNOWN`: 未对齐标量访问的性能未知
  - `RISCV_HWPROBE_MISALIGNED_SCALAR_EMULATED`: 未对齐标量访问通过软件模拟，模拟发生在内核中或内核之下。这些访问总是极慢
  - `RISCV_HWPROBE_MISALIGNED_SCALAR_SLOW`: 未对齐标量本机字大小的访问比同等数量的字节访问更慢。未对齐访问可能由硬件直接支持，也可能被捕获并由软件模拟
  - `RISCV_HWPROBE_MISALIGNED_SCALAR_FAST`: 未对齐标量本机字大小的访问比同等数量的字节访问更快
  - `RISCV_HWPROBE_MISALIGNED_SCALAR_UNSUPPORTED`: 完全不支持未对齐标量访问，会生成未对齐地址错误
- `RISCV_HWPROBE_KEY_ZICBOZ_BLOCK_SIZE`: 一个无符号整数，表Zicboz 块的大小（以字节为单位）
- `RISCV_HWPROBE_KEY_HIGHEST_VIRT_ADDRESS`: 一个无符号长整数，表示可用的最高用户空间虚拟地址
- `RISCV_HWPROBE_KEY_TIME_CSR_FREQ`: `time CSR` 的频率（单位 Hz）
- `RISCV_HWPROBE_KEY_MISALIGNED_VECTOR_PERF`: 一个枚举值，描述所选处理器集合上未对齐向量访问的性能
  - `RISCV_HWPROBE_MISALIGNED_VECTOR_UNKNOWN`: 未对齐向量访问的性能未知
  - `RISCV_HWPROBE_MISALIGNED_VECTOR_SLOW`: 使用向量寄存器的 32 位未对齐访问比通过向量寄存器的同等数量字节访问更慢。未对齐访问可能由硬件直接支持，也可能被捕获并由软件模拟
  - `RISCV_HWPROBE_MISALIGNED_VECTOR_FAST`: 使用向量寄存器的 32 位未对齐访问比通过向量寄存器的同等数量字节访问更快
  - `RISCV_HWPROBE_MISALIGNED_VECTOR_UNSUPPORTED`: 完全不支持未对齐向量访问，会生成未对齐地址错误
- `RISCV_HWPROBE_KEY_VENDOR_EXT_MIPS_0`: 一个位掩码，包含与 `RISCV_HWPROBE_BASE_BEHAVIOR_IMA`：基本系统行为兼容的 mips 厂商扩展
  - MIPS

    - `RISCV_HWPROBE_VENDOR_EXT_XMIPSEXECTL`: MIPS ISA 扩展规范中支xmipsexectl 厂商扩展
- `RISCV_HWPROBE_KEY_VENDOR_EXT_THEAD_0`: 一个位掩码，包含与 `RISCV_HWPROBE_BASE_BEHAVIOR_IMA`：基本系统行为兼容的 thead 厂商扩展
  - T-HEAD

    - `RISCV_HWPROBE_VENDOR_EXT_XTHEADVECTOR`: T-Head ISA 扩展规范中支xtheadvector 厂商扩展，自 commit a18c801634Add T-Head VECTOR vendor extension. "）起
- `RISCV_HWPROBE_KEY_ZICBOM_BLOCK_SIZE`: 一个无符号整数，表Zicbom 块的大小（以字节为单位）
- `RISCV_HWPROBE_KEY_VENDOR_EXT_SIFIVE_0`: 一个位掩码，包含与 `RISCV_HWPROBE_BASE_BEHAVIOR_IMA`：基本系统行为兼容的 sifive 厂商扩展
  - SIFIVE

    - `RISCV_HWPROBE_VENDOR_EXT_XSFVQMACCDOD`: SiFive Int8 矩阵乘法扩展规范 1.1 版中支持 Xsfqmaccdod 厂商扩展
    - `RISCV_HWPROBE_VENDOR_EXT_XSFVQMACCQOQ`: SiFive Int8 矩阵乘法指令扩展规范 1.1 版中支持 Xsfqmaccqoq 厂商扩展
    - `RISCV_HWPROBE_VENDOR_EXT_XSFVFNRCLIPXFQF`: SiFive FP32 int8 范围裁剪指令扩展规范 1.0 版中支持 Xsfvfnrclipxfqf 厂商扩展
    - `RISCV_HWPROBE_VENDOR_EXT_XSFVFWMACCQQQ`: 在矩阵乘累加指令扩展规范 1.0 版中支持 Xsfvfwmaccqqq 厂商扩展
- `RISCV_HWPROBE_KEY_ZICBOP_BLOCK_SIZE`: 一个无符号整数，表Zicbop 块的大小（以字节为单位）
- `RISCV_HWPROBE_KEY_IMA_EXT_1`: 一个位掩码，包含与 `RISCV_HWPROBE_BASE_BEHAVIOR_IMA`：基本系统行为兼容的附加扩展