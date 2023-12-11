use std::{fs, collections::HashMap};
use regex::Regex;

pub fn d1p1() -> i64 {
    fs::read_to_string("input/day1").unwrap().split("\n").map(|l: &str|  {
        let digits: Vec<char> = l.chars().filter(|e: &char| e.is_digit(10) ).collect();
        format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse::<i64>().unwrap()
    }).sum::<i64>()
}

pub fn d1p2() -> i64 {
    let numstrings: Vec<(&str, i64)> = vec![("one", 1), ("two", 2), ("three", 3),
        ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]; 
    fs::read_to_string("input/day1").unwrap().split("\n").map(|l: &str|  {
        let mut all: Vec<(usize, i64)> = vec![];
        ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].into_iter().for_each(|n: &str| 
            l.match_indices(n).map(|mi: (usize, &str)| mi.0).for_each( |f: usize| 
                all.push((f, numstrings.clone().into_iter().find(|ns| ns.0 == n).unwrap().clone().1))
            ));
        (1..10).for_each(|n: i64| 
            l.match_indices(&n.to_string()).for_each(|e: (usize, &str)| all.push((e.0, n)))
        );
        all.sort_by(|a: &(usize, i64), b: &(usize, i64)| a.0.cmp(&b.0));
        format!("{}{}", all.first().unwrap().1, all.last().unwrap().1).parse::<i64>().unwrap()
    }).sum::<i64>()
}

pub fn d2p1() -> i64 {
    fs::read_to_string("input/day2").unwrap().split("\n").map(|l: &str|  {
        let mut valid: bool = true;
        let line = l.split(":").last().unwrap();
        if Regex::new(r"([0-9]+) red").unwrap().captures_iter(line).any(|r|  {
            r.iter().nth(1).unwrap().unwrap().as_str().parse::<i64>().unwrap() > 12}) {valid = false};
        if Regex::new(r"([0-9]+) green").unwrap().captures_iter(line).any(|r|  {
            r.iter().nth(1).unwrap().unwrap().as_str().parse::<i64>().unwrap() > 13}) {valid = false};
        if Regex::new(r"([0-9]+) blue").unwrap().captures_iter(line).any(|r|  {
            r.iter().nth(1).unwrap().unwrap().as_str().parse::<i64>().unwrap() > 14}) {valid = false};
        if valid {Regex::new(r"Game ([0-9]+):").unwrap().captures(l).unwrap()[1].to_string().parse::<i64>().unwrap()} else {0}
    }).sum()
}

pub fn d2p2() -> i64 {
    fs::read_to_string("input/day2").unwrap().split("\n").map(|l: &str|  {
        let [mut minred, mut mingreen, mut minblue] = [0, 0, 0];
        let line = l.split(":").last().unwrap();
        Regex::new(r"([0-9]+) red").unwrap().captures_iter(line).map(|c| c.extract().1.map(|a| a.parse::<i64>().unwrap())).for_each(|[r]|  {
            if r > minred { minred = r }});
        Regex::new(r"([0-9]+) green").unwrap().captures_iter(line).map(|c| c.extract().1.map(|a| a.parse::<i64>().unwrap())).for_each(|[g]| {
            if g > mingreen { mingreen = g }});
        Regex::new(r"([0-9]+) blue").unwrap().captures_iter(line).map(|c| c.extract().1.map(|a| a.parse::<i64>().unwrap())).for_each(|[b]| {
            if b > minblue { minblue = b }});
        minred * mingreen * minblue
    }).sum()
}

