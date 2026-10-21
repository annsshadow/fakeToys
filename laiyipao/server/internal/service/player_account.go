package service

// 玩家账号（用户名 + 密码登录）。
//
// ## 它解决什么问题
//
// 游客通道给每个浏览器/设备一个独立账号，凭证存在客户端 localStorage。
// 对「随手玩两下」是合适的，对**测试环境**不合适：
//
//   - 测试数据散落在无数游客号里，后台看到的是一堆孤立的号；
//   - 换浏览器/清缓存 = 新号 = 「我刚打的关卡呢」；
//   - 多人同时测时互相看不见对方的进度。
//
// 所以这里提供「一个有口令的玩家身份」。测试环境把它开成**共享账号**
// （全体测试者用同一个），数据就聚在一处了。
//
// ## 与管理员账号完全独立
//
// 管理员在 `admin_users` 表、令牌 `kind=admin`；玩家在 `users` 表、
// 令牌 `kind=user`。两者不共享任何一行，也不共享任何权限面。
// 「测试账号的口令与超管一致」是**运维的选择**（省一套凭据），
// 不是数据模型的一部分 —— 见 config.PlayerLoginRequired 的注释。

import (
	"context"
	"errors"
	"fmt"
	"strings"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/auth"
)

// Login 用用户名 + 密码登录玩家账号。
//
// # 为什么错误信息与后台登录一致
//
// 「用户不存在」与「密码错误」返回**同一条**错误（ErrUnauthorized +
// 「用户名或密码错误」）。分开报就等于给了一个用户名枚举器：
// 攻击者能据此知道哪些用户名存在。
//
// # 为什么先验密码再查状态
//
// 后台登录是「先查 status 再验密码」（admin.go 的 AdminLogin），
// 于是「账号已停用」这条错误本身就是枚举信号。
// 这里反过来：密码不对就一律说「用户名或密码错误」，
// 封禁状态只有在**密码正确**之后才暴露 —— 那时对方已经证明了知情。
func (s *Service) Login(ctx context.Context, username, password string) (TokenPair, error) {
	username = strings.TrimSpace(username)
	if username == "" || password == "" {
		return TokenPair{}, fmt.Errorf("%w: 用户名或密码不能为空", ErrBadInput)
	}

	var (
		id     int64
		hash   string
		status int
	)
	err := s.pool.QueryRow(ctx,
		`SELECT id, password_hash, status FROM users
		 WHERE username = $1 AND password_hash IS NOT NULL`, username).
		Scan(&id, &hash, &status)
	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return TokenPair{}, fmt.Errorf("%w: 用户名或密码错误", ErrUnauthorized)
		}
		return TokenPair{}, fmt.Errorf("load player account: %w", err)
	}

	if err := VerifyPassword(hash, password); err != nil {
		return TokenPair{}, fmt.Errorf("%w: 用户名或密码错误", ErrUnauthorized)
	}
	if status != 1 {
		return TokenPair{}, fmt.Errorf("%w: 账号已被封禁", ErrForbidden)
	}

	u, err := s.loadUser(ctx, id)
	if err != nil {
		return TokenPair{}, err
	}
	return s.issueTokens(ctx, id, u)
}

// EnsureBootstrapPlayer 引导创建（或同步）共享测试账号。
//
// # 语义：账号是**长期资产**，密码是**可重置**的
//
// 账号已存在时**只同步密码**，绝不碰其它数据 ——
// 钱包、关卡进度、专精、战报、防线全都挂在这一行上，
// 「重建账号」会把测试数据清空 —— 那正是这个功能要解决的事。
//
// # 为什么每次启动都同步密码
//
// 因为这个账号的口令**就是运营超管的口令**（config 里刻意复用同一组变量）。
// 超管口令改了而测试账号没改，就会出现「后台能登、H5 登不进」，
// 而排查时很难想到两者本该一致。
//
// 代价：不能给测试账号单独设一个口令。要改就改 BOOTSTRAP_ADMIN_PASS，
// 重启后两边一起变 —— 这是「一致」这个语义本身要求的。
//
// # 返回值
//
// created=true 表示**新建**了账号（首启）；false 表示已存在、只同步了密码。
func (s *Service) EnsureBootstrapPlayer(ctx context.Context, username, password string) (bool, error) {
	username = strings.TrimSpace(username)
	if username == "" {
		return false, fmt.Errorf("测试账号需要用户名")
	}
	// 门槛与超管一致：至少 10 位。默认口令 admin12345 恰好 10 位，
	// 能通过 —— 所以这里**不**依赖长度做安全判断，
	// 只是别让一个 1 位口令被写进库（那通常是配置写错了）。
	if len(password) < 10 {
		return false, fmt.Errorf("测试账号口令至少 10 位（当前 %d 位）", len(password))
	}
	hash, err := HashPassword(password)
	if err != nil {
		return false, err
	}

	var id int64
	err = s.pool.QueryRow(ctx, `SELECT id FROM users WHERE username = $1`, username).Scan(&id)
	switch {
	case err == nil:
		if _, err := s.pool.Exec(ctx,
			`UPDATE users SET password_hash = $2 WHERE id = $1`, id, hash); err != nil {
			return false, fmt.Errorf("sync test account password: %w", err)
		}
		return false, nil
	case errors.Is(err, pgx.ErrNoRows):
		// 新建，见下。
	default:
		return false, fmt.Errorf("lookup test account: %w", err)
	}

	// `users.guest_token` 是 NOT NULL UNIQUE —— 新账号也得有一枚。
	// 它对账号登录没有意义（账号靠口令登录），但列不允许空，
	// 给一枚随机值最省事，且不会与任何游客号撞上。
	guestToken, err := auth.NewGuestToken()
	if err != nil {
		return false, err
	}
	// ⚠️ 必须用 QueryRow + RETURNING 拿 id：`initNewUser` 要用它发新手包。
	if err := s.pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, username, password_hash, nickname, is_guest, last_login_at)
		 VALUES ($1, $2, $3, $4, FALSE, now())
		 RETURNING id`, guestToken, username, hash, username).Scan(&id); err != nil {
		return false, fmt.Errorf("create test account: %w", err)
	}
	if err := s.initNewUser(ctx, id); err != nil {
		return false, err
	}
	return true, nil
}
