mod mpsc;

use std::{thread, time::Duration};

use mpsc::mpsc::sync_channel;

fn main() {
    let (sender, receiver) = sync_channel::<i32>(3);

    let mut sender_1 = sender.clone();
    let sender_handle_1 = thread::spawn(move || {
        println!("Send (11)");
        sender_1.send(11);
        println!("Send (12)");
        sender_1.send(12);
        println!("Send (13)");
        sender_1.send(13);
    });

    let mut sender_2 = sender.clone();
    let sender_handle_2 = thread::spawn(move || {
        // thread::sleep(Duration::from_secs(2));

        println!("Send (21)");
        sender_2.send(21);
        println!("Send (22)");
        sender_2.send(22);
        println!("Send (23)");
        sender_2.send(23);
    });

    let mut sender_3 = sender;

    let sender_handle_3 = thread::spawn(move || {
        // thread::sleep(Duration::from_secs(4));

        println!("Send (31)");
        sender_3.send(31);
        println!("Send (32)");
        sender_3.send(32);
        println!("Send (33)");
        sender_3.send(33);
    });

    // let receiver_handle = thread::spawn(move || loop {
    //     if let Some(data) = receiver.receive() {
    //         println!("Received {data}");
    //     } else {
    //         println!("No more senders...");
    //         break;
    //     }
    // });

    let receiver_handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        for data in receiver {
            println!("Received {data}");
            thread::sleep(Duration::from_secs(1));
        }
        println!("No more senders...");
    });

    let _ = receiver_handle.join().unwrap();
    let _ = sender_handle_1.join().unwrap();
    let _ = sender_handle_2.join().unwrap();
    let _ = sender_handle_3.join().unwrap();

    println!("Hello, world!");
}
