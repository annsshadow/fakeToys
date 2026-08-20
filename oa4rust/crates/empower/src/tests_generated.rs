#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    // SKIPPED: create requires Session parameter
    // SKIPPED: get requires Session parameter
    // SKIPPED: update requires Session parameter
    // SKIPPED: delete requires Session parameter
    // SKIPPED: enable requires Session parameter
    // SKIPPED: disable requires Session parameter
    // SKIPPED: manager_create requires Session parameter
    // SKIPPED: manager_update requires Session parameter
    // SKIPPED: manager_delete requires Session parameter
    // SKIPPED: manager_list_paging requires Session parameter
    // SKIPPED: list_current_person requires Session parameter
    // SKIPPED: list_current_person_enable requires Session parameter
    // SKIPPED: list_to requires Session parameter
    // SKIPPED: list_to_enable requires Session parameter
}