use std::mem::{self, MaybeUninit};




macro_rules! array {
    ($val:expr; $freq:expr; $typ:ty) => {{
        let x = {
            let mut x: [MaybeUninit<$typ>;$freq] = unsafe {
                MaybeUninit::uninit().assume_init()
            };
            for i in 0..$freq {
                x[i] = MaybeUninit::new($val);
            }
            unsafe {
                mem::transmute::<_ , [$typ;$freq]>(x)
            }

        };
        x
    }
    };
}

pub(crate) fn index_from_char(key: char) -> usize {
    assert!(key.is_ascii_alphabetic());
    assert!(key.is_ascii_lowercase());

    let index = key as u8 - 'a' as u8;

    index as usize
}






#[derive(Debug)]
struct TrieNode{
    children: [Option<Box<TrieNode>>;26],
    is_end: bool,
}

#[allow(unused)]
impl TrieNode {
    fn get_node()-> TrieNode {
        Self { 
            children: array!(None; 26; Option<Box<TrieNode>>),
            is_end: false,
        }
    }

    fn boxed_node() -> Option<Box<Self>>{
            Some (
                Box::new(
                    Self::get_node()
                )
            )
    }
}


pub struct Trie {
    root: TrieNode,
}


impl Trie {
    /// Creates a new prefixtrie
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prefix3::Trie;
    /// let trie : Trie = Trie::new();
    /// ```
    pub fn new() -> Trie{
        Self {
            root: TrieNode::get_node()
        }
    }

    /// Inserts a new word into the prefix trie
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prefix3::Trie;
    /// let mut trie : Trie = Trie::new();
    /// trie.insert("word");
    /// 
    /// assert_eq!(trie.search("word"),true);
    /// assert_eq!(trie.search("number"),false);
    /// ```
    pub fn insert(&mut self, word: &str) {
        let mut current_kid = &mut self.root;
        for char in word.chars(){
            let index = index_from_char(char);
            if let None = current_kid.children[index] {
                current_kid.children[index] = TrieNode::boxed_node();
                current_kid = current_kid.children[index].as_mut().unwrap();
            }else {
                current_kid = current_kid.children[index].as_mut().unwrap();
            }
        }
        current_kid.is_end = true;
    }

    /// Returns true if word is in prefixtree else returns false
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prefix3::Trie;
    /// let mut trie : Trie = Trie::new();
    /// trie.insert("word");
    /// 
    /// assert_eq!(trie.search("word"),true);
    /// assert_eq!(trie.search("number"),false);
    /// ```
    pub fn search(&self, word: &str) -> bool {
        let mut current_kid = &self.root;
        for char in word.chars() {
            let index = index_from_char(char);
            if let None = current_kid.children[index]{
                return false;
            }
            current_kid = current_kid.children[index].as_ref().unwrap();
        }
        current_kid.is_end
    }
}







#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let keys =  ["the","a","there","anaswe","any","by","their"];
        let mut prefixtrie = Trie::new();
        keys.iter().for_each(|word| prefixtrie.insert(*word));
        assert!(prefixtrie.search("the")==true);
        assert_eq!(prefixtrie.search("ana"),false);
    }
}
