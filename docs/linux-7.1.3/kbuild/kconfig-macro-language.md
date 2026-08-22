## Kconfig 宏语言


### 概念


其基本思想Make 的启发。当我们Make 时，会注意到它有点像是把两种语言
合二为一。一种语言描述由目标和先决条件组成的依赖图。另一种是执行文本替换宏语言
这两个语言阶段之间有清晰的区分。例如，```

    APP := foo
    SRC := foo.c
    CC := gcc

    $(APP): $(SRC)
            $(CC) -o $(APP) $(SRC)

```
宏语言将变量引用替换为展开后的形式```

    foo: foo.c
            gcc -o foo foo.c

```
然后，Make 分析依赖图并确定要更新的目标
Kconfig 中的思路非常相似——可以像下面这样描述一Kconfig
```

    CC := gcc

    config CC_HAS_FOO
            def_bool $(shell, $(srctree)/scripts/gcc-check-foo.sh $(CC))

```
Kconfig 中的宏语言将源文件处理为以下内```

    config CC_HAS_FOO
            def_bool y

```
然后，Kconfig 进入求值阶段，以解析符号间的依赖关系，kconfig-language.rst
中所述

### 变量


Make 中一样，Kconfig 中的变量充当宏变量。宏变量被“就地”展开，产生一可能进一步被展开的字符串。要获取变量的值，请将变量名括$( ) 中。即使是
单字母变量名也需要括号；$X 是语法错误{CC} 这种花括号形式也不被支持
变量有两种类型：简单展开变量和递归展开变量
简单展开变量使用 := 赋值运算符定义。读Kconfig 文件中的该行时，其右侧会
立即展开
递归展开变量使用 = 赋值运算符定义。其右侧只是原样存储为变量的值，不进行任展开。相反，展开是在变量被使用时进行的
还有另一种赋值运算符= 用于向变量追加文本。如果左侧最初被定义为简单变量，
+= 的右侧会立即展开。否则，其求值被延迟
```

  $(name,arg1,arg2,arg3)

```
你可以将参数化引用视为一个函数。（更准确地说，是相对于下文列出的“内置函数而言的“用户定义函数”。）

有用的函数必须在使用时展开，因为传入不同参数时同一函数的展开结果不同。因此，
用户定义函数使用 = 赋值运算符定义。参数在函数体定义中通过 $(1)(2) 等引用
事实上，递归展开变量和用户定义函数在内部是相同的。（换句话说，“变量”就“零参数函数”。）当我们广义地说“变量”时，它包含了“用户定义函数”

### 内置函数


Make 一样，Kconfig 提供若干内置函数。每个函数接受特定数量的参数
Make 中，每个内置函数至少接受一个参数。Kconfig 允许内置函数接受零个参数例如 $(filename)(lineno)。你可以把它们看作“内置变量”，但这终究只是我们
叫法的问题。这里我们就称“内置函数”，用来指代原生支持的功能
Kconfig 目前支持以下内置函数
 - $(shell,command)

  “shell”函数接受单个参数，该参数被展开后传递给shell 执行。命令的标准
  输出随后被读取并作为函数的值返回。输出中的每个换行符都被替换为空格。任  尾部换行符都会被删除。标准错误不会被返回，任何程序的退出状态也不会返回
 - $(info,text)

  “info”函数接受单个参数并将其打印stdout。其求值结果为空字符串
 - $(warning-if,condition,text)

  “warning-if”函数接受两个参数。如condition 部分为“y”，text 部分会被
  发送到 stderr。text 之前会加上当Kconfig 文件名和当前行号
 - $(error-if,condition,text)

  “error-if”函数与“warning-if”类似，但如condition 部分为“y”，它会
  立即终止解析
 - $(filename)

  'filename' 不接受参数，并且 $(filename) 被展开为正在被解析的文件名
 - $(lineno)

  'lineno' 不接受参数，并且 $(lineno) 被展开为正在被解析的行号

### Make Kconfig 对比


Kconfig 采用类似 Make 的宏语言，但函数调用语法略有不同
```

  $(func-name arg1,arg2,arg3)

```
函数名与第一个参数之间用一个或多个空白分隔。然后，第一个参数前面的空白会被
去除，而其它参数中的空白会被保留。你需要用某种技巧来让第一个参数以空格开头例如，如果你想要
```

  empty :=
  space := $(empty) $(empty)
  $(info $(space)$(space)hello)

```
Kconfig 仅使用逗号作为分隔符，并保留所有空```

  $(func-name, arg1, arg2, arg3)

```
在这种情况下，“func-name”将收到arg1”、arg2”、arg3”。前导空格的
存在可能会因函数而异。Make 也是如此——例如，$(subst .c, .o, $(sources)) 一个典型的错误；它会把 c替换.o”
Make 中，用户定义函数通过使用内置函数来引用，
```

    $(call my-func,arg1,arg2,arg3)

```
Kconfig 以相同的方式调用用户定义函数和内置函数。省'call' 使语法更简短
Make 中，某些函数将逗号视为字面字符而非参数分隔符。例如，$(shell echo
hello, world) 运行命令 “echo hello, world”。同样，$(info hello, world) “hello, world打印stdout。你可以说这是一种“有用”的不一致
Kconfig 中，为了简化实现并保持语法一致，逗号```

  $(shell, echo hello, world)

```
是一个错误，因为它在'shell' 函数传递两个参数，```

  comma := ,
  $(shell, echo hello$(comma) world)


```
### 注意事项


变量（或函数）不能跨 token 展开。因此，你不能将变量用作由多token 组成表达式的简写```

    RANGE_MIN := 1
    RANGE_MAX := 3

    config FOO
            int "foo"
            range $(RANGE_MIN) $(RANGE_MAX)

```
```

    RANGES := 1 3

    config FOO
            int "foo"
            range $(RANGES)

```
变量不能展开Kconfig 中的任何关键字。以下写```

    MY_TYPE := tristate

    config FOO
            $(MY_TYPE) "foo"
            default y

```
从设计上看很显然(shell command) 是在文本替换阶段展开的。你不能'shell'
函数传递符号
```

    config ENDIAN_FLAG
            string
            default "-mbig-endian" if CPU_BIG_ENDIAN
            default "-mlittle-endian" if CPU_LITTLE_ENDIAN

    config CC_HAS_ENDIAN_FLAG
            def_bool $(shell $(srctree)/scripts/gcc-check-flag ENDIAN_FLAG)

```
相反，你可以像下面这样做，以便任何函数调用都是静态的
```

    config CC_HAS_ENDIAN_FLAG
            bool
            default $(shell $(srctree)/scripts/gcc-check-flag -mbig-endian) if CPU_BIG_ENDIAN
            default $(shell $(srctree)/scripts/gcc-check-flag -mlittle-endian) if CPU_LITTLE_ENDIAN

```
