// use cursive::event::Key;
use cursive::traits::Nameable;
use cursive::views::{Checkbox, Dialog, EditView, ListView};
use cursive::Cursive;

// Wrap all form fields value in one struct
// so we can pass them around easily
struct CatsayOptions<'a> {
    message: &'a str,
    sleep: bool,
}
fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the cat")
            .content(
                ListView::new()
                    .child("Message:", EditView::new().with_name("message"))
                    .child("Sleep?", Checkbox::new().with_name("sleep")),
            )
            .button("OK", |s| {
                let message = s
                    .call_on_name("message", |t: &mut EditView| t.get_content())
                    .unwrap();
                let is_sleep = s
                    .call_on_name("sleep", |t: &mut Checkbox| t.is_checked())
                    .unwrap();
                let options = CatsayOptions {
                    message: &message,
                    sleep: is_sleep,
                };
                result_step(s, &options) // [2]
            }),
    );
}
fn result_step(siv: &mut Cursive, options: &CatsayOptions) {
    let eye = if options.sleep { "x" } else { "o" };
    let cat_text = format!(
        "{msg}\\
\\
/\\_/\\
( {eye} {eye} )
=( I )=",
        msg = options.message,
        eye = eye
    );
    siv.pop_layer(); // [3]
    siv.add_layer(
        // [4]
        Dialog::text(cat_text)
            .title("The cat says...")
            .button("OK", |s| s.quit()),
    );
}
fn main() {
    let mut siv = cursive::default();
    input_step(&mut siv); // [1]
    siv.run();
}
