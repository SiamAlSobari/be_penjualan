use crate::validation::auth::RegisterUser;
use actix_web::{HttpResponse, Responder, web};
use backend::{Response, ValidateErrItem, map_validation};
use sqlx::MySqlPool;
use uuid::Uuid;
use validator::Validate;

pub async fn register(user: web::Json<RegisterUser>, pool: web::Data<MySqlPool>) -> impl Responder {
    match user.validate() {
        Ok(_) => {
            let existing = sqlx::query!(
                "SELECT id,email FROM users WHERE email = ? LIMIT 1",
                user.email
            )
            .fetch_optional(pool.get_ref())
            .await;

            match existing {
                Ok(Some(u)) => {
                    eprint!("Email dengan {} telah digunakan ", u.email);
                    return HttpResponse::Conflict().json(Response::<()> {
                        status: "Failed".to_string(),
                        message: "Email sudah digunakan".to_string(),
                        data: None,
                    });
                }
                Ok(None) => {
                    let existing_user_name = sqlx::query!(
                        "SELECT user_name FROM profiles WHERE user_name = ? LIMIT 1",
                        user.user_name
                    )
                    .fetch_optional(pool.get_ref())
                    .await;

                    match existing_user_name {
                        Ok(Some(p)) => {
                            eprint!("{}", p.user_name);
                            return HttpResponse::Conflict().json(Response::<()> {
                                status: "Failed".to_string(),
                                message: "UserName sudah digunakan".to_string(),
                                data: None,
                            });
                        }
                        Ok(None) => {
                            let user_id = Uuid::new_v4().to_string();
                            let hashed_password =
                                bcrypt::hash(&user.password, 10).expect("hash err");

                            let _ = sqlx::query!(
                                "INSERT INTO users (id,email,hash_password) VALUES (?, ?, ?)",
                                user_id,
                                user.email,
                                hashed_password
                            )
                            .execute(pool.get_ref())
                            .await
                            .expect("Failed insert user");

                            let profile_id = Uuid::new_v4().to_string();

                            let _ = sqlx::query!(
                                "INSERT INTO profiles (id, user_name, user_id) VALUES (?, ?, ?)",
                                profile_id,
                                user.user_name,
                                user_id
                            )
                            .execute(pool.get_ref())
                            .await
                            .expect("Failed insert profile");

                            return HttpResponse::Ok().json(Response::<()> {
                                status: "Success".to_string(),
                                message: "User dan profile berhasil dibuat ✅".to_string(),
                                data: None,
                            });
                        }
                        Err(err) => {
                            return HttpResponse::InternalServerError().json(Response::<String> {
                                status: "Failed".to_string(),
                                message: "Database error".to_string(),
                                data: Some(format!("{}", err)),
                            });
                        }
                    }
                }
                Err(err) => {
                    return HttpResponse::InternalServerError().json(Response::<String> {
                        status: "Failed".to_string(),
                        message: "Database error".to_string(),
                        data: Some(format!("{}", err)),
                    });
                }
            }
        }
        Err(err) => {
            let errs = map_validation(err);
            return HttpResponse::BadRequest().json(Response::<Vec<ValidateErrItem>> {
                status: "Failed".to_string(),
                message: "Validation Error".to_string(),
                data: Some(errs),
            });
        }
    }
}

pub async fn login() -> impl Responder {
    //validate match
    //query db ke user apakah emailnya ada (usernya ada )
    //kalo ada verifikasi passwordnya
    //kalo bener masukan data user ke payload jwt lalu kirim ke cookie
    HttpResponse::Ok().json("a")
}
