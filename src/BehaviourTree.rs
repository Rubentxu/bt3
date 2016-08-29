use BlackBoard::*;
use Node::*;
use std::collections::HashSet;

pub trait Entity {
    // add code here
} 


pub struct Context<'a> {
    pub blackBoard: &'a BlackBoard,
    pub currentOpenNodes: HashSet<Node>,
    pub lastOpenNodes: HashSet<Node>
}

pub trait BehaviourTree {
    fn tick(&self, context: &Context, root: &Node) -> Status {
        let status: Status = self.execute(context, root);
       
        for node in context.lastOpenNodes.intersection(&context.currentOpenNodes) {
        	node.close(context);
        }

        status
    }

    fn execute(&self, context: &Context, node: &Node) -> Status {
        node.enter(context);

        let status = node.tick(context);

        if let Status::RUNNING = status {
            node.close(context);
        }

        node.exit(context);
        status
    }
}
