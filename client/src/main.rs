//使用系统标准库std::io,io prelude
use std::io::{self, prelude::*, BufReader, Write};
//使用系统网络库std::net
use std::net::TcpStream;
//使用系统标准库std::str;
use std::str;

fn main() -> std::io::Result<()> {
    //，创建一个流，发起链接到server端
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    //循环10次，发10次
    for _ in 0..10 {
        //创建空对象
        let mut input = String::new();
        //标准输入流 
        io::stdin()
        //读入一行到input
            .read_line(&mut input)
            //读取失败
            .expect("Failed to read from stdin");
        stream
        //将读取的数据写入流中
            .write(input.as_bytes())
            //写入失败
            .expect("Failed to write to stream");
        //使用BufReader,读
        let mut reader = BufReader::new(&stream);
        //创建 Vec<u8>;对象，缓冲
        let mut buffer: Vec<u8> = Vec::new();
        //读取
        reader
        //read_until标准输入读取数据，有换行，读到buffer中
            .read_until(b'\n', &mut buffer)
            //如果有错，打印
            .expect("Could not read into buffer");
        println!("read from server {}", 
        //读取之后打印，转换为buffer
            str::from_utf8(&buffer).expect("Could not write buffer as string"));//出错打印
        println!("");
    }
    //五任何报错信息
    Ok(())
}
