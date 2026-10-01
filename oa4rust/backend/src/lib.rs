// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::Router;
use shared::rate_limit::RateLimiter;
use shared::session::SessionManager;
use shared::Pool;

pub use ai;
pub use ai_assemble_control;
pub use attendance;
pub use attendance_assemble_control;
pub use auth;
pub use base;
pub use bbs;
pub use bbs_assemble_control;
pub use calendar;
pub use calendar_assemble_control;
pub use cms_assemble_control;
pub use cms_core_entity;
pub use cms_core_express;
pub use component;
pub use component_assemble_control;
pub use console;
pub use control;
pub use correlation;
pub use correlation_core_express;
pub use correlation_service_processing;
pub use file;
pub use file_assemble_control;
pub use general;
pub use general_assemble_control;
pub use hotpic;
pub use hotpic_assemble_control;
pub use jpush;
pub use jpush_assemble_control;
pub use meeting;
pub use meeting_assemble_control;
pub use message;
pub use message_assemble_communicate;
pub use mind;
pub use mind_assemble_control;
pub use organization_assemble_express;
pub use organization_core_entity;
pub use organization_core_express;
pub use personal;
pub use portal;
pub use portal_assemble_designer;
pub use portal_assemble_surface;
pub use processplatform_assemble_bam;
pub use processplatform_assemble_designer;
pub use processplatform_assemble_surface;
pub use processplatform_core_entity;
pub use processplatform_core_express;
pub use processplatform_service_processing;
pub use program_center;
pub use program_center_core_entity;
pub use program_init;
pub use query_assemble_designer;
pub use query_assemble_surface;
pub use query_core_entity;
pub use query_core_express;
pub use query_service;
pub use realtime;
pub use search;

pub async fn create_app(
    pool: Pool,
    session_manager: SessionManager,
    rate_limiter: RateLimiter,
) -> anyhow::Result<Router> {
    let auth_config = shared::config::AuthConfig::from_env()?;
    let public_origin = auth_config.public_origin.clone();
    let mut session_manager = session_manager;
    session_manager.auth_config = auth_config.clone();
    let security_state = shared::middleware::SecurityState {
        session_manager: session_manager.clone(),
        rate_limiter: rate_limiter.clone(),
        pool: pool.clone(),
    };

    let app = Router::new()
        .merge(shared::router::router())
        .merge(auth::router(
            pool.clone(),
            rate_limiter.clone(),
            session_manager.clone(),
        ))
        .merge(personal::router(pool.clone(), session_manager.clone()))
        .merge(cms_control::cms_control_router(pool.clone()))
        .merge(control::control_router(pool.clone()))
        .merge(personal_extend::personal_extend_router(
            pool.clone(),
            session_manager,
        ))
        .merge(program_init::program_init_router(pool.clone()))
        .merge(express::router(pool.clone()))
        .merge(message::router(pool.clone()))
        .merge(portal::router(pool.clone()))
        .merge(bbs::router(pool.clone()))
        .merge(calendar::router(pool.clone()))
        .merge(component::router(pool.clone()))
        .merge(file::router(pool.clone()))
        .merge(ai::router(pool.clone()))
        .merge(attendance::router(pool.clone()))
        .merge(correlation::router(pool.clone()))
        .merge(general::router(pool.clone()))
        .merge(hotpic::router(pool.clone()))
        .merge(jpush::router(pool.clone()))
        .merge(meeting::router(pool.clone()))
        .merge(mind::router(pool.clone()))
        .merge(cms_express::router(pool.clone()))
        .merge(cms_assemble_control::router(pool.clone()))
        .merge(process_express::router(pool.clone()))
        .merge(query_express::router(pool.clone()))
        .merge(process_designer::router(pool.clone()))
        .merge(program_center::router(pool.clone()))
        .merge(base::router(pool.clone()))
        .merge(query_service::router(pool.clone()))
        .merge(process_bam::router(pool.clone()))
        .merge(process_surface::router(pool.clone()))
        .merge(file_assemble_control::router(pool.clone()))
        .merge(ai_assemble_control::router(pool.clone()))
        .merge(hotpic_assemble_control::router(pool.clone()))
        .merge(organization_assemble_express::router(pool.clone()))
        .merge(organization_assemble_control::router(pool.clone()))
        .merge(mind_assemble_control::router(pool.clone()))
        .merge(attendance_assemble_control::router(pool.clone()))
        .merge(general_assemble_control::router(pool.clone()))
        .merge(meeting_assemble_control::router(pool.clone()))
        .merge(message_assemble_communicate::router(pool.clone()))
        .merge(portal_assemble_designer::router(pool.clone()))
        .merge(correlation_service_processing::router(pool.clone()))
        .merge(portal_assemble_surface::router(pool.clone()))
        .merge(processplatform_service_processing::router(pool.clone()))
        .merge(bbs_assemble_control::router(pool.clone()))
        .merge(calendar_assemble_control::router(pool.clone()))
        .merge(component_assemble_control::router(pool.clone()))
        .merge(jpush_assemble_control::router(pool.clone()))
        .merge(processplatform_core_entity::router(pool.clone()))
        .merge(portal_core_entity::router(pool.clone()))
        .merge(program_center_core_entity::router(pool.clone()).await)
        .merge(processplatform_core_express::router(pool.clone()))
        .merge(query_core_entity::router(pool.clone()))
        .merge(general_core_entity::router(pool.clone()))
        .merge(organization_core_entity::router(pool.clone()))
        .merge(cms_core_entity::router(pool.clone()))
        .merge(query_assemble_designer::router(pool.clone()))
        .merge(query_assemble_surface::router(pool.clone()))
        .merge(search::router(pool.clone()))
        .merge(console::router(pool.clone()))
        .merge(processplatform_assemble_surface::router(pool.clone()))
        .merge(bbs_core_entity::router(pool.clone()))
        .merge(calendar_core_entity::router(pool.clone()))
        .merge(component_core_entity::router(pool.clone()))
        .merge(file_core_entity::router(pool.clone()))
        .merge(ai_core_entity::router(pool.clone()))
        .merge(attendance_core_entity::router(pool.clone()))
        .merge(cms_core_express::router(pool.clone()))
        .merge(correlation_core_entity::router(pool.clone()))
        .merge(correlation_core_express::router(pool.clone()))
        .merge(hotpic_core_entity::router(pool.clone()))
        .merge(jpush_core_entity::router(pool.clone()))
        .merge(meeting_core_entity::router(pool.clone()))
        .merge(message_core_entity::router(pool.clone()))
        .merge(mind_core_entity::router(pool.clone()))
        .merge(organization_core_express::router(pool.clone()))
        .merge(processplatform_assemble_bam::router(pool.clone()))
        .merge(processplatform_assemble_designer::router(pool.clone()))
        .merge(query_core_express::router(pool.clone()))
        .merge(query_service_processing::router(pool.clone()))
        // P5：IM 实时协议 WebSocket（realtime crate，自带 RealtimeManager state）
        .merge(realtime::ws_route());

    let app = app
        .layer(axum::middleware::from_fn_with_state(
            security_state.clone(),
            shared::middleware::authorize_middleware,
        ))
        .layer(axum::middleware::from_fn_with_state(
            security_state.clone(),
            shared::middleware::auth_middleware,
        ))
        .layer(axum::middleware::from_fn_with_state(
            security_state.clone(),
            shared::middleware::csrf_middleware,
        ))
        .layer(axum::middleware::from_fn_with_state(
            security_state.clone(),
            shared::middleware::rate_limit_middleware,
        ))
        .layer(shared::middleware::cors_middleware_for_origin(
            &public_origin,
        ))
        .layer(axum::middleware::from_fn(
            shared::middleware::security_headers_middleware,
        ))
        .layer(axum::middleware::from_fn(
            shared::middleware::trace_middleware,
        ));

    Ok(app)
}

