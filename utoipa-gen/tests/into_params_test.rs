#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use utoipa::IntoParams;
use utoipa::openapi::{ObjectBuilder, Type};
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
#[derive(Deserialize, Serialize, ToSchema,IntoParams)]
struct Pet2 {
    id1: u64,
    name1: String,
    age1: Option<i32>,
    #[serde(flatten)]
    #[param(schema_with = schema_with_test1)]
    test2:TestB
}


#[derive(Deserialize, Serialize, ToSchema,IntoParams)]
struct Pet3{
    id1: u64,
    name1: String,
    age1: Option<i32>,
    #[serde(flatten)]
    #[param(schema_with = schema_with_test1)]
    test2: TestB
}




pub fn schema_with_test1(parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>) -> Vec<utoipa::openapi::path::Parameter>{
    vec![
        utoipa::openapi::path::ParameterBuilder::new()
            .name("bbb")
            .schema(ObjectBuilder::new().schema_type(Type::String).into())
            .parameter_in(parameter_in_provider().unwrap_or_default())
            .build()
    ]
}





#[test]
pub fn test(){
    let kk = Pet::into_params(||{None});
    let kk2 = Pet2::into_params(||{None});

    assert_eq!(kk.len(), 6_usize);
    assert_eq!(kk2.len(), 4_usize);


}
