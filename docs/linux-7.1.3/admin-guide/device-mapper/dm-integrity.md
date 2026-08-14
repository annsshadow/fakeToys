## dm-integrity


The dm-integrity target emulates 一个 块 设备 该 具有 额外
per-sector tags 该 可 为 使用 用于 storing integrity information.

一个 通用 problem 与 storing integrity tags 与 every 扇区 是 该
writing the 扇区 和 the integrity tag 必须 为 原子 - i.e. 如果发生
crash, 任一个 两者 扇区 和 integrity tag 或 none 的 them 是 written.

到 guarantee 写入 atomicity, the dm-integrity target uses journal, 它
writes 扇区 数据 和 integrity tags 进入 一个 journal, commits the journal
和 然后 copies the 数据 和 integrity tags 到 它们的 respective location.

The dm-integrity target 可 为 使用 与 the dm-crypt target - 在 此
situation the dm-crypt target creates the integrity 数据 和 passes them
到 the dm-integrity target 通过 bio_integrity_payload attached 到 the bio.
在 此 模式, the dm-crypt 和 dm-integrity targets 提供 authenticated
disk encryption - 若 the attacker modifies the encrypted 设备, 一个 I/O
错误 是 returned 而非 random 数据.

The dm-integrity target 可 也 为 使用 作为 一个 standalone target, 在 此
模式 它 calculates 和 verifies the integrity tag internally. 在 此
模式, the dm-integrity target 可 为 使用 到 detect silent 数据
corruption 在 the disk 或 在 the I/O path.

那里's 一个 alternate 模式 的 操作 何处 dm-integrity uses 一个 bitmap
而非 一个 journal. 若 一个 位 在 the bitmap 是 1, the corresponding
region's 数据 和 integrity tags 是 不 synchronized - 若 the machine
crashes, the unsynchronized regions 将 为 recalculated. The bitmap 模式
是 faster 比 the journal 模式, 因为 我们 don't 具有 到 写入 the 数据
twice, 但 它是 也 less reliable, 因为 若 数据 corruption happens
当 the machine crashes, 它 可 不 为 detected.

当 loading the target 用于 the 第一 time, the 内核 驱动 将 格式
the 设备. 但 它 将 仅 格式 the 设备 若 the superblock 包含
zeroes. 若 the superblock 是 两者都不 valid nor zeroed, the dm-integrity
target 可't 为 loaded.

Accesses 到 the on-disk metadata area containing checksums (aka tags) 是
buffered 使用 dm-bufio. 当 一个 access 到 任何 given metadata area
occurs, 每个 unique metadata area gets 其 own 缓冲区(s). The 缓冲区 大小
是 capped 在 the 大小 的 the metadata area, 但 可 为 smaller, thereby
requiring 多个 缓冲区 到 represent the full metadata area. 一个 smaller
缓冲区 大小 将 produce 一个 smaller resulting 读取/写入 操作 到 the
metadata area 用于 small reads/writes. The metadata 是 仍然 读取 even 在
一个 full 写入 到 the 数据 covered 由 一个 单个 缓冲区.

到 使用 the target 用于 the 第一 time:

1. overwrite the superblock 与 zeroes
2. 加载 the dm-integrity target 与 one-sector 大小, the 内核 驱动
   将 格式 the 设备
3. 卸载 the dm-integrity target
4. 读取 the "provided_数据_sectors" 值 来自 the superblock
5. 加载 the dm-integrity target 与 the target 大小
   "provided_数据_sectors"
6. 若 您 希望 到 使用 dm-integrity 与 dm-crypt, 加载 the dm-crypt target
   与 the 大小 "provided_数据_sectors"


Target arguments:

1. the underlying 块 设备

2. the 数字 的 reserved 扇区 在 the beginning 的 the 设备 - the
   dm-integrity won't 读取 的 写入 这些 sectors

3. the 大小 的 the integrity tag (若 "-" 是 使用, the 大小 是 taken 来自
   the internal-hash algorithm)

4. 模式:

	D - direct writes (无 journal)
		在 此 模式, journaling 是
		不 使用 和 数据 sectors 和 integrity tags 是 written
		separately. 如果发生 crash, 它是 可能 该 the 数据
		和 integrity tag doesn't match.
	J - journaled writes
		数据 和 integrity tags 是 written 到 the
		journal 和 atomicity 是 guaranteed. 如果发生 crash,
		任一个 两者 数据 和 tag 或 none 的 them 是 written. The
		journaled 模式 degrades 写入 throughput twice 因为 the
		数据 具有 到 为 written twice.
	B - bitmap 模式 - 数据 和 metadata 是 written 无 任何
		synchronization, the 驱动 maintains 一个 bitmap 的 dirty
		regions 何处 数据 和 metadata don't match. 此 模式 可
		仅 为 使用 与 内部 hash.
	R - recovery 模式 - 在 此 模式, journal 是 不 replayed,
		checksums 是 不 checked 和 writes 到 the 设备 是 不
		allowed. 此 模式 是 useful 用于 数据 recovery 若 the
		设备 cannot 为 activated 在 任何 的 the 其他 标准
		modes.
	I - inline 模式 - 在 此 模式, dm-integrity 将 store integrity
		数据 directly 在 the underlying 设备 sectors.
		The underlying 设备 必须 具有 一个 integrity profile 该
		allows storing 用户 integrity 数据 和 提供 enough
		space 用于 the selected integrity tag.

