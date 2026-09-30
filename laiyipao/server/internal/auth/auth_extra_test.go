package auth

// 补齐 auth.go 剩余分支的测试。
//
// 已有 auth_test.go 覆盖签发/校验主路径；本文件钉住三类此前无人管的点：
//  1. Verify 对「签名合法但载荷非法」的令牌必须拒绝 —— 这是最后一道防线：
//     签名只保证"密钥持有者签过"，不保证载荷本身是合法 JSON。
//  2. NewAdminToken 的 a_ 前缀契约（httpapi 侧的 admin 会话依赖它区分于玩家）。
//  3. TTL / FormatInt64 这类一眼看懂的小函数 —— 不测的话它们的覆盖率
//     会把整个包拖下 100%，且未来若 TTL 被改成读可变字段，没有报警。

import (
	"encoding/base64"
	"strings"
	"testing"
	"time"
)

// TestVerifyRejectsBadPayload 签名合法、载荷非法的令牌必须被拒。
//
// 构造方式：body 任意取（含非法 base64 或非法 JSON），再用**真密钥**签它。
// 这类令牌无法伪造（签名校验会通过吗？会 —— 签名是我们自己算的），
// 但载荷解码/反序列化必然失败。Verify 必须返回 ErrInvalidToken，
// 而不是 panic 或返回零值 Claims。
func TestVerifyRejectsBadPayload(t *testing.T) {
	s := NewSigner("test-secret", time.Hour)

	t.Run("载荷不是合法base64", func(t *testing.T) {
		body := b64.EncodeToString([]byte(`{"alg":"HS256","typ":"JWT"}`)) + "." + "!!!not-base64!!!"
		token := body + "." + s.sign(body)
		claims, err := s.Verify(token)
		if err == nil {
			t.Fatalf("非法 base64 载荷应被拒绝，实际返回 claims=%+v", claims)
		}
		if err != ErrInvalidToken {
			t.Errorf("应返回 ErrInvalidToken，实际 %v", err)
		}
	})

	t.Run("载荷解码后不是合法JSON", func(t *testing.T) {
		body := b64.EncodeToString([]byte(`{"alg":"HS256","typ":"JWT"}`)) + "." +
			b64.EncodeToString([]byte(`这不是JSON`))
		token := body + "." + s.sign(body)
		if _, err := s.Verify(token); err != ErrInvalidToken {
			t.Errorf("非 JSON 载荷应返回 ErrInvalidToken，实际 %v", err)
		}
	})
}

// TestTTL 报告构造时的有效期。
// 意义：Refresh / 会话管理用 Signer.TTL 决定落库过期时间，
// 若有人把它改成"永远返回 0"，所有会话立刻过期且测试不红。
func TestTTL(t *testing.T) {
	want := 7 * time.Minute
	s := NewSigner("k", want)
	if got := s.TTL(); got != want {
		t.Errorf("TTL() = %s，期望 %s", got, want)
	}
}

// TestNewAdminToken 管理员会话令牌的契约：
// 明文带 a_ 前缀（与玩家的 g_ 游客凭证、裸 refresh token 区分开），
// 摘要是对**带前缀明文**的 SHA-256 —— 落库校验时按同一规则复算。
func TestNewAdminToken(t *testing.T) {
	plain, hash, err := NewAdminToken()
	if err != nil {
		t.Fatalf("生成失败：%v", err)
	}
	if !strings.HasPrefix(plain, "a_") {
		t.Errorf("管理员会话明文应以 a_ 开头，实际 %q", plain)
	}
	if plain == "a_" {
		t.Error("前缀后必须有随机部分")
	}
	if hash != HashRefreshToken(plain) {
		t.Error("摘要应等于对带前缀明文的 SHA-256（落库按此复算）")
	}
	// 两次生成不同 —— 随机性不足会话即可预测
	plain2, _, _ := NewAdminToken()
	if plain == plain2 {
		t.Error("两次生成的会话令牌相同")
	}
}

// TestFormatInt64 token 落库把 int64 主键转字符串，语义必须与 strconv 一致。
func TestFormatInt64(t *testing.T) {
	cases := map[int64]string{
		0: "0", 1: "1", -1: "-1",
		9223372036854775807: "9223372036854775807",
	}
	for in, want := range cases {
		if got := FormatInt64(in); got != want {
			t.Errorf("FormatInt64(%d) = %q，期望 %q", in, got, want)
		}
	}
}

// TestNewRefreshTokenIsHighEntropy 摘要输入就是明文本身（此前 auth_test 已验证
// 复算一致）；这里补一条：明文必须是合法 base64url —— 它会进 HTTP 响应体，
// 若换实现带出 + / = 等字符，客户端不转义就会破坏 JSON/头部。
func TestNewRefreshTokenIsURLSafe(t *testing.T) {
	plain, _, err := NewRefreshToken()
	if err != nil {
		t.Fatalf("生成失败：%v", err)
	}
	if _, err := base64.RawURLEncoding.DecodeString(plain); err != nil {
		t.Errorf("明文应是合法 base64url（32 字节编码后 43 字符），实际 %q：%v", plain, err)
	}
}
