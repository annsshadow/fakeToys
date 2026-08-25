## Smack



    "Good for you, you've decided to clean the elevator!"
    - The Elevator, from Dark Star

Smack 即简化强制访问控制（Simplified Mandatory Access Control）内核
Smack 是一种基于内核实现的强制访问控制（mandatory access
control），其首要设计目标之一便是简单性

Smack 并非 Linux 上唯一的强制访问控制方案。刚接触强制访问控制
用户，建议将 Smack 与其他可用机制做对比，以确定哪一种最适合手头
问题

Smack 由三个主要部分组成：

    - 内核
    - 基础工具程序（有帮助但并非必需
    - 配置数据

Smack 的内核部分实现为一Linux 安全模块（LSM）模块。它依赖
netlabel，并且在具有扩展属性支持的文件系统上工作效果最好，不过
xattr 支持并非严格要求。在“原版（vanilla）”发行版下运Smack
内核是安全的

Smack 内核使用 CIPSO IP 选项。某些网络配置不能容IP 选项，会
妨碍访问Smack 这样使用它们的系统

Smack 用于 Tizen 操作系统。请访问 http://wiki.tizen.org 了解
Smack Tizen 中是如何使用的

Smack 用户空间的当git 仓库为：

	git://github.com/smack-team/smack.git

这在大多数现代发行版上应当可以直接编译并安装。smackutil 中包
五条命令

chsmack:
	显示或设Smack 扩展属性

smackctl:
	加载 Smack 访问规则

smackaccess:
	报告一个带有某标签的进程是否能访问带有另一标签的对

这两条命令在引入 smackfs/load2 smackfs/cipso2 接口后已废弃

smackload:
	将数据格式化为可写入 smackfs/load 的形

smackcipso:
	将数据格式化为可写入 smackfs/cipso 的形

按照 Smack 的设计意图，配置数据尽量少且并非严格要求。最重要
配置步骤是挂smackfs 伪文件系统。如果安装了 smackutil，启
脚本会负责此事，但也可以手动完成

```

    smackfs /sys/fs/smackfs smackfs defaults 0 0

```
`/sys/fs/smackfs` 目录由内核创建

Smack 使用扩展属性（xattrs）在文件系统对象上存储标签。这些属
存储在扩展属性的 security 命名空间中。进程必须具`CAP_MAC_ADMIN`
才能修改这些属性中的任何一个

Smack 使用的扩展属性有

SMACK64
	用于做出访问控制决策。在几乎所有情况下，新建文件系统对
	所获得的标签，都是创建它的进程的标签

SMACK64EXEC
	当进exec 一个带有此属性集的程序文件时，将以该属性的
	作为Smack 标签运行

SMACK64MMAP
	不允许文件被Smack 标签不允许此属性中所含标签进程所拥有
	的全部访问权限的进程所 mmap。这是一个针对共享库的非常特
	的使用场景

SMACK64TRANSMUTE
	只能取"TRUE"。若在某个目录上设置此属性，当在该目录中
	创建对象、且允许对该目录写访问的 Smack 规则（见下文）包
	transmutet"）模式时，该对象会获得目录的标签，而不
	创建进程的标签。如果所创建的对象是一个目录，SMACK64TRANSMUTE
	属性也会被设置

SMACK64IPIN
	此属性仅在套接字的文件描述符上可用。对投递到该套接字的数据包
	使用此属性中Smack 标签做访问控制决策

SMACK64IPOUT
	此属性仅在套接字的文件描述符上可用。对从该套接字发出的数据包，
	使用此属性中Smack 标签做访问控制决策

```

    # attr -S -s SMACK64 -V "value" path
    # chsmack -a value path

```
进程可以通过读取 `/proc/self/attr/current` 来查看自己运行所用的
Smack 标签。具`CAP_MAC_ADMIN` 的进程可以通过向该处写入来设置
进程Smack 标签

大多Smack 配置通过写入 smackfs 文件系统中的文件来完成。该
伪文件系统挂载于 `/sys/fs/smackfs`

