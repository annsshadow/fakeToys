## GCC 插件基础设施


## 简介


GCC 插件是可加载模块，为编译器 [^1^]_ 提供额外特性。它们对运行时插桩与静态分析很有用。
我们可以在编译期间通过回调 [^2^]_、GIMPLE [^3^]_、IPA [^4^]_ 与 RTL passes [^5^]_ 来分析、
修改并添加更多代码。

内核的 GCC 插件基础设施支持构建树外模块、交叉编译以及在独立目录中构建。插件源文件必须
能够被 C++ 编译器编译。

目前 GCC 插件基础设施仅支持部分架构。请 grep "select HAVE_GCC_PLUGINS" 来查明哪些架构
支持 GCC 插件。

此基础设施移植自 grsecurity [^6^]_ 与 PaX [^7^]_。

--

## 目的


GCC 插件旨在提供一个用于试验潜在编译器特性的场所，这些特性在 GCC 与 Clang 上游中都不存在。
一旦证明其实用性，目标就是将该特性并入 GCC（与 Clang）上游，然后最终在受支持的所有 GCC
版本都提供该特性后，将其从内核中移除。

具体而言，新插件应只实现在上游编译器中（无论是 GCC 还是 Clang）没有支持的特性。

当一个特性存在于 Clang 而不存在于 GCC 时，应努力将该特性引入上游 GCC（而不是仅仅作为一个
内核专用的 GCC 插件），以便整个生态都能从中受益。

类似地，即便某个由 GCC 插件提供的特性在 Clang 中**不**存在，但只要该特性被证明有用，也应
投入精力将其并入 GCC（与 Clang）上游。

在某个特性于上游 GCC 中可用后，该插件将变得无法对应 GCC 版本（及之后版本）构建。一旦所有
内核支持的 GCC 版本都提供了该特性，该插件将从内核中移除。

## 文件


**$(src)/scripts/gcc-plugins**

	这是 GCC 插件的目录。

**$(src)/scripts/gcc-plugins/gcc-common.h**

	这是一个 GCC 插件的兼容头文件。应始终包含它，而不是各个独立的 gcc 头文件。

**$(src)/scripts/gcc-plugins/gcc-generate-gimple-pass.h,
$(src)/scripts/gcc-plugins/gcc-generate-ipa-pass.h,
$(src)/scripts/gcc-plugins/gcc-generate-simple_ipa-pass.h,
$(src)/scripts/gcc-plugins/gcc-generate-rtl-pass.h**

	这些头文件自动生成 GIMPLE、SIMPLE_IPA、IPA 与 RTL passes 的注册结构。
	应优先使用它们，而非手工创建这些结构。

## 用法


你必须为你的 gcc 版本安装 gcc 插件头文件，
```

	apt-get install gcc-10-plugin-dev

```
```

	dnf install gcc-plugin-devel libmpc-devel

```
```

	dnf install libmpc-devel

```
启用 GCC 插件基础设施以及你想使用的某些插件
```

	CONFIG_GCC_PLUGINS=y
	CONFIG_GCC_PLUGIN_LATENT_ENTROPY=y
	...

```
```

	gcc -print-file-name=plugin
	CROSS_COMPILE=arm-linux-gnu- ${CROSS_COMPILE}gcc -print-file-name=plugin

```
```

	plugin

```
```

       /usr/lib/gcc/x86_64-redhat-linux/12/plugin

```
```

	make scripts

```
或者直接运行内核 make，并使用环路复杂度（cyclomatic complexity）GCC 插件编译整个内核。

## 4. 如何添加一个新的 GCC 插件


GCC 插件位于 scripts/gcc-plugins/ 中。你需要将插件源文件直接放在 scripts/gcc-plugins/ 下。
不支持创建子目录。它必须被添加到 scripts/gcc-plugins/Makefile、scripts/Makefile.gcc-plugins
以及一个相关的 Kconfig 文件中。
