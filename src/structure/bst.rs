use core::borrow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        }
        // empty right child case
        else {
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None
        }
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node) != true {
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node)
            && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone())
        {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap());
    }

    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }

    pub fn tree_insert(root: &BstNodeLink, value: i32) -> bool {
        let c_value = root.borrow().key.unwrap();

        if value == c_value {
            return false;
        }

        if value < c_value {
            let left_opt = root.borrow().left.clone();
            match left_opt {
                Some(left_node) => {
                    return BstNode::tree_insert(&left_node, value);
                }
                None => {
                    let new_node = BstNode::new_with_parent(root, value);
                    root.borrow_mut().left = Some(new_node);
                    return true;
                }
            }
        } else {
            let right_opt = root.borrow().right.clone();
            match right_opt {
                Some(right_node) => {
                    return BstNode::tree_insert(&right_node, value);
                }
                None => {
                    let new_node = BstNode::new_with_parent(root, value);
                    root.borrow_mut().right = Some(new_node);
                    return true;
                }
            }
        }
    }

    pub fn transplant(
        parent_opt: Option<Rc<RefCell<BstNode>>>,
        old_node: &Rc<RefCell<BstNode>>,
        new_node: Option<Rc<RefCell<BstNode>>>,
    ) {
        if let Some(parent_rc) = parent_opt.clone() {
            let mut parent = parent_rc.borrow_mut();

            if let Some(ref left) = parent.left {
                if Rc::ptr_eq(left, old_node) {
                    parent.left = new_node.clone();
                }
            }

            if let Some(ref right) = parent.right {
                if Rc::ptr_eq(right, old_node) {
                    parent.right = new_node.clone();
                }
            }

            // If new node exists, set its parent
            if let Some(child_rc) = new_node {
                child_rc.borrow_mut().parent = Some(Rc::downgrade(&parent_rc));
            }
        }
    }

    pub fn tree_delete(node: &BstNodeLink, value: i32) {
        let (node_key, parent_opt, left_opt, right_opt) = {
            let node_ref = node.borrow();
            (
                node_ref.key.clone().unwrap(),
                node_ref.parent.clone().and_then(|p| p.upgrade()),
                node_ref.left.clone(),
                node_ref.right.clone(),
            )
        };

        // Traverse from root
        if &value != &node_key {
            // Traverse the tree
            if value < node_key {
                if let Some(left) = left_opt {
                    BstNode::tree_delete(&left, value);
                }
            } else {
                if let Some(right) = right_opt {
                    BstNode::tree_delete(&right, value);
                }
            }
            return;
        }

        match (left_opt.clone(), right_opt.clone()) {
            // Case 1: No children (leaf node)
            (None, None) => {
                if let Some(parent_rc) = parent_opt {
                    let mut parent = parent_rc.borrow_mut();
                    if let Some(left) = &parent.left {
                        if Rc::ptr_eq(left, node) {
                            parent.left = None;
                        }
                    }
                    if let Some(right) = &parent.right {
                        if Rc::ptr_eq(right, node) {
                            parent.right = None;
                        }
                    }
                }
            }

            (Some(child), None) | (None, Some(child)) => {
                BstNode::transplant(parent_opt.clone(), node, Some(child.clone()));
            }

            // Still in progress, not working as expected.
            (Some(_), Some(_)) => {
                if let Some(successor_rc) = BstNode::tree_successor(node) {
                    let mut successor = successor_rc.borrow_mut();

                    // Successor might have a right child, transplant it
                    let successor_parent = successor.parent.as_ref().and_then(|wp| wp.upgrade());
                    let successor_right = successor.right.clone(); // Might be None

                    drop(successor); // Drop borrow to avoid double borrow

                    if !Rc::ptr_eq(&successor_rc, &node.borrow().right.as_ref().unwrap()) {
                        // Transplant successor with its right child (only case possible)
                        BstNode::transplant(
                            successor_parent.clone(),
                            &successor_rc,
                            successor_right.clone(),
                        );

                        // Set successor's right to current node's right
                        successor_rc.borrow_mut().right = node.borrow().right.clone();
                        if let Some(ref right_rc) = successor_rc.borrow().right {
                            right_rc.borrow_mut().parent = Some(Rc::downgrade(&successor_rc));
                        }
                    }

                    // Transplant current node with successor
                    BstNode::transplant(parent_opt.clone(), node, Some(successor_rc.clone()));

                    // Set successor's left to current node's left
                    successor_rc.borrow_mut().left = node.borrow().left.clone();
                    if let Some(ref left_rc) = successor_rc.borrow().left {
                        left_rc.borrow_mut().parent = Some(Rc::downgrade(&successor_rc));
                    }
                }
            }
        }
    }

}
