use itertools::Itertools;

#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq,
{
    pub arena: Vec<Node<T>>,
    pub current_node: usize,
}

#[derive(Debug)]
pub struct Node<T>
where
    T: PartialEq,
{
    pub idx: usize,
    pub val: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub descendant_files: Vec<usize>,
    pub size_on_disk: u64,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
            descendant_files: vec![],
            size_on_disk: 0,
        }
    }
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    pub fn node(&mut self, val: T) -> usize {
        //return node if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        //otherwise add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }

    pub fn current(&self) -> &Node<T> {
        &self.arena[self.current_node]
    }

    pub fn set_current(&mut self, idx: usize) {
        self.current_node = idx;
    }

    // pub fn remove_node(&mut self, idx: usize) {
    //     // disconnects node from parent
    //     // * doesn't drop nodes from arena,
    //     // as this would affect indexes in remaining nodes.

    //     // 1. unlink node from parent
    //     if let Some(pidx) = self.arena[idx].parent {
    //         let mut parent = &mut self.arena[pidx];
    //         let mut ch_idx: usize;
    //         for (chidx, ch) in parent.children.clone().into_iter().enumerate() {
    //             if ch == idx {
    //                 ch_idx = chidx;
    //                 parent.children.remove(ch_idx);
    //                 break;
    //             }
    //         }
    //         self.arena[idx].parent = None;
    //     }
    // }

    pub fn collect_decendants(&self, idx: usize) -> Vec<usize> {
        println!("entering collect_decendants {}", idx);

        self.arena
            .iter()
            .map(|n| n.idx)
            .filter(|n_idx| self.depth_to_target(idx, *n_idx).is_some())
            .collect()
    }

    pub fn size(&self) -> usize {
        self.arena.len()
    }

    pub fn edges(&self) -> usize {
        self.arena
            .iter()
            .fold(0, |acc, node| acc + node.children.len())
    }

    pub fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }

    fn depth_to_target(&self, idx: usize, target_idx: usize) -> Option<usize> {
        // how deep is the target
        // is the target a decendant of idx?

        // are we here yet? therefore, Some(0) distance
        if target_idx == idx {
            return Some(0);
        }
        let mut acc = 0;
        let mut p_idx = target_idx;
        while let Some(candidate_idx) = self.arena[p_idx].parent {
            acc += 1;
            if candidate_idx == idx {
                return Some(acc);
            }
            p_idx = candidate_idx;
        }

        // Target wasn't a decendant
        None
    }
}
