
## 引用层级数据节点


:Copyright: |copy| 2018, 2021 Intel Corporation
:Author: Sakari Ailus <sakari.ailus@linux.intel.com>

ACPI 通常只允许引用树中的设备对象。层级数据扩展节点无法直接被引用，因此本文档定义了一种实现此类引用的方案。

对 _DSD 层级数据节点的引用是一个字符串，由一个设备对象引用、一个点（“.”）以及到数据节点对象的相对路径组成。不要使用非字符串引用，因为那会产生层级数据节点的副本，而不是引用！

被引用的层级数据扩展节点应直接位于其父对象之下，即要么位于设备对象之下，要么位于另一个层级数据扩展节点之下 [dsd-guide]。

层级数据节点中的键应由节点名称、“@”字符以及节点的编号（十六进制表示，不带前后缀）组成。同一个 ACPI 对象应包含带有 “reg” 属性的 _DSD 属性扩展，该属性的数值应与节点编号相同。

如果某个层级数据扩展节点没有数值，则应从 ACPI 对象的 _DSD 属性中省略 “reg” 属性，并从层级数据扩展键中省略 “@” 字符与编号。


## 示例


在下面的 ASL 片段中，“reference” _DSD 属性包含对层级数据扩展节点 ANOD 的字符串引用，该节点位于 DEV1 父对象之下的 DEV0 之下。ANOD 同时也是该引用的最终目标节点。
```

	Device (DEV0)
	{
	    Name (_DSD, Package () {
		ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		Package () {
		    Package () { "node@0", "NOD0" },
		    Package () { "node@1", "NOD1" },
		}
	    })
	    Name (NOD0, Package() {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "reg", 0 },
		    Package () { "random-property", 3 },
		}
	    })
	    Name (NOD1, Package() {
		ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		Package () {
		    Package () { "reg", 1 },
		    Package () { "anothernode", "ANOD" },
		}
	    })
	    Name (ANOD, Package() {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "random-property", 0 },
		}
	    })
	}

	Device (DEV1)
	{
	    Name (_DSD, Package () {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "reference", "^DEV0.ANOD" }
		    },
		}
	    })
	}

```
另请参阅图表示例：
Documentation/firmware-guide/acpi/dsd/graph.rst。

## 参考


[dsd-guide] DSD Guide.
    https://github.com/UEFI/DSD-Guide/blob/main/dsd-guide.adoc，引用日期
    2021-11-30。
