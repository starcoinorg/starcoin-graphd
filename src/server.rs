use actix_web::{web, App, HttpServer, post, Responder};
use anyhow::Result;
use crate::dag_graph::DagGraphBuilder;


#[post("/dag_view")]
async fn dag_view_handler(builder: web::Data<DagGraphBuilder>) -> Result<impl Responder, actix_web::Error> {
    let graph = builder.generate()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(graph))
}

pub async fn start_server(builder: DagGraphBuilder) -> Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(builder.clone()))
            .service(dag_view_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
