use oxigraph::storage::numeric_encoder::StrHash;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::{Rc};
use std::cell::{RefCell};

use super::{MultiTreeNode};

#[derive(Clone)]
pub struct MultiTree{
    root: StrHash,
    hash_str_node: RefCell<HashMap<StrHash, Rc<MultiTreeNode>>>,   // StrHash -> 节点
    parent_way: RefCell<Vec<Vec<StrHash>>>,
    hash_parent_by_str: RefCell<HashMap<StrHash, Vec<Vec<StrHash>>>>
}

impl MultiTree {
    pub fn new(data: &str) -> Self {
        let root_strhash = StrHash::new(data);

        let mut hash = HashMap::new();
        hash.insert(root_strhash, Rc::new(MultiTreeNode::new(data)));

        Self {
            root: root_strhash,
            hash_str_node: RefCell::new(hash),
            hash_parent_by_str: RefCell::new(HashMap::new()),
            parent_way: RefCell::new(Vec::new()),
        }
    }

    pub fn is_root(&self, other: Rc<MultiTreeNode>) -> bool {
        if self.root == other.get_data() {
            true
        } else {
            false
        }
    }

    pub fn get_root(&self) -> Rc<MultiTreeNode> {
        Rc::clone(self.hash_str_node.borrow().get(&self.root).unwrap())
    }

    // 1、先判断父节点先前是否存在，若不存在，则父节点的父节点是root，将其添加进root的孩子中
    // 2、父节点添加子节点时，不会重复添加子节点
    // 3、如果子节点在root的子节点中，应将其从其中去掉；并且子节点也应去掉root父节点
    // 4、最后在由树维护的节点hash中插入父节点与子节点
    pub fn insert(&self, child_str: &str, parent_str: &str) -> bool {
        let if_parent_exist = self.if_exist(parent_str);   
        let mut parent_contain_root = false;

        let child = self.construct_node(child_str);
        let parent = self.construct_node(parent_str);

        if let true = parent.add_child(Rc::clone(&child)) {   
            for interval in &*(child.get_interval_nodes()) {   
                if let Ok(parent) = (*interval).get_parent() {
                    if parent.get_data() == self.get_root().get_data() {
                        self.get_root().remove_child(child.get_data());
                        parent_contain_root = true;
                    }
                }
            }

            if parent_contain_root {
                child.remove_parent(self.get_root().get_data());
            }

            if !if_parent_exist {   
                self.get_root().add_child(Rc::clone(&parent));
                parent.add_parent(Rc::clone(&self.get_root()));
            }

            child.add_parent(Rc::clone(&parent));

            self.hash_str_node.borrow_mut().insert(parent.get_data(), Rc::clone(&parent));
            self.hash_str_node.borrow_mut().insert(child.get_data(), Rc::clone(&child));

            return true;
        }

        false
    }

    // 根据str获得其后代节点的数量
    pub fn count_childs_by_str(&self, node_str: &str) -> Result<u32, ()> {
        if let Some(link_node) = self.hash_str_node.borrow().get(&StrHash::new(node_str)) {
            if link_node.get_childs().len() == 0 {
                return Ok(0u32);
            }

            let mut stack: Vec<Rc<MultiTreeNode>> = Vec::new();
            stack.push(Rc::clone(&link_node));

            let mut start = 0u32;

            while !stack.is_empty() {
                let node = stack.pop().unwrap();

                for child in node.get_childs().iter().rev(){
                    stack.push(Rc::clone(child));
                }

                start = start + 1u32;
            }

            Ok(start-1u32)
        } else {
            Err(())
        }
    }

    // 根据str获得其父节点个数
    pub fn count_parents_by_str(&self, node_str: &str) -> Result<usize, ()> {
        match self.hash_str_node.borrow().get(&StrHash::new(node_str)) {
            Some(node) => {Ok(node.get_interval_nodes().len())},
            None => Err(())
        }
    }

