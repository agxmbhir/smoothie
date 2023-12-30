use crate::node::Node;

pub struct mcts<T: Node> {
    root: T,
}

impl<T: Node> mcts<T> {
    pub fn new() -> Self {
        mcts { root: T::new() }
    }

    pub fn select(&self, node: T) -> T {
        // Find an unexplored descendant of the given node.
        if node.is_terminal() {
            return node;
        }
        let current_node = node;
        // Add a way to store the path

        while !current_node.is_terminal() {
            let children = current_node.get_children();
            let mut unexplored_children = Vec::new();
            for child in children {
                if !child.is_explored() {
                    unexplored_children.push(child);
                }
            }
            if !unexplored_children.is_empty() {
                // // if all the children have been explored
                // // max value is negative infinity
                // let mut max_value = f32::NEG_INFINITY;
                // let mut max = unexplored_children[0];

                // for child in unexplored_children {
                //     let bound = get_upper_confidence_bound(child, current_node.get_times_visited());
                //     if bound > max_value {
                //         max_value = bound;
                //         max = child;
                //     }
                // }
                current_node = current_node.select_child();
            } else {
                // if there are unexplored children
                let mut rng = rand::thread_rng();
                let random_child = rng.gen_range(0, unexplored_children.len());
                current_node = unexplored_children[random_child];
            }
        }

        get_upper_confidence_bound(current_node, current_node.get_times_visited());
    }
    pub fn expand(&self, node: T) -> T {}
    pub fn simulate(&self, node: T) -> T {}
    pub fn backpropagate(&self, node: T) -> T {}
    pub fn run(&self, node: T) -> T {}
}
