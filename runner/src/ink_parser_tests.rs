#![cfg(test)]

use crate::ink_lexer::InkToken::{Choice, Dialog, Divert, KnotTitle, Tag, VariableDeclaration};
use crate::ink_lexer::{lex, strip_comments};
use crate::ink_parser;
use crate::ink_parser::{
    lexed_to_parsed, DialogLine, InkStory, Knot, KnotEnd, Line, VariableValue,
};
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

fn default_intro_knot() -> (String, Knot<'static>) {
    ("__INTRO__".to_string(), Default::default())
}

fn empty_story_hashmap() -> BTreeMap<String, Knot<'static>> {
    BTreeMap::from([default_intro_knot()])
}

#[test]
fn full_test() {
    assert_eq!(
        lexed_to_parsed(&vec![VariableDeclaration("cool = \"beans\""),]),
        InkStory {
            global_variables_and_constants: BTreeMap::from([(
                "cool",
                VariableValue::Content("beans".to_string())
            )]),
            knots: empty_story_hashmap(),
            global_tags: vec![]
        }
    );

    assert_eq!(
        lexed_to_parsed(&vec![
            VariableDeclaration("int = 13"),
            VariableDeclaration("float = 6.28"),
            VariableDeclaration("divert = -> london"),
            VariableDeclaration("content = \"words\""),
        ]),
        InkStory {
            global_variables_and_constants: BTreeMap::from([
                ("int", VariableValue::Int(13)),
                ("content", VariableValue::Content("words".to_string())),
                ("divert", VariableValue::Address("london".to_string())),
                ("float", VariableValue::Float(6.28)),
            ]),
            knots: empty_story_hashmap(),
            global_tags: vec![]
        }
    );
    assert_eq!(
        lexed_to_parsed(&vec![Dialog("hi"),]),
        InkStory {
            global_variables_and_constants: Default::default(),
            knots: BTreeMap::from([(
                "__INTRO__".to_string(),
                Knot {
                    lines: vec!["hi".into()],
                    ..Default::default()
                }
            ),]),
            global_tags: vec![]
        }
    );

    assert_eq!(
        lexed_to_parsed(&vec![
            VariableDeclaration("health = 100"),
            VariableDeclaration("pettiness = 100"),
            Dialog("LONDON, 1872"),
            Dialog("Residence of Monsieur Phileas Fogg."),
            Divert("paris_downtown"),
            KnotTitle("paris_downtown"),
            Tag("downtown tag"),
            Dialog("It was cool downtown."),
            Divert("END"),
        ]),
        InkStory {
            global_variables_and_constants: BTreeMap::from([
                ("health", VariableValue::Int(100)),
                ("pettiness", VariableValue::Int(100)),
            ]),
            knots: BTreeMap::from([
                (
                    "__INTRO__".to_string(),
                    Knot {
                        lines: vec![
                            "LONDON, 1872".into(),
                            "Residence of Monsieur Phileas Fogg.".into(),
                        ],
                        end: "paris_downtown".into(),
                        ..Default::default()
                    }
                ),
                (
                    "paris_downtown".to_string(),
                    Knot {
                        title: "paris_downtown".to_string(),
                        lines: vec![Line::Dialog(DialogLine {
                            text: "It was cool downtown.",
                            tags: vec!["downtown tag"]
                        })],
                        knot_tags: vec!["downtown tag"],
                        ..Default::default()
                    }
                )
            ]),
            global_tags: vec![],
        }
    );

    assert_eq!(
        lexed_to_parsed(&vec![Choice("go"), Divert("END")]),
        InkStory {
            global_variables_and_constants: Default::default(),
            knots: BTreeMap::from([(
                "__INTRO__".to_string(),
                Knot {
                    end: KnotEnd::Choices(vec![ink_parser::Choice {
                        text: "go",
                        ..Default::default()
                    }]),
                    ..Default::default()
                }
            ),]),
            global_tags: vec![]
        }
    );

    //assert_eq!(
    //    lexed_to_parsed(&lex(include_str!("../samples/story_with_variables.ink"))),
    //    InkStory {
    //        global_variables_and_constants: Default::default(),
    //        knots: Default::default(),
    //        global_tags: vec![]
    //    }
    //);
}

