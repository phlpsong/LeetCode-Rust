struct Solution { }

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut cnt = [0; 5];
        text.chars().for_each(|c| match c {
            'b' => cnt[0] += 1,
            'a' => cnt[1] += 1,
            'l' => cnt[2] += 1,
            'o' => cnt[3] += 1,
            'n' => cnt[4] += 1,
            _ => (),
        });
        cnt[2] >>= 1;
        cnt[3] >>= 1;
        *cnt.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_number_of_balloons() {
        let case1 = Solution::max_number_of_balloons("nlaebolko".to_string());
        assert_eq!(case1, 1);

        let case2 = Solution::max_number_of_balloons("loonbalxballpoon".to_string());
        assert_eq!(case2, 2);

        let case3 = Solution::max_number_of_balloons("leetcode".to_string());
        assert_eq!(case3, 0);

        let case4 = Solution::max_number_of_balloons("balon".to_string());
        assert_eq!(case4, 0);

        let case5 = Solution::max_number_of_balloons("krhizmmgmcrecekgyljqkldocicziihtgpqwbticmvuyznragqoyrukzopfmjhjjxemsxmrsxuqmnkrzhgvtgdgtykhcglurvppvcwhrhrjoislonvvglhdciilduvuiebmffaagxerjeewmtcwmhmtwlxtvlbocczlrppmpjbpnifqtlninyzjtmazxdbzwxthpvrfulvrspycqcghuopjirzoeuqhetnbrcdakilzmklxwudxxhwilasbjjhhfgghogqoofsufysmcqeilaivtmfziumjloewbkjvaahsaaggteppqyuoylgpbdwqubaalfwcqrjeycjbbpifjbpigjdnnswocusuprydgrtxuaojeriigwumlovafxnpibjopjfqzrwemoinmptxddgcszmfprdrichjeqcvikynzigleaajcysusqasqadjemgnyvmzmbcfrttrzonwafrnedglhpudovigwvpimttiketopkvqw".to_string());
        assert_eq!(case5, 10);
    }
}