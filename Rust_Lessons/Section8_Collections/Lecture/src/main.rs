//The Rust standard library contains many collection types that are easy to use.

//Needed to add the rand library into the cargo.toml file
use rand::seq::SliceRandom;
use rand::thread_rng;

//A binary heap is a collection whos elements are kept loosely organized. 
//The greatest value always bubbles up to the front.
// push, pop, and peek are the three methods for it
#[cfg(feature = "bheap")]
use std::collections::BinaryHeap;

//A map is a collection of key pair values, no two values can have the same key.
//There are two types of maps available in Rust, hash maps, and binary tree maps.
//hash maps are stored in a single heap allocated table.
//btree maps are stored in a tree like structure, growing from the top down into many leaves, also called nodes.
#[cfg(feature = "maps")]
use std::collections::HashMap;
use std::collections::BTreeMap;

//Sets are a collection we can use for quickly checking if a valu exists.
//Sets can never contain multiple copies of the same value. A set is similar
//to map, except a set only has keys and not a key-pair value.
#[cfg(feature = "sets")]
use::std::collections::HashSet;



fn main() {
    #[cfg(feature = "vectors")]
    {
        let mut nums: Vec<i32> = vec![];
        nums.push(1);
        nums.push(2);
        nums.push(3);
        //nums becomes [1,2,3]

        let pop = nums.pop(); //Option<T>, return Non or Some(T)
        //nums becomes [1,2]

        println!("{:?}", pop);

        let two = nums[1]; //copy
        //&nums[1], creates a reference if copy is not available.

        println!("{}", two);

        let one = nums.first(); // returns an Option<T>, so None if vec is empty, or Some<T> is [0]
        println!("{:?}", one);

        //.last() takes last value in vec
        //.first_mut and .last_mut, so will borrow mutable references

        println!("{}", nums.len());
        println!("{}", nums.is_empty());

        nums.insert(0,10);
        //nums becomes [10,1,2]
        nums.insert(3,12);
        //nums becomes [10,1,2,12]
        nums.insert(2,25);
        //nums becomes [10,1,25,2,12]
        //println!("{:?}", nums);

        nums.remove(3);
        //nums becomes [10,1,25,12]
        nums.sort();
        //nums becomes [1,10,12,25]
        println!("{:?}", nums);
        nums.reverse();
        //nums becomes [25,12,10,1]
        println!("{:?}", nums);

        for i in 1..5
        {
            nums.shuffle(&mut thread_rng());
            println!("Shuffle {:?}, gives num of {:?}", i,nums);
        }
    }

    #[cfg(feature = "bheap")]
    {
        let mut bheap = BinaryHeap::new();

        bheap.push(1);
        bheap.push(42);
        bheap.push(7);
        bheap.push(20);

        println!("{:?}", bheap);

        bheap.pop();

        println!("{:?}", bheap);
        println!("{:?}", bheap.peek()); //peek is going to return Option<T>, return None if empty, or Some(T) otherwise
        
    }

    #[cfg(feature = "maps")]
    {
        let mut hm = HashMap::new();

        //hm.insert(key: <key_value>, value: <value>);
        hm.insert(1,1);
        hm.insert(5,2);
        hm.insert(30,3);
        println!("{:?}", hm);

        let old = hm.insert(30,4); //key is going to update the old value of 3, to the new value of 4. It will also return the old value.
        println!("{:?}", hm);
        println!("{:?}", old);

        println!("{}", hm.contains_key(&8));//returns false
        println!("{}", hm.contains_key(&5));//returns true

        println!("{:?}", hm.get(&5));//returns Option<T>

        let one = hm.remove(&1);
        println!("{:?}", hm);//returns {5:2, 30:4}
        println!("{:?}", one);//returns Some(1)

        let remove = hm.remove_entry(&5);
        println!("{:?}", remove);//returns Some((5,2))
        println!("{:?}", hm);//returns {30:4}

        hm.clear();
        println!("{:?}", hm.is_empty());//returns true
    }

    #[cfg(feature = "BTrees")]
    {
        let mut solar_system_map = BTreeMap::new();
        //BTreeMap<&str, f64>
        let planets = [
            ("Mercury", 0.4),
            ("Venus", 0.7),
            ("Earth", 1.0),
            ("Mars", 1.5),
            ("Jupiter", 5.2),
            ("Saturn", 9.5),
            ("Uranus", 19.0),
            ("Neptune", 30.0),
        ];

        for (planet,distance) in &planets{
            solar_system_map.insert(planet, distance);
        }
        println!("Planets based in Astronomical Units (AU)");

        for (planet, distance) in &solar_system_map {
            println!("{}: {} AU", planet, distance);
        }
    }
    

    //There is a hashset and a btree set.
    #[cfg(feature = "sets")]
    {
        let mut hs = HashSet::new();

        hs.insert(1);
        hs.insert(2);
        hs.insert(3);
        hs.insert(4);

        for x in hs.iter(){
            println!("Iter: {}", x);
        }
        
        //Removes the second element
        hs.remove(&2);

        for x in hs.iter(){
            println!("Iter: {}", x);
        }

        let mut hs2 = HashSet::new();

        hs2.insert(1);
        hs2.insert(3);
        hs2.insert(5);
        hs2.insert(7);

        for x in hs.intersection(&hs2){
            println!("Intersection: {}", x);
        }

        let intersection = &hs & &hs2;//shorthand intersection using the binary bitwise & operator

        for x in intersection{
            println!("Short hand way: {}", x);
        }

        let union = &hs | &hs2;

        for x in union{
            println!("Short hand union way: {}", x);
        }

        let dif = &hs - &hs2;

        for x in dif{
            println!("Short hand dif way: {}", x);
        }

    }
}


//C++ equivalent for conditional compilation
// #define ALLOWED
// #define NOTALLOWED

// int main(){
//     #ifdef ALLOWED

//     std::cout<< "This is allowed" <<std::endl;
//     Vector<int> Vec[];
//     #ifdef NOTALLOWED
//         for(int i = 0; i=100;i++)
//         {
//             Vec[i] = i;
//         }

//         api_call(Vec, post);
//     #endif

//     #endif
//     #ifdef NOTALLOWED
//     std::cout<< "This is not allowed" <<std::endl;
//     #endif

// }


