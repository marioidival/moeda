use primitive::Type;
use std::collections::HashMap;

pub struct FrameStack {
    stack: Vec<Frame>,
}
impl FrameStack {
    pub fn new() -> Self {
        FrameStack {
            stack: vec![Frame::new()],
        }
    }
    // Frame stack operations
    pub fn current(&mut self) -> &mut Frame {
        let stack_size = self.stack.len();
        &mut self.stack[stack_size - 1]
    }

    pub fn push(&mut self, block_scope: Frame) {
        self.stack.push(block_scope)
    }

    pub fn pop(&mut self) -> Frame {
        let old = self.stack.pop().unwrap();
        for (k, v) in old.locals.iter() {
            if self.current().has(k) {
                self.current().locals.insert(k.clone(), v.clone());
            }
        }
        old
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub iparents: HashMap<String, Type>,
    pub parents: HashMap<String, Type>,
    pub ilocals: HashMap<String, Type>,
    pub locals: HashMap<String, Type>,
}
impl Frame {
    pub fn new() -> Self {
        Frame {
            locals: HashMap::new(),
            parents: HashMap::new(),
            ilocals: HashMap::new(),
            iparents: HashMap::new(),
        }
    }

    pub fn has(&self, id: &str) -> bool {
        self.ilocals.contains_key(id) || self.locals.contains_key(id)
    }

    pub fn is_imutable(&self, id: &str) -> bool {
        self.ilocals.contains_key(id)
    }

    pub fn get(&self, id: &str) -> Option<Type> {
        // parenfunctionst
        if let Some(value) = self.iparents.get(&*id) {
            return Some(value.clone());
        };
        if let Some(value) = self.parents.get(&*id) {
            return Some(value.clone());
        };
        // current
        if let Some(value) = self.ilocals.get(&*id) {
            return Some(value.clone());
        };
        if let Some(value) = self.locals.get(&*id) {
            Some(value.clone())
        } else {
            None
        }
    }
}
