use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

#[derive(thiserror::Error, Debug)]
pub enum PathError {
    #[error("explored all nodes")]
    Exhausted,
    #[error("abort requested")]
    Abort,
}

#[derive(PartialEq, Eq)]
struct Node<T> {
    id: T,
    pred: Option<T>,
    cost: i64,
}

impl<T> Ord for Node<T>
where
    T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost, &self.id).cmp(&(self.cost, &other.id))
    }
}

impl<T> PartialOrd for Node<T>
where
    T: PartialEq + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (other.cost, &self.id).partial_cmp(&(self.cost, &other.id))
    }
}

pub trait Callbacks<T> {
    fn get_neighbours(&mut self, id: &T) -> Vec<(i64, T)>;
    fn found_path(&mut self, id: &T, cost: i64) -> Result<(), PathError>;
}

pub struct Pathfinder<T>
where
    T: Eq + Ord + Hash + Clone,
{
    open: BinaryHeap<Node<T>>,
    pred: HashMap<T, HashSet<Option<T>>>,
    dist: HashMap<T, i64>,
}

impl<T> Pathfinder<T>
where
    T: Eq + Ord + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            open: BinaryHeap::new(),
            pred: HashMap::new(),
            dist: HashMap::new(),
        }
    }

    fn start(&mut self, start_node: T) {
        self.open.clear();
        self.pred.clear();
        self.dist.clear();

        self.open.push(Node {
            id: start_node,
            pred: None,
            cost: 0,
        });
    }

    fn step<C>(&mut self, callbacks: &mut C) -> Result<(), PathError>
    where
        C: Callbacks<T>,
    {
        if let Some(node) = self.open.pop() {
            if let Some(dist) = self.dist.get(&node.id) {
                match node.cost.cmp(&dist) {
                    Ordering::Less => {
                        self.dist.insert(node.id.clone(), node.cost);
                        self.pred
                            .insert(node.id.clone(), HashSet::from([node.pred.clone()]));
                    }
                    Ordering::Equal => {
                        self.pred
                            .get_mut(&node.id)
                            .unwrap()
                            .insert(node.pred.clone());
                    }
                    Ordering::Greater => {
                        return Ok(());
                    }
                }
            } else {
                self.dist.insert(node.id.clone(), node.cost);
                self.pred
                    .insert(node.id.clone(), HashSet::from([node.pred.clone()]));
            }

            callbacks.found_path(&node.id, node.cost)?;

            for (weight, id) in callbacks.get_neighbours(&node.id) {
                self.open.push(Node {
                    id,
                    pred: Some(node.id.clone()),
                    cost: node.cost + weight,
                });
            }

            return Ok(());
        } else {
            return Err(PathError::Exhausted);
        }
    }

    pub fn run<C>(&mut self, callbacks: &mut C, start_node: T) -> Result<(), PathError>
    where
        C: Callbacks<T>,
    {
        self.start(start_node);

        loop {
            self.step(callbacks)?;
        }
    }

    pub fn get_path(&self, end_node: T) -> Vec<T> {
        let mut ret = Vec::new();

        let mut curr = Some(end_node.clone());
        while let Some(ref n) = curr {
            ret.push(n.clone());
            let p = self.pred.get(n);
            curr = p.and_then(|p| p.iter().next().unwrap().clone());
        }

        ret
    }

    pub fn get_all_preds(&self, end_node: T) -> HashSet<T> {
        let mut ret = HashSet::new();

        let mut open = vec![end_node];

        while let Some(n) = open.pop() {
            if ret.insert(n.clone()) {
                if let Some(pred) = self.pred.get(&n) {
                    for p in pred {
                        if let Some(p) = p {
                            open.push(p.clone());
                        }
                    }
                }
            }
        }

        ret
    }
}
