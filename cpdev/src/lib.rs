// #![no_std]

// mod vmdef;
// pub mod vmlib;

// pub use enumset::EnumSet;

// use crate::{
//     vmdef::Opcode,
//     vmlib::{
//         wmc_add, wmc_and, wmc_conv, wmc_custom, wmc_div_mod, wmc_eq, wmc_ge, wmc_gt, wmc_le,
//         wmc_limit, wmc_lt, wmc_max, wmc_min, wmc_mmv, wmc_mul, wmc_mux, wmc_ne, wmc_neg_not,
//         wmc_or, wmc_sel, wmc_shi_rot, wmc_sub, wmc_sysctrl, wmc_xor,
//     },
// };

// #[cfg(feature = "math")]
// use crate::vmlib::{WMC_DOW, WMC_EXPT_ABS};

// #[cfg(feature = "pointers")]
// use crate::vmlib::WMC_Pointers;

// #[cfg(feature = "strings")]
// use crate::vmlib::WMC_String;

// #[derive(Debug, enumset::EnumSetType)]
// pub enum RunMode {
//     STOP = 0x00,
//     NORMAL = 0x01,
//     DEBUG = 0x02,
//     EMULATION = 0x04,
//     CONTINUE = 0x08,
//     COLDRESTART = 0x10,
//     PAUSED = 0x20,
//     FIRSTSTART = 0x40,
//     RUNNING = 0x80,
// }

// #[derive(Debug, enumset::EnumSetType)]
// pub enum Status {
//     CYCLEOVERRUN = 0x0001,
//     ARRAYBOUND = 0x0002,
//     COLDRESTART = 0x0004,
//     FIRSTSTART = 0x0008,
//     BADFORMAT = 0x0010,
//     PAUSED = 0x4000,
//     STOPPED = 0x8000,
// }

// const _WM_TERMINATE: u8 = 1;
// const WM_UNKNOWN: u8 = 2;

// const CALL_STACK_SIZE: usize = 10;
// const DATA_STACK_SIZE: usize = 10;

// type WmByte = u8;
// type WmWord = u16;
// type WmAddress = u16;

// #[derive(Debug)]
// pub enum Error {
//     DbgVmeNativeBlock,
//     DbgVmeUnknownFunction,
// }

// type Hook<C> = fn(&mut C);

// pub struct CpDev<C> {
//     _w_calling_stack: [WmAddress; CALL_STACK_SIZE], // function block calling stack
//     w_calling_stack_ptr: WmByte,                    // and its pointer
//     _w_data_ofs_stack: [WmAddress; DATA_STACK_SIZE], // data offset stack
//     w_data_ofs_stack_ptr: WmByte,                   // and its pointer
//     b_run_mode: EnumSet<RunMode>,
//     w_status1: EnumSet<Status>,
//     w_program_counter: WmAddress, // program counter register (CSIP)
//     w_data_ofs: WmAddress,        // data pointer register (DPTR)
//     task_cycle: WmWord,           /* task cycle (ms) */
//     pgm_code: *const WmByte,      // main pointer to the VM program area
//     _cycle_start_adr: WmAddress,  //address of first instruction of a cycle
//     pgm_data: *mut WmByte,
//     b_result: WmByte, // command execution flag

//     n_cycles: u32, /* count of program cycles performed */
//     context: C,

//     pre_cycle: Option<Hook<C>>,
//     post_cycle: Option<Hook<C>>,
//     pre_program: Option<Hook<C>>,
//     post_program: Option<Hook<C>>,
//     pre_next_command: Option<Hook<C>>,

//     #[cfg(feature = "datatime_suport")]
//     rtc_value: WM_DATE_AND_TIME, /* czas astronomiczny odczytywany na początku cyklu */
// }

#![no_std]
mod vmdef;

use crate::vmdef::*;

const CALL_STACK_SIZE: usize = 10;
const DATA_STACK_SIZE: usize = 10;

pub struct VM {
    wCallingStack: [WmAddress; CALL_STACK_SIZE], // function block calling stack
    wCallingStackPtr: WmByte,                    // and its pointer
    wDataOfsStack: [WmAddress; DATA_STACK_SIZE], // data offset stack
    wDataOfsStackPtr: WmByte,                    // and its pointer

