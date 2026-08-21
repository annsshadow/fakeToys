## 不可biovecs biovec 迭代

Kent Overstreet <kmo@daterainc.com>

3.13 起，bio 一旦被提交，其 biovecs 就绝不应再被修改。取而代之的是，
我们引入了一个新结构 bvec_iter 来表biovec 的一个区间——在 bio 完成
过程中被修改的是迭代器，而非 biovec
更具体地说，旧代码中如果需要部分完成一bio，会更新 bi_sector bi_size，并bi_idx 推进到下一biovec。如果恰好停在某biovec 中间，则会递增 bv_offset 并递减 bv_len，减去该 biovec 中已完成的字节数
在新的方案中，为了部分完成一bio 而必须被修改的所有内容都被隔struct bvec_iter 中：bi_sector、bi_size bi_idx 已被移入其中；并不再修改 bv_offset bv_len，struct bvec_iter 改为使用 bi_bvec_done，它
表示当前 bvec 中已完成的字节数
有一系列新的辅助宏用于隐藏这些繁琐的细节——特别是呈现“部分完成的
biovecs”这一假象，使普通代码不必去处理 bi_bvec_done
 - 驱动代码不应再直接引biovecs；我们现在有bio_iovec()    bio_iter_iovec() 宏，它们返回由原biovecs 构造而成的字面量 struct
   biovecs，但同时考虑bi_bvec_done bi_size
   bio_for_each_segment() 已被更新为接受一bvec_iter 参数，而非一   对应bi_idx 的整数；对大量代码而言，转换只需改变传给
   bio_for_each_segment() 的参数类型即可
 - 推进一bvec_iter 通过 bio_advance_iter() 完成；bio_advance()    bio_advance_iter() 的一个包装，作用bio->bi_iter，并在存在时一   推进 bio 完整性（integrity）的迭代器
   还有一个更底层的推进函数——bvec_iter_advance()——它接受指向 biovec
   的指针而非 bio；bio 完整性代码会用到它
5.12 起，不再支持 bv_len 为零bvec 段
## 这一切能带来什么好处？


拥有真正的迭代器、并biovecs 不可变，具有若干优势
 - 以前，当你不是恰好一次处理一bvec 时，遍历 bios 会非常别扭——例   block/bio.c 中的 bio_copy_data()，它将一bio 的内容复制到另一   bio。由biovecs 不一定大小相同，旧代码错综复杂——它必须同时遍历
   两个 bios，并为每bio 各自维护 bi_idx 与当biovec 内的偏移
   新代码要直白得多——不妨一看。这种模式在很多地方都会出现；以前大   驱动实际上都在自行硬编码 bvec 迭代器，而拥有通用实现后大大简化了
   许多代码
 - 以前，任何可能需要在 bio 完成之后使用 biovec 的代码（也许要将数据
   复制到其他地方，或者出错时改向别处重新提交）都必须保存整个 bvec 数组
   ——同样，这也是在相当多的地方才会做的事
 - biovecs 可以在多bios 之间共享——一bvec iter 可以表示一个现   biovec 的任意区间，既可以从某个 biovec 中间开始，也可以在其中间结束   正是这一点使得任bios 的高效拆分成为可能。请注意，这意味着我们
   _只_ 使用 bi_size 来判断是否已到达 bio 的末尾，而非 bi_vcnt——并   bio_iovec() 宏在构biovecs 时会bi_size 考虑在内
 - 拆分 bios 现在简单得多。旧bio_split() 甚至无法处理包含多个 bvec    bios！现在，我们可以高效地拆分任意大小的 bios——因为新bio 可以共享
   鏃?bio 鐨?biovec銆。
   不过必须小心，确biovec 不会在拆分的 bio 仍在使用它时被释放，以防
   原始 bio 先完成。拆bios 时使bio_chain() 有助于避免此问题
 - 提交部分完成bios 现在完全没问题——这在堆叠块设备驱动中偶尔会出现   而各种代码（例如 md bcache）曾为此有一些丑陋的变通方案
   过去的情况是，向_大多数_设备提交部分完成bio 都能正常工作，但由于
   访问原始 bvec 数组曾是常态，并非所有驱动都会遵bi_idx，那些驱动就   出问题。现在，由于所有驱动都_必须_（并且已经被审计确认）经bvec
   迭代器，提交部分完成bios 完全没问题
## 其他影响

 - 现在几乎所有对 bi_idx 的使用都是错误的，且已被移除；取而代之，以往
   你会使用 bi_idx 的地方现在应使用 bvec_iter，很可能将其传给某个辅助宏
   也就是说，与其使bio_iovec_idx()（或 bio->bi_iovec[bio->bi_idx]），
   你现在应使用 bio_iter_iovec()，它接受一bvec_iter 并返回一个字面量
   struct bio_vec——它是在运行时由原始 biovec 构造而成，但考虑   bi_bvec_done（和 bi_size）
 - bi_vcnt 不能被驱动代码信任或依赖——即任何并非真正拥有bio 的代码   原因有二：首先，遍历 bio 已不再需要它——我们只bi_size。其次，   克隆一bio 并复用（原始 bio biovec 的一部分）时，为了计算新 bio
   bi_vcnt，我们不得不遍历bio 中的所biovecs——这很荒谬，因为   本就不需要
   所以，不要再使bi_vcnt 了
 - 当前接口允许块层按需拆分 bios，因此我们得以消除大量复杂性，特别是在
   堆叠驱动中。创bios 的代码于是可以创建任意方便的 bio 大小，更   要的是，堆叠驱动不必再同时处理自身的 bio 大小限制与底层设备的限制   因此，不再需要为各个块设备驱动定->merge_bvec_fn() 回调
## 辅助宏的使用

- 以下名称带有 `_all` 后缀的辅助宏只能用于BIO_CLONED bio。它们通常
  由文件系统代码使用。驱动不应使用它们，因为 bio 在到达驱动之前可能已  拆分
```

	bio_for_each_segment_all()
	bio_for_each_bvec_all()
	bio_first_bvec_all()
	bio_first_page_all()
	bio_first_folio_all()

```
- 以下辅助宏遍历单页段。所传入‘struct
```

	bio_for_each_segment()
	bio_for_each_segment_all()

```
- 以下辅助宏遍历多bvec。所传入‘struct
```

	bio_for_each_bvec()
	bio_for_each_bvec_all()
	rq_for_each_bvec()

```
