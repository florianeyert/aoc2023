use std::{fs, collections::HashMap};
use regex::Regex;

pub fn aoc23_1_1() -> i64 {
    fs::read_to_string("input/input_day1").unwrap().split("\n").map(|l: &str|  {
        let digits: Vec<char> = l.chars().filter(|e: &char| e.is_digit(10) ).collect();
        format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse::<i64>().unwrap()
    }).sum::<i64>()
}

pub fn aoc23_1_2() -> i64 {
    let numstrings: HashMap<&str, i64> = HashMap::from([("one", 1), ("two", 2), ("three", 3),
        ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]); 
    fs::read_to_string("input/input_day1").unwrap().split("\n").map(|l: &str|  {
        let mut all: Vec<(usize, i64)> = vec![];
        ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].into_iter().for_each(|n: &str| 
            l.match_indices(n).map(|mi: (usize, &str)| mi.0).for_each( |f: usize| 
                all.push((f, numstrings.get(n).unwrap().clone()))
            ));
        (1..10).for_each(|n: i64| 
            l.match_indices(&n.to_string()).for_each(|e: (usize, &str)| all.push((e.0, n)))
        );
        all.sort_by(|a: &(usize, i64), b: &(usize, i64)| a.0.cmp(&b.0));
        format!("{}{}", all.first().unwrap().1, all.last().unwrap().1).parse::<i64>().unwrap()
    }).sum::<i64>()
}

pub fn aoc23_2_1() -> i64 {
    let mut total: i64 = 0;
    fs::read_to_string("input/input_day2").unwrap().split("\n").for_each(|l: &str|  {
        let mut valid: bool = true;
        l.split(":").last().unwrap().split(";").for_each(|e| {
            Regex::new(r"([0-9]+) red").unwrap().captures_iter(e).map(|c| c.extract().1.map(|a| a.parse::<i64>().unwrap())).for_each(|[r]|  {
                if r > 12 { valid = false; }});
            Regex::new(r"([0-9]+) green").unwrap().captures_iter(e).map(|c| c.extract().1.map(|a| a.parse::<i64>().unwrap())).for_each(|[g]| {
                if g > 13 { valid = false; }});
            Regex::new(r"([0-9]+) blue").unwrap().captures_iter(e).map(|c| c.extract().1.map(|a| a.parse::<i64>().unwrap())).for_each(|[b]| {
                if b > 14 { valid = false; }});
        });
        if valid { total += Regex::new(r"Game ([0-9]+):").unwrap().captures(l).unwrap()[1].to_string().parse::<i64>().unwrap(); }
    });
    total
}

pub fn aoc23_2_2() -> i64 {
    fs::read_to_string("input/input_day2").unwrap().split("\n").map(|l: &str|  {
        let [mut minred, mut mingreen, mut minblue] = [0, 0, 0];
        l.split(":").last().unwrap().split(";").for_each(|e| {
            Regex::new(r"([0-9]+) red").unwrap().captures_iter(e).map(|c| c.extract().1.map(|a| a.parse::<i64>().unwrap())).for_each(|[r]|  {
                if r > minred { minred = r }});
            Regex::new(r"([0-9]+) green").unwrap().captures_iter(e).map(|c| c.extract().1.map(|a| a.parse::<i64>().unwrap())).for_each(|[g]| {
                if g > mingreen { mingreen = g }});
            Regex::new(r"([0-9]+) blue").unwrap().captures_iter(e).map(|c| c.extract().1.map(|a| a.parse::<i64>().unwrap())).for_each(|[b]| {
                if b > minblue { minblue = b }});
        });
        minred * mingreen * minblue
    }).sum()
}


