use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let mut free = BinaryHeap::from(
            servers
                .iter()
                .enumerate()
                .map(|(index, &weight)| Reverse((weight, index)))
                .collect::<Vec<_>>(),
        );
        let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        let mut answer = Vec::with_capacity(tasks.len());
        let mut time = 0_i64;

        for (arrival, duration) in tasks.into_iter().enumerate() {
            time = time.max(arrival as i64);

            if free.is_empty() {
                time = time.max(busy.peek().unwrap().0.0);
            }

            while let Some(&Reverse((finish, index))) = busy.peek() {
                if finish > time {
                    break;
                }
                busy.pop();
                free.push(Reverse((servers[index], index)));
            }

            let Reverse((_, index)) = free.pop().unwrap();
            answer.push(index as i32);
            busy.push(Reverse((time + duration as i64, index)));
        }

        answer
    }
}
