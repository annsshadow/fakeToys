
## Devicetree 源文件（DTS）编码风

编写 Devicetree 源文件（DTS）时，请遵守以下准则。它们应被视为对 Devicetree 规范以及
dtc 编译器（包括 W=1 W=2 构建）中已表达的任何规则的补充
各个架构和子架构可以定义额外的规则，从而让编码风格更加严格
### 命名与合法字

Devicetree 规范允许节点名和属性名使用范围很广的字符，但本编码风格收窄了该范围，以获得
更好的代码可读性
1. 节点名和属性名只能使用以下字符
   - 小写字符：[a-z]
   - 数字：[0-9]
   - 短横线：-

2. 标签只能使用以下字符
   - 小写字符：[a-z]
   - 数字：[0-9]
   - 下划线：_

3. 除非总线另有规定，单元地址应使用小写十六进制数字，不带前导零（不补零）
4. 属性中的十六进制值（例如 "reg"）应使用小写十六进制。地址部分可以用前导零补齐
```

	gpi_dma2: dma-controller@a00000 {
		compatible = "qcom,sm8550-gpi-dma", "qcom,sm6350-gpi-dma";
		reg = <0x0 0x00a00000 0x0 0x60000>;
	}

```
### 节点顺序


1. 任何总线上的节点（因此子节点使用单元地址）应按单元地址升序排列   对于某些子架构，也可以将同类型的节点分组放在一起，例如所I2C 控制器依次排列，
   即使这会打破单元地址的顺序
2. 不带单元地址的节点应按节点名的字母数字顺序排列。对于少数节点类型，可以按主属   排列，例如按 "pins" 属性的值对引脚配置状态排序
3. 在板DTS 中通过 &label 扩展节点时，条目的顺序应为字母数字顺序，或保DTSI 中的
   顺序，具体选择取决于子架构
上述排序规则易于在审阅阶段强制执行，减少了向同一文件同时添加新节点时发生冲突的机会，
并有助于DTS 源文件中导航
```

	/* SoC DTSI */

	/ {
		cpus {
			/* ... */
		};

		psci {
			/* ... */
		};

		soc@0 {
			dma: dma-controller@10000 {
				/* ... */
			};

			clk: clock-controller@80000 {
				/* ... */
			};
		};
	};

	/* Board DTS - 字母顺序 */

	&clk {
		/* ... */
	};

	&dma {
		/* ... */
	};

	/* Board DTS - 替代顺序，保持与 DTSI 一*/

	&dma {
		/* ... */
	};

	&clk {
		/* ... */
	};

```
### 设备节点中属性的顺序


设备节点中建议采用以下属性顺序：

1. "compatible"
2. "reg"
3. "ranges"
4. 标准/通用属性（由通用绑定定义，例如不带厂商前缀的）
5. 厂商特定属6. "status"（如适用），若其前面有内容则用空行隔开
7. 子节点，每个节点前用一个空行隔开

"status" 属性默认是 "okay"，因此可以省略
上述排序遵循如下思路
1. 最重要的属性放在节点开头：先是 compatible，然后是总线寻址以匹配单元地址2. 每个节点会在相似的位置拥有通用属性3. status 是最后的信息，用于标注该设备节点是否已完成（需要板级资源）
每组内的各个属性应按属性名使用自然排序
```

	/* SoC DTSI */

	device_node: device-class@6789abc {
		compatible = "vendor,device";
		reg = <0x0 0x06789abc 0x0 0xa123>;
		ranges = <0x0 0x0 0x06789abc 0x1000>;
		#dma-cells = <1>;
		clocks = <&clock_controller 0>, <&clock_controller 1>;
		clock-names = "bus", "host";
		#address-cells = <1>;
		#size-cells = <1>;
		vendor,custom-property = <2>;

		status = "disabled";

		child_node: child-class@100 {
			reg = <0x100 0x200>;
			/* ... */
		};
	};

	/* Board DTS */

	&device_node {
		vdd-0v9-supply = <&board_vreg1>;
		vdd-1v8-supply = <&board_vreg4>;
		vdd-3v3-supply = <&board_vreg2>;
		vdd-12v-supply = <&board_vreg3>;

		status = "okay";
	}

```
### 缩进与换

1. 缩进和换行遵Documentation/process/coding-style.rst2. 具有多个单元的数组中的每个条目（例如带两IO 地址"reg"）都应包含在 <> 中3. 对于跨多行的数组，最好在条目边界处拆分，并使续行的条目与首行的起< 对齐   通常避免拆分单个条目，除非它们显著超出换行限制
```

	thermal-sensor@c271000 {
		compatible = "qcom,sm8550-tsens", "qcom,tsens-v2";
		reg = <0x0 0x0c271000 0x0 0x1000>,
		      <0x0 0x0c222000 0x0 0x1000>;
		/* 超过编码风格换行限制的行*/
		interconnects = <&aggre1_noc MASTER_USB3_0 0 &mc_virt SLAVE_EBI1 0>,
				<&gem_noc MASTER_APPSS_PROC 0 &config_noc SLAVE_USB3_0 0>;
	};

```
### 缁勭粐 DTSI 鍜?DTS


DTSI DTS 文件应以便于表示硬件通用、可复用部分的方式组织。通常，这意味着DTSI DTS 文件组织为几个文件：

1. 包含整个 SoC 内容DTSI，不包含 SoC 上不存在的硬件节点2. 如适用：包含硬件通用或可复用部分DTSI，例如整个系统级模块（System-on-Module）3. 表示电路板的 DTS
电路板上存在的硬件组件应放在板级 DTS 中，而不SoC SoM DTSI 中。一个部分例是通用的外部参SoC 输入时钟，它可以作为 fixed-clock 编码SoC DTSI 中，其频率由板级 DTS 提供