#[test]
fn parse_choices() {
    assert_eq!(
        lexed_to_parsed(&vec![
            Choice("go"),
            Divert("END"),
            Choice("stay"),
            Divert("END")
        ]),
        InkStory {
            global_variables_and_constants: Default::default(),
            knots: BTreeMap::from([(
                "__INTRO__".to_string(),
                Knot {
                    end: KnotEnd::Choices(vec![
                        ink_parser::Choice {
                            text: "go",
                            ..Default::default()
                        },
                        ink_parser::Choice {
                            text: "stay",
                            ..Default::default()
                        }
                    ]),
                    ..Default::default()
                }
            ),]),
            global_tags: vec![]
        }
    );
}

#[test]
fn parse_london() {
    let lexed = lex(include_str!("../samples/london.ink"));
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed,
        InkStory {
            global_variables_and_constants: Default::default(),
            knots: BTreeMap::from([
                (
                    "__INTRO__".to_string(),
                    Knot {
                        end: KnotEnd::Divert("london".into()),
                        ..Default::default()
                    }
                ),
                (
                    "london".to_string(),
                    Knot {
                        title: "london".to_string(),
                        lines: vec!["Monsieur Phileas Fogg returned home early from the Reform Club, and in a new-fangled steam-carriage, besides!".into(),
                        "health: \"{health}\"".into(),
                        "\"Passepartout,\" said he. \"We are going around the world!\"".into()],
                        end: KnotEnd::Choices(vec![
                            ink_parser::Choice {
                                text: "❤",
                                lines: vec!["I was utterly astonished.".into()],
                                divert: "astonished".into(),
                                ..Default::default()
                            },
                            ink_parser::Choice {
                                text: "🙂",
                                divert: "nod".into(),
                                ..Default::default()
                            },
                        ]),
                        knot_tags: vec![]
                    }
                )
            ]),
            global_tags: vec![]
        }
    );
}

#[test]
fn parse_london2() {
    let lexed = lex(include_str!("../samples/london2.ink"));
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed,
        InkStory {
            global_variables_and_constants: BTreeMap::from([("health", VariableValue::Int(100))]),
            knots: BTreeMap::from([
                (
                    "__INTRO__".to_string(),
                    Knot {
                        title: "__INTRO__".to_string(),
                        lines: vec![
                            "LONDON, 1872".into(),
                            "Residence of Monsieur Phileas Fogg.".into()
                        ],
                        end: KnotEnd::Divert("london".into()),
                        knot_tags: vec![]
                    }
                ),
                (
                    "london".to_string(),
                    Knot {
                        title: "london".to_string(),
                        lines: vec!["in london".into()],
                        end: KnotEnd::Choices(vec![
                            ink_parser::Choice {
                                text: "heart",
                                lines: vec!["huh!?".into()],
                                divert: "astonished".into(),
                                ..Default::default()
                            },
                            ink_parser::Choice {
                                text: "happy",
                                divert: "END".into(),
                                ..Default::default()
                            },
                        ]),
                        knot_tags: vec![]
                    }
                ),
                (
                    "astonished".to_string(),
                    Knot {
                        title: "astonished".to_string(),
                        lines: vec!["wut!".into()],
                        end: KnotEnd::Choices(vec![ink_parser::Choice {
                            text: "sad",
                            ..Default::default()
                        },]),
                        knot_tags: vec![]
                    }
                )
            ]),
            global_tags: vec![]
        }
    );
}

#[test]
fn parse_image_tag() {
    let lexed = lex(include_str!("../samples/image.ink"));
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed,
        InkStory {
            global_variables_and_constants: BTreeMap::new(),
            knots: BTreeMap::from([
                (
                    "__INTRO__".to_string(),
                    Knot {
                        lines: vec![Line::Dialog(DialogLine {
                            text: "Location: The great castle of ooooooom",
                            tags: vec!["img:castle_lowres.jpg"]
                        }),],
                        end: KnotEnd::Divert("space".into()),
                        ..Default::default()
                    }
                ),
                (
                    "space".to_string(),
                    Knot {
                        title: "space".to_string(),
                        lines: vec![Line::Dialog(DialogLine {
                            text: "outer space is great",
                            tags: vec!["img:space.jpg"]
                        }),],
                        end: KnotEnd::Divert("END".into()),
                        knot_tags: vec!["img:space.jpg".into()]
                    }
                )
            ]),
            global_tags: vec![]
        }
    );
}

