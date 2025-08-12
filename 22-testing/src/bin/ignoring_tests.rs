// lib.rs
#[derive(Debug)]
struct Museum {
    paintings: Vec<String>,
    revenue: u32,
}

impl Museum {
    const MAXIMUM_CAPACITY: usize = 3;

    fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    fn buy_painting(&mut self, painting: &str) {
        if self.paintings.len() >= Self::MAXIMUM_CAPACITY {
            panic!("Museum does not have storage space for another painting!");
        }

        self.paintings.push(painting.to_string());
    }

    fn sell_ticket(&mut self) {
        self.revenue += 25;
    }

    fn has_impressive_collection(&self) -> bool {
        self.paintings.len() > 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn museum_sells_ticket_to_increase_revenue() -> Result<(), String> {
        let mut museum = Museum::new();
        museum.sell_ticket();

        if museum.revenue == 25 {
            Ok(())
        } else {
            Err(String::from(
                "The revenue from selling 1 ticket did not match expectations.",
            ))
        }
    }

    #[test]
    fn museum_can_sell_multiple_tickets() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 50);
    }

    #[test]
    fn museum_can_have_impressive_art_collection() -> Result<(), String> {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");

        if museum.has_impressive_collection() {
            Ok(())
        } else {
            Err(String::from(
                "The museum did not have an impressive collection despite having more than 2 paintings.",
            ))
        }
    }

    #[test]
    #[ignore]
    fn new_museums_are_equal() {
        let museum1 = Museum::new();
        let museum2 = Museum::new();
        assert_eq!(
            museum1, museum2,
            "Two new Museum instances were not found to be equal: {museum1:?} // {museum2:?}"
        );
    }

    #[test]
    #[should_panic(expected = "storage space")]
    fn it_prohibits_adding_painting_when_capacity_has_been_reached() {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");
        museum.buy_painting("Water Lilies");
    }
}