pub fn d3p1() -> i64 {
    let mut numbers: Vec<(i64, i64, i64, i64)>  = vec![];
    let mut symbols: Vec<(i64, i64)>  = vec![];
    fs::read_to_string("input/day3").unwrap().split("\n").enumerate().for_each(|(i, l)|  {
        Regex::new(r"([0-9]+)").unwrap().captures_iter(l).for_each(|c| {
            let found = c.iter().find(|cc| cc.is_some()).unwrap().unwrap();
            numbers.push((found.as_str().parse::<i64>().unwrap(), i as i64, found.start() as i64, found.end() as i64))
        });
        Regex::new(r"([^0-9, ^.])").unwrap().captures_iter(l).for_each(|c| {
            let found = c.iter().find(|cc| cc.is_some()).unwrap().unwrap();
            symbols.push((i as i64, found.start() as i64))
        });
    });
    numbers.into_iter().map(|n| {
        if symbols.clone().into_iter().map(|s| {
            s.0 >= (n.1 - 1) && s.0 <= (n.1 + 1) && s.1 >= n.2 - 1 && s.1 <= n.3
        }).reduce(|a, b| a || b).unwrap() {n.0} else {0}
    }).sum()
}

pub fn d3p2() -> i64 {
    let mut numbers: Vec<(i64, i64, i64, i64)>  = vec![];
    let mut symbols: Vec<(String, i64, i64, Vec<i64>)>  = vec![];
    fs::read_to_string("input/day3").unwrap().split("\n").enumerate().for_each(|(i, l)|  {
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

pub fn d4p1() -> i64 {
    let file = fs::read_to_string("input/day4").unwrap(); 
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
            if havenumbers.clone().into_iter().find(|hn| wn == *hn).is_some() {1} else {0}
        }).sum::<i64>();
        if all == 0 {0} else {(2 as i64).pow((all - 1) as u32)}
    }).sum::<i64>()
}

pub fn d4p2() -> i64 {
    let file = fs::read_to_string("input/day4").unwrap(); 
    let lines = file.split("\n");
    let tocheck: Vec<usize> = lines.clone().enumerate().map(|(i, _)| i).collect();
    let mut winmap: Vec<(usize, usize, usize)> = tocheck.clone().into_iter().map(|i|  {
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
            if havenumbers.clone().into_iter().find(|hn| wn == *hn).is_some() {1} else {0}
        }).sum::<i64>();
        (i, all as usize, 1)
    }).collect::<Vec<(usize, usize, usize)>>();
    (0..winmap.len()).into_iter().map(|gamenum| {
        (0..winmap[gamenum].1).for_each(|offset| {
            winmap[gamenum + 1 + offset].2 += winmap[gamenum].2;
        });
        winmap[gamenum].2 as i64
    }).sum()
}

pub fn d5p1() -> i64 {
    let mut seeds: Vec<i64> = vec![];
    let mut maps: Vec<(String, String, Vec<(i64, i64, i64)>)> = vec![];
    fs::read_to_string("input/day5").unwrap().split("\n\n").enumerate().for_each(|(i, block)|  {
        match i {
            0 => {
                Regex::new(r"([0-9]+)").unwrap().captures_iter(block).for_each(|c| {
                    let found = c.iter().find(|cc| cc.is_some()).unwrap().unwrap();
                    seeds.push(found.as_str().parse::<i64>().unwrap());
                });
            },
            _ => {
                let mut source: String = "".to_string();
                let mut destination: String = "".to_string();
                let mut numbers: Vec<(i64, i64, i64)> = vec![];
                block.split("\n").enumerate().for_each(|(i, line)| {
                    match i {
                        0 => {
                            Regex::new(r"([a-z]+)-to-([a-z]+) map").unwrap().captures_iter(block).for_each(|c| {
                                source = c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap().as_str().to_owned();
                                destination = c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap().as_str().to_owned();
                            });
                        },
                        _ => {
                            Regex::new(r"([0-9]+) ([0-9]+) ([0-9]+)").unwrap().captures_iter(line).for_each(|c| {
                                numbers.push((c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap().as_str().parse::<i64>().unwrap(),
                                c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap().as_str().parse::<i64>().unwrap(),
                                c.iter().filter(|cc| cc.is_some()).nth(3).unwrap().unwrap().as_str().parse::<i64>().unwrap()));
                            });
                        }
                    }
                });
                maps.push((source, destination, numbers))
            }
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
                    range.0 + (currentnumber - range.1)} else {0}
            }).sum();
            currentnumber = if currentnumbertmp > 0 {currentnumbertmp} else {currentnumber}
        }
        currentnumber
    }).min().unwrap()
}

