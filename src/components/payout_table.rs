use crate::logic::Board;
use yew::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SortBy {
    Avg,
    Max,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PayoutTableProps {
    pub board_handle: UseStateHandle<Board>,
    pub sort_by: SortBy,
    pub set_sort: Callback<SortBy>,
}

#[function_component(PayoutTable)]
pub fn payout_table(props: &PayoutTableProps) -> Html {
    let board = (*props.board_handle).clone();
    let rows = board.rows(props.sort_by);

    html! {
        <table class={classes!("cactpot-payout-table")}>
            <colgroup>
                <col />
                <col />
                <col />
                <col />
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
                { rows.iter().map(|row| {
                    html! {
                        <tr>
                            <td>{ &row.line_label }</td>
                            <td>{ row.avg }</td>
                            <td>{ row.max }</td>
                            <td>{ format!("{:.0}%", row.percent) }</td>
                        </tr>
                    }
                }).collect::<Html>() }
            </tbody>
        </table>
    }
}
