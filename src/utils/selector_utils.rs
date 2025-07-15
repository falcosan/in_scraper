use scraper::Selector;

pub enum SelectorInput<'a> {
    Single(&'a str),
    Multiple(&'a [&'a str]),
}

impl<'a> From<&'a str> for SelectorInput<'a> {
    fn from(selector_str: &'a str) -> Self {
        SelectorInput::Single(selector_str)
    }
}

impl<'a> From<&'a [&'a str]> for SelectorInput<'a> {
    fn from(selector_strs: &'a [&'a str]) -> Self {
        SelectorInput::Multiple(selector_strs)
    }
}

pub fn parse_selector<'a, T>(input: T) -> Selector where T: Into<SelectorInput<'a>> {
    let selector_input = input.into();
    match selector_input {
        SelectorInput::Single(selector_str) => {
            Selector::parse(selector_str).unwrap_or_else(|err| {
                panic!("Failed to parse selector '{selector_str}': {err:?}");
            })
        }
        SelectorInput::Multiple(selector_strs) => {
            for selector_str in selector_strs {
                if let Ok(selector) = Selector::parse(selector_str) {
                    return selector;
                }
            }
            panic!("Failed to parse any of the provided selectors: {selector_strs:?}");
        }
    }
}
