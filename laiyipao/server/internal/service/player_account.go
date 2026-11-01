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
	"unicode/utf8"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/auth"
)

// maxUsernameLen 是玩家用户名的最大长度（按字符计，不按字节）。
const maxUsernameLen = 32

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

	// 2026-10-31（新一轮第 12 轮）：账号密码登录此前**不更新**
	// `last_login_at` —— 游客登录（GuestLogin）与微信登录（WechatLogin）
	// 都走 `ON CONFLICT DO UPDATE SET last_login_at = now()`，唯独账号登录
	// 只读不写，于是 `/me` 的「最近登录」对账号玩家恒等于注册时间。
	// 先落时间戳再读用户，返回的 TokenPair.User 也带最新值。
	if _, err := s.pool.Exec(ctx,
		`UPDATE users SET last_login_at = now() WHERE id = $1`, id); err != nil {
		return TokenPair{}, fmt.Errorf("touch player login: %w", err)
	}

	u, err := s.loadUser(ctx, id)
	if err != nil {
		return TokenPair{}, err
	}
	return s.issueTokens(ctx, id, u)
}

// UpdateNickname 修改玩家昵称。
//
// 昵称会出现在排行榜、战报、防线候选与后台用户列表里，是**玩家可控**且
// 会被广泛读取的字段（上限见 MaxNicknameLen 的注释）。修改只落一行主键更新，
// 排行榜缓存（TTL 15s）短暂显示旧名，可接受。
func (s *Service) UpdateNickname(ctx context.Context, userID int64, nickname string) error {
	nickname = strings.TrimSpace(nickname)
	if nickname == "" {
		return fmt.Errorf("%w: 昵称不能为空", ErrBadInput)
	}
	if utf8.RuneCountInString(nickname) > MaxNicknameLen {
		return fmt.Errorf("%w: 昵称最长 %d 个字符", ErrBadInput, MaxNicknameLen)
	}
	ct, err := s.pool.Exec(ctx,
		`UPDATE users SET nickname = $2 WHERE id = $1`, userID, nickname)
	if err != nil {
		return fmt.Errorf("update nickname: %w", err)
	}
	if ct.RowsAffected() == 0 {
		return fmt.Errorf("%w: 用户不存在", ErrNotFound)
	}
	return nil
}

// UpgradeGuestAccount 把游客账号升级为「用户名 + 密码」账号。
//
// 游客号的全部数据（钱包 / 进度 / 专精 / 战报 / 防线）只靠一枚存在
// localStorage 的 `guest_token` 保护 —— 清缓存 / 换设备 / 浏览器隐私模式
// 一关，这枚凭证就没了，账号等于永久丢失。升级后玩家可以在任何设备
// 上用用户名 + 密码登录回**同一个号**。
//
// 语义要点：
//
//   - 仅限纯游客号（`password_hash IS NULL`）。已经是账号登录的行走这里
//     没有意义，直接拒绝。
//   - 用户名受 `idx_users_username` 部分唯一索引约束（NULL 不冲突）。
//     先查后写，把「用户名已被占用」翻译成 ErrBadInput，
//     而不是让 pgx 的唯一约束错误冒到 500。
//   - `guest_token` **保留不动**：老客户端冷启动仍会带旧凭证调 GuestLogin，
//     服务端按 guest_token 匹配回同一行 —— 转正不能把老客户端挤出局。
//   - 不吊销现有会话：身份没变（还是同一个 user_id），只是多了一把钥匙。
func (s *Service) UpgradeGuestAccount(ctx context.Context, userID int64, username, password string) error {
	username = strings.TrimSpace(username)
	password = strings.TrimSpace(password)
	if username == "" || password == "" {
		return fmt.Errorf("%w: 用户名和密码都不能为空", ErrBadInput)
	}
	if utf8.RuneCountInString(username) > maxUsernameLen {
		return fmt.Errorf("%w: 用户名最长 %d 个字符", ErrBadInput, maxUsernameLen)
	}
	if strings.ContainsAny(username, " \t\r\n") {
		return fmt.Errorf("%w: 用户名不能包含空白字符", ErrBadInput)
	}
	if len(password) < 10 {
		return fmt.Errorf("%w: 密码至少 10 位（当前 %d 位）", ErrBadInput, len(password))
	}

	var (
		hash   *string
		status int
	)
	err := s.pool.QueryRow(ctx,
		`SELECT password_hash, status FROM users WHERE id = $1`, userID).
		Scan(&hash, &status)
	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return fmt.Errorf("%w: 用户不存在", ErrNotFound)
		}
		return fmt.Errorf("load player account: %w", err)
	}
	if status != 1 {
		return fmt.Errorf("%w: 账号已被封禁", ErrForbidden)
	}
	if hash != nil && *hash != "" {
		return fmt.Errorf("%w: 该账号已经是账号登录，不能重复注册", ErrBadInput)
	}

	// 用户名占用检查（先查后写，避免把唯一约束错误暴露成 500）。
	var ownerID int64
	err = s.pool.QueryRow(ctx,
		`SELECT id FROM users WHERE username = $1`, username).Scan(&ownerID)
	switch {
	case err == nil:
		return fmt.Errorf("%w: 用户名已被使用", ErrBadInput)
	case !errors.Is(err, pgx.ErrNoRows):
		return fmt.Errorf("check username: %w", err)
	}

	newHash, err := HashPassword(password)
	if err != nil {
		return err
	}

	// 保持 guest_token 不变（见函数头注释）。
	if _, err := s.pool.Exec(ctx,
		`UPDATE users SET username = $2, password_hash = $3, is_guest = FALSE
		 WHERE id = $1`, userID, username, newHash); err != nil {
		return fmt.Errorf("upgrade guest account: %w", err)
	}
	return nil
}

