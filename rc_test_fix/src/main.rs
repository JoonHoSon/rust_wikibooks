use std::rc::Rc;

fn main() {
    let a_rc = Rc::new(1000);

    {
        let b_rc = Rc::clone(&a_rc);
        println!("{}", b_rc);

        println!("a_rc reference count : {}", Rc::strong_count(&a_rc));
    }

    println!("a_rc reference count : {}", Rc::strong_count(&a_rc));
}