5. the 数字 的 额外 arguments

额外 arguments:

journal_sectors:数字
	The 大小 的 journal, 此 参数 是 使用 仅 若 formatting the
	设备. 若 the 设备 是 已经 formatted, the 值 来自 the
	superblock 是 使用.

interleave_sectors:数字 (默认 32768)
	The 数字 的 interleaved sectors. 此 值 是 rounded down 到
	一个 电源 的 two. 若 the 设备 是 已经 formatted, the 值 来自
	the superblock 是 使用.

meta_设备:设备
	Don't interleave the 数据 和 metadata 在 the 设备. 使用 一个
	separate 设备 用于 metadata.

缓冲区_sectors:数字 (默认 128)
	The 数字 的 sectors 在 one metadata 缓冲区. The 值 是 rounded
	down 到 一个 电源 的 two.

journal_watermark:数字 (默认 50)
	The journal watermark 在 percents. 当 the 大小 的 the journal
	exceeds 此 watermark, the 线程 该 flushes the journal 将
	为 started.

commit_time:数字 (默认 10000)
	Commit time 在 milliseconds. 当 此 time passes, the journal 是
	written. The journal 是 也 written immediately 若 the FLUSH
	请求 是 received.

内部_hash:algorithm(:key)	(the key 是 可选)
	使用 内部 hash 或 crc.
	当 此 参数 是 使用, the dm-integrity target won't accept
	integrity tags 来自 the upper target, 但 它 将 automatically
	generate 和 verify the integrity tags.

	您可以 使用 一个 crc algorithm (例如 crc32), 然后 integrity target
	将 protect the 数据 against accidental corruption.
	您可以 也 使用 一个 hmac algorithm (例如
	"hmac(sha256):0123456789abcdef"), 在 此 模式 它 将 提供
	cryptographic authentication 的 the 数据 无 encryption.

	当 此 参数 是 不 使用, the integrity tags 是 accepted
	来自 一个 upper layer target, 例如 dm-crypt. The upper layer
	target 应当 check the validity 的 the integrity tags.

recalculate
	Recalculate the integrity tags automatically. 它是 仅 valid
	当 使用 内部 hash.

journal_crypt:algorithm(:key)	(the key 是 可选)
	Encrypt the journal 使用 given algorithm 到 确保 该 the
	attacker 可't 读取 the journal. 您可以 使用 一个 块 cipher 此处
	(例如 "cbc(aes)") 或 一个 流 cipher (例如 "chacha20"
	或 "ctr(aes)").

	The journal 包含 history 的 最后 writes 到 the 块 设备,
	一个 attacker reading the journal 可以 参见 the 最后 扇区 numbers
	该 曾是 written. 来自 the 扇区 numbers, the attacker 可 infer
	the 大小 的 文件 该 曾是 written. 到 protect against 此
	situation, 您可以 encrypt the journal.

journal_mac:algorithm(:key)	(the key 是 可选)
	Protect 扇区 numbers 在 the journal 来自 accidental 或 malicious
	modification. 到 protect against accidental modification, 使用 一个
	crc algorithm, 到 protect against malicious modification, 使用 一个
	hmac algorithm 与 一个 key.

	此 选项 是 不 needed 当 使用 internal-hash 因为 在 此
	模式, the integrity 的 journal 条目 是 checked 当 replaying
	the journal. 从而, modified 扇区 数字 将会 为 detected 在
	此 stage.

块_大小:数字 (默认 512)
	The 大小 的 一个 数据 块 在 bytes. The larger the 块 大小 the
	less overhead 存在 用于 per-block integrity metadata.
	受支持 值 是 512, 1024, 2048 和 4096 bytes.

sectors_每_位:数字
	在 the bitmap 模式, 此 参数 specifies the 数字 的
	512-byte sectors 该 corresponds 到 one bitmap 位.

bitmap_flush_interval:数字
	The bitmap flush interval 在 milliseconds. The metadata 缓冲区
	是 synchronized 当 此 interval expires.

允许_discards
	允许 块 discard requests (一个.k.一个. TRIM) 用于 the integrity 设备.
	Discards 是 仅 allowed 到 设备 使用 内部 hash.

fix_padding
	使用 一个 smaller padding 的 the tag area 即 更多
	space-efficient. 若 此 选项 是 不 present, large padding 是
	使用 - 即 用于 compatibility 与 older kernels.

