use nodes::node::Node;
use properties::{attributes::Attributes, children::Children, content::Content, tag::Tag};

use crate::*;

pub struct Child
{
    pub(crate) inner_xml: String,
    pub(crate) children: Children,
    pub(crate) content: Option<Content>,
}
impl Child
{
    pub fn get_inner_xml(&self) -> &str
    {
        &self.inner_xml
    }
    pub fn get_content(&self) -> &Option<Content>{
        &self.content
    }
    pub(crate) fn from_inner_xml(inner_xml: String) -> Self
    {
        Self {
            inner_xml,
            children: Children::default(),
            content: None
        }
    }
    pub fn get_children(&self) -> &Children
    {
        &self.children
    }
    pub fn get_tag(&self) -> Tag
    {
        let cut_off = self.inner_xml.len() - if self.inner_xml.ends_with("/>") { 2 } else { 1 };
        let ref_inner_xml = &self.inner_xml[1..cut_off];
        Tag::new(ref_inner_xml)
    }
    pub fn get_attributes(&self) -> Option<Attributes>
    {
        let cut_off = self.inner_xml.len() - if self.inner_xml.ends_with("/>") { 2 } else { 1 };
        let ref_inner_xml = &self.inner_xml[1..cut_off];
        Attributes::new(ref_inner_xml)
    }
    pub(crate) fn push(&mut self, child: Node)
    {
        self.children.push(child);
    }
    pub fn walk_for(&self, identifier: &str, index: usize) -> Option<&Child>
    {
        //This checks if the current child, hold the child we try to find, and then it returns it
        if let Some(Node::Child(ref child)) = self.children.get_child(identifier, index)
        {
            return Some(child);
        }
        
        //This finds the child, if its in the child tree
        for child_nodes in self.children.values()
        {
            for node in child_nodes
            {
                if let Node::Child(ref child) = node
                {
                    if let Some(result) = child.walk_for(identifier, index)
                    {
                        return Some(result);
                    }
                }
            }
        }

        None
    }
}
