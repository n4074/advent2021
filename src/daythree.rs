use anyhow::Result;

pub fn count_bits(input: &str) -> Result<Vec<i32>> {
    let bitlen = input.lines().next().unwrap().len();
    let mut counts = vec![0i32; bitlen];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => { counts[i] -= 1 }
                '1' => { counts[i] += 1 }
                _ => { anyhow::bail!("Unexpected input character: {}", c); }
            }
        } 
    }

    Ok(counts)
}

pub fn partone(input: String) -> Result<()> {
    
    let counts = count_bits(&input)?;
    let bitlen = counts.len();

    let mut gamma = 0;
    let mut episolon = 0;

    for i in 0..bitlen {
        if counts[i] >= 0 {
            gamma += 2u32.pow((bitlen-i-1) as u32);
        }

        if counts[i] <= 0 {
            episolon += 2u32.pow((bitlen-i-1) as u32);
        }
    }

    println!("{}", gamma*episolon);

    Ok(())
}

pub fn parttwo(input: String) -> Result<()> {
    let bitlen = input.lines().next().unwrap().len() - 1;
    let items: Vec<u32> = input.lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap()).collect();

    let o2 = partition(bitlen as i32, items.clone(), |(zeros, ones)| zeros.len() > ones.len());
    let co2 = partition(bitlen as i32,items, |(zeros, ones)| zeros.len() != 0 && zeros.len() <= ones.len());

    println!("{}", o2[0] * co2[0]);
    
    Ok(())
}

type V = Vec<u32>;

pub fn partition<'a, F>(index: i32, input: V, p: F) -> V where F: Fn((&V,&V)) -> bool {

    if input.len() <= 1 || index < 0 {
        return input;
    }
        
    let mut zeros = vec![];
    let mut ones = vec![];
    for item in input {
        if item & (1u32 << index) == 0 {
            zeros.push(item);
        } else {
            ones.push(item);
        }
    }

    let sub;

    if p((&zeros, &ones)) {
        sub = zeros;
    } else {
        sub = ones;
    }


    partition(index - 1, sub, p)
}