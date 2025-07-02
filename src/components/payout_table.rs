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
    pub set_sort: Callback<SortBy>,
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
                    <th>
                        {"Line"}
                    </th>
                    <th onclick={props.set_sort.reform(|_| SortBy::Avg)} class={classes!("cactpot-sort-btn")}>
                        {"Avg"}
                    </th>
                    <th onclick={props.set_sort.reform(|_| SortBy::Max)} class={classes!("cactpot-sort-btn")}>
                        {"Max"}
                    </th>
                    <th>
                        {"Max %"}
                    </th>
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
