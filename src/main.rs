


use std::string::FromUtf8Error;
extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use std::process;
use std::fs;
use std::env;
use std::process::{
    Command,
    Output,
    Stdio,
};
use std::io::{
    Error,
    Write
};

use std::env::temp_dir;
use std::path::PathBuf;


// 定数の宣言
const EXIT_STRING : &str = "exit";
const CLEAR_STRING: &str = "clear";
const DEL_STRING: &str = "del";


use std::ffi::{
    CString,
    CStr
};
use std::os::raw::{
    c_char,
    c_int,
    c_void
};

// echoモジュールを使用
mod echo;
// echoモジュール内のecho関数を単独で利用する
use echo::*;

fn main() {

    let function = | param: String | {
        echo (&param);
    };

    let default_command : String = "php".to_string();
    // 実行時のコマンドライン引数を取得
    let arguments: Vec<String> = env::args().collect();
    let command: String;
    if (arguments.len() >= 2) {
        command = arguments.get(1).unwrap().to_string();
    } else {
        command = "php".to_string();
        // panic!("Select any execute file.");
    }
    // 入力されたコマンドが php | ruby | python?
    let mut initialize_input : String;

    initialize_input = "".to_string();
    if (command == default_command) {
        initialize_input = "<?php \r\n".to_string();
    }



    // 起動中の自身のプロセスID
    let my_pid: u32 =  process::id();

    // 改行された回数を保持
    let mut previous_newline_count : i32 = 0;
    let mut current_newline_count : i32 = 0;

    // 検証用ソース・ファイルと実行用ソース・ファイルの2つのFileオブジェクトを取得する
    let mut validate_dir : PathBuf = env::temp_dir();
    validate_dir.push("validate_log.dat");
    let validate_file_path = format!("{}", validate_dir.display());
    let validate_file : Result<fs::File, Error> = fs::File::create(&validate_file_path);

    // 実行用
    let mut execute_dir: PathBuf = env::temp_dir();
    execute_dir.push("execute_log.dat");
    let execute_file_path = format!("{}", execute_dir.display());
    let execute_file : Result<fs::File, Error> = fs::File::create(&execute_file_path);

    // 検証用ソース・ファイルの作成失敗時
    if (validate_file.is_ok() != true) {
        panic!("{}", validate_file.unwrap_err());
        process::exit(my_pid as i32);
    }
    let mut validate_file = validate_file.unwrap();
    // 検証用ファイルの初期化
    validate_file.write_all(initialize_input.as_bytes());

    // 実行用ソース・ファイルの作成失敗時
    if (execute_file.is_ok() != true) {
        panic!("{}", execute_file.unwrap_err().to_string());
        process::exit(my_pid as i32);
    }
    let mut execute_file = execute_file.unwrap();


    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Input any source code with {}.", command);
    // while running.load(Ordering::SeqCst) {}

    // コマンドラインからの入力を取
    let mut input : String = String::new();

    // 書き込み時の戻り値を保持
    let mut written_bytes : Result<(), Error>;
    let mut line_number: usize = 0;
    while (true) {
        println!("{}:", line_number = line_number + 1);
        line_number = line_number + 1;
        let input_data : Result<usize, Error> = std::io::stdin().read_line(&mut input);

        // コマンドラインから入力されたbyte数を取得する
        if input_data.is_ok() != true
        {
            panic!("error => {}", input_data.unwrap_err());
        }
        // 入力内容から、改行文字を削除
        remove_newline(&mut input);



        // 入力内容によってループ内の処理を変更する
        // コマンドラインを終了するための処理
        if (EXIT_STRING.to_string() == input) {
            // exit
            break;
        } else if (CLEAR_STRING.to_string() == input ) {
            // 正常実行が完了していた直近のソースコードまで巻き上げる
            let backup_string : String = fs::read_to_string(&execute_file_path).unwrap();
            validate_file.set_len(0);
            validate_file.write_all(backup_string.as_bytes());
            input.clear();
            continue;
        } else if (DEL_STRING.to_string() == input) {
            validate_file.set_len(0);
            validate_file.write_all("<?php \r\n".as_bytes()).unwrap();
            execute_file.set_len(0);
            execute_file.write_all("<?php \r\n".as_bytes()).unwrap();
            input.clear();
            continue;
        } else {
            // 有効なコマンドとして評価する場合
        }

        // コマンドライン用の処理を通過後再度改行文字を付与する
        input.push_str("\n");

        let target_index : i32 = input.len() as i32 - 1;
        if str_position(&input, '\\' as u8) == target_index {
            // continue;
        }


        if (input.len() == 0) {
            continue;
        }
        validate_file.write_all(input.as_bytes());



        // 以下よりphpコマンドの実行
        let mut output_result : Result<Output, Error>;
        let mut output : Output;
        if cfg!(windows) {
            output_result = Command::new(&command).args(&[&validate_file_path]).output();
            if (output_result.is_ok() != true) {
                panic!("{}", output_result.unwrap_err().to_string());
            }
        } else {
            panic!("Your machine is unsupported.");
        }

        output = output_result.unwrap();
        let exit_code : Option<i32> = output.status.code();
        let mut for_output : Vec<u8> = Vec::new();
        // コマンドの実行結果が 「0」かどうかを検証
        if (exit_code.is_some() == true && exit_code.unwrap() == 0) {

            // 検証用ファイルでプログラムが正常終了した場合
            let mut temp_file : String = fs::read_to_string(&validate_file_path).unwrap();
            execute_file.set_len(0);

            // プログラムが正常終了している場合のみ改行出力を追加
            temp_file.push_str("\nprint(\"\n\"); \n");
            written_bytes = execute_file.write_all(temp_file.as_bytes());
            if (written_bytes.is_ok() != true) {
                panic!("Err: {}", written_bytes.unwrap_err().to_string());
            }

            // 検証用ファイルを再度削除し、改行出力を追加した分を再度保存
            validate_file.set_len(0);
            written_bytes = validate_file.write_all(temp_file.as_bytes());

            // 実行用ファイルで再度コマンド実行
            output_result = Command::new(&command).args(&[&execute_file_path]).output();
            if (output_result.is_ok() != true) {
                panic!("{}", output_result.unwrap_err().to_string());
            }
            output = output_result.unwrap();
            let mut for_output : Vec<u8> = Vec::new();
            let _output : Vec<u8> = output.stdout;
            let _enum = _output.iter().enumerate();
            for (index, value) in _enum {
                // 前回まで出力した分は破棄する
                if (previous_newline_count <= current_newline_count) {
                    for_output.push(*value);
                }
                if (*value == 10) {
                    current_newline_count = current_newline_count + 1;
                }

            }
            for_output.push(10);
            let executed_result: Result <String, FromUtf8Error> = String::from_utf8(for_output.clone());
            if (executed_result.is_ok() == true) {
                println!("{}", executed_result.unwrap());
            } else {
                print_c_string(for_output);
            }
            previous_newline_count = current_newline_count;
            current_newline_count = 0;
        } else {
            println!("Err2: Failed to be executed the command which you input on background!");
            println!("Err2: {}", String::from_utf8(output.stderr).unwrap());
        }
        input.clear();
    }
    // 終了コマンド実行後----
    println!("See you again!");
    // u32型から => i32型へキャスト
    process::exit(my_pid as i32);
}


