## IMA Template Management Mechanism


## Introduction


最初的 `ima` 模板是固定长度的，包含文件数据哈希（filedata hash）和路径名。文件数据哈希限制为 20 字节（md5/sha1）。路径名是一个以 null 结尾的字符串，限制为 255 个字符。为了克服这些限制并添加额外的文件元数据，有必要通过定义额外的模板来扩展当前版本IMA。例如，可能报告的信息有 inode UID/GID，或inode 以及访问它的进程LSM 标签
然而，引入此特性的主要问题是，每次定义一个新的模板时，生成和显示度量列表（measurements list）的函数都会包含处理新格式的代码，因此会随着时间的推移显著增长
所提出的解决方案通过将模板管理与 IMA 其余代码分离来解决此问题。该方案的核心是两个新数据结构的定义：一个模板描述符（template descriptor），用于决定度量列表中应包含哪些信息；一个模板字段（template field），用于生成和显示给定类型的数据
使用这些结构管理模板非常简单。要支持一种新的数据类型，开发者定义字段标识符，并实现两个函数 init() show()，分别用于生成和显示度量条目。定义一个新的模板描述符需要通过 `ima_template_fmt` 内核命令行参数指定模板格式（一个由 `|` 字符分隔的字段标识符字符串）。在启动时，IMA 通过将格式转换为取自受支持集合的模板字段结构数组来初始化所选的模板描述符
初始化步骤之后，IMA 将调`ima_alloc_init_template()`（在用于新模板管理机制的补丁中定义的新函数），以使用通过内核配置或新引入`ima_template` `ima_template_fmt` 内核命令行参数选择的模板描述符来生成一个新的度量条目。正是在这一阶段，新架构的优势清晰地展现出来：后一个函数不会包含处理给定模板的特定代码，而是简单地调用与所选模板描述符关联的模板字段的 `init()` 方法，并将结果（指向已分配数据的指针和数据长度）存储在度量条目结构中
显示度量条目采用了相同的机制。函`ima[_ascii]_measurements_show()` 为每个条目检索用于生成该条目的模板描述符，并对模板字段结构数组中的每一项调show() 方法


## Supported Template Fields and Descriptors


下面列出了受支持的模板字`('<identifier>': description)`，可以通过将其标识符添加到格式字符串中来用于定义新的模板描述符（稍后将添加对更多数据类型的支持）：

 - 'd'：事件的摘要（digest）（即被度量文件的摘要），使SHA1 MD5 哈希算法计算 - 'n'：事件的名称（即文件名），大小上限为 255 字节 - 'd-ng'：事件的摘要，使用任意哈希算法计算（字段格式hash algo>:digest）；
 - 'd-ngv2'：与 d-ng 相同，但前缀"ima" "verity" 摘要类型（字段格式：<digest type>:<hash algo>:digest）；
 - 'd-modsig'：不含附modsig 的事件的摘要 - 'n-ng'：事件的名称，无大小限制 - 'sig'：文件签名，基于文件fsverity 的摘要[^1^] EVM 可移植签名（如果 'security.ima' 包含文件哈希）；
 - 'modsig'：附加的文件签名 - 'buf'：用于生成哈希的缓冲区数据，无大小限制；
 - 'evmsig'：EVM 可移植签名；
 - 'iuid'：inode UID - 'igid'：inode GID - 'imode'：inode 模式 - 'xattrnames'：xattr 名称列表（以 `|` 分隔），仅当 xattr 存在时；
 - 'xattrlengths'：xattr 长度列表（u32），仅当 xattr 存在时；
 - 'xattrvalues'：xattr 值列表；

下面列出了已定义的模板描述符
 - "ima"：其格式`d|n` - "ima-ng"（默认）：其格式`d-ng|n-ng` - "ima-ngv2"：其格式`d-ngv2|n-ng` - "ima-sig"：其格式`d-ng|n-ng|sig` - "ima-sigv2"：其格式`d-ngv2|n-ng|sig` - "ima-buf"：其格式`d-ng|n-ng|buf` - "ima-modsig"：其格式`d-ng|n-ng|sig|d-modsig|modsig` - "evm-sig"：其格式`d-ng|n-ng|evmsig|xattrnames|xattrlengths|xattrvalues|iuid|igid|imode`
## Use


要指定用于生成度量条目的模板描述符，目前支持以下方法
 - 在内核配置支持的模板描述符中选择一个（`ima-ng` 为默认选择）；
 - 通过 `ima_template=` 参数从内核命令行指定一个模板描述符名称 - 通过内核命令行参`ima_template_fmt=` 注册一个具有自定义格式的新的模板描述符