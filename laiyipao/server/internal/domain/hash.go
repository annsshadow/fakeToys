package domain

import "hash/fnv"

// SnapshotHash 计算防线快照的稳定摘要。
//
// 用 FNV-1a 64 位而非 sha256：快照不是安全边界（防作弊靠 token 与上限校验），
// 只需要一个快速、跨端一致、可放进 URL 的短标识。
// 返回 16 位十六进制字符串，与 replay_hash 格式保持一致便于前端统一处理。
func SnapshotHash(raw []byte) string {
	h := fnv.New64a()
	_, _ = h.Write(raw)
	return hex16(h.Sum64())
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
