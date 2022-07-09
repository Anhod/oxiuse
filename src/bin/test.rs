
use oxigraph::storage::numeric_encoder::StrHash;

fn main() {
    println!("{:?}", StrHash::new("http://dbpedia.org/ontology/canonizedPlace"));
    println!("{:?}", StrHash::new("http://dbpedia.org/ontology/highestPlace"));
    println!("{:?}", StrHash::new("http://dbpedia.org/ontology/filmVersion"));
}