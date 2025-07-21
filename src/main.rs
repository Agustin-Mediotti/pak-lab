use pak_lab::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
