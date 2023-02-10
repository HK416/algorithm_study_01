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

struct Resource<const NRES: usize, const NTH: usize> {
    available: [usize; NRES],           // 이용 가능한 리소스
    allocation: [[usize; NRES]; NTH],   // 스레드 i가 확보 중인 리소스
    max: [[usize; NRES]; NTH]           // 스레드 i가 필요로 하는 리소스의 최댓값
}

impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
    fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Self { available, allocation: [[0; NRES]; NTH], max }
    }

    fn is_safe(&self) -> bool {
        let mut finish = [false; NTH]; // 스레드 i는 리소스 획득과 반환에 성공했는지 여부
        let mut work = self.available.clone(); // 이용 가능한 리소스의 시뮬레이션 값

        loop {
            // 모든 스레드 i와 리소스 j에 대해
            // finish[i] == false && work[j] >= need(i, j)를 만족하는 스레드를 찾는다.
            // 이때 need(i, j)는 (self.max[i][j] - self.allocation[i][j])
            let mut found = false;
            let mut num_true = 0;
            for (i, alloc) in self.allocation.iter().enumerate() {
                if finish[i] {
                    num_true += 1;
                    continue;
                }

                let need = self.max[i].iter().zip(alloc).map(|(m, a)| m - a);
                let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
                if is_avail {
                    // 스레드 i가 리소스 확보 가능
                    found = true;
                    finish[i] = true;
                    for (w, a) in work.iter_mut().zip(alloc) {
                        *w += a; // 스레드 i가 현재 확보하고 있는 리소스 반환.
                    }
                    break;
                }
            }

            if num_true == NTH {
                // 모든 스레드가 리소스 확보 가능하면 안전함.
                return true;
            }

            if !found {
                // 스레드가 리소스를 확보할 수 없음.
                break;
            }
        }
        false
    }

    fn take(&mut self, id: usize, resource: usize) -> bool {
        if id >= NTH || resource >= NRES || self.available[resource] == 0 {
            return false;
        }

        self.allocation[id][resource] += 1;
        self.available[resource] -= 1;

        if self.is_safe() {
            return true;
        }
        else {
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
            return false;
        }
    }

    fn release(&mut self, id: usize, resource: usize) {
        if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
            return;
        }

        self.allocation[id][resource] -= 1;
        self.available[resource] += 1;
    }
}

#[derive(Clone)]
struct Banker<const NRES: usize, const NTH: usize> {
    resource: Arc<Mutex<Resource<NRES, NTH>>>
}

impl<const NRES: usize, const NTH: usize> Banker<NRES, NTH> {
    fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Self { resource: Arc::new(Mutex::new(Resource::new(available, max))) }
    }

    fn take(&self, id: usize, resource: usize) -> bool {
        let mut r = self.resource.lock().unwrap();
        r.take(id, resource)
    }

    fn release(&self, id: usize, resource: usize) {
        let mut r = self.resource.lock().unwrap();
        r.release(id, resource)
    }
}

fn main() {
    let mut philosophers = Vec::with_capacity(MAX_PHILOSOPHERS);
    let amount_of_food = Arc::new(Mutex::new(AMOUNT_OF_FOOD));
    let table = Arc::new([
        Mutex::new(0), Mutex::new(1), Mutex::new(2), Mutex::new(3)
    ]);

    let banker = Banker::<MAX_FORKS, MAX_PHILOSOPHERS>::new(
        [1, 1, 1, 1], [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 1], [1, 0, 0, 1]]
    );


    for id in 0..MAX_PHILOSOPHERS {
        let banker = banker.clone();
        let foods = amount_of_food.clone();
        let forks = table.clone();
        philosophers.push(thread::spawn(move || {
            while *foods.lock().unwrap() > 0 {
                while !banker.take(id, (id + 0) % MAX_FORKS) { }
                while !banker.take(id, (id + 1) % MAX_FORKS) { }

                let left_fork = forks[(id + 0) % MAX_FORKS].lock().unwrap();
                let right_fork = forks[(id + 1) % MAX_FORKS].lock().unwrap();

                let mut foods = foods.lock().unwrap();
                if *foods > 0 {
                    *foods -= 1;
                    println!("철학자 {}번은 {}번과 {}번 포크로 음식을 먹었다. (남은 음식량:{})", id, *left_fork, *right_fork, *foods);
                }

                banker.release(id, (id + 1) % MAX_FORKS);
                banker.release(id, (id + 0) % MAX_FORKS);
            }
        }));
    }

    for th in philosophers {
        th.join().unwrap();
    }

    println!("잘 먹었습니다.");
}