    bRunMode: WmByte,

    wStatus1: WmWord, //0x4;					// bit 0 - przekroczono cykl zadania

    ip: *const WmWord,
    wProgramCounter: WmAddress, // program counter register (CSIP)
    wDataOfs: WmAddress,        // data pointer register (DPTR)

    nCycles: u32, /* count of program cycles performed */

    task_cycle: WmWord, /* task cycle (ms) */

    // #ifdef VM_DATETIME_SUPPORT
    // WM_DATE_AND_TIME wm_rtc_value;	/* czas astronomiczny odczytywany na pocz�tku cyklu */
    // #endif
    pgmCode: *const WmByte, // main pointer to the VM program area

    cycle_start_adr: WmAddress, //address of first instruction of a cycle

    pgmData: *mut WmByte,

    bResult: WmByte, // command execution flag
}

impl VM {
    #[inline(always)]
    pub unsafe fn read<T: Copy>(&self, offset: usize) -> T {
        unsafe {
            let ptr = self.pgmData.add(offset) as *const T;
            // let ptr = self.data.add(offset + self.w_data_ofs as usize) as *const T;
            core::ptr::read_unaligned(ptr)
        }
    }

    #[inline(always)]
    pub unsafe fn write<T>(&mut self, offset: usize, value: T) {
        unsafe {
            let ptr = self.pgmData.add(offset) as *mut T;
            // let ptr = self.data.add(offset + self.w_data_ofs as usize) as *mut T;
            core::ptr::write_unaligned(ptr, value);
        }
    }

    #[inline(always)]
    pub fn get_program_word(&mut self) -> WmWord {
        // byte by byte access
        #[cfg(all(feature = "byte_access"))]
        unsafe {
            let lo = *self.ip.add(1);
            let hi = *self.ip.add(0);
            let res = ((hi as WmWord) << 8) | lo as WmWord;
            self.ip = self.ip.add(core::mem::size_of::<WmWord>());
            #[cfg(feature = "big_endian")]
            let res = WmWord::from_be(res);
            res
        }
        // memcpy
        #[cfg(all(feature = "memcpy", not(feature = "byte_access")))]
        unsafe {
            let mut res = core::mem::MaybeUninit::<WmWord>::uninit();
            self.ip.copy_to_nonoverlapping(res.as_mut_ptr(), 1);
            self.ip = self.ip.add(core::mem::size_of::<WmWord>());
            let res = res.assume_init();
            #[cfg(feature = "big_endian")]
            let res = WmWord::from_be(res);
            res
        }
        // dereference
        #[cfg(all(not(feature = "memcpy"), not(feature = "byte_access")))]
        unsafe {
            let res = core::ptr::read_unaligned(self.ip);
            self.ip = self.ip.add(core::mem::size_of::<WmWord>());
            #[cfg(feature = "big_endian")]
            let res = WmWord::from_be(res);
            res
        }
    }

    #[inline(always)]
    pub fn get_program_dword(&mut self) -> WmDword {
        #[cfg(all(feature = "byte_access"))]
        {
            let lo = self.get_program_word();
            let hi = self.get_program_word();
            ((hi as WmDword) << 16) | lo as WmDword
        }
        // memcpy
        #[cfg(all(feature = "memcpy", not(feature = "byte_access")))]
        unsafe {
            let mut res = core::mem::MaybeUninit::<WmDword>::uninit();
            (self.ip as *const WmDword).copy_to_nonoverlapping(res.as_mut_ptr(), 1);
            self.ip = self.ip.add(core::mem::size_of::<WmDword>());
            let res = res.assume_init();
            #[cfg(feature = "big_endian")]
            let res = WmDword::from_be(res);
            res
        }
        // dereference
        #[cfg(all(not(feature = "memcpy"), not(feature = "byte_access")))]
        unsafe {
            let res = core::ptr::read_unaligned(self.ip as *const WmDword);
            self.ip = self.ip.add(core::mem::size_of::<WmDword>());
            #[cfg(feature = "big_endian")]
            let res = WmDword::from_be(res);
            res
        }
    }
}

