## DeviceTree 动态解析器说明


本文档描述位于 drivers/of/resolver.c 的内核内 DeviceTree 解析器的实现。

### 解析器的工作原理


解析器的输入是一棵使用恰当的 dtc 选项编译、并带有 /plugin/ 标签的任意设备树。这会生成相应的 __fixups__ 与 __local_fixups__ 节点。

解析器按顺序执行以下步骤：

1. 从实时设备树获取最大的设备树 phandle 值并加 1。
2. 调整待解析设备树的所有本地 phandle，使其增加该数值。
3. 利用 __local__fixups__ 节点的信息，按相同数值调整所有本地引用。
4. 对于 __fixups__ 节点中的每个属性，在实时设备树中定位其引用的节点。该标签用于标记此节点。
5. 获取 fixup 目标的 phandle。
6. 对于属性中的每个 fixup，定位 node:property:offset 位置，并将其替换为 phandle 值。
