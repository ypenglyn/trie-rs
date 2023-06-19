use trie_rs::{Fid, Louds, LoudsNodeNum};

#[test]
fn from_str() {
    let fid = Fid::from("01");
    assert_eq!(fid[0], false);
    assert_eq!(fid[1], true);
}

#[test]
fn fuzzing_test() {
    let samples = 10000;

    fn access_from_bit_string(s: &str, i: u64) -> bool {
        s.chars().collect::<Vec<char>>()[i as usize] == '1'
    }

    fn rank_from_bit_string(s: &str, i: u64) -> u64 {
        let chs = s.chars().collect::<Vec<char>>();
        let mut rank: u64 = 0;
        for j in 0..=i as usize {
            if chs[j] == '1' {
                rank += 1
            };
        }
        rank
    }

    fn rank0_from_bit_string(s: &str, i: u64) -> u64 {
        let chs = s.chars().collect::<Vec<char>>();
        let mut rank0: u64 = 0;
        for j in 0..=i as usize {
            if chs[j] == '0' {
                rank0 += 1
            };
        }
        rank0
    }

    fn select_from_bit_string(s: &str, num: u64) -> Option<u64> {
        if num == 0 {
            return Some(0);
        }

        let mut cnt: u64 = 0;
        for (i, ch) in s.chars().enumerate() {
            if ch == '1' {
                cnt += 1;
            }
            if cnt == num {
                return Some(i as u64);
            }
        }
        None
    }

    fn select0_from_bit_string(s: &str, num: u64) -> Option<u64> {
        if num == 0 {
            return Some(0);
        }

        let mut cnt: u64 = 0;
        for (i, ch) in s.chars().enumerate() {
            if ch == '0' {
                cnt += 1;
            }
            if cnt == num {
                return Some(i as u64);
            }
        }
        None
    }

    for _ in 0..samples {
        let s = &format!("{:b}", rand::random::<u128>());
        eprintln!("build(): bit vec = \"{}\"", s);

        let fid = Fid::from(s.as_str());

        for i in 0..s.len() {
            eprintln!("[] op: bit vec = \"{}\", i = {}, ", s, i);
            assert_eq!(
                fid[i as u64],
                access_from_bit_string(s, i as u64),
                "bit vec = \"{}\", i={}, Index<u64>()={}, access_from_bit_string={}",
                s,
                i,
                fid[i as u64],
                access_from_bit_string(s, i as u64)
            );

            eprintln!("rank(): bit vec = \"{}\", i = {}, ", s, i);
            assert_eq!(
                fid.rank(i as u64),
                rank_from_bit_string(s, i as u64),
                "bit vec = \"{}\", i={}, Fid::rank()={}, rank_from_bit_string={}",
                s,
                i,
                fid.rank(i as u64),
                rank_from_bit_string(s, i as u64)
            );

            let num = i as u64;
            eprintln!("select(): bit vec = \"{}\", num = {}, ", s, num);
            assert_eq!(
                fid.select(num),
                select_from_bit_string(s, num),
                "bit vec = \"{}\", num={}, Fid::select()={:?}, select_from_bit_string={:?}",
                s,
                num,
                fid.select(num),
                select_from_bit_string(s, num)
            );

            eprintln!("rank0(): bit vec = \"{}\", i = {}, ", s, i);
            assert_eq!(
                fid.rank0(i as u64),
                rank0_from_bit_string(s, i as u64),
                "bit vec = \"{}\", i={}, Fid::rank0()={}, rank0_from_bit_string={}",
                s,
                i,
                fid.rank0(i as u64),
                rank0_from_bit_string(s, i as u64)
            );

            let num = i as u64;
            eprintln!("select0(): bit vec = \"{}\", num = {}, ", s, num);
            assert_eq!(
                fid.select0(num),
                select0_from_bit_string(s, num),
                "bit vec = \"{}\", num={}, Fid::select0()={:?}, select0_from_bit_string={:?}",
                s,
                num,
                fid.select0(num),
                select0_from_bit_string(s, num)
            );
        }
    }
}

#[test]
fn fuzzing_test2() {
    use rand::prelude::*;

    let samples = 100;
    let mut rng = rand::thread_rng();

    fn generate_lbs(rng: &mut ThreadRng) -> String {
        let mut s = String::from("10");
        let (mut cnt0, mut cnt1) = (1u64, 1u64);
        while cnt0 < cnt1 + 1 {
            let r = rng.gen::<f64>();
            if r < 0.6 {
                s = format!("{}{}", s, "0");
                cnt0 += 1;
            } else {
                s = format!("{}{}", s, "1");
                cnt1 += 1;
            }
        }
        s
    }

    for _ in 0..samples {
        let s = generate_lbs(&mut rng);
        eprintln!("build(): LBS = \"{}\"", s);

        let n_nodes = s.len() / 2;
        let louds = Louds::from(s.as_str());

        for raw_node_num in 1..=n_nodes {
            let node_num = LoudsNodeNum(raw_node_num as u64);
            eprintln!("NodeNum({:?})", raw_node_num);

            // index(node_num_to_index(node_num)) == node_num
            let index = louds.node_num_to_index(node_num);
            assert_eq!(louds.index_to_node_num(index), node_num);

            // `node_num`'s children have `node_num` as parent.
            for child_index in louds.parent_to_children(node_num) {
                assert_eq!(louds.child_to_parent(child_index), node_num);
            }
        }
    }
}
