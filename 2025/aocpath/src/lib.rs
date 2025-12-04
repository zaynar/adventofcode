use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

#[derive(thiserror::Error, Debug)]
pub enum PathError {
    #[error("explored all nodes")]
    Exhausted,
    #[error("abort requested")]
    Abort,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Node<T> {
    id: T,
    pred: Option<T>,
    cost: i64,
    heuristic: i64,
}

impl<T> Ord for Node<T>
where
    T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic, &self.id).cmp(&(self.cost + self.heuristic, &other.id))
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
    // Return false to abort processing this node
    fn found_path(&mut self, id: &T, cost: i64) -> Result<bool, PathError>;

    fn heuristic(&mut self, id: &T) -> i64 {
        0
    }
}

pub struct Pathfinder<T>
where
    T: Eq + Ord + Hash + Clone,
{
    open: BinaryHeap<Node<T>>,
    open_deq: VecDeque<Node<T>>,
    pred: HashMap<T, HashSet<Option<T>>>,
    dist: HashMap<T, i64>,
    seen: HashSet<Node<T>>,
}

impl<T> Pathfinder<T>
where
    T: Eq + Ord + Hash + Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            open: BinaryHeap::new(),
            open_deq: VecDeque::new(),
            pred: HashMap::new(),
            dist: HashMap::new(),
            seen: HashSet::new(),
        }
    }

    fn start(&mut self, start_node: T) {
        self.open.clear();
        self.open_deq.clear();
        self.pred.clear();
        self.dist.clear();
        self.seen.clear();

        let start = Node {
            id: start_node.clone(),
            pred: None,
            cost: 0,
            heuristic: 0,
        };
        self.open.push(start.clone());
        self.open_deq.push_back(start.clone());
        self.pred.insert(start_node.clone(), HashSet::new());
        self.dist.insert(start_node.clone(), 0);
        self.seen.insert(start);
    }

    // Like step_dijkstra, but records all equal-cost predecessors in .pred,
    // so you can extract the set of all shortest paths
    fn step_dijkstra_all<C>(&mut self, node: Node<T>, callbacks: &mut C) -> Result<(), PathError>
    where
        C: Callbacks<T>,
    {
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

        if !callbacks.found_path(&node.id, node.cost)? {
            return Ok(());
        }

        for (weight, id) in callbacks.get_neighbours(&node.id) {
            let new_node = Node {
                id,
                pred: Some(node.id.clone()),
                cost: node.cost + weight,
                heuristic: 0,
            };

            if self.seen.insert(new_node.clone()) {
                self.open.push(new_node);
            }
        }

        Ok(())
    }

    fn step_dijkstra<C>(&mut self, node: Node<T>, callbacks: &mut C) -> Result<(), PathError>
    where
        C: Callbacks<T>,
    {
        // Skip superseded paths
        if let Some(&dist) = self.dist.get(&node.id) {
            if dist < node.cost {
                return Ok(());
            }
        }

        if !callbacks.found_path(&node.id, node.cost)? {
            return Ok(());
        }

        for (weight, id) in callbacks.get_neighbours(&node.id) {
            let new_node = Node {
                id,
                pred: Some(node.id.clone()),
                cost: node.cost + weight,
                heuristic: 0,
            };

            // Only push nodes if they're the best we've seen so far
            // (but we need to check them again before found_path() to
            // avoid spurious callbacks, since superseded paths won't get
            // deleted from the heap)
            match self.dist.get(&new_node.id) {
                Some(&dist) if dist <= new_node.cost => (),
                _ => {
                    self.dist.insert(new_node.id.clone(), new_node.cost);
                    self.open.push(new_node);
                }
            }
        }

        Ok(())
    }

    // XXX: this probably isn't correct
    fn step_astar<C>(&mut self, node: Node<T>, callbacks: &mut C) -> Result<(), PathError>
    where
        C: Callbacks<T>,
    {
        // Skip superseded paths
        if let Some(&dist) = self.dist.get(&node.id) {
            if dist < node.cost {
                return Ok(());
            }
        }

        if !callbacks.found_path(&node.id, node.cost)? {
            return Ok(());
        }

        for (weight, id) in callbacks.get_neighbours(&node.id) {
            let new_node = Node {
                id,
                pred: Some(node.id.clone()),
                cost: node.cost + weight,
                heuristic: callbacks.heuristic(&node.id),
            };

            // Only push nodes if they're the best we've seen so far
            // (but we need to check them again before found_path() to
            // avoid spurious callbacks, since superseded paths won't get
            // deleted from the heap)
            match self.dist.get(&new_node.id) {
                Some(&dist) if dist <= new_node.cost => (),
                _ => {
                    self.dist.insert(new_node.id.clone(), new_node.cost);
                    self.open.push(new_node);
                }
            }
        }

        Ok(())
    }

    fn step_bfs_dfs<C>(&mut self, node: Node<T>, callbacks: &mut C) -> Result<(), PathError>
    where
        C: Callbacks<T>,
    {
        if !callbacks.found_path(&node.id, node.cost)? {
            return Ok(());
        }

        for (weight, id) in callbacks.get_neighbours(&node.id) {
            assert_eq!(weight, 1, "BFS requires unweighted edges");
            let new_node = Node {
                id: id.clone(),
                pred: Some(node.id.clone()),
                cost: node.cost + 1,
                heuristic: 0,
            };
            self.dist.entry(new_node.id.clone()).or_insert_with(|| {
                let c = new_node.cost;
                self.open_deq.push_back(new_node);
                c
            });
        }

        Ok(())
    }

    pub fn dijkstra_all<C>(&mut self, callbacks: &mut C, start_node: T) -> Result<(), PathError>
    where
        C: Callbacks<T>,
    {
        self.start(start_node);

        while let Some(node) = self.open.pop() {
            self.step_dijkstra_all(node, callbacks)?;
        }
        Err(PathError::Exhausted)
    }

    pub fn dijkstra<C>(&mut self, callbacks: &mut C, start_node: T) -> Result<(), PathError>
    where
        C: Callbacks<T>,
    {
        self.start(start_node);

        while let Some(node) = self.open.pop() {
            self.step_dijkstra(node, callbacks)?;
        }
        Err(PathError::Exhausted)
    }

    pub fn astar<C>(&mut self, callbacks: &mut C, start_node: T) -> Result<(), PathError>
    where
        C: Callbacks<T>,
    {
        self.start(start_node);

        while let Some(node) = self.open.pop() {
            self.step_astar(node, callbacks)?;
        }
        Err(PathError::Exhausted)
    }

    pub fn bfs<C>(&mut self, callbacks: &mut C, start_node: T) -> Result<(), PathError>
    where
        C: Callbacks<T>,
    {
        self.start(start_node);

        while let Some(node) = self.open_deq.pop_front() {
            self.step_bfs_dfs(node, callbacks)?;
        }
        Err(PathError::Exhausted)
    }

    pub fn dfs<C>(&mut self, callbacks: &mut C, start_node: T) -> Result<(), PathError>
    where
        C: Callbacks<T>,
    {
        self.start(start_node);

        while let Some(node) = self.open_deq.pop_back() {
            self.step_bfs_dfs(node, callbacks)?;
        }
        Err(PathError::Exhausted)
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