access
	为向后兼容而提供。access2 接口更优，应当改用它
	此接口报告带有指Smack 标签的主体对带有指定 Smack 标签
	对象是否具有某种访问权限。向该文件写入一条固定格式的访问
	规则，下一次读取将指出该访问是否被允许。文本将"1"（表
	允许访问）或 "0"（表示拒绝）

access2
	此接口报告带有指Smack 标签的主体对带有指定 Smack 标签
	对象是否具有某种访问权限。向该文件写入一条长格式访问规则
	下一次读取将指出该访问是否被允许。文本将"1"（表示允
	访问）或 "0"（表示拒绝）

ambient
	此文件包含应用于未标记网络数据包Smack 标签

change-rule
	此接口允许修改已有的访问控制规则
```

		"%s %s %s %s"

	其中第一个字符串是主体标签，第二个是对象标签，第三个是允
	的访问，第四个是拒绝的访问。访问字符串只能包含 "rwxat-" 这些
	字符。若针对给定主体和对象已存在规则，则通过启用第三个字符串
	中的权限、禁用第四个字符串中的权限来修改它。若不存在此类规则，
	则使用第三、第四个字符串中指定的访问来创建它

```
cipso
	为向后兼容而提供。cipso2 接口更优，应当改用它
	此接口允许分配一个特定的 CIPSO 头部
```

		"%24s%4d%4d"["%4d"]...

	第一个字符串是一个定Smack 标签。第一个数字是所使用的级别
	第二个数字是类别的数量。后续的数字是类别：

		"level-3-cats-5-19          3   2   5  19"

```
cipso2
	此接口允许分配一个特定的 CIPSO 头部
```

		"%s%4d%4d"["%4d"]...

	第一个字符串是一个长 Smack 标签。第一个数字是所使用的级别
	第二个数字是类别的数量。后续的数字是类别：

		"level-3-cats-5-19   3   2   5  19"

```
direct
	此文件包含用于在网络数据包中直接表示 Smack 标签CIPSO 级别

doi
	此文件包含在网络数据包中使用CIPSO 解释域（domain of
	interpretation）

ipv6host
	此接口允许将特定IPv6 互联网地址当作单标签主机对待。数据包
	仅从对主机标签具Smack 写访问权限的进程发往单标签主机。所
	从单标签主机接收的数据包
```

		"%h:%h:%h:%h:%h:%h:%h:%h label" 鎴?
		"%h:%h:%h:%h:%h:%h:%h:%h/%d label"銆?

	不支"::" 地址简写。若标签"-DELETE"，则匹配的条目会被删除

```
load
	为向后兼容而提供。load2 接口更优，应当改用它
	此接口允许在系统定义的规则之外再指定访问控制规则。所接受
	格式
```

		"%24s%24s%5s"

	其中第一个字符串是主体标签，第二个是对象标签，第三个是所请求
	访问。访问字符串只能包含 "rwxat-" 字符，并指明允许哪类访问
	"-" 是不允许权限的占位符。字符串 "r-x--" 表示允许读和执行访问
	标签长度限制23 个字符

```
load2
	此接口允许在系统定义的规则之外再指定访问控制规则。所接受
	格式
```

		"%s %s %s"

	其中第一个字符串是主体标签，第二个是对象标签，第三个是所请求
	访问。访问字符串只能包含 "rwxat-" 字符，并指明允许哪类访问
	"-" 是不允许权限的占位符。字符串 "r-x--" 表示允许读和执行访问

```
load-self
	为向后兼容而提供。load-self2 接口更优，应当改用它
	此接口允许定义针对特定进程的访问规则。这些规则仅在访问本应被
	允许时才会被查阅，目的是对进程施加额外的限制。其格式load
	接口相同

load-self2
	此接口允许定义针对特定进程的访问规则。这些规则仅在访问本应被
	允许时才会被查阅，目的是对进程施加额外的限制。其格式load2
	接口相同

logging
	此文件包Smack 日志状态

mapped
	此文件包含用于在网络数据包中映射表示 Smack 标签CIPSO 级别

netlabel
	此接口允许将特定的互联网地址当作单标签主机对待。发往单标
	主机的数据包不带 CIPSO 头部，但仅来自对主机标签具有 Smack
	写访问权限的进程。所有从单标签主机接收的数据包都获得指定
