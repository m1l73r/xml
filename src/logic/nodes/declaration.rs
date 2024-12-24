use properties::{attributes::Attributes, tag::Tag};

use crate::*;

pub struct Declaration
{
    pub(crate) inner_xml: String,
}
impl Declaration
{
    pub fn get_inner_xml(&self) -> &str
    {
        &self.inner_xml
    }
    pub(crate) fn from_inner_xml(inner_xml: String) -> Self
    {
        Self { inner_xml }
    }
    pub fn get_tag(&self) -> Option<Tag>
    {
        Some(Tag::new(&self.inner_xml[2..]))
    }
    pub fn get_attributes(&self) -> Option<Attributes>
    {
        Attributes::new(&self.inner_xml)
    }
}
