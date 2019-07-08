struct Combination<T> {
    data_list: Vec<T>,
    length_of_combination: usize
}

impl<T> Combination<T> {
    fn new(data_list: Vec<T>, length_of_combination: usize) -> Combination<T> {
        Combination {
            data_list,
            length_of_combination
        }
    }

    fn iter(&self) -> CombIterator<T> {
        CombIterator::new(self)
    }
}

struct CombIterator<'a, T> {
    comb: &'a Combination<T>,
    keys_list: Vec<Vec<usize>>
}

impl<'a, T> CombIterator<'a, T> {
    fn new(comb: &'a Combination<T>) -> CombIterator<'a, T> {
        let mut keys: Vec<usize> = (0..comb.data_list.len()).collect();
        let mut keys_list = Vec::new();

        for _i in 0..comb.length_of_combination {
            keys_list.push(keys.clone());
            keys.pop();
        }

        CombIterator {
            comb,
            keys_list
        }
    }

    fn change_keys_list(&mut self) {
        self.keys_list.last_mut().unwrap().pop();

        for i in 1..self.keys_list.len() {
            let digit = self.keys_list.len() - i;
            if self.keys_list[digit].len() < self.keys_list.len() - digit {
                // 親の値を１つ消す
                self.keys_list[digit - 1].pop();
                let mut parent = self.keys_list[digit - 1].clone();
                parent.pop();

                self.keys_list[digit] = parent;

                // 子にも伝える
                for digit in digit+1..self.keys_list.len() {
                    if self.keys_list[digit].len() < self.keys_list.len() - digit {
                        let mut parent = self.keys_list[digit - 1].clone();
                        parent.pop();
                        self.keys_list[digit] = parent;
                    }
                }
            }
        }
    }
}

impl<'a, T: 'a> Iterator for CombIterator<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut key_list = Vec::new();
        for keys in &self.keys_list {
            if let Some(key) = keys.last() {
                key_list.push(*key);
            } else {
                return None;
            }

        }

        // keys_listを元に値のリストを作成
        let mut value_list = Vec::new();
        for key in key_list {
            value_list.push(&self.comb.data_list[key]);
        }

        self.change_keys_list();

        Some(value_list)
    }

}

#[test]
fn test_comb_iterator() {
    let data_list = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30
    ];
    let comb = Combination::new(data_list, 5);
    //let mut iter = comb.iter();

    for c in comb.iter() {
    }

    // 0.6s
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());

}
