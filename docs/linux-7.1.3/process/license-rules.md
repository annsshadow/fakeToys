## Linux 内核许可规则


Linux 内核仅依GNU 通用公共许可证第 2 版（GPL-2.0）的条款提供，如
LICENSES/preferred/GPL-2.0 所示，并带LICENSES/exceptions/Linux-syscall-note
中所述的明确系统调用例外，详COPYING 文件

本文件说明了应如何为每个源文件添加注释，以使其许可证清晰且无歧义。它不取代内核的许可证

COPYING 文件中描述的许可证适用于整个内核源代码，不过单个源文件可以具有不同的许可证
```

    GPL-1.0+  :  GNU General Public License v1.0 or later
    GPL-2.0+  :  GNU General Public License v2.0 or later
    LGPL-2.0  :  GNU Library General Public License v2 only
    LGPL-2.0+ :  GNU Library General Public License v2 or later
    LGPL-2.1  :  GNU Lesser General Public License v2.1 only
    LGPL-2.1+ :  GNU Lesser General Public License v2.1 or later

```
除此之外，单个文件可以在双重许可证下提供，例如某个兼容的 GPL 变体，或者替代地采用 BSD、MIT 等宽松许可证

描述用户空间程序与内核之间接口的用户空间 API（UAPI）头文件是一种特殊情况。根据内COPYING 文件中的说明，系统调用接口是一条清晰的边界，它不会GPL 要求扩展到任何使用该接口与内核通信的软件。由UAPI 头文件必须能够被包含到任何用于创建在 Linux 内核上运行的可执行文件的源文件中，该例外必须通过特殊的许可证表达式来记录

表达源文件许可证的常用方式是将相应的样板文本添加到文件顶部的注释中。由于格式、拼写错误等原因，这些“样板文本”难以被许可证合规相关的工具验证

样板文本的替代方案是在每个源文件中使用软件包数据交换（Software Package Data Exchange，SPDX）许可证标识符。SPDX 许可证标识符是机器可解析的精确简写，用于表示文件内容所基于的许可证。SPDX 许可证标识符Linux 基金会的 SPDX 工作组管理，并已由业界合作伙伴、工具厂商和法律团队共同商定。更多信息请参阅 https://spdx.org/

Linux 内核要求在所有源文件中使用精确的 SPDX 标识符。内核中使用的有效标识符`License identifiers`_ 一节中说明，它们与许可证文本一起从 https://spdx.org/licenses/ 的官SPDX 许可证列表中获取

### 许可证标识符语法


1. 位置

   SPDX 许可证标识符应添加到文件中能够包含注释的第一行可能的位置。对大多数文件而言这是第一行，但需要在第一行写 '#!PATH_TO_INTERPRETER' 的脚本除外。对于这些脚本，SPDX 许可证标识符放在第二行

   随后如需，可在许可证标识符行之后跟一行或多行 SPDX-FileCopyrightText

|

2. 风格

   SPDX 许可证标识符以注释的形式添加。注
```

      C source:	// SPDX-License-Identifier: <SPDX License Expression>
      C header:	/* SPDX-License-Identifier: <SPDX License Expression> */
      ASM:	/* SPDX-License-Identifier: <SPDX License Expression> */
      scripts:	# SPDX-License-Identifier: <SPDX License Expression>
      .rst:	.. SPDX-License-Identifier: <SPDX License Expression>
      .dts{i}:	// SPDX-License-Identifier: <SPDX License Expression>

   If a specific tool cannot handle the standard comment style, then the
   appropriate comment mechanism which the tool accepts shall be used. This
   is the reason for having the "/\* \*/" style comment in C header
   files. There was build breakage observed with generated .lds files where
   'ld' failed to parse the C++ comment. This has been fixed by now, but
   there are still older assembler tools which cannot handle C++ style
   comments.

```
|

