## Linux/PA-RISC 的寄存器使用


[ 星号用于表示目前尚未实现的计划用途 ]

## ABI 规定的通用寄存器


### 控制寄存器（Control Registers）


===============================	===============================================
CR 0 (Recovery Counter)		用于 ptrace
CR 1-CR 7(undefined)		未使用
CR 8 (Protection ID)		每进程的值*
CR 9, 12, 13 (PIDS)		未使用
CR10 (CCR)			惰性 FPU 保存*
CR11				按 ABI 规定（SAR）
CR14 (interruption vector)	初始化为 fault_vector
CR15 (EIEM)			初始化为全 1*
CR16 (Interval Timer)		读取用于周期计数/写入启动间隔定时器
CR17-CR22			中断参数
CR19				中断指令寄存器（Interrupt Instruction Register）
CR20				中断空间寄存器（Interrupt Space Register）
CR21				中断偏移寄存器（Interrupt Offset Register）
CR22				中断 PSW
CR23 (EIRR)			读取获取挂起中断/写入清除相应位
CR24 (TR 0)			内核空间页目录指针
CR25 (TR 1)			用户空间页目录指针
CR26 (TR 2)			未使用
CR27 (TR 3)			线程描述符指针
CR28 (TR 4)			未使用
CR29 (TR 5)			未使用
CR30 (TR 6)			current / 0
CR31 (TR 7)			临时寄存器，在多处使用
===============================	===============================================

### 空间寄存器（内核模式，Space Registers (kernel mode)）


===============================	===============================================
SR0				临时空间寄存器
SR4-SR7 			置为 0
SR1				临时空间寄存器
SR2				内核不应破坏此寄存器
SR3				用于用户态访问（当前进程）
===============================	===============================================

### 空间寄存器（用户模式，Space Registers (user mode)）


===============================	===============================================
SR0				临时空间寄存器
SR1                             临时空间寄存器
SR2                            保存 linux gateway 页的空间
SR3                            在内核态时保存用户地址空间值
SR4-SR7                        定义用户/内核的短地址空间
===============================	===============================================


### 处理器状态字（Processor Status Word）


===============================	===============================================
W (64-bit addresses)		0
E (Little-endian)		0
S (Secure Interval Timer)	0
T (Taken Branch Trap)		0
H (Higher-privilege trap)	0
L (Lower-privilege trap)	0
N (Nullify next instruction)	由 C 代码使用
X (Data memory break disable)	0
B (Taken Branch)		由 C 代码使用
C (code address translation)	1，执行实模式代码时为 0
V (divide step correction)	由 C 代码使用
M (HPMC mask)			0，执行 HPMC 处理程序时为 1*
C/B (carry/borrow bits)		由 C 代码使用
O (ordered references)		1*
F (performance monitor)		0
R (Recovery Counter trap)	0
Q (collect interruption state)	1（在紧邻 rfi 之前的代码中为 0）
P (Protection Identifiers)	1*
D (Data address translation)	1，执行实模式代码时为 0
I (external interrupt mask)	由 cli()/sti() 宏使用
===============================	===============================================

### "不可见"寄存器（"Invisible" Registers）


===============================	===============================================
PSW 默认 W 值			0
PSW 默认 E 值			0
Shadow Registers（影子寄存器）	由中断处理程序代码使用
TOC enable bit			TOC 使能位 1
===============================	===============================================

-------------------------------------------------------------------------

PA-RISC 架构定义了 7 个"影子寄存器"（shadow registers）。
这些寄存器用于 RETURN FROM INTERRUPTION AND RESTORE（从中断返回并恢复）指令，
通过消除中断处理程序中对通用寄存器（GR）的保存与恢复需求，来减少状态保存与
恢复的时间。
影子寄存器为 GR 1、8、9、16、17、24 和 25。

-------------------------------------------------------------------------

寄存器使用说明，最初来自 John Marvin，并补充了 Randolph Chung 的一些注释。

对于通用寄存器：

r1、r2、r19-r26、r28、r29 及 r31 可以在不先保存的情况下使用。当然，如果在调用
另一个过程之前关心它们的值，则需先保存。上述部分寄存器具有特殊含义，应当注意：

    r1：
	addil 指令被硬连线为将其结果放入 r1，因此若使用该指令需注意这一点。

    r2：
	这是返回指针。通常你不会想使用它，因为你需要该指针返回到调用者。不过，
	它被归入这组寄存器，因为调用者无法依赖返回时其值仍相同，即你可以将
	r2 复制到另一个寄存器，并在破坏 r2 之后通过那个寄存器返回，这不会给
	调用例程带来问题。

    r19-r22：
	通常被视为临时寄存器。注意在 64 位下它们是 arg7-arg4。

    r23-r26：
	这些是 arg3-arg0，即如果你不再关心传入的值，就可以使用它们。

    r28、r29：
	是 ret0 和 ret1。它们是用来传递返回值的。r28 是主要的返回值。当返回
	小型结构体时，r29 也可能被用来向调用者回传数据。

    r30：
	栈指针

    r31：
	ble 指令将返回指针放入此处。


    r3-r18、r27、r30 需要先保存并恢复。r3-r18 只是通用寄存器。r27 是数据
    指针（data pointer），用于更方便地引用全局变量。r30 是栈指针。
