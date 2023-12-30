// Implementation of the nodes used in the search tree.
// Important for modularity and reusability accross different problems.

// Implements the copy trait
#[derive(Copy, Clone)]
pub struct State {
    id: u32,     // id of the state
    action: u32, // action taken to get to this state
    reward: u32, // reward for getting to this state
}

pub trait Node {
    fn new(&self, state: State) -> Self;
    fn select_child(&self) -> Self; // Implemented as a trait so that it can be used for different problems.

    // Sets the node as explored.
    fn is_explored(&self) -> bool;
    fn set_explored(&mut self);

    // Returns true if the node is a terminal node.
    fn is_terminal(&self) -> bool;

    // Returns a vector of all the children of the node.
    fn get_children(&self) -> Vec<Box<Self>>; // Uses Box to not have to worry about the size of the vector.

    // Managing the number of times a node has been visited.
    fn get_times_visited(&self) -> u32;
    fn increment_times_visited(&mut self);

    // Managing the state of the Node, useful for backpropagation.
    fn get_state(&self) -> State;
    fn set_state(&mut self, state: State);
}

// Specific implementation of the node for UI adaptation problem.
pub struct AdaptationNode {
    state: State,
    times_visited: u32,
    explored: bool,
}

fn get_upper_confidence_bound(node: AdaptationNode, parent_visits: u32) -> f32 {
    let c = 1.0;
    let exploitation = node.get_state().id as f32;
    let exploration = c * ((parent_visits.ilog10() / node.get_times_visited()) as f32).sqrt();
    exploitation + exploration
}

impl Node for AdaptationNode {
    fn new(&self, state: State) -> Self {
        AdaptationNode {
            state,
            times_visited: 0,
            explored: false,
        }
    }
    fn select_child(&self) -> Self {
        // let c = 1.0;
        // let exploitation = self.get_state().id as f32;
        // let exploration = c * ((self.times_visited.ilog10() / node.get_times_visited()) as f32).sqrt();
        // exploitation + exploration

        let current_node = self;
        let mut unexplored_children = Vec::new();
        for child in self.get_children() {
            if !child.is_explored() {
                unexplored_children.push(child);
            }
        }

        if unexplored_children.is_empty() {
            let mut max_value = f32::NEG_INFINITY;
            let mut max = unexplored_children[0];

            for child in unexplored_children {
                let bound = get_upper_confidence_bound(child, current_node.get_times_visited());
                if bound > max_value {
                    max_value = bound;
                    max = child;
                }
            }
            max
        } else {
            let mut rng = rand::thread_rng();
            let random_child = rng.gen_range(0, unexplored_children.len());
            unexplored_children[random_child]
        }
    }

    fn is_terminal(&self) -> bool {
        self.get_children().is_empty()
    }

    fn get_children(&self) -> Vec<Box<AdaptationNode>> {
        todo!()
    }

    fn get_times_visited(&self) -> u32 {
        self.times_visited
    }

    fn get_state(&self) -> State {
        self.state
    }

    fn set_state(&mut self, state: State) {
        self.state = state;
    }

    fn is_explored(&self) -> bool {
        self.explored
    }

    fn set_explored(&mut self) {
        self.explored = true;
    }

    fn increment_times_visited(&mut self) {
        todo!()
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcts() {}
}
