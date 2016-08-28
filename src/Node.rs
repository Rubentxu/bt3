use BlackBoard:: *;


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
pub struct TriggerAction {
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
pub struct ToggleAction {
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