// impl<C> CpDev<C> {
//     pub fn new(
//         code_buffer: &[u8],
//         data_buffer: &mut [u8],
//         context: C,
//         pre_cycle: Option<Hook<C>>,
//         post_cycle: Option<Hook<C>>,
//         pre_program: Option<Hook<C>>,
//         post_program: Option<Hook<C>>,
//         pre_next_command: Option<Hook<C>>,
//     ) -> Self {
//         CpDev {
//             task_cycle: 200,
//             b_run_mode: RunMode::NORMAL.into(),
//             w_status1: EnumSet::empty(),
//             _w_calling_stack: [0; CALL_STACK_SIZE],
//             w_calling_stack_ptr: 0,
//             _w_data_ofs_stack: [0; DATA_STACK_SIZE],
//             w_data_ofs_stack_ptr: 0,
//             w_program_counter: 0,
//             w_data_ofs: 0,
//             pgm_code: code_buffer.as_ptr(),
//             _cycle_start_adr: 0,
//             pgm_data: data_buffer.as_mut_ptr(),
//             b_result: 0,
//             n_cycles: 0,
//             pre_cycle,
//             post_cycle,
//             pre_program,
//             post_program,
//             pre_next_command,
//             context,
//         }
//     }

//     pub fn set_task_cycle(&mut self, task_cycle: u16) {
//         self.task_cycle = task_cycle;
//     }

//     pub fn get_run_mode(&self) -> EnumSet<RunMode> {
//         self.b_run_mode
//     }

//     #[inline(always)]
//     pub unsafe fn read<T: Copy>(&self, offset: usize) -> T {
//         unsafe {
//             let ptr = self.pgm_data.add(offset + self.w_data_ofs as usize) as *const T;
//             core::ptr::read_unaligned(ptr)
//         }
//     }

//     #[inline(always)]
//     pub unsafe fn write<T>(&mut self, offset: usize, value: T) {
//         unsafe {
//             let ptr = self.pgm_data.add(offset + self.w_data_ofs as usize) as *mut T;
//             core::ptr::write_unaligned(ptr, value);
//         }
//     }

//     pub unsafe fn read_variable<T: Copy>(&mut self, variable: &Variable) -> T {
//         unsafe { self.read(variable.get_address() as usize) }
//     }

//     pub unsafe fn write_variable<T>(&mut self, variable: &Variable, value: T) {
//         unsafe {
//             self.write(variable.get_address() as usize, value);
//         }
//     }

//     pub unsafe fn read_buf(&self, offset: usize, count: usize, buffer: &mut [u8]) -> usize {
//         unsafe {
//             let ptr = self.pgm_data.add(offset + self.w_data_ofs as usize) as *const u8;
//             core::ptr::copy_nonoverlapping(ptr, buffer.as_mut_ptr(), count);
//         }
//         return count;
//     }

//     pub unsafe fn write_buf(&mut self, offset: usize, count: usize, buffer: &[u8]) -> usize {
//         unsafe {
//             let ptr = self.pgm_data.add(offset + self.w_data_ofs as usize) as *mut u8;
//             core::ptr::copy_nonoverlapping(buffer.as_ptr(), ptr, count);
//         }
//         return count;
//     }

//     pub unsafe fn read_variable_buf(&self, variable: &Variable, buffer: &mut [u8]) {
//         unsafe {
//             self.read_buf(variable.get_address() as usize, variable.get_size(), buffer);
//         }
//     }

//     pub unsafe fn write_variable_buf(&mut self, variable: &Variable, buffer: &[u8]) {
//         unsafe {
//             self.write_buf(variable.get_address() as usize, variable.get_size(), buffer);
//         }
//     }

//     pub fn initialize(&mut self, mode: EnumSet<RunMode>) {
//         self.b_run_mode = mode;

//         if !self.b_run_mode.contains(RunMode::CONTINUE) {
//             self.init_registers();
//         }

//         if self.b_run_mode.contains(RunMode::COLDRESTART) {
//             self.w_status1 = Status::COLDRESTART.into();
//         }

//         if self.b_run_mode.contains(RunMode::FIRSTSTART) {
//             self.w_status1 |= Status::FIRSTSTART;
//         }

//         self.w_status1 -= Status::STOPPED;
//         self.b_run_mode |= RunMode::RUNNING;

//         self.clear_stacks();

//         self.pre_program();
//     }

//     pub fn run_cycle(&mut self) {
//         #[cfg(feature = "datatime_suport")]
//         {
//             // VMP_ReadRTC(&wm_rtc_value);
//         }

