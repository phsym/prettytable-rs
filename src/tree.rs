//! module to make tree-table or tree from a list,
//! by computing the string prefix that contains line of to link item of the list
//! like a tree (multi-root)
//!
//! ```rust
//! use prettytable::tree;
//! use prettytable::{format, row, Table};
//!
//! fn main() {
//!     // Create data
//!     let data = vec![
//!         ("1", 1, "a"),
//!         ("1/2", 2, "b"),
//!         ("1/2/3", 3, "b"),
//!         ("1/4", 4, "c"),
//!         ("5", 5, "z"),
//!         ("5/6", 6, "z"),
//!     ];
//!     // Create the table
//!     let mut table = Table::new();
//!     let format = format::FormatBuilder::new()
//!         .separators(&[], format::LineSeparator::new('-', '+', '+', '+'))
//!         .padding(1, 1)
//!         .build();
//!     table.set_format(format);
//!     table.set_titles(row![bl->"t1", br->"t2"]);
//!     let prefixes = tree::provide_prefix(&data, |parent, item| {
//!         parent.0.split("/").count() + 1 == item.0.split("/").count() && item.0.starts_with(parent.0)
//!     });
//!     for (datum, prefix) in data.iter().zip(prefixes.iter()) {
//!         table.add_row(row![
//!             &format!("{} {}", prefix, datum.1),
//!             r-> &format!("{}", datum.2),
//!         ]);
//!     }
//!
//!     // Print the table to stdout
//!     table.printstd();
//! }
//! ```
//!
//! will print :
//!
//! ```txt
//!  t1        t2
//!   1         a
//!   ├─ 2      b
//!   │  └─ 3   b
//!   └─ 4      c
//!   5         z
//!   └─ 6      z
//! ```

#[derive(Debug, Clone)]
struct TreeNode {
    parent: Option<usize>,
    level: Vec<bool>,
    children: Vec<usize>,
}

fn level_to_string(level: &[bool]) -> String {
    const EMPTY: &str = "   ";
    const EDGE: &str = " └─";
    const PIPE: &str = " │ ";
    const BRANCH: &str = " ├─";

    let mut prefix = String::new();
    if !level.is_empty() {
        let last_col = level.len() - 1;
        for (col, is_last_child) in level.iter().enumerate() {
            let is_last_col = col == last_col;
            let s = match (*is_last_child, is_last_col) {
                (true, false) => EMPTY,
                (true, true) => EDGE,
                (false, false) => PIPE,
                (false, true) => BRANCH,
            };
            prefix.push_str(s);
        }
    }
    prefix
}

fn write_tree_level_of_children(nodes: &mut Vec<TreeNode>, idx: usize) {
    if let Some(node) = nodes.get(idx) {
        let treenode = node.clone();
        let mut d = treenode.children.len();
        for s in treenode.children.iter() {
            if let Some(child) = nodes.get_mut(*s) {
                let mut lnext = treenode.level.clone();
                lnext.push(d == 1);
                d -= 1;
                child.level = lnext;
            }
        }
    }
}

fn make_tree_by_reverse_depth_first<I, F>(items: &[I], is_parent_of: F) -> Vec<TreeNode>
where
    F: Fn(&I, &I) -> bool,
{
    let mut current: Option<usize> = None;
    let mut nodes: Vec<TreeNode> = vec![];
    for (i, item) in items.iter().enumerate() {
        while current.is_some() && !is_parent_of(&items[current.unwrap()], item) {
            current = nodes.get_mut(current.unwrap()).and_then(|n| n.parent);
        }
        let treenode = TreeNode {
            parent: current,
            level: vec![],
            children: vec![],
        };
        if let Some(parent) = current {
            if let Some(node) = nodes.get_mut(parent) {
                node.children.push(i);
            }
        }
        nodes.push(treenode);
        current = Some(i);
    }
    nodes
}

/// Generate a list of prefix to display items as a tree (multi-root)
/// - the input should be sorted in the target display order
/// - the input order of ìtems is preserved into output
/// - the input order is used to ask for parent of item (parent should be before child)
/// - output as the same number of element than input `items`
/// - output can be zipped with input ìtems
/// - is_parent_of(maybe_parent, item)
pub fn provide_prefix<I, F>(items: &[I], is_parent_of: F) -> Vec<String>
where
    F: Fn(&I, &I) -> bool,
{
    let mut nodes: Vec<TreeNode> = make_tree_by_reverse_depth_first(items, is_parent_of);
    //dbg!(&nodes);
    for i in 0..nodes.len() {
        write_tree_level_of_children(&mut nodes, i);
    }
    //dbg!(&nodes);
    nodes.iter().map(|n| level_to_string(&n.level)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {
        let items = vec!["1/2", "1/2/3", "1/2/3/4", "1/2/5", "6", "7", "7/8", "7/9"];

        let prefixes = provide_prefix(&items, |parent, item| {
            let pi = item.split("/");
            let pp = parent.split("/");
            (pi.count() == pp.count() + 1) && item.starts_with(parent)
        });

        let mut actual = String::new();
        prefixes
            .iter()
            .zip(items)
            .for_each(|(p, i)| actual.push_str(&format!("{} {}\n", p, i)));

        let expected = r#" 1/2
 ├─ 1/2/3
 │  └─ 1/2/3/4
 └─ 1/2/5
 6
 7
 ├─ 7/8
 └─ 7/9
"#;
        //dbg!(&actual);
        assert_eq!(actual, expected);
    }
}
