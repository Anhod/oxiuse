use std::rc::{Rc, Weak};
use std::cell::RefCell;

use super::MultiTreeNode;

// RefCell：内部可变性   Rc：引用计数
// 实现一个节点可以有对应的多个区间编码以及父类
#[derive(Debug)]
pub struct IntervalNode {
    start: RefCell<u32>, 
    end: RefCell<u32>,
    layer: RefCell<u16>,

    parent: RefCell<Weak<MultiTreeNode>>
}

impl IntervalNode {
    pub fn new(parent: Rc<MultiTreeNode>) -> Self {
        Self {
            start: RefCell::new(u32::default()),
            end: RefCell::new(u32::default()),
            layer: RefCell::new(u16::default()),

            parent: RefCell::new(Rc::downgrade(&parent))
        }
    }

    // 不知道能否保持一致性（直接返回Rc父节点）
    pub fn get_parent(&self) -> Result<Rc<MultiTreeNode>,()>{
        match self.parent.borrow_mut().upgrade(){
            Some(value) => Ok(Rc::clone(&value)),
            None => Err(())
        }
    }

    pub fn set_start(&self, start: u32) {
        *self.start.borrow_mut() = start;
    }

    pub fn get_start(&self) -> u32 {
        *(self.start.borrow())
    }

    pub fn set_end(&self, end: u32) {
        *self.end.borrow_mut() = end;
    }

    pub fn get_end(&self) -> u32 {
        *(self.end.borrow())
    }

    pub fn set_layer(&self, layer: u16) {
        *self.layer.borrow_mut() = layer;
    }

    pub fn get_layer(&self) -> u16 {
        *(self.layer.borrow())
    }
}