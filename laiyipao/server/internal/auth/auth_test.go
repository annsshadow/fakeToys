package auth

import (
	"testing"
	"time"

	"golang.org/x/crypto/bcrypt"
)

func TestIssueAndVerify(t *testing.T) {
	s := NewSigner("test-secret", time.Hour)
	token, exp, err := s.Issue(42, "user")
	if err != nil {
		t.Fatalf("Issue 失败：%v", err)
	}
	if exp.Before(time.Now()) {
		t.Error("过期时间应在未来")
	}
	c, err := s.Verify(token)
	if err != nil {
		t.Fatalf("Verify 失败：%v", err)
	}
	if c.Sub != 42 || c.Kind != "user" {
		t.Errorf("载荷错误：%+v", c)
	}
}

func TestVerifyRejectsTamperedToken(t *testing.T) {
	s := NewSigner("test-secret", time.Hour)
	token, _, _ := s.Issue(1, "user")

	t.Run("改载荷", func(t *testing.T) {
		forged := "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9." +
			"eyJzdWIiOjk5OTksImtub2QiOiJ1c2VyIiwiaWF0IjoxLCJleHAiOjk5OTk5OTk5OTl9.forged"
		if _, err := s.Verify(forged); err == nil {
			t.Error("伪造签名应被拒绝")
		}
	})

	t.Run("换密钥", func(t *testing.T) {
		other := NewSigner("other-secret", time.Hour)
		if _, err := other.Verify(token); err == nil {
			t.Error("不同密钥签发的令牌应被拒绝")
		}
	})

	t.Run("结构残缺", func(t *testing.T) {
		for _, bad := range []string{"", "a", "a.b", "a.b.c.d", "..."} {
			if _, err := s.Verify(bad); err == nil {
				t.Errorf("残缺令牌 %q 应被拒绝", bad)
			}
		}
	})
}

func TestVerifyRejectsExpired(t *testing.T) {
	s := NewSigner("test-secret", -time.Minute) // 直接签发已过期的
	token, _, _ := s.Issue(1, "user")
	if _, err := s.Verify(token); err != ErrExpiredToken {
		t.Errorf("应返回 ErrExpiredToken，实际 %v", err)
	}
}

func TestRefreshTokenHash(t *testing.T) {
	plain, hash, err := NewRefreshToken()
	if err != nil {
		t.Fatalf("生成失败：%v", err)
	}
	if plain == "" || hash == "" {
		t.Fatal("令牌或摘要为空")
	}
	if hash == plain {
		t.Error("摘要不应等于明文")
	}
	// 摘要必须稳定可复算，否则刷新链路会断
	if HashRefreshToken(plain) != hash {
		t.Error("HashRefreshToken 不可复算")
	}
	// 两次生成不应相同
	plain2, _, _ := NewRefreshToken()
	if plain == plain2 {
		t.Error("两次生成的 refresh token 相同，随机性不足")
	}
}

func TestGuestTokenFormat(t *testing.T) {
	tok, err := NewGuestToken()
	if err != nil {
		t.Fatalf("生成失败：%v", err)
	}
	if len(tok) < 10 || tok[:2] != "g_" {
		t.Errorf("游客令牌格式异常：%q", tok)
	}
}

func TestPasswordHashRoundTrip(t *testing.T) {
	hash, err := bcrypt.GenerateFromPassword([]byte("admin12345"), bcrypt.DefaultCost)
	if err != nil {
		t.Fatalf("生成哈希失败：%v", err)
	}
	if err := bcrypt.CompareHashAndPassword(hash, []byte("admin12345")); err != nil {
		t.Errorf("正确密码应通过：%v", err)
	}
	if err := bcrypt.CompareHashAndPassword(hash, []byte("wrong")); err == nil {
		t.Error("错误密码应被拒绝")
	}
}
