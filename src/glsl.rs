
use std::
{
    path::PathBuf,
    num::NonZeroU32,
    collections::HashMap
};
use super::timeline::*;
use anyhow::bail;

// ------------------------------------------------------------

#[derive(Clone)]
struct GLSLCode(String);

impl AsRef<str> for GLSLCode
{
    fn as_ref(&self) -> &str
    {
        &self.0
    }
}

impl Into<String> for GLSLCode
{
    fn into(self) -> String
    {
        self.0
    }
}

impl GLSLCode
{
    fn new(code: &str) -> anyhow::Result<Self>
    {
        if !code.is_ascii()
        {
            bail!("Non-ASCII characters found")
        }
        Ok(Self(code.to_string()))
    }

    fn strip_comments(&self) -> Self
    {
        let mut stripped = vec!();
        let mut stars = 0u32;
        let mut lines = self.0.lines().peekable();
        'line: while let Some(line) = lines.next()
        {
            let mut windows = line.as_bytes().windows(2).peekable();
            if let None = windows.peek()
            {
                match line.as_bytes()
                {
                    [] => continue,
                    [a] => stripped.push(*a),
                    _ => unreachable!()
                }
            }
            while let Some(window) = windows.next()
            {
                match window
                {
                    [47, 47] if stars == 0 =>
                    {
                        stripped.push(10);
                        continue 'line
                    }
                    [47, 42] =>
                    {
                        windows.nth(0);
                        stars += 1
                    }
                    [42, 47] =>
                    {
                        let next = windows.nth(0);
                        stars = stars.saturating_sub(1);
                        if stars == 0
                        {
                            match next
                            {
                                Some([47, b]) => match windows.peek()
                                {
                                    Some(_) => continue,
                                    None => stripped.push(*b)
                                }
                                None => stripped.push(10),
                                _ => unreachable!()
                            }
                        }
                    }
                    [a, b] if stars == 0 =>
                    {
                        stripped.push(*a);
                        if let None = windows.peek()
                        {
                            stripped.push(*b);
                            if let Some(_) = lines.peek()
                            {
                                stripped.push(10)
                            } 
                        }
                    }
                    _ => {}
                }
            }
        }
        Self(String::from_utf8(stripped).unwrap())
    }
}

// ------------------------------------------------------------

#[test]
fn strip_comments() -> ()
{
    for [input, stripped] in
    [
        ["a", "a"],
        ["abc", "abc"],
        ["abc\n", "abc"], // **
        ["abc\ndef", "abc\ndef"],
        ["abc//def", "abc\n"], // **
        ["abc//def\n", "abc\n"],
        ["abc/*def", "abc"],
        ["abc//def\nghi//jkl", "abc\nghi\n"], // **
        ["abc/*def*/ghi", "abcghi"],
        ["abc/*def\nghi*/jkl", "abcjkl"],
        ["abc/*/*def*/*/ghi", "abcghi"],
        ["abc/*//def*/ghi", "abcghi"],
        ["/**/a", "a"]
    ]
    {
        let code = String::from(input);
        let mut code = GLSLCode::new(&code).unwrap();
        code = code.strip_comments();
        assert_eq!(code.as_ref(), stripped)
    }
}

// ------------------------------------------------------------

pub struct CodeAnnotations
{
    resolution: [u32; 2],
    feedback: bool,
    rate: FPS,
    range: FrameRange,
    texture_paths: HashMap<String, PathBuf>
}

impl Default for CodeAnnotations
{
    fn default() -> Self
    {
        Self
        {
            resolution: [500; 2],
            feedback: Default::default(),
            rate: Default::default(),
            range: Default::default(),
            texture_paths: Default::default()
        }
    }
}

impl CodeAnnotations
{
    pub fn resolution(&self) -> [u32; 2]
    {
        self.resolution
    }

    pub fn feedback(&self) -> bool
    {
        self.feedback
    }

    pub fn rate(&self) -> FPS
    {
        self.rate
    }

    pub fn range(&self) -> FrameRange
    {
        self.range
    }

    pub fn texture_paths(&self) -> &HashMap<String, PathBuf>
    {
        &self.texture_paths
    }
}

// ------------------------------------------------------------

pub struct AnnotatedGLSL
{
    code: String,
    annotations: CodeAnnotations
}

