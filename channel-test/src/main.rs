#[tokio::main]
async fn main() {
    let (sen, rec) = flume::unbounded();

    for i in 0..5 {
        let sender = sen.clone();
        tokio::spawn(async move {
            println!("send={}", i);
            sender.send_async(i).await.unwrap();
        });
    }
    for _ in 0..7 {
        let receiver = rec.clone();
        tokio::spawn(async move {
            while !receiver.is_empty() {
                let result = receiver.recv_async().await.unwrap();
                println!("result={}", result);
            }
        });
    }

    // thread::spawn(move || async {
    //     (0..10).for_each(|i| async {
    //         println!("send={}", i);
    //         sen.send_async(i).await.unwrap();
    //         ()
    //     })
    // });

    // thread::spawn(move || async {
    //     (0..5).for_each(|_| async {
    //         let result = rec.recv_async().await.unwrap();
    //         println!("result={}", result);
    //         ()
    //     })
    // });
}
