use num_bigint::{BigInt, Sign};
use std::{cell::RefCell, sync::Arc};

struct Monkey {
    items: RefCell<Vec<BigInt>>,
    operation: Box<dyn Fn(BigInt) -> BigInt>,
    throw_to: Box<dyn Fn(BigInt) -> usize>,
    modulo: u32,
}
fn main() {
    let monkey0 = Monkey {
        items: RefCell::new(vec![
            BigInt::from(64),
            BigInt::from(89),
            BigInt::from(65),
            BigInt::from(95),
        ]),
        operation: Box::new(|old| old * 7),
        throw_to: Box::new(|val| if val % 3 == BigInt::from(0) { 4 } else { 1 }),
        modulo: 3,
    };
    let monkey1 = Monkey {
        items: RefCell::new(vec![
            BigInt::from(76),
            BigInt::from(66),
            BigInt::from(74),
            BigInt::from(87),
            BigInt::from(70),
            BigInt::from(56),
            BigInt::from(51),
            BigInt::from(66),
        ]),
        operation: Box::new(|old| old + 5),
        throw_to: Box::new(|val| if val % 13 == BigInt::from(0) { 7 } else { 3 }),
        modulo: 13,
    };
    let monkey2 = Monkey {
        items: RefCell::new(vec![BigInt::from(91), BigInt::from(60), BigInt::from(63)]),
        operation: Box::new(|old| old.clone() * old),
        throw_to: Box::new(|val| if val % 2 == BigInt::from(0) { 6 } else { 5 }),
        modulo: 2,
    };
    let monkey3 = Monkey {
        items: RefCell::new(vec![
            BigInt::from(92),
            BigInt::from(61),
            BigInt::from(79),
            BigInt::from(97),
            BigInt::from(79),
        ]),
        operation: Box::new(|old| old + 6),
        throw_to: Box::new(|val| if val % 11 == BigInt::from(0) { 2 } else { 6 }),
        modulo: 11,
    };
    let monkey4 = Monkey {
        items: RefCell::new(vec![BigInt::from(93), BigInt::from(54)]),
        operation: Box::new(|old| old * 11),
        throw_to: Box::new(|val| if val % 5 == BigInt::from(0) { 1 } else { 7 }),
        modulo: 5,
    };
    let monkey5 = Monkey {
        items: RefCell::new(vec![
            BigInt::from(60),
            BigInt::from(79),
            BigInt::from(92),
            BigInt::from(69),
            BigInt::from(88),
            BigInt::from(82),
            BigInt::from(70),
        ]),
        operation: Box::new(|old| old + 8),
        throw_to: Box::new(|val| if val % 17 == BigInt::from(0) { 4 } else { 0 }),
        modulo: 17,
    };
    let monkey6 = Monkey {
        items: RefCell::new(vec![
            BigInt::from(64),
            BigInt::from(57),
            BigInt::from(73),
            BigInt::from(89),
            BigInt::from(55),
            BigInt::from(53),
        ]),
        operation: Box::new(|old| old + 1),
        throw_to: Box::new(|val| if val % 19 == BigInt::from(0) { 0 } else { 5 }),
        modulo: 19,
    };
    let monkey7 = Monkey {
        items: RefCell::new(vec![BigInt::from(62)]),
        operation: Box::new(|old| old + 4),
        throw_to: Box::new(|val| if val % 7 == BigInt::from(0) { 3 } else { 2 }),
        modulo: 7,
    };
    let monkeys = Arc::new(vec![
        monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7,
    ]);
    let multiply = monkeys.iter().fold(1, |acc, item| acc * item.modulo);
    let mut inspects = [0; 8];
    for round in 0..10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            while let Some(item) = monkey.items.borrow_mut().pop() {
                inspects[i] += 1;
                let item = (*monkey.operation)(item).clone();
                let monkeys = monkeys.clone();
                let monkey_to = monkeys
                    .get((*monkey.throw_to)(item.clone()) as usize)
                    .unwrap();
                monkey_to
                    .items
                    .borrow_mut()
                    .push(BigInt::from(item) % multiply);
            }
        }
    }
    inspects.sort();
    dbg!(inspects
        .into_iter()
        .rev()
        .take(2)
        .fold(BigInt::from(1), |acc, item| BigInt::from(acc)
            * BigInt::from(item)));
}
