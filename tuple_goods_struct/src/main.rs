struct Item(String, i64);

fn main() {
    let banana: Item = Item("바나나".to_string(), 300);
    let apple: Item = Item("사과".to_string(), 200);
    let mango: Item = Item("망고".to_string(), 500);
    let items: Vec<Item> = vec![banana, apple, mango];
    let total: i64 = print_and_sum_items(&items);

    println!("합계는 {}원 입니다.", total);
}

fn print_tuple(item: &Item) {
    println!("{}를 {}원에 구입", item.0, item.1);
}

fn print_and_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;

    for i in items {
        print_tuple(&i);

        total += i.1;
    }

    total
}
