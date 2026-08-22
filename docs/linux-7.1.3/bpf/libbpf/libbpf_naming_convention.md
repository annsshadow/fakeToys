
## API 命名约定

libbpf API 提供对几组在逻辑上相互独立的函数和类型的访问。每一组都有自己在这里
描述的命名约定。建议在添加新函数或类型时遵循这些约定，以保libbpf API 整洁与一致
libbpf API 提供的所有类型和函数都应带有以下前缀之一：`bpf_`、`btf_`、`libbpf_``btf_dump_`、`ring_buffer_`、`perf_buffer_`
### 系统调用封装

系统调用封装是对 sys_bpf 系统调用所支持命令的简单封装。这些封装应当放`bpf.h`
头文件，并与相应命令一一对应
例如，`bpf_map_lookup_elem` 封装sys_bpf `BPF_MAP_LOOKUP_ELEM` 命令`bpf_prog_attach` 封装`BPF_PROG_ATTACH`，等等
### 对象

libbpf API 提供的另一类类型和函数对象"以及用于处理它们的函数。对象是高级
抽象，例BPF 程序BPF map。它们由相应的结构体表示，例`struct bpf_object``struct bpf_program`、`struct bpf_map` 等
结构体采用前向声明，对其字段的访问应当通过相应getter setter 提供，而不是直访问
这些对象与包含已编译 BPF 程序ELF 对象的相应部分相关联
例如，`struct bpf_object` 表示从一ELF 文件或缓冲区创建ELF 对象本身`struct bpf_program` 表示 ELF 对象中的一个程序，`struct bpf_map` 表示一map
处理对象的函数的命名由对象名、双下划线和描述函数用途的部分组成
例如，`bpf_object__open` 由相应对象的名称 `bpf_object`、双下划线和 `open` 组成后者定义了该函打开 ELF 文件并从中创`bpf_object`"的用途
除与 BTF 相关的对象外，所有对象及相应函数都应放入 `libbpf.h`。BTF 类型和函数应
放入 `btf.h`
### 辅助函数

不适合上述任何类别的辅助函数和类型应当带有 `libbpf_` 前缀，例`libbpf_get_error` `libbpf_prog_type_by_name`
### ABI

libbpf 既可以被静态链接，也可以作DSO 使用。为了避免与应用程序链接的其他库
可能发生的冲突，所有非静态的 libbpf 符号都应带有上面 API 文档中提到的某个前缀参见 API 命名约定，为新符号选择合适的名称
### 符号可见
libbpf 遵循这样的模型：默认情况下所有全局符号的可见性为 "hidden"，要使一个符可见，必须用 `LIBBPF_API` 宏显式标注。例如：


        LIBBPF_API int bpf_prog_get_fd_by_id(__u32 id);

这样可以防止意外导出一个本不应成为 ABI 一部分的符号，从而改libbpf 开发者和
用户的体验
### ABI 版本控制

为了使未来的 ABI 扩展成为可能，libbpf ABI 进行了版本控制。版本控制通过传递给
链接器的 `libbpf.map` 版本脚本实现
版本名为 `LIBBPF_` 前缀 + 三段式数字版本，`0.0.1` 开始
每当 ABI 发生变更（例如新增了一个符号，或者现有符号的语义发生了改变），就应当
提升 ABI 版本。每个内核开发周期最多提升一ABI 版本
例如，如`libbpf.map` 的当前状态是

        LIBBPF_0.0.1 {
        	global:
                        bpf_func_a;
                        bpf_func_b;
        	local:
        		\*;
        };

，并且要引入一个新的符`bpf_func_c`，那`libbpf.map` 应当这样修改

        LIBBPF_0.0.1 {
        	global:
                        bpf_func_a;
                        bpf_func_b;
        	local:
        		\*;
        };
        LIBBPF_0.0.2 {
                global:
                        bpf_func_c;
        } LIBBPF_0.0.1;

，其中新版本 `LIBBPF_0.0.2` 依赖于先前的 `LIBBPF_0.0.1`
版本脚本的格式以及处ABI 变更（包括不兼容的变更）的方式，[^1^] 中有详细描述
### 独立构建

https://github.com/libbpf/libbpf 处有一个用于独立构建的 libbpf 主线版本
（半自动）镜像
但是，对 libbpf 代码库的所有更改都必须通过主线内核树向上游提交
## API 文档约定

libbpf API 通过头文件中定义上方的注释进行文档化。这些注释可以被 doxygen sphinx 渲染为组织良好的 html 输出。本节描述这些注释应当采用的格式约定
以下是来btf.h 的一个例子：


        /**
         - @brief **btf__new()** creates a new instance of a BTF object from the raw
         - bytes of an ELF's BTF section
         - @param data raw bytes
         - @param size number of bytes passed in `data`
         - @return new BTF object instance which has to be eventually freed with
         - **btf__free()**
         *
         - On error, error-code-encoded-as-pointer is returned, not a NULL. To extract
         - error code from such a pointer `libbpf_get_error()` should be used. If
         - `libbpf_set_strict_mode(LIBBPF_STRICT_CLEAN_PTRS)` is enabled, NULL is
         - returned on error instead. In both cases thread-local `errno` variable is
         - always set to error code as well.
         */

注释必须'/\**\**' 形式的块注释开始
文档总是@brief 指令开始。这一行是对该 API 的简短描述。它API 的名称开头，粗体表示，如*api_name**。如果这是一个函数，请包含一对左右圆括号。随后跟API 的简短描述。更长的描述可以加在最后一个指令下方、注释的底部
参数@param 指令表示，每个参数都应有一个。如果这是一个具有非 void 返回值的函数请使@return 指令来记录它
### 许可
libbpf 采用 LGPL 2.1 BSD 2-Clause 双重许可
### 链接

[^1^] https://www.akkadia.org/drepper/dsohowto.pdf
    (Chapter 3. Maintaining APIs and ABIs).
