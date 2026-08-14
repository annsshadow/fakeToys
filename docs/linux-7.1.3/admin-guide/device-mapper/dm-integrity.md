## dm-integrity


The dm-integrity target emulates 涓€涓?鍧?璁惧 璇?鍏锋湁 棰濆
per-sector tags 璇?鍙?涓?浣跨敤 鐢ㄤ簬 storing integrity information.

涓€涓?閫氱敤 problem 涓?storing integrity tags 涓?every 鎵囧尯 鏄?璇?
writing the 鎵囧尯 鍜?the integrity tag 蹇呴』 涓?鍘熷瓙 - i.e. 濡傛灉鍙戠敓
crash, 浠讳竴涓?涓よ€?鎵囧尯 鍜?integrity tag 鎴?none 鐨?them 鏄?written.

鍒?guarantee 鍐欏叆 atomicity, the dm-integrity target uses journal, 瀹?
writes 鎵囧尯 鏁版嵁 鍜?integrity tags 杩涘叆 涓€涓?journal, commits the journal
鍜?鐒跺悗 copies the 鏁版嵁 鍜?integrity tags 鍒?瀹冧滑鐨?respective location.

The dm-integrity target 鍙?涓?浣跨敤 涓?the dm-crypt target - 鍦?姝?
situation the dm-crypt target creates the integrity 鏁版嵁 鍜?passes them
鍒?the dm-integrity target 閫氳繃 bio_integrity_payload attached 鍒?the bio.
鍦?姝?妯″紡, the dm-crypt 鍜?dm-integrity targets 鎻愪緵 authenticated
disk encryption - 鑻?the attacker modifies the encrypted 璁惧, 涓€涓?I/O
閿欒 鏄?returned 鑰岄潪 random 鏁版嵁.

The dm-integrity target 鍙?涔?涓?浣跨敤 浣滀负 涓€涓?standalone target, 鍦?姝?
妯″紡 瀹?calculates 鍜?verifies the integrity tag internally. 鍦?姝?
妯″紡, the dm-integrity target 鍙?涓?浣跨敤 鍒?detect silent 鏁版嵁
corruption 鍦?the disk 鎴?鍦?the I/O path.

閭ｉ噷's 涓€涓?alternate 妯″紡 鐨?鎿嶄綔 浣曞 dm-integrity uses 涓€涓?bitmap
鑰岄潪 涓€涓?journal. 鑻?涓€涓?浣?鍦?the bitmap 鏄?1, the corresponding
region's 鏁版嵁 鍜?integrity tags 鏄?涓?synchronized - 鑻?the machine
crashes, the unsynchronized regions 灏?涓?recalculated. The bitmap 妯″紡
鏄?faster 姣?the journal 妯″紡, 鍥犱负 鎴戜滑 don't 鍏锋湁 鍒?鍐欏叆 the 鏁版嵁
twice, 浣?瀹冩槸 涔?less reliable, 鍥犱负 鑻?鏁版嵁 corruption happens
褰?the machine crashes, 瀹?鍙?涓?涓?detected.

褰?loading the target 鐢ㄤ簬 the 绗竴 time, the 鍐呮牳 椹卞姩 灏?鏍煎紡
the 璁惧. 浣?瀹?灏?浠?鏍煎紡 the 璁惧 鑻?the superblock 鍖呭惈
zeroes. 鑻?the superblock 鏄?涓よ€呴兘涓?valid nor zeroed, the dm-integrity
target 鍙?t 涓?loaded.

Accesses 鍒?the on-disk metadata area containing checksums (aka tags) 鏄?
buffered 浣跨敤 dm-bufio. 褰?涓€涓?access 鍒?浠讳綍 given metadata area
occurs, 姣忎釜 unique metadata area gets 鍏?own 缂撳啿鍖?s). The 缂撳啿鍖?澶у皬
鏄?capped 鍦?the 澶у皬 鐨?the metadata area, 浣?鍙?涓?smaller, thereby
requiring 澶氫釜 缂撳啿鍖?鍒?represent the full metadata area. 涓€涓?smaller
缂撳啿鍖?澶у皬 灏?produce 涓€涓?smaller resulting 璇诲彇/鍐欏叆 鎿嶄綔 鍒?the
metadata area 鐢ㄤ簬 small reads/writes. The metadata 鏄?浠嶇劧 璇诲彇 even 鍦?
涓€涓?full 鍐欏叆 鍒?the 鏁版嵁 covered 鐢?涓€涓?鍗曚釜 缂撳啿鍖?

