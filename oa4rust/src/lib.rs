use axum::Router;
use shared::Pool;
use shared::rate_limit::RateLimiter;
use shared::session::SessionManager;

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

pub async fn create_app(
    pool: Pool,
    session_manager: SessionManager,
    rate_limiter: RateLimiter,
) -> anyhow::Result<Router> {
    let security_state = shared::middleware::SecurityState {
        session_manager: session_manager.clone(),
        rate_limiter: rate_limiter.clone(),
        pool: pool.clone(),
    };

    let app = Router::new()
        .merge(shared::router::router())
        .merge(auth::router(pool.clone(), rate_limiter.clone(), session_manager.clone()))
        .merge(personal::router(pool.clone(), session_manager.clone()))
        .merge(cms_control::cms_control_router(pool.clone()))
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
        .merge(query_service_processing::router(pool.clone()));

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
            shared::middleware::rate_limit_middleware,
        ))
        .layer(shared::middleware::cors_middleware())
        .layer(axum::middleware::from_fn(shared::middleware::security_headers_middleware))
        .layer(axum::middleware::from_fn(shared::middleware::trace_middleware));

    Ok(app)
}
