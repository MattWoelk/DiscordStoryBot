#![cfg(test)]

use crate::ink_lexer;
use crate::ink_lexer::{lex, strip_comments, InkToken};
use crate::ink_parser::KnotEnd::Choices;
use crate::ink_parser::Line::Dialog;
use crate::ink_parser::VariableValue::Float;
use crate::ink_parser::{
    get_author_from_tag, get_title_from_tag, lexed_to_parsed, Choice, DialogLine, Expression,
    InkStory, Knot, KnotEnd, Line, VariableValue,
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
        lexed_to_parsed(&vec![InkToken::VariableDeclaration("cool = \"beans\""),]),
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
            InkToken::VariableDeclaration("int = 13"),
            InkToken::VariableDeclaration("float = 6.28"),
            InkToken::VariableDeclaration("divert = -> london"),
            InkToken::VariableDeclaration("content = \"words\""),
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
        lexed_to_parsed(&vec![InkToken::Dialog(("hi", true)),]),
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
            InkToken::VariableDeclaration("health = 100"),
            InkToken::VariableDeclaration("pettiness = 100"),
            InkToken::Dialog(("LONDON, 1872", true)),
            InkToken::Dialog(("Residence of Monsieur Phileas Fogg.", true)),
            InkToken::Divert("paris_downtown"),
            InkToken::KnotTitle("paris_downtown"),
            InkToken::Tag("downtown tag"),
            InkToken::Dialog(("It was cool downtown.", true)),
            InkToken::Divert("END"),
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
                            has_newline: true,
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
        lexed_to_parsed(&vec![
            InkToken::Choice(("go", true)),
            InkToken::Divert("END")
        ]),
        InkStory {
            global_variables_and_constants: Default::default(),
            knots: BTreeMap::from([(
                "__INTRO__".to_string(),
                Knot {
                    end: KnotEnd::Choices(vec![Choice {
                        choice_text: "go".to_string(),
                        shown_text: "go".to_string(),
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
            ink_lexer::InkToken::Choice(("go", true)),
            InkToken::Divert("END"),
            ink_lexer::InkToken::Choice(("stay", true)),
            InkToken::Divert("END")
        ]),
        InkStory {
            global_variables_and_constants: Default::default(),
            knots: BTreeMap::from([(
                "__INTRO__".to_string(),
                Knot {
                    end: KnotEnd::Choices(vec![
                        Choice {
                            choice_text: "go".to_string(),
                            shown_text: "go".to_string(),
                            ..Default::default()
                        },
                        Choice {
                            choice_text: "stay".to_string(),
                            shown_text: "stay".to_string(),
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
                            Choice {
                                choice_text: "❤".to_string(),
                                shown_text: "❤".to_string(),
                                lines: vec!["I was utterly astonished.".into()],
                                divert: "astonished".into(),
                                ..Default::default()
                            },
                            Choice {
                                choice_text: "🙂".to_string(),
                                shown_text: "🙂".to_string(),
                                divert: "nod".into(),
                                has_newline: false,
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
                            Choice {
                                choice_text: "heart".to_string(),
                                shown_text: "heart".to_string(),
                                lines: vec!["huh!?".into()],
                                divert: "astonished".into(),
                                ..Default::default()
                            },
                            Choice {
                                choice_text: "happy".to_string(),
                                shown_text: "happy".to_string(),
                                has_newline: false,
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
                        end: KnotEnd::Choices(vec![Choice {
                            choice_text: "sad".to_string(),
                            shown_text: "sad".to_string(),
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
                            has_newline: true,
                            tags: vec!["img:castle_lowres.jpg"]
                        }),],
                        end: KnotEnd::Divert("space".into()),
                        knot_tags: vec!["img:castle_lowres.jpg".into()],
                        ..Default::default()
                    }
                ),
                (
                    "space".to_string(),
                    Knot {
                        title: "space".to_string(),
                        lines: vec![Line::Dialog(DialogLine {
                            text: "outer space is great",
                            has_newline: true,
                            tags: vec!["img:space.jpg"]
                        }),],
                        end: KnotEnd::Divert("END".into()),
                        knot_tags: vec!["img:space.jpg".into()]
                    }
                )
            ]),
            global_tags: vec!["img:castle_lowres.jpg"]
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
                    lines: vec![Line::Dialog(DialogLine {
                        text: "The story begins...",
                        has_newline: true,
                        tags: vec!["author: Cool Coolman", "title: The Gracious Wizard"]
                    })],
                    knot_tags: vec!["author: Cool Coolman", "title: The Gracious Wizard"],
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
                            Choice {
                                choice_text: "no".to_string(),
                                shown_text: "no".to_string(),
                                has_newline: false,
                                ..Default::default()
                            },
                            Choice {
                                choice_text: "yes".to_string(),
                                shown_text: "yes".to_string(),
                                divert: "bears".into(),
                                has_newline: false,
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
                        end: KnotEnd::Choices(vec![Choice {
                            choice_text: "yeah".to_string(),
                            shown_text: "yeah".to_string(),
                            has_newline: false,
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
                        lines: vec![Line::Dialog(DialogLine {
                            text: "hey",
                            has_newline: false,
                            tags: vec![]
                        })],
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
                            Choice {
                                choice_text: "wut".to_string(),
                                shown_text: "wut".to_string(),
                                has_newline: false,
                                divert: "fallback".into(),
                                ..Default::default()
                            },
                            Choice {
                                choice_text: "wutwut".to_string(),
                                shown_text: "wutwut".to_string(),
                                has_newline: true,
                                lines: vec!["text".into()],
                                divert: "fallback".into(),
                                ..Default::default()
                            },
                            Choice {
                                lines: vec!["can I put things here?".into()],
                                divert: "fallback2".into(),
                                has_newline: true,
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
                            Choice {
                                choice_text: "wut2".to_string(),
                                shown_text: "wut2".to_string(),
                                has_newline: false,
                                divert: "fallback2".into(),
                                ..Default::default()
                            },
                            Choice {
                                divert: "END".into(),
                                has_newline: false,
                                sticky: true,
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
                            Choice {
                                choice_text: "cool".to_string(),
                                shown_text: "cool".to_string(),
                                ..Default::default()
                            },
                            Choice {
                                choice_text: "uncool".to_string(),
                                shown_text: "uncool".to_string(),
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
                            Choice {
                                choice_text: "be late".to_string(),
                                shown_text: "be late".to_string(),
                                divert: "train.missed_train".into(),
                                ..Default::default()
                            },
                            Choice {
                                choice_text: "be early".to_string(),
                                shown_text: "be early".to_string(),
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
                        Choice {
                            choice_text: "Hey".to_string(),
                            shown_text: "Hey".to_string(),
                            lines: vec!["I'm Bruce.".into()],
                            ..Default::default()
                        },
                        Choice {
                            choice_text: "sup".to_string(),
                            shown_text: "".to_string(),
                            lines: vec!["What is up?".into()],
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
                    lines: vec![Line::Dialog(DialogLine {
                        text: "started",
                        has_newline: true,
                        tags: vec!["tag_is_here"]
                    })],
                    knot_tags: vec!["tag_is_here"],
                    ..Default::default()
                }
            ),]),
            global_tags: vec!["tag_is_here".into()]
        }
    );
}

#[test]
fn parse_author() {
    assert_eq!(
        get_author_from_tag("author: author name goes here"),
        Some("author name goes here".to_string())
    );

    let string = strip_comments(include_str!("../samples/author.ink"));
    let lexed = lex(&string);
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed.get_author(),
        Some("author name goes here".to_string())
    );
}

#[test]
fn parse_title() {
    assert_eq!(
        get_title_from_tag("title: title goes here"),
        Some("title goes here".to_string())
    );

    let string = strip_comments(include_str!("../samples/author.ink"));
    let lexed = lex(&string);
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed.get_title(),
        Some("The Title of the Story".to_string())
    );
}

#[test]
fn parse_choices_with_hidden_choice_text() {
    let string = strip_comments(include_str!("../samples/choices_with_hidden_text.ink"));
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
                        Choice {
                            choice_text: "\"Hey\"".to_string(),
                            shown_text: "\"Sup, my dude?\"".to_string(),
                            lines: vec!["He stared at me in disbelief.".into()],
                            ..Default::default()
                        },
                        Choice {
                            choice_text: "\"Why?\"".to_string(),
                            shown_text: "\"Why not!\"".to_string(),
                            lines: vec!["So we left, right there.".into()],
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
fn parse_choices_with_conditionals() {
    let string = strip_comments(include_str!("../samples/choices_with_conditionals.ink"));
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
                        title: "__INTRO__".to_string(),
                        lines: vec![],
                        end: KnotEnd::Divert("start".into()),
                        knot_tags: vec!["hidden"]
                    }
                ),
                (
                    "start".to_string(),
                    Knot {
                        title: "start".to_string(),
                        lines: vec![],
                        end: KnotEnd::Choices(vec![
                            Choice {
                                conditionals: vec![Expression::Not(Box::new("zoo".into()))],
                                choice_text: "to the zoo!".to_string(),
                                shown_text: "to the zoo!".to_string(),
                                has_newline: false,
                                lines: vec![],
                                divert: "zoo".into(),
                                sticky: true
                            },
                            Choice {
                                conditionals: vec!["zoo".into()],
                                choice_text: "complain about the zoo".to_string(),
                                shown_text: "complain about the zoo".to_string(),
                                has_newline: false,
                                lines: vec![],
                                divert: Default::default(),
                                sticky: true
                            },
                        ]),
                        knot_tags: vec![]
                    }
                ),
                (
                    "zoo".to_string(),
                    Knot {
                        title: "zoo".to_string(),
                        lines: vec!["At the zoo.".into()],
                        end: KnotEnd::Divert("start".into()),
                        knot_tags: vec![]
                    }
                )
            ]),
            global_tags: vec!["hidden"]
        }
    );
}

#[test]
fn parse_const() {
    let string = strip_comments(include_str!("../samples/const.ink"));
    let lexed = lex(&string);
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed,
        InkStory {
            global_variables_and_constants: BTreeMap::from([("GRAVITY", Float(9.81))]),
            knots: BTreeMap::from([(
                "__INTRO__".to_string(),
                Knot {
                    title: "__INTRO__".to_string(),
                    lines: vec![
                        Line::Dialog(DialogLine {
                            text: "Gravity is an acceleration of ",
                            has_newline: false,
                            tags: vec![],
                        }),
                        Line::Expression("GRAVITY".into()),
                        " m/s^2.".into(),
                    ],
                    ..Default::default()
                }
            ),]),
            global_tags: vec![]
        }
    );
}

#[test]
fn test_newlines() {
    let string = strip_comments(include_str!("../samples/hidden_choice_text.ink"));
    let lexed = lex(&string);
    dbg!(&lexed);
    let parsed = lexed_to_parsed(&lexed);
    assert_eq!(
        parsed.knots,
        BTreeMap::from([(
            "__INTRO__".to_string(),
            Knot {
                title: "__INTRO__".to_string(),
                lines: vec![Dialog(DialogLine {
                    text: "what to do?",
                    has_newline: true,
                    tags: vec!["hidden"]
                })],
                end: Choices(vec![
                    Choice {
                        choice_text: "😊".to_string(),
                        shown_text: "You smile, a grin as big as the sun.".to_string(),
                        has_newline: true,
                        ..Default::default()
                    },
                    Choice {
                        choice_text: "😀 - time to smile".to_string(),
                        shown_text: "😀 - you fight the need to frown, eyes watering.".to_string(),
                        has_newline: true,
                        ..Default::default()
                    },
                    Choice {
                        choice_text: "😎 - be cool".to_string(),
                        shown_text: "You are being very cool.".to_string(),
                        has_newline: false,
                        ..Default::default()
                    }
                ]),
                knot_tags: vec!["hidden"]
            }
        ),]),
    );
}
