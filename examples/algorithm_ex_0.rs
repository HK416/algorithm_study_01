/// 음식은 양손에 각각 하나씩 포크를 쥐어 먹어야 한다는 철학은 가진 4명의 철학자가
/// 테이블에 놓인 4개의 포크를 이용하여 주어진 음식을 먹어야 한다.
/// 
/// 가게 주인은 설거지 거리를 늘리기 싫어해 포크를 더 이상 제공하고 있지 않다.
/// 철학자들이 무사히 주어진 음식을 다 먹을 수 있도록 프로그래밍 해 보자.
/// 

use std::sync::{Arc, Mutex};
use std::thread;

const AMOUNT_OF_FOOD: u64 = 100_000; // 철학자는 어마무시하게 먹는것 같다.
const MAX_PHILOSOPHERS: usize = 4; // 최대 철학자의 수.
const MAX_FORKS: usize = 4; // 최대 포크의 갯수.

fn main() {
    let mut philosophers = Vec::with_capacity(MAX_PHILOSOPHERS);
    let amount_of_food = Arc::new(Mutex::new(AMOUNT_OF_FOOD));
    let table = Arc::new([
        Mutex::new(0), Mutex::new(1), Mutex::new(2), Mutex::new(3)
    ]);


    for id in 0..MAX_PHILOSOPHERS {
        let foods = amount_of_food.clone();
        let forks = table.clone();
        philosophers.push(thread::spawn(move || {
            while *foods.lock().unwrap() > 0 {
                let left_fork = forks[(id + 0) % MAX_FORKS].lock().unwrap();
                let right_fork = forks[(id + 1) % MAX_FORKS].lock().unwrap();

                let mut foods = foods.lock().unwrap();
                if *foods > 0 {
                    *foods -= 1;
                    println!("철학자 {}번은 {}번과 {}번 포크로 음식을 먹었다. (남은 음식량:{})", id, *left_fork, *right_fork, *foods);
                }
            }
        }));
    }

    for th in philosophers {
        th.join().unwrap();
    }

    println!("잘 먹었습니다.");
}