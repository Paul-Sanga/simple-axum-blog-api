use blog_api::App;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = dotenv!("PORT").parse::<u16>()
    .expect("PORT should be an integer");
    let app = App::new(port);
    app.run().await;
}
