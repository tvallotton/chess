use dioxus::prelude::*;
use dioxus_html_macro::html;

mod x_button;


pub const Parameters: Component = |ref s| {

    

    s.render(html!(
        <button class="btn btn-primary" r#type="button" "data-bs-toggle"="offcanvas" "data-bs-target"="#offcanvasRight"
            aria_controls="offcanvasRight">"Toggle right offcanvas"</button>

        <div class="offcanvas offcanvas-end" tabindex="-1" id="offcanvasRight" aria_labelledby="offcanvasRightLabel">
            <div class="offcanvas-header">
                <h5 id="offcanvasRightLabel">"Offcanvas right"</h5>
                <button r#type="button" class="btn-close text-reset" "data-bs-dismiss"="offcanvas" aria_label="Close"></button>
            </div>
            <div class="offcanvas-body">
                "..."
            </div>
        </div>
    ))
};
