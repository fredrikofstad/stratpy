use std::fs::OpenOptions;
use std::io::{Error, Write};
use pyo3::{Python};
use crate::tree::game::Game;
use crate::tree::node::Decision;

pub fn write_to_file(game: Game, scale: f32, file_path: &str, py: Python)
    -> Result<(), Error> {
    let latex = generate_latex(game, scale, py);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)?;
    file.write_all(latex.as_ref())?;
    Ok(())
}
pub fn generate_latex(game: Game, scale: f32, py: Python) -> String {
    // seperate style from nodes
    // leave style detailes for users in latex
    // idealy choose level distance based on average player, or action name lengths
    let root_name = game.root.borrow(py).clone().children[0].borrow(py).clone().player.name.clone();
    let mut information_sets= Vec::new().to_owned();
    let node_string = traverse_tree(game.root.borrow(py).clone(), 0, &mut information_sets, true, py);

    let begin = format!(r#"%%       Generated by stratpy       %%
% remember to include \usepackage{{tikz}}
% and \usetikzlibrary{{calc}}
\begin{{figure}}[h]
\begin{{tikzpicture}}[scale={scale}]"#);

    let root = format!("\n% The Tree\n\\node(0)[solid,label=above:{{{root_name}}}]{{}}");

    let end = format!(r#";
\end{{tikzpicture}}
\end{{figure}}"#);


    begin + style() + &*root + &*node_string + &*end
}

fn style() -> &'static str{
    // TODO: auto adjust distancing
    "\n
% Style for nodes
\\tikzstyle{solid}=[circle,draw,inner sep=1.2,fill=black]
\\tikzstyle{hollow}=[circle,draw,inner sep=1.2]\
\n% Spacing for every level of the tree
\\tikzstyle{level 1}=[level distance=10mm,sibling distance=35mm]
\\tikzstyle{level 2}=[level distance=15mm,sibling distance=15mm]"
}

//Traverses tree adding nodes to string and the ids of nodes sharing
// information set to the information_set Vec.

// creating a list style iterative recursive function to traverse the tree
fn traverse_tree(node: Decision, i:usize, information_set: &mut Vec<Vec<(usize, usize)>>, is_root: bool, py: Python) -> String {
    let dir = if i == 0 {"left"} else {"right"};
    let (node_id, edge_label) = (node.id, node.name);
    let (node_type, label) = match node.utility {
        Some(x) => {
            let utility = format!("({}, {})", x[0], x[1]);
            ("hollow", format!("label=below:{{${utility}$}}"))
        },
        None => {
            let player_name = node.children[0].borrow(py).player.name.clone();
            ("solid", format!("label=above {dir}:{{{player_name}}}"))
        }
    };
    let mut child_s = String::new();
    for (i, child) in node.children.iter().enumerate(){
        child_s.push_str(&*traverse_tree(child.borrow(py).clone(), i, information_set, false, py))
    }
    if is_root {
        child_s
    } else {
        let node_s = format!(r#"child{{node({node_id})[{node_type}, {label}]{{}}
    {child_s}edge from parent node[{dir}]{{${edge_label}$}}}}"#);
        node_s
    }
}


fn add_information_set() -> String {
    let start = String::from("% information set
\\draw[dashed,rounded corners=10]($(1) + (-.1,.125)$)rectangle($(2) +(.1,-.125)$);
% specify mover at 2nd information set");
    // for loop for infomation sets
    // need the nodes + player name
    let s = format!("information set");
    s
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_latex_string() {
        //println!("{}", latex_writer())
    }

}

