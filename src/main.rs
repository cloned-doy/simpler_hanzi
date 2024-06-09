use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use bincode::deserialize_from;

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    key: String,
    value: i32,
}

// Static initialization of the entries
static ENTRIES: Lazy<Vec<Entry>> =
    Lazy::new(|| deserialize_from(&include_bytes!("./data/data.profile")[..]).unwrap());

// Create an index for fast search
static INDEX: Lazy<HashMap<String, usize>> = Lazy::new(|| create_index(&ENTRIES));

// Function to create the index
fn create_index(entries: &Vec<Entry>) -> HashMap<String, usize> {
    let mut index = HashMap::new();
    for (i, entry) in entries.iter().enumerate() {
        index.insert(entry.key.clone(), i);
    }
    index
}

// Function to search for an entry
fn search_entry<'a>(key: &'a str, index: &'a HashMap<String, usize>, entries: &'a Vec<Entry>) -> Option<&'a Entry> {
    index.get(key).map(|&i| &entries[i])
}

fn is_hanzi(c: char) -> bool {
    // Check if the character falls within the Hanzi range
    ('\u{4E00}'..='\u{9FFF}').contains(&c)
}

fn filter_hanzi(sentence: &str) -> String {
    sentence.chars().filter(|&c| is_hanzi(c)).collect()
}

// calculate total score from any given sentence. 
// the function will try to search each first four characters from the beginning of sentence, 
// if the search success it will continue to the rest sentence. 
// if no result is founded it will retry to search by less and less character 
// untill try to search using only just a character from the index. 
// if not found, it will throw default score 500000. dont ask me why.
// the function will return the total score of earch founded characters of the given sentence.
fn calculate_score<'a>(sentence: &'a str, index: &'a HashMap<String, usize>, entries: &'a Vec<Entry>) -> i32 {
    let mut total_score = 0;
    let mut remaining_sentence = filter_hanzi(sentence);

    while !remaining_sentence.is_empty() {
        let mut found = false;
        for &i in [1, 2, 3, 4].iter().rev() {
            if let Some((idx, char)) = &remaining_sentence.char_indices().nth(i - 1) {
                let prefix = &remaining_sentence[..idx + char.len_utf8()];
                if let Some(entry) = search_entry(prefix, index, entries) {
                    total_score += entry.value;
                    remaining_sentence = remaining_sentence[idx + char.len_utf8()..].to_string();
                    found = true;
                    break;
                }
                
            }
            if i == 1 && !found {
                if let Some((idx, char)) = &remaining_sentence.char_indices().nth(i - 1){
                    remaining_sentence = remaining_sentence[idx + char.len_utf8()..].to_string();

                }
                
            }
        }
        if !found && remaining_sentence.is_empty() {
            // total_tokens += 1;
            total_score = 500000;
            break;
        }
    }

    // total_score = total_score / total_tokens;
    total_score
}

fn print_sentence_complexity(sentences: Vec<&str>, index: &HashMap<String, usize>, entries: &Vec<Entry>) {
    // Calculate scores for all sentences
    let mut scored_sentences: Vec<_> = sentences.iter().map(|&sentence| {
        let score = calculate_score(sentence, index, entries);
        (sentence, score)
    }).collect();

    // Sort the scored sentences based on scores
    scored_sentences.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Print the sentences
    for (sentence, score) in scored_sentences {
        println!("Score :{:<7} {:?}", score, sentence.chars().take(40).collect::<String>());
    }
}


fn main() {
    let sentences = vec![
        "短篇小说，小说的一种。其特点是篇幅短小、情节简洁、  人物集中、结构精巧。它往往选取和描绘富有典型意义的生活片断，着力刻画主要人物的性格特征，反映生活的某一侧面，使读者“借一斑略知全豹”。正如茅盾所说：“短篇小说主要是抓住一个富有典型意义的生活片断，来说明一个问题或表现比它本身广阔得多、也复杂得多的社会现象的。”这“也就决定了它的篇幅不可能长，它的故事不可能发生于长年累月(有些短篇小说的故事只发生于几天或几小时之内)，它的人物不可能太多，而人物也不可能一定要有性格的发展”.",
        "核武器（英文：Nuclear Weapon [8]），是利用能自持进行的原子核裂变或聚变反应瞬时释放的巨大能量、产生爆炸，实施大规模杀伤破坏的武器。核武器是军事威慑力量的重要组成部分 [5]核武器也叫核子武器或原子武器，利用核反应的光热辐射、电磁脉冲、冲击波和感生放射性造成杀伤和破坏作用，以及造成大面积放射性污染，阻止对方军事行动以达到战略目的的大杀伤力武器的总称。核武器主要包括裂变武器（第一代核武，通常称为原子弹）和聚变武器（亦称为氢弹，分为两级及三级式），亦有些还在武器内部放入具有感生放射的轻元素，以增大辐射强度扩大污染，或加强中子放射以杀伤人员（如中子弹）",
        "我是学生。",
        "灭火器是一种可携式灭火工具。灭火器内放置化学物品，用以救灭火灾。灭火器是常见的消防器材之一，存放在公众场所或可能发生火灾的地方，不同种类的灭火器内装填的成分不一样，是专为不同的火灾起因而设。使用时必须注意以免产生反效果及引起危险。",
        "我们去公园玩。",
        "我们一起学习。",
        "我的家在城市里。",
        "你喜欢喝茶还是咖啡？",
        "我喜欢听音乐。",
        "你家有几口人？",
    ];

    print_sentence_complexity(sentences, &INDEX, &ENTRIES);
}

