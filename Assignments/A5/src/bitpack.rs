use std::convert::TryInto;

/// Returns true if the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    if width == 0 {
        return false;
    }
    let n_shift : i64 = (n << (64 - width)) >> (64 - width);
    if n == n_shift { 
        return true;
    }
    false
}

/// Returns true if the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    if width == 0 {
        return false;
    }
    let n_shift = (n << (64 - width)) >> (64 - width);
    if n == n_shift { 
        return true;
    }
    false
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> Option<i64> {
    if width == 0 || width > 64 || lsb > 63 || lsb + width > 64 {
        return None;
    }
    let mask: u64 = ((1 << width) - 1) << lsb;
    let mut result: i64 = ((word & mask) >> lsb) as i64;
    // If the number is negative (sign bit is set), extend the sign bit.
    if (result & (1 << (width - 1))) != 0 {
        result |= !0 << width;
    }
    Some(result)
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    let place_holder = (1 << width) - 1;
    let part = (word >> lsb) & place_holder;

    return part.try_into().unwrap();
}


/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if width > 64 || width + lsb > 64 {
        panic!();
    }
    if !fitsu(value, width) {
        return None
    }
    let right: u64;
    let left: u64;
    if lsb == 0 { 
        right = 0; 
    } else { 
        right = (word << (64 - lsb)) >> (64 - lsb); 
    }
    if width + lsb == 64 { 
        left = 0; 
    } 
    else { 
        left = (word >> (width + lsb)) << (width + lsb);
    }
    let newu : Option<u64> = Some(left|value << lsb|right);
    newu
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if width > 64 || width + lsb > 64 {
        panic!();
    }
    if !fitss(value, width) {
        return None
    }
    let right: u64;
    let left: u64;
    let mut middle: u64;
    if lsb == 0 { 
        right = 0; 
    } 
    else { 
        right = (word << (64 - lsb)) >> (64 - lsb); 
    }
    if width+lsb == 64 { 
        left = 0; 
    } 
    else { 
        left = (word >> (width + lsb)) << (width + lsb);
    }
    if value < 0 {
        middle = (!(value<<(64-width))).try_into().unwrap();
        middle = (!middle)>>(64-width-lsb);
    }
    else {
        middle = (value<<lsb).try_into().unwrap();
    }
    let news : Option<u64> = Some(left|middle|right);
    news
}
