use oorandom::Rand32;

pub trait Weighted {
    fn get_weight_at(&self, index: usize) -> Option<usize>;

    fn get_random_index(&self, rng: &mut Rand32) -> Option<usize> {
        let mut total_weight = 0;
        let mut number_of_weights = 0;

        while let Some(weight) = self.get_weight_at(number_of_weights) {
            total_weight += weight;
            number_of_weights += 1;
        }

        let rand = rng.rand_range(0..total_weight as u32) as usize;
        let mut weight_sum = 0;
        for i in 0..number_of_weights {
            let weight = self.get_weight_at(i).expect("weight must exist");
            if rand >= weight_sum && rand < weight_sum + weight {
                return Some(i);
            }
            weight_sum += weight;
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct WeightedVec(Vec<(usize, String)>);

    impl Weighted for WeightedVec {
        fn get_weight_at(&self, index: usize) -> Option<usize> {
            self.0.get(index).map(|x| x.0)
        }
    }

    #[test]
    fn it() {
        let mut rng = Rand32::new(19950104);
        let weighted = WeightedVec(vec![
            (1, String::from("a")),
            (2, String::from("b")),
            (7, String::from("c")),
        ]);

        assert_eq!(weighted.get_weight_at(0), Some(1));
        assert_eq!(weighted.get_weight_at(1), Some(2));
        assert_eq!(weighted.get_weight_at(2), Some(7));
        assert_eq!(weighted.get_weight_at(3), None);

        let mut ones = 0;
        let mut twos = 0;
        let mut sevens = 0;
        for _i in 0..10000 {
            let w = weighted.get_random_index(&mut rng);
            match w {
                Some(0) => ones += 1,
                Some(1) => twos += 1,
                Some(2) => sevens += 1,
                Some(_) => panic!(),
                None => panic!(),
            }
        }

        assert_eq!(ones, 1004);
        assert_eq!(twos, 1951);
        assert_eq!(sevens, 7045);
    }
}
