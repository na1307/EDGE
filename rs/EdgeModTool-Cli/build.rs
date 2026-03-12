fn main() {
    embed_resource::compile("src/Resource_exe.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
