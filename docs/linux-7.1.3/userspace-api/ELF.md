## Linux 特有ELF 特


## 定义


"第一程序头是文件中偏移量最小的那个：e_phoff

"最后一程序头是文件中偏移量最大的那个：e_phoff + (e_phnum - 1) * sizeof(Elf_Phdr)

## PT_INTERP


第一PT_INTERP 程序头用于定ELF 解释器的文件名。其PT_INTERP 头被忽略（自 Linux 2.4.11 起）

## PT_GNU_STACK


最后一PT_GNU_STACK 程序头定义用户空间栈的可执行性（Linux 2.6.6 起）。其PT_GNU_STACK 头被忽略

## PT_GNU_PROPERTY


ELF 解释器的最后一PT_GNU_PROPERTY 程序头被使用（自 Linux 5.8 起）。若解释器没有该头，则使用可执行文件的最后一PT_GNU_PROPERTY 程序头。其PT_GNU_PROPERTY 头被忽略
