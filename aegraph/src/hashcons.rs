use core::fmt::Debug;
use core::hash::Hash;

use rustc_hash::FxHashMap;

pub type Id = usize;

pub trait Node: Clone + Debug + PartialEq + Eq + Hash {}

#[derive(Clone, Debug, Default)]
pub struct HashCons<N: Node> {
    nodes: Vec<N>,
    ids: FxHashMap<N, Id>,
}

impl<N: Node> HashCons<N> {
    pub fn add_node<F>(&mut self, node: N, smart: F) -> Id
    where
        F: Fn(N, &mut HashCons<N>) -> Id,
    {
        if let Some(id) = self.ids.get(&node) {
            *id
        } else {
            smart(node, self)
        }
    }

    pub fn add_node_raw(&mut self, node: N) -> Id {
        if let Some(id) = self.ids.get(&node) {
            *id
        } else {
            let id = self.nodes.len();
            self.nodes.push(node.clone());
            self.ids.insert(node, id);
            id
        }
    }

    pub fn get(&self, id: Id) -> &N {
        &self.nodes[id]
    }
}
