use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Heading {
    ATX(ATXHeading),
    SETEXT(SetextHeading),
}
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ATXHeading {
    pub level: HeadingLevel,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct SetextHeading {
    pub level: HeadingLevel,
}

impl TryFrom<usize> for HeadingLevel {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(HeadingLevel::H1),
            2 => Ok(HeadingLevel::H2),
            3 => Ok(HeadingLevel::H3),
            4 => Ok(HeadingLevel::H4),
            5 => Ok(HeadingLevel::H5),
            6 => Ok(HeadingLevel::H6),
            _ => Err(format!(
                "Not parsed heading level, value \"{}\" is not standard heading level",
                value
            )),
        }
    }
}

impl Heading {
    pub fn level(&self) -> &HeadingLevel {
        match self {
            Heading::ATX(atx) => &atx.level,
            Heading::SETEXT(heading) => &heading.level,
        }
    }
}
