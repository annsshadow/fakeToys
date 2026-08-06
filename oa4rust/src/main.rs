use anyhow::Context as _;
use axum::middleware;
use axum::Router;
use shared::db::create_pool;
use shared::middleware::{
    auth_middleware, authorize_middleware, cors_middleware, rate_limit_middleware, security_headers_middleware,
    trace_middleware, SecurityState,
};
use shared::rate_limit::RateLimiter;
use shared::session::SessionManager;
use tracing_subscriber::EnvFilter;
use express;
use message;
use portal;
use bbs;
use calendar;
use component;
use file;
use ai;
use attendance;
use correlation;
use general;
use hotpic;
use jpush;
use meeting;
use mind;
use cms_express;
use cms_assemble_control;
use process_express;
use query_express;
use process_designer;
use program_center;
use base;
use query_service;
use process_bam;
use process_surface;
use file_assemble_control;
use ai_assemble_control;
use hotpic_assemble_control;
use organization_assemble_express;
use organization_assemble_control;
use mind_assemble_control;
use attendance_assemble_control;
use general_assemble_control;
use meeting_assemble_control;
use message_assemble_communicate;
use portal_assemble_designer;
use correlation_service_processing;
use portal_assemble_surface;
use processplatform_service_processing;
use bbs_assemble_control;
use calendar_assemble_control;
use component_assemble_control;
use jpush_assemble_control;
use processplatform_core_entity;
use portal_core_entity;
use program_center_core_entity;
use processplatform_core_express;
use query_core_entity;
use general_core_entity;
use organization_core_entity;
use cms_core_entity;
use query_assemble_designer;
use query_assemble_surface;
use console;
use processplatform_assemble_surface;
use bbs_core_entity;
use calendar_core_entity;
use component_core_entity;
use file_core_entity;
use ai_core_entity;
use attendance_core_entity;
use cms_core_express;
use correlation_core_entity;
use correlation_core_express;
use hotpic_core_entity;
use jpush_core_entity;
use meeting_core_entity;
use message_core_entity;
use mind_core_entity;
use organization_core_express;
use processplatform_assemble_bam;
use processplatform_assemble_designer;
use query_core_express;
use query_service_processing;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("o2server=debug".parse()?))
        .init();

    dotenvy::dotenv().ok();

    let pool = create_pool().await.context("failed to create database pool")?;

    let session_manager = SessionManager::new();
    let rate_limiter = RateLimiter::new();

    // 认证 / 授权 / 限流中间件共享单一状态实例，避免状态分裂。
    let security_state = SecurityState {
        session_manager: session_manager.clone(),
        rate_limiter: rate_limiter.clone(),
        pool: pool.clone(),
    };

    // 6 个子 router 只提供业务路由，认证/限流/安全头统一在顶层挂载一次。
    let app = Router::new()
        .merge(shared::router::router())
        .merge(auth::router(pool.clone(), rate_limiter.clone(), session_manager.clone()))
        .merge(personal::router(pool.clone(), session_manager.clone()))
        .merge(cms_control::cms_control_router())
        .merge(control::control_router(pool.clone()))
        .merge(personal_extend::personal_extend_router(pool.clone(), session_manager))
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
        .merge(processplatform_service_processing::router(Some(pool.clone())))
        .merge(bbs_assemble_control::router(pool.clone()))
        .merge(calendar_assemble_control::router(pool.clone()))
        .merge(component_assemble_control::router(pool.clone()))
        .merge(jpush_assemble_control::router(pool.clone()))
        .merge(processplatform_core_entity::router(pool.clone()))
        .merge(portal_core_entity::router(pool.clone()))
        .merge(program_center_core_entity::router(pool.clone()))
        .merge(processplatform_core_express::router(pool.clone()))
        .merge(query_core_entity::router(pool.clone()))
        .merge(general_core_entity::router(pool.clone()))
        .merge(organization_core_entity::router(pool.clone()))
        .merge(cms_core_entity::router(pool.clone()))
        .merge(query_assemble_designer::router(pool.clone()))
        .merge(query_assemble_surface::router(pool.clone()))
        .merge(console::router(pool.clone()))
        .merge(processplatform_assemble_surface::router(Some(pool.clone())))
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
        .merge(processplatform_assemble_designer::router(Some(pool.clone())))
        .merge(query_core_express::router(pool.clone()))
        .merge(query_service_processing::router(pool.clone()));


    // 中间件执行顺序（请求流向）：trace → security_headers → cors → rate_limit → auth → authorize → handler。
    // axum 中后添加的 layer 包裹先添加的 layer，因此按反序添加。
    // 认证类端点（/jaxrs/authentication、/jaxrs/reset、/jaxrs/secret）统一由
    // rate_limit 中间件限流（10 次/分钟/IP），auth 各 handler 不再内置独立限流。
    let app = app
        .layer(middleware::from_fn_with_state(security_state.clone(), authorize_middleware))
        .layer(middleware::from_fn_with_state(security_state.clone(), auth_middleware))
        .layer(middleware::from_fn_with_state(security_state.clone(), rate_limit_middleware))
        .layer(cors_middleware())
        .layer(middleware::from_fn(security_headers_middleware))
        .layer(middleware::from_fn(trace_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    // 注入 ConnectInfo<SocketAddr>，供 client_ip 在不可信来源时回退 socket 地址
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
