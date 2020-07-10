

use std::os::raw::c_int;
use std::os::raw::c_char;
use std::ffi::*;
use std::str;
use std::io::prelude::*;
use printf::printf;


/// 引数に渡したString参照を出力する
/// 戻りはUnit型を返却
pub fn echo (target : &String ) -> ()
{
    let target_length : usize = target.len();
    println!("{}", target);
    return ();
}


/// 簡易にデバッグを実行する(trait境界も記述)
/// 戻りはUnit型を返却
pub fn dump<T>(target: T)  -> () where T : std::fmt::Debug {
    println!("{:?}", target);
    // 空のユニット型を返却する
    return () ;
}

pub fn print_c_string(output :Vec<u8>) -> isize {
    unsafe {
        extern "C" {
            fn puts(s: *const c_char) -> c_int;
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
    }
}


pub fn printf_c_string(output: Vec<u8>) -> isize {
    unsafe {
        #[link(name="legacy_stdio_definitions", kind="static")]
        extern "C" {
            fn printf(format: *const c_char, args: *mut c_char) -> c_int;
        }

        // %指定子を作成する
        let c_percent = CString::new("%s".to_string());
        let c_percent_cstring: std::ffi::CString;
        if c_percent.is_ok() == true {
            c_percent_cstring = c_percent.unwrap();
        } else {
            panic!(c_percent.unwrap_err());
        }
        let c_percent_ptr = c_percent_cstring.as_ptr() as *const c_char;


        // printf関数の第二引数を作成する
        let c_string_result = CString::new(output);
        let c_string: std::ffi::CString;
        if c_string_result.is_ok() == true {
            c_string = c_string_result.unwrap();
        } else {
            panic!(c_string_result.unwrap_err());
        }
        let c_string_ptr = c_string.as_ptr() as *mut c_char;

        printf(c_percent_ptr, c_string_ptr);
    }
    return -1;
}


/// printf関数の実装作業
pub fn _printf_c_string(output: Vec<u8>) -> isize {
    unsafe {
        #[link(name="legacy_stdio_definitions", kind="static")]
        extern "C" {
            fn printf(format: *const c_char, args: *mut c_void) -> c_int;
        }

        let c_percent = CString::new("%s".to_string()).unwrap();
        let c_percent_ptr = c_percent.as_ptr() as *const c_char;

        let c_string = CString::new(output).unwrap();
        let c_string_ptr = c_string.as_ptr() as *mut c_void;

        printf(c_percent_ptr, c_string_ptr);
    }
    return -1;
}


pub fn get_command_line () -> String
{
    // コマンドラインからの入力を取
    let input : String;
    let input_data = std::io::stdin().bytes();
    let mut ensure_bytes: Vec<u8> = Vec::new();

    let mut temporary_u : u8;
    // 延々と入力がループするので、任意のbyteで breakする
    for value
    in input_data {
        temporary_u = value.unwrap();
        if (temporary_u == 10) {
            break;
        }
        ensure_bytes.push(temporary_u);
    }

    // Vec<u8>をString型に変換し返却する
    return String
    ::
    from_utf8(ensure_bytes).unwrap();
}