use super::machine::instr::*;
use crate::ir::types::*;
use std::fmt;
use std::{cell::RefCell, rc::Rc};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PhysReg(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct VirtReg(pub usize);

#[derive(Debug, Clone)]
pub struct VirtRegGen {
    id: Rc<RefCell<usize>>,
}

// START: x64 dependent code
// TODO: THIS CODE WILL BE AUTO-GENERATED BY MACRO IN THE FUTURE

const GR32_NUM: isize = 16;
const GR64_NUM: isize = GR32_NUM;

// Remember to fix PhysReg::reg_class() when appending a variant
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum RegisterClassKind {
    GR32 = 0,
    GR64 = GR32_NUM,
    XMM = GR32_NUM + GR64_NUM,
}

// TODO: TEMPORARY FUNCTIONS. WILL BE REMOVED.
pub fn ty2rc(ty: &Type) -> Option<RegisterClassKind> {
    match ty {
        Type::Void => None,
        Type::Int32 => Some(RegisterClassKind::GR32),
        Type::Int64 => Some(RegisterClassKind::GR64),
        Type::F64 => Some(RegisterClassKind::XMM),
        Type::Pointer(_) => Some(RegisterClassKind::GR64),
        Type::Array(_) => Some(RegisterClassKind::GR64),
        e => unimplemented!("{:?}", e),
    }
}

pub fn rc2ty(rc: RegisterClassKind) -> Type {
    match rc {
        RegisterClassKind::GR32 => Type::Int32,
        RegisterClassKind::GR64 => Type::Int64,
        RegisterClassKind::XMM => Type::F64,
    }
}

impl PhysReg {
    pub fn reg_class(&self) -> RegisterClassKind {
        // TODO
        let n = self.retrieve();
        if RegisterClassKind::GR32 as usize <= n && n < RegisterClassKind::GR64 as usize {
            return RegisterClassKind::GR32;
        } else if RegisterClassKind::GR64 as usize <= n && n < RegisterClassKind::XMM as usize {
            return RegisterClassKind::GR64;
        }
        RegisterClassKind::XMM
    }
}

macro_rules! to_phys {
    ($($r:path),*) => {
        vec![$($r),*].iter().map(|r| r.as_phys_reg()).collect()
    };
}

impl RegisterClassKind {
    pub fn get_reg_order(&self) -> RegisterOrder {
        RegisterOrder::general_purpose(*self)
    }

    pub fn get_arg_reg_order(&self) -> RegisterOrder {
        RegisterOrder::arguments(*self)
    }

    pub fn get_nth_arg_reg(&self, nth: usize) -> Option<PhysReg> {
        self.get_arg_reg_order_vec().get(nth).map(|r| *r)
    }

    pub fn size_in_bits(&self) -> usize {
        match self {
            Self::GR32 => 32,
            Self::GR64 => 64,
            Self::XMM => 128,
        }
    }

    pub fn size_in_byte(&self) -> usize {
        self.size_in_bits() / 8
    }

    pub fn shares_same_register_file(&self, rc: RegisterClassKind) -> bool {
        self.register_file_base_class() == rc.register_file_base_class()
    }

    pub fn register_file_base_class(&self) -> RegisterClassKind {
        match self {
            Self::GR32 | Self::GR64 => RegisterClassKind::GR32,
            Self::XMM => RegisterClassKind::XMM,
        }
    }

    // Returns normal order of registers used to pass arguments
    // TODO: This is System V AMD64 ABI.
    // https://en.wikipedia.org/wiki/X86_calling_conventions#System_V_AMD64_ABI
    pub fn get_arg_reg_order_vec(&self) -> Vec<PhysReg> {
        match self {
            RegisterClassKind::GR32 => to_phys!(
                GR32::EDI,
                GR32::ESI,
                GR32::EDX,
                GR32::ECX,
                GR32::R8D,
                GR32::R9D
            ),
            RegisterClassKind::GR64 => to_phys!(
                GR64::RDI,
                GR64::RSI,
                GR64::RDX,
                GR64::RCX,
                GR64::R8,
                GR64::R9
            ),
            RegisterClassKind::XMM => to_phys!(
                XMM::XMM0,
                XMM::XMM1,
                XMM::XMM2,
                XMM::XMM3,
                XMM::XMM4,
                XMM::XMM5,
                XMM::XMM6,
                XMM::XMM7
            ),
        }
    }

    // Returns normal order of general-purpose registers
    pub fn get_gp_reg_order_vec(&self) -> Vec<PhysReg> {
        match self {
            RegisterClassKind::GR32 => to_phys!(
                GR32::EAX,
                GR32::ECX,
                GR32::EDX,
                GR32::ESI,
                GR32::EDI,
                GR32::R8D,
                GR32::R9D,
                GR32::R10D,
                GR32::R11D
                // TODO: following 4 regs need to be saved if changed
                // GR32::R12D,
                // GR32::R13D,
                // GR32::R14D,
                // GR32::R15D,
            ),
            RegisterClassKind::GR64 => to_phys!(
                GR64::RAX,
                GR64::RCX,
                GR64::RDX,
                GR64::RSI,
                GR64::RDI,
                GR64::R8,
                GR64::R9,
                GR64::R10,
                GR64::R11
                // TODO: following 4 regs need to be saved if changed
                // GR64::R12,
                // GR64::R13,
                // GR64::R14,
                // GR64::R15,
            ),
            RegisterClassKind::XMM => to_phys!(
                XMM::XMM0,
                XMM::XMM1,
                XMM::XMM2,
                XMM::XMM3,
                XMM::XMM4,
                XMM::XMM5,
                XMM::XMM6,
                XMM::XMM7,
                XMM::XMM8,
                XMM::XMM9,
                XMM::XMM10,
                XMM::XMM11,
                XMM::XMM12,
                XMM::XMM13,
                XMM::XMM14,
                XMM::XMM15
            ),
        }
    }

    pub fn return_value_register(&self) -> PhysReg {
        match self {
            Self::GR32 => GR32::EAX.as_phys_reg(),
            Self::GR64 => GR64::RAX.as_phys_reg(),
            Self::XMM => XMM::XMM0.as_phys_reg(),
        }
    }
}

// TODO: The definition of GR32 is now hard coded in ROOT/defs/src/register.rs
use defs::define_registers;
define_registers!(
    RegisterClass GR32 (i32) {
    }
);

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum GR64 {
    RAX,
    RBX,
    RCX,
    RDX,
    RDI,
    RSI,
    RBP,
    RSP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum XMM {
    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
    XMM8,
    XMM9,
    XMM10,
    XMM11,
    XMM12,
    XMM13,
    XMM14,
    XMM15,
}

pub trait TargetRegisterTrait: Copy + Clone {
    fn as_phys_reg(&self) -> PhysReg;
}

impl TargetRegisterTrait for PhysReg {
    fn as_phys_reg(&self) -> PhysReg {
        *self
    }
}

// register nubmering: https://corsix.github.io/dynasm-doc/instructions.html#registers

impl TargetRegisterTrait for GR32 {
    fn as_phys_reg(&self) -> PhysReg {
        let n = match self {
            GR32::EAX => 0,
            GR32::ECX => 1,
            GR32::EDX => 2,
            GR32::EBX => 3,
            GR32::ESP => 4,
            GR32::EBP => 5,
            GR32::ESI => 6,
            GR32::EDI => 7,
            GR32::R8D => 8,
            GR32::R9D => 9,
            GR32::R10D => 10,
            GR32::R11D => 11,
            GR32::R12D => 12,
            GR32::R13D => 13,
            GR32::R14D => 14,
            GR32::R15D => 15,
        };
        PhysReg(n + RegisterClassKind::GR32 as usize)
    }
}

impl TargetRegisterTrait for GR64 {
    fn as_phys_reg(&self) -> PhysReg {
        let n = match self {
            GR64::RAX => 0,
            GR64::RCX => 1,
            GR64::RDX => 2,
            GR64::RBX => 3,
            GR64::RSP => 4,
            GR64::RBP => 5,
            GR64::RSI => 6,
            GR64::RDI => 7,
            GR64::R8 => 8,
            GR64::R9 => 9,
            GR64::R10 => 10,
            GR64::R11 => 11,
            GR64::R12 => 12,
            GR64::R13 => 13,
            GR64::R14 => 14,
            GR64::R15 => 15,
        };
        PhysReg(n + RegisterClassKind::GR64 as usize)
    }
}

impl TargetRegisterTrait for XMM {
    fn as_phys_reg(&self) -> PhysReg {
        let n = match self {
            XMM::XMM0 => 0,
            XMM::XMM1 => 1,
            XMM::XMM2 => 2,
            XMM::XMM3 => 3,
            XMM::XMM4 => 4,
            XMM::XMM5 => 5,
            XMM::XMM6 => 6,
            XMM::XMM7 => 7,
            XMM::XMM8 => 8,
            XMM::XMM9 => 9,
            XMM::XMM10 => 10,
            XMM::XMM11 => 11,
            XMM::XMM12 => 12,
            XMM::XMM13 => 13,
            XMM::XMM14 => 14,
            XMM::XMM15 => 15,
        };
        PhysReg(n + RegisterClassKind::XMM as usize)
    }
}

// END: x64 dependent code

#[derive(Debug, Clone)]
pub struct RegisterOrder {
    order: Vec<PhysReg>,
    nth: usize,
    reg_class: RegisterClassKind,
}

impl RegisterOrder {
    pub fn general_purpose(reg_class: RegisterClassKind) -> Self {
        Self {
            reg_class,
            order: reg_class.get_gp_reg_order_vec(),
            nth: 0,
        }
    }

    pub fn arguments(reg_class: RegisterClassKind) -> Self {
        Self {
            reg_class,
            order: reg_class.get_arg_reg_order_vec(),
            nth: 0,
        }
    }
}

impl Iterator for RegisterOrder {
    type Item = PhysReg;

    fn next(&mut self) -> Option<Self::Item> {
        self.nth += 1;
        self.order.get(self.nth - 1).and_then(|item| Some(*item))
    }
}

impl VirtRegGen {
    pub fn new() -> Self {
        Self {
            id: Rc::new(RefCell::new(0)),
        }
    }

    pub fn gen_vreg(&self, rc: RegisterClassKind) -> RegisterInfo {
        let mut reg = RegisterInfo::new(rc);
        reg.set_vreg(VirtReg(self.next_id()));
        reg
    }

    fn next_id(&self) -> usize {
        let mut id = self.id.borrow_mut();
        *id += 1;
        *id
    }

    pub fn next_vreg(&self) -> VirtReg {
        VirtReg(self.next_id())
    }
}

impl PhysReg {
    pub fn retrieve(&self) -> usize {
        self.0
    }

    pub fn name(&self) -> &str {
        let reg_names = [
            "eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi", "r8d", "r9d", "r10d", "r11d",
            "r12d", "r13d", "r14d", "r15d", "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi",
            "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15", "xmm0", "xmm1", "xmm2", "xmm3",
            "xmm4", "xmm5", "xmm6", "xmm7", "xmm8", "xmm9", "xmm10", "xmm11", "xmm12", "xmm13",
            "xmm14", "xmm15",
        ];
        reg_names[self.retrieve()]
    }
}

impl VirtReg {
    pub fn retrieve(&self) -> usize {
        self.0
    }
}

pub fn str2reg(s: &str) -> Option<PhysReg> {
    Some(match s.to_ascii_lowercase().as_str() {
        "eax" => GR32::EAX.as_phys_reg(),
        "ecx" => GR32::ECX.as_phys_reg(),
        "edx" => GR32::EDX.as_phys_reg(),
        "ebx" => GR32::EBX.as_phys_reg(),
        "esp" => GR32::ESP.as_phys_reg(),
        "ebp" => GR32::EBP.as_phys_reg(),
        "esi" => GR32::ESI.as_phys_reg(),
        "edi" => GR32::EDI.as_phys_reg(),
        "r8d" => GR32::R8D.as_phys_reg(),
        "r9d" => GR32::R9D.as_phys_reg(),
        "r10d" => GR32::R10D.as_phys_reg(),
        "r11d" => GR32::R11D.as_phys_reg(),
        "r12d" => GR32::R12D.as_phys_reg(),
        "r13d" => GR32::R13D.as_phys_reg(),
        "r14d" => GR32::R14D.as_phys_reg(),
        "r15d" => GR32::R15D.as_phys_reg(),
        "rax" => GR64::RAX.as_phys_reg(),
        "rcx" => GR64::RCX.as_phys_reg(),
        "rdx" => GR64::RDX.as_phys_reg(),
        "rbx" => GR64::RBX.as_phys_reg(),
        "rsp" => GR64::RSP.as_phys_reg(),
        "rbp" => GR64::RBP.as_phys_reg(),
        "rsi" => GR64::RSI.as_phys_reg(),
        "rdi" => GR64::RDI.as_phys_reg(),
        "r8" => GR64::R8.as_phys_reg(),
        "r9" => GR64::R9.as_phys_reg(),
        "r10" => GR64::R10.as_phys_reg(),
        "r11" => GR64::R11.as_phys_reg(),
        "r12" => GR64::R12.as_phys_reg(),
        "r13" => GR64::R13.as_phys_reg(),
        "r14" => GR64::R14.as_phys_reg(),
        "r15" => GR64::R15.as_phys_reg(),
        "xmm0" => XMM::XMM0.as_phys_reg(),
        "xmm1" => XMM::XMM1.as_phys_reg(),
        "xmm2" => XMM::XMM2.as_phys_reg(),
        "xmm3" => XMM::XMM3.as_phys_reg(),
        "xmm4" => XMM::XMM4.as_phys_reg(),
        "xmm5" => XMM::XMM5.as_phys_reg(),
        "xmm6" => XMM::XMM6.as_phys_reg(),
        "xmm7" => XMM::XMM7.as_phys_reg(),
        "xmm8" => XMM::XMM8.as_phys_reg(),
        "xmm9" => XMM::XMM9.as_phys_reg(),
        "xmm10" => XMM::XMM10.as_phys_reg(),
        "xmm11" => XMM::XMM11.as_phys_reg(),
        "xmm12" => XMM::XMM12.as_phys_reg(),
        "xmm13" => XMM::XMM13.as_phys_reg(),
        "xmm14" => XMM::XMM14.as_phys_reg(),
        "xmm15" => XMM::XMM15.as_phys_reg(),
        _ => return None,
    })
}

impl fmt::Debug for PhysReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.name())
    }
}

impl fmt::Debug for VirtReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%vreg{}", self.retrieve())
    }
}
