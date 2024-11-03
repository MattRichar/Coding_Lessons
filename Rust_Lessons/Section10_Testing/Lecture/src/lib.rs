//cfg stands for configuration, and to only compile when <> is entered
//If you want to call any specific subset of the test is to type in the name for that subset.
#[cfg(test)]
mod tests{
    //In order for this function to be available, we need the following use
    use super::*;

    #[test]
    //This is a compiler directive to indicate that this will run if "test" is provided on the command line
    fn it_works(){
        let result = 2 + 2;
        //We are checking to see that result equals 4.
        assert_eq!(result,4);
    }

    #[test]
    //#[ignore] this tells the compiler to ignore.
    //#[should_panic] this tells the compiler to determine if the function will panic
    fn it_fails(){
        panic!("Test failed!");
    }

    #[test]
    fn call_simple_add(){
        assert!(simple_add());
    }
}

fn simple_add() -> bool {
    if 2+2 == 4{
        true
    }else {
        false
    }
}