pub fn aoc23_3_1() -> i64 {
    let mut numbers: Vec<(i64, i64, i64, i64)>  = vec![];
    let mut symbols: Vec<(String, i64, i64)>  = vec![];
    fs::read_to_string("input/input_day3").unwrap().split("\n").enumerate().for_each(|(i, l)|  {
        Regex::new(r"([0-9]+)").unwrap().captures_iter(l).for_each(|c| {
            let found = c.iter().find(|cc| cc.is_some()).unwrap().unwrap();
            numbers.push((found.as_str().parse::<i64>().unwrap(), i as i64, found.start() as i64, found.end() as i64))
        });
        Regex::new(r"([^0-9, ^.])").unwrap().captures_iter(l).for_each(|c| {
            let found = c.iter().find(|cc| cc.is_some()).unwrap().unwrap();
            symbols.push((found.as_str().to_owned(), i as i64, found.start() as i64))
        });
    });
    numbers.into_iter().map(|n| {
        if ((n.2 - 1)..(n.3 + 1)).map(|x| {
            symbols.clone().into_iter().map(|s| {
                s.1 >= (n.1 - 1) && s.1 <= (n.1 + 1) && x == s.2
            }).reduce(|a, b| a || b).unwrap()
        }).reduce(|a, b| a || b).unwrap() {n.0} else {0}
    }).sum()
}


pub fn aoc23_3_2() -> i64 {
    let mut numbers: Vec<(i64, i64, i64, i64)>  = vec![];
    let mut symbols: Vec<(String, i64, i64, Vec<i64>)>  = vec![];
    fs::read_to_string("input/input_day3").unwrap().split("\n").enumerate().for_each(|(i, l)|  {
        Regex::new(r"([0-9]+)").unwrap().captures_iter(l).for_each(|c| {
            let found = c.iter().find(|cc| cc.is_some()).unwrap().unwrap();
            numbers.push((found.as_str().parse::<i64>().unwrap(), i as i64, found.start() as i64, found.end() as i64))
        });
        Regex::new(r"([^0-9, ^.])").unwrap().captures_iter(l).for_each(|c| {
            let found = c.iter().find(|cc| cc.is_some()).unwrap().unwrap();
            symbols.push((found.as_str().to_owned(), i as i64, found.start() as i64, vec![]))
        });
    });
    numbers.into_iter().for_each(|n| {
        ((n.2 - 1)..(n.3 + 1)).for_each(|x| {
            (0..symbols.len()).for_each(|si| {
                if symbols[si].1 >= (n.1 - 1) && symbols[si].1 <= (n.1 + 1) && x == symbols[si].2 {
                    symbols[si].3.push(n.0);
                }
            })
        });
    });
    symbols.into_iter().map(|s| {
        if s.3.len() == 2 {s.3[0] * s.3[1]} else {0}
    }).sum()
}


pub fn aoc23_4_1() -> i64 {
    let file = fs::read_to_string("input/input_day4").unwrap(); 
    let lines = file.split("\n");
    let tocheck: Vec<usize> = lines.clone().enumerate().map(|(i, _)| i).collect();
    tocheck.clone().into_iter().map(|i|  {
        let line = lines.clone().into_iter().nth(i).unwrap();
        let winning = line.split(":").last().unwrap().split("|").next().unwrap();
        let have = line.split(":").last().unwrap().split("|").last().unwrap();
        let winningnumbers: Vec<i64> = Regex::new(r"([0-9]+)").unwrap().captures_iter(winning).map(|c| {
            let found = c.iter().find(|cc| cc.is_some()).unwrap().unwrap();
            found.as_str().parse::<i64>().unwrap()
        }).collect();
        let havenumbers: Vec<i64> = Regex::new(r"([0-9]+)").unwrap().captures_iter(have).map(|c| {
            let found = c.iter().find(|cc| cc.is_some()).unwrap().unwrap();
            found.as_str().parse::<i64>().unwrap()
        }).collect();
        let all: i64 = winningnumbers.into_iter().map(|wn| {
            if havenumbers.clone().into_iter().map(|hn| {
                if wn == hn {1} else {0}
            }).sum::<i64>() > 0 {1} else {0}
        }).sum::<i64>();
        if all == 0 {0} else {(2 as i64).pow((all - 1) as u32)}
    }).sum::<i64>()
}

