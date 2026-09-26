// Package migrations 把 SQL 迁移文件嵌入二进制，使服务端无需依赖外部文件即可自举数据库。
package migrations

import "embed"

// FS 包含本目录下全部 .sql 迁移文件，供 goose 执行。
//
//go:embed *.sql
var FS embed.FS
