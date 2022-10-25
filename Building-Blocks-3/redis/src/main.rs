use redis::Commands;

extern crate redis;

fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    redis::cmd("SET").arg("my_key").arg(42).query(&mut con)?;
    let v: String = con.get("my_key")?;
    println!("{v}");
    Ok(())
}

fn main() -> redis::RedisResult<()> {
    println!("Hello, world!");
    do_something()?;
    Ok(())
}
