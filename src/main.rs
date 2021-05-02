//! Handles command-line input, and displays it in an acceptable manner

use abg_code_algorithms::{decode, match_web, CodeAssignments, DecodeError, IngredientAssignments};
use prettytable::{cell, Cell, Row, Table};
use structopt::StructOpt;

const INGREDIENT_SHEET_ORDER: [&'static str; 8] = [
    "Mushroom", "Fern", "Toad", "Claw", "Flower", "Root", "Scorpion", "Feather",
];

const ALCHEMICALS_SHEET_ORDER: [&'static str; 8] = ["B-", "B+", "G-", "G+", "R-", "R+", "--", "++"];

const GOLEM_ORDER: [&'static str; 2] = ["Head", "Chest"];

#[derive(Debug, StructOpt)]
pub struct Input {
    /// Replicate any known companion web app discrepancies
    #[structopt(short, long)]
    pub web: bool,
    /// Output results like the deduction sheet
    #[structopt(short, long)]
    pub sheet: bool,
    /// Companion app codes
    pub codes: Vec<String>,
}

fn main() -> Result<(), DecodeError> {
    let input = Input::from_args();

    let mut results = Vec::with_capacity(input.codes.len());

    for code in input.codes {
        let row = if input.web {
            (code.to_ascii_uppercase(), match_web(&code, decode(&code)?)?)
        } else {
            (code.to_ascii_uppercase(), decode(&code)?)
        };

        results.push(row);
    }

    if input.sheet {
        print_sheet(&results);
    } else {
        print_compact(&results);
    }

    Ok(())
}

// Output formatting past this point

fn print_compact(results: &[(String, CodeAssignments)]) {
    println!(
        "Code, {}, {}",
        INGREDIENT_SHEET_ORDER.join(", "),
        GOLEM_ORDER.join(", ")
    );

    for (code, assignments) in results {
        let golem_vals = match &assignments.golem {
            Some(golem) => golem.values().collect::<Vec<_>>(),
            None => vec!["", ""],
        };

        println!(
            "{}, {}, {}",
            code,
            reordered_ingredient_alchemicals(&assignments.ingredients).join(", "),
            golem_vals.join(", ")
        );
    }
}

fn print_sheet(results: &[(String, CodeAssignments)]) {
    for (code, assignments) in results {
        let mut table = Table::new();

        let mut header_cells = Vec::with_capacity(INGREDIENT_SHEET_ORDER.len() + 1);
        header_cells.push(Cell::from(code));

        for heading in &INGREDIENT_SHEET_ORDER {
            header_cells.push(Cell::from(heading));
        }

        table.add_row(Row::from(header_cells));

        let ingredients = reordered_ingredient_alchemicals(&assignments.ingredients);

        for alchemical in &ALCHEMICALS_SHEET_ORDER {
            let values = table.add_empty_row(); // Vec::with_capacity(INGREDIENT_SHEET_ORDER.len() + 1);

            values.add_cell(cell![*alchemical]);

            for i in 0..ingredients.len() {
                if ingredients[i] == *alchemical {
                    values.add_cell(cell![c->"x"]);
                } else {
                    values.add_cell(cell![""]);
                }
            }
        }

        table.printstd();

        if let Some(golem) = &assignments.golem {
            println!("Golem head: {}, chest: {}", golem.head, golem.chest);
        }

        println!();
    }
}

fn reordered_ingredient_alchemicals(ingredients: &IngredientAssignments) -> [&'static str; 8] {
    [
        ingredients.mushroom,
        ingredients.fern,
        ingredients.toad,
        ingredients.claw,
        ingredients.flower,
        ingredients.root,
        ingredients.scorpion,
        ingredients.feather,
    ]
}