鍒?浣跨敤 the target 鐢ㄤ簬 the 绗竴 time:

1. overwrite the superblock 涓?zeroes
2. 鍔犺浇 the dm-integrity target 涓?one-sector 澶у皬, the 鍐呮牳 椹卞姩
   灏?鏍煎紡 the 璁惧
3. 鍗歌浇 the dm-integrity target
4. 璇诲彇 the "provided_鏁版嵁_sectors" 鍊?鏉ヨ嚜 the superblock
5. 鍔犺浇 the dm-integrity target 涓?the target 澶у皬
   "provided_鏁版嵁_sectors"
6. 鑻?鎮?甯屾湜 鍒?浣跨敤 dm-integrity 涓?dm-crypt, 鍔犺浇 the dm-crypt target
   涓?the 澶у皬 "provided_鏁版嵁_sectors"


Target arguments:

1. the underlying 鍧?璁惧

2. the 鏁板瓧 鐨?reserved 鎵囧尯 鍦?the beginning 鐨?the 璁惧 - the
   dm-integrity won't 璇诲彇 鐨?鍐欏叆 杩欎簺 sectors

3. the 澶у皬 鐨?the integrity tag (鑻?"-" 鏄?浣跨敤, the 澶у皬 鏄?taken 鏉ヨ嚜
   the internal-hash algorithm)

4. 妯″紡:

	D - direct writes (鏃?journal)
		鍦?姝?妯″紡, journaling 鏄?
		涓?浣跨敤 鍜?鏁版嵁 sectors 鍜?integrity tags 鏄?written
		separately. 濡傛灉鍙戠敓 crash, 瀹冩槸 鍙兘 璇?the 鏁版嵁
		鍜?integrity tag doesn't match.
	J - journaled writes
		鏁版嵁 鍜?integrity tags 鏄?written 鍒?the
		journal 鍜?atomicity 鏄?guaranteed. 濡傛灉鍙戠敓 crash,
		浠讳竴涓?涓よ€?鏁版嵁 鍜?tag 鎴?none 鐨?them 鏄?written. The
		journaled 妯″紡 degrades 鍐欏叆 throughput twice 鍥犱负 the
		鏁版嵁 鍏锋湁 鍒?涓?written twice.
	B - bitmap 妯″紡 - 鏁版嵁 鍜?metadata 鏄?written 鏃?浠讳綍
		synchronization, the 椹卞姩 maintains 涓€涓?bitmap 鐨?dirty
		regions 浣曞 鏁版嵁 鍜?metadata don't match. 姝?妯″紡 鍙?
		浠?涓?浣跨敤 涓?鍐呴儴 hash.
	R - recovery 妯″紡 - 鍦?姝?妯″紡, journal 鏄?涓?replayed,
		checksums 鏄?涓?checked 鍜?writes 鍒?the 璁惧 鏄?涓?
		allowed. 姝?妯″紡 鏄?useful 鐢ㄤ簬 鏁版嵁 recovery 鑻?the
		璁惧 cannot 涓?activated 鍦?浠讳綍 鐨?the 鍏朵粬 鏍囧噯
		modes.
	I - inline 妯″紡 - 鍦?姝?妯″紡, dm-integrity 灏?store integrity
		鏁版嵁 directly 鍦?the underlying 璁惧 sectors.
		The underlying 璁惧 蹇呴』 鍏锋湁 涓€涓?integrity profile 璇?
		allows storing 鐢ㄦ埛 integrity 鏁版嵁 鍜?鎻愪緵 enough
		space 鐢ㄤ簬 the selected integrity tag.

5. the 鏁板瓧 鐨?棰濆 arguments

棰濆 arguments:

journal_sectors:鏁板瓧
	The 澶у皬 鐨?journal, 姝?鍙傛暟 鏄?浣跨敤 浠?鑻?formatting the
	璁惧. 鑻?the 璁惧 鏄?宸茬粡 formatted, the 鍊?鏉ヨ嚜 the
	superblock 鏄?浣跨敤.

interleave_sectors:鏁板瓧 (榛樿 32768)
	The 鏁板瓧 鐨?interleaved sectors. 姝?鍊?鏄?rounded down 鍒?
	涓€涓?鐢垫簮 鐨?two. 鑻?the 璁惧 鏄?宸茬粡 formatted, the 鍊?鏉ヨ嚜
	the superblock 鏄?浣跨敤.

