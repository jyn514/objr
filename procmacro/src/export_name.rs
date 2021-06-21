/// __static_asciz!("LINK_SECTION","EXPORT_NAME_1","EXPORT_NAME_2",IDENT,"ascii")
/// Should expand to something like
///```
/// #[link_section="__TEXT,test_section"]
/// #[export_name="EXPORT_NAME_1EXPORT_NAME_2"]
/// static IDENT: [u8; 6] = *b"ascii\0";
/// ```
pub fn export_name_ascii(link_section:&str, export_name: &str, ident: &str, ascii: &str) -> String {
    format!(
        r#"
        #[link_section="{LINK_SECTION}"]
        #[export_name="{EXPORT_NAME}"]
        static {IDENT}: [u8; {ASCII_LEN}] = *b"{ASCII}\0";
        "#
    ,LINK_SECTION=link_section,EXPORT_NAME=export_name,IDENT=ident,ASCII=ascii,ASCII_LEN=ascii.len() + 1)
}

pub fn export_name_attrs(link_section: &str, export_name_1: &str, export_name_2: &str) -> String {
    format!(
        r#"
            #[link_section="{LINK_SECTION}"]
            #[export_name="{EXPORT_NAME_1}{EXPORT_NAME_2}"]
        "#
        ,LINK_SECTION=link_section,EXPORT_NAME_1=export_name_1, EXPORT_NAME_2=export_name_2
    )
}
pub fn export_name_attrs3(link_section: &str, export_name_1: &str, export_name_2: &str,export_name_3: &str) -> String {
    format!(
        r#"
            #[link_section="{LINK_SECTION}"]
            #[export_name="{EXPORT_NAME_1}{EXPORT_NAME_2}{EXPORT_NAME_3}"]
        "#
        ,LINK_SECTION=link_section,EXPORT_NAME_1=export_name_1, EXPORT_NAME_2=export_name_2,EXPORT_NAME_3=export_name_3
    )
}