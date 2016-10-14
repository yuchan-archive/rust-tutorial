fn foo() {
    let mut v1 = vec![1,2,3];
    v1.pop();
    v1.push(4);

    for v in v1 {
        println!("print {}.", v)
    }
}

fn main() {
    let mut vs = vec![0,1,2,3,4,5,6];

    for v in &vs {
        println!("I'm borrowing {}.", v);
    }

    for v in &mut vs {
        *v = *v + 1;
        println!("I'm mutably borrowing {}.", v);
    }

    for v in vs {
        println!("I now own {}! AHAHAHAHAH!", v);
    }

    let x: i32 = 12;
    let y = x; // `i32` is `Copy`, so it's not moved :D
    println!("x still works: {}, and so does y: {}", x, y);

    foo();
}
