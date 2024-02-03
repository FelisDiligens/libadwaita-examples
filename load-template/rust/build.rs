fn main() {
    glib_build_tools::compile_resources(
        &["src"],
        "src/gresources.xml",
        "gresources.gresource",
    );
}