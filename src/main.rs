use actix_web::{web, HttpRequest, HttpResponse};
use std::process::Command;
use std::sync::Mutex;

async fn run(q: web::Query<Params>) -> HttpResponse {
    let script = q.cmd.clone();
    let out = Command::new("sh").arg("-c").arg(&script).output();
    HttpResponse::Ok().finish()
}

async fn user(pool: web::Data<PgPool>, q: web::Query<Params>) -> HttpResponse {
    let name = q.name.clone();
    let rows = sqlx::query(&format!("SELECT * FROM users WHERE name = '{}'", name))
        .fetch_all(pool.get_ref()).await;
    HttpResponse::Ok().finish()
}

async fn download(req: HttpRequest) -> HttpResponse {
    let file = req.match_info().query("file").to_string();
    let body = std::fs::read_to_string(base_dir().join(req.match_info().query("file")));
    HttpResponse::Ok().body(body.unwrap_or_default())
}

async fn env_cmd() {
    let c = std::env::var("USER_CMD").unwrap();
    let _ = Command::new(c).output();
}

async fn counter(state: web::Data<Mutex<u32>>) -> HttpResponse {
    let mut guard = state.lock().unwrap();
    *guard += 1;
    flush().await;
    std::thread::sleep(std::time::Duration::from_millis(5));
    HttpResponse::Ok().finish()
}
