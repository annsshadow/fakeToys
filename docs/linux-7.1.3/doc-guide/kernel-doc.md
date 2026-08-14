
## 编写 kernel-doc 注释


Linux 内核源文件中可能包含采用 kernel-doc 格式的结构化文档注释，用于描述代码的函数、类型和设计。当文档嵌入在源文件中时，更容易保持文档与代码同步更新。

   gtk-doc 或 Doxygen 类似，但由于历史原因又明显不同。内核源码中包含数以万计的 kernel-doc 注释。请遵循此处描述的风格。

   请参阅 Documentation/rust/general-information.rst。

kernel-doc 结构会从注释中提取出来，并据此生成带锚点的、格式正确的 `Sphinx C Domain`_ 函数与类型描述。这些描述会经过特殊 kernel-doc 高亮与交叉引用的过滤处理。详见下文。


每个通过 `EXPORT_SYMBOL` 或 `EXPORT_SYMBOL_GPL` 导出给可加载模块的函数都应拥有 kernel-doc 注释。头文件中供模块使用的函数与数据结构同样应当拥有 kernel-doc 注释。

为其他内核文件可见（未标记为 `static`）的函数提供 kernel-doc 格式的文档也是一种良好的实践。我们还建议为私有（文件级 `static`）函数也提供 kernel-doc 格式文档，以保持内核源码布局的一致性。这属于较低优先级，由该内核源文件维护者自行决定。

### 如何格式化 kernel-doc 注释


kernel-doc 注释使用开头的注释标记 `/**`。`kernel-doc` 工具会提取以此方式标记的注释。注释的其余部分格式如同普通的多行注释，左侧有一列星号，并以独占一行的 `*/` 结束。

函数与类型的 kernel-doc 注释应紧邻所描述的函数或类型之前放置，以最大程度提高代码改动者同时更新文档的可能性。概述类（overview）的 kernel-doc 注释可以放在顶层缩进级别的任意位置。

以更高的详细程度运行 `kernel-doc` 工具且不实际生成输出，可用于验证 kernel-doc 注释格式是否正确：

```
	tools/docs/kernel-doc -v -none drivers/foo/bar.c
```

`.c` 文件的文档格式也会由内核构建过程验证：

```
	make W=n
```

不过，上述命令不会验证头文件。头文件应使用 `kernel-doc` 单独检查。

### 函数文档


```
  /**
   * function_name() - Brief description of function.
   * @arg1: Describe the first argument.
   * @arg2: Describe the second argument.
   *        One can provide multiple line descriptions
   *        for arguments.
   *
   * A longer description, with more discussion of the function function_name()
   * that might be useful to those using or modifying it. Begins with an
   * empty comment line, and may include additional embedded empty
   * comment lines.
   *
   * The longer description may have multiple paragraphs.
   *
   * Context: Describes whether the function can sleep, what locks it takes,
   *          releases, or expects to be held. It can extend over multiple
   *          lines.
   * Return: Describe the return value of function_name.
   *
   * The return value description can also have multiple paragraphs, and should
   * be placed at the end of the comment block.
   */
```

函数名之后的简要描述可以跨越多行，并在参数描述、空注释行或注释块结束时终止。

#### 函数参数


每个函数参数应按顺序紧随简短的函数描述之后进行描述。函数描述与参数之间、以及各参数之间不要留空行。

每个 `@argument:` 描述可以跨越多行。


   If the `@argument` description has multiple lines, the continuation
```
      * @argument: some long description
      *            that continues on next lines

   or::

      * @argument:
      *		some long description
      *		that continues on next lines
```

如果函数拥有数量可变的参数，其描述应为：

```
      * @...: description
```

#### 函数上下文


函数可被调用的上下文应在一个名为 `Context` 的小节中描述。其中应包括该函数是否会休眠、能否在中断上下文中调用，以及它获取、释放或期望其调用者持有哪些锁。

```
  * Context: Any context.
  * Context: Any context. Takes and releases the RCU lock.
  * Context: Any context. Expects <lock> to be held by caller.
  * Context: Process context. May sleep if @gfp flags permit.
  * Context: Process context. Takes and releases <mutex>.
  * Context: Softirq or process context. Takes and releases <lock>, BH-safe.
  * Context: Interrupt context.
```

#### 返回值


