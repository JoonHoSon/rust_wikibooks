#[macro_export]
macro_rules! basic_for {
    // for i = 1 to 10과 같이 작성하는 경우
    (
        for $i:ident = $from:tt to $to:tt
        $block:block
    ) => {{
        for $i in $from..=$to {
            $block
        }
    }};

    // for i = 1 to 10 step 2와 같이 작성하는 경우
    (
        for $i:ident = $from:tt to $to:tt step $step:tt
        $block:block
    ) => {{
        let mut $i = $from;

        loop {
            if $i > $to {
                break;
            }

            $block
            $i += $step;
        }

    }};
}

fn main() {
    let mut total = 0;

    basic_for! {
        for i = 1 to 10 {
            total += i;
        }
    }

    println!("{}", total);

    basic_for!(
        for i = 0 to 10 step 3 {
            println!("i = {}", i);
        }
    );
}
