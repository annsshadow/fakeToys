## Linux 特有的 ELF 特性


## 定义


"第一个"程序头是文件中偏移量最小的那个：e_phoff。

"最后一个"程序头是文件中偏移量最大的那个：e_phoff + (e_phnum - 1) * sizeof(Elf_Phdr)。

## PT_INTERP


第一个 PT_INTERP 程序头用于定位 ELF 解释器的文件名。其他 PT_INTERP 头被忽略（自 Linux 2.4.11 起）。

## PT_GNU_STACK


最后一个 PT_GNU_STACK 程序头定义用户空间栈的可执行性（自 Linux 2.6.6 起）。其他 PT_GNU_STACK 头被忽略。

## PT_GNU_PROPERTY


ELF 解释器的最后一个 PT_GNU_PROPERTY 程序头被使用（自 Linux 5.8 起）。若解释器没有该头，则使用可执行文件的最后一个 PT_GNU_PROPERTY 程序头。其他 PT_GNU_PROPERTY 头被忽略。