```

		"%d.%d.%d.%d label" 鎴?"%d.%d.%d.%d/%d label"銆?

	若指定的标签"-CIPSO"，则该地址被视为支CIPSO 头部的主机

```
onlycap
	此文件包含使 CAP_MAC_ADMIN `CAP_MAC_OVERRIDE` 生效所需
	标签。如果该文件为空，则这些 capability 对任何标签的进程
	生效。其值通过向该文件写入以空格分隔的期望标签来设置，或通过
	向该文件写入 "-" 来清除

ptrace
	此接口用于定义当前的 ptrace 策略

	0 - 默认
	    这是依赖 Smack 访问规则的策略。对`PTRACE_READ`，主体需
	    对对象具有读访问权限。对`PTRACE_ATTACH`，则需要读-
	    访问权限

	1 - exact锛。
	    这是限制 `PTRACE_ATTACH` 的策略。仅当主体与对象的标
	    相等时才允许 attach。`PTRACE_READ` 不受影响。可以通过
	    `CAP_SYS_PTRACE` 覆盖

	2 - draconian锛。
	    此策略行为同上述 "exact"，唯一的例外是它不能被
	    `CAP_SYS_PTRACE` 覆盖

revoke-subject
	向此处写入一Smack 标签，会将所有以该主体标签为条件、访
	'-' 的规则设置好

unconfined
	若内核配置了 `CONFIG_SECURITY_SMACK_BRINGUP`，具
	`CAP_MAC_ADMIN` 的进程可以向此接口写入一个标签。此后，涉及
	该标签的访问将被记录，并在本来不会被允许的情况下被允许。注
	这很危险，会破坏系统的正确标记。绝不应在生产环境中使用

relabel-self
	此接口包含一个进程可通过写入 `/proc/self/attr/current` 转换
	到的标签列表。通常进程可以将自己的标签改为任何合法值，但仅
	它具`CAP_MAC_ADMIN` 时。此接口允许没有 `CAP_MAC_ADMIN` 
	进程将自己重新标记为预定义列表中的某个标签。没`CAP_MAC_ADMIN`
	的进程只能更改一次自己的标签。一旦更改，此列表将被清除
	其值通过向该文件写入以空格分隔的期望标签来设置，或通过写入
	"-" 来清除

如果使用 smackload 工具
```

    subjectlabel objectlabel access

```
access 是字rwxatb 的组合，指明带有 subjectlabel 的主体对带有
objectlabel 的对象被允许哪种访问。如果没有规则，则不允许任何访问

如需更多程序，请访问 http://schaufler-ca.com

## Simplified Mandatory Access Control Kernel（白皮书

Casey Schaufler
casey@schaufler-ca.com

### 强制访问控制

计算机系统采用多种方案来约束使用机器的人员与服务之间信息的共享
其中一些方案允许程序或用户决定允许哪些其他程序或用户访问某些数据
这些方案被称为自主访问控制（discretionary access control）机制，
因为访问控制是由用户自行决定的。另一些方案则不把用户或程序可
访问什么的决定权留给用户或程序。这些方案被称为强制访问控制
（mandatory access control）机制，因为你对于能访问数据的用户或
程序没有选择余地

### Bell & LaPadula

20 世纪 80 年代中期到世纪之交，强制访问控制（MAC）一直与
Bell & LaPadula 安全模型紧密相关，后者是对美国国防部纸质文档
标记策略的数学描述。这种形式的 MAC 在首都环线（Capital Beltway
一带和斯堪的纳维亚的超级计算中心有一些追随者，但常被认为未
满足一般需求

### Domain Type Enforcement

世纪之交前后，Domain Type Enforcement（DTE）开始流行。此方案
用户、程序与数据组织到彼此隔离的域中。该方案作为流行 Linux 发行
的一个组件被广泛部署。维护该方案所需的管理开销，以及提供安全的
域映射所必需的、对整个系统的深入理解，导致在多数情况下该方案被
禁用或以有限方式使用

### Smack