meta_璁惧:璁惧
	Don't interleave the 鏁版嵁 鍜?metadata 鍦?the 璁惧. 浣跨敤 涓€涓?
	separate 璁惧 鐢ㄤ簬 metadata.

缂撳啿鍖篲sectors:鏁板瓧 (榛樿 128)
	The 鏁板瓧 鐨?sectors 鍦?one metadata 缂撳啿鍖? The 鍊?鏄?rounded
	down 鍒?涓€涓?鐢垫簮 鐨?two.

journal_watermark:鏁板瓧 (榛樿 50)
	The journal watermark 鍦?percents. 褰?the 澶у皬 鐨?the journal
	exceeds 姝?watermark, the 绾跨▼ 璇?flushes the journal 灏?
	涓?started.

commit_time:鏁板瓧 (榛樿 10000)
	Commit time 鍦?milliseconds. 褰?姝?time passes, the journal 鏄?
	written. The journal 鏄?涔?written immediately 鑻?the FLUSH
	璇锋眰 鏄?received.

鍐呴儴_hash:algorithm(:key)	(the key 鏄?鍙€?
	浣跨敤 鍐呴儴 hash 鎴?crc.
	褰?姝?鍙傛暟 鏄?浣跨敤, the dm-integrity target won't accept
	integrity tags 鏉ヨ嚜 the upper target, 浣?瀹?灏?automatically
	generate 鍜?verify the integrity tags.

	鎮ㄥ彲浠?浣跨敤 涓€涓?crc algorithm (渚嬪 crc32), 鐒跺悗 integrity target
	灏?protect the 鏁版嵁 against accidental corruption.
	鎮ㄥ彲浠?涔?浣跨敤 涓€涓?hmac algorithm (渚嬪
	"hmac(sha256):0123456789abcdef"), 鍦?姝?妯″紡 瀹?灏?鎻愪緵
	cryptographic authentication 鐨?the 鏁版嵁 鏃?encryption.

	褰?姝?鍙傛暟 鏄?涓?浣跨敤, the integrity tags 鏄?accepted
	鏉ヨ嚜 涓€涓?upper layer target, 渚嬪 dm-crypt. The upper layer
	target 搴斿綋 check the validity 鐨?the integrity tags.

recalculate
	Recalculate the integrity tags automatically. 瀹冩槸 浠?valid
	褰?浣跨敤 鍐呴儴 hash.

journal_crypt:algorithm(:key)	(the key 鏄?鍙€?
	Encrypt the journal 浣跨敤 given algorithm 鍒?纭繚 璇?the
	attacker 鍙?t 璇诲彇 the journal. 鎮ㄥ彲浠?浣跨敤 涓€涓?鍧?cipher 姝ゅ
	(渚嬪 "cbc(aes)") 鎴?涓€涓?娴?cipher (渚嬪 "chacha20"
	鎴?"ctr(aes)").

	The journal 鍖呭惈 history 鐨?鏈€鍚?writes 鍒?the 鍧?璁惧,
	涓€涓?attacker reading the journal 鍙互 鍙傝 the 鏈€鍚?鎵囧尯 numbers
	璇?鏇炬槸 written. 鏉ヨ嚜 the 鎵囧尯 numbers, the attacker 鍙?infer
	the 澶у皬 鐨?鏂囦欢 璇?鏇炬槸 written. 鍒?protect against 姝?
	situation, 鎮ㄥ彲浠?encrypt the journal.

journal_mac:algorithm(:key)	(the key 鏄?鍙€?
	Protect 鎵囧尯 numbers 鍦?the journal 鏉ヨ嚜 accidental 鎴?malicious
	modification. 鍒?protect against accidental modification, 浣跨敤 涓€涓?
	crc algorithm, 鍒?protect against malicious modification, 浣跨敤 涓€涓?
	hmac algorithm 涓?涓€涓?key.

	姝?閫夐」 鏄?涓?needed 褰?浣跨敤 internal-hash 鍥犱负 鍦?姝?
	妯″紡, the integrity 鐨?journal 鏉＄洰 鏄?checked 褰?replaying
	the journal. 浠庤€? modified 鎵囧尯 鏁板瓧 灏嗕細 涓?detected 鍦?
	姝?stage.

鍧梍澶у皬:鏁板瓧 (榛樿 512)
	The 澶у皬 鐨?涓€涓?鏁版嵁 鍧?鍦?bytes. The larger the 鍧?澶у皬 the
	less overhead 瀛樺湪 鐢ㄤ簬 per-block integrity metadata.
	鍙楁敮鎸?鍊?鏄?512, 1024, 2048 鍜?4096 bytes.

sectors_姣廮浣?鏁板瓧
	鍦?the bitmap 妯″紡, 姝?鍙傛暟 specifies the 鏁板瓧 鐨?
	512-byte sectors 璇?corresponds 鍒?one bitmap 浣?

bitmap_flush_interval:鏁板瓧
	The bitmap flush interval 鍦?milliseconds. The metadata 缂撳啿鍖?
	鏄?synchronized 褰?姝?interval expires.

鍏佽_discards
	鍏佽 鍧?discard requests (涓€涓?k.涓€涓? TRIM) 鐢ㄤ簬 the integrity 璁惧.
	Discards 鏄?浠?allowed 鍒?璁惧 浣跨敤 鍐呴儴 hash.

fix_padding
	浣跨敤 涓€涓?smaller padding 鐨?the tag area 鍗?鏇村
	space-efficient. 鑻?姝?閫夐」 鏄?涓?present, large padding 鏄?
	浣跨敤 - 鍗?鐢ㄤ簬 compatibility 涓?older kernels.

fix_hmac
	Improve 瀹夊叏 鐨?鍐呴儴_hash 鍜?journal_mac:

 - the section 鏁板瓧 鏄?mixed 鍒?the mac, 鍥犳 璇?涓€涓?attacker 鍙?t
	  copy sectors 鏉ヨ嚜 one journal section 鍒?another journal section
 - the superblock 鏄?protected 鐢?journal_mac
 - 涓€涓?16-byte salt stored 鍦?the superblock 鏄?mixed 鍒?the mac, 鍥犳
	  璇?the attacker 鍙?t detect 璇?two disks 鍏锋湁 the 鐩稿悓 hmac
	  key 鍜?涔?鍒?disallow the attacker 鍒?move sectors 鏉ヨ嚜 one
	  disk 鍒?another

legacy_recalculate
	鍏佽 recalculating 鐨?volumes 涓?HMAC keys. 杩欐槸 宸茬鐢?鐢?
	榛樿 鐢ㄤ簬 瀹夊叏 reasons - 涓€涓?attacker 鍙互 modify the volume,
	set recalc_鎵囧尯 鍒?zero, 鍜?the 鍐呮牳 灏嗕細 涓?detect the
	modification.

The journal 妯″紡 (D/J), 缂撳啿鍖篲sectors, journal_watermark, commit_time 鍜?
鍏佽_discards 鍙?涓?changed 褰?reloading the target (鍔犺浇 涓€涓?inactive
琛?鍜?swap the 琛?涓?suspend 鍜?resume). The 鍏朵粬 arguments
搴斿綋 涓?涓?changed 褰?reloading the target 鍥犱负 the layout 鐨?disk
鏁版嵁 depend 鍦?them 鍜?the reloaded target 灏嗕細 涓?non-functional.

渚嬪, 鍦?涓€涓?璁惧 浣跨敤 the 榛樿 interleave_sectors 鐨?32768, 涓€涓?
鍧梍澶у皬 鐨?512, 鍜?涓€涓?鍐呴儴_hash 鐨?crc32c 涓?涓€涓?tag 澶у皬 鐨?4
bytes, 瀹?灏?take 128 KiB 鐨?tags 鍒?track 涓€涓?full 鏁版嵁 area, requiring
256 sectors 鐨?metadata 姣?鏁版嵁 area. 涓?the 榛樿 缂撳啿鍖篲sectors 鐨?
128, 璇?means 閭ｉ噷 灏?涓?2 缂撳啿鍖?姣?metadata area, 鎴?2 缂撳啿鍖?
姣?16 MiB 鐨?鏁版嵁.

鐘舵€?line:

1. the 鏁板瓧 鐨?integrity mismatches
2. provided 鏁版嵁 sectors - 鍗?the 鏁板瓧 鐨?sectors 璇?the 鐢ㄦ埛
   鍙互 浣跨敤
3. the 鐢垫祦 recalculating position (鎴?'-' 鑻?鎴戜滑 didn't recalculate)


The layout 鐨?the formatted 鍧?璁惧:

- reserved sectors
    (瀹冧滑鏄?涓?浣跨敤 鐢?姝?target, 瀹冧滑 鍙?涓?浣跨敤 鐢ㄤ簬
    storing LUKS metadata 鎴?鐢ㄤ簬 鍏朵粬 purpose), the 澶у皬 鐨?the reserved
    area 鏄?specified 鍦?the target arguments

- superblock (4kiB)
 - magic 瀛楃涓?- identifies 璇?the 璁惧 鏇炬槸 formatted
 - 鐗堟湰
 - log2(interleave sectors)
 - integrity tag 澶у皬
 - the 鏁板瓧 鐨?journal sections
 - provided 鏁版嵁 sectors - the 鏁板瓧 鐨?sectors 璇?姝?target
	  鎻愪緵 (i.e. the 澶у皬 鐨?the 璁惧 minus the 澶у皬 鐨?鍏ㄩ儴
	  metadata 鍜?padding). The 鐢ㄦ埛 鐨?姝?target 搴斿綋 涓?send
	  bios 璇?access 鏁版嵁 beyond the "provided 鏁版嵁 sectors" limit.
 - 鏍囧織
	    SB_鏍囧織_鍏锋湁_JOURNAL_MAC
  - 涓€涓?鏍囧織 鏄?set 鑻?journal_mac 鏄?浣跨敤
	    SB_鏍囧織_RECALCULATING
  - recalculating 鏄?鍦?progress
	    SB_鏍囧織_DIRTY_BITMAP
  - journal area 鍖呭惈 the bitmap 鐨?dirty
		  鍧?
 - log2(sectors 姣?鍧?
 - 涓€涓?position 浣曞 recalculating finished
- journal
	The journal 鏄?divided 杩涘叆 sections, 姣忎釜 section 鍖呭惈:

 - metadata area (4kiB), 瀹?鍖呭惈 journal 鏉＄洰

   - every journal 鏉＄洰 鍖呭惈:

  - logical 鎵囧尯 (specifies 浣曞 the 鏁版嵁 鍜?tag 搴斿綋
		  涓?written)
  - 鏈€鍚?8 bytes 鐨?鏁版嵁
  - integrity tag (the 澶у皬 鏄?specified 鍦?the superblock)

   - every metadata 鎵囧尯 ends 涓?

  - mac (8-bytes), 鍏ㄩ儴 the macs 鍦?8 metadata sectors form 涓€涓?
		  64-byte 鍊? 瀹冩槸 浣跨敤 鍒?store hmac 鐨?鎵囧尯
		  numbers 鍦?the journal section, 鍒?protect against 涓€涓?
		  possibility 璇?the attacker tampers 涓?鎵囧尯
		  numbers 鍦?the journal.
  - commit id

 - 鏁版嵁 area (the 澶у皬 鏄?variable; 瀹?depends 鍦?濡備綍 璁稿 journal
	  鏉＄洰 fit 杩涘叆 the metadata area)

     - every 鎵囧尯 鍦?the 鏁版嵁 area 鍖呭惈:

  - 鏁版嵁 (504 bytes 鐨?鏁版嵁, the 鏈€鍚?8 bytes 鏄?stored 鍦?
		  the journal 鏉＄洰)
  - commit id

	鍒?test 鑻?the whole journal section 鏇炬槸 written correctly, every
	512-byte 鎵囧尯 鐨?the journal ends 涓?8-byte commit id. 鑻?the
	commit id matches 鍦?鍏ㄩ儴 sectors 鍦?涓€涓?journal section, 鐒跺悗 瀹冩槸
	assumed 璇?the section 鏇炬槸 written correctly. 鑻?the commit id
	doesn't match, the section 鏇炬槸 written partially 鍜?瀹?搴斿綋 涓?
	涓?replayed.

- one 鎴?鏇村 runs 鐨?interleaved tags 鍜?鏁版嵁.
    姣忎釜 杩愯 鍖呭惈:

 - tag area - 瀹?鍖呭惈 integrity tags. 瀛樺湪 one tag 鐢ㄤ簬 姣忎釜
	  鎵囧尯 鍦?the 鏁版嵁 area. The 澶у皬 鐨?姝?area 鏄?濮嬬粓 4KiB 鎴?
	  greater.
 - 鏁版嵁 area - 瀹?鍖呭惈 鏁版嵁 sectors. The 鏁板瓧 鐨?鏁版嵁 sectors
	  鍦?one 杩愯 蹇呴』 涓?涓€涓?鐢垫簮 鐨?two. log2 鐨?姝?鍊?鏄?stored
	  鍦?the superblock.
