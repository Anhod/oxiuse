// 此文件进行lubm数据的查询（原生查询和推理查询）

use oxigraph::storage::StorageReader;
use oxigraph::storage::numeric_encoder::StrHash;
use oxigraph::store::{Store, QuadIter};
use oxigraph::model::*;
use oxigraph::io::GraphFormat;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use stopwatch::Stopwatch;

use oxiuselib::extendedTree::{MultiTree};
use oxiuselib::extendedTree::vocab::{rdf,rdfs,owl,lubm};


#[derive(Hash, PartialEq, Eq)]
struct Triple {
    subject: String,
    predicate: String,
    object: String
}

impl Triple {
    fn new(subject: String, predicate: String, object: String) -> Self {
        Triple { subject, predicate, object }
    }
}


fn main() {
    let dataset = vec!["src/dataset/lubm25.nt", "src/dataset/lubm50.nt",
                            "src/dataset/lubm100.nt", "src/dataset/lubm150.nt"];

    let dataset = vec!["src/dataset/lubm150.nt"];

    for ds in dataset {
        println!("lubm查询2(不推理): {}", ds);
        query(ds);

        println!("lubm查询2(推理): {}", ds);
        query_reason(ds);
    }
}

// 进行查询不推理
fn query(dataset: &str) {
    // 读取数据并进行编码
    let store = Store::new().unwrap();
    let mut str_buf = String::new();
    File::open(dataset).unwrap().read_to_string(&mut str_buf);

    store.bulk_loader().load_graph(str_buf.as_bytes().as_ref(), GraphFormat::NTriples, GraphNameRef::DefaultGraph, None).unwrap();

    let sw = Stopwatch::start_new();

    let s = NamedNode::new("http://www.Department0.University0.edu/FullProfessor4").unwrap();
    let results: QuadIter = store.quads_for_pattern(Some((&s).into()), None, None, None);
    let results: Result<Vec<Quad>,_> = results.collect();
    if let Ok(vec) = results {
        println!("三元组条数：{}", vec.len());
    }
    println!("dataset: {} took {:.8} ms\n", dataset, sw.elapsed_ms());
}

// 进行推理查询
fn query_reason(dataset: &str) {
    let store = Store::new().unwrap();
    let mut str_buf = String::new();
    File::open(dataset).unwrap().read_to_string(&mut str_buf);

    let classTree = construct_tree(dataset).unwrap();
    store.bulk_loader().load_graph(str_buf.as_bytes().as_ref(), GraphFormat::Turtle, GraphNameRef::DefaultGraph, None).unwrap();

    // 进行查询
    let sw = Stopwatch::start_new();

    let s = NamedNode::new("http://www.Department0.University0.edu/FullProfessor4").unwrap();
    let results: QuadIter = store.quads_for_pattern(Some((&s).into()), None, None, None);
    let quad_iter = results.reader.clone();

    let mut count = 0;

    let results: Result<Vec<Quad>,_> = results.collect();
    if let Ok(vec) = results {
        count = reason(store, quad_iter, vec.clone(), classTree.clone());
        println!("三元组条数：{}", count + vec.len());
    }

    println!("dataset: {} took {:.8} ms\n", dataset, sw.elapsed_ms());
}

// 谓词都是worksFor
fn reason(store: Store, quad_iter: StorageReader, vec_quad: Vec<Quad>, class_tree: MultiTree) -> usize{
    let mut all_quad:HashSet<Triple> = HashSet::new();

    for quad in vec_quad {
        let p = quad.predicate.as_str();

        let sub = {
            if let Subject::NamedNode(iri) = quad.subject {
                Ok(iri.clone())
            } else {
                Err(())
            }
        };

        let obj = {
            if let Term::NamedNode(iri) = quad.object {
                Ok(iri.clone())
            } else {
                Err(())
            }
        };

        
        match sub{
            Ok(s) => {
                match obj {
                    Ok(o) => {
                        // all_quad.insert(Triple::new(s.as_str().into(), p.into(), o.as_str().into()));

                        if p == lubm::WORKS_FOR {
                            let parent_way_vec = class_tree.get_parent_way_by_str(StrHash::new(o.as_str()));

                            match parent_way_vec {
                                Ok(parent_way) => {
                                    for way in parent_way {
                                    for parent_strhash in way {

                                        match quad_iter.get_str(&parent_strhash) {
                                            Ok(op) => {
                                                match op {
                                                    Some(parent) => {
                                                        if o.as_str() != parent.as_str() {
                                                            all_quad.insert(Triple::new(o.as_str().into(), lubm::SUB_ORGANIZATION.into(), parent.clone()));
                                                        }
                                                        all_quad.insert(Triple::new(s.as_str().into(), p.into(), parent));
                                                        
                                                    },
                                                    None => {}
                                                }
                                            },
                                            Err(_) => {}
                                        }
                                    }
                                }
                            },
                                Err(()) => ()
                                }
                            }
                        
                        // 加入domain、range信息，对s o分别查询type，然后分别加入domain range
                        let class_type = NamedNode::new(rdf::TYPE).unwrap();
                        let results: Result<Vec<Quad>, _> = store.quads_for_pattern(Some((&s).into()), Some((&class_type).into()), None, None).collect();
                        
                        if let Ok(vec) = results {
                            for quad in vec {
                                match quad.object {
                                    Term::NamedNode(node) => {
                                        all_quad.insert(Triple::new(s.as_str().into(), class_type.as_str().into(), node.as_str().into()));
                                        all_quad.insert(Triple::new(p.into(), rdfs::DOMAIN.into(), node.as_str().into()));
                                },
                                _ => {}
                                }
                            }
                        }
                        

                        let results: Result<Vec<Quad>, _> = store.quads_for_pattern(Some((&o).into()), Some((&class_type).into()), None, None).collect();
                        
                        if let Ok(vec) = results {
                            for quad in vec {
                                match quad.object {
                                    Term::NamedNode(node) => {
                                        all_quad.insert(Triple::new(o.as_str().into(), class_type.as_str().into(), node.as_str().into()));
                                        all_quad.insert(Triple::new(p.into(), rdfs::RANGE.into(), node.as_str().into()));
                                    },
                                    _ => {}
                                }
                            }
                        }
                        
                    },
                _ => {}
                }
            },
            _ => {}
        } 
    }

    all_quad.len()
}

pub fn construct_tree(path: &str) -> Result<MultiTree, ()>{
    if let Ok(lines) = read_lines(path) {
        let classTree = MultiTree::new(owl::OWL_CLASS);

        for line in lines {
            if let Ok(triple) = line {
                let vec:Vec<&str> = triple.split(' ').collect();

                let p = &vec[1][1..vec[1].len()-1];
                if p == lubm::SUB_ORGANIZATION {
                    let s = &vec[0][1..vec[0].len()-1];
                    let o = &vec[2][1..vec[2].len()-1];
                    
                    classTree.insert(s, o);
                }
            }      
        }   

        classTree.encode();

        return Ok(classTree)
    }
    Err(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}