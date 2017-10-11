use std::collections::BTreeMap;

pub struct School(BTreeMap<u32, Vec<String>>);

impl School {
    pub fn new() -> Self {
        School(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.0.entry(grade).or_insert(Vec::new());
        students.push(student.into());
        students.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.0.get(&grade).map(|students| students.clone())
    }
}
