// 该文件进行dbpedia数据集的查询（原生查询与推理查询）

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

use oxiuselib::extendedTree::{MultiTree, MultiTreeNode};
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
    let dataset = "src/dataset/ontology.nt";
    
    println!("############################## 原生查询 ##############################");
    query(dataset);    // 不进行推理

    println!("############################## 推理查询 ##############################");
    query_reason(dataset);   // 进行推理
}

fn query(dataset: &str) {
    let store = Store::new().unwrap();
    let mut str_buf = String::new();
    File::open(dataset).unwrap().read_to_string(&mut str_buf);

    store.bulk_loader().load_graph(str_buf.as_bytes().as_ref(), GraphFormat::NTriples, GraphNameRef::DefaultGraph, None).unwrap();


    let node = vec!["http://www.w3.org/2000/01/rdf-schema#subClassOf", "http://www.w3.org/2000/01/rdf-schema#subPropertyOf",
    "http://www.w3.org/2000/01/rdf-schema#domain", "http://www.w3.org/2000/01/rdf-schema#range"];
 
    for query in node {
        println!("dbpepia查询(不推理): {}", query);
        let sw = Stopwatch::start_new();

        let ex = NamedNode::new(query).unwrap();
        let results: QuadIter = store.quads_for_pattern(None, Some((&ex).into()), None, None);
        let results: Result<Vec<Quad>,_> = results.collect();
        if let Ok(vec) = results {
            println!("三元组条数：{}", vec.len());
        };

        println!("dataset: {} took {:.8} ms\n", query, sw.elapsed_ms());
    }
}

fn query_reason(dataset: &str) {
    let store = Store::new().unwrap();
    let mut str_buf = String::new();
    File::open(dataset).unwrap().read_to_string(&mut str_buf);

    let (classTree, propertyTree) = construct_tree("src/dataset/dbpedia.nt").unwrap();
    store.bulk_loader().load_graph(str_buf.as_bytes().as_ref(), GraphFormat::NTriples, GraphNameRef::DefaultGraph, None).unwrap();


    let node = vec!["http://www.w3.org/2000/01/rdf-schema#subClassOf", "http://www.w3.org/2000/01/rdf-schema#subPropertyOf",
    "http://www.w3.org/2000/01/rdf-schema#domain", "http://www.w3.org/2000/01/rdf-schema#range"];
    
    for query in node {
        println!("dbpepia查询推理: {}", query);
        let sw = Stopwatch::start_new();

        let ex = NamedNode::new(query).unwrap();
        let results: QuadIter = store.quads_for_pattern(None, Some((&ex).into()), None, None);
        let quad_iter = results.reader.clone();

        let results: Result<Vec<Quad>,_> = results.collect();
        if let Ok(vec) = results {
            reason(quad_iter, vec.clone(), classTree.clone(), propertyTree.clone());
        };

        println!("dataset: {} took {:.8} ms\n", query, sw.elapsed_ms());
    }
}


fn reason(quad_iter: StorageReader, vec_quad: Vec<Quad>, class_tree: MultiTree, property_tree: MultiTree){
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
        
        if p == rdfs::SUB_CLASS_OF || p == lubm::SUB_ORGANIZATION {  // s o
            match sub{
                Ok(s) => {
                    match obj {
                        Ok(o) => {
                            all_quad.insert(Triple::new(s.as_str().into(), p.into(), o.as_str().into()));
                            let parent_way_vec = class_tree.get_parent_way_by_str(StrHash::new(o.as_str()));

                            match parent_way_vec {
                                Ok(parent_way) => {
                                    for way in parent_way {
                                        for parent_strhash in way {

                                            match quad_iter.get_str(&parent_strhash) {
                                                Ok(op) => {
                                                    match op {
                                                        Some(parent) => {
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
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        } else if p == rdfs::SUB_PROPERTY_OF { // s o
            match sub{
                Ok(s) => {
                    match obj {
                        Ok(o) => {
                            let parent_way_vec = property_tree.get_parent_way_by_str(StrHash::new(o.as_str()));
                            all_quad.insert(Triple::new(s.as_str().into(), p.into(), o.as_str().into()));

                            match parent_way_vec {
                                Ok(parent_way) => {
                                    for way in parent_way {
                                    for parent_strhash in way {

                                        match quad_iter.get_str(&parent_strhash) {
                                            Ok(op) => {
                                                match op {
                                                    Some(parent) => {
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
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        } else if (p == rdfs::DOMAIN) || (p == rdfs::RANGE) {  // o
            match sub{
                Ok(s) => {
                    match obj {
                        Ok(o) => {
                            let parent_way_vec = class_tree.get_parent_way_by_str(StrHash::new(o.as_str()));
                            all_quad.insert(Triple::new(s.as_str().into(), p.into(), o.as_str().into()));

                            match parent_way_vec {
                                Ok(parent_way) => {
                                    for way in parent_way {
                                    for parent_strhash in way {

                                        match quad_iter.get_str(&parent_strhash) {
                                            Ok(op) => {
                                                match op {
                                                    Some(parent) => {
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
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    println!("三元组条数: {}", all_quad.len());
}

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

        return Ok((classTree, propertyTree))
    }
    Err(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}