pub trait ThreadOperatingCallback {

    fn suspend(&self) -> Result<u64, u32>;
    
    fn resume(&self, ctx: u64);

}

pub trait CodeProtectModifyingCallback {

    fn set_protect_to_rwe(&self, addr: usize, len: usize) -> Result<u64, u32>;

    fn recover_protect(&self, addr: usize, len: usize, old_prot: u64);

}

pub struct InternalCodeProtectModifyingCallback {}

impl CodeProtectModifyingCallback for InternalCodeProtectModifyingCallback {
    fn set_protect_to_rwe(&self, addr: usize, len: usize, old_prot: u64) {

    }
}

pub const DEFAULT_CODE_PROTECT_MODIFYING_CALLBACK: InternalCodeProtectModifyingCallback = InternalCodeProtectModifyingCallback {};

fn modify_mem_protect_to_rwe(addr: usize, len: usize) -> Result<u64, u32> {
    let mut old_prot: u32 = 0;
    let old_prot_ptr = std::ptr::addr_of_mut!(old_prot);
}