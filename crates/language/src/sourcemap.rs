use anyhow::Result;
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

    pub fn new_section(
        &mut self,
        outer_range: std::ops::Range<usize>,
        inner_range_end: usize,
        format: SourceValueFormat,
        inner_end_trim: usize,
    ) {
        self.sections.push(SourceMapSection {
            outer_range: ByteRange::new(outer_range.start, outer_range.end),
            inner_range_end,
            format,
            inner_end_trim,
        })
    }

    pub fn clone_with_edits<'a>(
        &self,
        adjustments: impl Iterator<Item = &'a (std::ops::Range<usize>, usize)>,
    ) -> Result<EmbeddedSourceMap> {
        let mut new_map = self.clone();

        println!("New adjustment cycle!");

        let mut section_iter = new_map.sections.iter().enumerate().peekable();

        let mut section_adjustments: Vec<i64> = vec![0; new_map.sections.len()];

        for (source_range, replacement_length) in adjustments {
            // Find the section that contains the source range
            while let Some((index, section)) = section_iter.peek() {
                // If the section contains the source range, apply the adjustment
                if section.inner_range_end > source_range.start {
                    let length_diff =
                        *replacement_length as i64 - (source_range.end - source_range.start) as i64;
                    println!("Adjusting {:?} with {}", source_range, length_diff);

                    section_adjustments[*index] += length_diff;
                    break;
                }
                // Otherwise, move to the next section
                section_iter.next();
            }
        }

        // Apply the accumulated offset to the section
        let mut accumulated_offset = 0;
        for (section, adjustment) in new_map.sections.iter_mut().zip(section_adjustments) {
            accumulated_offset += adjustment;
            println!("Adding {} to section {:?}", accumulated_offset, section);
            section.inner_range_end =
                (section.inner_range_end as i64 + accumulated_offset) as usize;
        }

        Ok(new_map)
    }

    pub fn fill_with_inner(&self, new_inner_source: &str) -> Result<String> {
        let mut outer_source = self.outer_source.clone();

        println!("inner output: {}", new_inner_source);

        let mut current_inner_offset = 0;
        let mut current_outer_offset = 0;

        for section in &self.sections {
            let (start, end) = (
                current_inner_offset,
                section.inner_range_end - section.inner_end_trim,
            );

            let replacement_code = new_inner_source.get(start..end).ok_or(anyhow::anyhow!(
                "Section range {}-{} is out of bounds inside {}",
                start,
                end,
                new_inner_source.len()
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
        let adjusted = source_map.clone_with_edits(adjustments.iter()).unwrap();
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
        let adjusted = source_map.clone_with_edits(adjustments.iter()).unwrap();
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
        let adjusted = adjusted.clone_with_edits(adjustments.iter()).unwrap();
        assert_eq!(adjusted.sections[0].inner_range_end, 3);
        assert_eq!(adjusted.sections[1].inner_range_end, 8);
        assert_eq!(adjusted.sections[2].inner_range_end, 12);

        assert_eq!(
            adjusted.fill_with_inner("bc|efgh|zko|").unwrap(),
            r#"["bc", "efgh", "zko"]"#
        );

        // Third pass, we do nothing
        let adjustments = vec![];
        let adjusted = adjusted.clone_with_edits(adjustments.iter()).unwrap();
        assert_eq!(
            adjusted.fill_with_inner("bc|efgh|zko|").unwrap(),
            r#"["bc", "efgh", "zko"]"#
        );

        // Final pass, we make even more changes but only in the middle
        // e -> ekg
        let adjustments = vec![(3..4, 3)];
        let adjusted = adjusted.clone_with_edits(adjustments.iter()).unwrap();
        assert_eq!(
            adjusted.fill_with_inner("bc|ekgfgh|zko|").unwrap(),
            r#"["bc", "ekgfgh", "zko"]"#
        );
    }

    #[test]
    fn test_five_sections_with_single_edit() {
        let mut source_map = EmbeddedSourceMap::new(r#"["abcd", "efgh", "zko", "znzo"]"#);
        source_map.new_section(1..7, 5, SourceValueFormat::String, 1);
        source_map.new_section(9..15, 10, SourceValueFormat::String, 1);
        source_map.new_section(17..22, 14, SourceValueFormat::String, 1);
        source_map.new_section(24..30, 19, SourceValueFormat::String, 1);

        // Verify the initial state
        assert_eq!(
            source_map.fill_with_inner("abcd|efgh|zko|znzo|").unwrap(),
            r#"["abcd", "efgh", "zko", "znzo"]"#
        );

        // First pass, only edit the 3rd section (adding 2 characters)
        // k -> PP
        let adjustments = vec![(11..12, 2)];
        let adjusted = source_map.clone_with_edits(adjustments.iter()).unwrap();
        assert_eq!(
            adjusted.fill_with_inner("abcd|efgh|zPPo|znzo|").unwrap(),
            r#"["abcd", "efgh", "zPPo", "znzo"]"#
        );
    }
}
