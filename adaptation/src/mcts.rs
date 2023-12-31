use crate::node::Node;

pub struct mcts<T: Node> {
    root: T,
}

fn get_upper_confidence_bound(node: AdaptationNode, parent_visits: u32) -> f32 {
    let c = 1.0;
    let exploitation = node.get_state().id as f32;
    let exploration = c * ((parent_visits.ilog10() / node.get_times_visited()) as f32).sqrt();
    exploitation + exploration
}

impl<T: Node> mcts<T> {
    pub fn new() -> Self {
        mcts { root: T::new() }
    }

    fn select_child(&mut self, node: T) -> Self {
        // If node is not fully expanded or terminal, return it.
        // If the node is fully Expanded, return the child with the highest UCB and descend into it.
        if self.is_terminal() {
            return self.clone();
        } else if !self.is_fully_expanded() {
            return self.expand(); // Expand the node and return the most promising child.
        } else {
            let mut max_value = f32::NEG_INFINITY;
            let mut max = node.get_children()[0].clone();
            for child in node.get_children() {
                let bound = get_upper_confidence_bound(*child.clone(), node.get_times_visited());
                if bound > max_value {
                    max_value = bound;
                    max = child;
                }
            }
            return max.select_child();
        }
    }

    fn expand(&mut self, node: T) -> Self {
        let actions = self.get_actions();
        let mut rng = rand::thread_rng();
        let random_action = rng.gen_range(0..actions.len());

        let new_state = State {
            id: actions[random_action].id,
            action: Box::new(actions[random_action].clone()),
            reward: 0,
        };
        let child = AdaptationNode::new(new_state);
        node.add_child(child);
        child
    }

    pub fn simulate(&self, node: T) -> T {
        let node = node.clone();
        while True {
            if node.is_terminal() {
                return node;
            }
            let rand = rand::thread_rng().gen_range(0..node.get_actions().len());
            node = node.get_actions()[rand].clone();
        }
    }
    pub fn backpropagate(&self, node: T) -> T {}
    pub fn run(&self, node: T) -> T {}
}
