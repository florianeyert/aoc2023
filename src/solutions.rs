use std::fs;
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
                                let sourceparsed = c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap();
                                let destinationparsed = c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap();
                                source = sourceparsed.as_str().to_owned();
                                destination = destinationparsed.as_str().to_owned();
                            });
                        },
                        _ => {
                            Regex::new(r"([0-9]+) ([0-9]+) ([0-9]+)").unwrap().captures_iter(line).for_each(|c| {
                                let destinationrange = c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap();
                                let sourcerange = c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap();
                                let rangelen = c.iter().filter(|cc| cc.is_some()).nth(3).unwrap().unwrap();
                                numbers.push((destinationrange.as_str().parse::<i64>().unwrap(),
                                sourcerange.as_str().parse::<i64>().unwrap(), rangelen.as_str().parse::<i64>().unwrap()));
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
                    range.0 + (currentnumber - range.1)
                } else {0}
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
                                let sourceparsed = c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap();
                                let destinationparsed = c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap();
                                source = sourceparsed.as_str().to_owned();
                                destination = destinationparsed.as_str().to_owned();
                            });
                        },
                        _ => {
                            Regex::new(r"([0-9]+) ([0-9]+) ([0-9]+)").unwrap().captures_iter(line).for_each(|c| {
                                let destinationrange = c.iter().filter(|cc| cc.is_some()).nth(1).unwrap().unwrap();
                                let sourcerange = c.iter().filter(|cc| cc.is_some()).nth(2).unwrap().unwrap();
                                let rangelen = c.iter().filter(|cc| cc.is_some()).nth(3).unwrap().unwrap();
                                numbers.push((destinationrange.as_str().parse::<i64>().unwrap(),
                                sourcerange.as_str().parse::<i64>().unwrap(), rangelen.as_str().parse::<i64>().unwrap()));
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
