#![no_std]

pub const VAILD_AVID_MAX: u64 = 29460791296;
pub const BVID_LEN: usize = 10;
const XORN: u64 = 177451812;
const ADDN: u64 = 100618342136696320;
const TABLE: [u8; 58] = *b"fZodR9XQDSUm21yCkr6zBqiveYah8bt4xsWpHnJE7jL5VG3guMTKNPAwcF";
const MAP: [usize; BVID_LEN] = [9, 8, 1, 6, 2, 4, 0, 7, 3, 5];
const REV_TABLE: [u8; 74] = [
    13, 12, 46, 31, 43, 18, 40, 28, 5, 0, 0, 0, 0, 0, 0, 0, 54, 20, 15, 8, 39, 57, 45, 36, 0, 38,
    51, 42, 49, 52, 0, 53, 7, 4, 9, 50, 10, 44, 34, 6, 25, 1, 0, 0, 0, 0, 0, 0, 26, 29, 56, 3, 24,
    0, 47, 27, 22, 41, 16, 0, 11, 37, 2, 35, 21, 17, 33, 30, 48, 23, 55, 32, 14, 19,
];
const POW58: [u64; BVID_LEN] = [
    1, 58, 3364, 195112, 11316496, 656356768, 38068692544,
    2207984167552, 128063081718016, 7427658739644928,
];

pub fn av2bv(avid: u64) -> Option<[u8; BVID_LEN]> {
    if avid >= VAILD_AVID_MAX {
        None
    } else {
        let a = (avid ^ XORN) + ADDN;
        let mut bvid = [0; BVID_LEN];
        for i in 0..BVID_LEN {
            bvid[MAP[i]] = TABLE[(a / POW58[i]) as usize % 58];
        }
        Some(bvid)
    }
}

pub fn bv2av(bvid: [u8; BVID_LEN]) -> Option<u64> {
    let mut a = 0;
    for i in 0..BVID_LEN {
        a += REV_TABLE[bvid[MAP[i]] as usize - 49] as u64 * POW58[i];
    }
    let avid = (a - ADDN) ^ XORN;
    if avid >= VAILD_AVID_MAX {
        None
    } else {
        Some(avid)
    }
}

#[cfg(feature = "arraystr")]
mod arraystr;
#[cfg(feature = "arraystr")]
pub use arraystr::Bvid;

#[cfg(test)]
mod tests {
    use super::*;

    fn calc_rev_table() -> [u8; 74] {
        let mut rev_table = [0; 74];
        for i in 0..58 {
            rev_table[TABLE[i as usize] as usize - 49] = i;
        }
        rev_table
    }

    fn calc_pow58() -> [u64; BVID_LEN] {
        let mut pow58 = [0; BVID_LEN];
        for i in 0..BVID_LEN {
            pow58[i] = 58u64.pow(i as u32);
        }
        pow58
    }

    #[test]
    fn derived_consts() {
        assert_eq!(calc_pow58(), POW58);
        assert_eq!(calc_rev_table(), REV_TABLE);
    }

    macro_rules! cases {
        ($($avid:literal -> $bvid:literal)*) => {$(
            assert_eq!(*$bvid, av2bv($avid).unwrap());
            assert_eq!($avid, bv2av(*$bvid).unwrap());
        )*};
    }

    #[test]
    fn customary_example() {
        cases! {
            170001 -> b"17x411w7KC"
        }
    }

    #[test]
    fn my_examples() {
        cases! {
            422650327 -> b"1Y3411v7b9"
            681226291 -> b"1ES4y1579M"
            763311329 -> b"1Z64y187P4"
        }
    }
}
