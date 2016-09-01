use behaviour_tree::*;
use std::fmt::Debug;


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

pub trait ID {
    fn id(&self) -> &String;
}


pub trait Node: ID + Debug {
    fn category(&self) -> NodeCategorie;

    #[allow(unused_variables)]
    fn open(&self, context: &Context) {}
    #[allow(unused_variables)]
    fn close(&self, context: &Context) {}
    #[allow(unused_variables)]
    fn enter(&self, context: &Context) {}
    #[allow(unused_variables)]
    fn exit(&self, context: &Context) {}

    fn tick(&self, context: &Context) -> Status;

    fn execute(&self, context: &Context) -> Status {
        self.enter(context);

        let status = self.tick(context);

        if let Status::RUNNING = status {
            self.close(context);
        }

        self.exit(context);
        status
    }
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

impl ID for TriggerAction {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Node for TriggerAction {
    fn category(&self) -> NodeCategorie {
        NodeCategorie::ACTION
    }

    #[allow(unused_variables)]
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


impl ID for ToggleAction {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Node for ToggleAction {
    fn category(&self) -> NodeCategorie {
        NodeCategorie::ACTION
    }

    #[allow(unused_variables)]
    fn tick(&self, context: &Context) -> Status {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triggerAction_test() {
        let action = TriggerAction::new("idTrigger".to_string(), Status::RUNNING);
        // assert_eq!(action.tick(&Context {}), Status::RUNNING);
    }


}
