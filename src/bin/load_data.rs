use oxigraph::storage::numeric_encoder::StrHash;
use oxigraph::store::{Store, QuadIter};
use oxigraph::model::*;
use oxigraph::io::GraphFormat;
use oxigraph::sparql::QueryResults;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use stopwatch::Stopwatch;

use oxiuselib::extendedTree::{MultiTree, MultiTreeNode};
use oxiuselib::extendedTree::vocab::{rdf,rdfs,owl};

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


// construct_tree方法由oxigraph最新
// 数据读取方法oxiuse最新
fn main() {
    // load_dataset_all("F:/dbpedia/ontology.nt", "F:/dbpedia/ontology.nt");
    // load_dataset_all("F:/oxiuse/dataset/test.nt", "F:/oxiuse/dataset/test.nt");
}

fn reason(quad_iter: QuadIter, vec_quad: Vec<Quad>, class_tree: MultiTree, property_tree: MultiTree){
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
        
        if p == rdfs::SUB_CLASS_OF {  // s o
            match sub{
                Ok(s) => {
                    match obj {
                        Ok(o) => {
                            let parent_way_vec = class_tree.get_parent_way_by_str(StrHash::new(o.as_str()));
                            match parent_way_vec {
                                Ok(parent_way) => {
                                    for way in parent_way {
                                    for parent_strhash in way {
                                        all_quad.insert(Triple::new(s.as_str().into(), p.into(), o.as_str().into()));

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
                            match parent_way_vec {
                                Ok(parent_way) => {
                                    for way in parent_way {
                                    for parent_strhash in way {
                                        all_quad.insert(Triple::new(s.as_str().into(), p.into(), o.as_str().into()));

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
                            match parent_way_vec {
                                Ok(parent_way) => {
                                    for way in parent_way {
                                    for parent_strhash in way {
                                        all_quad.insert(Triple::new(s.as_str().into(), p.into(), o.as_str().into()));

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

    for quad in &all_quad {
        println!("{} {} {}", (*quad).subject, (*quad).predicate, (*quad).object);
    }
    println!("三元组条数: {}", all_quad.len());

}

fn load_dataset_all(dataset_path: &str, tree_path: &'static str) {
    let sw = Stopwatch::start_new();
    // 进行区间编码
    // let (class_tree, property_tree) = construct_tree(tree_path).unwrap();
    println!("It took {:.8} ms", sw.elapsed_ms());

    let store = Store::new().unwrap();

    let classTree = MultiTree::new(owl::OWL_CLASS);
    let propertyTree = MultiTree::new(rdf::PROPERTY); 

    if let Ok(lines) = read_lines("F:/oxiuse/dataset/test.ttl") {
        for line in lines {
            if let Ok(triple) = line {
                store.load_graph(triple.clone().as_bytes().as_ref(), GraphFormat::NTriples, GraphNameRef::DefaultGraph, None).unwrap();

                let vec:Vec<&str> = triple.split(' ').collect();

                let p = &vec[1][1..vec[1].len()-1];
                if p == rdfs::SUB_CLASS_OF {
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
    }
    

    let mut str_buf = String::new();
    File::open(dataset_path).unwrap().read_to_string(&mut str_buf);

    // store.bulk_loader().load_graph(str_buf.as_bytes().as_ref(), GraphFormat::NTriples, GraphNameRef::DefaultGraph, None).unwrap();
    // store.bulk_loader().load_graph(str_buf.as_bytes().as_ref(), GraphFormat::NTriples, GraphNameRef::DefaultGraph, None).unwrap();
    

    println!("#################### 进行查询 ######################");
    // 直接查询
    let ex = NamedNodeRef::new("http://www.w3.org/2000/01/rdf-schema#subClassOf").unwrap();

    let results: QuadIter = store.quads_for_pattern(None, Option::Some((ex.into())), None, None);
    let results: QuadIter = store.quads_for_pattern(None, None, None, None);

    // let quad_iter = results.clone();
    let results: Result<Vec<Quad>,_> = results.collect();

    // let string = quad_iter.get_str(&StrHash::new("http://a"));
    // println!("{:?}", string);

    // 进行推理获得全部的四元组
    if let Ok(vec) = results {
        // reason(quad_iter, vec.clone(), classTree, propertyTree);

        for quad in vec {
            println!("{} {} {}", quad.subject, quad.predicate, quad.object);
        }
    };
}



fn directly_query(store: Store) {
    // 直接查询
    let ex = NamedNodeRef::new("http://www.w3.org/2000/01/rdf-schema#subPropertyOf").unwrap();

    let results: Result<Vec<Quad>,_> = store.quads_for_pattern(Option::Some((ex.into())), None, None, None).collect();
    let results: Result<Vec<Quad>,_> = store.quads_for_pattern(None, None, None, None).collect();

    if let Ok(vec) = results {
        for quad in vec {
            println!("{}", quad);
        }
    };
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
                if p == rdfs::SUB_CLASS_OF {
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

        let node_vec = vec![owl::OWL_CLASS,"http://a","http://b","http://c","http://d","http://e","http://f",
        "http://g","http://h","http://i","http://j","http://k","http://l","http://m"];

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