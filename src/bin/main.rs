use axum::{
    routing::{delete, get, post, put},
    Router,
};
use seagull::{api, model::db};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    init().await;
    serve().await;
}

fn get_listen_port() -> u16 {
    let port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT must be set");
    port.parse().expect("LISTEN_PORT must be a number")
}

async fn init() {
    dotenvy::dotenv().ok();

    let file_appender = tracing_appender::rolling::daily("log", "seagull.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .init();
    tracing::info!("log init finished");

    db::init().await.expect("Failed to connect to database");
}

async fn serve() {
    let addr = SocketAddr::from(([0, 0, 0, 0], get_listen_port()));
    println!("listening on {}", addr);

    let app = Router::new()
        // 管理员注册
        .route("/merchant/admin", post(api::admin::register))
        // 管理员登陆
        .route("/merchant/admin/login", post(api::admin::login))
        // 添加店铺
        .route("/merchant/shop", post(api::shop::add))
        // 修改店铺
        .route("/merchant/shop", post(api::shop::update))
        // 添加菜品
        .route("/merchant/dish", post(api::dish::add))
        // 修改菜品
        .route("/merchant/dish", put(api::dish::update))
        // 删除菜品
        .route("/merchant/dish", delete(api::dish::delete))
        // 添加菜品类型
        .route("/merchant/dishtype", post(api::dish_type::add))
        // 修改菜品类型
        .route("/merchant/dishtype", put(api::dish_type::update))
        // 删除菜品类型
        .route("/merchant/dishtype", delete(api::dish_type::delete))
        // 获取优惠券
        .route("/merchant/coupon", get(api::coupon::get))
        // 添加优惠券
        .route("/merchant/coupon", post(api::coupon::add))
        // 修改优惠券
        .route("/merchant/coupon", put(api::coupon::update))
        // 删除优惠券
        .route("/merchant/coupon", delete(api::coupon::delete))
        // 用户注册
        .route("/user", post(api::user::register))
        // 用户登陆
        .route("/user/login", post(api::user::login))
        // 用户登出
        .route("/user/logout", post(api::user::logout))
        // 获取菜品列表
        .route("/user/dish", get(api::dish::get))
        // 加入购物车
        .route("/user/cart", post(api::cart::add))
        // 支付
        .route("/user/pay", post(api::pay::pay));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
