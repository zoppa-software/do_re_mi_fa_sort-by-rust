extern crate do_re_mi_fa_sort;

use std::time::Instant;
use std::{fs, mem};
use std::io::*;

fn main() {
    let mut _buf: Vec<i32> = Vec::new();
    let mut b: [u8; 4] = unsafe { mem::uninitialized() };
    let mut f = BufReader::new(fs::File::open("inpt.dat").unwrap());
    for _ in 0 .. 100 {
        f.read_exact(&mut b).unwrap();
        _buf.push(b[3] as i32 * 0x1000000 + b[2] as i32 * 0x10000 + b[1] as i32 * 0x100 + b[0] as i32);
    }

    let start = Instant::now();
    
    do_re_mi_fa_sort::sort_to_vec(&mut _buf);
    //let s = _buf.sort();
    let end = start.elapsed();
    println!("{}.{:03}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);
    //for v in _buf.iter() {
    //    println!("{},", v);
    //}
}