pub mod rdf {
    pub const PROPERTY: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";

    pub const TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
}

pub mod rdfs {
    pub const SUB_CLASS_OF: &str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";

    pub const SUB_PROPERTY_OF: &str = "http://www.w3.org/2000/01/rdf-schema#subPropertyOf";

    pub const DOMAIN: &str = "http://www.w3.org/2000/01/rdf-schema#domain";

    pub const RANGE: &str = "http://www.w3.org/2000/01/rdf-schema#range";
}

pub mod owl {
    pub const OWL_CLASS: &str = "http://www.w3.org/2002/07/owl#Class";
}

pub mod lubm {
    pub const SUB_ORGANIZATION: &str = "tju:#subOrganizationOf";

    pub const UnderGraduate_Degree_From: &str = "tju:#undergraduateDegreeFrom";

    pub const Master_Degree_From: &str = "tju:#mastersDegreeFrom";

    pub const Doctoral_Degree_From: &str = "tju:#doctoralDegreeFrom";

    pub const WORKS_FOR: &str = "tju:#worksFor";
}