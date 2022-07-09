use oxigraph::store::Store;
use oxigraph::model::*;
use oxigraph::io::GraphFormat;
use oxigraph::sparql::QueryResults;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use oxiuselib::extendedTree::vocab::{owl, rdf, rdfs};


fn main() {
    load_dataset_all("F:/oxiuse/dataset/test.nt", "F:/oxiuse/dataset/test.nt");
}

fn load_dataset_all(dataset_path: &str, tree_path: &'static str) {
    let store = Store::new().unwrap();

    let mut str_buf = String::new();
    File::open(dataset_path).unwrap().read_to_string(&mut str_buf);

    store.bulk_loader().load_graph_oxiuse_value(str_buf.as_bytes().as_ref(), GraphFormat::NTriples, GraphNameRef::DefaultGraph, None, tree_path).unwrap();

    // 直接查询
    let ex = NamedNodeRef::new("http://www.w3.org/2000/01/rdf-schema#subPropertyOf").unwrap();

    // let results: Result<Vec<Quad>,_> = store.quads_for_pattern(Option::Some((ex.into())), None, None, None).collect();
    let results: Result<Vec<Quad>,_> = store.quads_for_pattern(None, None, None, None).collect();

    if let Ok(vec) = results {
        for quad in vec {
            println!("{}", quad);
        }
    };
}
