use std::io::{stdin, Read};
use rand::Rng;

fn main() {
    //Todo - get input somehow

    /*
    println!("Provide text input.");
    
    let mut input_str = String::new();
    
    stdin().read_line(&mut input_str).unwrap();

    println!("Data {}" input_str);
    */

    //Build a vector with words, each word a weight - a chance to be chosen
    //This will be the occurrence of this word after the previous word in the provided text.
    let mut words_weighted = Vec::new();
    words_weighted.push((String::from("Lorem"), 5));
    words_weighted.push((String::from("Ipsum"), 5));
    words_weighted.push((String::from("Dolor"), 5));
    words_weighted.push((String::from("Sit"), 5));
    words_weighted.push((String::from("Amet"), 14));

    //Use the weight to create a distribution so we can run a random between 1 and highest and pick a value.
    let words_distributed = create_distributed_range(&words_weighted);

    let mut words_printed = 0;
    loop {
        let result = pick_word(&words_distributed);
        print!("{} ", result);
        words_printed += 1;
        if words_printed == 50
        {
            break;
        }
    }

    /*
    println!("Press enter..");
    stdin().read(&mut [0]).unwrap();
    */
}

fn create_distributed_range(words_weighted: &Vec<(String, i32)>) -> Vec<(String, i32)>
{
    let mut sum = 0;
    let mut words_distributed = Vec::new();
    for x in words_weighted {
        sum = sum + x.1;
        words_distributed.push((String::from(x.0.to_owned()), sum));
    }
    words_distributed
}

//Pick a value from the distributed vector.
fn pick_word(words: &Vec<(String, i32)>) -> String
{
    let max_value = words.last().unwrap().1;
    let i: i32 = rand::thread_rng().gen_range(1, max_value);

    let mut counter = 0;
    loop {
        if words[counter].1 >= i
        {
            break words[counter].0.to_owned();
        }
        counter += 1;
    }
}