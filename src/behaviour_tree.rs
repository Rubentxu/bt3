use blackboard::*;
use node::*;
use std::collections::HashMap;

pub trait Entity {
    // add code here
}


pub struct Context<'a, 'b> {
    pub blackboard: &'a BlackBoard,
    pub current_open_nodes: HashMap<String, &'b Node>,
    pub last_open_nodes: HashMap<String, &'b Node>,
}

pub struct BehaviourTree<T: Node + ?Sized> {
    id: String,
    root: T,
}


impl<T: Node + ?Sized> BehaviourTree<T> {
    fn tick(&self, context: &mut Context) -> Status {
        use std::mem;
        let status: Status = self.root.execute(context);

        context.last_open_nodes
            .iter()
            .filter(|&(key, node)| !context.current_open_nodes.contains_key(key))
            .map(|(key, node)| node.close(context));

        context.last_open_nodes.clear();
        mem::swap(&mut context.last_open_nodes, &mut context.current_open_nodes);

        status
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use node::*;
    use blackboard::*;
    use std::collections::HashMap;

    #[derive(Debug)]
    struct NodeTest {
        id: String,
    }

    impl ID for NodeTest {
        fn id(&self) -> &String {
            &self.id
        }
    }

    impl Node for NodeTest {
        fn category(&self) -> NodeCategorie {
            NodeCategorie::ACTION
        }

        fn tick(&self, context: &Context) -> Status {
            Status::RUNNING
        }
    }


    #[derive(Debug)]
    struct NodeTest2 {
        id: String,
        check: bool
    }

    impl ID for NodeTest2 {
        fn id(&self) -> &String {
            &self.id
        }
    }

    impl Node for NodeTest2 {
        fn category(&self) -> NodeCategorie {
            NodeCategorie::ACTION
        }

        fn tick(&self, context: &Context) -> Status {
            Status::RUNNING
        }

        fn close(&self, context: &Context) {

            println!("{:?} has", context.current_open_nodes);
        }
    }


    #[derive(Debug)]
    struct NodeTest3 {
        id: String,
    }

    impl ID for NodeTest3 {
        fn id(&self) -> &String {
            &self.id
        }
    }

    impl Node for NodeTest3 {
        fn category(&self) -> NodeCategorie {
            NodeCategorie::ACTION
        }

        fn tick(&self, context: &Context) -> Status {
            Status::RUNNING
        }
    }


    #[test]
    fn behaviourTreeTick_test() {
        let bt = BehaviourTree {
            id: "Pruebas".to_string(),
            root: NodeTest { id: "Test".to_string() },
        };

        let node = NodeTest2 { id: "Test2".to_string(), check: false };

        let mut context = Context {
            current_open_nodes: HashMap::new(),
            last_open_nodes: HashMap::new(),
            blackboard: &BlackBoard::new(),
        };
        context.current_open_nodes.insert("Test2".to_string(), &node);
        println!("contiene {:?} ", context.current_open_nodes);

        assert_eq!(bt.tick(&mut context), Status::RUNNING);

        assert_eq!(context.last_open_nodes.len(), 1);
        assert!(context.last_open_nodes.contains_key("Test2"));
        assert!(node.check);
    }


}