Smack 是一种强制访问控制机制，旨在提供有用MAC，同时避免其
前辈的陷阱。Bell & LaPadula 的局限性通过提供这样一种方案得
解决：访问控制可以根据系统及其目的的需求来设定，而不是由晦涩
政府策略强加。Domain Type Enforcement 的复杂性则通过依据已经
在使用的访问模式来定义访问控制而得以避免

### Smack 术语

谈论 Smack 所用的行话，对已接触过其他 MAC 系统的人会很熟悉，对
初学者也不应太难掌握。有四个以特定方式使用、尤其重要的术语

  Subject（主体）
	主体是计算机系统上的主动实体。在 Smack 中，主体是一个任
	（task），而任务又是执行的基本单元

  Object（对象）
	对象是计算机系统上的被动实体。在 Smack 中，各类文件、IPC
	以及任务都可以是对象

  Access（访问）
	主体试图将信息放入对象或从对象获取信息的任何尝试都是一
	访问

  Label（标签）
	标识主体或对象的强制访问控制特征的数据

这些定义与安全社区的传统用法一致。还有一些可能冒出来Linux
术语

  Capability（能力）
	拥有某项能力的任务，被许可违反由该特定能力所标识的系
	安全策略的某个方面。拥有一个或多个能力的任务是有特权的任务
	而没有能力的任务是无特权的任务

  Privilege（特权）
	被允许违反系统安全策略的任务被称为拥有特权。截至本文撰写时
	任务可以通过拥有能力或拥root 这一有效用户来获得特权

### Smack 基础

Smack Linux 系统的一个扩展。它根据附加在每个主体与对象上的标签
对主体可以访问哪些对象施加额外的限制

#### 标签

Smack 标签ASCII 字符串。它们最长可255 个字符，但建议保持在
23 个字符以内。使用特殊字符（即除字母或数字之外的任何字符）的
字符标签，保留给 Smack 开发团队使用。Smack 标签是无结构的、大小写
敏感的，对其执行的唯一操作是比较是否相等。Smack 标签不能包含不可
打印字符/"（斜杠）\"（反斜杠）'"（单引号）和 '"'（双引号
字符。Smack 标签不能'-' 开头，这保留给特殊选项使用

```

	_ 	读作 "floor"，单个下划线字符
	^ 	读作 "hat"，单个扬抑符（circumflex）字符
	* 	读作 "star"，单个星号字符
	 	读作 "huh"，单个问号字符
	@ 	读作 "web"，单at 符号字符

```
Smack 系统上的每个任务都被分配一个标签。一个进程的 Smack 标签通常
由系统初始化机制分配

#### 访问规则

Smack 使用 Linux 传统的访问模式。这些模式是读、执行、写，偶尔还
追加（append）。有少数情况下访问模式可能不明显，包括：

  Signals（信号）
	信号是从主体任务到对象任务的一次写操作

  Internet Domain IPC锛。
	数据包的传输被视为从源任务到目标任务的一次写操作

Smack 根据附加在主体上的标签和附加在其试图访问的对象上的标签来
限制访问。所强制的规则按顺序是：

 1. 标签"*" 的任务所请求的任何访问都被拒绝
 2. 标签"^" 的任务所请求的读或执行访问被允许
 3. 对标签为 "_" 的对象所请求的读或执行访问被允许
 4. 对标签为 "*" 的对象所请求的任何访问被允许
 5. 任务对具有相同标签的对象所请求的任何访问被允许
 6. 在已加载规则集中被显式定义的任何访问被允许
 7. 任何其他访问都被拒绝

#### Smack 访问规则

借助 Smack 提供的隔离，访问分离是很简单的。在很多有趣的场景下
希望主体能以有限访问权限访问具有不同标签的对象。一个例子是熟悉
敏感性间谍模型：在一个高度机密项目上工作的科学家能够读取低密
文档，而她所写的一切都将“天生”高度机密。为了适应这类方案，Smack
包含一种机制，用于指定允许标签之间访问的规则

#### 访问规则格式

```

	subject-label object-label access

```
其中 subject-label 是任务的 Smack 标签，object-label 是被访问事物
Smack 标签，access 是指明所允许访问种类的字符串。在访问规范中查
描述访问模式的字母：

	a: 表示应授予追加访问
	r: 表示应授予读访问
	w: 表示应授予写访问
	x: 表示应授予执行访问
	t: 表示规则请求转换（transmutation）
	b: 表示规则应被报告用于 bring-up

