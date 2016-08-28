use BehaviourTree:: *;
use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct MemoryScope {
    baseScope: String,
    treeScope: String,
    nodeScope: String,
    param: String,
}

impl MemoryScope {
    fn base(baseScope: &str, param: &str) -> MemoryScope {
        MemoryScope {
            baseScope: baseScope.to_string(),
            param: param.to_string(),
            ..Default::default()
        }
    }

    fn tree(baseScope: &str, treeScope: &str, param: &str) -> MemoryScope {
        MemoryScope {
            baseScope: baseScope.to_string(),
            treeScope: treeScope.to_string(),
            param: param.to_string(),
            ..Default::default()
        }
    }

    fn node(baseScope: &str, treeScope: &str, nodeScope: &str, param: &str) -> MemoryScope {
        MemoryScope {
            baseScope: baseScope.to_string(),
            treeScope: treeScope.to_string(),
            nodeScope: nodeScope.to_string(),
            param: param.to_string(),
        }
    }
}

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
        let scope = MemoryScope::tree("baseScope", "treeScope", "test");

        blackboard.insert(scope, MemoryParams::Type_Int(2));

        let item = blackboard.get(&MemoryScope::tree("baseScope", "treeScope", "test"));
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
        let scope = MemoryScope::node("baseScope", "treeScope", "nodeScope", "test");

        blackboard.insert(scope, MemoryParams::Type_String("Pruebas".to_string()));

        let item = blackboard.get(&MemoryScope::node("baseScope", "treeScope", "nodeScope", "test"));
        let mut result = "";
        if let Some(&MemoryParams::Type_String(ref i)) = item {
            result = i;
        };

        assert_eq!("Pruebas", result);

    }
}