//         self.pre_cycle();

//         if self.b_run_mode.is_empty() {
//             return;
//         }

//         while self.run_command() != 0 && !self.b_run_mode.is_empty() {}

//         self.post_cycle();

//         self.w_status1 -= Status::COLDRESTART | Status::FIRSTSTART;

//         self.n_cycles += 1;
//     }

//     pub fn shutdown(&mut self) {
//         self.w_status1 |= Status::STOPPED;
//         self.b_run_mode -= RunMode::RUNNING;
//         self.post_program();
//     }

//     fn pre_cycle(&mut self) {
//         if let Some(cb) = self.pre_cycle.as_mut() {
//             cb(&mut self.context);
//         }
//     }

//     fn post_cycle(&mut self) {
//         if let Some(cb) = self.post_cycle.as_mut() {
//             cb(&mut self.context);
//         }
//     }

//     fn pre_program(&mut self) {
//         if let Some(cb) = self.pre_program.as_mut() {
//             cb(&mut self.context);
//         }
//     }

//     fn post_program(&mut self) {
//         if let Some(cb) = self.post_program.as_mut() {
//             cb(&mut self.context);
//         }
//     }

//     fn pre_next_command(&mut self) {
//         if let Some(cb) = self.pre_next_command.as_mut() {
//             cb(&mut self.context);
//         }
//     }

//     fn clear_stacks(&self) {
//         todo!()
//     }

//     fn run_command(&mut self) -> u8 {
//         let w_cmd_code: u16;
//         let w_cmd: u8;
//         let w_cmd_opt: u8;

//         self.pre_next_command();

//         #[cfg(not(feature = "4B_align"))]
//         {
//             w_cmd_code = self.get_program();
//         }
//         #[cfg(feature = "4B_align")]
//         {
//             w_cmd_code = get_program();
//             self.w_program_counter += 2;
//             //let w_cmd_code = get_program_dword();
//         }
//         w_cmd = (w_cmd_code & 0x00FF) as u8;
//         w_cmd_opt = ((w_cmd_code & 0xFF00) >> 8) as u8;

//         self.b_result = 0;

//         //--wDataOfs = wDataOffsetReg;	/* set data offset if the highest bit set */
//         //--wCmd &= 0x7F;					/* and clear the bit */
//         if let Ok(op) = Opcode::try_from(w_cmd) {
//             match op {
//                 Opcode::FgNop => {}
//                 Opcode::FgHalt => {
//                     self.b_result = WM_UNKNOWN;
//                 }

//                 Opcode::FgSysctrl => {
//                     wmc_sysctrl(w_cmd_opt);
//                 }

//                 Opcode::FgSysmem => {
//                     wmc_mmv(w_cmd_opt);
//                 }

//                 Opcode::FgAdd => {
//                     wmc_add(self, w_cmd_opt);
//                 }

//                 Opcode::FgAnd => {
//                     wmc_and(w_cmd_opt);
//                 }

//                 Opcode::FgOr => {
//                     wmc_or(w_cmd_opt);
//                 }

//                 Opcode::FgXor => {
//                     wmc_xor(w_cmd_opt);
//                 }

//                 Opcode::FgMul => {
//                     wmc_mul(w_cmd_opt);
//                 }

//                 Opcode::FgSub => {
//                     wmc_sub(w_cmd_opt);
//                 }

//                 Opcode::FgDivMod => {
//                     wmc_div_mod(w_cmd_opt);
//                 }

//                 Opcode::FgNegNot => {
//                     wmc_neg_not(w_cmd_opt);
//                 }

//                 #[cfg(feature = "math")]
//                 Opcode::FgExptAbsDow => {
//                     if w_cmd_opt < (VMF_GET_DAYOFWEEK_DATE & 0xFF) {
//                         WMC_EXPT_ABS(w_cmd_opt);
//                     } else {
//                         WMC_DOW(w_cmd_opt);
//                     }
//                 }

//                 Opcode::FgShiRot => {
//                     wmc_shi_rot(w_cmd_opt);
//                 }

//                 Opcode::FgSel => {
//                     wmc_sel(w_cmd_opt);
//                 }

