use BlackBoard::*;
use Node::*;
use std::collections::HashMap;

pub trait Entity {
    // add code here
}


pub struct Context<'a, 'b> {
    pub blackBoard: &'a BlackBoard,
    pub currentOpenNodes: HashMap<String, &'b Node>,
    pub lastOpenNodes: HashMap<String, &'b Node>,
}

pub struct BehaviourTree<T: Node+ ?Sized> {
    id: String,
    root: T,
}


impl<T: Node+ ?Sized> BehaviourTree<T> {
    fn tick(&self, context: &mut Context) -> Status {
        let status: Status = self.root.execute(context);

        context.lastOpenNodes
            .iter()
            .filter(|&(key, node)| context.currentOpenNodes.contains_key(key))
            .map(|(_, node)| node.close(context));


        context.lastOpenNodes.clear();

        let mut openNodes: Vec<(String, &Node)> = context.currentOpenNodes.drain().collect();

        openNodes.iter()
            .map(|&(ref key, ref node)| context.lastOpenNodes.insert(key.clone(), node.clone()));

        status
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use Node::*;
    use BlackBoard::*;
    use std::collections::HashMap;

    struct NodeTest {
    	id : String,
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

    struct NodeTest2 {
    	id : String,
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
    }

     struct NodeTest3 {
    	id : String,
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
            root: NodeTest { id: "Test".to_string()},
        };
        let mut context =  Context { currentOpenNodes: HashMap::new(), lastOpenNodes: HashMap::new(), blackBoard : &BlackBoard::new()};
        assert_eq!(bt.tick(&mut context), Status::RUNNING);

        let node = NodeTest2 { id: "Test2".to_string()};

        context.currentOpenNodes.insert("Test2".to_string(), &node);

        assert!(context.lastOpenNodes.contains_key("Test2"));
    }


}
