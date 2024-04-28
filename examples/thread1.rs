use std::{sync::mpsc, thread};

const NUM_PRODUCERS: usize = 4;
fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(|| {
        for msg in rx {
            println!("consumer: idx: {}, value: {}", msg.idx, msg.value);
        }
        println!("consumer exit");
        42
    });

    let secret = consumer
        .join()
        .map_err(|e| anyhow::anyhow!("Thread join error: {:?}", e))?;

    print!("secret: {}", secret);

    Ok(())
}

#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> anyhow::Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg { idx, value })?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(std::time::Duration::from_millis(sleep_time as u64));
        if value % 5 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    Ok(())
}
