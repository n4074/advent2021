
use anyhow::Result;

pub fn partone(input: String) -> Result<()> {
    let mut depths = input.lines();
    let mut incs = 0u32;
    let mut prev = depths.next().unwrap().parse::<u32>()?;

    for line in depths {
        let depth = line.parse::<u32>()?;
        if depth > prev {
            incs += 1;
        }
        prev = depth;
    }

    println!("{}", incs);

    Ok(())
}

pub fn parttwo(input: String) -> Result<()> {
    let mut depths = input.lines().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut incs = 0u32;

    let mut prev: u32 = depths[0] + depths[1] + depths[2];
    let mut cur: u32;

    for i in 3..depths.len() {
        cur = prev - depths[i-3] + depths[i];
        if cur > prev {
            incs += 1;
        }
        prev = cur;
    }

    println!("{}", incs);

    Ok(())
}