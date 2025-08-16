use saladworks::{Dressing, Protein, Salad, Vegetable};

fn main() {
    let protein = Protein::CrispyChicken;
    let vegetables = vec![
        Vegetable::Tomato,
        Vegetable::Cucumber,
        Vegetable::SweetPotato,
    ];
    let dressing = Dressing::Ranch;

    let salad = Salad::new(protein, vegetables, dressing);

    println!("Our delicious salad: {:?}", salad);
    println!("Is the salad valid? {}", salad.is_valid());
    println!("Total calories: {}", salad.calories());
    println!(
        "Does the salad have duplicate vegetables? {}",
        salad.has_duplicate_vegetables()
    );

    let salad_with_duplicates = Salad::new(
        Protein::Steak,
        vec![Vegetable::Tomato, Vegetable::Tomato],
        Dressing::Italian,
    );
    println!(
        "Salad with duplicates: {:?}",
        salad_with_duplicates.has_duplicate_vegetables()
    );
}
