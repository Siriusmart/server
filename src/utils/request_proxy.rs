use actix_web::{get, http::header::ContentType, web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize)]
pub struct Query {
    pub url: String,
    pub r#type: Option<String>,
    pub label: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    pub r#type: Option<String>,
    pub label: Option<String>,
    pub content: String,
}

#[get("/normal")]
async fn normal(query: web::Query<Query>) -> Result<String, Box<dyn Error>> {
    let res = reqwest::get(&query.url).await?;
    let content = res.text().await?;
     
    Ok(content)
}

#[get("/html")]
async fn html(query: web::Query<Query>) -> Result<HttpResponse, Box<dyn Error>> {

    let res = reqwest::get(&query.url).await?;
    let content = res.text().await?;
    
    let res = Response {
        r#type: query.r#type.clone(),
        label: query.label.clone(),
        content,
    };

    let html = format!(
        r#"
<!DOCTYPE html>
<html>
<script>
window.parent.postMessage({}, '*');
</script>
</html>
"#, serde_json::to_string(&res)?);
    
    Ok(
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(html)
        )
}

pub fn scope() -> Scope {
    web::scope("/request-proxy").service(normal).service(html)
}
