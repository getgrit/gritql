use anyhow::{bail, Result};
use grit_util::ByteRange;
use serde_json::json;

/// A source map is used when the code we are parsing is embedded inside a larger file.
/// For example, we want to focus on the Python code inside a Jupyter notebook.
#[derive(Debug, Clone)]
pub struct EmbeddedSourceMap {
    sections: Vec<SourceMapSection>,
    /// This is a bit suboptimal, but we assume nobody has tons of embedded files
    pub(crate) outer_source: String,
}

impl EmbeddedSourceMap {
    pub fn new(outer_source: &str) -> Self {
        Self {
            sections: vec![],
            outer_source: outer_source.to_string(),
        }
    }

    pub fn add_section(&mut self, section: SourceMapSection) {
        self.sections.push(section);
    }

    pub fn clone_with_adjusments<'a>(
        &self,
        adjustments: impl Iterator<Item = &'a (std::ops::Range<usize>, usize)>,
    ) -> Result<EmbeddedSourceMap> {
        let mut new_map = self.clone();

        println!("Cloning and adjusting sections: {:?}", new_map.sections);

        let mut section_iterator = new_map.sections.iter_mut();
        let mut current = match section_iterator.next() {
            Some(section) => section,
            None => return Ok(new_map),
        };

        let mut accumulated_offset: i32 = 0;

        for (source_range, replacement_length) in adjustments {
            println!(
                "Compare {} to {}",
                source_range.start, current.inner_range_end
            );

            // Make sure we are on the right section
            while source_range.start > current.inner_range_end {
                println!(
                    "IT IS TIME TO ADVANCE from {} becuse we hit {} - adding {} to {}",
                    current.inner_range_end,
                    source_range.start,
                    accumulated_offset,
                    current.inner_range_end
                );
                // Apply the accumulated offset to the section
                current.inner_range_end =
                    (current.inner_range_end as i32 + accumulated_offset) as usize;
                current = match section_iterator.next() {
                    Some(section) => section,
                    None => {
                        bail!("Section range is out of bounds")
                    }
                };
            }

            let length_diff =
                *replacement_length as i32 - (source_range.end - source_range.start) as i32;

            // Accumulate the overall offset, which we will use for future sections
            accumulated_offset += length_diff;
        }

        // Apply the accumulated offset to all remaining sections (including the last one we were on)
        current.inner_range_end = (current.inner_range_end as i32 + accumulated_offset) as usize;
        for section in section_iterator {
            println!(
                "IT IS TIME TO ADVANCE from {} becuse we hit the end - adding {}",
                current.inner_range_end, accumulated_offset
            );
            section.inner_range_end =
                (section.inner_range_end as i32 + accumulated_offset) as usize;
        }

        println!(
            "We adjusted by a net of {} into the sections {:?}",
            accumulated_offset, new_map.sections
        );

        Ok(new_map)
    }

    pub fn fill_with_inner(&self, new_inner_source: &str) -> Result<String> {
        let mut outer_source = self.outer_source.clone();

        let mut current_inner_offset = 0;
        let mut current_outer_offset = 0;

        println!(
            "it is fill time!: {}, {:?} {:?}",
            new_inner_source, self.sections, self.outer_source,
        );

        for section in &self.sections {
            // TODO: actually get the *updated* range
            let replacement_code = new_inner_source
                .get(current_inner_offset..(section.inner_range_end - section.inner_end_trim))
                .ok_or(anyhow::anyhow!("Section range is out of bounds"))?;

            let json = section.as_json(replacement_code);

            let outer_range = (section.outer_range.start as i32 + current_outer_offset) as usize
                ..(section.outer_range.end as i32 + current_outer_offset) as usize;

            let length_diff = json.len() as i32 - (outer_range.end - outer_range.start) as i32;
            current_outer_offset += length_diff;
            current_inner_offset = section.inner_range_end + section.inner_end_trim;

            outer_source.replace_range(outer_range, &json);
        }

        Ok(outer_source)
    }
}

#[derive(Debug, Clone)]
pub struct SourceMapSection {
    /// The range of the code within the outer document
    pub(crate) outer_range: ByteRange,
    /// The end of the range from the inner document
    pub(crate) inner_range_end: usize,
    pub(crate) format: SourceValueFormat,
    /// Content we should trim from the inner document before inserting back in (ex. "\n" at the end)
    pub(crate) inner_end_trim: usize,
}

impl SourceMapSection {
    pub fn as_json(&self, code: &str) -> String {
        let structure = match self.format {
            SourceValueFormat::String => serde_json::Value::String(code.to_string()),
            SourceValueFormat::Array => {
                json!(vec![code])
            }
        };
        structure.to_string()
    }

    pub fn new(
        outer_range: ByteRange,
        inner_range_end: usize,
        format: SourceValueFormat,
        inner_end_trim: usize,
    ) -> Self {
        Self {
            outer_range,
            inner_range_end,
            format,
            inner_end_trim,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SourceValueFormat {
    String,
    Array,
}

#[cfg(test)]
mod tests {
    use super::*;
    use grit_util::ByteRange;

    #[test]
    fn test_clone_with_adjustments_single_stage() {
        let mut source_map = EmbeddedSourceMap::new(r#"["abcd", "efgh"]"#);
        source_map.add_section(SourceMapSection::new(
            ByteRange::new(1, 7),
            5,
            SourceValueFormat::String,
            1,
        ));
        source_map.add_section(SourceMapSection::new(
            ByteRange::new(9, 15),
            10,
            SourceValueFormat::String,
            1,
        ));

        // Get the source map with the adjustments
        let code = source_map.fill_with_inner("abcd\nefgh").unwrap();
        assert_eq!(code, r#"["abcd", "efgh"]"#);

        // Add a total of 2 characters to the source map, in the first section
        let adjustments = vec![(1..2, 2), (3..4, 2)];
        let adjusted = source_map
            .clone_with_adjusments(adjustments.iter())
            .unwrap();
        assert_eq!(
            adjusted.fill_with_inner("abbccd\nefgh").unwrap(),
            r#"["abbccd", "efgh"]"#
        );
    }

    #[test]
    fn test_clone_with_adjustments_multi_stage() {
        let mut source_map = EmbeddedSourceMap::new(r#"["abcd", "efgh"]"#);
        source_map.add_section(SourceMapSection::new(
            ByteRange::new(1, 7),
            5,
            SourceValueFormat::String,
            1,
        ));
        source_map.add_section(SourceMapSection::new(
            ByteRange::new(9, 15),
            10,
            SourceValueFormat::String,
            1,
        ));

        // First pass
        // d -> ddd
        // f -> fff
        let adjustments = vec![(4..5, 3), (7..8, 3)];
        let adjusted = source_map
            .clone_with_adjusments(adjustments.iter())
            .unwrap();

        // The first range should end at 

        assert_eq!(
            adjusted.fill_with_inner("abcddd\nefffgh").unwrap(),
            r#"["abcddd", "nefffgh"]"#
        );
    }
}
