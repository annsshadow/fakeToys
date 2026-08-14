
## Sparse


Sparse 是 C 程序的语义检查器；它可以用来发现内核代码中的若干潜在问题。关于
sparse 的概述请参见 https://lwn.net/Articles/689907/；本文档包含一些内核相关的
sparse 信息。更多关于 sparse 的信息（主要关于其内部实现）可以在其官方页面
https://sparse.docs.kernel.org 找到。


### 使用 sparse 进行类型检查


```

        typedef int __bitwise pm_request_t;

        enum pm_request {
                PM_SUSPEND = (__force pm_request_t) 1,
                PM_RESUME = (__force pm_request_t) 2
        };

```
这使得 PM_SUSPEND 与 PM_RESUME 成为“bitwise”整数（这里的 "__force" 是因为 sparse
会抱怨向/从 bitwise 类型进行强制转换，但在此例中我们确实_想要_强制转换）。并且因为
枚举值都是同一类型，现在 "enum pm_request" 也会是那个类型。

而对于 gcc，所有的 "__bitwise"/"__force 那些东西" 都会消失，对 gcc 来说它们最终
看起来就只是普通的整数。

坦白说，你并不需要那里的枚举。上面的内容实际上都可以归结为一种特殊的
"int __bitwise" 类型。

```

        typedef int __bitwise pm_request_t;

        #define PM_SUSPEND ((__force pm_request_t) 1)
        #define PM_RESUME ((__force pm_request_t) 2)

```
现在你就拥有了进行严格类型检查所需的全部基础设施。

一个小提示：常量整数 "0" 是特殊的。你可以在不引发 sparse 任何抱怨的情况下，将
常量 0 用作 bitwise 整数类型。这是因为 "bitwise"（顾名思义）被设计用来确保 bitwise
类型不会被混淆（小端 vs 大端 vs cpu 端 vs 其它），而在那里常量 "0" 确实_是_特殊的。

### 获取 sparse


你可以从以下地址获取最新发布版本的 tarball：
https://www.kernel.org/pub/software/devel/sparse/dist/

另外，你可以获取最新开发版本的快照

```

        git://git.kernel.org/pub/scm/devel/sparse/sparse.git

```
```

        make
        make install

```
作为普通用户，它会将 sparse 安装到你的 ~/bin 目录下。

### 使用 sparse


执行 "make C=1" 的内核编译，可对重新编译的所有 C 文件运行 sparse；或者使用
"make C=2" 对文件运行 sparse，无论它们是否需要重新编译。如果你已经构建过整个
代码树，后者是检查整棵树的一种快速方式。

可选的 make 变量 CF 可用于向 sparse 传递参数。构建系统会自动向 sparse 传递
-Wbitwise。

注意，sparse 定义了 __CHECKER__ 预处理宏。
