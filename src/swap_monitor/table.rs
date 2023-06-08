use crate::swap_monitor::token_pair::TokenPair;
use comfy_table::*;

pub fn generate_table(token_pairs: &Vec<TokenPair>) -> Table {
    let mut table = Table::new();
    table
        .set_header(vec!["Swap Number", "Token Pair", "Pair Address"])
        .set_content_arrangement(ContentArrangement::DynamicFullWidth);

    for token_pair in token_pairs.iter().take(25) {
        let pair_tokens = format!(
            "{} - {}",
            &token_pair.token_0.symbol, &token_pair.token_1.symbol
        );

        table.add_row(vec![
            token_pair.num_updates.to_string(),
            pair_tokens,
            token_pair.address.clone(),
        ]);
    }

    if let Some(column) = table.column_mut(1) {
        column.set_constraint(ColumnConstraint::UpperBoundary(Width::Fixed(50)));
    }

    table
}
