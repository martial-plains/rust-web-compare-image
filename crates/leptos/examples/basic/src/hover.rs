use leptos::*;
use leptos_compare_image::LeptosCompareImage;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    let img1_src = "images/image1.png";
    let img2_src = "images/image2.png";

    mount_to_body(move || {
        view! {
            <div style="max-width: 640px;">
                <LeptosCompareImage left_image=img1_src right_image=img2_src hover=true />
            </div>
        }
    })
}
