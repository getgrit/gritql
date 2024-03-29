// Based on: https://github.com/skmendez/tree-sitter-traversal/blob/main/src/lib.rs
// License: MIT License
//
// Copyright (c) 2021 Sebastian Mendez

//!
//! Iterators to traverse trees with a [`Cursor`] trait to allow for traversing
//! arbitrary n-ary trees.
//!

use crate::ast_node::AstNode;
use core::iter::FusedIterator;

/// Trait which represents a stateful cursor in a n-ary tree.
/// The cursor can be moved between nodes in the tree by the given methods,
/// and the node which the cursor is currently pointing at can be read as well.
pub trait AstCursor {
    /// The type of the nodes which the cursor points at; the cursor is always pointing
    /// at exactly one of this type.
    type Node: AstNode;

    /// Move this cursor to the first child of its current node.
    ///
    /// This returns `true` if the cursor successfully moved, and returns `false`
    /// if there were no children.
    fn goto_first_child(&mut self) -> bool;

    /// Move this cursor to the parent of its current node.
    ///
    /// This returns `true` if the cursor successfully moved, and returns `false`
    /// if there was no parent node (the cursor was already on the root node).
    fn goto_parent(&mut self) -> bool;

    /// Move this cursor to the next sibling of its current node.
    ///
    /// This returns `true` if the cursor successfully moved, and returns `false`
    /// if there was no next sibling node.
    fn goto_next_sibling(&mut self) -> bool;

    /// Get the node which the cursor is currently pointing at.
    fn node(&self) -> Self::Node;
}

impl<'a, T> AstCursor for &'a mut T
where
    T: AstCursor,
{
    type Node = T::Node;

    fn goto_first_child(&mut self) -> bool {
        T::goto_first_child(self)
    }

    fn goto_parent(&mut self) -> bool {
        T::goto_parent(self)
    }

    fn goto_next_sibling(&mut self) -> bool {
        T::goto_next_sibling(self)
    }

    fn node(&self) -> Self::Node {
        T::node(self)
    }
}

/// Order to iterate through a n-ary tree; for n-ary trees only
/// Pre-order and Post-order make sense.
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Order {
    Pre,
    Post,
}

/// Iterative traversal of the tree; serves as a reference for both
/// PreorderTraversal and PostorderTraversal, as they both will call the exact same
/// cursor methods in the exact same order as this function for a given tree; the order
/// is also the same as traverse_recursive.
#[allow(dead_code)]
fn traverse_iterative<C: AstCursor, F>(mut c: C, order: Order, mut cb: F)
where
    F: FnMut(C::Node),
{
    loop {
        // This is the first time we've encountered the node, so we'll call if preorder
        if order == Order::Pre {
            cb(c.node());
        }

        // Keep travelling down the tree as far as we can
        if c.goto_first_child() {
            continue;
        }

        let node = c.node();

        // If we can't travel any further down, try going to next sibling and repeating
        if c.goto_next_sibling() {
            // If we succeed in going to the previous nodes sibling,
            // we won't be encountering that node again, so we'll call if postorder
            if order == Order::Post {
                cb(node);
            }
            continue;
        }

        // Otherwise, we must travel back up; we'll loop until we reach the root or can
        // go to the next sibling of a node again.
        loop {
            // Since we're retracing back up the tree, this is the last time we'll encounter
            // this node, so we'll call if postorder
            if order == Order::Post {
                cb(c.node());
            }
            if !c.goto_parent() {
                // We have arrived back at the root, so we are done.
                return;
            }

            let node = c.node();

            if c.goto_next_sibling() {
                // If we succeed in going to the previous node's sibling,
                // we will go back to travelling down that sibling's tree, and we also
                // won't be encountering the previous node again, so we'll call if postorder
                if order == Order::Post {
                    cb(node);
                }
                break;
            }
        }
    }
}

/// Idiomatic recursive traversal of the tree; this version is easier to understand
/// conceptually, but the recursion is actually unnecessary and can cause stack overflow.
#[allow(dead_code)]
fn traverse_recursive<C: AstCursor, F>(mut c: C, order: Order, mut cb: F)
where
    F: FnMut(C::Node),
{
    traverse_helper(&mut c, order, &mut cb);
}

