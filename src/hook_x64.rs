const MAX_INSN_LEN: usize = 15;

pub type JmpBackRoutine = unsafe extern "win64" fn(regs: *mut Any, user_data: usize);

#[derive(Debug, Clone)]
pub enum HookType {
    JmpBack(JmpBackRoutine),
    Ret(0),
    JmpToAddr(usize, 0),
    JmpToRet(0),
}

#[derive(Debug, Clone)]
pub enum JmpType {
    RipRelative,
    Direct(*mut u8, usize),
    DirectWithRipRelative(usize)
}


/// Represents different types of jump instructions and hook types for a system.
/// 
/// `MAX_INSN_LEN` defines the maximum instruction length.
/// 
/// `JmpBackRoutine` is a type alias for a function pointer used in jump-back routines.
/// 
/// `HookType` is an enum that defines various hook types, including jump-back routines,
/// return hooks, and jumps to specific addresses or return addresses.
/// 
/// `JmpType` is an enum that specifies different jump instruction types, such as
/// RIP-relative, direct jumps, and direct jumps with RIP-relative addressing.
/// 
/// The `get_jmp_insn_size` method calculates the size of the jump instruction
/// based on the `JmpType`.
impl JmpType {
    fn get_jmp_insn_size(&self) -> usize {
        match &self {
            JmpType::RipRelative => 14,
            JmpType::Direct(_, _) => 5,
            JmpType::DirectWithRipRelative(_) => 5
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Registers {
    pub xmm0: u128,
    pub xmm1: u128,
    pub xmm2: u128,
    pub xmm3: u128,
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64
    pub r9: u64,
    pub r8: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rsp: u64,
    pub rflags: u64,
    pub _no_use: u64,
    pub rax: u64
}

impl Registers {

    #[must_use]
    pub unsafe fn get_stack(&self, cnt: usize) -> u64 {
        *((self.rsp as usize + cnt * 8) as *mut u64)
    }
}

#[derive(Debug, Clone)]
pub struct Hook<'a> {
    addr: usize, 
    hook_type: HookType,
    options: HookOptions<'a>
}

#[derive(Debug, Clone)]
pub struct HookOptions<'a> {
    pub first_jmp_type: JmpType,
    pub thread_operating_callback: Option<&'a dyn ThreadOpeartingCallback>,
    pub code_protection_callback: Option<&'a dyn CodeProtectModifyingCallbacky>
}

impl<'a> Default for HookOptions<'a> {
    fn default() -> Self {
        HookOptions {
            first_jmp_type: JmpType::RipRelative,
            thread_operating_callback: None,
            code_protection_callback: None
        }
    }
}