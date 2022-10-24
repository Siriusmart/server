use actix_web::{get, http::header::ContentType, web, HttpResponse, Scope};
use serde_json::{Map, Value};
use std::error::Error;

use crate::api::structs::errors::BlankError;

async fn process<'a>(
    query: &'a mut web::Query<Value>,
) -> Result<&mut Map<String, Value>, Box<dyn Error>> {
    let value = query.as_object_mut().unwrap();
    match value.get("url") {
        Some(url) => {
            let res = reqwest::get(url.as_str().unwrap()).await?;
            let content = res.text().await?;

            value.remove("url");
            value.insert(String::from("content"), Value::String(content));
        }
        None => return Err(BlankError.into()),
    }

    Ok(value)
}

#[get("/normal")]
async fn normal(mut query: web::Query<Value>) -> Result<HttpResponse, Box<dyn Error>> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(process(&mut query).await?)?))
}

#[get("/html")]
async fn html(mut query: web::Query<Value>) -> Result<HttpResponse, Box<dyn Error>> {
    let map = process(&mut query).await?;
    let content =
        html_escape::encode_safe(map.get("content").unwrap().as_str().unwrap()).to_string();
    *map.get_mut("content").unwrap() = Value::String(content);

    let html = format!(
        r#"
<!DOCTYPE html>
<html>
<script>
window.parent.postMessage({}, '*');
</script>
</html>
"#,
        serde_json::to_string(&map)?
    );

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub fn scope() -> Scope {
    web::scope("/request-proxy").service(normal).service(html)
}
