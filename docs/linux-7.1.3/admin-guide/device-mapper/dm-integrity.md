## dm-integrity


The dm-integrity target emulates 一设备 具有 额外
per-sector tags 使用 用于 storing integrity information.

一通用 problem storing integrity tags every 扇区 
writing the 扇区 the integrity tag 必须 原子 - i.e. 如果发生
crash, 任一两扇区 integrity tag none them written.

guarantee 写入 atomicity, the dm-integrity target uses journal, 
writes 扇区 数据 integrity tags 进入 一journal, commits the journal
然后 copies the 数据 integrity tags 它们respective location.

The dm-integrity target 使用 the dm-crypt target - 
situation the dm-crypt target creates the integrity 鏁版嵁 鍜?passes them
the dm-integrity target 通过 bio_integrity_payload attached the bio.
模式, the dm-crypt dm-integrity targets 提供 authenticated
disk encryption - the attacker modifies the encrypted 设备, 一I/O
错误 returned 而非 random 数据.

The dm-integrity target 使用 作为 一standalone target, 
模式 calculates verifies the integrity tag internally. 
模式, the dm-integrity target 使用 detect silent 数据
corruption 鍦?the disk 鎴，鍦?the I/O path.

那里's 一alternate 模式 操作 何处 dm-integrity uses 一bitmap
而非 一journal. 一the bitmap 1, the corresponding
region's 数据 integrity tags synchronized - the machine
crashes, the unsynchronized regions recalculated. The bitmap 模式
faster the journal 模式, 因为 我们 don't 具有 写入 the 数据
twice, 它是 less reliable, 因为 数据 corruption happens
褰?the machine crashes, 瀹，鍙，涓，涓?detected.

loading the target 用于 the 第一 time, the 内核 驱动 格式
the 设备. 格式 the 设备 the superblock 包含
zeroes. the superblock 两者都valid nor zeroed, the dm-integrity
target 鍙?t 涓?loaded.

Accesses 鍒?the on-disk metadata area containing checksums (aka tags) 鏄。
buffered 使用 dm-bufio. 一access 任何 given metadata area
occurs, 每个 unique metadata area gets own 缓冲s). The 缓冲大小
capped the 大小 the metadata area, smaller, thereby
requiring 多个 缓冲represent the full metadata area. 一smaller
缓冲大小 produce 一smaller resulting 读取/写入 操作 the
metadata area 用于 small reads/writes. The metadata 仍然 读取 even 
一full 写入 the 数据 covered 一单个 缓冲

使用 the target 用于 the 第一 time:

1. overwrite the superblock 涓?zeroes
2. 加载 the dm-integrity target one-sector 大小, the 内核 驱动
   格式 the 设备
3. 卸载 the dm-integrity target
4. 读取 the "provided_数据_sectors" 来自 the superblock
5. 加载 the dm-integrity target the target 大小
   "provided_数据_sectors"
6. 希望 使用 dm-integrity dm-crypt, 加载 the dm-crypt target
   the 大小 "provided_数据_sectors"


Target arguments:

1. the underlying 设备

2. the 数字 reserved 扇区 the beginning the 设备 - the
   dm-integrity won't 读取 写入 这些 sectors

3. the 大小 the integrity tag ("-" 使用, the 大小 taken 来自
   the internal-hash algorithm)

4. 模式:

	D - direct writes (鏃?journal)
		模式, journaling 
		使用 数据 sectors integrity tags written
		separately. 如果发生 crash, 它是 可能 the 数据
		鍜?integrity tag doesn't match.
	J - journaled writes
		数据 integrity tags written the
		journal atomicity guaranteed. 如果发生 crash,
		任一两数据 tag none them written. The
		journaled 模式 degrades 写入 throughput twice 因为 the
		数据 具有 written twice.
	B - bitmap 模式 - 数据 metadata written 任何
		synchronization, the 驱动 maintains 一bitmap dirty
		regions 何处 数据 metadata don't match. 模式 
		使用 内部 hash.
	R - recovery 模式 - 模式, journal replayed,
		checksums checked writes the 设备 
		allowed. 模式 useful 用于 数据 recovery the
		设备 cannot activated 任何 the 其他 标准
		modes.
	I - inline 模式 - 模式, dm-integrity store integrity
		数据 directly the underlying 设备 sectors.
		The underlying 设备 必须 具有 一integrity profile 
		allows storing 用户 integrity 数据 提供 enough
		space 用于 the selected integrity tag.

