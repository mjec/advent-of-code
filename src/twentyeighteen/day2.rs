use return_codes;
use std::collections::HashMap;
use std::io::{self, Read};

const CHARS_PER_ID: usize = 26;
const LINE_COUNT: usize = 250;

make_dispatch_and_help!(
    "advent-of-code 2018 2",
    "1" => part1 => part1_help => "" => "Part 1: product of counts of IDs containing letters which occur exactly twice and three times (takes line-delimited input on stdin)",
    "2" => part2 => part2_help => "" => "Part 2: the common letters of two IDs which are the same except in one place (takes line-delimited input on stdin)"
);

fn part1(args: &mut Vec<String>) -> Result<(), i32> {
    require_exactly_arguments!(args, 0, part1_help);

    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        println!("Unable to read from stdin: {:?}", e);
        return Err(e.raw_os_error().unwrap_or(return_codes::BAD_DATA));
    }

    println!(
        "{}",
        match number_of_ids_with_letters_appearing_two_or_three_times(&buffer) {
            (x, y) => x * y,
        }
    );
    Ok(())
}

fn part2(args: &mut Vec<String>) -> Result<(), i32> {
    require_exactly_arguments!(args, 0, part1_help);

    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        println!("Unable to read from stdin: {:?}", e);
        return Err(e.raw_os_error().unwrap_or(return_codes::BAD_DATA));
    }

    println!(
        "{}",
        common_letters_between_ids(&buffer).unwrap_or_else(|| String::from("No matching IDs :/"))
    );
    Ok(())
}

fn number_of_ids_with_letters_appearing_two_or_three_times(stdin: &str) -> (u32, u32) {
    let mut twos: u32 = 0;
    let mut threes: u32 = 0;
    let mut counts: HashMap<char, u32> = HashMap::with_capacity(CHARS_PER_ID);

    for line in stdin.lines() {
        let mut seen_two = false;
        let mut seen_three = false;
        counts.clear();
        for ch in line.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        for (_, val) in counts.iter() {
            match val {
                2 if !seen_two => {
                    twos += 1;
                    seen_two = true;
                }
                3 if !seen_three => {
                    threes += 1;
                    seen_three = true;
                }
                _ => (),
            }
        }
    }
    (twos, threes)
}

