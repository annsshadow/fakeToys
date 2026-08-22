## Linux 内核 VFP 支持代码发布说明


日期004 5 20 
作者：Russell King

这是 Linux 内核 VFP 支持代码的首次发布。它提供对从 ARM926EJ-S VFP 硬件
弹回的异常的支持
此版本已针对 John R. Hauser SoftFloat-2b 库，使用 TestFloat-2a 测试套件
进行验证。有关该库与测试套件的细节可在此处找到：

   http://www.jhauser.us/arithmetic/SoftFloat.html

已使用此包测试的运算有：

 - fdiv
 - fsub
 - fadd
 - fmul
 - fcmp
 - fcmpe
 - fcvtd
 - fcvts
 - fsito
 - ftosi
 - fsqrt

上述所有运算均通过 softfloat 测试，但有以下例外：

- fadd/fsub 在输入操作数符号不同时，+0 / -0 结果的处理有些差异- 下溢异常的处理略有不同。如果一个结果在舍入前下溢，但在舍入后变为规范化
  数，我们不会发出下溢异常
其他已通过基本纯汇编测试运算的有：

 - fcpy
 - fabs
 - fneg
 - ftoui
 - ftosiz
 - ftouiz

未测试的组合运算有：

 - fmac
 - fnmac
 - fmsc
 - fnmsc
 - fnmul
