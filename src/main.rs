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

    let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    let words_weighted_2 = create_word_index(lorem);

    for w in words_weighted_2
    {
        println!("{}", w.0);
        for sw in w.1
        {
            println!(" -> {} {}", sw.0, sw.1);
        }
    }

    //Build a vector with words, each word a weight - a chance to be chosen
    //This will be the occurrence of this word after the previous word in the provided text.
    let mut words_weighted = Vec::new();
    words_weighted.push((&lorem[0..5], 5));
    words_weighted.push((&lorem[6..11], 5));
    words_weighted.push((&lorem[12..17], 5));
    words_weighted.push((&lorem[18..21], 5));
    words_weighted.push((&lorem[22..26], 5));

    //Use the weight to create a distribution so we can run a random between 1 and highest and pick a value.
    let ref_words_weighted = &mut words_weighted;
    modify_to_distributed_range(ref_words_weighted);

    let mut words_printed = 0;
    loop {
        let result = pick_word(&words_weighted);
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

fn create_word_index (text: &str) -> Vec<(&str, Vec<(&str, i32)>)>
{
    let mut counter = 0;
    let mut word_index: Vec<(&str, Vec<(&str, i32)>)> = Vec::new();
    let mut word_start = 0;
    let mut preceding_word: &str = "";
    while counter < text.len()
    {
        if &text[counter..counter + 1] == " " || counter + 1 == text.len() 
        {
            if counter + 1 == text.len()
            {
                counter = counter + 1;
            }

            println!("word: {} ", &text[word_start..counter]);
            println!("preceded by word: {} ", preceding_word);

            if !word_index.iter().any(|known_word: &(&str, Vec<(&str, i32)>)| known_word.0 == &text[word_start..counter])
            {
                println!("word {} is new!", &text[word_start..counter]);
                word_index.push((&text[word_start..counter], Vec::new()));
            }
            
            let preceding_word_option = word_index.iter_mut().find(|known_word| known_word.0 == preceding_word);

            if preceding_word_option != None
            {
                let preceding_word_vec = preceding_word_option.unwrap();
                if !preceding_word_vec.1.iter().any(|subsequent_word| subsequent_word.0 == &text[word_start..counter])
                {
                    &preceding_word_vec.1.push((&text[word_start..counter], 1));
                }
                else 
                {
                    preceding_word_vec.1.iter_mut().find(|subsequent_word| subsequent_word.0 == &text[word_start..counter]).unwrap().1 += 1;
                }
            }

            preceding_word = &text[word_start..counter];

            //Start the next word
            word_start = counter + 1;

        }

        counter += 1;
    }

    word_index
}

fn modify_to_distributed_range(words_weighted: &mut Vec<(&str, i32)>)
{
    let mut sum = 0;
    for x in words_weighted {
        sum = sum + x.1;
        x.1 = sum;
    }
}

//Pick a value from the distributed vector.
fn pick_word(words: &Vec<(&str, i32)>) -> String
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