extern crate env_logger;
extern crate mvneer;

fn main() {

    env_logger::init().unwrap();

    //println!("Hello, world!");
    mvneer::Client::run("org.typelevel", "cats_2.11").unwrap();
}
