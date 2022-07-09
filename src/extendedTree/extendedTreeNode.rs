use oxigraph::storage::numeric_encoder::StrHash;
use std::cell::{RefCell, RefMut, Ref};
use std::fmt::Result;
use std::rc::{Rc};

use super::IntervalNode;

// 子节点的区间编码实质由其父节点决定，以此迭代
// 所以将父节点与其区间编码信息绑定在一起，以此形式来适应多继承
// 在获取编码的时候,对其区间编码节点(interval_node)进行迭代
#[derive(Debug)]
pub struct MultiTreeNode {
    childs: RefCell<Vec<Rc<MultiTreeNode>>>,
    data: StrHash,
    interval_node: RefCell<Vec<Rc<IntervalNode>>>,
    index: u32
}

impl MultiTreeNode {
    pub fn new(data: &str) -> Self {
        Self {
            childs: RefCell::new(Vec::new()),
            data: StrHash::new(data),
            interval_node: RefCell::new(Vec::new()),
            index: u32::default()
        }
    }

    pub fn get_data(&self) -> StrHash {
        self.data
    }

    // 先检查是否已添加该子节点，否则返回Err，表示添加不成功
    pub fn add_child(&self, child_node: Rc<MultiTreeNode>) -> bool {
        {
            let vec = &*(self.childs.borrow());
            for child in vec {
                if (*child).data == child_node.get_data() {
                    return false;
                }
            }
        }

        self.childs.borrow_mut().push(child_node);
        true
    }

    // 找到某个child在childs中的下标以供进行删除
    pub fn find_child_index(&self, child_strhash: StrHash) -> usize {
        for (index, node) in self.childs.borrow().iter().enumerate() {
            if (*node).get_data() == child_strhash{
                return index;
            }
        }
        usize::MAX
    }

    pub fn remove_child(&self, child_strhash: StrHash){
        let index = self.find_child_index(child_strhash);
        if  index != usize::MAX {
            self.childs.borrow_mut().remove(index);
        }
    }

    pub fn find_parent_index(&self, parent_strhash: StrHash) -> usize {
        for (index,node) in self.interval_node.borrow().iter().enumerate() {
            if (*node).get_parent().unwrap().get_data() == parent_strhash {
                return index;
            }
        }

        usize::MAX
    }

    pub fn remove_parent(&self, parent_strhash: StrHash) {
        let index = self.find_parent_index(parent_strhash);
        if index != usize::MAX {
            self.interval_node.borrow_mut().remove(index);
        }
    }

    // 得到IntervalNode的vec列表
    pub fn get_interval_nodes(&self) -> Ref<Vec<Rc<IntervalNode>>>{
        self.interval_node.borrow()
    }

    // 子节点列表（注意不可更改），要更改子节点的操作应该直接在结构体内部进行更改而不能在结构体外部更改
    pub fn get_childs(&self) -> Ref<Vec<Rc<MultiTreeNode>>> {
        self.childs.borrow()
    }

    // 添加父节点
    pub fn add_parent(&self, parent: Rc<MultiTreeNode>) {
        self.interval_node.borrow_mut().push(Rc::new(IntervalNode::new(parent)));
    }

    pub fn count_parents(&self) -> usize {
        self.interval_node.borrow().len()
    }

    // 判断该节点是否含有某父节点
    pub fn if_exist_parent(&self, parent: Rc<MultiTreeNode>) -> bool {
        for interval in self.interval_node.borrow().iter() {
            match interval.get_parent() {
                Ok(node) => {
                    if node.get_data() == parent.get_data() {
                        return true;
                    }
                },
                Err(()) => return false
            }
        }

        false
    }

    // 找到第一个还未被编码的父节点
    pub fn find_first_unencoded_parent(&self) -> usize {
        for (index, interval) in self.interval_node.borrow().iter().enumerate() {
            if (*interval).get_start() == 0 {
                return index;
            }
        }

        usize::MAX
    }
}