impl AnnotatedGLSL
{
    pub fn new(code: &str) -> anyhow::Result<Self>
    {
        let code = GLSLCode::new(code)?;
        let code: String = code.strip_comments().into();
        let mut annotations = CodeAnnotations::default();
        let mut to_strip = vec!();
        for (index, substring) in code.match_indices("uniform sampler2D ")
        {
            let start = index + substring.len();
            let mut at = start;
            for character in code[start..].chars()
            {
                match character
                {
                    ';' => break,
                    '@' =>
                    {
                        let name = code[start..at].trim();
                        if let Some(mut end) = code[at + 1..].find(";")
                        {
                            end += at;
                            let path = PathBuf::from(code[at + 1..=end].trim());
                            annotations.texture_paths
                                .insert(name.to_string(), path);
                            to_strip.extend(at..=end)
                        }
                    }
                    _ => at += 1
                }
            }
        }
        annotations.feedback = code
            .find("uniform sampler2D previous")
            .is_some();
        for parameter in ["size", "rate", "loop"]
        {
            let prefix = format!("#define {parameter} ");
            let value = code.lines().map(str::trim)
                .filter(|l| l.starts_with(&prefix)).last()
                .map(|l| l[prefix.len()..].trim());
            match (value, parameter)
            {
                (None, "loop") => {},
                (Some(value), "loop" | "rate") => match
                    value.parse::<NonZeroU32>()
                {
                    Ok(value) => match parameter
                    {
                        "loop" => annotations.range =
                            FrameRange::Bounded(value),
                        "rate" => annotations.rate =
                            FPS(value),
                        _ => unreachable!()
                    }
                    Err(error) => bail!
                        (format!("Could not parse '{parameter}' directive: {error}"))
                }
                (Some(value), "size") => match 
                    value.split_ascii_whitespace().map(str::parse::<u32>)
                        .collect::<Result<Vec<u32>, _>>()
                {
                    Ok(values) if values.len() == 2 =>
                        annotations.resolution =
                            values.try_into().unwrap(),
                    Ok(_) => bail!
                        (format!("Expected 2 values for the '{parameter}' directive")),
                    Err(error) => bail!
                        (format!("Could not parse '{parameter}' directive: {error}"))
                }
                (None, "rate") => {}
                (None, "size") => {}
                _ => unreachable!()
            }
        }
        let code = code.into_bytes().iter().enumerate()
            .filter(|(i, _)| !to_strip.contains(i))
            .map(|(_, c)| *c).collect::<Vec<u8>>();
        let code = std::str::from_utf8(&code)
            .unwrap().to_string();
        Ok(Self{code, annotations})
    }

    pub fn code(&self) -> &str
    {
        &self.code
    }

    pub fn annotations(&self) -> &CodeAnnotations
    {
        &self.annotations
    }
}

// ------------------------------------------------------------

// uniform\nsampler2D will not work, which is valid GLSL
#[test]
fn parse() -> ()
{
    let CodeAnnotations
    {
        resolution,
        rate,
        range,
        texture_paths,
        ..
    } = AnnotatedGLSL::new
    (
        "
        ...
        #define size 200 200
        #define size 500 500
        //#define size 800 800
        #define rate 25
        #define loop 100
        // ...
        uniform sampler2D a @ imag/**/es/a.jpg;
        uniform sampler2D b @ b.jpg;
        // uniform sampler2D c @ c.jpg;
        uniform sampler/**/2D d @ d d/**/;
        uniform sampler2D 
            e
            @/**/
            e//

            ;

        "
    ).unwrap().annotations;
    assert_eq!(resolution, [500, 500]);
    assert_eq!(rate.0.get(), 25);
    match range
    {
        FrameRange::Bounded(end)
            => assert_eq!(end.get(), 100),
        _ => panic!()
    }
    assert_eq!(texture_paths.len(), 4);
    for (key, value) in texture_paths
    {
        let value = value.to_str().unwrap();
        match key.as_str()
        {
            "a" => assert_eq!(value, "images/a.jpg"),
            "b" => assert_eq!(value, "b.jpg"),
            "d" => assert_eq!(value, "d d"),
            "e" => assert_eq!(value, "e"),
            _ => unreachable!()
        }
    }
}

