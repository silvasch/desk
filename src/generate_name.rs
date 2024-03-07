use rand::seq::SliceRandom;

const ADJECTIVES: [&str; 50] = [
    "simple",
    "disturbed",
    "noxious",
    "handy",
    "breezy",
    "sincere",
    "zany",
    "relieved",
    "cruel",
    "milky",
    "lovely",
    "wooden",
    "yellow",
    "fluffy",
    "peaceful",
    "heavy",
    "ceaseless",
    "skillful",
    "roomy",
    "furry",
    "spicy",
    "guarded",
    "willing",
    "gaudy",
    "silent",
    "broken",
    "elfin",
    "pushy",
    "cloudy",
    "wasteful",
    "juicy",
    "annoyed",
    "itchy",
    "dreary",
    "useless",
    "daffy",
    "dapper",
    "secret",
    "lying",
    "muddled",
    "tacky",
    "jolly",
    "husky",
    "splendid",
    "filthy",
    "kaput",
    "many",
    "cautious",
    "narrow",
    "ruddy",
];

const NOUNS: [&str; 50] = [
    "river",
    "success",
    "problem",
    "disease",
    "unit",
    "person",
    "driver",
    "bathroom",
    "apple",
    "series",
    "diamond",
    "birthday",
    "homework",
    "volume",
    "payment",
    "wedding",
    "coffee",
    "science",
    "cancer",
    "teacher",
    "student",
    "city",
    "winner",
    "tennis",
    "affair",
    "freedom",
    "sample",
    "bonus",
    "guidance",
    "speaker",
    "uncle",
    "reading",
    "police",
    "extent",
    "session",
    "finding",
    "advice",
    "tension",
    "virus",
    "user",
    "topic",
    "football",
    "theory",
    "platform",
    "dinner",
    "software",
    "device",
    "music",
    "region",
    "boyfriend",
];

pub fn generate_name(taken_names: Vec<&String>) -> String {
    let mut name;

    loop {
        let mut rng = rand::thread_rng();
        let adjective = ADJECTIVES
            .choose(&mut rng)
            .expect("list of adjectives has more than 0 elements");
        let noun = NOUNS
            .choose(&mut rng)
            .expect("list of nouns has more than 0 elements");
        name = format!("{}-{}", adjective, noun);

        if !taken_names.contains(&&name) {
            break;
        }
    }

    name
}
