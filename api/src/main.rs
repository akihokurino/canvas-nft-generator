mod graph;

use actix_web::web::Data;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use app::{di, ethereum};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use std::env;
use std::str::FromStr;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::aws::ssm::may_load_dotenv()
        .await
        .expect("failed to load ssm parameter store");

    let port = env::var("PORT").unwrap_or("8000".to_string());
    let with_lambda: bool = env::var("WITH_LAMBDA")
        .map(|v| bool::from_str(&v).expect("failed to parse WITH_LAMBDA"))
        .unwrap_or(false);
    let with_playground = env::var("WITH_PLAYGROUND")
        .map(|v| bool::from_str(&v).expect("failed to parse WITH_PLAYGROUND"))
        .unwrap_or(true);

    let handler = graph::HttpHandler::new().await;
    let my_wallet = di::MY_WALLET.get().await.clone();

    let app_factory = move || {
        let mut app = App::new()
            .app_data(Data::new(handler.clone()))
            .app_data(Data::new(my_wallet.clone()))
            .service(
                web::scope("/graphql")
                    .service(web::resource("").guard(guard::Post()).to(graphql_route)),
            );

        if with_playground {
            app = app.service(
                web::scope("/playground")
                    .service(web::resource("").guard(guard::Get()).to(playground_route)),
            );
        }

        app
    };

    if with_lambda {
        lambda_web::run_actix_on_lambda(app_factory)
            .await
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
    } else {
        println!("listen as http server on port {}", port);
        HttpServer::new(app_factory)
            .bind(format!("127.0.0.1:{}", port))?
            .run()
            .await
    }
}

async fn graphql_route(
    handler: Data<graph::HttpHandler>,
    my_wallet: Data<ethereum::MyWallet>,
    http_req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    handler
        .handle(http_req, gql_req, my_wallet.as_ref().clone())
        .await
}

async fn playground_route() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}
