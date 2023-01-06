fn main() {
    let mut i : i32 = 0;

    while i < 10 {
        println!("while i = {},", i);
        i += 1;
    }

    loop {
        if i == 15 {
            break;
        }
        println!("loop i = {},", i);
        i += 1;
    }

    for j in 0..10 {
        println!("for j = {},", j);
    }

    let mut tab = Vec::new();
    tab.push(1);
    tab.push(2);
    tab.push(3);

    for elem in tab {
        println!("elem = {},", elem);
    }

    'first : for _ in 0..5 {
        'second : for j in 0..5 {
            'third : for k in 0..5 {
                if j > 3 {
                    break 'first;
                }
                if j % 2 == 0 {
                    continue 'second;
                }
                if k % 2 == 0 {
                    continue 'third;
                }
                println!("j = {} et k = {}", j, k);
            }
        }
    }
}