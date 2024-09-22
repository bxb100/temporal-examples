use serde::{Deserialize, Serialize};
use serde_yml::with::singleton_map_recursive;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dsl {
    pub variables: HashMap<String, String>,
    #[serde(with = "singleton_map_recursive")]
    pub root: Statement,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Statement {
    #[serde(rename = "activity")]
    Activity(ActivityInvocation),
    #[serde(rename = "sequence")]
    Sequence(SequenceStruct),
    #[serde(rename = "parallel")]
    Parallel(ParallelStruct),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SequenceStruct {
    pub elements: Vec<Statement>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ActivityInvocation {
    pub name: String,
    pub arguments: Option<Vec<String>>,
    pub result: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ParallelStruct {
    pub branches: Vec<Statement>,
}

unsafe impl Send for Statement {}
unsafe impl Send for SequenceStruct {}
unsafe impl Send for ActivityInvocation {}
unsafe impl Send for ParallelStruct {}

#[cfg(test)]
mod tests {
    use crate::dsl::types::*;

    #[test]
    fn test_deserialize() {
        let workflow1 = std::fs::read_to_string("workflow1.yaml").unwrap();
        let dsl = serde_yml::from_str::<Dsl>(&workflow1).unwrap();
        assert!(!dsl.variables.is_empty());
        assert_eq!(dsl.variables["arg1"], "value1".to_string());
        assert_eq!(dsl.variables["arg2"], "value2".to_string());
        assert_eq!(
            dsl.root,
            Statement::Sequence(SequenceStruct {
                elements: vec![
                    Statement::Activity(ActivityInvocation {
                        name: "activity1".to_string(),
                        arguments: Some(vec!["arg1".to_string()]),
                        result: Some("result1".to_string()),
                    }),
                    Statement::Activity(ActivityInvocation {
                        name: "activity2".to_string(),
                        arguments: Some(vec!["result1".to_string()]),
                        result: Some("result2".to_string()),
                    }),
                    Statement::Activity(ActivityInvocation {
                        name: "activity3".to_string(),
                        arguments: Some(vec!["arg2".to_string(), "result2".to_string()]),
                        result: Some("result3".to_string()),
                    }),
                ],
            })
        );
    }
}
