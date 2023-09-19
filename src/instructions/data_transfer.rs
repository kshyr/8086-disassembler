use crate::registers::{address_calculation, decode_register};

pub fn mov_rm_tf_reg(d: u8, w: u8, mod_: u8, reg: u8, rm: u8, disp_lo_opt: Option<u8>, disp_hi_opt: Option<u8>) {
    let reg = decode_register(reg, w).to_string();

    let rm = if mod_ != 0b11 { address_calculation(rm, mod_) } else { decode_register(rm, w).to_string() };
    let rm = rm + match (disp_lo_opt, disp_hi_opt) {
        (Some(disp_lo), Some(disp_hi)) => {
            let lo = disp_lo as i16;
            let hi = disp_hi as i16;
            let result = (lo | (hi << 8));
            if result > 0 { format!(" + {}]", result) } else { "]".to_string() }
        }
        (Some(disp_lo), None) => {
            let result = disp_lo as i8;
            if result > 0 { format!(" + {}]", result) } else { "]".to_string() }
        }
        (None, None) => "".to_string(),
        _ => unreachable!()
    }.as_str();

    let (src, dest) = if d == 1 { (rm, reg) } else { (reg, rm) };

    println!("mov {}, {}", dest, src);
}

pub fn mov_im_to_rm() {
    println!("mov_im_to_rm");
}

pub fn mov_im_to_reg(w: u8, reg: u8, data_lo: u8, data_hi_opt: Option<u8>) {
    let reg = decode_register(reg, w);

    let data: i16 = match data_hi_opt {
        Some(data_hi) => {
            let lo = data_lo as i16;
            let hi = data_hi as i16;
            let result = (lo | (hi << 8));
            result
        }
        None => (data_lo as i8).into(),
    };

    println!("mov {}, {}", reg, data);
}

pub fn mov_mem_to_acc() {
    println!("mov_mem_to_acc");
}

pub fn mov_acc_to_mem() {
    println!("mov_acc_to_mem");
}

pub fn mov_rm_to_sr() {
    println!("mov_rm_to_sr");
}

pub fn mov_sr_to_rm() {
    println!("mov_sr_to_rm");
}