// ChangePassword 修改玩家账号密码，并吊销该账号全部未过期 refresh token。
//
// 返回吊销的 refresh token 条数。语义要点：
//
//   - 原密码必须正确。错误与「输入非法」同族（ErrBadInput），
//     而不是 ErrUnauthorized —— 请求方已经持有效 access token 通过鉴权，
//     再用 401 会把客户端踢回登录页，体验上是「改个密码被登出」。
//   - 新密码门槛与其他引导账号一致（至少 10 位），防止绕过引导规则注册弱口令。
//   - 修改与吊销必须在**同一事务**里：中途失败不能出现
//     「密码已改但旧 refresh token 仍有效」的中间态。
//   - 游客号（password_hash 为 NULL）没有密码可改，直接拒绝。
func (s *Service) ChangePassword(ctx context.Context, userID int64, oldPassword, newPassword string) (int64, error) {
	oldPassword = strings.TrimSpace(oldPassword)
	newPassword = strings.TrimSpace(newPassword)
	if oldPassword == "" || newPassword == "" {
		return 0, fmt.Errorf("%w: 原密码和新密码都不能为空", ErrBadInput)
	}
	if len(newPassword) < 10 {
		return 0, fmt.Errorf("%w: 新密码至少 10 位（当前 %d 位）", ErrBadInput, len(newPassword))
	}
	if oldPassword == newPassword {
		return 0, fmt.Errorf("%w: 新密码不能与原密码相同", ErrBadInput)
	}

	var (
		hash   *string
		status int
	)
	err := s.pool.QueryRow(ctx,
		`SELECT password_hash, status FROM users WHERE id = $1`, userID).
		Scan(&hash, &status)
	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return 0, fmt.Errorf("%w: 用户不存在", ErrNotFound)
		}
		return 0, fmt.Errorf("load player account: %w", err)
	}
	if hash == nil || *hash == "" {
		return 0, fmt.Errorf("%w: 该账号没有设置密码，无法修改", ErrBadInput)
	}
	if status != 1 {
		return 0, fmt.Errorf("%w: 账号已被封禁", ErrForbidden)
	}
	if err := VerifyPassword(*hash, oldPassword); err != nil {
		return 0, fmt.Errorf("%w: 原密码不正确", ErrBadInput)
	}

	newHash, err := HashPassword(newPassword)
	if err != nil {
		return 0, err
	}

	tx, err := s.pool.Begin(ctx)
	if err != nil {
		return 0, fmt.Errorf("begin change password: %w", err)
	}
	defer tx.Rollback(ctx) //nolint:errcheck // 提交后回滚是无害的 ErrTxClosed

	if _, err := tx.Exec(ctx,
		`UPDATE users SET password_hash = $2 WHERE id = $1`, userID, newHash); err != nil {
		return 0, fmt.Errorf("update password hash: %w", err)
	}
	ct, err := tx.Exec(ctx,
		`UPDATE refresh_tokens SET revoked_at = now()
		  WHERE user_id = $1 AND revoked_at IS NULL`, userID)
	if err != nil {
		return 0, fmt.Errorf("revoke refresh tokens: %w", err)
	}
	if err := tx.Commit(ctx); err != nil {
		return 0, fmt.Errorf("commit change password: %w", err)
	}
	return ct.RowsAffected(), nil
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
