package domain

import (
	"hash/fnv"
)

// fnv1a64 返回字符串的 FNV-1a 64 位摘要。
//
// ⚠️ 必须与 TS `lcg.ts` 的 `fnv1a64` **逐位一致** ——
// 它是 `SnapshotDigest`（客户端）与 `SnapshotHash`（服务端）两侧的共同底座。
//
// 这里用 `hash/fnv` 的 New64a 而不是手写循环：手写版本一旦在
// 「是否含终止字节」或移位顺序上与标准差一位，两端就会静默失配 ——
// 而症状是「快照摘要永远对不上」，看起来像数据坏了。
// 标准库那个是规范实现，不给这种错误留机会。
func fnv1a64(s string) uint64 {
	h := fnv.New64a()
	_, _ = h.Write([]byte(s))
	return h.Sum64()
}

// SnapshotHash 计算防线快照的稳定摘要。
//
// 用 FNV-1a 64 位而非 sha256：快照不是安全边界（防作弊靠 token 与上限校验），
// 只需要一个快速、跨端一致、可放进 URL 的短标识。
// 返回 16 位十六进制字符串，与 replay_hash 格式保持一致便于前端统一处理。
func SnapshotHash(raw []byte) string {
	return hex16(fnv1a64(string(raw)))
}

func hex16(v uint64) string {
	const digits = "0123456789abcdef"
	var buf [16]byte
	for i := 15; i >= 0; i-- {
		buf[i] = digits[v&0xF]
		v >>= 4
	}
	return string(buf[:])
}

// Hex16 把 uint64 格式化为 16 位十六进制（回放哈希用）。
func Hex16(v uint64) string { return hex16(v) }