规范字母的大写形式同样允许。访问模式规范可以按任意顺序排列。可接受
规则的例
```

	TopSecret Secret  rx
	Secret    Unclass R
	Manager   Game    x
	User      HR      w
	Snap      Crackle rwxatb
	New       Old     rRrRr
	Closed    Off     -

```
```

	Top Secret Secret     rx
	Ace        Ace        r
	Odd        spells     waxbeans

```
标签中不允许有空格。由于主体总能访问具有相同标签的文件，为这种情
指定规则毫无意义。访问规范中只允许有效的字母（rwxatbRWXATB）和
短横线（'-'）字符。短横线是占位符，因"a-r" 等同"ar"。单独的
一个短横线用来表示不允许任何访问

#### 应用访问规则

Linux 开发者很少定义新种类的东西，通常从其他系统导入方案与概念
最常见的情况是，其他系统是 Unix 的变体。Unix 有许多讨人喜欢的特性，
但访问控制模型的一致性不在其列。Smack 力求在贴合底层机制精神的
同时，尽可能合理地将各类访问统一对待

文件系统对象（包括文件、目录、命名管道、符号链接和设备）所需的访
权限，与模式位访问所用的权限十分接近。以读方式打开文件需要对文件
具有读访问权限。搜索目录需要执行访问权限。以写访问权限创建文件需
对所在目录同时具有读和写访问权限。删除文件需要对文件和所在目录都
具有读和写访问权限。可能出现这样的情况：用户能看见某文件存在却
无法看到它的任何属性，原因是用户对所在目录有读访问权限但对这
标签不同的文件没有。这是文件名作为目录中数据、而非文件一部分
产物

如果目录被标记为转换中（SMACK64TRANSMUTE=TRUE），且允许进程在
该目录中创建对象的访问规则包't' 访问权限，则分配给新对象的标
将是目录的标签，而不是创建进程的标签。这让两个标签不同的进程能够
共享数据，而无需授予对彼此所有文件的访问权限

IPC 对象、消息队列、信号量集合和内存段存在于扁平命名空间中，访
请求只需匹配相关对象

进程对象反映系统上的任务，用于访问它们的 Smack 标签与该任务用于
自身访问尝试Smack 标签相同。通过 kill() 系统调用发送信号是
信号发送者到接收者的一次写操作。调试一个进程需要既读又写。创建新
任务是一种内部操作，会产生两个具有相Smack 标签的任务，且不需
任何访问检查

套接字是附加到进程的数据结构，从一个进程向另一个进程发送数据包
要求发送者对接收者具有写访问权限。接收者无需对发送者具有读访问
权限

#### 设置访问规则

配置文件 /etc/smack/accesses 包含在系统启动时设置的规则。其内容
写入特殊文件 /sys/fs/smackfs/load2。规则可以随时添加并立即生效
对于任何一对主体和对象标签，只能有一条规则，最近指定的规则会覆
任何更早的指定

#### 任务属

进程Smack 标签可以`/proc/<pid>/attr/current` 读取。进程可以从
`/proc/self/attr/current` 读取自己Smack 标签。有特权的进程可以通过
写入 `/proc/self/attr/current` 来更改自己的 Smack 标签，但不能更改
另一个进程的标签

写入的格式为：仅标签，或标签后跟以下 3 个尾部之一：`\n`（按
`/proc/...` 接口的通常约定）、`\0`（因为某些应用程序错误地包含了它）
`\n\0`（因为我们认为某些应用程序可能错误地包含它）

#### 文件属

文件系统对象Smack 标签作为名为 SMACK64 的扩展属性存储在文件上
该属性位security 命名空间中。它只能由有特权的进程更改

#### 特权

具有 CAP_MAC_OVERRIDE CAP_MAC_ADMIN 的进程是有特权的
CAP_MAC_OVERRIDE 允许进程访问本来会被拒绝的对象。CAP_MAC_ADMIN
允许进程更改 Smack 数据，包括规则和属性

