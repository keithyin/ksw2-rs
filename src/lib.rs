use std::ffi::c_int;

use ksw2_sys::km_destroy;

pub mod ksw2_sys;

pub const DNA_SEQ_ASCII2IDX: [u8; 128] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 0, 4, 1, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 0, 4, 1, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
];

/// ksw2 gg dna
/// 
/// args:
///     bw: band width
/// Return:
///     cigar string
pub fn ksw2_gg_dna(
    query: &str,
    target: &str,
    m_score: i8,
    mm_score: i8,
    gapo: i8,
    gape: i8,
    bw: c_int,
) -> String {
    let mm_score = if mm_score > 0 { -mm_score } else { mm_score };
    let mat = [
        m_score, mm_score, mm_score, mm_score, 0, mm_score, m_score, mm_score, mm_score, 0,
        mm_score, mm_score, m_score, mm_score, 0, mm_score, mm_score, mm_score, m_score, 0, 0, 0,
        0, 0, 0,
    ];

    let q = query
        .as_bytes()
        .iter()
        .map(|c| DNA_SEQ_ASCII2IDX[*c as usize])
        .collect::<Vec<_>>();
    let t = target
        .as_bytes()
        .iter()
        .map(|c| DNA_SEQ_ASCII2IDX[*c as usize])
        .collect::<Vec<_>>();

    let cigar_str = unsafe {
        let km = ksw2_sys::km_init();
        let mut ez: ksw2_sys::ksw_extz_t = std::mem::zeroed();
        ksw2_sys::ksw_gg(
            km,
            q.len() as c_int,
            q.as_ptr(),
            t.len() as c_int,
            t.as_ptr(),
            5,
            mat.as_ptr(),
            gapo,
            gape,
            bw,
            &mut ez.m_cigar,
            &mut ez.n_cigar,
            &mut ez.cigar
        );

        let cigar_str = (0..ez.n_cigar as usize).into_iter().map(|idx| {
            let c = *ez.cigar.add(idx);
            let cnt = c >> 4;
            let op = c & 0xf;
            format!("{}{}", cnt, b"MID"[op as usize] as char)
        }).collect::<Vec<_>>().join("");

        km_destroy(km);
        cigar_str
    };
    cigar_str


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       
        for i in b"acgtACGT" {
            println!("ASCII {}: {}", *i as char, DNA_SEQ_ASCII2IDX[*i as usize]);
        }
    }

    #[test]
    fn test_gg() {

        println!("{}", ksw2_gg_dna("AACTG", "CTG", 2, -4, 4, 2, 0));

    }
}
