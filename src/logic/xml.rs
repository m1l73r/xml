use functions::is;
use nodes::{child::Child, node::Node};

use crate::*;

pub struct XmlParseOptions
{
    pub(crate) trim_content: bool,
}
impl XmlParseOptions
{
    pub fn trim_content(&mut self, should_trim: bool) -> &mut Self
    {
        self.trim_content = should_trim;
        self
    }
}
impl Default for XmlParseOptions
{
    fn default() -> Self
    {
        Self { trim_content: true }
    }
}
pub struct Xml
{
    pub xml: String,
}

//Init
impl Xml
{
    pub fn from(xml: String) -> Self
    {
        Self { xml }
    }
    fn next(&self, start_i: usize) -> Option<(usize, usize)>
    {
        let open_i = self.xml[start_i..].find('<')? + start_i;
        let close_i = self.xml[open_i..].find('>')? + open_i;
        Some((open_i, close_i))
    }
    fn is_valid_content(s: &str) -> bool
    {
        !s.contains('<') && !s.trim().is_empty()
    }
    pub fn parse(&self, options: Option<XmlParseOptions>) -> Node
    {
        let options = options.unwrap_or_default();
        let mut last_open_i = 0;
        let mut last_close_i = 0;
        let mut stack = vec![Node::Child(Child::from_inner_xml("document".to_owned()))];
        while let Some((open_i, close_i)) = self.next(last_open_i)
        {
            //let header: String = xml.xml.drain(open_i..close_i + 1).collect();
            let header = self.xml[open_i..close_i + 1].to_string();
            let is_open = is::open(&header);
            let is_close = is::close(&header);
            let node = Node::from_header(header);

            if is_open
            {
                stack.push(node);
            }
            else if is_close
            {
                let mut popped_node = stack.pop().unwrap();
                let content = &self.xml[last_close_i..open_i];
                if Self::is_valid_content(content)
                {
                    if options.trim_content
                    {
                        popped_node.push_content(content.trim().to_string());
                    }
                    else
                    {
                        popped_node.push_content(content.to_string());
                    }
                }

                //popped_node.push_content(content.clone());
                stack.last_mut().unwrap().push(popped_node);
            }
            else
            {
                stack.last_mut().unwrap().push(node);
            }
            last_open_i = open_i + 1;
            last_close_i = close_i + 1;
        }
        assert!(
            stack.len() == 1,
            "Length is {} when it should be 1",
            stack.len()
        );

        stack.pop().unwrap()
    }
}
