// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// idempotency.go — R86：管理端写接口的幂等键（同事务查重/执行/记录）。
//
// # 探针结论（idempotency_probe_test，改前）
//
// 「同一请求打两次」确有重复后果的三条管理端写路径：
// /admin/users/:id/grant（余额 +200）、/admin/announcements（公告 ×2）、
// /admin/redeem-codes/batch（随机码 ×2）。本文件给它们接上
// `Idempotency-Key`；已天然幂等的（指定 code 建码 / 玩家兑换 /
// 任务领奖 / 结算）不动。
//
// # 语义契约
//
//   - 同 key + 同端点 + 同请求体（sha256）：24h 窗口内重放返回
//     **第一次的响应**（状态码与响应体），业务不再执行，审计不重写；
//   - 同 key 换请求体 → ErrIdemConflict（409 idempotency_conflict，
//     不能拿旧结果糊弄新请求）；
//   - 业务失败 → 幂等记录随事务一起回滚，重试干净重来
//     （先写幂等表再执行业务会留下「已处理」假记录，比重复更糟）。
//
// # 并发
//
// 并发同 key 走 PK 冲突的规范行为：ON CONFLICT 阻塞到对方事务结束，
// 对方提交 → 本方回滚业务并重查对方响应重放；对方回滚 → 本方插入成功。
// 没有「两个都执行」的窗口（00027 注释同口径）。

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"regexp"

	"github.com/jackc/pgx/v5"
)

// ErrIdemConflict 同 key 换了请求体（对外 409 idempotency_conflict）。
var ErrIdemConflict = errors.New("idempotency key 冲突：同键不同请求体")

// errIdemRace 并发同 key 的内部哨兵（外层据此重查对方响应）。
var errIdemRace = errors.New("idempotency race")

// IdemKeyPattern 是幂等键白名单（8~128；字符集覆盖 UUID/语义键的常见写法）。
// 超限**拒绝**（400），绝不截断 —— 截断会让两个不同的长 key 撞成一个。
var IdemKeyPattern = regexp.MustCompile(`^[A-Za-z0-9_\-.:]{8,128}$`)

// IdemWindow 是重放窗口（24h；理由见 00027）。
const IdemWindow = "24 hours"

// HashRequestBody 计算请求体的指纹（sha256 hex）。
func HashRequestBody(raw []byte) string {
	sum := sha256.Sum256(raw)
	return hex.EncodeToString(sum[:])
}

// RunIdempotent 在**同一个事务**内执行「查重 → 业务 → 记录」。
//
// fn 在事务内执行业务并返回响应对象（将被 json.Marshal 存储与回显）；
// 返回的 body 是序列化后的字节（第一次是新算，重放是库存原文）。
// replayed=true 表示本次是重放（fn 未执行）。
//
// key 为空时直接执行 fn（不走幂等）——但调用方约定：无 key 应走
// 原方法而不是这里（保持老路径零变化，R86 要求 6）。
func (s *Service) RunIdempotent(ctx context.Context, endpoint, key, reqHash string,
	fn func(tx pgx.Tx) (any, error)) (body []byte, replayed bool, err error) {

	var out []byte
	var replay bool
	err = s.DB.Tx(ctx, func(tx pgx.Tx) error {
		// 1) 查重（24h 窗口内）。
		var storedHash string
		var stored []byte
		qerr := tx.QueryRow(ctx, `
			SELECT request_hash, response FROM admin_idempotency_keys
			 WHERE endpoint = $1 AND key = $2
			   AND created_at > now() - $3::interval`,
			endpoint, key, IdemWindow).Scan(&storedHash, &stored)
		switch {
		case qerr == nil:
			if storedHash != reqHash {
				return ErrIdemConflict
			}
			out, replay = stored, true
			return nil
		case !errors.Is(qerr, pgx.ErrNoRows):
			return fmt.Errorf("idempotency lookup: %w", qerr)
		}

		// 1.5) 窗口外同 key 的残留行必须先删掉（R135 反向用例实证）。
		// 否则 INSERT 撞 PK → errIdemRace → **已执行完的业务被整体回滚**，
		// 再拿窗口外的旧响应重放给客户端：看起来成功，实际什么都没发生。
		// 清理是「写入时顺带」的，过期行未必已被清掉，这里要自己兜住。
		if _, derr := tx.Exec(ctx, `
			DELETE FROM admin_idempotency_keys
			 WHERE endpoint = $1 AND key = $2
			   AND created_at <= now() - $3::interval`,
			endpoint, key, IdemWindow); derr != nil {
			return fmt.Errorf("idempotency expire: %w", derr)
		}

		// 2) 执行业务（失败 → 整体回滚，不留幂等记录）。
		resp, err := fn(tx)
		if err != nil {
			return err
		}
		raw, err := json.Marshal(resp)
		if err != nil {
			return fmt.Errorf("idempotency response marshal: %w", err)
		}

		// 3) 同事务记录；并发同 key → ON CONFLICT 阻塞至对方结束后
		//    RowsAffected=0（对方已提交）→ 哨兵让外层重查。
		tag, err := tx.Exec(ctx, `
			INSERT INTO admin_idempotency_keys (endpoint, key, request_hash, status, response)
			VALUES ($1,$2,$3,200,$4)
			ON CONFLICT (endpoint, key) DO NOTHING`,
			endpoint, key, reqHash, raw)
		if err != nil {
			return fmt.Errorf("idempotency insert: %w", err)
		}
		if tag.RowsAffected() == 0 {
			return errIdemRace
		}

		// 4) 写入时顺带清过期行（单表体积小，无需后台任务；LIMIT 防长锁）。
		if _, err := tx.Exec(ctx, `
			DELETE FROM admin_idempotency_keys
			 WHERE ctid IN (SELECT ctid FROM admin_idempotency_keys
			                 WHERE created_at < now() - $1::interval LIMIT 100)`,
			IdemWindow); err != nil {
			return fmt.Errorf("idempotency cleanup: %w", err)
		}
		out = raw
		return nil
	})
	if errors.Is(err, errIdemRace) {
		// 对方已提交：重查它的响应重放（READ COMMITTED 下现已可见）。
		// 若对方在我们重查前恰好也回滚（极端：对方后续步骤失败），
		// 则退化为「客户端重试」——如实上抛，调用方会拿到错误而不是假结果。
		var stored []byte
		qerr := s.pool.QueryRow(ctx, `
			SELECT response FROM admin_idempotency_keys
			 WHERE endpoint = $1 AND key = $2
			   AND created_at > now() - $3::interval`,
			endpoint, key, IdemWindow).Scan(&stored)
		if qerr != nil {
			return nil, false, fmt.Errorf("idempotency race retry: %w", qerr)
		}
		return stored, true, nil
	}
	if err != nil {
		return nil, false, err
	}
	return out, replay, nil
}
