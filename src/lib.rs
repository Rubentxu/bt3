
enum Status {
    NONE,
    SUCCESS,
    FAILURE,
    RUNNING,
    ERROR,
}

enum NodeCategorie {
    COMPOSITE,
    DECORATOR,
    ACTION,
    CONDITION,
}

struct Context {

}

trait NodeInteraction {
    fn open(&self, context: &Context);

    fn close(&self, context: &Context);
}

trait NodeLifecycle {
    fn enter(&self, context: &Context);

    fn exit(&self, context: &Context);
}

trait Node {
    fn tick(&self, context: &Context) -> Status;

    fn execute(&self, context: &Context) -> Status;
}

struct BaseNode {
    id: String,
    name: String,
    category: NodeCategorie,
}

impl Node for BaseNode {
    fn tick(&self, context: &Context) -> Status {
        Status::NONE
    }

    fn execute(&self, context: &Context) -> Status {
        Status::NONE
    }
}

type Condition = BaseNode;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