// テキストに任意の文字列が含まれているかどうか
// 含まれない場合 -1の負の数を返却する
fn str_position (target : &String, needle : u8) -> i32
{
    check_type(target.as_bytes().iter().enumerate());
    for (index, value) in target.as_bytes().iter().enumerate() {
        if (needle == *value) {
            return (index as i32);
        }
    }
    return -1;
}


// 末尾の改行文字を削除
fn remove_newline(newline_string : &mut String)
{
    if (newline_string.len() == 0) {
        return ();
    }
    let new_vec : Vec<u8>;
    unsafe {
        {
            let _bytes = newline_string.as_mut_vec();
            // println!("{}", _bytes[_bytes.len() -1]);
            if (_bytes[_bytes.len() -1] == 10) {
                _bytes.pop();
            }
            if (_bytes[_bytes.len() - 1] == 13) {
                _bytes.pop();
            }
            check_type(&_bytes);
            new_vec = _bytes.to_vec();
            check_type(&_bytes);
            check_type(&new_vec);
        }
        let response : Result<String, FromUtf8Error> = String::from_utf8(new_vec);
        if (response.is_ok() == true) {
            *newline_string = response.unwrap();
        } else {
            *newline_string = response.unwrap_err().to_string();
        }
    }
}

fn check_type<T>(_: T) -> String {
    // println!("{}", std::any::type_name::<T>());
    return std::any::type_name::<T>().to_string();
}