3. 语法

   一<SPDX License Expression>（SPDX 许可证表达式）要么是 SPDX 许可证列表中找到SPDX 简短形式许可证标识符，要么是在存在许可证例外时由两个用 "WITH" 分隔SPDX 简短形式许可证标识符组合而成。当多个许可证适用时，表达式由关键"AND"OR" 分隔子表达式，并"(")" 包围

   对于带有 'or later'（或更高版本）选项[L]GPL 等许可证的标识符
```

      // SPDX-License-Identifier: GPL-2.0+
      // SPDX-License-Identifier: LGPL-2.1+

   WITH should be used when there is a modifier to a license needed.
   For example, the linux kernel UAPI files use the expression::

      // SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
      // SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note

   Other examples using WITH exceptions found in the kernel are::

      // SPDX-License-Identifier: GPL-2.0 WITH mif-exception
      // SPDX-License-Identifier: GPL-2.0+ WITH GCC-exception-2.0

   Exceptions can only be used with particular License identifiers. The
   valid License identifiers are listed in the tags of the exception text
   file. For details see the point `Exceptions`_ in the chapter `License
   identifiers`_.

   OR should be used if the file is dual licensed and only one license is
   to be selected.  For example, some dtsi files are available under dual
   licenses::

      // SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause

   Examples from the kernel for license expressions in dual licensed files::

      // SPDX-License-Identifier: GPL-2.0 OR MIT
      // SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
      // SPDX-License-Identifier: GPL-2.0 OR Apache-2.0
      // SPDX-License-Identifier: GPL-2.0 OR MPL-1.1
      // SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT
      // SPDX-License-Identifier: GPL-1.0+ OR BSD-3-Clause OR OpenSSL

   AND should be used if the file has multiple licenses whose terms all
   apply to use the file. For example, if code is inherited from another
   project and permission has been given to put it in the kernel, but the
   original license terms need to remain in effect::

      // SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) AND MIT

   Another other example where both sets of license terms need to be
   adhered to is::

      // SPDX-License-Identifier: GPL-1.0+ AND LGPL-2.1+

```
### 许可证标识符


当前使用的许可证，以及为内核添加代码所使用的许可证，可以分为：

1. _`首选许可证`

   应尽可能使用这些许可证，因为它们已被确认为完全兼容且被广泛使用。这些许可证取自
```

      LICENSES/preferred/

   内核源代码树

   该目录中的文件包含完整的许可证文本和 `Metatags`_。文件名与源文件中用于该许可证的 SPDX 许可证标识符完全相同

   Examples::

      LICENSES/preferred/GPL-2.0

   Contains the GPL version 2 license text and the required metatags::

      LICENSES/preferred/MIT

   Contains the MIT license text and the required metatags

   _`Metatags`锛?

   许可证文件中必须包含以下元标签：

   - Valid-License-Identifier（有效许可证标识符）

     一行或多行，声明在项目内哪些许可证标识符可用来引用此特定许可证文本。通常这是一个有效的标识符，但对于带'or later' 选项的许可证，则有两个有效标识符

   - SPDX-URL锛?

     包含该许可证相关附加信息SPDX 页面URL

   - Usage-Guidance（使用指导）

     用于使用建议的自由格式文本。文本必须包含正确的 SPDX 许可证标识符示例，正如它们应`License identifier syntax`_ 指南放入源文件那样

   - License-Text（许可证文本）：

     此标签之后的所有文本均被视为原始许可证文本

   File format examples::

      Valid-License-Identifier: GPL-2.0
      Valid-License-Identifier: GPL-2.0+
      SPDX-URL: https://spdx.org/licenses/GPL-2.0.html
      Usage-Guide:
        To use this license in source code, put one of the following SPDX
	tag/value pairs into a comment according to the placement
	guidelines in the licensing rules documentation.
	For 'GNU General Public License (GPL) version 2 only' use:
	  SPDX-License-Identifier: GPL-2.0
	For 'GNU General Public License (GPL) version 2 or any later version' use:
	  SPDX-License-Identifier: GPL-2.0+
      License-Text:
        Full license text

   ::

      SPDX-License-Identifier: MIT
      SPDX-URL: https://spdx.org/licenses/MIT.html
      Usage-Guide:
	To use this license in source code, put the following SPDX
	tag/value pair into a comment according to the placement
	guidelines in the licensing rules documentation.
	  SPDX-License-Identifier: MIT
      License-Text:
        Full license text

```
|

2. 已弃用许可证

   这些许可证只应用于现有代码或从其他项目导入的代码。这些许可证取自
```

      LICENSES/deprecated/

   内核源代码树

   该目录中的文件包含完整的许可证文本和 `Metatags`_。文件名与源文件中用于该许可证的 SPDX 许可证标识符完全相同

   Examples::

      LICENSES/deprecated/ISC

   Contains the Internet Systems Consortium license text and the required
   metatags::

      LICENSES/deprecated/GPL-1.0

   Contains the GPL version 1 license text and the required metatags.

   Metatags:

   'other'（其他）许可证的元标签要求与 `Preferred licenses`_（首选许可证）的要求相同

   File format example::

      Valid-License-Identifier: ISC
      SPDX-URL: https://spdx.org/licenses/ISC.html
      Usage-Guide:
        Usage of this license in the kernel for new code is discouraged
	and it should solely be used for importing code from an already
	existing project.
        To use this license in source code, put the following SPDX
	tag/value pair into a comment according to the placement
	guidelines in the licensing rules documentation.
	  SPDX-License-Identifier: ISC
      License-Text:
        Full license text

```
|

3. 仅双重许

   这些许可证只应用于与另一许可证（除首选许可证外）一起对代码进行双重许可。这些许可证取自
```

      LICENSES/dual/

   内核源代码树

   该目录中的文件包含完整的许可证文本和 `Metatags`_。文件名与源文件中用于该许可证的 SPDX 许可证标识符完全相同

   Examples::

      LICENSES/dual/MPL-1.1

   Contains the Mozilla Public License version 1.1 license text and the
   required metatags::

      LICENSES/dual/Apache-2.0

   Contains the Apache License version 2.0 license text and the required
   metatags.

   Metatags:

   'other' 许可证的元标签要求与 `Preferred licenses`_ 的要求相同

   File format example::

      Valid-License-Identifier: MPL-1.1
      SPDX-URL: https://spdx.org/licenses/MPL-1.1.html
      Usage-Guide:
        Do NOT use. The MPL-1.1 is not GPL2 compatible. It may only be used for
        dual-licensed files where the other license is GPL2 compatible.
        If you end up using this it MUST be used together with a GPL2 compatible
        license using "OR".
        To use the Mozilla Public License version 1.1 put the following SPDX
        tag/value pair into a comment according to the placement guidelines in
        the licensing rules documentation:
      SPDX-License-Identifier: MPL-1.1
      License-Text:
        Full license text

```
|

4. _`例外`

   某些许可证可以通过例外进行修订，以授予原始许可证所不赋予的特定权利。这些例外取
```

      LICENSES/exceptions/

   内核源代码树。该目录中的文件包含完整的例外文本和所需`Exception Metatags`_（例外元标签）

   Examples::

      LICENSES/exceptions/Linux-syscall-note

   Contains the Linux syscall exception as documented in the COPYING
   file of the Linux kernel, which is used for UAPI header files.
   e.g. /\* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note \*/::

      LICENSES/exceptions/GCC-exception-2.0

   Contains the GCC 'linking exception' which allows to link any binary
   independent of its license against the compiled version of a file marked
   with this exception. This is required for creating runnable executables
   from source code which is not compatible with the GPL.

   _`Exception Metatags`（例外元标签）：

   例外文件中必须包含以下元标签

   - SPDX-Exception-Identifier（SPDX 例外标识符）

     一个可SPDX 许可证标识符一起使用的例外标识符

   - SPDX-URL锛?

     包含该例外相关附加信息的 SPDX 页面URL

   - SPDX-Licenses锛?

     该例外可使用SPDX 许可证标识符的逗号分隔列表

   - Usage-Guidance（使用指导）

     用于使用建议的自由格式文本。文本之后必须跟有正确的 SPDX 许可证标识符示例，正如它们应`License identifier syntax`_ 指南放入源文件那样

   - Exception-Text（例外文本）

     此标签之后的所有文本均被视为原始例外文

   File format examples::

      SPDX-Exception-Identifier: Linux-syscall-note
      SPDX-URL: https://spdx.org/licenses/Linux-syscall-note.html
      SPDX-Licenses: GPL-2.0, GPL-2.0+, GPL-1.0+, LGPL-2.0, LGPL-2.0+, LGPL-2.1, LGPL-2.1+
      Usage-Guidance:
        This exception is used together with one of the above SPDX-Licenses
	to mark user-space API (uapi) header files so they can be included
	into non GPL compliant user-space application code.
        To use this exception add it with the keyword WITH to one of the
	identifiers in the SPDX-Licenses tag:
	  SPDX-License-Identifier: <SPDX-License> WITH Linux-syscall-note
      Exception-Text:
        Full exception text

   ::

      SPDX-Exception-Identifier: GCC-exception-2.0
      SPDX-URL: https://spdx.org/licenses/GCC-exception-2.0.html
      SPDX-Licenses: GPL-2.0, GPL-2.0+
      Usage-Guidance:
        The "GCC Runtime Library exception 2.0" is used together with one
	of the above SPDX-Licenses for code imported from the GCC runtime
	library.
        To use this exception add it with the keyword WITH to one of the
	identifiers in the SPDX-Licenses tag:
	  SPDX-License-Identifier: <SPDX-License> WITH GCC-exception-2.0
      Exception-Text:
        Full exception text


```
所SPDX 许可证标识符和例外都必须LICENSES 子目录中有对应的文件。这是为了让工具能够进行验证（例checkpatch.pl），并让许可证可以直接从源代码中读取和提取，这也是各类自由开源软件（FOSS）组织（例如 `FSFE REUSE initiative <https://reuse.software/>`_）所推荐的

### _`MODULE_LICENSE`


   可加载内核模块还需要一MODULE_LICENSE() 标签。该标签既不能替代正确的源代码许可证信息（SPDX-License-Identifier），也绝不用于表达或确定模块源代码所基于的确切许可证

   此标签的唯一目的是为内核模块加载器和用户空间工具提供充足的信息，以判断该模块是自由软件还是专有软件

   MODULE_LICENSE() 的有效许可证字符串如下：

    ============================= =============================================
    "GPL"			 模块依据 GPL 2 版许可。这并不区分
				  GPL-2.0-only GPL-2.0-or-later。确切的
				  许可证信息只能通过相应源文件中的许可证
				  信息来确定

    "GPL v2"			 "GPL" 相同。因其历史原因而存在

    "GPL and additional rights"   用于表达模块源代码在 GPL v2 变体MIT
				  许可证下双重许可的历史变体。请勿在新代
				  中使用

    "Dual MIT/GPL"		 表达模块GPL v2 变体MIT 许可证中
				  选择双重许可的正确方式

    "Dual BSD/GPL"		 模块GPL v2 变体BSD 许可证中选择双重
				  许可。BSD 许可证的确切变体只能通过相应
				  源文件中的许可证信息来确定

    "Dual MPL/GPL"		 模块GPL v2 变体Mozilla 公共许可
				  （MPL）中选择双重许可。MPL 许可证的确切
				  变体只能通过相应源文件中的许可证信息
				  确定

    "Proprietary"		 模块依据专有许可证Proprietary" 仅应理解
				  “该许可证与 GPLv2 不兼容”。此字符串仅用于
				  GPL2 兼容的第三方模块，不能用于源代码
				  位于内核树中的模块。以此标记的模块在加
				  时会'P' 标志污染内核，且内核模块加载
				  拒绝将此类模块链接到EXPORT_SYMBOL_GPL()
				  导出的符号
    ============================= =============================================
