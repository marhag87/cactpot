use yew::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SortBy {
    Avg,
    Max,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PayoutTableProps {
    pub rows: Vec<(usize, String, u32, u32, f64)>,
    pub sort_by: SortBy,
    pub on_sort_avg: Callback<MouseEvent>,
    pub on_sort_max: Callback<MouseEvent>,
}

#[function_component(PayoutTable)]
pub fn payout_table(props: &PayoutTableProps) -> Html {
    html! {
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
                    <th onclick={props.on_sort_avg.clone()} class={classes!("cactpot-sort-btn")}>
                        {"Avg"}
                        { if props.sort_by == SortBy::Avg { " ↓" } else { "" } }
                    </th>
                    <th onclick={props.on_sort_max.clone()} class={classes!("cactpot-sort-btn")}>
                        {"Max"}
                        { if props.sort_by == SortBy::Max { " ↓" } else { "" } }
                    </th>
                    <th>{"Max %"}</th>
                </tr>
            </thead>
            <tbody>
                { props.rows.iter().map(|(_i, line_label, avg, max, percent)| {
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
    }
}
