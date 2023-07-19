use reqwest::{header::{ HeaderMap, HeaderName,  HeaderValue }, Method, Client, Response};
use serde::Deserialize;
use clap::Parser;
use toml::Value;
use std::{fs::{self}, error::Error, path::PathBuf, env, collections::HashMap, str::FromStr,};



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "toml请求文件相对路径" )]
    path: String
}



#[derive(Deserialize, Debug)]
struct TomlTemplate {
    url: UrlTemplate,
    body: Option<HashMap<String, Value>>,
    params: Option<HashMap<String, Value>>,
    header: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
enum MyMethod{
    GET,
    POST,
    PUT,
    DELETE,
    PATCH
}


#[derive(Deserialize, Debug)]
struct UrlTemplate {
    localhost: String,
    method: MyMethod,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let arg= Args::parse();
    let root = env::current_dir()?;
    let toml_text = get_file(&join_dir_path(&root, &arg.path).unwrap())?;
    let toml: TomlTemplate =  cover_toml(&toml_text)?;
   
    request_process(&toml).await?;
    Ok(())
}

fn cover_toml(toml_text: &str) -> Result<TomlTemplate, Box<dyn Error>> {
    let cover_toml: TomlTemplate = toml::from_str(toml_text)?;
    Ok(cover_toml)
}

async fn request_process(toml: &TomlTemplate) -> Result<(), Box<dyn Error>> {
    let header_map = get_header();
    let response = request_clinet(&header_map, toml).await?;
    println!("{:?}", response.json::<Value>().await?);
    Ok(())
}


fn join_dir_path(path_buf: &PathBuf, path2: &str) -> Result<String, Box<String>> {
    match path_buf.join(path2).to_str() {
       Some(value) => {
        Ok(value.to_string())
       },
       None => {
        Err(Box::new("路径拼接错误".to_string()))
       }
    }
}

fn get_file(filepath: &str)  -> Result<String, Box<dyn Error>>{
    let current = fs::read_to_string(filepath)?;
    Ok(current)
}


async fn request_clinet(header_map: &Option<HashMap<String, String>>, toml: &TomlTemplate) -> Result<Response, Box<dyn Error>> {
    let header = insert_header(&toml.header,header_map);
    let client = reqwest::Client::builder().build()?;
    let response = match &toml.url.method {
        MyMethod::GET => {
            request(&client, Method::GET, toml, header).await?
        },
        MyMethod::POST => {
            request(&client, Method::POST, toml, header).await?
        },
        MyMethod::DELETE => {
            request(&client, Method::DELETE, toml, header).await?
        },
        MyMethod::PUT => {
            request(&client, Method::PUT, toml, header).await?
        }
        MyMethod::PATCH => {
            request(&client, Method::PATCH, toml, header).await?
        }
    };
    Ok(response)
}


fn insert_header(
    toml_header: &Option<HashMap<String, String>>, 
    origin_header: &Option<HashMap<String, String>>) -> HeaderMap {
    let mut header = HeaderMap::new();
    match origin_header {
        Some(header_map) => {
            for key in header_map.keys() {
                let value = header_map.get(key).unwrap();
                header.insert(HeaderName::from_str(&key).unwrap(), HeaderValue::from_str(value).unwrap());
            }
        },
        _ => {}
    }
    match toml_header {
        Some(header_map) => {
            for key in header_map.keys() {
                let value = header_map.get(key).unwrap();
                header.insert(HeaderName::from_str(&key).unwrap(), HeaderValue::from_str(value).unwrap());
            }
        },
        _ => {}
    }

    return header;
}

async fn request(client: &Client, method: Method, toml: &TomlTemplate, header: HeaderMap) -> Result<Response, Box<dyn Error>> {
    let mut r = client.request(method, &toml.url.localhost);
    match &toml.params {
        Some(params) => {
          r =  r.form(params);
        },
        _ => {}
    }
    match  &toml.body {
        Some(body) => {
           r = r.json(body);
        },
        _ => {}
    }

    let res = r.headers(header).send().await?;
    Ok(res)
 }

fn get_header() -> Option<HashMap<String, String>>{
    let root = env::current_dir().unwrap();

    let header_path = root.join("header.toml");

    let header_file = fs::read_to_string(header_path);

    match header_file {
        Ok(value) => {
            let header_map:HashMap<String, String> = toml::from_str(&value).expect("全局 header.toml 解析失败");
            Some(header_map)
        },
        Err(_) => {
            None
        }
    }
}