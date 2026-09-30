// Package auth 负责令牌签发与校验：access token（短期 JWT）+ refresh token（长期、落库可吊销）。
//
// 设计取舍：不用单一长期 token，是因为休闲游戏客户端常被逆向；
// access token 短命（默认 2h）+ refresh 可吊销，是成本与安全的合理平衡。
// 未引入第三方 JWT 库 —— 令牌结构简单，自己实现 HS256 并严格校验更可控。
package auth

import (
	"crypto/hmac"
	"crypto/rand"
	"crypto/sha256"
	"crypto/subtle"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"strconv"
	"strings"
	"time"
)

var (
	// ErrInvalidToken 表示令牌结构或签名非法。
	ErrInvalidToken = errors.New("令牌非法")
	// ErrExpiredToken 表示令牌已过期。
	ErrExpiredToken = errors.New("令牌已过期")
)

// Claims 是 access token 载荷。
type Claims struct {
	Sub      int64  `json:"sub"` // user_id 或 admin_id
	Kind     string `json:"knd"` // "user" / "admin"
	IssuedAt int64  `json:"iat"`
	Expires  int64  `json:"exp"`
}

// Signer 签发与校验 HS256 JWT。
type Signer struct {
	secret []byte
	ttl    time.Duration
}

// NewSigner 构造签发器。
func NewSigner(secret string, ttl time.Duration) *Signer {
	return &Signer{secret: []byte(secret), ttl: ttl}
}

// TTL 返回令牌有效期。
func (s *Signer) TTL() time.Duration { return s.ttl }

var b64 = base64.RawURLEncoding

// Issue 签发 access token。
func (s *Signer) Issue(userID int64, kind string) (string, time.Time, error) {
	now := time.Now()
	exp := now.Add(s.ttl)
	header := b64.EncodeToString([]byte(`{"alg":"HS256","typ":"JWT"}`))
	payload, err := json.Marshal(Claims{
		Sub: userID, Kind: kind, IssuedAt: now.Unix(), Expires: exp.Unix(),
	})
	if err != nil {
		return "", time.Time{}, fmt.Errorf("marshal claims: %w", err)
	}
	body := header + "." + b64.EncodeToString(payload)
	return body + "." + s.sign(body), exp, nil
}

func (s *Signer) sign(body string) string {
	mac := hmac.New(sha256.New, s.secret)
	mac.Write([]byte(body))
	return b64.EncodeToString(mac.Sum(nil))
}

// Verify 校验令牌并返回载荷。
func (s *Signer) Verify(token string) (*Claims, error) {
	parts := strings.Split(token, ".")
	if len(parts) != 3 {
		return nil, ErrInvalidToken
	}
	body := parts[0] + "." + parts[1]
	// 使用 subtle 恒定时间比较，避免时序侧信道
	if subtle.ConstantTimeCompare([]byte(parts[2]), []byte(s.sign(body))) != 1 {
		return nil, ErrInvalidToken
	}
	raw, err := b64.DecodeString(parts[1])
	if err != nil {
		return nil, ErrInvalidToken
	}
	var c Claims
	if err := json.Unmarshal(raw, &c); err != nil {
		return nil, ErrInvalidToken
	}
	if time.Now().Unix() >= c.Expires {
		return nil, ErrExpiredToken
	}
	return &c, nil
}

// NewRefreshToken 生成高熵 refresh token。
// 返回明文（发给客户端）与 SHA-256 摘要（落库）—— 数据库被拖库也无法直接冒用。
func NewRefreshToken() (plain string, hash string, err error) {
	buf := make([]byte, 32)
	if _, err := rand.Read(buf); err != nil {
		return "", "", fmt.Errorf("read random: %w", err)
	}
	plain = base64.RawURLEncoding.EncodeToString(buf)
	return plain, HashRefreshToken(plain), nil
}

// HashRefreshToken 计算 refresh token 的落库摘要。
func HashRefreshToken(plain string) string {
	sum := sha256.Sum256([]byte(plain))
	return hex.EncodeToString(sum[:])
}

// NewGuestToken 生成游客设备凭证。
func NewGuestToken() (string, error) {
	buf := make([]byte, 24)
	if _, err := rand.Read(buf); err != nil {
		return "", fmt.Errorf("read random: %w", err)
	}
	return "g_" + hex.EncodeToString(buf), nil
}

// NewAdminToken 生成管理员会话令牌。
// 摘要必须对**带前缀的完整明文**计算（与 service.NewAdminSession 一致）：
// 落库与校验都按收到的完整明文复算，若这里只对裸随机串算哈希，
// 两个入口签发的令牌会有一方永远验证失败。
func NewAdminToken() (string, string, error) {
	plain, _, err := NewRefreshToken()
	if err != nil {
		return "", "", err
	}
	token := "a_" + plain
	return token, HashRefreshToken(token), nil
}

// FormatInt64 把 int64 转为字符串（token 落库用）。
func FormatInt64(v int64) string { return strconv.FormatInt(v, 10) }