返回值（若有）应在一个名为 `Return`（或 `Returns`）的专用小节中描述。


  #) 你提供的多行描述性文本**不会**被识别为：

```
	* Return:
	* %0 - OK
	* %-EINVAL - invalid argument
	* %-ENOMEM - out of memory
```

     上述写法会被全部拼接在一起，产生：

```
	Return: 0 - OK -EINVAL - invalid argument -ENOMEM - out of memory
```

     因此，为了产生期望的换行，需要使用 ReST 列表，例如：

```
      * Return:
      * * %0		- OK to runtime suspend the device
      * * %-EBUSY	- Device should not be runtime suspended
```

  #) 如果你提供的描述性文本中有以“短语加冒号”开头的行，每个这样的短语都会被当作新的小节标题，这很可能无法产生期望的效果。

### 结构体、联合体与枚举文档


`struct`、`union` 与 `enum` 的 kernel-doc 通用格式为：

```
  /**
   * struct struct_name - Brief description.
   * @member1: Description of member1.
   * @member2: Description of member2.
   *           One can provide multiple line descriptions
   *           for members.
   *
   * Description of the structure.
   */
```

你可以将上述示例中的 `struct` 替换为 `union` 或 `enum` 来描述联合体或枚举。`member` 一词既用于指代 `struct` 和 `union` 的成员名，也用于指代 `enum` 中的枚举项。

结构体名之后的简要描述可以跨越多行，并在成员描述、空注释行或注释块结束时终止。

#### 成员


结构体、联合体与枚举的成员应如同函数参数一样进行文档化；它们紧跟简短描述之后，并且可以跨多行。

在 `struct` 或 `union` 描述内部，你可以使用 `private:` 与 `public:` 注释标签。位于 `private:` 区域内部的结构体字段不会在生成的输出文档中列出。

`private:` 与 `public:` 标签必须紧跟在 `/*` 注释标记之后开始。它们可以可选地包含位于 `:` 与结束的 `*/` 标记之间的注释。

当 `private:` 用于嵌套结构体时，它只会传播到内层结构体/联合体。


```
  /**
   * struct my_struct - short description
   * @a: first member
   * @b: second member
   * @d: fourth member
   *
   * Longer description
   */
  struct my_struct {
      int a;
      int b;
  /* private: internal use only */
      int c;
  /* public: the next one is public */
      int d;
  };
```

#### 嵌套结构体/联合体


```
      /**
       * struct nested_foobar - a struct with nested unions and structs
       * @memb1: first member of anonymous union/anonymous struct
       * @memb2: second member of anonymous union/anonymous struct
       * @memb3: third member of anonymous union/anonymous struct
       * @memb4: fourth member of anonymous union/anonymous struct
       * @bar: non-anonymous union
       * @bar.st1: struct st1 inside @bar
       * @bar.st2: struct st2 inside @bar
       * @bar.st1.memb1: first member of struct st1 on union bar
       * @bar.st1.memb2: second member of struct st1 on union bar
       * @bar.st2.memb1: first member of struct st2 on union bar
       * @bar.st2.memb2: second member of struct st2 on union bar
       */
      struct nested_foobar {
        /* Anonymous union/struct*/
        union {
          struct {
            int memb1;
            /* private: hides memb2 from documentation */
            int memb2;
          };
          /* Everything here is public again, as private scope finished */
          struct {
            void *memb3;
            int memb4;
          };
        };
        union {
          struct {
            int memb1;
            int memb2;
          } st1;
          struct {
            void *memb1;
            int memb2;
          } st2;
        } bar;
      };
```

   #) 在为嵌套结构体或联合体编写文档时，如果 `struct`/`union` `foo` 具名，则其内部的成员 `bar` 应记为 `@foo.bar:`。
   #) 当嵌套的 `struct`/`union` 为匿名时，其中的成员 `bar` 应记为 `@bar:`。

#### 行内成员文档注释


结构体成员也可以在其定义内部以行内方式编写文档。有两种风格：单行注释（开头 `/**` 与结尾 `*/` 位于同一行），以及多行注释（二者各占一行）：