#### Smack 网络

如前所述，Smack 在网络协议传输上强制执行访问控制。Smack 进程发出
每个数据包都带有Smack 标签。这是通过IP 数据包头部添CIPSO
标签来实现的。每个收到的数据包都预期带有标识标签CIPSO 标签，如
缺少此类标签，则假定使用网络环境（ambient）标签。在投递数据包之前
会做一次检查以确定带有该数据包标签的主体对接收进程具有写访问权限，
若非如此，则丢弃该数据包

#### CIPSO 配置

通常无需指定 CIPSO 配置。系统使用的默认值处理所有内部情况。Smack
会在无需管理干预的情况下，自动组合出与所使用 Smack 标签相匹配的
CIPSO 标签值。进入系统的未标记数据包将被赋予环境标签

在可能遇到来自非 Smack 但讲 CIPSO 的系统的数据包时，Smack 需要配置
通常这会是一Trusted Solaris 系统，但也存在其他部署较少的系统
CIPSO 为每个数据包提供 3 个重要值：解释域（DOI）、一个级别和一
类别。DOI 旨在标识一组使用兼容标记方案的系统，且 Smack 系统上指定的
DOI 必须与远程系统匹配，否则数据包会被丢弃。DOI 默认3。该值可以从
/sys/fs/smackfs/doi 读取，也可通过写入 /sys/fs/smackfs/doi 来更改

标签和类别集按照 /etc/smack/cipso 中的定义映射Smack 标签

```

	smack level [category [category]*]

```
Smack 不期望级别或类别集以任何特定方式相关，也不基于它们假定或
分配访问权限。一些例
```

	TopSecret 7
	TS:A,B    7 1 2
	SecBDE    5 2 4 6
	RAFTERS   7 12 26

```
":" "," 字符允许出现Smack 标签中，但没有特殊含义

Smack 标签CIPSO 值的映射通过写入 /sys/fs/smackfs/cipso2 来定义

除显式映射外，Smack 还支持直CIPSO 映射。使用一CIPSO 级别
表示数据包中传入的类别集实际上是Smack 标签的编码。默认使用的
级别250。该值可以从 /sys/fs/smackfs/direct 读取，也可通过写入
/sys/fs/smackfs/direct 来更改

#### 濂楁帴瀛楀睘鎬。

有两个与套接字相关的属性。这些属性只能由有特权的任务设置，但任何
任务都可以为自己的套接字读取它们

  SMACK64IPIN:
	任务对象Smack 标签。一个会强制执行策略的有特权程序可以将其
	设置为星号标签

  SMACK64IPOUT:
	随传出数据包一起传输的 Smack 标签。一个有特权的程序可以将
	设置为匹配它希望与之通信的另一个任务的标签

带有 BSD 地址UNIX 域套接字（UDS）既作为文件系统中的文件，又作为
套接字。作为文件，它带SMACK64 属性。该属性不参与 Smack 安全强制
并被不可变地分配标签 "*"

#### Smack Netlabel 例外

你会经常发现，你的有标签应用程序不得不与外部、无标签的世界通信
为此有一个特殊文/sys/fs/smackfs/netlabel
```

	@IP1	   LABEL1 鎴?
	@IP2/MASK  LABEL2

```
这意味着，如果你的应用程序对 LABEL1 有写访问权限，它将对 @IP1 具有
无标签访问权限；如果LABEL2 有写访问权限，则对子@IP2/MASK 具有
访问权限

/sys/fs/smackfs/netlabel 文件中的条目按最长掩码优先匹配，类似
无类 IPv4 路由

```

	@      表示互联网，任何标签的任意应用程序都可访问它
	-CIPSO 表示标准 CIPSO 网络

```
```

	echo 127.0.0.1 -CIPSO > /sys/fs/smackfs/netlabel
	echo 0.0.0.0/0 @      > /sys/fs/smackfs/netlabel

```
如果你在 192.168.0.0/16 局域网中使CIPSO，并且还需要无标签访问
```

	echo 127.0.0.1      -CIPSO > /sys/fs/smackfs/netlabel
	echo 192.168.0.0/16 -CIPSO > /sys/fs/smackfs/netlabel
	echo 0.0.0.0/0      @      > /sys/fs/smackfs/netlabel

```
### Smack 编写应用程序

