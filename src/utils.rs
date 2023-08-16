#![allow(dead_code)]

use chrono::offset::Utc;

pub fn hex_nibble(num: u8) -> char {
    if num > 0xF {
        return '0';
    }
    let nibble = if num > 0x9 { num + 7 } else { num };

    (0x30 + nibble) as char
}

pub fn hexa(byte: u8) -> String {
    let (nibble1, nibble2) = (hex_nibble((byte & 0xF0) >> 4), hex_nibble(byte & 0x0F));
    let mut strr = String::from("");
    strr.push(nibble1);
    strr.push(nibble2);
    strr
}

pub fn get_time() -> String {
    Utc::now().to_string()
}

pub fn hexdump(data: Vec<u8>) -> String {
    let mut len:usize = 0;
    let mut content = String::from("0: ");
    for byte in data.clone() {
        let hex_byte = hexa(byte);
        content+=&(hex_byte + " ");
        if len%16 == 0 && len > 0 && data.len()-len-1 != 0 {
            for ch in &data[(len-16)..len] {
                if *ch > 0x20 && *ch < 0x7f {
                    content.push(*ch as char);
                }else {
                    content += ".";
                }
            }
            content+=&format!("\n{:X}: ", len);
        }else if len == data.len()-1 {
            if len%16 !=0 {
                for _ in 0..(16 - len%16) {
                    content+="00 ";
                }
            }
            for ch in &data[len - (len%16)..len] {
                if *ch > 0x20 && *ch < 0x7f {
                    content.push(*ch as char);
                }else {
                    content+=".";
                }
            }
        }
        len+=1;
    }
    content
}