```
  /**
   * struct foo - Brief description.
   * @foo: The Foo member.
   */
  struct foo {
        int foo;
        /**
         * @bar: The Bar member.
         */
        int bar;
        /**
         * @baz: The Baz member.
         *
         * Here, the member description may contain several paragraphs.
         */
        int baz;
        union {
                /** @foobar: Single line description. */
                int foobar;
        };
        /** @bar2: Description for struct @bar2 inside @foo */
        struct {
                /**
                 * @bar2.barbar: Description for @barbar inside @foo.bar2
                 */
                int barbar;
        } bar2;
  };
```

### Typedef 文档


```
  /**
   * typedef type_name - Brief description.
   *
   * Description of the type.
   */
```

```
  /**
   * typedef type_name - Brief description.
   * @arg1: description of arg1
   * @arg2: description of arg2
   *
   * Description of the type.
   *
   * Context: Locking context.
   * Returns: Meaning of the return value.
   */
   typedef void (*type_name)(struct v4l2_ctrl *arg1, void *arg2);
```

### 变量文档


```
  /**
   * var var_name - Brief description.
   *
   * Description of the var_name variable.
   */
   extern int var_name;
```

### 类对象宏文档


类对象宏（object-like macro）与类函数宏（function-like macro）不同。二者的区分在于：类函数宏的宏名是否紧接左圆括号 `'('`，类对象宏的宏名则不紧随左圆括号。

类函数宏由 `tools/docs/kernel-doc` 像函数一样处理。它们可能带有参数列表。类对象宏没有参数列表。

```
  /**
   * define object_name - Brief description.
   *
   * Description of the object.
   */
```

```
  /**
   * define MAX_ERRNO - maximum errno value that is supported
   *
   * Kernel pointers have redundant information, so we can use a
   * scheme where we can return either an error code or a normal
   * pointer with the same return value.
   */
  #define MAX_ERRNO	4095
```

```
  /**
   * define DRM_GEM_VRAM_PLANE_HELPER_FUNCS - \
   *	Initializes struct drm_plane_helper_funcs for VRAM handling
   *
   * This macro initializes struct drm_plane_helper_funcs to use the
   * respective helper functions.
   */
  #define DRM_GEM_VRAM_PLANE_HELPER_FUNCS \
	.prepare_fb = drm_gem_vram_plane_helper_prepare_fb, \
	.cleanup_fb = drm_gem_vram_plane_helper_cleanup_fb
```

### 高亮与交叉引用


以下特殊模式会在 kernel-doc 注释的描述性文本中被识别，并被转换为正确的 reStructuredText 标记与 `Sphinx C Domain`_ 引用。

	       注意：**不能**在普通的 reStructuredText 文档中使用。

`funcname()`
  函数引用。

`@parameter`
  函数参数的名称。（仅作格式化，不进行交叉引用。）

`%CONST`
  常量的名称。（仅作格式化，不进行交叉引用。）

```
    %0    %NULL    %-1    %-EFAULT    %-EINVAL    %-ENOMEM
```

```literal```
  一个应原样处理的字面块。输出将使用 `等宽字体`。

  如果你需要使用一些特殊字符（否则这些字符会被 kernel-doc 脚本或 reStructuredText 赋予特定含义），该语法特别有用。

  当你需要在函数描述中使用类似 `%ph` 这样的东西时，这尤其有用。

`$ENVVAR`
  环境变量的名称。（仅作格式化，不进行交叉引用。）

`&struct name`
  结构体引用。

`&enum name`
  枚举引用。

`&typedef name`
  Typedef 引用。

`&struct_name->member` 或 `&struct_name.member`
  `struct` 或 `union` 成员引用。交叉引用指向 `struct` 或 `union` 的定义，而非直接指向成员。

`&name`
  通用类型引用。建议优先使用上述完整引用形式。这主要用于遗留注释。

#### 从 reStructuredText 进行交叉引用


从 reStructuredText 文档中交叉引用 kernel-doc 注释里定义的函数与类型无需额外语法。只需在函数名后加上 `()`，并在类型前写上 `struct`、`union`、`enum` 或 `typedef` 即可。

```
  See foo().
  See struct foo.
  See union bar.
  See enum baz.
  See typedef meh.
```

不过，如果你希望交叉引用链接使用自定义文字，可以这样写：

```
  See :c:func:`my custom link text for function foo <foo>`.
  See :c:type:`my custom link text for struct bar <bar>`.
```

更多细节请参考 `Sphinx C Domain`_ 文档。

   变量不会被自动进行交叉引用。对于这些变量，你需要显式添加 C 域交叉引用。

