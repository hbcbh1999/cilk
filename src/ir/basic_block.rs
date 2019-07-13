use super::{module::*, opcode::*, value::*};
use id_arena::*;
use rustc_hash::FxHashSet;
use std::{
    collections::LinkedList,
    {
        cell::{Ref, RefCell, RefMut},
        rc::Rc,
    },
};

pub type BasicBlockId = Id<BasicBlock>;

#[derive(Clone, Debug)]
pub struct BasicBlock {
    /// Information for liveness analysis
    pub liveness: Rc<RefCell<LivenessInfo>>,

    /// Predecessors
    pub pred: Vec<BasicBlockId>,

    /// Successors
    pub succ: Vec<BasicBlockId>,

    /// Instruction list
    pub iseq: Rc<RefCell<LinkedList<Value>>>,
}

#[derive(Clone, Debug)]
pub struct LivenessInfo {
    pub def: FxHashSet<InstructionId>,
    pub live_in: FxHashSet<InstructionId>,
    pub live_out: FxHashSet<InstructionId>,
}

impl BasicBlock {
    pub fn new() -> Self {
        Self {
            iseq: Rc::new(RefCell::new(LinkedList::new())),
            pred: vec![],
            succ: vec![],
            liveness: Rc::new(RefCell::new(LivenessInfo::new())),
        }
    }

    pub fn iseq_ref<'b>(&'b self) -> Ref<LinkedList<Value>> {
        self.iseq.borrow()
    }

    pub fn iseq_ref_mut(&self) -> RefMut<LinkedList<Value>> {
        self.iseq.borrow_mut()
    }

    pub fn to_string(&self, m: &Module) -> String {
        self.iseq_ref().iter().fold("".to_string(), |s, instr| {
            format!("{}{}\n", s, instr.to_string(m, true))
        })
    }
}

impl LivenessInfo {
    pub fn new() -> Self {
        Self {
            def: FxHashSet::default(),
            live_in: FxHashSet::default(),
            live_out: FxHashSet::default(),
        }
    }
}
