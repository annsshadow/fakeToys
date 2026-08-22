
## 旧版 Generic Netlink 族的 Netlink 规范支持


本文档描述了描述较老的 Generic Netlink 族（构成 `genetlink-legacy` 协议层级）所需的诸多额外特性与属性
## 规范


### 全局属性（Globals

在规范文件根层级直接列出的属性
#### version


Generic Netlink 族版本，默认值为 1
`version` 在历史上用于引入可能会破坏向后兼容性的族变更。由于通常不允许破坏兼容性的变更，因`version` 很少被使用
### 属性类型嵌套（Attribute type nests

新的 Netlink 族应使用 `multi-attr` 来定义数组。较老的族（例如 `genetlink` 控制族）尝试复用属性类型来携带数组类型信息
```

  [ARRAY-ATTR]
    [INDEX (optionally)]
    [MEMBER1]
    [MEMBER2]
  [SOME-OTHER-ATTR]
  [ARRAY-ATTR]
    [INDEX (optionally)]
    [MEMBER1]
    [MEMBER2]

```
其中 `ARRAY-ATTR` 是数组条目类型
#### indexed-array


`indexed-array` 将整个数组包裹在一个额外的属性中（因此其大小被限制为 64kB）。`ENTRY` 嵌套是特殊的，其类型为条目的索引，而不是普通的属性类型
需要一`sub-type` 来描`ENTRY` 中的类型。`nest` 这种 `sub-type` 表示 `ENTRY` 中包含嵌套数组，其结构如下：

```

  [SOME-OTHER-ATTR]
  [ARRAY-ATTR]
    [ENTRY]
      [MEMBER1]
      [MEMBER2]
    [ENTRY]
      [MEMBER1]
      [MEMBER2]

```
其他 `sub-type`（如 `u32`）表示只有一个成员，如下所示：

```

  [SOME-OTHER-ATTR]
  [ARRAY-ATTR]
    [ENTRY u32]
    [ENTRY u32]

```
#### type-value


`type-value` 是一种利用属性类型来携带单个对象信息的构造（常用于逐条转储数组条目时）
`type-value` 可以有多层嵌套，例如
```

  [POLICY-IDX]
    [ATTR-IDX]
      [POLICY-INFO-ATTR1]
      [POLICY-INFO-ATTR2]

```
其中第一层嵌套以策略索引作为其属性类型，它包含一个单独的嵌套，该嵌套以属性索引作为其类型。在属性索引嵌套内部是策略属性。现代的 Netlink 族本应将其定义为扁平结构，这里的嵌套没有任何好处
## 操作


### 枚举（消ID）模

#### unified


现代族使`unified` 消息 ID 模型，即族内所有消息使用单一枚举。请求与响应共享同一个消ID。通知使用来自同一空间的独ID。例如给定以下操作列表：


  -
    name: a
    value: 1
    do: ...
  -
    name: b
    do: ...
  -
    name: c
    value: 4
    notify: a
  -
    name: d
    do: ...

操作 `a` 的请求和响应将使ID 1，操`b` 的请求和响应使用 2（由于没有显式的 `value`，其 ID 为前一操作 `+ 1`）。通知 `c` 将使ID 4，操`d` 使用 5，依此类推
#### directional


`directional` 模型按照消息的方向来分配 ID。来自内核和发往内核的消息不会相互混淆，因此这种方式节省ID 空间（代价是使编程更加繁琐）
在这种情况下，`value` 属性应在操作的 `request` `reply` 段中指定（如果一个操作同时有 `do` `dump`，则 ID 是共享的，`value` 应在 `do` 中设置）。对于通知，`value` op 层级提供，但它只分配一`reply`（即“来自内核”的 ID）。来看一个例子：


  -
    name: a
    do:
      request:
        value: 2
        attributes: ...
      reply:
        value: 1
        attributes: ...
  -
    name: b
    notify: a
  -
    name: c
    notify: a
    value: 7
  -
    name: d
    do: ...

在这种情况下，`a` 在发送消息给内核时使2，并期望收到 ID 1 的响应。通知 `b` 分配一个“来自内核”的 ID，值为 2。`c` 分配“来自内核”的 ID 7。如果操`d` 没有在规范中显式设置 `values`，则会为请求分配 3（`a` 是前一个带 request 段、value 2 的操作），为响应分配 8（`c` 是“来自内核”方向上的前一个操作）
## 其他特

### 结构体（Structures

旧版族可以定C 结构体，既用作属性的内容，也用作固定的消息头。结构体`definitions` 中定义，并在操作或属性中引用
#### members


 - `name` - 结构体成员的属性名
 - `type` - 标量类型之一：`u8`、`u16`、`u32`、`u64`、`s8`、`s16`、`s32`、`s64`、`string`、`binary` `bitfield32`
 - `byte-order` - `big-endian` 鎴?`little-endian`
 - `doc`、`enum`、`enum-as-flags`、`display-hint` - 与属性定<attribute_properties> 相同

注意，YAML 中定义的结构体按C 约定隐式地紧凑排列（packed）。例如，下面的结构体4 字节，而不6 字节

  struct {
          u8 a;
          u16 b;
          u8 c;
  }

任何填充都必须显式添加，C 语言应根据成员是否自然对齐来推断是否需要显式填充
下面是上面结构体YAML 定义

  definitions:
    -
      name: message-header
      type: struct
      members:
        -
          name: a
          type: u8
        -
          name: b
          type: u16
        -
          name: c
          type: u8

#### Fixed Headers


固定的消息头可以通过 `fixed-header` 添加到操作中。`fixed-header` 的默认值可以在 `operations` 中设置，也可以为每个操作设置或覆盖

  operations:
    fixed-header: message-header
    list:
      -
        name: get
        fixed-header: custom-header
        attribute-set: message-attrs

#### Attributes


`binary` 属性可以通过带有结构体定义名称的 `struct` 属性解释为 C 结构体。`struct` 属性隐`sub-type: struct`，因此无需再指定子类型

  attribute-sets:
    -
      name: stats-attrs
      attributes:
        -
          name: stats
          type: binary
          struct: vport-stats

### C Arrays


旧版族也使用 `binary` 属性来封装 C 数组。`sub-type` 用于标识要提取的标量类型

  attributes:
    -
      name: ports
      type: binary
      sub-type: u32

### Multi-message DO


新的 Netlink 族绝不应在响DO 操作时设`NLM_F_MULTI` 并返回多个回复。应改用过滤转储（filtered dump）
在规范层面，我们可以`do` 定义一`dumps` 属性，其值可能为 `combine` `multi-object`，具体取决于解析应如何实现（解析为单个回复，或解析为对象列表，即几乎等同于一次转储）