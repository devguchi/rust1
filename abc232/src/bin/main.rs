use itertools::{iproduct, Itertools};

fn main() {
    let v = (0..5).collect_vec();
    let v2 = (5..10).collect_vec();
    assert_eq!(v, vec![0, 1, 2, 3, 4]);

    let ans = v.iter().any(|&x| x > 3);
    assert_eq!(ans, true);

    let ans = v.iter().all(|&x| x >= 0);
    assert_eq!(ans, true);

    assert_eq!(v.iter().count(), 5);
    assert_eq!(v.iter().last().unwrap(), &4);

    assert_eq!(v.iter().nth(0).unwrap(), &0);
    assert_eq!(v.iter().nth(1).unwrap(), &1);
    let mut iter = v.iter();
    assert_eq!(iter.nth(2).unwrap(), &2);
    assert_eq!(iter.next().unwrap(), &3);
    assert_eq!(iter.nth(0).unwrap(), &4);

    let mut v12 = v.iter().zip(v2.iter());
    assert_eq!(v12.next(), Some((&0, &5)));
    assert_eq!(v12.next(), Some((&1, &6)));

    let str1 = vec!["hoge", "is", "good"]
        .iter()
        .copied()
        .intersperse(" ")
        .collect::<String>();
    assert_eq!(str1, String::from("hoge is good"));

    let mut v3 = v.iter().map(|x| x * 2);
    assert_eq!(v3.next(), Some(0));
    assert_eq!(v3.next(), Some(2));
    assert_eq!(v3.next(), Some(4));

    for (i, j) in iproduct!(0..2, 0..2) {
        println!("iproduct: ({},{})", i, j);
    }
    for (i, j) in (0..3).tuple_combinations() {
        println!("tuple_combinations: ({},{})", i, j);
    }
}