//                 Opcode::FgLimit => {
//                     wmc_limit(w_cmd_opt);
//                 }

//                 Opcode::FgMax => {
//                     wmc_max(w_cmd_opt);
//                 }

//                 Opcode::FgMin => {
//                     wmc_min(w_cmd_opt);
//                 }

//                 Opcode::FgGt => {
//                     wmc_gt(w_cmd_opt);
//                 }

//                 Opcode::FgGe => {
//                     wmc_ge(w_cmd_opt);
//                 }

//                 Opcode::FgLt => {
//                     wmc_lt(w_cmd_opt);
//                 }

//                 Opcode::FgLe => {
//                     wmc_le(w_cmd_opt);
//                 }

//                 Opcode::FgEq => {
//                     wmc_eq(w_cmd_opt);
//                 }

//                 Opcode::FgNe => {
//                     wmc_ne(w_cmd_opt);
//                 }

//                 Opcode::FgMux => {
//                     wmc_mux(w_cmd_opt);
//                 }

//                 Opcode::FgConv => {
//                     wmc_conv(w_cmd_opt);
//                 }

//                 Opcode::FgCustomspace => {
//                     self.b_result = wmc_custom(w_cmd_opt);
//                 }

//                 #[cfg(feature = "pointers")]
//                 Opcode::FgPointers => {
//                     WMC_Pointers(w_cmd_opt);
//                 }

//                 #[cfg(feature = "strings")]
//                 Opcode::FgString => {
//                     WMC_String(w_cmd_opt);
//                 }
//             }
//         } else {
//             self.b_result = WM_UNKNOWN;
//         }

//         if self.b_result == WM_UNKNOWN {
//             match Opcode::try_from(w_cmd) {
//                 Ok(op) if op == Opcode::FgCustomspace => {
//                     panic!(
//                         "Runtime error {:?} {:?} near {}",
//                         Error::DbgVmeNativeBlock,
//                         ((w_cmd as u16) << 8) | w_cmd_opt as u16,
//                         self.w_program_counter
//                     );
//                 }
//                 _ => {
//                     panic!(
//                         "Runtime error {:?} {:?} near {}",
//                         Error::DbgVmeUnknownFunction,
//                         ((w_cmd as u16) << 8) | w_cmd_opt as u16,
//                         self.w_program_counter
//                     );
//                 }
//             }
//             // return -1;
//         }
//         return self.b_result;
//     }

//     fn init_registers(&mut self) {
//         self.w_calling_stack_ptr = 0;
//         self.w_data_ofs_stack_ptr = 0;
//         self.w_status1 = EnumSet::empty();
//         self.w_program_counter = 0;
//         self.w_data_ofs = 0;
//         self.n_cycles = 0;
//         #[cfg(feature = "datatime_suport")]
//         {
//             // VMP_ReadRTC(&wm_rtc_value);
//         }
//         self.b_result = 0;
//     }

//     pub fn get_cycles(&self) -> u32 {
//         self.n_cycles
//     }

//     pub fn get_status1(&self) -> u8 {
//         self.w_status1.as_u8()
//     }

//     pub fn get_program_address(&self) -> Address {
//         todo!()
//     }

//     pub fn run(&mut self, mode: EnumSet<RunMode>) {
//         self.initialize(mode);

//         while !self.b_run_mode.is_empty() {
//             self.run_cycle();
//         }

//         self.shutdown();
//     }

//     pub fn stop(&mut self) {
//         self.b_run_mode = EnumSet::empty();
//     }

//     fn get_program<T: Copy>(&mut self) -> T {
//         let v = unsafe { self.read(self.pgm_code as usize + self.w_program_counter as usize) };
//         self.w_program_counter += size_of::<T>() as u16;
//         return v;
//     }
// }

// type Address = u32;

// pub struct Variable<'a> {
//     pub var_name: &'a str,
//     pub var_address: Address,
//     pub var_size: u8,
// }

// impl<'a> Variable<'a> {
//     pub fn new(name: &'a str, address: Address, size: u8) -> Self {
//         Variable {
//             var_name: name,
//             var_address: address,
//             var_size: size,
//         }
//     }

//     pub fn get_address(&self) -> Address {
//         self.var_address
//     }

//     pub fn get_size(&self) -> usize {
//         self.var_size as usize
//     }
// }
