pub struct Comment
{
    pub(crate) inner_xml: String,
}
impl Comment
{
    pub fn get_inner_xml(&self) -> &str
    {
        &self.inner_xml
    }
    pub fn get_raw(&self) -> &str
    {
        &self.inner_xml[4..self.inner_xml.len() - 3]
    }
    pub(crate) fn from_inner_xml(inner_xml: String) -> Self
    {
        Self { inner_xml }
    }
}
