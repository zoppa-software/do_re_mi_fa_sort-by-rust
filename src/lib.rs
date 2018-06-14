fn insertion<T: PartialOrd>(ptr: *mut T, start: isize, end: isize) {  
    let mut scan = end - 1;

    unsafe {
        while scan >= start {
            let i = scan;
            if *ptr.offset(i + 1) < *ptr.offset(i) {
                let mut temp: T = std::mem::uninitialized();
                std::ptr::copy_nonoverlapping(ptr.offset(i), &mut temp, 1);

                let mut j = i;
                while {
                    std::ptr::copy_nonoverlapping(ptr.offset(j + 1), ptr.offset(j), 1);
                    j += 1;
                    j < end && *ptr.offset(j + 1) < temp
                } {}
                std::ptr::copy_nonoverlapping(&temp, ptr.offset(j), 1);
                std::mem::forget(temp);
            }
            scan -= 1;
        }
    }
}

fn swap<T: PartialOrd>(ptr: *mut T, lf: isize, rt: isize)
{
    unsafe {
        let mut temp: T = std::mem::uninitialized();
        std::ptr::copy_nonoverlapping(ptr.offset(lf), &mut temp, 1);
        std::ptr::copy_nonoverlapping(ptr.offset(rt), ptr.offset(lf), 1);
        std::ptr::copy_nonoverlapping(&temp, ptr.offset(rt), 1);
        std::mem::forget(temp);
    }
}

fn dual_sort<T: PartialOrd>(ptr: *mut T, lf: isize, rt: isize)
{
    unsafe {
        if *ptr.offset(lf) > *ptr.offset(rt) {
            swap(ptr, lf, rt);
        }
    }
}

fn tri_sort<T: PartialOrd>(ptr: *mut T, p1: isize, p2: isize, p3: isize)
{
	dual_sort(ptr, p1, p2);
	dual_sort(ptr, p2, p3);
	dual_sort(ptr, p1, p2);
}

fn do_re_mi_fa_sort<T: PartialOrd>(ptr: *mut T, start: isize, end: isize) {
    unsafe {
        if end > start {
            if end - start > 48 {
                let rec_me = (end - start) / 2 + start;
                let rec_re = (rec_me - start) / 2 + start;
                let rec_fa = (end - rec_me) / 2 + rec_me;
                tri_sort(ptr, start, rec_re, rec_me);
                tri_sort(ptr, rec_re, rec_me, rec_fa);
                tri_sort(ptr, rec_me, rec_fa, end);
                dual_sort(ptr, start, rec_re);
                dual_sort(ptr, rec_re, rec_me);
                swap(ptr, rec_me, end);

                let mut temp: T = std::mem::uninitialized();
                std::ptr::copy_nonoverlapping(ptr.offset(end), &mut temp, 1);
                let pivot: T = temp;

                let mut n_pivot_r = end;
                while n_pivot_r > start &&
                    *ptr.offset(n_pivot_r - 1) == pivot {
                    n_pivot_r -= 1;
                }

                if n_pivot_r == start {
                    return;
                }

                let mut n_ptr_r = n_pivot_r;
                let mut n_pivot_l = start - 1;
                let mut n_ptr_l = start - 1;

                loop {
                    loop {
                        n_ptr_l += 1;
                        if *ptr.offset(n_ptr_l) >= pivot { break; }
                    }

                    loop {
                        n_ptr_r -= 1;
                        if pivot >= *ptr.offset(n_ptr_r) ||
                        n_ptr_r <= start {
                            break;
                        }
                    }

                    if n_ptr_l >= n_ptr_r { break; }

                    swap(ptr, n_ptr_l, n_ptr_r);
                    if *ptr.offset(n_ptr_l) == pivot {
                        n_pivot_l += 1;
                        swap(ptr, n_pivot_l, n_ptr_l);
                    }
                    if pivot == *ptr.offset(n_ptr_r) {
                        n_pivot_r -= 1;
                        swap(ptr, n_ptr_r, n_pivot_r);
                    }
                }

                swap(ptr, n_ptr_l, end); 
                n_ptr_r = n_ptr_l - 1;
                n_ptr_l = n_ptr_l + 1;

                let mut n_temp: isize;
                n_temp = start;
                while n_temp <= n_pivot_l {
                    swap(ptr, n_temp, n_ptr_r);
                    n_temp += 1;
                    n_ptr_r -= 1;                
                }
                n_temp = end - 1;
                while n_temp >= n_pivot_r {
                    swap(ptr, n_ptr_l, n_temp);
                    n_temp -= 1;
                    n_ptr_l += 1;
                }

                if n_ptr_r > start  {
                    do_re_mi_fa_sort(ptr, start, n_ptr_r);
                }
                if n_ptr_l < end {
                    do_re_mi_fa_sort(ptr, n_ptr_l, end);
                }
            }
            else {
                insertion(ptr, start, end);
            }
        }
    }
}

pub fn sort<T: PartialOrd>(src: &mut[T])
{
    let len = src.len() - 1;
    let ptr = src.as_mut_ptr();

    do_re_mi_fa_sort(ptr, 0, len as isize);
}
