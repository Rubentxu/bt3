use BlackBoard:: *;


pub struct Context {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    NONE,
    SUCCESS,
    FAILURE,
    RUNNING,
    ERROR,
}

#[derive(Debug)]
pub enum NodeCategorie {
    COMPOSITE,
    DECORATOR,
    ACTION,
    CONDITION,
}

trait Node {
    fn id(&self) -> &String;

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
pub struct TriggerAction {
    id: String,
    status: Status,
}

impl TriggerAction {
    fn new(_id: String, _status: Status) -> Self {
        TriggerAction {
            id: _id,
            status: _status,
        }
    }
}

impl Node for TriggerAction {
    fn id(&self) -> &String {
        &self.id
    }


    fn category(&self) -> NodeCategorie {
        NodeCategorie::ACTION
    }

    fn tick(&self, context: &Context) -> Status {
        self.status
    }
}

#[derive(Debug)]
pub struct ToggleAction {
    id: String,
    enabled: Status,
    disabled: Status,
    current: Status,
}

impl ToggleAction {
    fn new(_id: String, _enabled: Status, _disabled: Status, _current: Status) -> Self {
        ToggleAction {
            id: _id,
            enabled: _enabled,
            disabled: _disabled,
            current: _current,
        }
    }
}

impl Node for ToggleAction {
    fn id(&self) -> &String {
        &self.id
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
    use Node::Node;

    #[test]
    fn triggerAction_test() {
        let action = TriggerAction::new("idTrigger".to_string(), Status::RUNNING);
        assert_eq!(action.tick(&Context {}), Status::RUNNING);
    }


}
