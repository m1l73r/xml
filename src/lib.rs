mod utils;
pub use utils::*;
mod logic_v2;
pub use logic_v2::*;

#[cfg(test)]
mod tests {
    use crate::{nodes::node::Node, xml::Xml};

    #[test]
    fn test_another() {}

    #[test]
    fn test() {
        use std::{fs::File, io::Read, path::Path};
        let mut file = File::open(Path::new(r"../assets/styles.xml")).unwrap();
        let mut buffer = String::new();
        _ = file.read_to_string(&mut buffer);

        let node = Xml::from(buffer).parse(None);
        if let Node::Child(child) = node {
            let child = child.walk_for("sz", 0).unwrap();
            let attributes = child.get_attributes().unwrap();
            let attribute = attributes.get_attribute("val").unwrap();
            println!("{attribute}");
        }
    }

    #[test]
    fn generate_dir_tree() {
        //file_tree
        let current_dir = std::env::current_dir().unwrap();
        let mut output_dir = current_dir.clone();
        output_dir.push("docs");

        chart::file_tree::create_dir_tree_file(
            &chart::file_tree::Connectors::default(),
            &Some(chart::ignore!["target", "test", ".git"]),
            &current_dir,
            &output_dir,
        );
    }
}
