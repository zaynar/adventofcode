fn main() {
    let (mut x, mut y) = (0, 0);

    let mut num: i64 = 20151125;

    loop {
        //  To continue, please consult the code grid in the manual.  Enter the code at row 3010, column 3019.
        if y+1 == 3010 && x+1 == 3019 {
            println!("col={} row={} {}", x+1, y+1, num);
            break;
        }

        if y == 0 {
            y = x + 1;
            x = 0;
        } else {
            y -= 1;
            x += 1;
        }

        num = (num * 252533) % 33554393;

    }

}
