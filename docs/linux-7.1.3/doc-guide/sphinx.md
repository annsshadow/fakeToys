
## 使用 Sphinx 生成内核文档

Linux 内核使用 `Sphinx`_ `Documentation` 下的 `reStructuredText`_ 文件生成
美观的文档。要HTML PDF 格式构建文档，可使用 `make htmldocs` `make pdfdocs`生成的文档被放置`Documentation/output` 中
reStructuredText 文件可能包含用于从源文件包含结构化文档注释（kernel-doc 注释）的指令通常这些用于描述代码的函数、类型与设计。kernel-doc 注释有一些特殊的结构与格式，但除此之它们也被视为 reStructuredText
最后，`Documentation` 周围散布着数以千计的纯文本文档文件。其中一些很可能随着时间的推被转换为 reStructuredText，但其中的大部分将保持纯文本形式

## 安装 Sphinx

Documentation/ 文件当前使用ReST 标记旨在使用 `Sphinx` 3.4.3 或更高版本构建
有一个脚本用于检Sphinx 的需求。更多细节请参阅 sphinx-pre-install
大多数发行版都附Sphinx，但其工具链较为脆弱，升级它或机器上的某些其Python 导致文档构建失败的情况并不少见
避免这一点的一种方法是使用与发行版所附带版本不同的版本。为此，建议在虚拟环境中安装 Sphinx使用 `virtualenv-3` `virtualenv`，具体取决于你的发行版如何打Python 3
总之，如果你想安装最新版本的 Sphinx，可以：

```
       $ virtualenv sphinx_latest
       $ . sphinx_latest/bin/activate
       (sphinx_latest) $ pip install -r Documentation/sphinx/requirements.txt

```
运行 `. sphinx_latest/bin/activate` 后，提示符会改变，以指示你正在使用新的环境如果你打开一个新shell，需要在构建文档之前重新运行该命令以再次进入虚拟环境
### 图像输出

内核文档构建系统包含一个处GraphViz SVG 两种格式图像的扩展（参见 sphinx_kfigure）
要使其工作，你需要安GraphViz ImageMagick 两个包。如果未安装这些包，构建系统仍会
构建文档，但不会在输出中包含任何图像
### PDF LaTeX 构建

此类构建当前仅支Sphinx 2.4 及更高版本
对于 PDF LaTeX 输出，你还需`XeLaTeX` 3.14159265 版本
根据发行版的不同，你可能还需要安装一系列 `texlive` 包，以提`XeLaTeX` 工作所需最小功能集
### HTML 中的数学表达
一ReST 页面包含数学表达式。由Sphinx 的工作方式，这些表达式使LaTeX 记法书写Sphinx 有两种选项来在 html 输出中渲染数学表达式。一种是名为 `imgmath`_ 的扩展，它将
数学表达式转换为图像并嵌入到 html 页面中。另一种是名为 `mathjax`_ 的扩展，它将数学渲染
委托给支JavaScript Web 浏览器。前者是 6.1 之前内核文档的唯一选项，它需要相当多texlive 包，其中包括 amsfonts amsmath 等
自内6.1 版本起，包含数学表达式的 html 页面可以在不安装任何 texlive 包的情况下构建更多信息请参`Choice of Math Renderer`_

### 检Sphinx 依赖
有一个脚本会自动检Sphinx 依赖项。如果它能识别你的发行版，它还会给出
安装提示
```
	$ ./tools/docs/sphinx-pre-install
	Checking if the needed tools for Fedora release 26 (Twenty Six) are available
	Warning: better to also install "texlive-luatex85".
	You should run:

		sudo dnf install -y texlive-luatex85
		/usr/bin/virtualenv sphinx_2.4.4
		. sphinx_2.4.4/bin/activate
		pip install -r Documentation/sphinx/requirements.txt

	Can't build as 1 mandatory dependency is missing at ./tools/docs/sphinx-pre-install line 468.

```
默认情况下，它会检html PDF 的全部需求，包括图像、数学表达式LaTeX 构建的需求，
并假定将使用 Python 虚拟环境。用html 构建的需求被假定为强制的；其它则为可选的
它支持两个可选参数：

