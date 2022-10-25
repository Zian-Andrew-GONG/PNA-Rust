extern crate redis;

fn ping() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1")?;
    let mut con = client.get_connection()?;
    let res: String = redis::cmd("PING").query(&mut con)?;
    println!("{res}");
    Ok(())
}

fn main() -> redis::RedisResult<()> {
    println!("Hello, world!");
    ping()?;
    Ok(())
}