有三种类型的应用程序会运行在 Smack 系统上。应用程序与 Smack 
交互方式决定了它需要在 Smack 下正常工作必须做什么

### 无视 Smack 的应用程

绝大多数应用程序没有任何理由关心 Smack 的独特属性。既然调用程
对与该进程关联的 Smack 标签没有影响，唯一可能出现的顾虑就是进
是否对该程序具有执行访问权限

### Smack 相关的应用程

有些程序可以通过了解 Smack 而得到改善，但自己不做任何安全决策
工具程序 ls(1) 就是这类程序的一个例子

### 强制 Smack 的应用程

这些是特殊的程序，不仅了Smack，还参与系统策略的强制执行。在
大多数情况下，这些是用于建立用户会话的程序。也有一些网络服务会
向以各种标签运行的进程提供信息

### 文件系统接口

Smack 使用扩展属性在文件系统对象上维护标签。文件、目录或其他文件
系统对象Smack 标签可以如下获取
```

	len = getxattr("/", "security.SMACK64", value, sizeof (value));

```
会将根目录的 Smack 标签放入 value。一个有特权的进
```

	len = strlen("Rubble");
	rc = setxattr("/foo", "security.SMACK64", "Rubble", len, 0);

```
会在程序具有适当特权时将 /foo Smack 标签设置"Rubble"

### 濂楁帴瀛楁帴鍙。

套接字属性可以使fgetxattr(2) 读取

有特权的进程可以使用以下方式设置传出数据包的 Smack 标签
```

	len = strlen("Rubble");
	rc = fsetxattr(fd, "security.SMACK64IPOUT", "Rubble", len, 0);

```
会在程序具有适当特权时，将套接字发出数据包的 Smack 标签设为 "Rubble"
```

	rc = fsetxattr(fd, "security.SMACK64IPIN, "*", strlen("*"), 0);

```
会在程序具有适当特权时，Smack 标签 "*" 设置为用于检查传入数据包
的对象标签

### 管理

Smack 支持一些挂载选项

  smackfsdef=label:
	指定给缺Smack 标签扩展属性的文件的标签

  smackfsroot=label:
	指定在缺Smack 扩展属性时分配给文件系统根部的标签

  smackfshat=label:
	指定一个必须对文件系统中设置的所有标签具有读访问权限的标签
	尚未强制执行

  smackfsfloor=label:
	指定文件系统中设置的所有标签都必须对其具有读访问权限的标签
	尚未强制执行

  smackfstransmute=label:
	行为完全smackfsroot，只是它还会在挂载根上设transmute
	标志

这些挂载选项适用于所有文件系统类型

### Smack 审计

如果你想要对安全事件进行 Smack 审计，需要在内核配置中设
CONFIG_AUDIT銆。
默认情况下，所有被拒绝的事件都会被审计。你可以通过以下方式改变
此行
```

	0 : 不记
	1 : 记录被拒绝的事件（默认）
	2 : 记录被接受的事件
	3 : 记录被拒绝和被接受的事件

```
事件'key=value' 对的形式记录，对于每个事件你至少会得到主体
对象、所请求的权限、动作、触发该事件的内核函数，以及取决于被审计
事件类型的其他键值对

### Bringup 模式

Bringup 模式提供日志记录功能，可使应用程序配置和系统启动更轻松
在内核中配置 CONFIG_SECURITY_SMACK_BRINGUP 来启用这些功能。当
启用 bringup 模式时，由于被标记为 "b" 访问模式的规则而成功的访问
会被记录。当为进程引入一个新标签时，可以积极添加标记"b" 的规则
日志记录可以追踪哪些规则实际被该标签使用

Bringup 模式的另一个特性是 "unconfined" 选项。向 /sys/fs/smackfs/unconfined
写入一个标签，会使带有该标签的主体能够访问任何对象，而带有该标签
的对象可被所有主体访问。任何因某标签不受限而被允许的访问都会被记录
此特性很危险，因为文件和目录可能在策略被强制时本不能存在的地方被
创建