fix_hmac
	Improve 安全 的 内部_hash 和 journal_mac:

 - the section 数字 是 mixed 到 the mac, 因此 该 一个 attacker 可't
	  copy sectors 来自 one journal section 到 another journal section
 - the superblock 是 protected 由 journal_mac
 - 一个 16-byte salt stored 在 the superblock 是 mixed 到 the mac, 因此
	  该 the attacker 可't detect 该 two disks 具有 the 相同 hmac
	  key 和 也 到 disallow the attacker 到 move sectors 来自 one
	  disk 到 another

legacy_recalculate
	允许 recalculating 的 volumes 与 HMAC keys. 这是 已禁用 由
	默认 用于 安全 reasons - 一个 attacker 可以 modify the volume,
	set recalc_扇区 到 zero, 和 the 内核 将会 不 detect the
	modification.

The journal 模式 (D/J), 缓冲区_sectors, journal_watermark, commit_time 和
允许_discards 可 为 changed 当 reloading the target (加载 一个 inactive
表 和 swap the 表 与 suspend 和 resume). The 其他 arguments
应当 不 为 changed 当 reloading the target 因为 the layout 的 disk
数据 depend 在 them 和 the reloaded target 将会 为 non-functional.

例如, 在 一个 设备 使用 the 默认 interleave_sectors 的 32768, 一个
块_大小 的 512, 和 一个 内部_hash 的 crc32c 与 一个 tag 大小 的 4
bytes, 它 将 take 128 KiB 的 tags 到 track 一个 full 数据 area, requiring
256 sectors 的 metadata 每 数据 area. 与 the 默认 缓冲区_sectors 的
128, 该 means 那里 将 为 2 缓冲区 每 metadata area, 或 2 缓冲区
每 16 MiB 的 数据.

状态 line:

1. the 数字 的 integrity mismatches
2. provided 数据 sectors - 即 the 数字 的 sectors 该 the 用户
   可以 使用
3. the 电流 recalculating position (或 '-' 若 我们 didn't recalculate)


The layout 的 the formatted 块 设备:

- reserved sectors
    (它们是 不 使用 由 此 target, 它们 可 为 使用 用于
    storing LUKS metadata 或 用于 其他 purpose), the 大小 的 the reserved
    area 是 specified 在 the target arguments

- superblock (4kiB)
 - magic 字符串 - identifies 该 the 设备 曾是 formatted
 - 版本
 - log2(interleave sectors)
 - integrity tag 大小
 - the 数字 的 journal sections
 - provided 数据 sectors - the 数字 的 sectors 该 此 target
	  提供 (i.e. the 大小 的 the 设备 minus the 大小 的 全部
	  metadata 和 padding). The 用户 的 此 target 应当 不 send
	  bios 该 access 数据 beyond the "provided 数据 sectors" limit.
 - 标志
	    SB_标志_具有_JOURNAL_MAC
  - 一个 标志 是 set 若 journal_mac 是 使用
	    SB_标志_RECALCULATING
  - recalculating 是 在 progress
	    SB_标志_DIRTY_BITMAP
  - journal area 包含 the bitmap 的 dirty
		  块
 - log2(sectors 每 块)
 - 一个 position 何处 recalculating finished
- journal
	The journal 是 divided 进入 sections, 每个 section 包含:

 - metadata area (4kiB), 它 包含 journal 条目

   - every journal 条目 包含:

  - logical 扇区 (specifies 何处 the 数据 和 tag 应当
		  为 written)
  - 最后 8 bytes 的 数据
  - integrity tag (the 大小 是 specified 在 the superblock)

   - every metadata 扇区 ends 与

  - mac (8-bytes), 全部 the macs 在 8 metadata sectors form 一个
		  64-byte 值. 它是 使用 到 store hmac 的 扇区
		  numbers 在 the journal section, 到 protect against 一个
		  possibility 该 the attacker tampers 与 扇区
		  numbers 在 the journal.
  - commit id

 - 数据 area (the 大小 是 variable; 它 depends 在 如何 许多 journal
	  条目 fit 进入 the metadata area)

     - every 扇区 在 the 数据 area 包含:

  - 数据 (504 bytes 的 数据, the 最后 8 bytes 是 stored 在
		  the journal 条目)
  - commit id

	到 test 若 the whole journal section 曾是 written correctly, every
	512-byte 扇区 的 the journal ends 与 8-byte commit id. 若 the
	commit id matches 在 全部 sectors 在 一个 journal section, 然后 它是
	assumed 该 the section 曾是 written correctly. 若 the commit id
	doesn't match, the section 曾是 written partially 和 它 应当 不
	为 replayed.

- one 或 更多 runs 的 interleaved tags 和 数据.
    每个 运行 包含:

 - tag area - 它 包含 integrity tags. 存在 one tag 用于 每个
	  扇区 在 the 数据 area. The 大小 的 此 area 是 始终 4KiB 或
	  greater.
 - 数据 area - 它 包含 数据 sectors. The 数字 的 数据 sectors
	  在 one 运行 必须 为 一个 电源 的 two. log2 的 此 值 是 stored
	  在 the superblock.
