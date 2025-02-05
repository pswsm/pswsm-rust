use std::io::Read;

const INC_FLAG: u8 = 0;
const DEC_FLAG: u8 = 2;
type LocalReport = Vec<usize>;

fn main() -> Result<(), std::io::Error> {
    let mut buf = String::new();
    let mut file = std::fs::File::open("input.txt")?;

    file.read_to_string(&mut buf)?;

    let reps_raw: Vec<String> = buf.lines().map(String::from).collect();
    let reports: Vec<LocalReport> = reps_raw
        .iter()
        .map(|level| {
            level
                .split_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut safety_vec: Vec<bool> = Vec::with_capacity(reports.len());

    let mut compare_flag = INC_FLAG;
    for report in reports {
        match should_skip(&report) {
            true => {
                safety_vec.push(false);
            }
            false => {
                check_level_order(&report, &mut compare_flag);
                let is_safe = match compare_flag {
                    INC_FLAG => check_increase(&report),
                    DEC_FLAG => check_decrease(&report),
                    _ => unreachable!(),
                };
                safety_vec.push(is_safe);
            }
        }
    }

    let safes: Vec<bool> = safety_vec.into_iter().filter(|r| r == &true).collect();

    println!("{:?}", safes.len());

    Ok(())
}

fn should_skip(local_report: &LocalReport) -> bool {
    let mut uniq = std::collections::HashSet::new();
    !local_report.iter().all(|x| uniq.insert(x))
}

fn check_level_order(level: &LocalReport, flag_holder: &mut u8) -> () {
    // println!("level[0]: {} --- level[1]: {}", level[0], level[1]);
    if level[0] < level[1] {
        // println!("Setting INC_FLAG");
        *flag_holder = INC_FLAG;
    } else {
        // println!("Setting DEC_FLAG");
        *flag_holder = DEC_FLAG;
    }
    ()
}

fn check_increase(local_report: &LocalReport) -> bool {
    let mut safety_local: Vec<bool> = Vec::new();
    let mut report_peek = local_report.iter().peekable();
    while let (Some(level), Some(next_level)) = (report_peek.next(), report_peek.peek()) {
        if [
            next_level,
            &next_level.saturating_sub(1),
            &next_level.saturating_sub(2),
            &next_level.saturating_sub(3),
        ]
        .contains(&level)
        {
            safety_local.push(true);
        } else {
            safety_local.push(false);
        }
    }
    // println!("inc --- {:?}", safety_local);
    if !safety_local.iter().all(|a| a == &true) {
        return false;
    }
    true
}

fn check_decrease(local_report: &LocalReport) -> bool {
    let mut safety_local: Vec<bool> = Vec::new();
    let mut report_peek = local_report.iter().peekable();
    while let (Some(level), Some(next_level)) = (report_peek.next(), report_peek.peek()) {
        if [
            next_level,
            &next_level.saturating_add(1),
            &next_level.saturating_add(2),
            &next_level.saturating_add(3),
        ]
        .contains(&level)
        {
            safety_local.push(true);
        } else {
            safety_local.push(false)
        };
    }
    // println!("dec --- {:?}", safety_local);
    if !safety_local.iter().all(|a| a == &true) {
        return false;
    }
    true
}