#[test]
fn parse_global_tags() {
    let lexed = lex("# author: Cool Coolman\n# title: The Gracious Wizard\nThe story begins...");
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed,
        InkStory {
            global_variables_and_constants: BTreeMap::new(),
            knots: BTreeMap::from([(
                "__INTRO__".to_string(),
                Knot {
                    lines: vec!["The story begins...".into()],
                    ..Default::default()
                }
            ),]),
            global_tags: vec!["author: Cool Coolman", "title: The Gracious Wizard"]
        }
    );
}

#[test]
fn parse_sticky_options() {
    let lexed = lex("cool?\n*no->END\n+yes->bears\n==bears\n+yeah->END");
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed,
        InkStory {
            global_variables_and_constants: BTreeMap::new(),
            knots: BTreeMap::from([
                (
                    "__INTRO__".to_string(),
                    Knot {
                        title: "__INTRO__".to_string(),
                        lines: vec!["cool?".into()],
                        end: KnotEnd::Choices(vec![
                            ink_parser::Choice {
                                text: "no",
                                ..Default::default()
                            },
                            ink_parser::Choice {
                                text: "yes",
                                divert: "bears".into(),
                                sticky: true,
                                ..Default::default()
                            }
                        ]),
                        knot_tags: vec![]
                    }
                ),
                (
                    "bears".to_string(),
                    Knot {
                        title: "bears".to_string(),
                        lines: vec![],
                        end: KnotEnd::Choices(vec![ink_parser::Choice {
                            text: "yeah",
                            sticky: true,
                            ..Default::default()
                        }]),
                        knot_tags: vec![]
                    }
                )
            ]),
            global_tags: vec![]
        }
    );
}

#[test]
fn parse_fallbacks() {
    let string = strip_comments(include_str!("../samples/fallbacks.ink"));
    let lexed = lex(&string);
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed,
        InkStory {
            global_variables_and_constants: BTreeMap::new(),
            knots: BTreeMap::from([
                (
                    "__INTRO__".to_string(),
                    Knot {
                        lines: vec!["hey".into()],
                        end: KnotEnd::Divert("fallback".into()),
                        ..Default::default()
                    }
                ),
                (
                    "fallback".to_string(),
                    Knot {
                        title: "fallback".to_string(),
                        lines: vec!["sup".into()],
                        end: KnotEnd::Choices(vec![
                            ink_parser::Choice {
                                text: "wut",
                                divert: "fallback".into(),
                                ..Default::default()
                            },
                            ink_parser::Choice {
                                text: "wutwut",
                                lines: vec!["text".into()],
                                divert: "fallback".into(),
                                ..Default::default()
                            },
                            ink_parser::Choice {
                                lines: vec!["can I put things here?".into()],
                                divert: "fallback2".into(),
                                sticky: true,
                                ..Default::default()
                            }
                        ]),
                        knot_tags: vec![]
                    }
                ),
                (
                    "fallback2".to_string(),
                    Knot {
                        title: "fallback2".to_string(),
                        lines: vec!["sup2".into()],
                        end: KnotEnd::Choices(vec![
                            ink_parser::Choice {
                                text: "wut2",
                                divert: "fallback2".into(),
                                ..Default::default()
                            },
                            ink_parser::Choice {
                                divert: "END".into(),
                                sticky: true,
                                show_text: true,
                                ..Default::default()
                            }
                        ]),
                        knot_tags: vec![]
                    }
                )
            ]),
            global_tags: vec![]
        }
    );
}

#[test]
fn parse_stitches() {
    let string = strip_comments(include_str!("../samples/stitches.ink"));
    let lexed = lex(&string);
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed,
        InkStory {
            global_variables_and_constants: BTreeMap::new(),
            knots: BTreeMap::from([
                (
                    "__INTRO__".to_string(),
                    Knot {
                        end: KnotEnd::Divert("train".into()),
                        ..Default::default()
                    }
                ),
                (
                    "train".to_string(),
                    Knot {
                        title: "train".to_string(),
                        end: KnotEnd::Divert("train.first_class".into()),
                        ..Default::default()
                    }
                ),
                (
                    "train.first_class".to_string(),
                    Knot {
                        title: "train.first_class".to_string(),
                        lines: vec!["first class".into()],
                        end: KnotEnd::Divert("train.missed_train".into()),
                        ..Default::default()
                    }
                ),
                (
                    "train.second_class".to_string(),
                    Knot {
                        title: "train.second_class".to_string(),
                        lines: vec!["second class".into()],
                        end: KnotEnd::Divert("train.missed_train".into()),
                        ..Default::default()
                    }
                ),
                (
                    "train.missed_train".to_string(),
                    Knot {
                        title: "train.missed_train".to_string(),
                        lines: vec!["you missed the train".into()],
                        ..Default::default()
                    }
                )
            ]),
            global_tags: vec![]
        }
    );
}

