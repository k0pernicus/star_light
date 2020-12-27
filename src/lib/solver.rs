use std::{
    collections::VecDeque,
    fmt::{self, Display},
    str::FromStr,
};

const MAX_LIGHTS: usize = 25;

#[derive(Debug, PartialEq, Eq)]
pub struct Lights(Vec<bool>);

impl Display for Lights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for light in self.0.iter() {
            let write_result = match light {
                true => write!(f, "1"),
                false => write!(f, "0"),
            };
            if write_result.is_err() {
                return write_result;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum SolverError {
    IndexOutOfBounds,
    NoLight,
    TooManyLights,
}

impl Display for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolverError::IndexOutOfBounds => write!(f, "index out of bounds"),
            SolverError::NoLight => write!(f, "not enough lights to process"),
            SolverError::TooManyLights => {
                write!(f, "too much lights to process (max is {})", MAX_LIGHTS)
            }
        }
    }
}

impl Lights {
    // Flip a switch at index `index`.
    pub fn flip(&mut self, index: usize) -> Result<(), SolverError> {
        if index >= self.0.len() {
            return Err(SolverError::IndexOutOfBounds);
        }
        self.0[index] = !self.0[index];
        Ok(())
    }
    // Returns a boolean value that represents if a light could be switched or not.
    pub fn could_be_flipped(&self, index: usize) -> bool {
        let nb_elements = self.0.len();
        // The last light could always be flipped...
        if index == nb_elements - 1 {
            return true;
        }
        if index == nb_elements - 2 {
            return self.0[nb_elements - 1];
        }
        (!self.0[index + 2..].iter().any(|l| *l)) && self.0[index + 1]
    }
    // Returns the first light to switch, from left to right, in order to match `target`, from index `first_index`.
    // If no light has to be switch, return None.
    pub fn get_first_different_index(&self, target: &Lights, first_index: usize) -> Option<usize> {
        let zip_iter = self.0[first_index..]
            .iter()
            .zip(target.0[first_index..].iter());
        for (index, (init_elt, target_elt)) in zip_iter.enumerate() {
            if init_elt != target_elt {
                return Some(index);
            }
        }
        None
    }
    // Returns the index of the light to switch in order to switch the light at index `index`, if it exists.
    // If no light has to be switch, return None.
    pub fn need_light_to_flip(&self, index: usize) -> Option<usize> {
        if self.could_be_flipped(index) {
            return None;
        }
        if !self.0[index + 1] {
            return Some(index + 1);
        }
        for (add_index, light) in self.0[index + 2..].iter().enumerate() {
            if *light {
                let index_to_flip = add_index + 2 + index;
                return Some(index_to_flip);
            }
        }
        None
    }
}

impl FromStr for Lights {
    type Err = SolverError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(SolverError::NoLight);
        }
        if s.len() > MAX_LIGHTS {
            return Err(SolverError::TooManyLights);
        }
        Ok(Lights(
            s.chars()
                .filter_map(|c| match c {
                    '1' => Some(true),
                    '0' => Some(false),
                    _ => None,
                })
                .collect::<Vec<bool>>(),
        ))
    }
}

pub fn get_min_steps(mut start: Lights, target: Lights, limit: Option<usize>) -> Option<usize> {
    let mut steps = 0;
    // Easy win...
    if start == target {
        return Some(steps);
    }
    // Check if both inputs are correct
    let nb_lights = start.0.len();
    if nb_lights != target.0.len() {
        return None;
    }
    // Initialize the VecDeque to push / pop the lights to switch
    let mut lights_to_flips: VecDeque<usize> = VecDeque::new();
    // Get the first light to swift (from the left), and push it to the VecDeque
    let left_light = start.get_first_different_index(&target, 0);
    lights_to_flips.push_back(left_light.unwrap());
    // Now, let's solve the problem...
    while !lights_to_flips.is_empty() {
        // If user limits have been set and reached, stop there
        if let Some(limit) = limit {
            if steps >= limit {
                println!("Warning: Users limits have been reached...");
                break;
            }
        }
        // Get the light to flip
        let mut first_light_to_flip = lights_to_flips.pop_back().unwrap();
        // Search for the other lights to flip first
        loop {
            match start.need_light_to_flip(first_light_to_flip) {
                Some(new_light_to_flip) => {
                    if lights_to_flips.contains(&new_light_to_flip) {
                        break;
                    }
                    lights_to_flips.push_back(first_light_to_flip);
                    first_light_to_flip = new_light_to_flip;
                }
                None => {
                    break;
                }
            }
        }
        // If something failed during the switch, stop there
        if let Err(error) = start.flip(first_light_to_flip) {
            println!(
                "Error when flipping light at index {}, due to {}",
                first_light_to_flip, error
            );
            break;
        }
        // Push the next if empty
        if lights_to_flips.is_empty() {
            if let Some(next_index) = start.get_first_different_index(&target, 0) {
                lights_to_flips.push_back(next_index);
            }
        }
        steps += 1;
    }
    Some(steps)
}

#[cfg(test)]
mod test {
    use super::{get_min_steps, Lights};
    use std::str::FromStr;
    #[test]
    fn examples_tests() {
        let lights = [
            ("1101", "0100"),
            ("101010", "010101"),
            ("11001001000", "10000110011"),
        ];
        let expected_outputs = [2, 26, 877];
        for index in 0..3 {
            let (start_lights, target_lights) = lights[index];
            let expected_output = expected_outputs[index];
            let start = Lights::from_str(start_lights);
            if start.is_err() {
                panic!(
                    "Error when parsing the start from {}, due to {}",
                    start_lights,
                    start.unwrap_err()
                );
            }
            let target = Lights::from_str(target_lights);
            if target.is_err() {
                panic!(
                    "Error when parsing the target from {}, due to {}",
                    target_lights,
                    start.unwrap_err()
                );
            }
            let nb_steps = get_min_steps(start.unwrap(), target.unwrap(), None);
            match nb_steps {
                Some(nb_steps) => assert_eq!(nb_steps, expected_output),
                None => panic!("Error: received None from get_min_steps..."),
            }
        }
    }
}
