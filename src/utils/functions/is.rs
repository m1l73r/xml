
pub fn open(header: &str) -> bool
{
    child(header) && header.starts_with('<') && !header.ends_with("/>") && !header.starts_with("</")
}
pub fn close(header: &str) -> bool
{
    child(header) && header.starts_with("</")
}


pub fn comment(header: &str) -> bool
{
    header.starts_with("<!--") && header.ends_with("-->")
}
pub fn declaration(header: &str) -> bool
{
    header.starts_with("<?") && header.ends_with("?>")
}
pub fn child(header: &str) -> bool
{
    !comment(header) && !declaration(header)
}