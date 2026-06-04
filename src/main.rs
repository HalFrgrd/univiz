use std::io::{self, Read};
use clap::Parser;
use colored::*;
use serde::Serialize;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;
use unicode_names2::name;

/// A command-line tool for analyzing Unicode strings, providing detailed information 
/// about graphemes, code points, and UTF-8 byte sequences.
#[derive(Parser)]
#[command(name = "univiz")]
#[command(about = "Unicode string analyzer - visualize graphemes, code points, and UTF-8 encoding")]
#[command(long_about = "Univiz analyzes Unicode strings and provides detailed information about:
- Grapheme cluster analysis with display width
- Code point information with Unicode values and names  
- UTF-8 byte sequence breakdown with binary representation
- Detailed byte indexing (local and global)")]
#[command(version)]
struct Cli {
    /// The string to analyze (default: 'aé😀👩‍💻')
    text: Option<String>,

    /// Print as JSON instead of bare text
    #[arg(long)]
    json: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AnalysisResult {
    text: String,
    width: usize,
    bytes: usize,
    graphemes: Vec<GraphemeInfo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GraphemeInfo {
    grapheme: String,
    width: usize,
    bytes: usize,
    #[serde(rename = "byteIdx")]
    byte_idx: usize,
    #[serde(rename = "codePoints")]
    code_points: Vec<CodePointInfo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CodePointInfo {
    #[serde(rename = "codePoint")]
    code_point: char,
    unicode: String,
    name: String,
    width: usize,
    #[serde(rename = "byteIdx")]
    byte_idx: usize,
    #[serde(rename = "byteIdxGlobal")]
    byte_idx_global: usize,
    #[serde(rename = "utf8Bytes")]
    utf8_bytes: Vec<Utf8ByteInfo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Utf8ByteInfo {
    idx: usize,
    hex: String,
    binary: String,
}

fn main() {
    let cli = Cli::parse();
    let s = if let Some(text) = cli.text {
        text
    } else {
        let mut buffer = String::new();
        if io::stdin().read_to_string(&mut buffer).is_ok() && !buffer.is_empty() {
            buffer
        } else {
            "aé😀👩‍💻".to_string()
        }
    };

    if cli.json {
        let mut graphemes = Vec::new();
        let mut total_byte_idx = 0;
        for (gi, g) in s.grapheme_indices(true) {
            let mut code_points = Vec::new();
            for (ci, c) in g.char_indices() {
                let mut buffer = [0; 4];
                let bytes = c.encode_utf8(&mut buffer).as_bytes();
                let mut utf8_bytes = Vec::new();
                for (bi, b) in bytes.iter().enumerate() {
                    utf8_bytes.push(Utf8ByteInfo {
                        idx: bi,
                        hex: format!("{:X}", b),
                        binary: format!("{:#b}", b),
                    });
                }
                code_points.push(CodePointInfo {
                    code_point: c,
                    unicode: format!("U+{:X}", c as u32),
                    name: name(c).map(|n| n.to_string()).unwrap_or("<unknown>".to_string()),
                    width: c.width().unwrap_or(0),
                    byte_idx: ci,
                    byte_idx_global: total_byte_idx + ci,
                    utf8_bytes: utf8_bytes,
                });
            }
            graphemes.push(GraphemeInfo {
                grapheme: g.to_string(),
                width: g.width(),
                bytes: g.len(),
                byte_idx: gi,
                code_points: code_points,
            });
            total_byte_idx += g.len();
        }
        let result = AnalysisResult {
            text: s.clone(),
            width: s.width(),
            bytes: s.len(),
            graphemes,
        };
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        println!(
            "Analyzing string: '{}' {} {}",
            s,
            format!("width={}", s.width()).dimmed(),
            format!("bytes={}", s.len()).dimmed()
        );
        let mut total_byte_idx = 0;
        for (gi, g) in s.grapheme_indices(true) {
            println!(
                "grapheme='{}' {} {} {}",
                g,
                format!("width={}", g.width()).dimmed(),
                format!("bytes={}", g.len()).dimmed(),
                format!("byteIdx={}", gi).dimmed()
            );
            for (ci, c) in g.char_indices() {

                let char_name = name(c).map(|n| n.to_string()).unwrap_or("<unknown>".to_string());
                println!(
                    "  codePoint='{}' (U+{:X}) ({}) {} {} {}",
                    c,
                    c as u32,
                    char_name,
                    format!("width={}", c.width().unwrap_or(0)).dimmed(),
                    format!("byteIdx={}", ci).dimmed(),
                    format!("byteIdxGlobal={}", total_byte_idx + ci).dimmed()
                );

                let mut buffer = [0; 4];
                let bytes = c.encode_utf8(&mut buffer).as_bytes();
                for (bi, b) in bytes.iter().enumerate() {
                    let binary = format!("{:#b}", b);
                    let prefix_to_color = match bi {
                        0 => match bytes.len() {
                            1 => 3,
                            2 => 5,
                            3 => 6,
                            4 => 7,
                            _ => 0,
                        },
                        _ => 4,
                    };

                    // Split string at char boundaries
                    let colored: String = binary
                        .chars()
                        .enumerate()
                        .map(|(i, c)| {
                            if i < prefix_to_color {
                                c.to_string().red().to_string()
                            } else {
                                c.to_string()
                            }
                        })
                        .collect();

                    let idx_string = format!("({})", bi).dimmed();
                    println!("    utf8Byte {}: {:X} {}", idx_string, *b, colored);
                    assert_eq!(s.as_bytes()[total_byte_idx + ci + bi], *b);
                }
            }
            total_byte_idx += g.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case_labels() {
        // This is a manual verification check essentially,
        // but we can at least check that the structs serialize to camelCase.
        let byte_info = Utf8ByteInfo {
            idx: 0,
            hex: "41".to_string(),
            binary: "0b1000001".to_string(),
        };
        let json = serde_json::to_string(&byte_info).unwrap();
        assert!(json.contains("\"idx\":0"));

        let cp_info = CodePointInfo {
            code_point: 'A',
            unicode: "U+41".to_string(),
            name: "LATIN CAPITAL LETTER A".to_string(),
            width: 1,
            byte_idx: 0,
            byte_idx_global: 0,
            utf8_bytes: vec![byte_info],
        };
        let json = serde_json::to_string(&cp_info).unwrap();
        assert!(json.contains("\"codePoint\":\"A\""));
        assert!(json.contains("\"byteIdx\":0"));
        assert!(json.contains("\"byteIdxGlobal\":0"));
        assert!(json.contains("\"utf8Bytes\":["));
    }
}
