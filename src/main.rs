mod logic;
mod components;

use logic::NUM_CELLS;
use logic::possible_line_payouts;
use yew::prelude::*;
use components::grid::Grid;
use components::payout_table::{PayoutTable, SortBy};
use components::app_logic::{prepare_rows, sort_rows, get_best_line_cells};


#[function_component(App)]
fn app() -> Html {
    let numbers = use_state(|| vec![None; NUM_CELLS]);
    let sort_by = use_state(|| SortBy::Max);

    let on_clear = {
        let numbers = numbers.clone();
        Callback::from(move |idx: usize| {
            let mut nums = (*numbers).clone();
            nums[idx] = None;
            numbers.set(nums);
        })
    };

    let on_reset = {
        let numbers = numbers.clone();
        Callback::from(move |_| {
            numbers.set(vec![None; NUM_CELLS]);
        })
    };

    let filled_count = numbers.iter().filter(|n| n.is_some()).count();
    let max_inputs_reached = filled_count >= 4;

    // Compute payouts for each line
    let payouts = {
        let board: [Option<u8>; NUM_CELLS] = {
            let mut arr = [None; NUM_CELLS];
            for (i, v) in numbers.iter().enumerate() {
                arr[i] = *v;
            }
            arr
        };
        possible_line_payouts(&board)
    };

    // Prepare and sort table rows
    let rows = sort_rows(prepare_rows(&payouts), *sort_by);

    // Handlers for sorting
    let sort_by_avg = {
        let sort_by = sort_by.clone();
        Callback::from(move |_| {
            sort_by.set(SortBy::Avg);
        })
    };
    let sort_by_max = {
        let sort_by = sort_by.clone();
        Callback::from(move |_| {
            sort_by.set(SortBy::Max);
        })
    };

    let best_line_cells = get_best_line_cells(&rows, filled_count);

    html! {
        <div class={classes!("cactpot-vertical-center")}>
            <div class={classes!("cactpot-flex")}>
                <div class={classes!("cactpot-grid-container")}>
                    <Grid
                        set_numbers={numbers.clone()}
                        best_line_cells={best_line_cells}
                        on_clear={on_clear.clone()}
                        max_inputs_reached={max_inputs_reached}
                    />
                </div>
                <div class={classes!("cactpot-table-container")}>
                    <PayoutTable
                        rows={rows.clone()}
                        sort_by={*sort_by}
                        on_sort_avg={sort_by_avg.clone()}
                        on_sort_max={sort_by_max.clone()}
                    />
                </div>
            </div>
            <button onclick={on_reset} class={classes!("cactpot-reset-btn")}>{"Reset"}</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
