package domain

// 快照摘要（hash.go）的契约测试。
//
// SnapshotHash 不是安全哈希（注释里说得很清楚：防作弊靠 token 与上限校验），
// 它的契约是三条：**确定**（同输入同输出，跨端对齐的前提）、
// **稳定**（不随进程/平台变化）、**定长 16 位十六进制**（与 replay_hash
// 格式一致，前端按同一格式处理）。

import "testing"

// FNV-1a 64 的空串偏移基值是公开常量 0xcbf29ce484222325。
// 用它钉住算法本身：换掉哈希实现（比如误改成 FNV-1）会立刻红。
const fnv1a64OffsetBasis = "cbf29ce484222325"

func TestSnapshotHashIsStableAndDeterministic(t *testing.T) {
	h1 := SnapshotHash([]byte(`{"defense":[1,2,3]}`))
	h2 := SnapshotHash([]byte(`{"defense":[1,2,3]}`))
	if h1 != h2 {
		t.Fatalf("同输入必须同输出：%s != %s", h1, h2)
	}
	if len(h1) != 16 {
		t.Fatalf("摘要必须定长 16 位十六进制，实际 %d 位：%s", len(h1), h1)
	}
	for _, c := range h1 {
		if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) {
			t.Fatalf("摘要必须是小写十六进制，出现非法字符 %q：%s", c, h1)
		}
	}
	if SnapshotHash([]byte(`{"defense":[1,2,4]}`)) == h1 {
		t.Fatal("不同输入不应碰撞（单探测，防实现退化成常量）")
	}
}

func TestSnapshotHashKnownVector(t *testing.T) {
	// 空输入的 FNV-1a 64 结果就是偏移基值（未消费任何字节）。
	if got := SnapshotHash(nil); got != fnv1a64OffsetBasis {
		t.Fatalf("空输入应为 FNV-1a 偏移基值 %s，实际 %s", fnv1a64OffsetBasis, got)
	}
	// "a" 的 FNV-1a 64 已知值（RFC 风格公开向量，多实现交叉验证过）。
	if got := SnapshotHash([]byte("a")); got != "af63dc4c8601ec8c" {
		t.Fatalf("输入 \"a\" 的已知向量不匹配，实际 %s", got)
	}
}

func TestHex16(t *testing.T) {
	for _, c := range []struct {
		in   uint64
		want string
	}{
		{0, "0000000000000000"},
		{1, "0000000000000001"},
		{^uint64(0), "ffffffffffffffff"},
		{0xcbf29ce484222325, fnv1a64OffsetBasis},
	} {
		if got := Hex16(c.in); got != c.want {
			t.Errorf("Hex16(%#x) = %s，期望 %s", c.in, got, c.want)
		}
	}
}
