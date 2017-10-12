use std::fmt;
use std::collections::HashMap;

struct Team {
    name: String,
    wins: u32,
    losses: u32,
    draws: u32,
}

impl Team {
    fn new(name: &str) -> Self {
        Team {
            name: name.to_string(),
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }

    fn matches_played(&self) -> u32 {
        self.wins + self.losses + self.draws
    }

    fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            self.name,
            self.matches_played(),
            self.wins,
            self.draws,
            self.losses,
            self.points()
        )
    }
}

struct Tournament(HashMap<String, Team>);

impl Tournament {
    fn new() -> Self {
        Tournament(HashMap::new())
    }

    fn win(&mut self, home: &str, away: &str) {
        self.0
            .entry(home.to_string())
            .or_insert(Team::new(home))
            .wins += 1;

        self.0
            .entry(away.to_string())
            .or_insert(Team::new(away))
            .losses += 1;
    }

    fn draw(&mut self, home: &str, away: &str) {
        self.0
            .entry(home.to_string())
            .or_insert(Team::new(home))
            .draws += 1;

        self.0
            .entry(away.to_string())
            .or_insert(Team::new(away))
            .draws += 1;
    }
}

impl fmt::Display for Tournament {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut teams = self.0.values().collect::<Vec<_>>();

        teams.sort_by(|a, b| {
            a.points().cmp(&b.points()).reverse().then(
                a.name.cmp(&b.name),
            )
        });

        write!(
            f,
            "{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            "Team",
            "MP",
            "W",
            "D",
            "L",
            "P",
        )?;

        for team in teams {
            write!(f, "\n{}", team)?;
        }

        Ok(())
    }
}

pub fn tally(input: &str) -> String {
    let mut tournament = Tournament::new();

    for line in input.lines() {
        let line = line.split(';').collect::<Vec<_>>();

        match line[2] {
            "win" => tournament.win(line[0], line[1]),
            "loss" => tournament.win(line[1], line[0]),
            "draw" => tournament.draw(line[0], line[1]),
            _ => (),
        }
    }

    tournament.to_string()
}
