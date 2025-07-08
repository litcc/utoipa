#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use utoipa::IntoParams;
use utoipa_gen::ToSchema;


#[derive(Deserialize, Serialize, ToSchema,IntoParams)]
struct TestB {
    id2: u64,
    name2: String,
    age2: Option<i32>,
}

#[derive(Deserialize, Serialize, ToSchema,IntoParams)]
struct Pet {
    id1: u64,
    name1: String,
    age1: Option<i32>,
    #[serde(flatten)]
    test2:TestB
}



#[test]
pub fn test(){
    let kk = Pet::into_params(||{None});

    println!("{:#?}", kk);
}
