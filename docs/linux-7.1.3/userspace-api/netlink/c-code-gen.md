
## Netlink 规范 C 代码生成


本文档介绍如何使Netlink 规范生成 C 代码（uAPI、策略等）。同时定义了旧式 family `genetlink-c` 协议层级所允许的额外属性，用于控制命名
为简洁起见，本文档以对象类型来指代各类对象的 `name` 属性。例`$attr` 表示某个 attribute `name` 的值，`$family` 表示 family 的名称（即全局`name` 属性）
大写用于表示一个字面量值，例如 `$family-CMD` 表示 `$family`、连字符以及字面`CMD` 的拼接
`#defines` 和枚举值的名称始终转换为大写，连字符（`-`）会被替换为下划线（`_`）
如果构造出的名称是 C 语言关键字，则追加一个下划线（`do` -> `do_`）
## 全局属

`c-family-name` 控制 family 名称对应`#define` 名称，默认值为 `$family-FAMILY-NAME`
`c-version-name` 控制 family 版本对应`#define` 名称，默认值为 `$family-FAMILY-VERSION`
`max-by-define` 选择是否将枚举的最大值定义为 `#define` 而非置于枚举内部
## 定义


### 常量


每个常量都渲染为一`#define`。常量名称为 `$family-$constant`，其值根据其类型在规范中渲染为字符串或整数
### 枚举与标

枚举命名`$family-$enum`。可以通过 `enum-name` 属性直接设置或屏蔽其完整名称。默认条目名称为 `$family-$enum-$entry`。若指定`name-prefix`，则替换条目名称中的 `$family-$enum` 部分
布尔属`render-max` 控制是否创建最大值（对于 attribute 枚举，默认启用）。这些最大值命名为 `__$pfx-MAX` `$pfx-MAX`。第一个值的名称可通过 `enum-cnt-name` 属性覆盖
## 属

每个属性集（分数集除外）都渲染为一个枚举
netlink 头文件中，属性枚举传统上是不具名的。若需要命名，可使`enum-name` 指定名称
如果属性集的名称与 family 名称相同，则默认属性名称前缀`$family-A`；若名称不同，则前缀`$family-A-$set`。该前缀可被属性集`name-prefix` 属性覆盖。下文以 `$pfx` 表示此前缀
属性命名为 `$pfx-$attribute`
属性枚举以两个特殊`__$pfx-MAX` `$pfx-MAX` 结尾，用于确定属性表的大小。这两个名称可分别通过 `attr-cnt-name` `attr-max-name` 属性直接指定
如果在全局层面`max-by-define` 设为 `true`，则 `attr-max-name` 会被指定`#define` 而非枚举值
## 操作


操作命名`$family-CMD-$operation`。若指定`name-prefix`，则替换名称中的 `$family-CMD` 部分
与属性枚举类似，操作枚举也以特殊的计数与最大值属性结尾。对于操作，这些属性可通过 `cmd-cnt-name` `cmd-max-name` 重命名。若 `max-by-define` `true`，则最大值将是一define
## 澶氭挱缁。

每个多播组都会生成一define，写入内uAPI 头文件。该 define 的名称为 `$family-MCGRP-$group`，可通过 `c-define-name` 属性覆盖
## 代码生成


默认头文件搜索路径下，uAPI 头文件假定来`<linux/$family.h>`。可通过全局属`uapi-header` 更改