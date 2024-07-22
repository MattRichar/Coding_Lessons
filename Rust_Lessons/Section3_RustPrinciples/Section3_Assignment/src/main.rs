fn main() {

    let mut val = vec![1, 3, 5, 7];
    println!("{:?}", val);

    if zeroth_is_one(&val)
    {
        println!("The first value is one!");
    }

    add_fifteen(&mut val);
    println!("{:?}", val);

    let mut num = 2;
    println!("Number before adding two {}", num);
    add_two(&mut num);
    println!("Number after adding two {}", num);
}

fn zeroth_is_one(vec: &Vec<i32>) -> bool
{
    if vec[0] == 1
    {
        true
    }
    else
    {
        false
    }
}

fn add_fifteen(vec: &mut Vec<i32>)
{
    vec.push(15);
}

fn add_two(num: &mut i8)
{
    *num += 2;
}
