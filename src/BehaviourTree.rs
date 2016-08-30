use BlackBoard::*;
use Node::*;
use std::collections::HashMap;

pub trait Entity {
    // add code here
}


pub struct Context<'a> {
    pub blackBoard: &'a BlackBoard,
    pub currentOpenNodes: HashMap<String, &'a Node>,
    pub lastOpenNodes: HashMap<String, &'a Node>,
}

pub struct BehaviourTree {
    id: String,
    root: Node,
}


impl BehaviourTree {
    fn tick(&self, context: &mut Context) -> Status {
        let status: Status = self.root.execute(context);

        context.lastOpenNodes
            .iter()
            .filter(|&(key, node)| context.currentOpenNodes.contains_key(key))
            .map(|(key, node)| node.close(context));


        context.lastOpenNodes.clear();

        let mut openNodes: Vec<(String, &Node)> = context.currentOpenNodes.drain().collect();

        openNodes.iter()
            .map(|&(ref key, ref node)| context.lastOpenNodes.insert(key.clone(), node.clone()));

        status
    }
}
