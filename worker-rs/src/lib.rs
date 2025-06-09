use worker::*;
use scraper::Selector;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {    
    let content_type = match get_content_type(&req) {
        Some(content_type) => content_type,
        None => return Response::error("Missing content-type header", 400),
    };

    if !content_type.contains("application/json") {
        return Response::error("Invalid content type", 415);
    }

    let req_data = match req.method() {
        Method::Post => handle_post_req(req).await,
        Method::Get => handle_get_req(req),
        _ => return Response::error("Send a POST request with JSON!", 405),
    };

    let req_data = match req_data {
        Some(req_data) => req_data,
        None => return Response::error("Invalid JSON request or missing fields", 400),
    };
            
    let html_text = match get_html_text(&req_data.id).await {
        Some(html_data) => html_data,
        None => return Response::error("Failed to fetch RockGymPro data", 500),
    };

    let rgp_data_unparsed = match parse_html(&html_text) {
        Some(rgp_data_unparsed) => rgp_data_unparsed,
        None => return Response::error("Failed to read RockGymPro data", 500),
    };

    let rgp_data = match deserialize_rgp_data(req_data, &rgp_data_unparsed) {
        Some(rgp_data) => rgp_data,
        None => return Response::error("Failed to parse RockGymPro data", 500),
    };
    
    match json5::to_string(&rgp_data) {
        Ok(json_str) => Response::ok(json_str),
        Err(e) => Response::error(format!("Failed to serialise data: {}", e), 500),
    }
}

fn get_content_type(req: &Request) -> Option<String> {
    let content_type = match req.headers().get("content-type") {
        Ok(content_type) => match  content_type{
            Some(content_type) => content_type,
            None => "".to_string(),
        },
        Err(_) => return None,
    };
    
    Some(content_type)
}

#[derive(Deserialize)]
struct RequestBody {
    id: String,
    gym_id: Option<String>,
}

async fn handle_post_req(mut req: Request) -> Option<RequestBody> {
    match req.json::<RequestBody>().await {
        Ok(req_data) => Some(req_data),
        Err(_) => None,
    }
}

fn handle_get_req(req: Request) -> Option<RequestBody> {
    match req.query::<RequestBody>() {
        Ok(req_data) => Some(req_data),
        Err(_) => None,
    }
}

async fn get_html_text(rgp_id: &str) -> Option<String> {
    let base_url = "https://portal.rockgympro.com/";
    let html_url = format!("{}/portal/public/{}/occupancy", base_url, rgp_id);

    let html_text = match reqwest::get(html_url).await {
        Ok(body) => match body.text().await {
            Ok(html_text) => html_text,
            Err(_) => return None,
        },
        Err(_) => return None,
    };

    Some(html_text)
}

fn parse_html(html_text: &str) -> Option<String> {
    let document = scraper::Html::parse_document(html_text);

    let script_selector = Selector::parse("script").unwrap();

    for script in document.select(&script_selector) {
        let script_text = script.text().collect::<Vec<_>>().join("");

        if !script_text.contains("var data =") {
            continue;
        }

        let re = regex::Regex::new(
            r"(?s)var\s+data\s*?=\s*?(\{.*?\})\;"
        ).unwrap();

        // Group[0] is the full string
        if let Some(match_) = re.captures(&script_text) {
            return Some(match_[1].to_string());
        };
    }
    
    None
}

#[derive(Deserialize, Serialize, Clone)]
struct RGPData {
    capacity: u32,
    count: u32,
}

fn deserialize_rgp_data(req_data: RequestBody, rgp_data_unparsed: &str) -> Option<RGPData> {
    // The HTML response contains JavaScript, not real JSON
    // json5 is more lenient than serde which only accepts strict compliance
    let data: HashMap<String, RGPData> = match json5::from_str(&rgp_data_unparsed) {
        Ok(data) => data,
        Err(_) => return None,
    };

    // If the user specified a gym ID, we use that one
    if let Some(gym_id) = req_data.gym_id {
        if let Some(rgp_data) = data.get(&gym_id) {
            return Some(rgp_data.clone());
        }
    };
    
    // If the user didn't provide a gym ID, default to the first gym in the map
    if let Some((_key, val)) = data.iter().next() {
        return Some(val.clone());
    };

    // The hashmap is empty
    None
}
