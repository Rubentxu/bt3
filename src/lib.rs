
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

trait Node {
    type C;

    fn open(&self, &Self::C);

    fn close(&self, &Self::C);

    fn enter(&self, &Self::C);

    fn exit(&self, &Self::C);

    fn tick(&self, &Self::C) -> Status;

    fn execute(&self, &Self::C) -> Status;
}

struct BaseNode {
    id: String,
    name: String,
    category: NodeCategorie,
}

impl Node for BaseNode {
    type C = Context;

    fn open(&self, context: &Context) {
        unimplemented!();
    }

    fn close(&self, context: &Context) {
        unimplemented!();
    }

    fn enter(&self, context: &Context) {
        unimplemented!();
    }

    fn exit(&self, context: &Context) {
        unimplemented!();
    }

    fn tick(&self, context: &Context) -> Status {
        Status::NONE
    }

    fn execute(&self, context: &Context) -> Status {
        Status::NONE
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
