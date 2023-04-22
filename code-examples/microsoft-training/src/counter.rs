#[derive(Debug, PartialEq)]
pub struct CarDemand {
    length: usize,
    count: usize,
}

impl CarDemand {
    pub fn new(length: usize) -> CarDemand {
        CarDemand { count: 0, length }
    }
}

impl Iterator for CarDemand {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_demand_new() {
        let car_demand = CarDemand::new(5);
        assert_eq!(car_demand.length, 5);
        assert_eq!(car_demand.count, 0);
    }

    #[test]
    fn test_car_demand_iterator() {
        let car_demand = CarDemand::new(3);
        let expected: Vec<usize> = vec![1, 2, 3];
        let actual: Vec<usize> = car_demand.collect();
        assert_eq!(expected, actual);
    }
}
