// use actix_web::get;
//
// /// Returns all blacklisted IP's
// ///
// /// **Permissions**
// /// - rauthy_admin
// #[utoipa::path(
//     get,
//     path = "/blacklist",
//     tag = "blacklist",
//     responses(
//         (status = 200, description = "Ok", body = [Group]),
//         (status = 401, description = "Unauthorized", body = ErrorResponse),
//         (status = 403, description = "Forbidden", body = ErrorResponse),
//     ),
// )]
// #[get("/groups")]
// #[has_roles("rauthy_admin")]
// pub async fn get_groups(
//     data: web::Data<AppState>,
//     principal: web::ReqData<Option<Principal>>,
// ) -> Result<HttpResponse, ErrorResponse> {
//     let principal = Principal::from_req(principal)?;
//     principal.validate_rauthy_admin()?;
//
//     Group::find_all(&data)
//         .await
//         .map(|rls| HttpResponse::Ok().json(rls))
// }
