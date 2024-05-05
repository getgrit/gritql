use elsa::FrozenVec;
use grit_util::{Ast, MatchRanges};
use std::cell::RefCell;
use std::fmt::{self, Debug};
use std::ops;
use std::path::PathBuf;

pub struct FileOwners<Tree: Ast>(FrozenVec<Box<FileOwner<Tree>>>);

impl<Tree: Ast> FileOwners<Tree> {
    pub fn new() -> Self {
        Self(FrozenVec::new())
    }

    pub fn push(&self, file: FileOwner<Tree>) {
        self.0.push(Box::new(file))
    }
}

impl<Tree: Ast> Default for FileOwners<Tree> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Tree: Ast> ops::Deref for FileOwners<Tree> {
    type Target = FrozenVec<Box<FileOwner<Tree>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Tree: Ast> Debug for FileOwners<Tree> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0
            .iter()
            .try_fold((), |_, file| writeln!(f, "{}", file.name.display()))
    }
}

#[derive(Debug, Clone)]
pub struct FileOwner<Tree: Ast> {
    pub absolute_path: PathBuf,
    pub name: PathBuf,
    // todo wrap in Rc<RefCell<Option<>>>
    // so that we can lazily parse
    pub tree: Tree,
    pub matches: RefCell<MatchRanges>,
    pub new: bool,
}

impl<Tree: Ast> PartialEq for FileOwner<Tree> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.tree == other.tree
    }
}
