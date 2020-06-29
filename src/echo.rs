

use std::os::raw::c_int;
use std::os::raw::c_char;
use std::ffi::*;
use std::str;
use printf::printf;
// 引数に渡したString参照を出力する
pub fn echo (target : &String )
{
    println!("{}", target);
}


// 簡易にデバッグを実行する(trait境界も記述)
pub fn dump<T>(target: T)  -> () where T : std::fmt::Debug {
    println!("{:?}", target);
    // 空のユニット型を返却する
    return ();
}

pub
fn print_c_string(output :Vec<u8>) -> isize {
    unsafe {
        extern "C" {
            fn puts(s: *const c_char) -> c_int;
            fn putchar(s: c_int) -> c_int;
        }
        // Vectorのサイズを取得
        let output_size: isize = output.len() as isize;

        // VectorからCStringを生成
        let to_print = CString::new(output);
        // check_type(&to_print);

        // 無事にCStringを取り出せたとき
        if (to_print.is_ok() == true) {
            puts(to_print.unwrap().as_ptr());
            return output_size;
        } else {
            panic!("{}", to_print.unwrap_err())
        }
        return -1;
    }
}
