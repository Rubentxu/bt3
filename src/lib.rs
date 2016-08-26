use std::collections::HashMap;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    NONE,
    SUCCESS,
    FAILURE,
    RUNNING,
    ERROR,
}

#[derive(Debug)]
enum NodeCategorie {
    COMPOSITE,
    DECORATOR,
    ACTION,
    CONDITION,
}

struct Context {}

#[derive(Debug, PartialEq, Eq, Hash)]
struct MemoryScope<'a> {
    baseScope: &'a str,
    treeScope: &'a str,
    nodeScope: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
enum MemoryParams {
    Type_String(String),
    Type_Int(i32),
    Type_None,
}

struct BlackBoard<'a> {
    memory: HashMap<MemoryScope<'a>, MemoryParams>,
}

impl<'a> BlackBoard<'a> {
    fn new() -> BlackBoard<'a> {
        BlackBoard { memory: HashMap::new() }
    }

    fn getBaseParam<'b>(&'a self, baseScope: &'a str) -> &'a MemoryParams {
        let item = self.memory.get(&MemoryScope::<'a> {
            baseScope: baseScope,
            treeScope: "",
            nodeScope: "",
        });
        let result = MemoryParams::Type_None;
        match item {
            Some(i) => return  i,
            _ => return &result         
        };
       
    }
}




trait Node {
    fn id(&self) -> &'static str;

    fn category(&self) -> NodeCategorie;

    fn tick(&self, context: &Context) -> Status {
        Status::NONE
    }

    fn execute(&self, context: &Context) -> Status {
        self.enter(context);

        let status = self.tick(context);

        if let Status::RUNNING = status {
            self.close(context);
        }

        self.exit(context);
        status
    }

    fn open(&self, context: &Context) {}

    fn close(&self, context: &Context) {}

    fn enter(&self, context: &Context) {}

    fn exit(&self, context: &Context) {}
}


#[derive(Debug)]
struct TriggerAction {
    id: &'static str,
    status: Status,
}

impl TriggerAction {
    fn new(_id: &'static str, _status: Status) -> Self {
        TriggerAction {
            id: _id,
            status: _status,
        }
    }
}

impl Node for TriggerAction {
    fn id(&self) -> &'static str {
        self.id
    }


    fn category(&self) -> NodeCategorie {
        NodeCategorie::ACTION
    }

    fn tick(&self, context: &Context) -> Status {
        self.status
    }
}

#[derive(Debug)]
struct ToggleAction {
    id: &'static str,
    enabled: Status,
    disabled: Status,
    current: Status,
}

impl ToggleAction {
    fn new(_id: &'static str, _enabled: Status, _disabled: Status, _current: Status) -> Self {
        ToggleAction {
            id: _id,
            enabled: _enabled,
            disabled: _disabled,
            current: _current,
        }
    }
}

impl Node for ToggleAction {
    fn id(&self) -> &'static str {
        self.id
    }

    fn category(&self) -> NodeCategorie {
        NodeCategorie::ACTION
    }

    fn tick(&self, context: &Context) -> Status {
        self.current
    }
}







#[cfg(test)]
mod tests {
    use super::*;
    use super::Node;
    use super::TriggerAction;
    use super::Context;
    use super::Status;
    use super::Status::*;
    use super::BlackBoard;
    use super::MemoryScope;
    use std::collections::HashMap;
    use super::MemoryParams;
    use super::MemoryParams::*;

    // #[test]
    // fn triggerAction_test() {
    // let action: TriggerAction = TriggerAction::new("idTrigger", Status::RUNNING);
    // assert_eq!(action.tick(&Context {}), Status::RUNNING);
    // }
    //
    #[test]
    fn blackboard_test() {
        println!("Inicializando blackboard_test.....");
        let mut blackboard: BlackBoard = BlackBoard::new();
        let scope = MemoryScope {
            baseScope: "base",
            treeScope: "",
            nodeScope: "",
        };

        blackboard.insert(scope, MemoryParams::Type_Int(1));

        let item = blackboard.get(&MemoryScope {
            baseScope: "base",
            treeScope: "",
            nodeScope: "",
        });
        let result;
        match item {
            Some(&Type_Int(i)) => result = i,
            _ => unreachable!(),           
        };

        assert_eq!(1, result);

    }
}
