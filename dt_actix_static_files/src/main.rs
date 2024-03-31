// start serve individual file

// use actix_files::NamedFile;
// use actix_web::{web, App, HttpRequest, HttpServer, Result};
// use std::path::PathBuf;

// async fn index(req: HttpRequest) -> Result<NamedFile> {
//     let path: PathBuf = req.match_info().query("filename").parse().unwrap();
//     Ok(NamedFile::open(path)?)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
//         .bind(("127.0.0.1", 8099))?
//         .run()
//         .await
// }

// end serve individual file

// start serve individual file with regex and sanitized path
use actix_files::NamedFile;
use actix_web::{error, web, App, HttpRequest, HttpServer, Result};
use regex::Regex;
use std::io;
use std::path::{Path, PathBuf};

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let filename: PathBuf = req.match_info().query("filename").parse().unwrap();

    let filename_str = filename.to_string_lossy();
    let filename_regex = Regex::new(r"^[\w\./_]+$").unwrap();

    if !filename_regex.is_match(&filename_str) {
        return Err(error::ErrorNotFound("invalid filename"));
    }

    let sanitized_filename = sanitize_filename::sanitize(&filename_str);

    let base_dir = "./static";
    let path = Path::new(&base_dir).join(&sanitized_filename);

    if !path.starts_with(&base_dir) {
        return Err(error::ErrorNotFound("invalid file path"));
    }

    match NamedFile::open(path) {
        Ok(file) => Ok(file),
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                Err(error::ErrorNotFound("file not found"))
            } else {
                Err(error::ErrorInternalServerError("internal server error"))
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
        .bind(("127.0.0.1", 8099))?
        .run()
        .await
}

// end serve individual file with regex and sanitized path