pub fn aoc23_4_2() -> i64 {
    let file = fs::read_to_string("input/input_day4").unwrap(); 
    let lines = file.split("\n");
    let mut tocheck: Vec<usize> = lines.clone().enumerate().map(|(i, _)| i).collect();
    let mut total = tocheck.clone().len();
    let winmap: Vec<(usize, usize)> = tocheck.clone().into_iter().map(|i|  {
        let line = lines.clone().into_iter().nth(i).unwrap();
        let winning = line.split(":").last().unwrap().split("|").next().unwrap();
        let have = line.split(":").last().unwrap().split("|").last().unwrap();
        let winningnumbers: Vec<i64> = Regex::new(r"([0-9]+)").unwrap().captures_iter(winning).map(|c| {
            let found = c.iter().find(|cc| cc.is_some()).unwrap().unwrap();
            found.as_str().parse::<i64>().unwrap()
        }).collect();
        let havenumbers: Vec<i64> = Regex::new(r"([0-9]+)").unwrap().captures_iter(have).map(|c| {
            let found = c.iter().find(|cc| cc.is_some()).unwrap().unwrap();
            found.as_str().parse::<i64>().unwrap()
        }).collect();
        let all = winningnumbers.into_iter().map(|wn| {
            havenumbers.clone().into_iter().map(|hn| {
                if wn == hn {1} else {0}
            }).sum::<i64>()
        }).sum::<i64>();
        (i, all as usize)
    }).collect::<Vec<(usize, usize)>>();
    while tocheck.len() > 0 {
        let mut newtocheck = vec![];
        tocheck.clone().into_iter().for_each(|tc| {
            let won = winmap.clone().into_iter().find(|wp| wp.0 == tc);
            if won.is_some() {
                (0..won.unwrap().1).for_each(|new| {
                    newtocheck.push(tc + 1 + (new as usize));
                });
            }
        });
        total += newtocheck.len();
        tocheck = newtocheck;
    }
    total as i64
}

pub fn aoc23_5_1() -> i64 {
    let mut seeds: Vec<i64> = vec![];
    let mut maps: Vec<(String, String, Vec<(i64, i64, i64)>)> = vec![];
    fs::read_to_string("input/input_day5").unwrap().split("\n\n").enumerate().for_each(|(i, block)|  {
        if i == 0 {
            Regex::new(r"([0-9]+)").unwrap().captures_iter(block).for_each(|c| {
                let found = c.iter().find(|cc| cc.is_some()).unwrap().unwrap();
                seeds.push(found.as_str().parse::<i64>().unwrap());
            });
        } else {
            let mut source: String = "".to_string();
            let mut destination: String = "".to_string();
            let mut numbers: Vec<(i64, i64, i64)> = vec![];
            block.split("\n").enumerate().for_each(|(i, line)| {
                if i == 0 {
                    Regex::new(r"([a-z]+)-to-([a-z]+) map").unwrap().captures_iter(block).for_each(|c| {
                        let sourceparsed = c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap();
                        let destinationparsed = c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap();
                        source = sourceparsed.as_str().to_owned();
                        destination = destinationparsed.as_str().to_owned();
                     });
                } else {
                    Regex::new(r"([0-9]+) ([0-9]+) ([0-9]+)").unwrap().captures_iter(line).for_each(|c| {
                        let destinationrange = c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap();
                        let sourcerange = c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap();
                        let rangelen = c.iter().filter(|cc| cc.is_some()).nth(3).unwrap().unwrap();
                        numbers.push((destinationrange.as_str().parse::<i64>().unwrap(),
                        sourcerange.as_str().parse::<i64>().unwrap(), rangelen.as_str().parse::<i64>().unwrap()));
                    });
                }
            });
            maps.push((source, destination, numbers))
        }
    });
    seeds.into_iter().map(|seed| {
        let mut currentsource = "seed".to_string();
        let mut currentnumber = seed.clone();
        while currentsource != "location".to_string() {
            let currentmap = maps.clone().into_iter().find(|map| map.0 == currentsource).unwrap();
            currentsource = currentmap.1;
            let currentnumbertmp: i64 = currentmap.2.into_iter().map(|range| {
                if currentnumber >= range.1 && currentnumber < range.1 + range.2 {
                    range.0 + (currentnumber - range.1)
                } else {0}
            }).sum();
            currentnumber = if currentnumbertmp > 0 {currentnumbertmp} else {currentnumber}
        }
        currentnumber
    }).min().unwrap()
}


