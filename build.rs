fn main() {
    slint_build::compile("ui/pegs.slint").unwrap();
    slint_build::compile("ui/appwindow.slint").unwrap();
}
