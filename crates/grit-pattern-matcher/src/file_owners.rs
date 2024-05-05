use elsa::FrozenVec;
use grit_util::{Ast, MatchRanges};
use std::cell::{Ref, RefCell};
use std::fmt::{self, Debug};
use std::path::PathBuf;
use std::{mem, ops};

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

    // This is a hack for now
    raw_tree: RefCell<Option<Tree>>,

    // Parse the tree the first time we use it
    parsed_tree: RefCell<Option<Tree>>,
    pub matches: RefCell<MatchRanges>,
    pub new: bool,
}

impl<Tree: Ast> FileOwner<Tree> {
    pub fn new(
        name: PathBuf,
        absolute_path: PathBuf,
        raw_tree: Tree,
        matches: MatchRanges,
        new: bool,
    ) -> Self {
        Self {
            name,
            absolute_path,
            raw_tree: RefCell::new(Some(raw_tree)),
            parsed_tree: RefCell::new(None),
            matches: RefCell::new(matches),
            new,
        }
    }

    pub fn tree(&self) -> Ref<Tree> {
        if self.parsed_tree.borrow().is_none() {
            // Take the raw tree out of the RefCell and put it back in
            let raw_tree = self.raw_tree.borrow_mut().take().unwrap();
            *self.parsed_tree.borrow_mut() = Some(raw_tree);
        }
        Ref::map(self.parsed_tree.borrow(), |opt| opt.as_ref().unwrap())
    }
}

impl<Tree: Ast> PartialEq for FileOwner<Tree> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        // && self.tree() == other.tree()
    }
}