pub fn d5p2() -> i64 {
    let mut seedranges: Vec<(i64, i64)> = vec![];
    let mut maps: Vec<(String, String, Vec<(i64, i64, i64)>)> = vec![];
    fs::read_to_string("input/day5").unwrap().split("\n\n").enumerate().for_each(|(i, block)|  {
        match i {
            0 => {
                Regex::new(r"([0-9]+) ([0-9]+)").unwrap().captures_iter(block).for_each(|c| {
                    let rangestart = c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap().as_str().parse::<i64>().unwrap();
                    let rangelen = c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap().as_str().parse::<i64>().unwrap();
                    seedranges.push((rangestart, rangelen));
                });
            },
            _ => {
                let mut source: String = "".to_string();
                let mut destination: String = "".to_string();
                let mut numbers: Vec<(i64, i64, i64)> = vec![];
                block.split("\n").enumerate().for_each(|(i, line)| {
                    match i {
                        0 => {
                            Regex::new(r"([a-z]+)-to-([a-z]+) map").unwrap().captures_iter(block).for_each(|c| {
                                source = c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap().as_str().to_owned();
                                destination = c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap().as_str().to_owned();
                            });
                        },
                        _ => {
                            Regex::new(r"([0-9]+) ([0-9]+) ([0-9]+)").unwrap().captures_iter(line).for_each(|c| {
                                numbers.push((c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap().as_str().parse::<i64>().unwrap(),
                                c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap().as_str().parse::<i64>().unwrap(),
                                c.iter().filter(|cc| cc.is_some()).nth(3).unwrap().unwrap().as_str().parse::<i64>().unwrap()));
                            });
                        }
                    }
                });
                maps.push((source, destination, numbers))
            }
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

pub fn d6p1() -> i64 {
    let lines = fs::read_to_string("input/day6").unwrap(); 
    let times: Vec<i64> = Regex::new(r"([0-9]+)").unwrap().captures_iter(lines.split("\n").nth(0).unwrap()).map(|c| {
        c.iter().nth(1).unwrap().unwrap().as_str().parse::<i64>().unwrap()
    }).collect();
    let distances: Vec<i64> = Regex::new(r"([0-9]+)").unwrap().captures_iter(lines.split("\n").nth(1).unwrap()).map(|c| {
        c.iter().nth(1).unwrap().unwrap().as_str().parse::<i64>().unwrap()
    }).collect();
    times.into_iter().enumerate().map(|(i, t)| {
        (0..t+1).into_iter().map(|wait| {
            if wait * (t - wait) > distances[i] {1} else {0}
        }).sum::<i64>()
    }).reduce(|a, b| a * b).unwrap()
}

pub fn d6p2() -> i64 {
    let lines = fs::read_to_string("input/day6").unwrap(); 
    let time: i64 = Regex::new(r"([0-9]+)").unwrap().captures_iter(lines.split("\n").nth(0).unwrap()).map(|c| {
        c.iter().nth(1).unwrap().unwrap().as_str().to_owned()
    }).reduce(|a, b| a + b.as_str()).unwrap().parse::<i64>().unwrap();
    let distance: i64 = Regex::new(r"([0-9]+)").unwrap().captures_iter(lines.split("\n").nth(1).unwrap()).map(|c| {
        c.iter().nth(1).unwrap().unwrap().as_str().to_owned()
    }).reduce(|a, b| a + b.as_str()).unwrap().parse::<i64>().unwrap();
    let wait: f64 = ((time as f64)+ ((time.pow(2) - 4 * distance) as f64).sqrt()) / 2.0;
    time - (2 * wait.min((time as f64) - wait).ceil() as i64) + 1
}

pub fn d7p1() -> i64 {
    let file: String = fs::read_to_string("input/day7").unwrap();
    let hands = file.split("\n").map(|l|
        (l.split(" ").nth(0).unwrap().chars().collect::<Vec<char>>(), l.split(" ").nth(1).unwrap().parse::<i64>().unwrap())); 
    let card_values: HashMap<char, i64> = HashMap::from([('A', 14), ('K', 13), ('Q', 12), ('J', 11), ('T', 10),
        ('9', 9), ('8', 8), ('7', 7), ('6', 6), ('5', 5), ('4', 4), ('3', 3), ('2', 2), ('1', 1),]);
    let mut res: Vec<(i64, i64)> = hands.map(|h| 
        ({
            let mut all: HashMap<char, usize> = HashMap::new();
            h.0.iter().for_each(|card| {
                all.insert(*card, {
                    match all.get(card) { Some(c) => c + 1, None => 1 }
                });
            });
            let mut vals: Vec<usize> = all.into_iter().map(|e| e.1).collect();
            vals.sort_by(|a, b| b.cmp(a));
            match (vals.iter().nth(0).unwrap(), vals.iter().nth(1).unwrap_or(&0)) {
                (5, _) => 7, (4, _) => 6, (3, 2) => 5, (3, _) => 4, (2, 2) => 3, (2, _) => 2, _ =>  1
            }
        } * (10 as i64).pow(10)
        + (0..=4).into_iter().map(|i| {
            card_values.get(h.0.iter().nth(i).unwrap()).unwrap().to_owned() * (10 as i64).pow(8 - 2*i as u32)
        }).sum::<i64>(),
        h.1)
    ).collect::<Vec<(i64, i64)>>();
    res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    res.into_iter().enumerate().map(|(i, e)| ((i + 1) as i64) * e.1).sum()
}

pub fn d7p2() -> i64 {
    let file: String = fs::read_to_string("input/day7").unwrap();
    let hands = file.split("\n").map(|l|
        (l.split(" ").nth(0).unwrap().chars().collect::<Vec<char>>(), l.split(" ").nth(1).unwrap().parse::<i64>().unwrap())); 
    let card_values: HashMap<char, i64> = HashMap::from([('A', 14), ('K', 13), ('Q', 12), ('J', 11), ('T', 10),
        ('9', 9), ('8', 8), ('7', 7), ('6', 6), ('5', 5), ('4', 4), ('3', 3), ('2', 2), ('1', 1), ('J', 0)]);
    let mut res: Vec<(i64, i64)> = hands.map(|h| 
        ({
            let mut all: HashMap<char, usize> = HashMap::new();
            h.0.iter().for_each(|card| {
                if *card != 'J' {
                    all.insert(*card, {
                        match all.get(card) { Some(c) => c + 1, None => 1 }
                    });
                }
            });
            let mut vals: Vec<(char, usize)> = all.into_iter().map(|e| e).collect();
            vals.sort_by(|a, b| b.1.cmp(&a.1));
            all = HashMap::new();
            h.0.iter().for_each(|card| {
                let replacedcard = if *card == 'J' {vals.iter().nth(0).unwrap_or(&('A', 5)).0}
                    else {card.to_owned()};
                all.insert(replacedcard, {
                    match all.get(&replacedcard) { Some(c) => c + 1, None => 1 }
                });
            });
            let mut vals: Vec<usize> = all.into_iter().map(|e| e.1).collect();
            vals.sort_by(|a, b| b.cmp(&a));
            match (vals.iter().nth(0).unwrap(), vals.iter().nth(1).unwrap_or(&0)) {
                (5, _) => 7, (4, _) => 6, (3, 2) => 5, (3, _) => 4, (2, 2) => 3, (2, _) => 2, _ =>  1
            }
        } * (10 as i64).pow(10)
         + (0..=4).into_iter().map(|i| {
            card_values.get(h.0.iter().nth(i).unwrap()).unwrap().to_owned() * (10 as i64).pow(8 - 2*i as u32)
        }).sum::<i64>(),
        h.1)
    ).collect::<Vec<(i64, i64)>>();
    res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    res.into_iter().enumerate().map(|(i, e)| ((i + 1) as i64) * e.1).sum()
}

pub fn d8p1() -> i64 {
    let file: String = fs::read_to_string("input/day8").unwrap();
    let directions: Vec<char> = file.split("\n").nth(0).unwrap().chars().collect::<Vec<char>>();
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    file.split("\n").skip(2).for_each(|nl| {
        Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap().captures_iter(nl).for_each(|c| {
            network.insert(c.iter().nth(1).unwrap().unwrap().as_str().to_owned(),
            (c.iter().nth(2).unwrap().unwrap().as_str().to_owned(), c.iter().nth(3).unwrap().unwrap().as_str().to_owned()));
        });
    });
    let mut currentnode: String = "AAA".to_owned();
    let mut di: usize = 0;
    let mut steps: i64 = 0;
    while currentnode != "ZZZ" {
        match directions[di] {
            'L' => {currentnode = network.get(&currentnode).unwrap().0.clone()},
            'R' => {currentnode = network.get(&currentnode).unwrap().1.clone()},
            _ => {}
        }
        di = (di + 1) % directions.len();
        steps += 1;
    }
    steps
}

pub fn d8p2() -> i64 {
    let file: String = fs::read_to_string("input/day8").unwrap();
    let directions: Vec<char> = file.split("\n").nth(0).unwrap().chars().collect::<Vec<char>>();
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    file.split("\n").skip(2).for_each(|nl| {
        Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap().captures_iter(nl).for_each(|c| {
            network.insert(c.iter().nth(1).unwrap().unwrap().as_str().to_owned(),
            (c.iter().nth(2).unwrap().unwrap().as_str().to_owned(), c.iter().nth(3).unwrap().unwrap().as_str().to_owned()));
        });
    });
    let currentnodes: Vec<&String> = network.keys().filter(|k| k.chars().into_iter().nth(2).unwrap() == 'A').collect::<Vec<&String>>(); 
    let mut di: usize = 0;
    currentnodes.clone().into_iter().map(|cn| {
        let (mut steps, mut iterations, mut iteratingnode) = (0, 0, cn.to_owned());
        while iteratingnode.chars().into_iter().nth(2).unwrap() != 'Z' { 
            match directions[di] {
                'L' => { iteratingnode = network.get(&iteratingnode).unwrap().0.clone(); },
                'R' => { iteratingnode = network.get(&iteratingnode).unwrap().1.clone(); },
                _ => {}
            }
            di = (di + 1) % directions.len();
            if di == 0 {iterations += 1}
            steps = if di == 0 {0} else {steps + 1}
        };
        steps + (directions.len() as i64) * iterations
    }).reduce(|a, b| {
        let (mut gcd, mut rem) = (a, b);
        while rem != 0 {
            let tmp: i64 = rem;
            rem = gcd % rem;
            gcd = tmp;
        }
        a * b / gcd
    }).unwrap()
}

pub fn d9p1() -> i64 {
    let file = fs::read_to_string("input/day9").unwrap();
    file.split("\n").map(|l| {
        let mut allvalues: Vec<Vec<i64>> = vec![l.split(" ").map(|v| v.parse::<i64>().unwrap()).collect()];
        while allvalues.last().unwrap().into_iter().find(|v| **v != 0).is_some() {
            let last = allvalues.last().unwrap();
            allvalues.push((0..(last.len() - 1)).map(|i| {
                last[i + 1] - last[i]
            }).collect::<Vec<i64>>());
        }
        allvalues.into_iter().rev().map(|vs| {
            vs.last().unwrap().to_owned()
        }).sum::<i64>()
    }).sum::<i64>()
}

pub fn d9p2() -> i64 {
    let file = fs::read_to_string("input/day9").unwrap();
    file.split("\n").map(|l| {
        let mut allvalues: Vec<Vec<i64>> = vec![l.split(" ").map(|v| v.parse::<i64>().unwrap()).collect()];
        while allvalues.last().unwrap().into_iter().find(|v| **v != 0).is_some() {
            let last = allvalues.last().unwrap();
            allvalues.push((0..(last.len() - 1)).map(|i| {
                last[i + 1] - last[i]
            }).collect::<Vec<i64>>());
        }
        allvalues.into_iter().rev().map(|vs| {
            vs.first().unwrap().to_owned()
        }).reduce(|a, b| -a + b).unwrap()
    }).sum::<i64>()
}

pub fn d10p1() -> i64 {
    let file = fs::read_to_string("input/day10").unwrap();
    let mut map: HashMap<(usize, usize), (bool, bool, bool, bool)>= HashMap::new();
    let mut start: (usize, usize) = (0, 0);
    file.split("\n").enumerate().for_each(|(li, l)| {
        l.chars().into_iter().enumerate().for_each(|(ci, c)| {
            let kind: Option<(bool, bool, bool, bool)> = match c {
                '|' => Some((true, false, true, false)),
                '-' => Some((false, true, false, true)),
                'L' => Some((true, true, false, false)),
                'J' => Some((true, false, false, true)),
                '7' => Some((false, false, true, true)),
                'F' => Some((false, true, true, false)),
                _ => None
            };
            if kind.is_some() {
                map.insert((li, ci), kind.unwrap());
            }
            if c == 'S' {start = (li, ci);}
        });
    });
    let mut current = vec![];
    if map.get(&(start.0 - 1, start.1)).unwrap_or(&(false, false, false, false)).2 {
        current.push((start.0 - 1, start.1))
    }
    if map.get(&(start.0, start.1 + 1)).unwrap_or(&(false, false, false, false)).3 {
        current.push((start.0, start.1 + 1))
    }
    if map.get(&(start.0 + 1, start.1)).unwrap_or(&(false, false, false, false)).0 {
        current.push((start.0 + 1, start.1))
    }
    if map.get(&(start.0, start.1 - 1)).unwrap_or(&(false, false, false, false)).1 {
        current.push((start.0, start.1 - 1))
    }
    let mut step: i64 = 0;
    let mut partoftheloop: Vec<(usize, usize)> = vec![start.clone()];
    while current.len() > 0 {
        partoftheloop.push(current[0]);
        partoftheloop.push(current[1]);
        let mut newcurrent = vec![];
        [0, 1].into_iter().for_each(|ci| {
            let currentnode = map.get(&current[ci]).unwrap();
            if currentnode.0 && !partoftheloop.contains(&(current[ci].0 - 1, current[ci].1))
                && map.get(&(current[ci].0 - 1, current[ci].1)).unwrap_or(&(false, false, false, false)).2 {
                newcurrent.push((current[ci].0 - 1, current[ci].1));
            }
            if currentnode.1 && !partoftheloop.contains(&(current[ci].0, current[ci].1 + 1))
                && map.get(&(current[ci].0, current[ci].1 + 1)).unwrap_or(&(false, false, false, false)).3 {
                newcurrent.push((current[ci].0, current[ci].1 + 1));
            }
            if currentnode.2 && !partoftheloop.contains(&(current[ci].0 + 1, current[ci].1))
                && map.get(&(current[ci].0 + 1, current[ci].1)).unwrap_or(&(false, false, false, false)).0 {
                newcurrent.push((current[ci].0 + 1, current[ci].1));
            }
            if currentnode.3 && !partoftheloop.contains(&(current[ci].0, current[ci].1 - 1))
                && map.get(&(current[ci].0, current[ci].1 - 1)).unwrap_or(&(false, false, false, false)).1 {
                newcurrent.push((current[ci].0, current[ci].1 - 1));
            }
        });
        step = step + 1;
        current = newcurrent;
    }
    step
}

pub fn d10p2() -> i64 {
    let file = fs::read_to_string("input/day10").unwrap();
    let mut map: HashMap<(usize, usize), (bool, bool, bool, bool)>= HashMap::new();
    let mut start: (usize, usize) = (0, 0);
    let mut width: usize = 0;
    let mut height: usize = 0;
    file.split("\n").enumerate().for_each(|(li, l)| {
        height = li + 1;
        l.chars().into_iter().enumerate().for_each(|(ci, c)| {
            width = ci + 1;
            let kind: Option<(bool, bool, bool, bool)> = match c {
                '|' => Some((true, false, true, false)),
                '-' => Some((false, true, false, true)),
                'L' => Some((true, true, false, false)),
                'J' => Some((true, false, false, true)),
                '7' => Some((false, false, true, true)),
                'F' => Some((false, true, true, false)),
                _ => None
            };
            if kind.is_some() {
                map.insert((li, ci), kind.unwrap());
            }
            if c == 'S' {start = (li, ci);}
        });
    });
    let mut current = vec![];
    if map.get(&(start.0 - 1, start.1)).unwrap_or(&(false, false, false, false)).2 {
        current.push((start.0 - 1, start.1))
    }
    if map.get(&(start.0, start.1 + 1)).unwrap_or(&(false, false, false, false)).3 {
        current.push((start.0, start.1 + 1))
    }
    if map.get(&(start.0 + 1, start.1)).unwrap_or(&(false, false, false, false)).0 {
        current.push((start.0 + 1, start.1))
    }
    if map.get(&(start.0, start.1 - 1)).unwrap_or(&(false, false, false, false)).1 {
        current.push((start.0, start.1 - 1))
    }
    map.insert((start.0, start.1), (
        map.get(&(start.0 - 1, start.1)).unwrap_or(&(false, false, false, false)).2,
        map.get(&(start.0, start.1 + 1)).unwrap_or(&(false, false, false, false)).3,
        map.get(&(start.0 + 1, start.1)).unwrap_or(&(false, false, false, false)).0,
        map.get(&(start.0, start.1 - 1)).unwrap_or(&(false, false, false, false)).1
    ));
    let mut step: i64 = 0;
    let mut partoftheloop: Vec<(usize, usize)> = vec![start.clone()];
    while current.len() > 0 {
        partoftheloop.push(current[0]);
        partoftheloop.push(current[1]);
        let mut newcurrent = vec![];
        [0, 1].into_iter().for_each(|ci| {
            let currentnode = map.get(&current[ci]).unwrap();
            if currentnode.0 && !partoftheloop.contains(&(current[ci].0 - 1, current[ci].1))
                && map.get(&(current[ci].0 - 1, current[ci].1)).unwrap_or(&(false, false, false, false)).2 {
                newcurrent.push((current[ci].0 - 1, current[ci].1));
            }
            if currentnode.1 && !partoftheloop.contains(&(current[ci].0, current[ci].1 + 1))
                && map.get(&(current[ci].0, current[ci].1 + 1)).unwrap_or(&(false, false, false, false)).3 {
                newcurrent.push((current[ci].0, current[ci].1 + 1));
            }
            if currentnode.2 && !partoftheloop.contains(&(current[ci].0 + 1, current[ci].1))
                && map.get(&(current[ci].0 + 1, current[ci].1)).unwrap_or(&(false, false, false, false)).0 {
                newcurrent.push((current[ci].0 + 1, current[ci].1));
            }
            if currentnode.3 && !partoftheloop.contains(&(current[ci].0, current[ci].1 - 1))
                && map.get(&(current[ci].0, current[ci].1 - 1)).unwrap_or(&(false, false, false, false)).1 {
                newcurrent.push((current[ci].0, current[ci].1 - 1));
            }
        });
        step = step + 1;
        current = newcurrent;
    }
    (0..height).into_iter().map(|h| {
        (0..width).into_iter().map(|w| {
            if !partoftheloop.contains(&(h, w))
                && ({
                    let [mut bottom, mut top] = [false, false];
                    let mut barriers = 0;
                    partoftheloop.iter().filter(|e| e.0 < h && e.1 == w).into_iter().map(|e| map.get(&e).unwrap()).for_each(|e| {
                        if e.3 {bottom = !bottom}
                        if e.1 {top = !top}
                        if bottom && top {
                            [bottom, top] = [false, false];
                            barriers += 1;
                        }
                    });
                    barriers % 2 == 1
                }
                || {
                    let [mut left, mut right] = [false, false];
                    let mut barriers = 0;
                    partoftheloop.iter().filter(|e| e.0 == h && e.1 < w).into_iter().map(|e| map.get(&e).unwrap()).for_each(|e| {
                        if e.0 {right = !right}
                        if e.2 {left = !left}
                        if left && right {
                            [left, right] = [false, false];
                            barriers += 1;
                        }
                    });
                    barriers % 2 == 1
                }) {1} else {0}
        }).sum::<i64>()
    }).sum::<i64>()
}

pub fn d11p1() -> i64 {
    let file = fs::read_to_string("input/day11").unwrap();
    let mut original_cosmos: Vec<Vec<bool>> = vec![];
    file.split("\n").for_each(|l| {
        let mut line: Vec<bool> = vec![];
        l.chars().into_iter().for_each(|c| {
            line.push(c == '#');
        });
        original_cosmos.push(line);
    });
    let [height, width] = [original_cosmos.len(), original_cosmos.first().unwrap().len()];
    let mut emptylines: Vec<usize> = vec![];
    let mut emptyrows: Vec<usize> = vec![];
    (0..height).into_iter().for_each(|h| {
        if original_cosmos[h].iter().map(|e| if *e {1} else {0}).sum::<usize>() == 0 {
            emptylines.push(h)
        }
    });
    (0..width).into_iter().for_each(|w| {
        if original_cosmos.iter().map(|e| if e[w] {1} else {0}).sum::<usize>() == 0 {
            emptyrows.push(w)
        }
    });
    let mut galaxies: Vec<(usize, usize)> = vec![];
    original_cosmos.iter().enumerate().for_each(|(li, l)| {
        l.iter().enumerate().for_each(|(ei, e)| {
            if *e { galaxies.push((li, ei)); }
        });
    });
    galaxies.clone().iter().enumerate().for_each(|(gi1, g1)| {
        galaxies[gi1] = (g1.0 + emptylines.iter().map(|el| { if *el < g1.0 {1} else {0} }).sum::<usize>(),
        g1.1 + emptyrows.iter().map(|er| { if *er < g1.1 {1} else {0} }).sum::<usize>());
    });
    galaxies.iter().enumerate().map(|(gi1, g1)| {
        ((gi1 + 1)..galaxies.len()).into_iter().map(|gi2| {
            ((g1.0 - galaxies[gi2].0) as i64).abs() + ((g1.1 - galaxies[gi2].1) as i64).abs()
        }).sum::<i64>()
    }).sum::<i64>()
}

pub fn d11p2() -> i64 {
    let file = fs::read_to_string("input/day11").unwrap();
    let mut original_cosmos: Vec<Vec<bool>> = vec![];
    file.split("\n").for_each(|l| {
        let mut line: Vec<bool> = vec![];
        l.chars().into_iter().for_each(|c| {
            line.push(c == '#');
        });
        original_cosmos.push(line);
    });
    let [height, width] = [original_cosmos.len(), original_cosmos.first().unwrap().len()];
    let mut emptylines: Vec<usize> = vec![];
    let mut emptyrows: Vec<usize> = vec![];
    (0..height).into_iter().for_each(|h| {
        if original_cosmos[h].iter().map(|e| if *e {1} else {0}).sum::<usize>() == 0 {
            emptylines.push(h)
        }
    });
    (0..width).into_iter().for_each(|w| {
        if original_cosmos.iter().map(|e| if e[w] {1} else {0}).sum::<usize>() == 0 {
            emptyrows.push(w)
        }
    });
    let mut galaxies: Vec<(usize, usize)> = vec![];
    original_cosmos.iter().enumerate().for_each(|(li, l)| {
        l.iter().enumerate().for_each(|(ei, e)| {
            if *e { galaxies.push((li, ei)); }
        });
    });
    galaxies.clone().iter().enumerate().for_each(|(gi1, g1)| {
        galaxies[gi1] = (g1.0 + emptylines.iter().map(|el| {
            if *el < g1.0 {1} else {0}
        }).sum::<usize>() * 999999,
        g1.1 + emptyrows.iter().map(|er| {
            if *er < g1.1 {1} else {0}
        }).sum::<usize>() * 999999);
    });
    galaxies.iter().enumerate().map(|(gi1, g1)| {
        ((gi1 + 1)..galaxies.len()).into_iter().map(|gi2| {
            ((g1.0 - galaxies[gi2].0) as i64).abs() + ((g1.1 - galaxies[gi2].1) as i64).abs()
        }).sum::<i64>()
    }).sum::<i64>()
}
