use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct MemoryScope {
    base_scope: String,
    tree_scope: String,
    node_scope: String,
    param: String,
}


impl MemoryScope {
    #[allow(dead_code)]
    fn base(base_scope: &str, param: &str) -> MemoryScope {
        MemoryScope {
            base_scope: base_scope.to_string(),
            param: param.to_string(),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    fn tree(base_scope: &str, tree_scope: &str, param: &str) -> MemoryScope {
        MemoryScope {
            base_scope: base_scope.to_string(),
            tree_scope: tree_scope.to_string(),
            param: param.to_string(),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    fn node(base_scope: &str, tree_scope: &str, node_scope: &str, param: &str) -> MemoryScope {
        MemoryScope {
            base_scope: base_scope.to_string(),
            tree_scope: tree_scope.to_string(),
            node_scope: node_scope.to_string(),
            param: param.to_string(),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum MemoryParams {
    Type_String(String),
    Type_Int(i32),
}

pub type BlackBoard = HashMap<MemoryScope, MemoryParams>;




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blackboard_scope_base_test() {
        let mut blackboard: BlackBoard = BlackBoard::new();
        let scope = MemoryScope::base("base","test");

        blackboard.insert(scope, MemoryParams::Type_Int(1));

        let item = blackboard.get(&MemoryScope::base("base","test"));
        let result;
        match item {
            Some(&MemoryParams::Type_Int(i)) => result = i,
            _ => unreachable!(),           
        };

        assert_eq!(1, result);

    }

    #[test]
    fn blackboard_scope_tree_test() {
        let mut blackboard: BlackBoard = BlackBoard::new();
        let scope = MemoryScope::tree("base_scope", "tree_scope", "test");

        blackboard.insert(scope, MemoryParams::Type_Int(2));

        let item = blackboard.get(&MemoryScope::tree("base_scope", "tree_scope", "test"));
        let result;
        match item {
            Some(&MemoryParams::Type_Int(i)) => result = i,
            _ => result = 0,           
        };
        assert_eq!(2, result);

    }

    #[test]
    fn blackboard_scope_node_test() {
        let mut blackboard: BlackBoard = BlackBoard::new();
        let scope = MemoryScope::node("base_scope", "tree_scope", "node_scope", "test");

        blackboard.insert(scope, MemoryParams::Type_String("Pruebas".to_string()));

        let item = blackboard.get(&MemoryScope::node("base_scope", "tree_scope", "node_scope", "test"));
        let mut result = "";
        if let Some(&MemoryParams::Type_String(ref i)) = item {
            result = i;
        };

        assert_eq!("Pruebas", result);

    }
}
