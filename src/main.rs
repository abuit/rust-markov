use rand::Rng;

fn main() {
    //let test_input = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let test_input = "Als vliegen vliegen achterna";
    let word_graph = create_word_graph(test_input);

    //Start with first word
    let mut words_printed = 0;
    let mut next_word = word_graph.first().unwrap().0.to_owned();
    print!("{} ", next_word);

    loop {
        let next_word_choices = &word_graph.iter().find(|word| String::from(word.0) == next_word).unwrap().1;
        let possible_next_word = pick_word(next_word_choices);
        if possible_next_word == None
        {
            break;
        }
        else 
        {
            next_word = possible_next_word.unwrap();
            print!("{} ", next_word);
            words_printed += 1;
            if words_printed == 50
            {
                break;
            }
        }
    }
}

fn create_word_graph(text: &str) -> Vec<(&str, Vec<(&str, i32)>)>
{
    let mut counter = 0;
    let mut word_graph: Vec<(&str, Vec<(&str, i32)>)> = Vec::new();
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

            //The found word is &text[word_start..counter]);
            if !word_graph.iter().any(|known_word: &(&str, Vec<(&str, i32)>)| known_word.0 == &text[word_start..counter])
            {
                //The word is new
                word_graph.push((&text[word_start..counter], Vec::new()));
            }
            
            //Find the preceding word
            if let Some(preceding_word_vec) = word_graph.iter_mut().find(|known_word| known_word.0 == preceding_word)
            {
                if !preceding_word_vec.1.iter().any(|subsequent_word| subsequent_word.0 == &text[word_start..counter])
                {
                    //The subsequent word is new. Initialize with 1 occurrence.
                    &preceding_word_vec.1.push((&text[word_start..counter], 1));
                }
                else 
                {
                    //The subsequent word is not new, update with 1 occurrence
                    preceding_word_vec.1.iter_mut().find(|subsequent_word| subsequent_word.0 == &text[word_start..counter]).unwrap().1 += 1;
                }
            }

            //Save the preceding word for the next word
            preceding_word = &text[word_start..counter];

            //Start the next word
            word_start = counter + 1;

        }

        counter += 1;
    }

    word_graph
}

//Pick a value from the possible weighted choices.
fn pick_word(words: &Vec<(&str, i32)>) -> Option<String>
{
    if words.len() == 0
    {
        return None;
    }
    else if words.len() == 1
    {
        return Some(words.first().unwrap().0.to_owned());
    }

    //Create a distributed range from the weighted values.
    //For instance (word1, 1) (word2, 5) (word3, 2) will become (word1, 1), (word2, 6), (word3, 8). 
    //Using a random value between 1-8, pick the first node that's greater than or equal to this random value.
    //This will result in a weighted chance for every word
    let dist_words = create_distributed_range(words);
    let max_value = dist_words.last().unwrap().1;
    let i: i32 = rand::thread_rng().gen_range(0, max_value);

    let mut counter = 0;
    loop {
        //max_value is not included so use greater then instead of greater then or equal
        if dist_words[counter].1 > i
        {
            break Some(dist_words[counter].0.to_owned());
        }
        counter += 1;
    }
}

fn create_distributed_range(words_weighted: &Vec<(&str, i32)>) -> Vec<(String, i32)>
{
    //Todo: Don't create a new vector but introduce another i32 in the tuples for the distribution.
    //Update those accordingly with the sum. We do need a mutable reference.
    let mut dist_words = Vec::new();
    let mut sum = 0;
    for x in words_weighted {
        sum = sum + x.1;
        dist_words.push((x.0.to_owned(), sum));
    }

    dist_words
}