#[test]
fn parse_stitches_with_choices() {
    let string = strip_comments(include_str!("../samples/stitches_with_choices.ink"));
    let lexed = lex(&string);
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed,
        InkStory {
            global_variables_and_constants: BTreeMap::new(),
            knots: BTreeMap::from([
                (
                    "__INTRO__".to_string(),
                    Knot {
                        end: KnotEnd::Divert("train".into()),
                        ..Default::default()
                    }
                ),
                (
                    "metro".to_string(),
                    Knot {
                        title: "metro".to_string(),
                        lines: vec![],
                        end: KnotEnd::Choices(vec![
                            ink_parser::Choice {
                                text: "cool",
                                ..Default::default()
                            },
                            ink_parser::Choice {
                                text: "uncool",
                                divert: "train.missed_train".into(),
                                ..Default::default()
                            },
                        ]),
                        knot_tags: vec![]
                    }
                ),
                (
                    "train".to_string(),
                    Knot {
                        title: "train".to_string(),
                        end: KnotEnd::Divert("train.first_class".into()),
                        ..Default::default()
                    }
                ),
                (
                    "train.first_class".to_string(),
                    Knot {
                        title: "train.first_class".to_string(),
                        lines: vec!["first class".into()],
                        end: KnotEnd::Choices(vec![
                            ink_parser::Choice {
                                text: "be late",
                                divert: "train.missed_train".into(),
                                ..Default::default()
                            },
                            ink_parser::Choice {
                                text: "be early",
                                divert: "train.second_class".into(),
                                ..Default::default()
                            }
                        ]),
                        knot_tags: vec![]
                    }
                ),
                (
                    "train.second_class".to_string(),
                    Knot {
                        title: "train.second_class".to_string(),
                        lines: vec!["second class".into()],
                        end: KnotEnd::Divert("train.missed_train".into()),
                        ..Default::default()
                    }
                ),
                (
                    "train.missed_train".to_string(),
                    Knot {
                        title: "train.missed_train".to_string(),
                        lines: vec!["you missed the train".into()],
                        end: KnotEnd::Divert("metro".into()),
                        ..Default::default()
                    }
                )
            ]),
            global_tags: vec![]
        }
    );
}

#[test]
fn parse_choices_with_hidden_text() {
    let string = strip_comments(include_str!("../samples/choices.ink"));
    let lexed = lex(&string);
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed,
        InkStory {
            global_variables_and_constants: BTreeMap::new(),
            knots: BTreeMap::from([(
                "__INTRO__".to_string(),
                Knot {
                    title: "__INTRO__".to_string(),
                    lines: vec!["What do you want to say?".into()],
                    end: KnotEnd::Choices(vec![
                        ink_parser::Choice {
                            text: "Hey",
                            lines: vec!["I'm Bruce.".into()],
                            ..Default::default()
                        },
                        ink_parser::Choice {
                            text: "sup",
                            lines: vec!["What is up?".into()],
                            show_text: false,
                            ..Default::default()
                        }
                    ]),
                    knot_tags: vec![]
                }
            ),]),
            global_tags: vec![]
        }
    );
}

#[test]
fn parse_top_level_tag() {
    let string = strip_comments("# tag_is_here\n\n\nstarted\n\n\n-> END");
    let lexed = lex(&string);
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed,
        InkStory {
            global_variables_and_constants: BTreeMap::new(),
            knots: BTreeMap::from([(
                "__INTRO__".to_string(),
                Knot {
                    lines: vec!["started".into()],
                    ..Default::default()
                }
            ),]),
            global_tags: vec!["tag_is_here".into()]
        }
    );
}
