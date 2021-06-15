use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::{thread, time::Duration};


pub fn read_from_file(path_of_file:&std::path::Path) -> Result<String, io::Error>  {
    let mut f = File::open(path_of_file)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
 }


fn main()
{

    let input_path = Path::new("nmea-raw-signals.txt");
    
   if input_path.exists() {
    println!("Reading Raw Input File.");
    } 
    else {
        panic!("Input File not found.");
    }

    let contents = read_from_file(input_path);
    let contents = match contents {
       Ok(s) => s,
       Err(_) => "Can not read file content".to_string(),
   };

    use std::net::UdpSocket;
    let socket  = UdpSocket::bind("127.0.0.1:7788")
    .expect("can't bind socket to 127.0.0.1:7788");

    socket.connect("127.0.0.1:5001")
    .expect("can't bind socket to 127.0.0.1:5001");

    // Interval at which msg will be sent in seconds
    let sleep_period = Duration::from_secs(1);
    let mut count = 0;
    loop{
        // Send any message to specified IP:Port

        // socket.send("Hello From Client!\n\r".as_bytes()).expect("Can't send");

        count+=1;
        println!("Repeat msg count: {}",count);
        for line in contents.lines()
        {   
            socket.send(line.as_bytes()).expect("Can't send");
            println!("{:?}",line);            
            thread::sleep(sleep_period);
        }
    }

}
