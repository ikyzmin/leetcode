use std::ops::Deref;

fn main() {
    let mut trie = Trie::new();
    trie.insert(String::from("apple"));
    print!("{}", trie.search(String::from("apple")));
}

struct Trie {
    root: Node,
}

struct Node {
    links: Vec<Option<Box<Node>>>,
    end: bool,
}

impl Node {
    fn new() -> Self {
        return Node { links: Vec::with_capacity(26), end: false };
    }

    fn char_index(ch: char) -> usize {
        (ch as u8 - 'a' as u8) as usize
    }

    fn get(&self, char: char) -> &Option<Box<Node>> {
        &self.links[Node::char_index(char)]
    }

    fn contains(&self, char: char) -> bool {
        if let Some(_) = self.links[Node::char_index(char)] {
            true
        } else {
            false
        }
    }

    fn put(&mut self, char: char, node: Node) {
        self.links[Node::char_index(char)] = Some(Box::new(node));
    }

    fn set_end(&mut self) {
        self.end = true;
    }

    fn end(self) -> bool {
        return self.end;
    }
}

impl Trie {
    fn new() -> Self {
        Trie { root: Node::new() }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for char in word.chars() {
            if let Some(n) = node.get(char).as_mut() {
                n.put(char, Node::new())
            }

            if let Some(value) = node.get(char).as_mut() {
                node = value;
            }
        }
        node.set_end();
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for char in word.chars() {
            if node.contains(char) {
                if let Some(ref i) = node.get(char) {
                    node = i;
                }
            } else {
                return false;
            }
        }
        node.end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for char in prefix.chars() {
            if node.contains(char) {
                if let Some(ref i) = node.get(char) {
                    node = i;
                }
            } else {
                return false;
            }
        }
        true
    }
}