### 概述文档注释


为了便于让源代码与注释彼此靠近，你可以包含 kernel-doc 文档块，它们是自由格式注释，而不是针对函数、结构体、联合体、枚举、typedef 或变量的 kernel-doc。例如，这可用于描述某个驱动或库代码的运行原理。

这通过使用带有小节标题的 `DOC:` 段关键字来实现。

```
  /**
   * DOC: Theory of Operation
   *
   * The whizbang foobar is a dilly of a gizmo. It can do whatever you
   * want it to do, at any time. It reads your mind. Here's how it works.
   *
   * foo bar splat
   *
   * The only drawback to this gizmo is that is can sometimes damage
   * hardware, software, or its subject(s).
   */
```

`DOC:` 之后的标题既作为源文件中的标题，也作为提取该文档注释的标识符。因此，标题在文件内必须唯一。

## 包含 kernel-doc 注释


文档注释可以使用专用的 kernel-doc Sphinx 指令扩展，包含进任意 reStructuredText 文档中。

```
  .. kernel-doc:: source
     :option:
```

**source** 是相对于内核源码树的源文件路径。支持以下指令选项：

export: **[source-pattern ...]**
  包含 **source** 中所有已通过 `EXPORT_SYMBOL` 或 `EXPORT_SYMBOL_GPL` 导出的函数的文档，导出位置可以是 **source** 本身，也可以是 **source-pattern** 指定的任意文件。

  **source-pattern** 在 kernel-doc 注释被放在头文件中、而 `EXPORT_SYMBOL` 与 `EXPORT_SYMBOL_GPL` 紧邻函数定义时非常有用。

```
    .. kernel-doc:: lib/bitmap.c
       :export:

    .. kernel-doc:: include/net/mac80211.h
       :export: net/mac80211/*.c
```

internal: **[source-pattern ...]**
  包含 **source** 中所有**未**通过 `EXPORT_SYMBOL` 或 `EXPORT_SYMBOL_GPL` 导出的函数与类型的文档，导出位置可以是 **source** 本身，也可以是 **source-pattern** 指定的任意文件。

```
    .. kernel-doc:: drivers/gpu/drm/i915/intel_audio.c
       :internal:
```

identifiers: **[ function/type ...]**
  包含 **source** 中每个 **function** 与 **type** 的文档。如果未指定 **function**，则会包含 **source** 中所有函数与类型的文档。**type** 可以是 `struct`、`union`、`enum`、`typedef` 或 `var` 标识符。

```
    .. kernel-doc:: lib/bitmap.c
       :identifiers: bitmap_parselist bitmap_parselist_user

    .. kernel-doc:: lib/idr.c
       :identifiers:
```

no-identifiers: **[ function/type ...]**
  排除 **source** 中每个 **function** 与 **type** 的文档。

```
    .. kernel-doc:: lib/bitmap.c
       :no-identifiers: bitmap_parselist
```

functions: **[ function/type ...]**
  这是 `identifiers` 指令的别名，已废弃。

doc: **title**
  包含 **source** 中由 **title** 标识的 `DOC:` 段落的文档。**title** 中允许包含空格；不要为 **title** 加引号。**title** 仅作为该段落的标识符，不会包含在输出中。请确保在外围的 reStructuredText 文档中有合适的标题。

```
    .. kernel-doc:: drivers/gpu/drm/i915/intel_audio.c
       :doc: High Definition Audio over HDMI and Display Port
```

不带选项时，kernel-doc 指令会包含源文件中所有的文档注释。

kernel-doc 扩展位于内核源码树中，路径为 `Documentation/sphinx/kerneldoc.py`。它在内部使用 `tools/docs/kernel-doc` 脚本来从源码中提取文档注释。

### 如何使用 kernel-doc 生成 man 手册页


```
  $ make mandocs
```

```
  $ ./tools/docs/sphinx-build-wrapper mandocs

输出会位于输出目录下的 `/man` 目录中（默认：`Documentation/output`）。

可选地，也可以通过使用 SPHINXDIRS 来生成部分 man 手册页集合：

  $ make SPHINXDIRS=driver-api/media mandocs


   当使用 SPHINXDIRS={subdir} 时，它只会为显式位于 `Documentation/{subdir}/.../*.rst` 文件中的内容生成 man 手册页。