#[cfg(test)]
mod cors_guard {
    use super::*;

    /// `create_app` 将 80 个 crate 的路由链式 `.merge()` 成一个巨型 axum Router，
    /// 构造过程需要约 64MB 栈（见 `.cargo/config.toml` 的说明）。
    ///
    /// 该文件的 `[target.x86_64-pc-windows-msvc]` 链接参数只对 **Windows 主线程**
    /// 生效；在 Linux/CI 上，`#[tokio::test]` 的 worker 线程走默认栈（约 2MB），
    /// 于是本用例必然栈溢出 -> panic -> `cargo test` 以 exit code 101 结束，
    /// 使 `cargo test --workspace --lib` 这个 job 在任何平台上都无法通过
    /// （CI 历史：该 job 从未成功过一次）。
    ///
    /// 因此这里不再依赖平台链接参数，而是在显式指定栈大小的线程上构造 Router，
    /// 让测试行为与平台无关。
    const CREATE_APP_STACK_SIZE: usize = 64 * 1024 * 1024;

    #[tokio::test]
    async fn create_app_builds_without_panic() {
        let pool = shared::testing::test_pool();
        let session_manager = shared::session::SessionManager::with_pool(pool.clone());
        let rate_limiter = shared::rate_limit::RateLimiter::new();

        // Build asserts: panics on route conflicts would surface here.
        // 在专用的大栈线程上构造，避免依赖平台特定的链接器 /STACK 参数。
        let handle = std::thread::Builder::new()
            .name("create_app_build".to_string())
            .stack_size(CREATE_APP_STACK_SIZE)
            .spawn(move || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("build tokio runtime for create_app")
                    .block_on(async move {
                        create_app(pool, session_manager, rate_limiter)
                            .await
                            .expect("unified create_app must build without panic")
                    })
            })
            .expect("spawn create_app build thread");

        let _app = handle.join().expect("create_app must build without panic");
    }
}
