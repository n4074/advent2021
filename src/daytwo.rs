use anyhow::Result;

pub fn partone(input: String) -> Result<()> {
    let mut depth = 0;
    let mut displacement = 0;

    for line in input.lines() {
        let command: Vec<&str> = line.split(" ").collect();
        let direction = command.get(0);
        let distance = command.get(1).and_then(|s| s.parse::<u32>().ok());
        match (direction, distance) {
           (Some(&"forward"), Some(n)) => { displacement += n; }
           (Some(&"down"), Some(n)) => { depth += n; }
           (Some(&"up"), Some(n)) => { depth -= n; } 
           _ => anyhow::bail!("Invalid input line: {}", line)
        }
    }

    println!("{}", depth * displacement);

    Ok(())
}

pub fn parttwo(input: String) -> Result<()> {
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut displacement: i32 = 0;

    for line in input.lines() {
        let components: Vec<&str> = line.split(" ").collect();
        let command = components.get(0);
        let operand = components.get(1).and_then(|s| s.parse::<i32>().ok());
        match (command, operand) {
           (Some(&"forward"), Some(n)) => { displacement += n; depth += n * aim; }
           (Some(&"down"), Some(n)) => { aim += n; }
           (Some(&"up"), Some(n)) => { aim -= n; } 
           _ => anyhow::bail!("Invalid input line: {}", line)
        }
    }

    println!("{}", depth * displacement);

    Ok(())
}