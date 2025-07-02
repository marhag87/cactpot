mod logic;

use yew::prelude::*;
use logic::NUM_CELLS;
use logic::possible_line_payouts;

const MIN_NUM: u8 = 1;
const MAX_NUM: u8 = 9;

#[derive(PartialEq, Eq, Clone, Copy)]
enum SortBy {
    Avg,
    Max,
}

#[function_component(App)]
fn app() -> Html {
    let numbers = use_state(|| vec![None; NUM_CELLS]);
    let sort_by = use_state(|| SortBy::Max);

    let on_wheel = {
        let numbers = numbers.clone();
        Callback::from(move |(idx, delta): (usize, i32)| {
            let mut nums = (*numbers).clone();
            let current = nums[idx];
            let used: Vec<u8> = nums.iter().filter_map(|&n| n).collect();
            let unused: Vec<u8> = (MIN_NUM..=MAX_NUM).filter(|n| !used.contains(n) || Some(*n) == current).collect();
            if unused.is_empty() {
                return;
            }
            let next = match current {
                Some(n) => {
                    let pos = unused.iter().position(|&x| x == n).unwrap();
                    if delta < 0 {
                        // Scroll up: increment
                        if pos + 1 < unused.len() {
                            Some(unused[pos + 1])
                        } else {
                            Some(n) // At max, do nothing
                        }
                    } else {
                        // Scroll down: decrement
                        if pos > 0 {
                            Some(unused[pos - 1])
                        } else {
                            Some(n) // At min, do nothing
                        }
                    }
                }
                None => {
                    if delta < 0 {
                        // Scroll up: set to lowest unused
                        Some(*unused.iter().min().unwrap())
                    } else {
                        // Scroll down: set to highest unused
                        Some(*unused.iter().max().unwrap())
                    }
                }
            };
            nums[idx] = next;
            numbers.set(nums);
        })
    };

    let on_clear = {
        let numbers = numbers.clone();
        Callback::from(move |idx: usize| {
            let mut nums = (*numbers).clone();
            nums[idx] = None;
            numbers.set(nums);
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

    // Prepare sortable data
    let mut rows: Vec<(usize, String, u32, u32, f64)> = payouts.iter().enumerate().map(|(i, vals)| {
        let avg = if vals.is_empty() { 0.0 } else { vals.iter().map(|&v| v as f64).sum::<f64>() / vals.len() as f64 };
        let max = vals.iter().copied().max().unwrap_or(0);
        let max_count = vals.iter().filter(|&&v| v == max).count();
        let percent = if !vals.is_empty() && max > 0 { (max_count as f64 / vals.len() as f64) * 100.0 } else { 0.0 };
        let line_label = match i {
            0 => "Row 1",
            1 => "Row 2",
            2 => "Row 3",
            3 => "Col 1",
            4 => "Col 2",
            5 => "Col 3",
            6 => "Diag 1",
            7 => "Diag 2",
            _ => "",
        };
        (i, line_label.to_string(), avg.floor() as u32, max, percent)
    }).collect();

    // Always sort descending
    let sort_by_val = *sort_by;
    rows.sort_by(|a, b| {
        let ord = match sort_by_val {
            SortBy::Avg => a.2.cmp(&b.2),
            SortBy::Max => a.3.cmp(&b.3),
        };
        ord.reverse()
    });

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

    let highlight_best = filled_count == 4;
    let best_line_indices = if highlight_best {
        Some(rows[0].0)
    } else {
        None
    };
    let best_line_cells = best_line_indices.map(|idx| logic::LINES[idx]);

    html! {
        <div class={classes!("cactpot-flex")}> 
            <div>
                <div class={classes!("cactpot-grid")}> 
                    { (0..NUM_CELLS).map(|i| {
                        let numbers = numbers.clone();
                        let on_wheel = on_wheel.clone();
                        let on_clear = on_clear.clone();
                        let value = numbers[i];
                        let is_empty = value.is_none();
                        let onwheel = if !max_inputs_reached || !is_empty {
                            let on_wheel = on_wheel.clone();
                            Callback::from(move |e: web_sys::WheelEvent| {
                                e.prevent_default();
                                let delta = e.delta_y() as i32;
                                on_wheel.emit((i, delta));
                            })
                        } else {
                            Callback::from(|e: web_sys::WheelEvent| {
                                e.prevent_default();
                            })
                        };
                        let oncontextmenu = if !max_inputs_reached || !is_empty {
                            let on_clear = on_clear.clone();
                            Callback::from(move |e: web_sys::MouseEvent| {
                                e.prevent_default();
                                on_clear.emit(i);
                            })
                        } else {
                            Callback::from(|e: web_sys::MouseEvent| {
                                e.prevent_default();
                            })
                        };
                        let is_best = best_line_cells.map_or(false, |line| line.contains(&i));
                        let cell_class = if is_best { classes!("cactpot-cell", "cactpot-best-cell") } else { classes!("cactpot-cell") };
                        html! {
                            <div class={cell_class}
                                 tabindex="0"
                                 onwheel={onwheel}
                                 oncontextmenu={oncontextmenu}>
                                { value.map(|n| n.to_string()).unwrap_or_default() }
                            </div>
                        }
                    }).collect::<Html>() }
                </div>
            </div>
            <div class={classes!("cactpot-table-container")}> 
                <table class={classes!("cactpot-payout-table")}> 
                    <colgroup>
                        <col class="line" />
                        <col class="avg" />
                        <col class="max" />
                        <col class="maxpct" />
                    </colgroup>
                    <thead>
                        <tr>
                            <th>{"Line"}</th>
                            <th onclick={sort_by_avg} class={classes!("cactpot-sort-btn")}>
                                {"Avg"}
                            </th>
                            <th onclick={sort_by_max} class={classes!("cactpot-sort-btn")}>
                                {"Max"}
                            </th>
                            <th>{"Max %"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { rows.into_iter().map(|(_i, line_label, avg, max, percent)| {
                            html! {
                                <tr>
                                    <td>{ line_label }</td>
                                    <td>{ avg }</td>
                                    <td>{ max }</td>
                                    <td>{ format!("{:.0}%", percent) }</td>
                                </tr>
                            }
                        }).collect::<Html>() }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