fn common_letters_between_ids(stdin: &str) -> Option<String> {
    let mut ids: [[char; CHARS_PER_ID]; LINE_COUNT] = [[0 as char; CHARS_PER_ID]; LINE_COUNT];
    let mut differing_location: Option<usize> = None;

    for (i, line) in stdin.lines().take(LINE_COUNT).enumerate() {
        for (j, ch) in line.chars().take(CHARS_PER_ID).enumerate() {
            ids[i][j] = ch;
        }
    }

    for i in 0..LINE_COUNT {
        for j in (i + 1)..LINE_COUNT {
            for n in 0..CHARS_PER_ID {
                if differing_location.is_some() && ids[i][n] != ids[j][n] {
                    differing_location = None;
                    break;
                } else if ids[i][n] != ids[j][n] {
                    differing_location = Some(n);
                }
            }
            if differing_location.is_some() {
                let mut result: String = String::with_capacity(CHARS_PER_ID - 1);
                for k in 0..CHARS_PER_ID {
                    if k == differing_location.unwrap() {
                        continue;
                    }
                    result.push(ids[i][k]);
                }
                return Some(result);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_ids_with_letters_appearing_two_or_three_times() {
        assert_eq!(
            (4, 3),
            number_of_ids_with_letters_appearing_two_or_three_times(&String::from(
                "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"
            ))
        );
    }

    #[test]
    fn test_common_letters_between_ids() {
        assert_eq!(
            Some(String::from("xpysnnkqrbuhefmcajodplyzw")),
            common_letters_between_ids(&String::from(
                "xrysntkqrduheficajodiglvzw\nxzymntkqrbuhefmcajodiflvzw\nxpysetkyrbuhefmcajodiglvgw\nxtysnykqrbuhefmczjodiglvzw\nxpysntkqyvuhefmjajodiglvzw\nxpysntkqrbuqjfmcajodiglvzu\nxpysntkqrbuhefmvaoodimlvzw\nxplsntkqrbuhefmcajohimlvzw\ncpysntkqrbuhefmcajodiglpmw\nxpysndkqrbuhefmcajzdiglmzw\nxpysntkqrbuhsfmctjoqiglvzw\nxpysztkkrbuhewmcajodiglvzw\nxpysntkguzuhefmcajodiglvzw\nxpysnbkvrbuhyfmcajodiglvzw\nxpgsntkqrbphefmcajodzglvzw\nxpysntkqrbdhefmcajodvgqvzw\nxpmsntkqrbuhedmcajodiglvnw\nwpysntkqmbuheemcajodiglvzw\nxpysntdvrbuhdfmcajodiglvzw\nxpbsetkqrbuhefmcayodiglvzw\nxpasntkqrbuhefmcajydigovzw\nxpylntkqrbuhefmcajudiglnzw\nxpysqtkqrbuoefmcrjodiglvzw\nxlysntkqrbuhefmuejodiglvzw\nxpysntkqrzuhewmcajodaglvzw\nxpysotkqrbuhefmxajowiglvzw\nxpysntkqrbuhefmcajodinlyza\nxpysntkqrbuhemmmajofiglvzw\nxpysntkqcbuhezmcyjodiglvzw\nxpysntyqrbuhefmcajodiglvfn\nxpysntkqrbudefmcarodcglvzw\nxpysntkqrjuhefkcajodiglvzg\nxpysntkqrbuhefmcahooiglvow\nxpysntkqrqfhevmcajodiglvzw\nxpysntlqrbuhefmcajodivlvzu\nxpyjotkqrbuhefmcavodiglvzw\nxpysntqqrbuhebmcgjodiglvzw\nxpjsnikqbbuhefmcajodiglvzw\nxpysntkqrbuhefmcalodeflvzw\nxpysntktrbuhefmcaqoviglvzw\nxpysntkqrbuhefmcrjogiglvzk\nxpysntkqrjuhekmcajodixlvzw\nxpypntyqrbuhefmcajodigzvzw\ncpysnwkqrbuhefmcajodiglvww\nxpysntkqqbuqefmcajodiglvlw\nxpysntkqrbuhvfmcajldiglvzh\nxpysntkqrbuhefmzmjodislvzw\nxpysnnkqrbuhefmcajodjplyzw\nxpysntdqobuhefmcajbdiglvzw\nxpyzntknrbuhefmcajodigtvzw\nxpysntkqbbohefmcajodialvzw\nxpysntkqryuhefmcayoxiglvzw\nxxysntkqrbuhefmcajodiglgzl\njzysntkqrbuhmfmcajodiglvzw\nxpysntocrbuhefmcakodiglvzw\nxpysntkqrbuheomrgjodiglvzw\nxpysntkerbuhefmxajoduglvzw\nxpysntkqobuhefpcajodiglvzd\nxpysntkqrbvhefmcajopislvzw\nxpysntkqrbuhefucqjodiglvqw\nxpysvtkqrbuhdfmcajokiglvzw\nxqysnjkqrbuhemmcajodiglvzw\nxpysnteqxbuhefmcajodiglvdw\nxpysntkqrbuhefzcajodignvzc\nxpysntkqrbuledmcajoyiglvzw\nxpysntkqrbuhofmcajkdigpvzw\nxpysmtkqrbuhefmcajodaglvze\nxlysntiqrbuhefmqajodiglvzw\nxpysntvqrbugefmcajodiglizw\nxpysntkqrbuhefmuwjndiglvzw\nypysntkqrbuhefmbajodislvzw\nxpycntkqrbuhemmcaqodiglvzw\nxpysntkqrbuhefmcajobivdvzw\nxpysnfkprbuhefmcajodiglvkw\nxtysntkwrjuhefmcajodiglvzw\nxpysntkqrbqhefmcaxodiulvzw\nxpysntkqrbuhefmcajsdigkvkw\nxpysnekqrbugefmcajwdiglvzw\nxpysntkqjbuhefmcajoduglbzw\nxpysntkqrbohlfmcajodiggvzw\nxpysntkqrbfhefmcajodiglhnw\nxpmsntkirbusefmcajodiglvzw\nxpnsntkqrbehefmcajodigrvzw\nxpytntgqrbuhefmcajodigllzw\nxpysntkqrbuhefmcejodiglvvm\nxpyvntkqrbuhefmmajodiglmzw\nrpysntkirbuhefmcajodiglyzw\nxpysntkqrbuhefmcajoznglvzz\nxpysntkqrbmhefmcafodigwvzw\nxpysntkqrbuhetmcarodiglvzx\nxpysntnqrbuhefmcajogiglfzw\nxpysutkqjbuheomcajodiglvzw\nxpysnqkprbuhefkcajodiglvzw\nxpysntuqrbuhemmcajodyglvzw\nspysntkqrbupefmcajodigpvzw\nqpygntkqrbuhefmcajodqglvzw\nxpysnteqrbuhefmcatodiylvzw\nxpysntkqrbusefsckjodiglvzw\nxpysntkqrbubeflcajodiglvcw\nxpysntwqrbuhefmcajodigekzw\nxpysntkqrbuhefmcnjodiguvtw\nxpysntkqrbqhefmkasodiglvzw\nxmysntkqrbuhefmkcjodiglvzw\nxpysntkqrquhefmcajodiouvzw\nxpysnnkqrbuhefmcajodiplyzw\nxpysntkorbuhefaqajodiglvzw\nnpysntkqobuhefmcajodiglvfw\nxlysntkqrbuhefmcazbdiglvzw\nxpysftkqrbihefmcajodiglezw\nxpysolqqrbuhefmcajodiglvzw\nxpysntkqrwuheixcajodiglvzw\nxoysntkqibuhefmcqjodiglvzw\nxpysntkqrbupefmcajodtulvzw\nxpyentkqrbuhvfmcajudiglvzw\nxpysntksrbuhefmgajodqglvzw\nxpysntkqrbuhcfmcvjodigldzw\ngpyrntdqrbuhefmcajodiglvzw\nvpysntvqrbuhefmcajndiglvzw\nxpvsntkqrbuhefvcajhdiglvzw\nxpysntkqrbubefmcajowiglvzl\nxpysndkqibuhefmcajodiblvzw\nxpysntkqrbuhefmldjodjglvzw\nxpysntzqrbuhefmcyjomiglvzw\nxpysntkqrbuhefmedjodiwlvzw\nxpysntklrbuhefmcyjodiglvjw\nxpypnlkqrbehefmcajodiglvzw\nxpysntkqrbuhefycacodiglvzz\nypysntkqrbuhefmcajadiglvew\nxpysntkqobuhefmcajadiglhzw\nxpysbtxlrbuhefmcajodiglvzw\nxpysntkqrbuhefdcajoviglvww\nxpysntkqrbuhefmcaaodiblvzc\nwpysntkqrbuhefmcajddiglvbw\nwpysntmqrbuhefmcajodiglvza\nepysntkqrbuhemmcajoniglvzw\nvpysntkqrbuhejmcajodiglvzo\nxpysntkqrbuhebmmajodiglazw\nlpysntkqrguhtfmcajodiglvzw\nxpksndkqpbuhefmcajodiglvzw\nxpydnukqrbuhefmcajmdiglvzw\nxpysnmkqrbuhefioajodiglvzw\nxpysntkqruuhefmcajodyglvzx\nxpysntkqrmuhefmcmjidiglvzw\nxpysntksrbupefmcajodiplvzw\nxpykntkqrbuhefmmajodiglxzw\nxpssntkqrbuhefmzajodiglvgw\nxpysntkqrbuhefmcrjodigovzd\nxpysntkqrbuhefmpajodirlvqw\nxpysnskqrruhefmtajodiglvzw\nxpysnzkqrbuhezmcajodiglvzj\nxpysntkcrburefmckjodiglvzw\nxpysntkqrbuhefecpoodiglvzw\nxpysnjkqrbuhexmcajodigrvzw\nxpysztkqhbuhefmcanodiglvzw\nxpysntkqrbuhefmcajozyelvzw\nxpyuntkmrbuhefmcajodigcvzw\nxpysnykmrbuhefmaajodiglvzw\njpysntkqrbuhefmcajodigumzw\nxtysntkqhbuhefmcajodiglvzz\nxpysntkqrbqhefmcxjouiglvzw\nxpysntkqreujefmmajodiglvzw\nxnysntzqrbuhefacajodiglvzw\nxpysntkqriuhefmcajkdiqlvzw\nxposntkqrbuheffcajodihlvzw\nxpysntkqpquhefmcajodrglvzw\nxpysjtkqrbudefmcajouiglvzw\nxpysnxkqrbulefmcacodiglvzw\nxpygntkqrbuhefmfajzdiglvzw\nxpysntkqrbuhefmuayodiglyzw\nxpysnbksrbuhefmcajediglvzw\nxkyzntkqrbuhefmcajodiglvzz\nxpysntkqrbehlxmcajodiglvzw\nxpysntkqryuhefmcaxodiklvzw\nxpysntdqrbuhefmcjjodiglvzt\nxpysntkqrbuhefmcauodigqvzy\nxpysftkqrbuhefmcajodrgvvzw\nxpysntkqrbuhefmczjodiglivw\nxpysatkorbuhefmcyjodiglvzw\nxhysntkqrbthefmcajodxglvzw\nxpysnpkqrbuhefmcajoqyglvzw\nxpyyntkqrbuhefmcjjodxglvzw\nxpysntkqrozhefhcajodiglvzw\nxpymftkqrbuhefmcajodiglvzo\nxpysntkqrbuhvfmcajodiyllzw\nxpysatsqrbuhefmcajodiglvzo\ncpysntkqqbuhefmcajodlglvzw\nxpysntkyrblhefmcajodiglvzz\nxpysntkfrbuhefbcajodiglbzw\nupysotkqrbuhpfmcajodiglvzw\nxpysdokqrbuheflcajodiglvzw\nxrysntkqrbuhefpcanodiglvzw\nxdysntkqrbuhefpcajodiglmzw\nxpyynteqrbjhefmcajodiglvzw\nxpysntkqrbuhefkcajodielvhw\nxplsktkqrbuhefmcajodtglvzw\nxpysrtkqrbuhefmcdjodiglvzx\nupysntkmrbuhefmcajodiglvzt\nxnysnpkqrbusefmcajodiglvzw\nxtysntnqrbuhexmcajodiglvzw\nxpysngkmrbfhefmcajodiglvzw\nxpysnhkhrbuhefmcajodiplvzw\nxpysntvqrbuhefmcaoodsglvzw\nxpyzntkqrbuhefmcajlviglvzw\nxpysndkqrbuievmcajodiglvzw\nxpysntkqrsuhcfzcajodiglvzw\nxpysntkqlbuhefmcajjdlglvzw\nxpysntkqrbuhifmcajoliylvzw\nxpysntkqxbphefmcajodialvzw\nbnyswtkqrbuhefmcajodiglvzw\nupysatkqrbuhefmcajodvglvzw\nxpysntkqqbuhefmcajodxglzzw\nxpysntkqryuhefmcafodinlvzw\nxpzsntkqrcuhefmcajokiglvzw\nxpynntkqrbuheimccjodiglvzw\nxpysnfkqduuhefmcajodiglvzw\nxpywntkqrbuhefmcajodigllzz\nxpysftkqrbahefmcajsdiglvzw\nxpysntkkrbutefmcljodiglvzw\nxfysntkqrbbhkfmcajodiglvzw\nxpysgtksrbuhefkcajodiglvzw\nxpysntyqrbuhefmcajodijlvzg\nxpysnpkqobuhefmcljodiglvzw\nxvysntkqrbuhefmcsjodiclvzw\nxpysntkqrtuhwfmcajodillvzw\nxpysntkprbuhejmcajouiglvzw\napysntkqrbuhefmcaboviglvzw\nxpysqykqrbuhefmcajodirlvzw\nxpysntkqrluhefmcajowiglvzf\ndpysnokqrbuhefmcajoaiglvzw\nxppsntkqmbuheumcajodiglvzw\nxpysntkqrbuhlymcaoodiglvzw\ndpysntkqrbuhmfmcajodiglvzt\nxpysltbqrbuhefmcajoliglvzw\nxpysntpqrbuhefmcnjoniglvzw\nxpysntpqrbuhektcajodiglvzw\nxpysntkhrbshefmqajodiglvzw\nzpysntuqrbuhefmcajmdiglvzw\nxpysxtdqrbuhefmcajoyiglvzw\nxpysntkubbumefmcajodiglvzw\nxpysntkqzouhefmcajsdiglvzw\nxpysntkqrbuhefmcojoziglvyw\njpysntkqrmuhefmcajodidlvzw\nxpmsttkqrhuhefmcajodiglvzw\nxpysntkqlbuhefmcajgdiflvzw\nxpysntxkrbuhefmcauodiglvzw\nxpysotkqubuhefscajodiglvzw\nxjysntkqrbvheemcajodiglvzw\nxpykntkqrbuhefmcpjodiglvow\nxplsntkqrbuvefmcajediglvzw\nupysntwqrbuhefmfajodiglvzw"
            ),)
        );
    }

}