fn traverse_helper<C: AstCursor, F>(c: &mut C, order: Order, cb: &mut F)
where
    F: FnMut(C::Node),
{
    // If preorder, call the callback when we first touch the node
    if order == Order::Pre {
        cb(c.node());
    }
    if c.goto_first_child() {
        // If there is a child, recursively call on
        // that child and all its siblings
        loop {
            traverse_helper(c, order, cb);
            if !c.goto_next_sibling() {
                break;
            }
        }
        // Make sure to reset back to the original node;
        // this must always return true, as we only get here if we go to a child
        // of the original node.
        assert!(c.goto_parent());
    }
    // If preorder, call the callback after the recursive calls on child nodes
    if order == Order::Post {
        cb(c.node());
    }
}

struct PreorderTraverse<C> {
    cursor: Option<C>,
}

impl<C> PreorderTraverse<C> {
    pub fn new(c: C) -> Self {
        PreorderTraverse { cursor: Some(c) }
    }
}

impl<C> Iterator for PreorderTraverse<C>
where
    C: AstCursor,
{
    type Item = C::Node;

    fn next(&mut self) -> Option<Self::Item> {
        let c = match self.cursor.as_mut() {
            None => {
                return None;
            }
            Some(c) => c,
        };

        // We will always return the node we were on at the start;
        // the node we traverse to will either be returned on the next iteration,
        // or will be back to the root node, at which point we'll clear out
        // the reference to the cursor
        let node = c.node();

        // First, try to go to a child or a sibling; if either succeed, this will be the
        // first time we touch that node, so it'll be the next starting node
        if c.goto_first_child() || c.goto_next_sibling() {
            return Some(node);
        }

        loop {
            // If we can't go to the parent, then that means we've reached the root, and our
            // iterator will be done in the next iteration
            if !c.goto_parent() {
                self.cursor = None;
                break;
            }

            // If we get to a sibling, then this will be the first time we touch that node,
            // so it'll be the next starting node
            if c.goto_next_sibling() {
                break;
            }
        }

        Some(node)
    }
}

struct PostorderTraverse<C> {
    cursor: Option<C>,
    retracing: bool,
}

impl<C> PostorderTraverse<C> {
    pub fn new(c: C) -> Self {
        PostorderTraverse {
            cursor: Some(c),
            retracing: false,
        }
    }
}

impl<C> Iterator for PostorderTraverse<C>
where
    C: AstCursor,
{
    type Item = C::Node;

    fn next(&mut self) -> Option<Self::Item> {
        let c = match self.cursor.as_mut() {
            None => {
                return None;
            }
            Some(c) => c,
        };

        // For the postorder traversal, we will only return a node when we are travelling back up
        // the tree structure. Therefore, we go all the way to the leaves of the tree immediately,
        // and only when we are retracing do we return elements
        if !self.retracing {
            while c.goto_first_child() {}
        }

        // Much like in preorder traversal, we want to return the node we were previously at.
        // We know this will be the last time we touch this node, as we will either be going
        // to its next sibling or retracing back up the tree
        let node = c.node();
        if c.goto_next_sibling() {
            // If we successfully go to a sibling of this node, we want to go back down
            // the tree on the next iteration
            self.retracing = false;
        } else {
            // If we weren't already retracing, we are now; travel upwards until we can
            // go to the next sibling or reach the root again
            self.retracing = true;
            if !c.goto_parent() {
                // We've reached the root again, and our iteration is done
                self.cursor = None;
            }
        }

        Some(node)
    }
}

// Used for visibility purposes, in case this struct becomes public
struct Traverse<C> {
    inner: TraverseInner<C>,
}

enum TraverseInner<C> {
    Post(PostorderTraverse<C>),
    Pre(PreorderTraverse<C>),
}

impl<C> Traverse<C> {
    pub fn new(c: C, order: Order) -> Self {
        let inner = match order {
            Order::Pre => TraverseInner::Pre(PreorderTraverse::new(c)),
            Order::Post => TraverseInner::Post(PostorderTraverse::new(c)),
        };
        Self { inner }
    }
}

/// Traverse an n-ary tree using `cursor`, returning the nodes of the tree through an iterator
/// in an order according to `order`.
///
/// `cursor` must be at the root of the tree
/// (i.e. `cursor.goto_parent()` must return false)
pub fn traverse<C: AstCursor>(mut cursor: C, order: Order) -> impl FusedIterator<Item = C::Node> {
    assert!(!cursor.goto_parent());
    Traverse::new(cursor, order)
}

impl<C> Iterator for Traverse<C>
where
    C: AstCursor,
{
    type Item = C::Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner {
            TraverseInner::Post(ref mut i) => i.next(),
            TraverseInner::Pre(ref mut i) => i.next(),
        }
    }
}

// We know that PreorderTraverse and PostorderTraverse are fused due to their implementation,
// so we can add this bound for free.
impl<C> FusedIterator for Traverse<C> where C: AstCursor {}
