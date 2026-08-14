
## 用 json-schema 编写 Devicetree 绑定


设备树（Devicetree）绑定使用 json-schema 词汇来编写。模式（Schema）文件采用 YAML 的
一个与 JSON 兼容的子集编写。之所以使用 YAML 而非 JSON，是因为它被认为更易读，并且
具有一些优势，例如允许注释（以 '#' 为前缀）。

另请参阅 example-schema。

### Schema 内容


每个模式文档都是一个结构化的 json-schema，由一组顶层属性定义。通常，每个文件定义一个
绑定。所使用的顶层 json-schema 属性如下：

$id
  一个 json-schema 唯一标识符字符串。该字符串必须是一个有效的 URI，通常包含绑定的
  文件名和路径。对于 DT schema，它必须以 "http://devicetree.org/schemas/" 开头。该 URL
  用于构造对 schema 的 "$ref" 属性中指定的其他文件的引用。带有前导 '/' 的 $ref 值会被
  加上主机名。仅包含相对路径或文件名的 $ref 值会被加上当前 schema 文件 '$id' 值的主机名
  和路径部分。即使对于本地文件也使用 URL，但实际可能并不存在位于这些位置的文件。

$schema
  指明该 schema 文件所遵循的元模式（meta-schema）。

title
  一行描述，说明绑定 schema 中所描述硬件的内容。

maintainers
  DT 特有的属性。包含一个或多个维护该绑定的维护者的电子邮件地址列表。

description
  可选。一个多行文本块，包含关于该硬件的任何详细信息。它应包含诸如该模块或设备的作用、
  设备所遵循的标准，以及指向数据手册以获取更多信息的链接等内容。

  YAML 格式有几种定义文本块格式的选项。这些选项由键后面的指示符字符控制（例如
  "description: \|"）。应使用文本块所需的最小格式。格式控制不仅会影响 YAML 是否能被
  正确解析，而且在将文本块渲染为其他形式时也很重要。选项如下。

  没有任何指示符的默认是流式（flowed）的纯标量（plain scalar）风格，会去掉单换行符和
  前导空白。段落由空行（即双换行符）分隔。这种风格不能包含 ": "，因为它会被解释为键。
  任何 " #" 序列都会被解释为注释。对其他字符也有更多限制。大多数限制是关于首字符可以
  是什么。

  第二种风格是折叠（folded），由 ">" 字符指示。除了在双换行符处保留换行外，折叠风格还
  保留超出首行缩进的前导空白。缩进行上的换行符也会被保留。

  第三种风格是字面（literal），由 "\|" 字符指示。字面风格保留所有换行符和空白（超出
  首行缩进的部分）。

  以上并非对 YAML 文本块的完整描述。关于多行 YAML 文本块的更多细节可以在网上找到：

  https://yaml-multiline.info/

  https://www.yaml.info/learn/quote.html

select
  可选。一个 json-schema，用于匹配要应用该 schema 的节点。默认情况下，在没有 'select'
  时，节点会依据其可能的 compatible 字符串值或节点名进行匹配。大多数绑定不需要 select。

allOf
  可选。要包含的其他 schema 的列表。用于包含该绑定所遵循的其他 schema。这些可以是某类
  设备（例如 I2C 或 SPI 控制器）的 schema。

properties
  一组子 schema，定义该绑定的所有 DT 属性。具体的 schema 语法取决于属性是已知的公共
  属性（例如 'interrupts'）还是绑定/厂商特定的属性。

一个属性也可以定义一个子 DT 节点，其下定义子属性。

关于 properties 部分的更多细节，请参阅 'Property Schema' 一节。

patternProperties
  可选。与 'properties' 类似，但名称是正则表达式。

required
  来自 'properties' 节、必须始终存在的 DT 属性列表。

additionalProperties / unevaluatedProperties
  控制 schema 如何验证未被本 schema 的 'properties' 或 'patternProperties' 匹配到的
  属性的关键字。每个 schema 都应在顶层部分恰好包含这些关键字之一，即 additionalProperties
  或 unevaluatedProperties。嵌套节点（即作为对象的属性）也应包含一个。

  - additionalProperties: false
      最常见的情况，即不引用额外的 schema，或者本绑定允许来自其他被引用 schema 的属性
      的子集。

  - unevaluatedProperties: false
      当本绑定引用了其他 schema，并且应允许其所有属性时使用。

  - additionalProperties: true
      - 顶层部分：
        罕见情况，用于实现一组公共属性的 schema。此类 schema 应被其他 schema 引用，后者
        再使用 'unevaluatedProperties: false'。通常是总线或公共部分的 schema。
      - 嵌套节点：
        当仅列出嵌套节点的期望 compatible，并且存在另一个匹配该 compatible、以上述两种
        情况之一（'false'）结尾的 schema 时。

