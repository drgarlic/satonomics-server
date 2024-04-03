use std::{collections::BTreeMap, fmt::Debug};

use axum::{extract::Path, response::Response, routing::get, Router};
use itertools::Itertools;
use serde::{de::DeserializeOwned, Serialize};

use crate::{json::Json, json_to_reponse, AppState, Binary, Source};

const DATE: &str = "date";
const HEIGHT: &str = "height";

pub fn add_satonomics_routes(router: Router<AppState>) -> Router<AppState> {
    router.route("/*path", get(file_handler))
}

async fn file_handler(Path(path): Path<String>) -> Response {
    if path.contains("favicon") {
        return Response::default();
    }

    println!("fetch: {}", path);

    let sanitized = path.replace('.', "");
    let sanitized = sanitized.replace('/', "");

    let mut split = sanitized.split('-');

    // Get if date or height
    let kind = split.next().unwrap();

    if kind != "date" && kind != "height" {
        return Response::default();
    }

    // pop the "to"
    split.next();

    let joined_split = split.join("/");

    if joined_split == "close" {
        let relative_path = format!("../explorer/price/{joined_split}/{kind}.json");

        return match kind {
            DATE => json_to_response(import_map::<f32>(&relative_path)),
            HEIGHT => json_to_response(import_vec::<f32>(&relative_path)),
            _ => panic!(),
        };
    }

    let end_of_path = format!("datasets/{}/{kind}.bin", joined_split);

    let relative_path = format!("../explorer/{}", end_of_path);
    let wrong_path = format!("./{}", end_of_path);

    let path_to_type: BTreeMap<String, String> =
        crate::json::Json::import("../explorer/datasets/paths.json").unwrap();

    let type_name = path_to_type
        .get(&wrong_path)
        .unwrap_or_else(|| panic!("Fail for {wrong_path}"));

    match kind {
        DATE => match type_name.as_str() {
            "u32" => json_to_response(import_map::<u32>(&relative_path)),
            "u64" => json_to_response(import_map::<u64>(&relative_path)),
            "usize" => json_to_response(import_map::<usize>(&relative_path)),
            "f32" => json_to_response(import_map::<f32>(&relative_path)),
            "f64" => json_to_response(import_map::<f64>(&relative_path)),
            _ => panic!(),
        },
        HEIGHT => match type_name.as_str() {
            "u32" => json_to_response(import_vec::<u32>(&relative_path)),
            "u64" => json_to_response(import_vec::<u64>(&relative_path)),
            "usize" => json_to_response(import_vec::<usize>(&relative_path)),
            "f32" => json_to_response(import_vec::<f32>(&relative_path)),
            "f64" => json_to_response(import_vec::<f64>(&relative_path)),
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn json_to_response<T>(value: T) -> Response
where
    T: Serialize,
{
    json_to_reponse(
        value,
        Source {
            name: "Satonomics".to_owned(),
            url: "https://satonomics.xyz".to_owned(),
            color: "#ffffff".to_owned(),
        },
        Some(10),
    )
}

fn import_map<T>(relative_path: &str) -> BTreeMap<String, T>
where
    T: Serialize + Debug + DeserializeOwned,
{
    match relative_path.split('.').last() {
        Some("json") => Json::import::<BTreeMap<String, T>, _>(&relative_path).unwrap(),
        Some("bin") => Binary::import::<BTreeMap<String, T>, _>(&relative_path).unwrap(),
        _ => unreachable!("Shouldn't end here"),
    }
}

fn import_vec<T>(relative_path: &str) -> Vec<T>
where
    T: Serialize + Debug + DeserializeOwned,
{
    match relative_path.split('.').last() {
        Some("json") => Json::import::<Vec<T>, _>(&relative_path).unwrap(),
        Some("bin") => Binary::import::<Vec<T>, _>(&relative_path).unwrap(),
        _ => unreachable!("Shouldn't end here"),
    }
}