`--no-pdf`
	禁用PDF 的检查；

`--no-virtualenv`
	使用操作系统打包Sphinx，而非 Python 虚拟环境
### 安装 Sphinx 最小版
在更Sphinx 构建系统时，确保最小版本仍受支持很重要。如今，在现代发行版上这样做正变愈发困难，因为无法在 Python 3.13 及以上版本中安装
可以使用 Documentation/process/changes.rst 中定义的最低受支持 Python 版本进行测试方法为创建：

```
	/usr/bin/python3.9 -m venv sphinx_min
	. sphinx_min/bin/activate
	pip install -r Documentation/sphinx/min_requirements.txt

```
可以使用以下方式做更全面的测试：

	tools/docs/test_doc_build.py

该脚本为每个受支持的版本创建一Python venv，并可选地为一系列 Sphinx 版本构建文档

## 构建 Sphinx 文档

生成文档的通常方式是运`make htmldocs` `make pdfdocs`。还有其它可用格式：
请参`make help` 的文档部分。生成的文档被放置在 `Documentation/output` 下特定于格式子目录中
要生成文档，显然必须安装 Sphinx（`sphinx-build`）。对PDF 输出，你还需要来ImageMagick
`XeLaTeX` `convert(1)` (https://www.imagemagick.org)。\ [#ink]_ 这些都能广泛获取并由发行版打包
要向 Sphinx 传递额外选项，可以使`SPHINXOPTS` make 变量。例如，使用
`make SPHINXOPTS=-v htmldocs` 可获得更详细的输出
也可以通过使用 `DOCS_CSS` make 变量传入额外DOCS_CSS 覆盖文件，以自定html 布局
默认情况下，构建 HTML 文档使用 "Alabaster" 主题；该主题Sphinx 一同提供，无需单独安装Sphinx 主题可以通过使用 `DOCS_THEME` make 变量来覆盖

   有些人可能更喜欢html 输出使用 RTD 主题。根Sphinx 版本的不同，它应使用
   `pip install sphinx_rtd_theme` 单独安装
还有另一make 变量 `SPHINXDIRS`，在测试构建文档子集时很有用。例如，你可以通过运行
`make SPHINXDIRS=doc-guide htmldocs` 来构`Documentation/doc-guide` 下的文档`make help` 的文档部分会显示你可以指定的子目录列表
要移除生成的文档，运`make cleandocs`
	 同样也能改善嵌入 PDF 文档的图像质量，尤其是对于内5.18 及更高版本
### 数学渲染器的选择

自内6.1 版本起，mathjax 作为 html 输出数学渲染器的回退方案工作。\ [#sph1_8]_

数学渲染器根据可用命令选择，如下所示：


    ============= ================= ============
    Math renderer Required commands Image format
    ============= ================= ============
    imgmath       latex, dvipng     PNG (raster)
    mathjax
    ============= ================= ============


可以通过以下方式设置环境变量 `SPHINX_IMGMATH` 来覆盖该选择

    ====================== ========
    Setting                Renderer
    ====================== ========
    `SPHINX_IMGMATH=yes` imgmath
    `SPHINX_IMGMATH=no`  mathjax
    ====================== ========



## 编写文档

添加新文档可以很简单：

1. `Documentation` 下的某处添加一个新 `.rst` 文件2. `Documentation/index.rst` 中的 Sphinx `TOC tree`_ 引用它

这对于简单文档（就像你现在正在阅读的这份）通常已经足够，但对于较大的文档，建议创建一子目录（或使用已有的子目录）。例如，图形子系统文档位`Documentation/gpu`，拆分为若干
`.rst` 文件，并拥有自身单独`index.rst`（带有自己的 `toctree`），由主索引引用
关于你可以用 Sphinx reStructuredText 做什么，请参`Sphinx`_ `reStructuredText`_
的文档。特别是，Sphinx `reStructuredText Primer`_ 是开始学reStructuredText 的好去处也有一`Sphinx specific markup constructs`_

### 内核文档的特定准
以下是针对内核文档的一些特定准则：

- 请不要过度使reStructuredText 标记。保持简单。在大多数情况下，文档应为纯文本  只需在格式上保持足够的一致性，以便能转换为其它格式
- 在将现有文档转换reStructuredText 时，请尽量保持格式改动最小
- 在转换文档时，也要更新内容，而不仅仅是格式
- 请遵循以下标题装饰符的顺序：

```

       ==============
       Document title
       ==============

  2. ``=`` for chapters::

       Chapters
       ========

  3. ``-`` for sections::

       Section
       -------

  4. ``~`` for subsections::

       Subsection
       ~~~~~~~~~~

  Although RST doesn't mandate a specific order ("Rather than imposing a fixed
  number and order of section title adornment styles, the order enforced will be
  the order as encountered."), having the higher levels the same overall makes
  it easier to follow the documents.

```

- 对于插入固定宽度的文本块（用于代码示例、用例示例等），对不真正受益于语法高亮的内容
  （尤其是短片段）使用 `::`。对受益于高亮的较长代码块使`.. code-block:: <language>`  对于嵌入文本中的短代码片段，使用 \`\`

### C 鍩。
**Sphinx C *（名c）适用C API 的文档。例如一个函数原型：


    .. c:function:: int ioctl( int fd, int request )

kernel-doc C 域有一些附加特性。例如，你可以用 `open` `ioctl` 这样的通用名称
**重命*一个函数的引用名：


     .. c:function:: int ioctl( int fd, int request )
        :name: VIDIOC_LOG_STATUS

func-name（例ioctl）保留在输出中，ref-name `ioctl` 更改`VIDIOC_LOG_STATUS`该函数的索引条目也随之更改为 `VIDIOC_LOG_STATUS`
请注意，无需使用 `c:func:` 来生成到函数文档的交叉引用。由于某Sphinx 扩展的魔法，
如果给定函数名存在索引条目，文档构建系统会自动将`function()` 的引用转换为交叉引用如果你在内核文档中看`c:func:` 的使用，请随意将其移除
### 表格

reStructuredText 为表格语法提供了若干选项。内核表格风格倾向于使*简单表*语法**网格表格**语法。更多细节请参阅 `reStructuredText user reference for table syntax`_
   https://docutils.sourceforge.io/docs/user/rst/quickref.html#tables

#### 列表表格

list-table 格式对于不易用通常Sphinx ASCII 字符画格式排布的表格很有用。不过，对于
纯文本文档的读者而言，这些格式几乎无法理解，在没有充分理由的情况下应避免使用
`flat-table` 是一个类似于 `list-table` 的两级列表，带有一些附加特性：

- column-span：通过角色 `cspan`，一个单元格可扩展到额外的列

- row-span：通过角色 `rspan`，一个单元格可扩展到额外的行

- 自动将表格行最右侧的单元格跨过该表格行右侧缺失的单元格。通过选项 `:fill-cells:` 可将  行为*自动跨列（auto span*更改*自动填充（auto fill*，即自动插入（空）单元格  而非跨接最后一个单元格
options锛。
- `:header-rows:`   [int] 表头行数
- `:stub-columns:`  [int] 存根列数
- `:widths:`        [[int] [int] ... ] 列宽
- `:fill-cells:`    自动插入缺失单元格，而非自动跨接缺失单元
roles锛。
- `:cspan:` [int] 额外列数*morecols**- `:rspan:` [int] 额外行数*morerows**
下面的示例展示了如何使用此标记。分级列表的第一级是 **table-row**。在 **table-row** 只允许一种标记，即该 **table-row** 中单元格的列表。例外是 **comments**`..` ）和
**targets**（例如对 ``last row <last row>`` 的引/ :ref:`last row <last row>`）

   .. flat-table:: table title
      :widths: 2 1 1 3

      - - head col 1
        - head col 2
        - head col 3
        - head col 4

      - - row 1
        - field 1.1
        - field 1.2 with autospan

      - - row 2
        - field 2.1
        - `1` `1` field 2.2 - 3.3

      - .. _`last row`:

        - row 3

渲染为：

   .. flat-table:: table title
      :widths: 2 1 1 3

      - - head col 1
        - head col 2
        - head col 3
        - head col 4

      - - row 1
        - field 1.1
        - field 1.2 with autospan

      - - row 2
        - field 2.1
        - `1` `1` field 2.2 - 3.3

      - .. _`last row`:

        - row 3

### 交叉引用

从一个文档页交叉引用到另一个文档页，只需写出文档文件的路径即可，无需特殊语法路径可以是绝对路径或相对路径。对于绝对路径，"Documentation/" 开头。例如，要交叉引到本页，根据当前文档的目录（注意
```
    See Documentation/doc-guide/sphinx.rst. This always works.
    Take a look at sphinx.rst, which is at this same directory.
    Read ../sphinx.rst, which is one directory above.

```
如果你希望链接具有不同于文档路径的渲染文本，可以
```
    See :doc:`my custom link text for document sphinx <sphinx>`.

```
对于大多数用例，前者更受青睐，因为它更干净，更适合阅读源文件的人。如果你遇到没有带来任何
价值的 `:doc:` 用法，请随意将其转换为仅文档路径
关于交叉引用kernel-doc 函数或类型的信息，请参阅 Documentation/doc-guide/kernel-doc.rst
#### 引用提交

git 提交的引用会自动变为超链接，只要它们是：

```
    commit 72bf4f1767f0
    commit 72bf4f1767f0 ("net: do not leave an empty skb in write queue")

```

## 图表与图
如果你想添加图像，应使用 `kernel-figure` `kernel-image` 指令。例如，要插入一可缩放的图：

```
    .. kernel-figure::  svg_image.svg
       :alt:    simple SVG image

       SVG image example

```

   :alt:    simple SVG image

   SVG image example

内核图（和图像）指令支持 **DOT** 格式的文件，参见

- DOT: http://graphviz.org/pdf/dotguide.pdf
- Graphviz: http://www.graphviz.org/content/dot-language

```
  .. kernel-figure::  hello.dot
     :alt:    hello world

     DOT's hello world example

```

   :alt:    hello world

   DOT's hello world example

嵌入**render** 标记（或语言），Graphviz **DOT**，由
```
  .. kernel-render:: DOT
     :alt: foobar digraph
     :caption: Embedded **DOT** (Graphviz) code

     digraph foo {
      "bar" -> "baz";
     }

```
其渲染方式取决于所安装的工具。如果安装了 Graphviz，你会看到矢量图像。否则，原始标记会作**literal-block**（hello_dot_render）插入

   :alt: foobar digraph
   :caption: Embedded **DOT** (Graphviz) code

   digraph foo {
      "bar" -> "baz";
   }

**render** 指令拥有 **figure** 指令已知的所有选项，外加选项 `caption`。如`caption` 有值，
则插入一**figure** 节点；否则插入一**image** 节点。如果你想要引用它（hello_svg_render），
也都需要一`caption`
```
  .. kernel-render:: SVG
     :caption: Embedded **SVG** markup
     :alt: so-nw-arrow

     <?xml version="1.0" encoding="UTF-8"?>
     <svg xmlns="http://www.w3.org/2000/svg" version="1.1" ...>
        ...
     </svg>

```

   :caption: Embedded **SVG** markup
   :alt: so-nw-arrow

   <?xml version="1.0" encoding="UTF-8"?>
   <svg xmlns="http://www.w3.org/2000/svg"
     version="1.1" baseProfile="full" width="70px" height="40px" viewBox="0 0 700 400">
   <line x1="180" y1="370" x2="500" y2="50" stroke="black" stroke-width="15px"/>
   <polygon points="585 0 525 25 585 50" transform="rotate(135 525 25)"/>
   </svg>
