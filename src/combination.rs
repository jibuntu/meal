use std::rc::Rc;

pub struct Combination<T> {
    data_list: Vec<T>,
    length_of_combination: usize
}

impl<T> Combination<T> {
    pub fn new(data_list: Vec<T>, length_of_combination: usize) -> Combination<T> {
        Combination {
            data_list,
            length_of_combination
        }
    }

    pub fn iter(&self) -> CombIterator<T> {
        CombIterator::new(self)
    }
}

pub struct CombIterator<'a, T> {
    comb: &'a Combination<T>,
    keys_list: Vec<usize>,
    keys_list_len: usize,
    result: Rc<Vec<&'a T>>,
}

impl<'a, T> CombIterator<'a, T> {
    fn new(comb: &'a Combination<T>) -> CombIterator<'a, T> {
        let mut keys: Vec<usize> = (0..comb.data_list.len()).collect();
        let mut keys_list = Vec::new();
        let result = Rc::new(vec![&comb.data_list[0];comb.length_of_combination]);

        for i in 0..comb.length_of_combination {
            keys_list.push(comb.data_list.len() - i);
        }

        let keys_list_len = keys_list.len();

        CombIterator {
            comb,
            keys_list,
            keys_list_len,
            result
        }
    }

    fn change_keys_list(&mut self) {
        self.keys_list[self.keys_list_len - 1] -= 1;
        if self.keys_list[self.keys_list_len - 1] != 0 {
            return;
        }

        for i in 1..self.keys_list_len {
            let digit = self.keys_list_len - i;
            if self.keys_list[digit] < self.keys_list_len - digit {
                // 親の値を１つ消す
                self.keys_list[digit - 1] -= 1;
                // 親の値から１つ少ない数を自身に入れる
                self.keys_list[digit] = self.keys_list[digit - 1] - 1;

                // 子にも伝える
                for digit in digit+1..self.keys_list_len {
                    if self.keys_list[digit] < self.keys_list_len - digit {
                        self.keys_list[digit] = self.keys_list[digit - 1] - 1;
                    }
                }
            }
        }
    }
}

impl<'a, T: 'a> Iterator for CombIterator<'a, T> {
    type Item = Rc<Vec<&'a T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Rc::make_mut(&mut self.result);
        for (index, key) in self.keys_list.iter().enumerate() {
            if *key == 0 {
                return None;
            }
            result[index] = &self.comb.data_list[*key-1];
        }
        self.change_keys_list();

        Some(Rc::clone(&self.result))
    }
}

#[test]
fn test_comb_iterator() {
    let data_list = vec![1, 2, 3, 4, 5];
    let comb = Combination::new(data_list, 3);

    let mut iter = comb.iter();
    assert_eq!(iter.next(), Some(Rc::new(vec![&5, &4, &3])));
    assert_eq!(iter.next(), Some(Rc::new(vec![&5, &4, &2])));
    assert_eq!(iter.next(), Some(Rc::new(vec![&5, &4, &1])));
    assert_eq!(iter.next(), Some(Rc::new(vec![&5, &3, &2])));
    assert_eq!(iter.next(), Some(Rc::new(vec![&5, &3, &1])));
    assert_eq!(iter.next(), Some(Rc::new(vec![&5, &2, &1])));

    assert_eq!(iter.next(), Some(Rc::new(vec![&4, &3, &2])));
    assert_eq!(iter.next(), Some(Rc::new(vec![&4, &3, &1])));
    assert_eq!(iter.next(), Some(Rc::new(vec![&4, &2, &1])));

    assert_eq!(iter.next(), Some(Rc::new(vec![&3, &2, &1])));

    assert_eq!(iter.next(), None);

    let data_list = vec![1, 2, 3, 4, 5];
    let comb = Combination::new(data_list, 3);
    let list: Vec<usize> = comb.iter().map(|c| 0).collect();
    assert_eq!(list.len(), (5 * 4 * 3) / (3 * 2 * 1));

    let data_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let comb = Combination::new(data_list, 5);
    let list: Vec<usize> = comb.iter().map(|c| 0).collect();
    assert_eq!(list.len(), (10 * 9 * 8 * 7 * 6) / (5 * 4 * 3 * 2 * 1));
}

#[test]
#[ignore]
fn test_comb_speed() {
    let data_list = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
        31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    ];
    let comb = Combination::new(data_list, 5);
    let mut iter = comb.iter();
    let mut count = 2118760;

    for i in 0..count {
        iter.next();
    }

    println!("{}", count);
}