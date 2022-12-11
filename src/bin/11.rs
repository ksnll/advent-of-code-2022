use std::{cell::RefCell, sync::Arc};

struct Monkey {
    items: RefCell<Vec<u32>>,
    operation: Box<dyn Fn(u32) -> u32>,
    throw_to: Box<dyn Fn(u32) -> u32>,
}
fn main() {
    let monkey0 = Monkey {
        items: RefCell::new(vec![64, 89, 65, 95]),
        operation: Box::new(|old| old * 7),
        throw_to: Box::new(|val| if val % 3 == 0 { 4 } else { 1 }),
    };
    let monkey1 = Monkey {
        items: RefCell::new(vec![76, 66, 74, 87, 70, 56, 51, 66]),
        operation: Box::new(|old| old + 5),
        throw_to: Box::new(|val| if val % 13 == 0 { 7 } else { 3 }),
    };
    let monkey2 = Monkey {
        items: RefCell::new(vec![91, 60, 63]),
        operation: Box::new(|old| old * old),
        throw_to: Box::new(|val| if val % 2 == 0 { 6 } else { 5 }),
    };
    let monkey3 = Monkey {
        items: RefCell::new(vec![92, 61, 79, 97, 79]),
        operation: Box::new(|old| old + 6),
        throw_to: Box::new(|val| if val % 11 == 0 { 2 } else { 6 }),
    };
    let monkey4 = Monkey {
        items: RefCell::new(vec![93, 54]),
        operation: Box::new(|old| old * 11),
        throw_to: Box::new(|val| if val % 5 == 0 { 1 } else { 7 }),
    };
    let monkey5 = Monkey {
        items: RefCell::new(vec![60, 79, 92, 69, 88, 82, 70]),
        operation: Box::new(|old| old + 8),
        throw_to: Box::new(|val| if val % 17 == 0 { 4 } else { 0 }),
    };
    let monkey6 = Monkey {
        items: RefCell::new(vec![64, 57, 73, 89, 55, 53]),
        operation: Box::new(|old| old + 1),
        throw_to: Box::new(|val| if val % 19 == 0 { 0 } else { 5 }),
    };
    let monkey7 = Monkey {
        items: RefCell::new(vec![62]),
        operation: Box::new(|old| old + 4),
        throw_to: Box::new(|val| if val % 7 == 0 { 3 } else { 2 }),
    };
    let monkeys = Arc::new(vec![
        monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7,
    ]);
    let mut inspects = [0; 8];
    for _ in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            while let Some(item) = monkey.items.borrow_mut().pop() {
                inspects[i] += 1;
                let item = (*monkey.operation)(item) % 3;
                (monkeys
                    .clone()
                    .get((*monkey.throw_to)(item) as usize)
                    .unwrap())
                .items
                .borrow_mut()
                .push(item);
            }
        }
    }
    inspects.sort();
    dbg!(inspects.into_iter().rev().take(2).collect::<Vec<usize>>());
}
