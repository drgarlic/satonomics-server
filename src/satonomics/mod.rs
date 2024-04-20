use std::{collections::BTreeMap, fmt::Debug};

use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    response::Response,
    routing::get,
    Router,
};
use color_eyre::eyre::eyre;
use itertools::Itertools;
use parser::{
    DateMap, HeightMap, Json, Serialization, SerializedDateMap, SerializedHeightMap,
    HEIGHT_MAP_CHUNK_SIZE,
};
use reqwest::header::HOST;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{generic_to_reponse, AppState, Chunk, Source};

enum Kind {
    Date,
    Height,
    Last,
}

#[derive(Deserialize)]
pub struct SatonomicsQuery {
    chunk: Option<usize>,
}

impl Kind {
    pub fn from_str(string: &str) -> Self {
        match string {
            "date" => Self::Date,
            "height" => Self::Height,
            "value" | "last" | "" => Self::Last,
            _ => panic!(),
        }
    }
}

pub fn add_satonomics_routes(router: Router<AppState>) -> Router<AppState> {
    router.route("/*path", get(file_handler))
}

async fn file_handler(
    headers: HeaderMap,
    path: Path<String>,
    query: Query<SatonomicsQuery>,
) -> Response {
    _file_handler(headers, path, query).unwrap_or_default()
}

fn _file_handler(
    headers: HeaderMap,
    Path(path): Path<String>,
    query: Query<SatonomicsQuery>,
) -> color_eyre::Result<Response> {
    if path.contains("favicon") {
        return Err(eyre!("Don't support favicon"));
    }

    println!("fetch: {}", path);

    let sanitized = path.replace(['.', '/', '\\'], "");
    let transformed = sanitized.replace('-', "/");

    let mut kind = Kind::Last;
    let mut dataset_path = format_dataset_path(&format!("{transformed}/last.bin"));
    let mut type_name = None;
    let mut relative_path = format_relative_path(&dataset_path);
    let mut chunk = None;

    if sanitized.starts_with("date-to") || sanitized.starts_with("height-to") {
        let mut split = transformed.split('/');

        let kind_str = split.next().unwrap();

        kind = Kind::from_str(kind_str);

        // skip the "to"
        split.next();

        let joined_split = format!("{}/{}", split.join("/"), kind_str);

        let is_price_folder = joined_split.starts_with("close/");

        if is_price_folder {
            type_name = Some("f32".to_string());

            dataset_path = format!("price/{joined_split}");
        } else {
            dataset_path = format_dataset_path(&joined_split);
        }

        relative_path = format_relative_path(&dataset_path);

        let serialization = if is_price_folder {
            Serialization::Json
        } else {
            Serialization::Binary
        };

        let datasets = match kind {
            Kind::Date => DateMap::<usize>::_read_dir(&relative_path, &serialization),
            Kind::Height => HeightMap::<usize>::_read_dir(&relative_path, &serialization),
            _ => panic!(),
        };

        let (last_chunk_id, last_chunk_path) = datasets.last_key_value().unwrap();

        let mut chunk_id = query.chunk.unwrap_or(*last_chunk_id);

        relative_path = datasets
            .get(&chunk_id)
            .unwrap_or_else(|| {
                chunk_id = *last_chunk_id;
                last_chunk_path
            })
            .to_str()
            .unwrap()
            .to_string();

        let offset = match kind {
            Kind::Date => 1,
            Kind::Height => HEIGHT_MAP_CHUNK_SIZE,
            _ => panic!(),
        };

        let offsetted_to_url = |offseted| {
            datasets.get(&offseted).map(|_| {
                let host = headers[HOST].to_str().unwrap();
                let scheme = if host.contains("0.0.0.0") || host.contains("localhost") {
                    "http"
                } else {
                    "https"
                };

                format!("{scheme}://{host}/{sanitized}?chunk={offseted}",)
            })
        };

        chunk = Some(Chunk {
            id: chunk_id,
            next: chunk_id.checked_add(offset).and_then(offsetted_to_url),
            previous: chunk_id.checked_sub(offset).and_then(offsetted_to_url),
        })
    }

    if type_name.is_none() {
        let saved_path = format!("./{}", dataset_path);

        let path_to_type: BTreeMap<String, String> =
            Json::import("../parser/datasets/paths.json").unwrap();

        type_name = Some(
            path_to_type
                .get(&saved_path)
                .cloned()
                .unwrap_or_else(|| panic!("Fail for {saved_path}")),
        );
    }

    dbg!(&relative_path);

    let type_name = type_name.unwrap();

    let value = match type_name.as_str() {
        "u32" => typed_value_to_response::<u32>(kind, &relative_path, chunk)?,
        "u64" => typed_value_to_response::<u64>(kind, &relative_path, chunk)?,
        "usize" => typed_value_to_response::<usize>(kind, &relative_path, chunk)?,
        "f32" => typed_value_to_response::<f32>(kind, &relative_path, chunk)?,
        "f64" => typed_value_to_response::<f64>(kind, &relative_path, chunk)?,
        _ => panic!("Incompatible type: {type_name}"),
    };

    Ok(value)
}

fn format_dataset_path(query_path: &str) -> String {
    format!("datasets/{}", query_path)
}

fn format_relative_path(dataset_path: &str) -> String {
    format!("../parser/{}", dataset_path)
}

fn typed_value_to_response<T>(
    kind: Kind,
    relative_path: &str,
    chunk: Option<Chunk>,
) -> color_eyre::Result<Response>
where
    T: Serialize + Debug + DeserializeOwned + savefile::Deserialize + savefile::ReprC,
{
    Ok(match kind {
        Kind::Date => dataset_to_response(import_map::<T>(relative_path)?, chunk.unwrap()),
        Kind::Height => dataset_to_response(import_vec::<T>(relative_path)?, chunk.unwrap()),
        Kind::Last => value_to_response(import_value::<T>(relative_path)?),
    })
}

fn value_to_response<T>(value: T) -> Response
where
    T: Serialize,
{
    generic_to_reponse(value, None, None, Some(5))
}

fn dataset_to_response<T>(dataset: T, chunk: Chunk) -> Response
where
    T: Serialize,
{
    generic_to_reponse(
        dataset,
        Some(Source {
            name: "Satonomics".to_owned(),
            url: "https://satonomics.xyz".to_owned(),
            color: "#ffffff".to_owned(),
        }),
        Some(chunk),
        Some(10),
    )
}

fn import_map<T>(relative_path: &str) -> color_eyre::Result<SerializedDateMap<T>>
where
    T: Serialize + Debug + DeserializeOwned + savefile::Deserialize + savefile::ReprC,
{
    let x = Serialization::from_extension(relative_path.split('.').last().unwrap())
        .import(relative_path);
    dbg!(&x);
    x
}

fn import_vec<T>(relative_path: &str) -> color_eyre::Result<SerializedHeightMap<T>>
where
    T: Serialize + Debug + DeserializeOwned + savefile::Deserialize + savefile::ReprC,
{
    Serialization::from_extension(relative_path.split('.').last().unwrap()).import(relative_path)
}

fn import_value<T>(relative_path: &str) -> color_eyre::Result<T>
where
    T: Serialize + Debug + DeserializeOwned + savefile::Deserialize + savefile::ReprC,
{
    Serialization::from_extension(relative_path.split('.').last().unwrap())
        .import::<T>(relative_path)
}
