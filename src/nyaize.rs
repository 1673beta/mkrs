use fancy_regex::Regex;

pub fn nyaize(input: &str) -> String {
    let en_regex1 = Regex::new(r"(?<=n)a").unwrap();
    let en_regex2 = Regex::new(r"(?<=morn)ing").unwrap();
    let en_regex3 = Regex::new(r"(?<=every)one").unwrap();
    let ko_regex1 = Regex::new(r"[나-낳]").unwrap();
    let ko_regex2 = Regex::new(r"(다$)|(다(?=\.))|(다(?= ))|(다(?=!))|(다(?=\?))").unwrap();
    let ko_regex3 = Regex::new(r"(야(?=\?))|(야$)|(야(?= ))").unwrap();

    let mut result = input.to_string();

    // ja-JP
    result = result
        .replace("な", "にゃ")
        .replace("ナ", "ニャ")
        .replace("ﾅ", "ﾆｬ");

    // en-US
    result = en_regex1
        .replace_all(
            &result,
            |caps: &fancy_regex::Captures| {
                if &caps[0] == "A" {
                    "YA"
                } else {
                    "ya"
                }
            },
        )
        .to_string();
    result = en_regex2
        .replace_all(
            &result,
            |caps: &fancy_regex::Captures| {
                if &caps[0] == "ING" {
                    "YAN"
                } else {
                    "yan"
                }
            },
        )
        .to_string();

    result = en_regex3
        .replace_all(
            &result,
            |caps: &fancy_regex::Captures| {
                if &caps[0] == "ONE" {
                    "NYAN"
                } else {
                    "nyan"
                }
            },
        )
        .to_string();

    // ko-KR
    result = ko_regex1
        .replace_all(&result, |caps: &fancy_regex::Captures| {
            let ch = caps[0].chars().next().unwrap();
            let new_ch = std::char::from_u32(ch as u32 + '냐' as u32 - '나' as u32).unwrap();
            new_ch.to_string()
        })
        .to_string();

    result = ko_regex2.replace_all(&result, "다냥").to_string();
    result = ko_regex3.replace_all(&result, "냥").to_string();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nyaize_jp() {
        let result = nyaize("なす");
        assert_eq!(result, "にゃす");
    }

    #[test]
    fn nyaize_en() {
        let result = nyaize("good morning everyone");
        assert_eq!(result, "good mornyan everynyan");
    }
}
