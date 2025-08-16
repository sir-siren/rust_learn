/*
We're running a salad restaurant! You discover some starter
code from a previous developer working at the company. The
code  includes:
- A Vegetable enum
- A Protein enum
- A Dressing enum

Our next goal is to build a Salad struct. Each Salad will
have a 'protein', 'vegetables', and a 'dressing' field. A
Salad can store 1 protein, any number of vegetables, and
1 dressing. Use a vector to store the vegetables. Derive
the Debug trait.

We need to implement the following 4 functions/methods on
a Salad. All 4 must have a complementary unit test. It's up
to you whether you want to write your tests first (TDD) or
write your implementation first. Follow the best practices
for unit tests (modules, configuration, etc). Feel free
to utilize any helper crates (pretty_assertions, rstest,
etc).

First, define a 'new' constructor function that accepts a
'protein', a 'vegetables' vector, and a 'dressing' and
returns an instance of the Salad. In the test, assert that
the 3 fields of the Salad are correctly populated.

Next, define an 'is_valid' method that returns a Boolean.
Return a true if a salad has more than 0 vegetables.

Next, define a 'calories' method that calculates the total
calories in the salad. The Vegetable, Protein, and Dressing
enums all support a 'calories' method that return the
calories of the item. Remember that 'vegetables' is a vector
of multiple Vegetable values -- you'll have to include all of
them in your calculation.

Finally, define a 'has_duplicate_vegetables' method. It
should determine if the salad includes any vegetable more
than once. Return a Boolean.
*/

pub trait Caloric {
    fn calories(&self) -> u32;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Vegetable {
    Tomato,
    Cucumber,
    SweetPotato,
}

impl Caloric for Vegetable {
    fn calories(&self) -> u32 {
        match self {
            Self::Tomato => 20,
            Self::Cucumber => 15,
            Self::SweetPotato => 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protein {
    CrispyChicken,
    FriedChicken,
    Steak,
    Tofu,
}

impl Caloric for Protein {
    fn calories(&self) -> u32 {
        match self {
            Self::CrispyChicken => 400,
            Self::FriedChicken => 500,
            Self::Steak => 300,
            Self::Tofu => 200,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dressing {
    Ranch,
    Vinaigrette,
    Italian,
}

impl Caloric for Dressing {
    fn calories(&self) -> u32 {
        match self {
            Self::Ranch => 150,
            Self::Vinaigrette => 120,
            Self::Italian => 130,
        }
    }
}

#[derive(Debug)]
pub struct Salad {
    protein: Protein,
    vegetables: Vec<Vegetable>,
    dressing: Dressing,
}

impl Salad {
    pub fn new(protein: Protein, vegetables: Vec<Vegetable>, dressing: Dressing) -> Self {
        Self {
            protein,
            vegetables,
            dressing,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.vegetables.is_empty()
    }

    pub fn calories(&self) -> u32 {
        let veggie_calories: u32 = self.vegetables.iter().map(|v| v.calories()).sum();
        self.protein.calories() + veggie_calories + self.dressing.calories()
    }

    pub fn has_duplicate_vegetables(&self) -> bool {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        for veggie in &self.vegetables {
            if !seen.insert(veggie) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_salad() {
        let protein = Protein::Tofu;
        let vegetables = vec![Vegetable::Tomato, Vegetable::Cucumber];
        let dressing = Dressing::Vinaigrette;
        let salad = Salad::new(protein.clone(), vegetables.clone(), dressing.clone());

        assert_eq!(salad.protein, protein);
        assert_eq!(salad.vegetables, vegetables);
        assert_eq!(salad.dressing, dressing);
    }

    #[test]
    fn test_is_valid() {
        let valid_salad = Salad::new(
            Protein::Tofu,
            vec![Vegetable::Tomato],
            Dressing::Vinaigrette,
        );
        assert!(valid_salad.is_valid());

        let invalid_salad = Salad::new(Protein::Tofu, vec![], Dressing::Vinaigrette);
        assert!(!invalid_salad.is_valid());
    }

    #[test]
    fn test_calories() {
        let salad = Salad::new(
            Protein::Tofu,
            vec![Vegetable::Tomato, Vegetable::SweetPotato],
            Dressing::Vinaigrette,
        );
        // Tofu: 200, Tomato: 20, SweetPotato: 100, Vinaigrette: 120
        // Total: 200 + 20 + 100 + 120 = 440
        assert_eq!(salad.calories(), 440);
    }

    #[test]
    fn test_has_duplicate_vegetables() {
        let salad_with_duplicates = Salad::new(
            Protein::Tofu,
            vec![Vegetable::Tomato, Vegetable::Cucumber, Vegetable::Tomato],
            Dressing::Vinaigrette,
        );
        assert!(salad_with_duplicates.has_duplicate_vegetables());

        let salad_without_duplicates = Salad::new(
            Protein::Tofu,
            vec![
                Vegetable::Tomato,
                Vegetable::Cucumber,
                Vegetable::SweetPotato,
            ],
            Dressing::Vinaigrette,
        );
        assert!(!salad_without_duplicates.has_duplicate_vegetables());
    }
}
