use super::square::Square;
use yew::prelude::{function_component as component, *};

#[component(Board)]
pub fn board() -> Html {
    html!(
        <div>
            <div class="board-row">
                <Square color={"white"} rank={0} file={0} />
                <Square color={"black"} rank={1} file={0} />
                <Square color={"white"} rank={2} file={0} />
                <Square color={"black"} rank={3} file={0} />
                <Square color={"white"} rank={4} file={0} />
                <Square color={"black"} rank={5} file={0} />
                <Square color={"white"} rank={6} file={0} />
                <Square color={"black"} rank={7} file={0} />
            </div>
            <div class="board-row">
                <Square color={"black"} rank={0} file={1} />
                <Square color={"white"} rank={1} file={1} />
                <Square color={"black"} rank={2} file={1} />
                <Square color={"white"} rank={3} file={1} />
                <Square color={"black"} rank={4} file={1} />
                <Square color={"white"} rank={5} file={1} />
                <Square color={"black"} rank={6} file={1} />
                <Square color={"white"} rank={7} file={1} />
            </div>
            <div class="board-row">
                <Square color={"white"} rank={0} file={1} />
                <Square color={"black"} rank={1} file={1} />
                <Square color={"white"} rank={2} file={1} />
                <Square color={"black"} rank={3} file={1} />
                <Square color={"white"} rank={4} file={1} />
                <Square color={"black"} rank={5} file={1} />
                <Square color={"white"} rank={6} file={1} />
                <Square color={"black"} rank={7} file={1} />
            </div>
            <div class="board-row">
                <Square color={"black"} rank={0} file={1} />
                <Square color={"white"} rank={1} file={1} />
                <Square color={"black"} rank={2} file={1} />
                <Square color={"white"} rank={3} file={1} />
                <Square color={"black"} rank={4} file={1} />
                <Square color={"white"} rank={5} file={1} />
                <Square color={"black"} rank={6} file={1} />
                <Square color={"white"} rank={7} file={1} />
            </div>
            <div class="board-row">
                <Square color={"white"} rank={0} file={1} />
                <Square color={"black"} rank={1} file={1} />
                <Square color={"white"} rank={2} file={1} />
                <Square color={"black"} rank={3} file={1} />
                <Square color={"white"} rank={4} file={1} />
                <Square color={"black"} rank={5} file={1} />
                <Square color={"white"} rank={6} file={1} />
                <Square color={"black"} rank={7} file={1} />
            </div>

            <div class="board-row">
                <Square color={"black"} rank={0} file={1} />
                <Square color={"white"} rank={1} file={1} />
                <Square color={"black"} rank={2} file={1} />
                <Square color={"white"} rank={3} file={1} />
                <Square color={"black"} rank={4} file={1} />
                <Square color={"white"} rank={5} file={1} />
                <Square color={"black"} rank={6} file={1} />
                <Square color={"white"} rank={7} file={1} />
            </div>
            <div class="board-row">
                <Square color={"white"} rank={0} file={1} />
                <Square color={"black"} rank={1} file={1} />
                <Square color={"white"} rank={2} file={1} />
                <Square color={"black"} rank={3} file={1} />
                <Square color={"white"} rank={4} file={1} />
                <Square color={"black"} rank={5} file={1} />
                <Square color={"white"} rank={6} file={1} />
                <Square color={"black"} rank={7} file={1} />
            </div>
            <div class="board-row">
                <Square color={"black"} rank={0} file={1} />
                <Square color={"white"} rank={1} file={1} />
                <Square color={"black"} rank={2} file={1} />
                <Square color={"white"} rank={3} file={1} />
                <Square color={"black"} rank={4} file={1} />
                <Square color={"white"} rank={5} file={1} />
                <Square color={"black"} rank={6} file={1} />
                <Square color={"white"} rank={7} file={1} />
            </div>
        </div>

    )
}
