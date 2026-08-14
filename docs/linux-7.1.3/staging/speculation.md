## 推测执行（Speculation）


本文档解释了推测可能带来的影响，以及如何通过使用通用 API 以可移植的方式缓解不良后果。

------------------------------------------------------------------------------

为了提高性能并降低平均延迟，许多当代 CPU 采用了推测执行技术，例如分支预测，即执行一些可能在后续阶段被丢弃的工作。

通常，从架构状态（例如寄存器的内容）无法观察到推测执行。然而在某些情况下，可以观察到其对微架构状态（例如缓存中数据存在与否）的影响。此类状态可能形成侧信道（side-channel），可被观测以提取秘密信息。

例如，在存在分支预测的情况下，被推测执行的代码有可能忽略边界检查。考虑如下
```

	int load_array(int *array, unsigned int index)
	{
		if (index >= MAX_ARRAY_ELEMS)
			return 0;
		else
			return array[index];
	}

```
```

	CMP	<index>, #MAX_ARRAY_ELEMS
	B.LT	less
	MOV	<returnval>, #0
	RET
  less:
	LDR	<returnval>, [<array>, <index>]
	RET

```
CPU 有可能错误预测条件分支，从而即使 index >= MAX_ARRAY_ELEMS，也会推测性地加载 array[index]。该值随后会被丢弃，但被推测的加载可能影响微架构状态，而该状态随后可被测量。

涉及多个相互依赖的内存访问的更复杂序列可能导致敏感信息泄露。考虑以下
```

	int load_dependent_arrays(int *arr1, int *arr2, int index)
	{
		int val1, val2,

		val1 = load_array(arr1, index);
		val2 = load_array(arr2, val1);

		return val2;
	}

```
在推测下，第一次对 load_array() 的调用可能返回一个越界地址的值，而第二次调用将影响依赖于该值的微架构状态。这可能提供一种任意读的原语。

## 缓解推测侧信道


内核提供了一个通用 API，以确保即使处于推测之下，边界检查也会被遵守。受推测侧信道影响的架构应当实现这些原语。

<linux/nospec.h> 中的 array_index_nospec() 辅助函数可用于防止信息通过侧信道泄露。

对 array_index_nospec(index, size) 的调用会返回一个经过净化的索引值，即使在 CPU 推测条件下，该值也被限制在 [0, size) 范围内。

```

	int load_array(int *array, unsigned int index)
	{
		if (index >= MAX_ARRAY_ELEMS)
			return 0;
		else {
			index = array_index_nospec(index, MAX_ARRAY_ELEMS);
			return array[index];
		}
	}

```
