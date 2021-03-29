//使用标准std::io库,处理错误
use std::io::{Error, Read, Write};
//使用socket标准std::net网络库,TcpListener,TcpStream
use std::net::{TcpListener, TcpStream};
//使用标准std::thread线程库，线程池
use std::thread;
//使用标准时间库
use std::time;

//监听TCP客户端链接,处理流
fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    //定义数组buf大小为512,初始值为0
    let mut buf = [0; 512];
    //遍1000次循环
    for _ in 0..1000 {
        //从流中读取特定数量的字节到buf
        let bytes_read = stream.read(&mut buf)?;
        //没有读到数据
        if bytes_read == 0 {
           //没有任何错误发生,Result::Ok
            return Ok(());
        }
        //函数-向该写入器中写入一个缓冲区，写回去
        //向buf中写入bytes_read 字节
        stream.write(&buf[..bytes_read])?;
        //睡眠1s
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(())//只要执行到这里就说明完全没有错误了
}


fn main() -> std::io::Result<()> {
    //绑定本地IP:Port
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    //创建容器放线程池句柄
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    //返回连结接收的迭代器
    for stream in listener.incoming() {
        //转流，转换失败,failed
        let stream = stream.expect("failed!");
        //创建一个的线程，闭包处理，handle_client，stream使用unwrap_or_else
        //handle类型
        let handle = thread::spawn(move || {
            handle_client(stream)
        //可以进行一些自定义的非 panic! 的错误处理
        //打印error
        .unwrap_or_else(|error| eprintln!("{:?}", error));
        });

        //将线程句柄加入到容器里面
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        //等待每个生线程结束
        handle.join().unwrap();
    }
//只要执行到这里就说明完全没有错误了，返回ok,错误返回 error
    Ok(())
}