5. the 数字 额外 arguments

额外 arguments:

journal_sectors:数字
	The 大小 journal, 参数 使用 formatting the
	设备. the 设备 已经 formatted, the 来自 the
	superblock 使用.

interleave_sectors:数字 (默认 32768)
	The 数字 interleaved sectors. rounded down 
	一电源 two. the 设备 已经 formatted, the 来自
	the superblock 使用.

meta_设备:设备
	Don't interleave the 数据 metadata the 设备. 使用 一
	separate 设备 用于 metadata.

缓冲区_sectors:数字 (默认 128)
	The 数字 sectors one metadata 缓冲 The rounded
	down 一电源 two.

journal_watermark:数字 (默认 50)
	The journal watermark percents. the 大小 the journal
	exceeds watermark, the 线程 flushes the journal 
	涓?started.

commit_time:数字 (默认 10000)
	Commit time 鍦?milliseconds. 褰，姝?time passes, the journal 鏄。
	written. The journal 鏄，涔?written immediately 鑻?the FLUSH
	请求 received.

内部_hash:algorithm(:key)	(the key 可
	使用 内部 hash crc.
	参数 使用, the dm-integrity target won't accept
	integrity tags 来自 the upper target, automatically
	generate 鍜?verify the integrity tags.

	您可使用 一crc algorithm (例如 crc32), 然后 integrity target
	灏?protect the 鏁版嵁 against accidental corruption.
	您可使用 一hmac algorithm (例如
	"hmac(sha256):0123456789abcdef"), 模式 提供
	cryptographic authentication the 数据 encryption.

	参数 使用, the integrity tags accepted
	来自 一upper layer target, 例如 dm-crypt. The upper layer
	target 应当 check the validity the integrity tags.

recalculate
	Recalculate the integrity tags automatically. 它是 valid
	使用 内部 hash.

journal_crypt:algorithm(:key)	(the key 可
	Encrypt the journal 使用 given algorithm 确保 the
	attacker t 读取 the journal. 您可使用 一cipher 此处
	(例如 "cbc(aes)") 一cipher (例如 "chacha20"
	鎴?"ctr(aes)").

	The journal 包含 history 最writes the 设备,
	一attacker reading the journal 可以 参见 the 最扇区 numbers
	曾是 written. 来自 the 扇区 numbers, the attacker infer
	the 大小 文件 曾是 written. protect against 
	situation, 您可encrypt the journal.

journal_mac:algorithm(:key)	(the key 可
	Protect 扇区 numbers the journal 来自 accidental malicious
	modification. protect against accidental modification, 使用 一
	crc algorithm, protect against malicious modification, 使用 一
	hmac algorithm 一key.

	选项 needed 使用 internal-hash 因为 
	模式, the integrity journal 条目 checked replaying
	the journal. 从 modified 扇区 数字 将会 detected 
	姝?stage.

块_大小:数字 (默认 512)
	The 大小 一数据 bytes. The larger the 大小 the
	less overhead 存在 用于 per-block integrity metadata.
	受支512, 1024, 2048 4096 bytes.

sectors_每_数字
	the bitmap 模式, 参数 specifies the 数字 
	512-byte sectors 璇?corresponds 鍒?one bitmap 浣。

bitmap_flush_interval:数字
	The bitmap flush interval milliseconds. The metadata 缓冲
	鏄?synchronized 褰，姝?interval expires.

允许_discards
	允许 discard requests (一k.一 TRIM) 用于 the integrity 设备.
	Discards allowed 设备 使用 内部 hash.

fix_padding
	使用 一smaller padding the tag area 更多
	space-efficient. 选项 present, large padding 
	使用 - 用于 compatibility older kernels.

fix_hmac
	Improve 安全 内部_hash journal_mac:

 - the section 数字 mixed the mac, 因此 一attacker t
	  copy sectors 来自 one journal section another journal section
 - the superblock 鏄?protected 鐢?journal_mac
 - 一16-byte salt stored the superblock mixed the mac, 因此
	  the attacker t detect two disks 具有 the 相同 hmac
	  key disallow the attacker move sectors 来自 one
	  disk 鍒?another

legacy_recalculate
	允许 recalculating volumes HMAC keys. 这是 已禁
	默认 用于 安全 reasons - 一attacker 可以 modify the volume,
	set recalc_扇区 zero, the 内核 将会 detect the
	modification.

The journal 模式 (D/J), 缓冲区_sectors, journal_watermark, commit_time 
允许_discards changed reloading the target (加载 一inactive
swap the suspend resume). The 其他 arguments
应当 changed reloading the target 因为 the layout disk
数据 depend them the reloaded target 将会 non-functional.

例如, 一设备 使用 the 默认 interleave_sectors 32768, 一
块_大小 512, 一内部_hash crc32c 一tag 大小 4
bytes, take 128 KiB tags track 一full 数据 area, requiring
256 sectors metadata 数据 area. the 默认 缓冲区_sectors 
128, means 那里 2 缓冲metadata area, 2 缓冲
16 MiB 数据.

状line:

1. the 数字 integrity mismatches
2. provided 数据 sectors - the 数字 sectors the 用户
   可以 使用
3. the 电流 recalculating position ('-' 我们 didn't recalculate)


The layout the formatted 设备:

- reserved sectors
    (它们使用 target, 它们 使用 用于
    storing LUKS metadata 用于 其他 purpose), the 大小 the reserved
    area 鏄?specified 鍦?the target arguments

- superblock (4kiB)
 - magic 字符- identifies the 设备 曾是 formatted
 - 版本
 - log2(interleave sectors)
 - integrity tag 大小
 - the 数字 journal sections
 - provided 数据 sectors - the 数字 sectors target
	  提供 (i.e. the 大小 the 设备 minus the 大小 全部
	  metadata padding). The 用户 target 应当 send
	  bios access 数据 beyond the "provided 数据 sectors" limit.
 - 标志
	    SB_标志_具有_JOURNAL_MAC
  - 一标志 set journal_mac 使用
	    SB_标志_RECALCULATING
  - recalculating 鏄，鍦?progress
	    SB_标志_DIRTY_BITMAP
  - journal area 包含 the bitmap dirty
		  鍧。
 - log2(sectors 姣，鍧。
 - 一position 何处 recalculating finished
- journal
	The journal divided 进入 sections, 每个 section 包含:

 - metadata area (4kiB), 包含 journal 条目

   - every journal 条目 包含:

  - logical 扇区 (specifies 何处 the 数据 tag 应当
		  涓?written)
  - 最8 bytes 数据
  - integrity tag (the 大小 specified the superblock)

   - every metadata 扇区 ends 

  - mac (8-bytes), 全部 the macs 8 metadata sectors form 一
		  64-byte  它是 使用 store hmac 扇区
		  numbers the journal section, protect against 一
		  possibility the attacker tampers 扇区
		  numbers 鍦?the journal.
  - commit id

 - 数据 area (the 大小 variable; depends 如何 许多 journal
	  条目 fit 进入 the metadata area)

     - every 扇区 the 数据 area 包含:

  - 数据 (504 bytes 数据, the 最8 bytes stored 
		  the journal 条目)
  - commit id

	test the whole journal section 曾是 written correctly, every
	512-byte 扇区 the journal ends 8-byte commit id. the
	commit id matches 全部 sectors 一journal section, 然后 它是
	assumed the section 曾是 written correctly. the commit id
	doesn't match, the section 曾是 written partially 应当 
	涓?replayed.

- one 更多 runs interleaved tags 数据.
    每个 运行 包含:

 - tag area - 包含 integrity tags. 存在 one tag 用于 每个
	  扇区 the 数据 area. The 大小 area 始终 4KiB 
	  greater.
 - 数据 area - 包含 数据 sectors. The 数字 数据 sectors
	  one 运行 必须 一电源 two. log2 stored
	  鍦?the superblock.
