// 该文件进行时间测试

use oxigraph::storage::numeric_encoder::StrHash;
use oxigraph::store::{Store, QuadIter};
use oxigraph::model::*;
use oxigraph::io::GraphFormat;
use oxigraph::sparql::QueryResults;
use std::borrow::Borrow;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use stopwatch::Stopwatch;
use std::sync::atomic::AtomicUsize;

use oxiuselib::extendedTree::{MultiTree};
use oxiuselib::extendedTree::vocab::{rdf,rdfs,owl,lubm};


fn main() {
    // let dataset = vec!["src/dataset/dbpedia.nt", "src/dataset/lubm25.nt", "src/dataset/lubm50.nt",
    // "src/dataset/lubm100.nt", "src/dataset/lubm150.nt"];


    let dataset = vec!["src/dataset/test_lubm.nt", "src/dataset/test_dbpedia.nt"];

    println!("############################# 时间代价：不进行语义编码 #############################");
    for ds in &dataset {
        let sw = Stopwatch::start_new();
        test_time_without_semantic(ds);
        println!("dataset: {} took {:.8} ms\n", ds, sw.elapsed_ms());
    }
    
    println!("############################# 时间代价：进行语义编码 #############################");
    for ds in dataset {
        let sw = Stopwatch::start_new();
        test_time_semantic(ds);
        println!("dataset: {} took {:.8} ms\n", ds, sw.elapsed_ms());
    }
}

fn test_time_without_semantic(dataset: &'static str) {
    let store = Store::new().unwrap();
    let mut str_buf = String::new();

    File::open(dataset).unwrap().read_to_string(&mut str_buf);

    store.bulk_loader().load_graph(str_buf.as_bytes().as_ref(), GraphFormat::NTriples, GraphNameRef::DefaultGraph, None).unwrap();
}

fn test_time_semantic(dataset: &'static str) {
    let store = Store::new().unwrap();
    let mut str_buf = String::new();
    File::open(dataset).unwrap().read_to_string(&mut str_buf);

    // construct_tree(dataset);
    store.bulk_loader().load_graph_oxiuse_value(str_buf.as_bytes().as_ref(), GraphFormat::NTriples, GraphNameRef::DefaultGraph, None, dataset.clone()).unwrap();
}


// 在其中进行树的预处理，不插入图（可以逐行读取数据集文件，完成插入，可查询）（去除了尖括号）
pub fn construct_tree(path: &str) -> Result<(MultiTree, MultiTree), ()>{
    if let Ok(lines) = read_lines(path) {
        let classTree = MultiTree::new(owl::OWL_CLASS);
        let propertyTree = MultiTree::new(rdf::PROPERTY); 

        for line in lines {
            if let Ok(triple) = line {
                let vec:Vec<&str> = triple.split(' ').collect();

                let p = &vec[1][1..vec[1].len()-1];
                if p == rdfs::SUB_CLASS_OF  || p == lubm::SUB_ORGANIZATION {
                    let s = &vec[0][1..vec[0].len()-1];
                    let o = &vec[2][1..vec[2].len()-1];
                    
                    classTree.insert(s, o);
                } else if p == rdfs::SUB_PROPERTY_OF {
                    let s = &vec[0][1..vec[0].len()-1];
                    let o = &vec[2][1..vec[2].len()-1];
                    
                    propertyTree.insert(s, o);
                }
            }      
        }   

        classTree.encode();
        propertyTree.encode();

        println!("{}", classTree.get_root().get_childs().borrow().len());

        return Ok((classTree, propertyTree))
    }
    Err(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}