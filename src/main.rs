use std::path::PathBuf;
use kalosm::language::*;

#[tokio::main]
async fn main() {
    let documents = FsDocument::try_from(PathBuf::from("./publication-app-manual.pdf"))
        .unwrap()
        .into_document()
        .await
        .unwrap();

    println!("document: {:?}", documents);
}
