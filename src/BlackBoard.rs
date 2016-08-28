use std::collections::HashMap;


pub struct Context {}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct MemoryScope {
    baseScope: String,
    treeScope: String,
    nodeScope: String,
}

impl MemoryScope {
    fn base(baseScope: &str) -> MemoryScope {
        MemoryScope { baseScope: baseScope.to_string(), ..Default::default() }
    }

    fn tree(baseScope: &str, treeScope: &str) -> MemoryScope {
        MemoryScope {
            baseScope: baseScope.to_string(),
            treeScope: treeScope.to_string(),
            ..Default::default()
        }
    }

    fn node(baseScope: &str, treeScope: &str, nodeScope: &str) -> MemoryScope {
        MemoryScope {
            baseScope: baseScope.to_string(),
            treeScope: treeScope.to_string(),
            nodeScope: nodeScope.to_string(),
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
   

    // #[test]
    // fn triggerAction_test() {
    // let action: TriggerAction = TriggerAction::new("idTrigger", Status::RUNNING);
    // assert_eq!(action.tick(&Context {}), Status::RUNNING);
    // }
    //
    #[test]
    fn blackboard_scope_base_test() {
        let mut blackboard: BlackBoard = BlackBoard::new();
        let scope = MemoryScope::base("base");

        blackboard.insert(scope, MemoryParams::Type_Int(1));

        let item = blackboard.get(&MemoryScope::base("base"));
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
        let scope = MemoryScope::tree("baseScope", "treeScope");

        blackboard.insert(scope, MemoryParams::Type_Int(2));
       
        let item = blackboard.get(&MemoryScope::tree("baseScope", "treeScope"));
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
        let scope = MemoryScope::node("baseScope", "treeScope", "nodeScope");

        blackboard.insert(scope, MemoryParams::Type_String("Pruebas".to_string()));
       
        let item = blackboard.get(&MemoryScope::node("baseScope", "treeScope", "nodeScope"));
        let mut result = "";
        if let Some(&MemoryParams::Type_String(ref i)) = item {
            result = i;
        };
          
        assert_eq!("Pruebas", result);

    }
}
