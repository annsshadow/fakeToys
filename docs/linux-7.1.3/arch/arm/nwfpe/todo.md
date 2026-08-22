## 待办事项（TODO LIST

尚未实现的函数如下（当前由编译器发出，并libc 中的例程处理）。这些函数已FPA11 硬件上实现，由浮点支持代码处理。未来版本将实现其余部分

```
  POW{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - power
  RPW{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - reverse power
  POL{cond}<S|D|E>{P,M,Z} Fd, Fn, <Fm,#value> - polar angle (arctan2)

  LOG{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - logarithm to base 10
  LGN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - logarithm to base e
  EXP{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - exponent
  SIN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - sine
  COS{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - cosine
  TAN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - tangent
  ASN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - arcsine
  ACS{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - arccosine
  ATN{cond}<S|D|E>{P,M,Z} Fd, <Fm,#value> - arctangent

```

可以通过几种途径来实现这些超越函数。其中一种方法是使用基于查找表的精确方法来编写这些例程。我S. Gal IBM 以色列海法（Haifa）研究实验室发表的几篇论文，似乎有望在合理的速度下达到极高的精度（约 99.8%）。该方法使用 GLIBC 的超越函数

另一种途径是我知之甚少CORDIC（Coordinate Rotation Digital Computer，坐标旋转数字计算机）方法，它通过移位以及少量的乘法和除法来计算超越函数。ARM 在移位与加法方面表现出色，因此该方法可能很有前景，但需要更多的研究来确定其可行性

### 舍入模式（Rounding Methods

IEEE 标准定义4 种舍入模式。默认是向最接近值舍入（round to nearest），也允许向正无穷、负无穷以及向零舍入。许多体系结构允许通过修改控制寄存器中的位来指定舍入模式。ARM FPA11 体系结构则通过一条专门的指令来改变舍入模式

这一点使得移植基准测试变得困难。有可能在模拟器中引入相应的能力。FPCR 中包含描述舍入模式的位。模拟器可以修改这些位、检查标志，并强制忽略指令中指定的舍入模式，转而使FPCR 中位所指定的模式

这需要一种获设置标志FPCR 中位的方法。这需要内核调ArmLinux WFC/RFC 监管者（supervisor）指令。如果有人有想法或意见，我希望能听听

注意

以下内容摘自 ARM 浮点文档（特别是 Acorn FPE），但有所删节

浮点控制寄存器（FPCR）在现有实现中并不存在：它用于控制硬件以特定方式实现——例如禁用浮点系统。在用户模式下，ARM 不允许使用该寄存器（因为保留给实现方更改），WFC/RFC 指令若在用户模式下尝试执行会触发异常（trap）

因此，答案是：可以，但运行它的风险很高，因为当硬件浮点模拟出现时，它会变得孤立无援

-- Russell.
