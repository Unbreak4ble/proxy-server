
pub fn isIp(content: String) -> bool {
    let mut dots = 0;
    for ch in content.chars() {
        if ch == '.' {
            dots+=1;
        }else if ((ch as u8) < 0x30) || ((ch as u8) > 0x39) {
            break;
        }
        if dots == 3 {
            return true;
        }
    }
    false
}
