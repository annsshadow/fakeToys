
## Open Firmware Devicetree 单元测试


作者：Gaurav Minocha <gaurav.minocha.os@gmail.com>

## 1. 引言


本文档说明执OF 单元测试所需的测试数
如何动态地附加到活动树（live tree）上，而与机器
体系结构无关

建议在继续之前阅读以下文档

(1) Documentation/devicetree/usage-model.rst
(2) http://www.devicetree.org/Device_Tree_Usage

OF Selftest 旨在测试提供给设备驱动开发者的接口（include/linux/of.h），
以从中获取设备信息等
该接口从展开（unflattened）的设备树数据结构中获取信息，被
大多数设备驱动在各种用例中使用


## 2. 详细输出（EXPECT


如果 unittest 检测到问题，它会向控制台打印警告或错误消息
Unittest 还会故意使用错误的测试数据来触发来自其他
内核代码的警告和错误消息。这导致
混淆：被触发的消息究竟是测试
预期结果，还是存在与 unittest 无关的真正问题

已向 unittest 中添'EXPECT \ : text'（开始）'EXPECT / : text'（结束）消息
以报告某个警告或错误是预期之中的。其
中开始消息在触发警告或错误之前打印，结束消息
在触发之后打印

EXPECT 消息会导致控制台输出非常嘈杂、难
阅读。为此创建了脚本 scripts/dtc/of_unittest_expect 来过
这些冗余信息，并高亮显示被触发的警告和错
与预期警告和错误之间的不匹配。更多信息可
通过 'scripts/dtc/of_unittest_expect --help' 获取


## 3. 测试数据


设备树源文件（drivers/of/unittest-data/testcases.dtso）包
执行自动化单元测试所需
```

    drivers/of/unittest-data/tests-*.dtsi

```
针对 testcases.dtso 中所包含Device Tree Source Include 文件dtsi）的

当内核在启用 CONFIG_OF_UNITTEST 的情况下构建时，会使用以make
```

    $(obj)/%.dtbo: $(src)/%.dtso $(DTC) FORCE
	    $(call if_changed_dep,dtc)

```
DT 源文件（testcases.dtso）编译为二进blob
（testcases.dtbo），也称为扁平化 DT（flattened DT）

之后，使用以下规则将上述二进blob 包装
```

    $(obj)/%.dtbo.S: $(obj)/%.dtbo FORCE
	    $(call if_changed,wrap_S_dtb)

```
该汇编文件被编译为目标文件（testcases.dtbo.o），
链接进内核镜像


### 3.1 添加测试数据


展开的设备树结构

展开的设备树由以树形连接device_node 组成
```

    // following struct members are used to construct the tree
    struct device_node {
	...
	struct  device_node *parent;
	struct  device_node *child;
	struct  device_node *sibling;
	...
    };

```
1 描述了机器展开设备树的通用结构
仅考虑子节点与兄弟节点指针。还存在另一个指
`*parent`，用于反向遍历树。因此，
特定层级上，子节点与所有兄弟节点都会有一个指
公共节点的父指针（例child1、sibling2、sibling3、sibling4 
```

    root ('/')
    |
    child1 -> sibling2 -> sibling3 -> sibling4 -> null
    |         |           |           |
    |         |           |          null
    |         |           |
    |         |        child31 -> sibling32 -> null
    |         |           |          |
    |         |          null       null
    |         |
    |      child21 -> sibling22 -> sibling23 -> null
    |         |          |            |
    |        null       null         null
    |
    child11 -> sibling12 -> sibling13 -> sibling14 -> null
    |           |           |            |
    |           |           |           null
    |           |           |
    null        null       child131 -> null
			    |
			    null

```
1：展开设备树的通用结构


在执OF unittest 之前，需要将测试数据附加
机器的设备树（如果存在）。因此，当调selftest_data_add() 时，
它首先读取链接进内核镜像的展开设备树数据，
```

    __dtb_testcases_begin - address marking the start of test data blob
    __dtb_testcases_end   - address marking the end of test data blob

```
其次，它调用 of_fdt_unflatten_tree() 来展开（unflatten
blob。最后，如果机器的设备树（即 live tree）存在，
则它将展开后的测试数据树附加到 live tree；否
它将自身作为 live 设备树附加

attach_node_and_children() 使用 of_attach_node() 将节点附加到
live tree，如下所述。为说明这一点，下面描述的测试数据树
```

    root ('/')
	|
    testcase-data
	|
    test-child0 -> test-sibling1 -> test-sibling2 -> test-sibling3 -> null
	|               |                |                |
    test-child01      null             null             null


```
2：要附加live tree 的示例测试数据树

根据上述场景，live tree 已经存在，因此无需
附加根（'/'）节点。所有其他节点通过调用
每个节点上的 of_attach_node() 来附加

of_attach_node() 函数中，新节点作为给定父节点
的子节点附加live tree。但是，如果父节点已有子节点，则新节
会替换当前子节点，并将其变为自己的兄弟节点。因此，当将上述
测试数据节点附加到上面的 live tree（图 1）时，最终结构为
```

    root ('/')
    |
    testcase-data -> child1 -> sibling2 -> sibling3 -> sibling4 -> null
    |               |          |           |           |
    (...)             |          |           |          null
		    |          |         child31 -> sibling32 -> null
		    |          |           |           |
		    |          |          null        null
		    |          |
		    |        child21 -> sibling22 -> sibling23 -> null
		    |          |           |            |
		    |         null        null         null
		    |
		    child11 -> sibling12 -> sibling13 -> sibling14 -> null
		    |          |            |            |
		    null       null          |           null
					    |
					    child131 -> null
					    |
					    null
    -----------------------------------------------------------------------

    root ('/')
    |
    testcase-data -> child1 -> sibling2 -> sibling3 -> sibling4 -> null
    |               |          |           |           |
    |             (...)      (...)       (...)        null
    |
    test-sibling3 -> test-sibling2 -> test-sibling1 -> test-child0 -> null
    |                |                   |                |
    null             null                null         test-child01


```
3：附加测试数据后live 设备树结构


细心的读者会注意到，test-child0 节点变成
与先前结构（2）相比的最后一个兄弟节点。在附加第一
test-child0 之后，附test-sibling1 会将子节
（即 test-child0）推为兄弟节点，并使自身成为子节点，
如上所述

如果发现重复节点（即存在具有相同 full_name 属性的节点
已经存在live tree 中），则该节点不会被附加，而是将其
属性通过调用函数
update_node_properties() 更新live tree 的节点上


### 3.2 移除测试数据


一旦测试用例执行完成，就会调用 selftest_data_remove
以移除最初附加的设备节点（首先分离叶节点
然后向上移除父节点，最终移
整棵树）。selftest_data_remove() 调用 detach_node_and_children()，后者使
of_detach_node() 将节点从 live 设备树中分离

要分离一个节点，of_detach_node() 要么更新给定节点父节点的子指
为其兄弟节点，要么将前一个兄弟节点附加到给定节点
兄弟节点上，视情况而定。就是这:)