    // 判断树里是否存在某个节点
    pub fn if_exist(&self, value: &str) -> bool {
        self.hash_str_node.borrow().contains_key(&StrHash::new(value))
    }

    // 根据 strhash 获得节点
    pub fn get_node_by_strhash(&self, strhash: StrHash) -> Result<Rc<MultiTreeNode>,()> {
        match self.hash_str_node.borrow().get(&strhash) {
            Some(node) => Ok(Rc::clone(node)),
            None => Err(())
        }
    }

    // 对树进行编码
    pub fn encode(&self) {
        self.initial_root();

        let root = self.get_root();
        let root_parent = self.get_root().get_interval_nodes().get(0).unwrap().get_parent().unwrap();

        self.tao();

        self.parent_way_by_strhash();
    }

    // 其为私有方法，以保证插入过程可以正常进行下去
    fn construct_node(&self, value: &str) -> Rc<MultiTreeNode> {
        if !self.if_exist(value) {
            let treenode = Rc::new(MultiTreeNode::new(value));
            self.hash_str_node.borrow_mut().insert(treenode.get_data(), treenode);
        }

        Rc::clone(self.hash_str_node.borrow().get(&StrHash::new(value)).unwrap())
    }

    // TODO：计算某节点的后代节点数
    // 多继承节点的子节点会被重复计算
    pub fn count_childs(&self, node: Rc<MultiTreeNode>) -> u32 {
        if node.get_childs().len() == 0 {
            return 0;
        }

        let mut stack: Vec<Rc<MultiTreeNode>> = Vec::new();
        stack.push(Rc::clone(&node));

        let mut count: u32 = 0;

        while !stack.is_empty() {
            let node = stack.pop().unwrap();

            for child in node.get_childs().iter().rev(){
                stack.push(Rc::clone(child));
            }

            count = count + 1;
        }

        count-1
    }

    pub fn tao(&self) {
        let mut count = 0;
        let mut way:Vec<StrHash> = Vec::new();
        self.recursive(self.get_root(), self.get_root().get_interval_nodes().borrow().get(0).unwrap().get_parent().unwrap(), count, 1u16, way);
    }

    pub fn recursive(&self, current_node: Rc<MultiTreeNode>, parent: Rc<MultiTreeNode>, count: u32, layer: u16, parent_way: Vec<StrHash>) -> u32{
        let mut current = count + 1;   // 区间编码的左边界

        let mut way: Vec<StrHash> = Vec::new();
        way.push(current_node.get_data());

        for node in &parent_way {
            if *node != self.get_root().get_data() {
                way.push(*node);
            }
        }

        self.parent_way.borrow_mut().push(way.clone());
 
        for child in current_node.get_childs().iter() {
            current = self.recursive(Rc::clone(child), Rc::clone(&current_node), current, layer+1, way.clone());
        }
    
        for interval in current_node.get_interval_nodes().iter() {
            if interval.get_parent().unwrap().get_data() == parent.get_data() {
                interval.set_start(count+1);
                interval.set_end(current);
                interval.set_layer(layer);
            }
        }

        return current;
    }

    // 形成节点到根节点的路径（从根节点自顶向下）
    pub fn parent_way_by_strhash(&self) {
        for way in self.parent_way.borrow().iter() {      
            (*self.hash_parent_by_str.borrow_mut().entry(*(way.get(0).unwrap())).or_insert(Vec::new())).push(way.clone());
        }
    }

    pub fn get_parent_way_by_str(&self, strhash: StrHash) -> Result<Vec<Vec<StrHash>>,()> {
        if self.hash_parent_by_str.borrow().contains_key(&strhash) {
            Ok(self.hash_parent_by_str.borrow().get(&strhash).unwrap().clone())            
        } else {
            Err(())
        }
    }

    pub fn initial_root(&self) {
        // 设置根节点的父节点以及其编码
        self.get_root().add_parent(self.construct_node("root_parent"));
    }
}