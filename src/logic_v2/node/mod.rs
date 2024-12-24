use child::Child;
use comment::Comment;
use declaration::Declaration;

pub mod declaration;
pub mod child;
pub mod comment;

pub trait NodeType {}

pub enum Node
{
    Child { header: String, inner: Child },
    Comment { header: String, inner: Comment },
    Declaration { header: String, inner: Declaration },
}
impl Node {
    pub fn into_node<T: NodeType>(self) -> T{
        match self {
            Self::Child { inner, .. } => inner,
            Self::Declaration { inner, .. } => inner,
            Self::Comment { inner, .. } => inner,
        }
    }
}