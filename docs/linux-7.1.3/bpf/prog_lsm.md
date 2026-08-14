
## LSM BPF 程序


这些 BPF 程序允许特权用户在运行时对 LSM 钩子（hook）进行插桩（instrumentation），以使用 eBPF 实现系统级的 MAC（强制访问控制，Mandatory Access Control）和审计（Audit）策略。

### 结构


示例展示了一个可以附加到 `file_mprotect` LSM 钩子的 eBPF 程序：


其他可被插桩的 LSM 钩子可在 `security/security.c` 中找到。

使用 Documentation/bpf/btf.rst 的 eBPF 程序无需包含内核头文件即可访问所附加 eBPF 程序上下文中的信息。它们只需在 eBPF 程序中声明这些结构，并仅指定需要访问的字段即可。


	struct mm_struct {
		unsigned long start_brk, brk, start_stack;
	} __attribute__((preserve_access_index));

	struct vm_area_struct {
		unsigned long start_brk, brk, start_stack;
		unsigned long vm_start, vm_end;
		struct mm_struct *vm_mm;
	} __attribute__((preserve_access_index));



如果（在构建时可访问 BTF 信息）通过以下命令生成 `vmlinux.h`，则可进一步简化：


	# bpftool btf dump file <path-to-btf-vmlinux> format c > vmlinux.h

	  构建环境与 BPF 程序部署的环境相匹配。

然后只需在 BPF 程序中包含 `vmlinux.h`，而无需定义这些类型。

eBPF 程序可以使用 `tools/lib/bpf/bpf_tracing.h`_ 中定义的 `BPF_PROG` 宏声明。在本例中：

 - `"lsm/file_mprotect"` 表示该程序必须附加到的 LSM 钩子
 - `mprotect_audit` 是 eBPF 程序的名称


	SEC("lsm/file_mprotect")
	int BPF_PROG(mprotect_audit, struct vm_area_struct *vma,
		     unsigned long reqprot, unsigned long prot, int ret)
	{
		/* ret 是前一个 BPF 程序的返回值
   - 如果是第一个钩子则为 0。
		 */
		if (ret != 0)
			return ret;

		int is_heap;

		is_heap = (vma->vm_start >= vma->vm_mm->start_brk &&
			   vma->vm_end <= vma->vm_mm->brk);

		/* 返回 -EPERM，或向 perf 事件缓冲区写入信息
   - 用于审计
		 */
		if (is_heap)
			return -EPERM;
	}

`__attribute__((preserve_access_index))` 是 clang 的一个特性，允许 BPF 验证器（verifier）在运行时使用 Documentation/bpf/btf.rst 信息更新访问的偏移量。由于 BPF 验证器了解这些类型，它还会验证 eBPF 程序中对各种类型的所有访问。

### 加载


eBPF 程序可以通过 `bpf(2)` 系统调用的 `BPF_PROG_LOAD` 操作加载：


	struct bpf_object *obj;

	obj = bpf_object__open("./my_prog.o");
	bpf_object__load(obj);

使用 `bpftool` 生成的 skeleton 头文件可以简化这一过程：


	# bpftool gen skeleton my_prog.o > my_prog.skel.h

程序可以通过包含 `my_prog.skel.h` 并使用生成的辅助函数 `my_prog__open_and_load` 来加载。

### 附加到 LSM 钩子


LSM 允许使用 `bpf(2)` 系统调用的 `BPF_RAW_TRACEPOINT_OPEN` 操作将 eBPF 程序作为 LSM 钩子附加，或者更简单地使用 libbpf 辅助函数 `bpf_program__attach_lsm`。

可以通过**销毁**（destroying）`bpf_program__attach_lsm` 返回的 `link` 链接（使用 `bpf_link__destroy`）将程序从 LSM 钩子分离。

也可以使用 `my_prog.skel.h` 中生成的辅助函数，即 `my_prog__attach` 用于附加、`my_prog__destroy` 用于清理。

### 示例


示例 eBPF 程序可在 `tools/testing/selftests/bpf/progs/lsm.c`_ 中找到，相应的用户态代码在 `tools/testing/selftests/bpf/prog_tests/test_lsm.c`_

   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/tools/lib/bpf/bpf_tracing.h
   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/tools/testing/selftests/bpf/progs/lsm.c
   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/tools/testing/selftests/bpf/prog_tests/test_lsm.c
