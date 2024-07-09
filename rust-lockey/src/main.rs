use lockclient;
use lockservice;


fn main() {
    println!("Hello, world!");

    lockclient::callout();
    lockservice::callout();
}
