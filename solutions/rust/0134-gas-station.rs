impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut i = 0;

        while i < n {
            let mut curr_gas = gas[i];
            let mut completed = true;

            for j in 0..n {
                let index = (i + j) % n;
                curr_gas -= cost[index];
                if curr_gas < 0 {
                    i += j; // skip invalid starting points
                    completed = false;
                    break;
                }
                curr_gas += gas[(index + 1) % n];
            }

            if completed {
                return i as i32;
            }
            i += 1;
        }

        -1
    }
}
