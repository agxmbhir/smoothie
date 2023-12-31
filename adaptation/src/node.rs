// Implementation of the nodes used in the search tree.
// Important for modularity and reusability accross different problems.
use rand::Rng;

// Implements the copy trait
#[derive(Clone)]
pub struct State {
    id: u32,             // id of the state
    action: Box<Action>, // action taken to get to this state
    reward: u32,         // reward for getting to this state
}

#[derive(Clone)]
pub struct Action {
    id: u32,
    state: Box<State>,
    probability: f32,
}

trait Node {
    fn new(state: State) -> Self;

    // Means all the children have been visited.
    fn is_fully_expanded(&self) -> bool;
    fn set_is_fully_expanded(&mut self);

    // Returns true if the node is a terminal node.
    fn is_terminal(&self) -> bool;
    fn get_actions(&self) -> Vec<Action>; // Returns a vector of all the actions that can be taken from the node.

    // Returns a vector of all the children of the node.
    fn get_children(&self) -> Vec<Box<Self>>; // Uses Box to not have to worry about the size of the vector.
    fn add_child(&mut self, child: AdaptationNode);

    // Managing the number of times a node has been visited.
    fn get_times_visited(&self) -> u32;
    fn increment_times_visited(&mut self);

    // Managing the state of the Node, useful for backpropagation.
    fn get_state(&self) -> State;
    fn set_state(&mut self, state: State);
}

// Specific implementation of the node for UI adaptation problem.
#[derive(Clone)]
pub struct AdaptationNode {
    state: State,
    times_visited: u32,
    children: Vec<Box<AdaptationNode>>,
    explored: bool,
}

impl Node for AdaptationNode {
    fn new(state: State) -> Self {
        AdaptationNode {
            state,
            times_visited: 0,
            children: Vec::new(),
            explored: false,
        }
    }

    fn is_terminal(&self) -> bool {
        self.get_actions().is_empty()
    }

    fn get_children(&self) -> Vec<Box<AdaptationNode>> {
        todo!()
    }
    fn add_child(&mut self, child: AdaptationNode) {
        self.children.push(Box::new(child));
    }

    fn get_times_visited(&self) -> u32 {
        self.times_visited
    }

    fn get_state(&self) -> State {
        self.state.clone()
    }

    fn set_state(&mut self, state: State) {
        self.state = state;
    }

    fn increment_times_visited(&mut self) {
        todo!()
    }

    fn is_fully_expanded(&self) -> bool {
        todo!()
    }

    fn set_is_fully_expanded(&mut self) {
        todo!()
    }

    fn get_actions(&self) -> Vec<Action> {
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
