use std::fs::read;
use std::io::Read;
use std::path::Path;

use crate::instructions::data_transfer::{mov_acc_to_mem, mov_im_to_reg, mov_im_to_rm, mov_mem_to_acc, mov_rm_tf_reg, mov_rm_to_sr, mov_sr_to_rm};

mod registers;
mod instructions;

fn main() {
    let bin = Path::new("compiled_asm/more_movs");
    let bytes = read(bin).unwrap();
    decode(&bytes);
}

fn decode(bytes: &[u8]) {
    let mut i = 0;

    while i < bytes.len() {
        let op_byte = bytes[i];

        if op_byte >> 2 == 0b100010 {
            let d = (op_byte >> 1) & 0b1;
            let w = op_byte & 0b1;

            let next_byte = bytes[i + 1];
            let mod_ = next_byte >> 6;
            let reg = (next_byte >> 3) & 0b111;
            let rm = next_byte & 0b111;

            let disp = match mod_ {
                0b00 => {
                    if rm == 0b110 { 2 } else { 0 }
                }
                0b01 => 1,
                0b10 => 2,
                0b11 => 0,
                _ => unreachable!()
            };

            let disp_lo = if disp >= 1 { Some(bytes[i + 2]) } else { None };
            let disp_hi = if disp == 2 { Some(bytes[i + 3]) } else { None };

            mov_rm_tf_reg(d, w, mod_, reg, rm, disp_lo, disp_hi);
            i += disp + 2;
        } else if op_byte >> 1 == 0b1100011 {
            mov_im_to_rm();
            i += 1;
        } else if op_byte >> 4 == 0b1011 {
            let w = (op_byte >> 3) & 0b1;
            let reg = op_byte & 0b111;

            let data_lo = bytes[i + 1];
            let data_hi = if w == 1 { Some(bytes[i + 2]) } else { None };

            mov_im_to_reg(w, reg, data_lo, data_hi);
            i += 2 + w as usize;
        } else if op_byte >> 1 == 0b1010000 {
            mov_mem_to_acc();
            i += 1;
        } else if op_byte >> 1 == 0b1010001 {
            mov_acc_to_mem();
            i += 1;
        } else if op_byte == 0b10001110 {
            mov_rm_to_sr();
            i += 1;
        } else if op_byte == 0b10001100 {
            mov_sr_to_rm();
            i += 1;
        } else {
            i += 1;
            println!("UNKNOWN");
        }
    }
}