examples
  可选。实现一个或多个仅包含本绑定的 DTS 片段的列表。示例不应包含不相关的设备节点，例如
  在 provider 绑定中的 consumer 节点，或其他通过 phandle 引用的节点。
  注意：YAML 不允许使用前导制表符，因此必须改用空格。

除非另有说明，所有属性都是必需的。

### 属性 Schema（Property Schema）


schema 的 'properties' 部分包含某个绑定的所有 DT 属性。每个属性包含一组使用该属性
json-schema 词汇的约束。属性 schema 用于对 DT 文件进行验证。

对于公共属性，只需要定义公共绑定 schema 未涵盖的额外约束，例如有多少个值是有效的或
哪些可能的值是有效的。

厂商特定的属性通常需要更详细的 schema。除布尔属性外，它们应引用 schemas/types.yaml 中的
某个类型。始终需要一个 "description" 属性。

设备树 schema 与 dtc 产生的 YAML 编码的 DT 数据并不完全匹配。它们被简化了，以使其更
紧凑并避免大量样板。工具会处理 schema 文件以生成用于验证的最终 schema。目前工具执行
两种转换。

json-schema 中数组的默认情况是变长的，并且允许比显式定义的更多的条目。这可以通过定义
'minItems'、'maxItems' 和 'additionalItems' 来限制。然而，对于设备树 Schema，在大多数
情况下需要固定大小，因此这些属性会根据 'items' 列表中的条目数量添加。

YAML 设备树格式还将所有字符串值变为数组、将标量值变为矩阵（以便定义分组），即使只有
单个值时也是如此。schema 中的单个条目会被修正以匹配这种编码。

当绑定覆盖多个在某些属性上不同的相似设备时，应对每个设备的这些属性加以约束。这通常
意味着：

 - 在顶层 'properties' 中定义具有最宽泛约束的属性。
 - 在 'if:then:' 块中，进一步收窄这些属性的约束。
 - 不要在 'if:then:' 块内定义属性（注意 'additionalItems' 也不允许那样做）。

### 代码风格（Coding style）


使用 YAML 代码风格（两空格缩进）。对于 schema 中的 DTS 示例，建议使用四空格缩进。

将 'properties' 和 'required' 节中的条目按相同顺序排列，使用
Documentation/devicetree/bindings/dts-coding-style.rst 中的风格。

### 测试


#### 依赖


必须安装 DT schema 项目，以便验证 DT schema 绑定文档并使用 DT schema 验证 DTS 文件。
DT schema
```
    pip3 install dtschema
```
注意，'dtschema' 的安装需要 'swig' 和 Python 开发文件
```
    apt install swig python3-dev
```
会安装几个可执行文件（dt-doc-validate、dt-mk-schema、dt-validate）。请确保它们在你的
PATH 中（默认为 ~/.local/bin）。

还建议安装 yamllint（在存在时由 dtschema 使用）。

#### 运行检查


DT schema 绑定文档必须使用元模式（meta-schema，即 schema 的 schema）进行验证，以确保
它们既是有效的 json-schema，也是有效的绑定 schema。所有 DT 绑定文档都可以通过
```
    make dt_binding_check
```
```
    make sram/sram.yaml
```
```
    make dtbs_check
```
注意，`dtbs_check` 会跳过任何有错误的绑定 schema 文件。必须使用 `dt_binding_check` 才能
获得绑定 schema 文件中的所有验证错误。
```
    make dt_binding_check dtbs_check
```
也可以将运行上述命令与一部分匹配的 schema 文件结合起来，方法是把 `DT_SCHEMA_FILES`
变量设置为一个或多个特定的 schema 文件或模式（固定字符串的部分匹配）。每个文件或模式
应以 ':' 分隔。
```
    make dt_binding_check DT_SCHEMA_FILES=trivial-devices.yaml
    make dt_binding_check DT_SCHEMA_FILES=trivial-devices.yaml:rtc.yaml
    make dt_binding_check DT_SCHEMA_FILES=/gpio/
    make dtbs_check DT_SCHEMA_FILES=trivial-devices.yaml


```
### json-schema 资源


`JSON-Schema Specifications <http://json-schema.org/>`_

`Using JSON Schema Book <http://usingjsonschema.com/>`_

### 带注释的示例 Schema


也可作为单独的文件获取：`example-schema.yaml`
