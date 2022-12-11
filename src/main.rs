#![feature(get_many_mut)]

use std::env;
use crate::tasks::execute;

mod tasks;

fn main(){
    let day_n = env::args()
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    println!("{}", execute(day_n));
}
