use crate::node::Node;
use rand::Rng; // Make sure to include the rand crate in your dependencies

pub struct mcts<T: Node> {
    root: T,
}

fn get_upper_confidence_bound<T: Node>(node: &T, parent_visits: u32) -> f32 {
    let c = 1.0;
    let exploitation = node.get_state().id as f32;
    let exploration = c * ((parent_visits as f32).ln() / node.get_times_visited() as f32).sqrt();
    exploitation + exploration
}

impl<T: Node> mcts<T> {
    pub fn new(root: T) -> Self {
        mcts { root }
    }

    fn select_child(&self, node: &T) -> T {
        if node.is_terminal() {
            return node.clone();
        } else if !node.is_fully_expanded() {
            return self.expand(node); // Expand the node and return the most promising child.
        } else {
            let mut max_value = f32::NEG_INFINITY;
            let mut max_child = node.get_children()[0].clone();
            for child in node.get_children() {
                let bound = get_upper_confidence_bound(child, node.get_times_visited());
                if bound > max_value {
                    max_value = bound;
                    max_child = child.clone();
                }
            }
            return max_child;
        }
    }

    fn expand(&self, node: &T) -> T {
        let actions = node.get_actions();
        let mut rng = rand::thread_rng();
        let random_action = actions[rng.gen_range(0..actions.len())].clone();

        let new_state = random_action.apply(node.get_state()); // Assuming there is an apply method
        let child = T::from_state(new_state);
        node.add_child(&child);
        child
    }

    pub fn simulate(&self, node: T, move_limit: Option<u32>) -> T {
        let mut node = node;
        let mut move_count = 0;
        while !node.is_terminal() && move_limit.map_or(true, |limit| move_count < limit) {
            let actions = node.get_actions();
            let rand_action = actions[rand::thread_rng().gen_range(0..actions.len())].clone();
            node = rand_action.apply(&node.get_state()); // Assuming there is an apply method
            move_count += 1;
        }
        node
    }

    pub fn backpropagate(&self, node: &T) {
        let mut current_node = node.clone();
        while let Some(parent) = current_node.get_parent() {
            parent.update_value(current_node.get_value());
            current_node = parent;
        }
    }

    pub fn run(&mut self, move_limit: Option<u32>) -> T {
        let mut current_node = self.root.clone();
        while !current_node.is_terminal() {
            current_node = self.select_child(&current_node);
            current_node = self.expand(&current_node);
            current_node = self.simulate(current_node, move_limit);
            self.backpropagate(&current_node);
        }
        current_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_select_child() {
        let mut mcts = MCTS::new();
        let node = Node::new();
        let selected_child = mcts.select_child(&node);
        assert!(node.get_children().contains(&selected_child));
    }

    #[test]
    fn test_expand() {
        let mut mcts = MCTS::new();
        let node = Node::new();
        let expanded_node = mcts.expand(&node);
        assert!(node.get_children().contains(&expanded_node));
    }

    #[test]
    fn test_simulate() {
        let mut mcts = MCTS::new();
        let node = Node::new();
        let simulated_node = mcts.simulate(node, Some(10));
        assert!(simulated_node.is_terminal() || simulated_node.get_actions().len() == 10);
    }

    #[test]
    fn test_backpropagate() {
        let mut mcts = MCTS::new();
        let node = Node::new();
        mcts.backpropagate(&node);
        // Add assertions based on your backpropagation logic
    }

    #[test]
    fn test_run() {
        let mut mcts = MCTS::new();
        let result_node = mcts.run(Some(10));
        assert!(result_node.is_terminal());
    }
}


