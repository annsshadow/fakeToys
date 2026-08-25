
## Devicetree Overlay 笔记


本文档描述位drivers/of/overlay.c 的内核内设备overlay 功能的实现，
Documentation/devicetree/dynamic-resolution-notes.rst[^1^] 的配套文档

### overlay 如何工作


Devicetree overlay 目的是修改内核的实时树（live tree），并使该修改以反映变更的方
影响内核的状态。由于内核主要处理设备，任何导致一个活动设备的新设备节点都应当在创建时
被建立，而如果设备节点被禁用或整体移除，受影响的设备应当被注销

```

    ---- foo.dts ---------------------------------------------------------------
	/* FOO platform */
	/dts-v1/;
	/ {
		compatible = "corp,foo";

		/* shared resources */
		res: res {
		};

		/* On chip peripherals */
		ocp: ocp {
			/* peripherals that are always instantiated */
			peripheral1 { ... };
		};
	};
    ---- foo.dts ---------------------------------------------------------------

```
overlay bar.dtso锛。
```

    ---- bar.dtso - overlay target location by label ---------------------------
	/dts-v1/;
	/plugin/;
	&ocp {
		/* bar peripheral */
		bar {
			compatible = "corp,bar";
			... /* various properties and child nodes */
		};
	};
    ---- bar.dtso --------------------------------------------------------------

```
```

    ---- foo+bar.dts -----------------------------------------------------------
	/* FOO platform + bar peripheral */
	/ {
		compatible = "corp,foo";

		/* shared resources */
		res: res {
		};

		/* On chip peripherals */
		ocp: ocp {
			/* peripherals that are always instantiated */
			peripheral1 { ... };

			/* bar peripheral */
			bar {
				compatible = "corp,bar";
				... /* various properties and child nodes */
			};
		};
	};
    ---- foo+bar.dts -----------------------------------------------------------

```
作为 overlay 的结果，一个新的设备节点（bar）被创建，因此一bar platform 设备会被注册
如果加载了匹配的设备驱动，该设备会如期被创建

如果基础 DT 在编译时没有使用 -@ 选项，那"&ocp" 标签将不可用于把 overlay 节点解析
基础 DT 中的正确位置。在这种情况下，可以提供目标路径。基于标签语法的目标位置是首选的
因为 overlay 可以应用到任何包含该标签的基础 DT，无论该标签DT 中出现在何处

```

    ---- bar.dtso - overlay target location by explicit path -------------------
	/dts-v1/;
	/plugin/;
	&{/ocp} {
		/* bar peripheral */
		bar {
			compatible = "corp,bar";
			... /* various properties and child nodes */
		}
	};
    ---- bar.dtso --------------------------------------------------------------


```
### 鍐呮牳鍐?overlay API


API 使用起来相当容易

1) 调用 of_overlay_fdt_apply() 以创建并应用一overlay changeset。返回值要么是一个错误，
   要么是一个标识该 overlay cookie

2) 调用 of_overlay_remove() 以移除并清理之前通过 of_overlay_fdt_apply() 调用创建overlay
   changeset。不允许移除被另一overlay 堆叠overlay changeset

最后，如果你需要一次性移除所overlay，只需调用 of_overlay_remove_all()，它会以正确的顺
移除每一overlay

还可以注册在 overlay 操作时调用的通知器（notifier）。详of_overlay_notifier_register/unregister
鍜?enum of_overlay_notify_action銆。

针对 OF_OVERLAY_PRE_APPLY、OF_OVERLAY_POST_APPLY OF_OVERLAY_PRE_REMOVE 的通知器回
可以overlay 或其内容中保存指向设备树节点的指针，但这些指针在 OF_OVERLAY_POST_REMOVE
的通知器回调返回后不得继续存在。包overlay 的内存会OF_OVERLAY_POST_REMOVE 通知器被
调用后被 kfree()。注意，即使 OF_OVERLAY_POST_REMOVE 的通知器返回错误，该内存仍会被 kfree()

drivers/of/dynamic.c 中的 changeset 通知器是第二类可能由应用或移overlay 触发的通知器
这些通知器不允许保存指向 overlay 中设备树节点或其内容的指针。overlay 代码并不防止此类指针
在包overlay 的内存因移除 overlay 而被释放时仍然保持活动

任何其它保留指向 overlay 节点或数据指针的代码都被视为缺陷（bug），因为在移overlay 
该指针将指向已释放的内存

overlay 的使用者必须特别留意系统上发生的整体操作，以确保其它内核代码不会保留任何指
overlay 节点或数据的指针。一个无意中使用此类指针的例子是：在 overlay 被应用之后才加载
驱动或子系统模块，而该驱动或子系统扫描整个设备树或其大部分，包overlay 节点
