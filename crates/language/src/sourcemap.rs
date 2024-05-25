use std::mem;

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
        mut adjustments: impl Iterator<Item = &'a (std::ops::Range<usize>, usize)>,
    ) -> Result<EmbeddedSourceMap> {
        let mut new_map = self.clone();

        println!("Cloning and adjusting sections: {:?}", new_map.sections);

        // let mut section_iterator = new_map.sections.iter_mut();

        let mut accumulated_offset: i32 = 0;
        let mut next_offset = 0;

        for section in new_map.sections.iter_mut() {
            let mut section_offset = mem::take(&mut next_offset);
            for (source_range, replacement_length) in adjustments.by_ref() {
                println!(
                    "Checking section {} with range {:?}",
                    section.inner_range_end, source_range
                );

                let length_diff =
                    *replacement_length as i32 - (source_range.end - source_range.start) as i32;

                // Ok, we hit the end of this section, apply what we have learned so far
                if source_range.start >= section.inner_range_end {
                    // Save this diff, since we will not be able to read it the next time
                    next_offset = length_diff;
                    break;
                }

                // Accumulate the overall offset, which we will use for future sections
                section_offset += length_diff;
            }
            // Apply the accumulated offset to the section
            accumulated_offset += section_offset;
            println!(
                "Adjusting section {} by {}",
                section.inner_range_end, accumulated_offset
            );

            section.inner_range_end =
                (section.inner_range_end as i32 + accumulated_offset) as usize;
        }

        // for (source_range, replacement_length) in adjustments {
        //     // Make sure we are on the right section
        //     while source_range.start > current.inner_range_end {
        //         println!(
        //             "Hit the end of {}, adding {} and looking for next",
        //             current.inner_range_end, accumulated_offset
        //         );
        //         // Apply the accumulated offset to the section
        //         current.inner_range_end =
        //             (current.inner_range_end as i32 + accumulated_offset) as usize;
        //         current = match section_iterator.next() {
        //             Some(section) => section,
        //             None => {
        //                 // We hit the end
        //                 bail!("Section range is out of bounds");
        //                 return Ok(new_map);
        //             }
        //         };
        //     }

        //     let length_diff =
        //         *replacement_length as i32 - (source_range.end - source_range.start) as i32;

        //     // Accumulate the overall offset, which we will use for future sections
        //     accumulated_offset += length_diff;
        // }

        // Apply the accumulated offset to all remaining sections (including the last one we were on)
        // println!(
        //     "ADJUSTING current {} by {}",
        //     current.inner_range_end, accumulated_offset
        // );
        // current.inner_range_end = (current.inner_range_end as i32 + accumulated_offset) as usize;
        // for section in section_iterator {
        //     println!(
        //         "IT IS TIME TO ADVANCE from {} becuse we hit the end - adding {}",
        //         current.inner_range_end, accumulated_offset
        //     );
        //     section.inner_range_end =
        //         (section.inner_range_end as i32 + accumulated_offset) as usize;
        // }

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
            let (start, end) = (
                current_inner_offset,
                section.inner_range_end - section.inner_end_trim,
            );
            println!(
                "Attempt to read range: {}..{} from {}",
                start, end, new_inner_source
            );

            let replacement_code = new_inner_source.get(start..end).ok_or(anyhow::anyhow!(
                "Section range {}-{} is out of bounds",
                start,
                end
            ))?;

            let json = section.as_json(replacement_code);

            let outer_range = (section.outer_range.start as i32 + current_outer_offset) as usize
                ..(section.outer_range.end as i32 + current_outer_offset) as usize;

            let length_diff = json.len() as i32 - (outer_range.end - outer_range.start) as i32;
            current_outer_offset += length_diff;
            current_inner_offset = section.inner_range_end;

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

        let code = source_map.fill_with_inner("abcd\nefgh\n").unwrap();
        assert_eq!(code, r#"["abcd", "efgh"]"#);

        // Add a total of 2 characters to the source map, in the first section
        let adjustments = vec![(1..2, 2), (2..3, 2)];
        let adjusted = source_map
            .clone_with_adjusments(adjustments.iter())
            .unwrap();
        assert_eq!(adjusted.sections[0].inner_range_end, 7);
        assert_eq!(adjusted.sections[1].inner_range_end, 12);
        assert_eq!(
            adjusted.fill_with_inner("abbccd\nefgh\n").unwrap(),
            r#"["abbccd", "efgh"]"#
        );
    }

    #[test]
    fn test_clone_with_adjustments_multi_stage() {
        let mut source_map = EmbeddedSourceMap::new(r#"["abcd", "efgh", "zko"]"#);
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
        source_map.add_section(SourceMapSection::new(
            ByteRange::new(17, 22),
            14,
            SourceValueFormat::String,
            1,
        ));

        // First pass
        // d -> ddd
        // f -> fff
        let adjustments = vec![(3..4, 3), (6..7, 3)];
        let adjusted = source_map
            .clone_with_adjusments(adjustments.iter())
            .unwrap();
        assert_eq!(adjusted.sections[0].inner_range_end, 7);
        assert_eq!(adjusted.sections[1].inner_range_end, 14);
        assert_eq!(adjusted.sections[2].inner_range_end, 18);
        assert_eq!(
            adjusted.fill_with_inner("abcddd|efffgh|zko|").unwrap(),
            r#"["abcddd", "efffgh", "zko"]"#
        );

        // Second pass, we make even more changes
        // a -> deleted
        // ddd -> deleted
        // fff -> f
        let adjustments = vec![(0..1, 0), (3..6, 0), (8..11, 1)];
        let adjusted = adjusted.clone_with_adjusments(adjustments.iter()).unwrap();
        assert_eq!(adjusted.sections[0].inner_range_end, 3);
        assert_eq!(adjusted.sections[1].inner_range_end, 8);
        assert_eq!(adjusted.sections[2].inner_range_end, 12);

        assert_eq!(
            adjusted.fill_with_inner("bc|efgh|zko|").unwrap(),
            r#"["bc", "efgh", "zko"]"#
        );

        // Third pass, we do nothing
        let adjustments = vec![];
        let adjusted = adjusted.clone_with_adjusments(adjustments.iter()).unwrap();
        assert_eq!(
            adjusted.fill_with_inner("bc|efgh|zko|").unwrap(),
            r#"["bc", "efgh", "zko"]"#
        );

        // Third pass, we make even more changes but only in the middle
        // e -> ekg
        let adjustments = vec![(3..4, 3)];
        let adjusted = adjusted.clone_with_adjusments(adjustments.iter()).unwrap();
        assert_eq!(
            adjusted.fill_with_inner("bc|ekgfgh|zko|").unwrap(),
            r#"["bc", "ekgfgh", "zko"]"#
        );
    }
}
