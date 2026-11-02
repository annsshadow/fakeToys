// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 第 66 轮：浏览量「内存聚合 + 定期 flush」行为测试。

#[cfg(test)]
mod view_count_tests {
    use crate::ViewCountBuffer;
    use shared::testing::{is_db_available, mock_pool, test_pool};

    #[tokio::test]
    async fn record_accumulates_and_flush_upserts_to_count_table() {
        if !is_db_available().await {
            eprintln!("skip: no database");
            return;
        }
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        let doc = format!("r66-doc-{}", std::process::id());
        client
            .execute(
                "INSERT INTO x_cms_document (id) VALUES ($1) ON CONFLICT (id) DO NOTHING",
                &[&doc],
            )
            .await
            .unwrap();
        client
            .execute(
                "DELETE FROM x_cms_document_view_count WHERE doc_id = $1",
                &[&doc],
            )
            .await
            .unwrap();

        let buffer = ViewCountBuffer::new(pool.clone());
        assert_eq!(buffer.record(&doc), 1);
        assert_eq!(buffer.record(&doc), 2);
        assert_eq!(buffer.record(&doc), 3);

        buffer.flush().await;
        let v: i32 = client
            .query_opt(
                "SELECT view_count FROM x_cms_document_view_count WHERE doc_id = $1",
                &[&doc],
            )
            .await
            .unwrap()
            .unwrap()
            .get(0);
        assert_eq!(v, 3, "3 次 record 后 flush 应落 3");

        // pending 已清：再 record 从 1 起算，flush 后累计 4
        assert_eq!(buffer.record(&doc), 1);
        buffer.flush().await;
        let v: i32 = client
            .query_opt(
                "SELECT view_count FROM x_cms_document_view_count WHERE doc_id = $1",
                &[&doc],
            )
            .await
            .unwrap()
            .unwrap()
            .get(0);
        assert_eq!(v, 4, "flush 必须累加而非覆盖");

        client
            .execute(
                "DELETE FROM x_cms_document_view_count WHERE doc_id = $1",
                &[&doc],
            )
            .await
            .unwrap();
        client
            .execute("DELETE FROM x_cms_document WHERE id = $1", &[&doc])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn flush_failure_puts_counts_back_pending() {
        // mock_pool 连不上：flush 不得丢已记录的浏览量（放回 pending 等下轮）。
        let buffer = ViewCountBuffer::new(mock_pool());
        buffer.record("doc-x");
        buffer.flush().await;
        assert_eq!(
            buffer.record("doc-x"),
            2,
            "flush 失败后 pending 必须保留（第二次 record 应得 2）"
        );
    }
}
