//! The tag module contains everything needed to perform strongly typed access
//! to tags associated with a message.

use std::ops::Range;
use std::slice::Iter;

/// An implementation of Iterator that iterates over the key/value pairs
/// (in the form of a tuple) of the tags of a `Message`.
#[derive(Clone)]
pub struct TagIter<'a> {
    source: &'a str,
    iter: Iter<'a, (Range<usize>, Option<Range<usize>>)>,
}

impl<'a> TagIter<'a> {
    pub(crate) fn new(
        source: &'a str,
        iter: Iter<'a, (Range<usize>, Option<Range<usize>>)>,
    ) -> TagIter<'a> {
        TagIter { source, iter }
    }
}

impl<'a> Iterator for TagIter<'a> {
    type Item = (&'a str, Option<&'a str>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, value)| {
            (
                &self.source[key.clone()],
                value.clone().map(|value| &self.source[value]),
            )
        })
    }
}

impl<'a> DoubleEndedIterator for TagIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(key, value)| {
            (
                &self.source[key.clone()],
                value.clone().map(|value| &self.source[value]),
            )
        })
    }
}

/// The tag trait is a trait implemented by types for use with the `Message::tag` method.
/// It is used to search for a specified tag and provide stronglyy typed access to it.
pub trait Tag<'a> {
    /// The name of the tag being searched for.
    const NAME: &'static str;

    /// This method attempts to parse the tag input into a strongly typed representation.
    /// If parsing failes, it returns `None`.
    fn parse(tag: Option<&'a str>) -> Option<Self>
    where
        Self: Sized;

    /// A default implementation that searches for a tag with the associated name and
    /// attempts to parse it.
    fn try_match(mut tags: TagIter<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        tags.find(|&(key, _)| key == Self::NAME)
            .and_then(|(_, value)| Self::parse(value))
    }
}
