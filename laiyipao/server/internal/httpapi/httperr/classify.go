// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httperr

import (
	"context"
	"errors"
	"io"
	"net"
	"strings"
	"syscall"

	"github.com/jackc/pgx/v5/pgconn"
	"github.com/jackc/puddle/v2"

	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/service"
)

// IsTransientDependency 判断错误是否属于「稍后重试可能成功」的依赖故障。
//
// 只收**已知的瞬时类别**，不做「所有非 nil 错误都当 503」的兜底：
// 后者会把 SQL 语法错误、约束冲突、扫描类型不符这些真 bug 伪装成
// 「服务暂时不可用」，客户端会无限重试，排障也会被误导。
//
// 分类依据（逐条都有 `sqlstate_test.go` 的真实触发用例）：
//   - 连接池已关闭（优雅停机/重建）→ puddle.ErrClosedPool；
//   - 连接级故障 → PostgreSQL class 08（connection exception）、
//     57P01/57P02/57P03（后台终止/崩溃/暂不可连）、53300（连接数满）、
//     53200（内存不足）；
//   - 语句被取消 → 57014（statement_timeout / 主动 cancel），
//     这也是本仓库 `DB_STATEMENT_TIMEOUT` 超时的实际错误码；
//   - 网络层 → *net.OpError、ECONNREFUSED/ECONNRESET/EPIPE、EOF
//     （数据库断连与微信等上游不可达都会落到这里）；
//   - 请求/加载超时 → context.DeadlineExceeded。
//
// 故意**不**包含 context.Canceled：那是客户端主动断开，响应已经无人接收，
// 归为 503 只会制造一条误导性的告警。
//
// 也**不**包含 23505（唯一约束冲突）与 42P01（表不存在）：
// 前者是业务重试/竞态的信号，后者是迁移没跑 —— 都是「重试没有意义」。
func IsTransientDependency(err error) bool {
	if err == nil {
		return false
	}
	if errors.Is(err, puddle.ErrClosedPool) ||
		errors.Is(err, context.DeadlineExceeded) ||
		errors.Is(err, io.EOF) || errors.Is(err, io.ErrUnexpectedEOF) ||
		errors.Is(err, syscall.ECONNREFUSED) ||
		errors.Is(err, syscall.ECONNRESET) ||
		errors.Is(err, syscall.EPIPE) {
		return true
	}
	var pgErr *pgconn.PgError
	if errors.As(err, &pgErr) {
		switch {
		case strings.HasPrefix(pgErr.Code, "08"):
			return true
		case pgErr.Code == "57014", pgErr.Code == "57P01", pgErr.Code == "57P02",
			pgErr.Code == "57P03", pgErr.Code == "53300", pgErr.Code == "53200":
			return true
		}
		return false
	}
	var netErr *net.OpError
	return errors.As(err, &netErr)
}

// IsSettleRejection 判断错误是否属于「结算被拒」类（422）。
//
// ⚠️ 这份清单必须与 domain 包的结算错误**同步**，否则新错误会掉进 500。
// 后果有两处，都很糟：
//  1. 客户端收到「服务内部错误」，无法区分"你作弊了"和"我们坏了"；
//  2. 服务端日志被大量假 500 刷满，真正的故障被淹没。
//
// `TestEverySettleSentinelIsClassified` 会把两边的差集直接打出来。
func IsSettleRejection(err error) bool {
	for _, target := range []error{
		// 凭证类
		domain.ErrTokenNotFound, domain.ErrTokenUsed, domain.ErrTokenExpired,
		domain.ErrLevelMismatch,
		// 数值越界类
		domain.ErrTooManyKills, domain.ErrTooShort, domain.ErrScoreRateExceeded,
		domain.ErrInvalidHitRate, domain.ErrWaveExceeded,
		// 上界校验类（2026-09-26 补齐，此前全部误返回 500）
		domain.ErrTooManyReactions, domain.ErrTooManyShots,
		domain.ErrTooManyLeaked, domain.ErrInvalidField,
		// HPLeft / slot-budget / replay_skills（第 58 轮补）。
		domain.ErrHPLeftExceedsBase,
		service.ErrSlotBudgetExceeded,
		service.ErrReplaySkillsMismatch,
	} {
		if errors.Is(err, target) {
			return true
		}
	}
	return false
}