pub fn aoc23_5_2() -> i64 {
    let mut seedranges: Vec<(i64, i64)> = vec![];
    let mut maps: Vec<(String, String, Vec<(i64, i64, i64)>)> = vec![];
    fs::read_to_string("input/input_day5").unwrap().split("\n\n").enumerate().for_each(|(i, block)|  {
        if i == 0 {
            Regex::new(r"([0-9]+) ([0-9]+)").unwrap().captures_iter(block).for_each(|c| {
                let rangestart = c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap().as_str().parse::<i64>().unwrap();
                let rangelen = c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap().as_str().parse::<i64>().unwrap();
                seedranges.push((rangestart, rangelen));
            });
        } else {
            let mut source: String = "".to_string();
            let mut destination: String = "".to_string();
            let mut numbers: Vec<(i64, i64, i64)> = vec![];
            block.split("\n").enumerate().for_each(|(i, line)| {
                if i == 0 {
                    Regex::new(r"([a-z]+)-to-([a-z]+) map").unwrap().captures_iter(block).for_each(|c| {
                        let sourceparsed = c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap();
                        let destinationparsed = c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap();
                        source = sourceparsed.as_str().to_owned();
                        destination = destinationparsed.as_str().to_owned();
                     });
                } else {
                    Regex::new(r"([0-9]+) ([0-9]+) ([0-9]+)").unwrap().captures_iter(line).for_each(|c| {
                        let destinationrange = c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap();
                        let sourcerange = c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap();
                        let rangelen = c.iter().filter(|cc| cc.is_some()).nth(3).unwrap().unwrap();
                        numbers.push((destinationrange.as_str().parse::<i64>().unwrap(),
                        sourcerange.as_str().parse::<i64>().unwrap(), rangelen.as_str().parse::<i64>().unwrap()));
                    });
                }
            });
            maps.push((source, destination, numbers))
        }
    });
    let result = seedranges.into_iter().map(|seedrange| {
        let mut currentsource = "seed".to_string();
        let mut currentranges = vec![seedrange];
        while currentsource != "location".to_string() {
            let currentmap = maps.clone().into_iter().find(|map| map.0 == currentsource).unwrap();
            currentsource = currentmap.1;
            let mut matchedranges: Vec<(i64, i64)> = vec![];
            currentmap.2.into_iter().for_each(|maprange| {
                let mut unmatchedranges: Vec<(i64, i64)> = vec![];
                currentranges.clone().into_iter().for_each(|currentrange| {
                    if maprange.1.max(currentrange.0) < (maprange.1 + maprange.2).min(currentrange.0 + currentrange.1) {
                        let matchedrangestart = maprange.1.max(currentrange.0) - (maprange.1 - maprange.0);
                        let matchedrangelen = (maprange.1 + maprange.2).min(currentrange.0 + currentrange.1) - maprange.1.max(currentrange.0);
                        matchedranges.push((matchedrangestart, matchedrangelen));
                        if currentrange.0 < maprange.1 {
                            unmatchedranges.push((currentrange.0, maprange.1 - currentrange.0));
                        }
                        if maprange.1 + maprange.2 < currentrange.0 + currentrange.1 {
                            unmatchedranges.push((maprange.1 + maprange.2, currentrange.0 + currentrange.1 - (maprange.1 + maprange.2)));
                        }
                    } else {
                        unmatchedranges.push(currentrange);
                    }
                });
                currentranges = unmatchedranges.clone();
            });
            matchedranges.into_iter().for_each(|ncr| currentranges.push(ncr))
        }
        currentranges
    });
    result.clone().flat_map(|r| r.into_iter().map(|rr| rr.0)).min().unwrap()
}