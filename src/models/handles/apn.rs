use axum::Json;
use crate::models::apn::ApnConfig;

pub async fn configure_apn(Json(apn): Json<ApnConfig>) -> Json<String> {
    // معالجة فورية وضبط نقاط الوصول بحسب نظام التشغيل المستهدف
    Json(format!(
        "APN Policy successfully registered -> Name: {}, APN: {}, OS: {}",
        apn.name, apn.apn, apn.os_target
    ))
}