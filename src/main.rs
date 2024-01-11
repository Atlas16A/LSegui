#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, App, CreationContext};
use egui::Context;

use egui_graphs::{Graph, GraphView, SettingsInteraction, SettingsNavigation};
use petgraph::{
    stable_graph::{DefaultIx, NodeIndex, StableGraph},
    Directed,
};

/* use eframe::{egui, App, CreationContext};
use egui::{epaint::CubicBezierShape, Context, Pos2, Stroke};

use egui_graphs::{Graph, GraphView, Metadata, SettingsInteraction, SettingsNavigation};
use petgraph::{
    stable_graph::{DefaultIx, NodeIndex, StableGraph},
    Directed,
}; */

//eframe and egui styling
mod theme;
//Edge Display code
mod edge;
use edge::EdgeShape;
//Node Display code
mod node;
use node::NodeShape;

mod circle_layout;
use circle_layout::CircleLayout;

/* #[derive(Clone)]
struct Circles {
    circles: Vec<Circle>,
}

impl Circles {
    fn new() -> Self {
        Self { circles: vec![] }
    }

    fn circle_layout(
        &mut self,
        graph: &mut egui_graphs::Graph<
            (),
            (),
            petgraph::Directed,
            u32,
            node::NodeShape,
            edge::EdgeShape,
        >,
    ) {
        let mut center_y = 0.0;
        self.circles
            .clone()
            .iter_mut()
            .enumerate()
            .for_each(|(i, circle)| {
                match (
                    circle.word.layout_top.clone(),
                    circle.word.layout_bottom.clone(),
                ) {
                    (RepelTop, Alone) => {
                        //Last circle and first letter of the word does not match the last letter of the previous word
                        let word_len = circle.word.word.len() as f32;
                        let mut angle: f32 = -40.0;
                        let angle_increment = (360.0 - (30.0 + 45.0)) / word_len;
                        circle.radius = 20.0 * word_len;
                        if i != 0 {
                            center_y += self.circles[i - 1].radius + circle.radius;
                        }

                        circle.word.nodes.iter().for_each(|node| {
                            let x = circle.center_x + angle.to_radians().cos() * circle.radius;
                            let y = center_y + angle.to_radians().sin() * circle.radius;
                            graph
                                .node_mut(*node)
                                .expect("NodeIndex should be within node indices")
                                .set_location(egui::Pos2::new(x, y));
                            angle += angle_increment;
                        });
                        self.circles[i].center_y = center_y;
                        self.circles[i].radius = circle.radius;
                    }
                    (SameCharTop, Alone) => {
                        //Last circle and first letter of the word does match the last letter of the previous word
                        let word_len = circle.word.word.len() as f32;
                        let mut angle: f32 = -90.0;
                        let angle_increment = 360.0 / word_len;
                        circle.radius = 20.0 * word_len;
                        if i != 0 {
                            center_y += self.circles[i - 1].radius + circle.radius;
                        }

                        circle.word.nodes.iter().enumerate().for_each(|(i, node)| {
                            if i < circle.word.word.len() {
                                let x = circle.center_x + angle.to_radians().cos() * circle.radius;
                                let y = center_y + angle.to_radians().sin() * circle.radius;
                                graph
                                    .node_mut(*node)
                                    .expect("NodeIndex should be within node indices")
                                    .set_location(egui::Pos2::new(x, y));

                                angle += angle_increment;
                            }
                        });
                        self.circles[i].center_y = center_y;
                        self.circles[i].radius = circle.radius;
                    }
                    (Alone, SameCharBottom) | (SameCharTop, SameCharBottom) => {
                        //First circle and last letter of the word does match the first letter of the next word
                        let word_len = circle.word.word.len() as f32;
                        let mut angle: f32 = -90.0;
                        let angle_increment = 180.0 / (word_len - 1.0);
                        circle.radius = 20.0 * word_len;
                        if i != 0 {
                            center_y += self.circles[i - 1].radius + circle.radius;
                        }

                        circle.word.nodes.iter().enumerate().for_each(|(_i, node)| {
                            let x = circle.center_x + angle.to_radians().cos() * circle.radius;
                            let y = center_y + angle.to_radians().sin() * circle.radius;
                            graph
                                .node_mut(*node)
                                .expect("NodeIndex should be within node indices")
                                .set_location(egui::Pos2::new(x, y));

                            angle += angle_increment;
                        });
                        self.circles[i].center_y = center_y;
                        self.circles[i].radius = circle.radius;
                    }
                    (Alone, RepelBottom) | (SameCharTop, RepelBottom) => {
                        //First circle and last letter of the word does not match the first letter of the next word
                        let word_len = circle.word.word.len() as f32;
                        let mut angle: f32 = -90.0;
                        let angle_increment = 360.0 / word_len;
                        circle.radius = 20.0 * word_len;
                        if i != 0 {
                            center_y += self.circles[i - 1].radius + circle.radius;
                        }

                        circle.word.nodes.iter().enumerate().for_each(|(i, node)| {
                            if i == (word_len / 2.0).round() as usize {
                                angle += 45.0;
                            }

                            let x = circle.center_x + angle.to_radians().cos() * circle.radius;
                            let y = center_y + angle.to_radians().sin() * circle.radius;
                            graph
                                .node_mut(*node)
                                .expect("NodeIndex should be within node indices")
                                .set_location(egui::Pos2::new(x, y));

                            angle += angle_increment;
                        });
                        self.circles[i].center_y = center_y;
                        self.circles[i].radius = circle.radius;
                    }
                    (Alone, _) => {
                        //Only one word/circle in the phrase
                        let word_len = circle.word.word.len() as f32;
                        let mut angle: f32 = -90.0;
                        let angle_increment = 360.0 / word_len;
                        circle.radius = 20.0 * word_len;
                        if i != 0 {
                            center_y += self.circles[i - 1].radius + circle.radius;
                        }

                        circle.word.nodes.iter().enumerate().for_each(|(i, node)| {
                            if i < circle.word.word.len() {
                                let x = circle.center_x + angle.to_radians().cos() * circle.radius;
                                let y = center_y + angle.to_radians().sin() * circle.radius;
                                graph
                                    .node_mut(*node)
                                    .expect("NodeIndex should be within node indices")
                                    .set_location(egui::Pos2::new(x, y));

                                angle += angle_increment;
                            }
                        });
                        self.circles[i].center_y = center_y;
                        self.circles[i].radius = circle.radius;
                    }
                    (RepelTop, RepelBottom) => {
                        //The first and last letter of the word does not match the first letter of the next word and last letter of the previous word
                        let word_len = circle.word.word.len() as f32;
                        let mut angle: f32 = -40.0;
                        let angle_increment = (360.0 - (30.0 + 45.0)) / word_len;
                        circle.radius = 20.0 * word_len;
                        if i != 0 {
                            center_y += self.circles[i - 1].radius + circle.radius;
                        }

                        circle.word.nodes.iter().for_each(|node| {
                            let x = circle.center_x + angle.to_radians().cos() * circle.radius;
                            let y = center_y + angle.to_radians().sin() * circle.radius;
                            graph
                                .node_mut(*node)
                                .expect("NodeIndex should be within node indices")
                                .set_location(egui::Pos2::new(x, y));
                            angle += angle_increment;
                        });
                        self.circles[i].center_y = center_y;
                        self.circles[i].radius = circle.radius;
                    }
                    (RepelTop, SameCharBottom) => (),
                    _ => (),
                }
                println!("Center y: {}", center_y);
                println!("Radius: {}", circle.radius)
            });
    }
    //Draw the circles within the graph view
    fn draw_circles(&self, ui: &mut egui::Ui) {
        /* let perfect_bezier = CubicBezierShape {
            points: [
                Pos2::new(0.0, 1.000_055_2),
                Pos2::new(0.55342686, 0.99873585),
                Pos2::new(0.99873585, 0.55342686),
                Pos2::new(1.000_055_2, 0.0),
            ],
            closed: false,
            stroke: Stroke::new(1.0, color),
            fill: Default::default(),
        }; */

        self.circles.iter().for_each(|circle| {
            let circle_center = Metadata::get(ui).canvas_to_screen_pos(egui::Pos2 {
                x: (circle.center_x),
                y: (circle.center_y),
            });
            let circle_radius = Metadata::get(ui).canvas_to_screen_size(circle.radius);

            let a = 1.000_055_2 + circle_radius;
            let b = 0.55342686 + circle_radius / 1.81;
            let c = 0.99873585 + circle_radius;

            let p0 = Pos2::new(circle_center.x, circle_center.y + a); //Bottom point
            let p1 = Pos2::new(circle_center.x + b, circle_center.y + c);
            let p2 = Pos2::new(circle_center.x + c, circle_center.y + b);
            let p3 = Pos2::new(circle_center.x + a, circle_center.y); //Right point
            let p4 = Pos2::new(circle_center.x + c, circle_center.y - b);
            let p5 = Pos2::new(circle_center.x + b, circle_center.y - c);
            let p6 = Pos2::new(circle_center.x, circle_center.y - a); //Top point
            let p7 = Pos2::new(circle_center.x - b, circle_center.y - c);
            let p8 = Pos2::new(circle_center.x - c, circle_center.y - b);
            let p9 = Pos2::new(circle_center.x - a, circle_center.y); //Left point
            let p10 = Pos2::new(circle_center.x - c, circle_center.y + b);
            let p11 = Pos2::new(circle_center.x - b, circle_center.y + c);

            ui.painter().add(CubicBezierShape {
                points: [p0, p1, p2, p3],
                stroke: Stroke::new(1.0, egui::Color32::WHITE),
                fill: Default::default(),
                closed: false,
            });
            ui.painter().add(CubicBezierShape {
                points: [p3, p4, p5, p6],
                stroke: Stroke::new(1.0, egui::Color32::RED),
                fill: Default::default(),
                closed: false,
            });
            ui.painter().add(CubicBezierShape {
                points: [p6, p7, p8, p9],
                stroke: Stroke::new(1.0, egui::Color32::BLUE),
                fill: Default::default(),
                closed: false,
            });
            ui.painter().add(CubicBezierShape {
                points: [p9, p10, p11, p0],
                stroke: Stroke::new(1.0, egui::Color32::GREEN),
                fill: Default::default(),
                closed: false,
            });
        })
    }
}

#[derive(Clone)]
struct Circle {
    center_x: f32,
    center_y: f32,
    radius: f32,
    word: Word,
}

impl Circle {
    fn new(word: Word) -> Self {
        let center_x = 0.0;
        let center_y = 0.0;
        let radius = word.word.len() as f32 * 20.0;

        Self {
            center_x,
            center_y,
            radius,
            word,
        }
    }
} */

#[derive(Clone)]
pub struct Word {
    word: String,
    nodes: Vec<NodeIndex<u32>>,
    layout_top: NodeLayout,
    layout_bottom: NodeLayout,
}

impl Default for Word {
    fn default() -> Self {
        Self {
            word: String::new(),
            nodes: vec![],
            layout_top: Alone,
            layout_bottom: Alone,
        }
    }
}

impl Word {
    fn new(word: String, nodes: Vec<NodeIndex<u32>>) -> Self {
        Self {
            word,
            nodes,
            layout_top: Alone,
            layout_bottom: Alone,
        }
    }
}

pub struct Phrase {
    //phrase: Vec<String>,
    phrase_words: Vec<Word>,
    graph: StableGraph<(), ()>,
}

impl Phrase {
    fn new(phrase: &str) -> Self {
        let mut g: StableGraph<(), ()> = StableGraph::new();

        let phrase = phrase
            .chars()
            .filter(|c| c.is_ascii_alphabetic() || c.is_whitespace())
            .collect::<String>()
            .to_uppercase()
            .split_whitespace()
            .map(|word| {
                if word.chars().next() == word.chars().last() {
                    let new_word = word.chars().take(word.len() - 1).collect::<String>();
                    new_word
                } else {
                    word.to_string()
                }
            })
            .collect::<Vec<_>>();
        let mut phrase_words = vec![];
        phrase.clone().iter().for_each(|word| {
            let mut node_indices: Vec<NodeIndex<u32>> = vec![];
            word.chars().for_each(|_char| {
                node_indices.push(g.add_node(()));
            });

            let word = Word::new(word.to_string(), node_indices);

            phrase_words.push(word);
        });

        Self {
            //phrase,
            phrase_words,
            graph: g,
        }
    }
    fn analyse_phrase(&mut self) {
        //Analyse the phrase, given N words, where 1 is the first word and N is the last word
        //for each word in the phrase:
        //  find a letter shared between each subsequent word, so that word 1 shares a letter with word 2, word 2 shares a letter with word 3, etc.
        //  if there is no letter shared between the current word and the next word
        //      then shift the nodes of each of the two words away from the border between the two words
        //      so that the nodes of the current word are shifted away from the bottom most point of the circle of the current word
        //      and the nodes of the next word are shifted away from the top most point of the circle of the next word
        //      node shift should shift the nodes as little distance as possible

        //If the phrase has more than one word
        /* if self.phrase_words.len() > 1 {
            self.phrase_words
                .clone()
                .iter()
                .enumerate()
                .for_each(|(cword_index, word)| {
                    //If the current word is not the last word in the phrase
                    if self.phrase_words.len() > cword_index + 1 {
                        let current_word = word.clone();
                        let next_word = self.phrase_words[cword_index + 1].clone();
                        /* let current_word_char_node_pairs = current_word
                            .word
                            .chars()
                            .zip(current_word.nodes.clone())
                            .collect::<Vec<_>>();
                        let next_word_char_node_pairs = next_word
                            .word
                            .chars()
                            .zip(next_word.nodes.clone())
                            .collect::<Vec<_>>();
                        let current_next_char_pairs = current_word
                            .word
                            .chars()
                            .zip(next_word.word.chars().rev().clone())
                            .collect::<Vec<_>>(); */
                        let mut shared_char: char = ' ';
                        current_word.word.chars().enumerate().for_each(|(ci, c)| {
                            //If the last character of the current word is the same as the first character of the next word
                            if ci == current_word.word.len() - 1 && next_word.word.starts_with(c) {
                                shared_char = c;
                                self.phrase_words[cword_index].layout_bottom = SameCharBottom;
                                self.phrase_words[cword_index + 1].layout_top = SameCharTop;
                            }
                            //If the last character of the current word is not the same as the first character of the next word
                            if ci == current_word.word.len() - 1 && !next_word.word.starts_with(c) {
                                self.phrase_words[cword_index].layout_bottom = RepelBottom;
                                self.phrase_words[cword_index + 1].layout_top = RepelTop;
                            }
                            //If the current word starts with a character in the previous word
                            if ci == 0 && next_word.word.ends_with(c) {
                                shared_char = c;
                                self.phrase_words[cword_index].layout_bottom = SameCharBottom;
                                self.phrase_words[cword_index + 1].layout_top = SameCharTop;
                            }
                        });
                    }
                })
        } else {
            //If the phrase only has one word
            self.phrase_words[0].layout_top = Alone;
            self.phrase_words[0].layout_bottom = Alone;
        } */
    }
}

#[derive(Clone, Debug)]
enum NodeLayout {
    /* RepelTop,
    RepelBottom,
    SameCharTop,
    SameCharBottom, */
    Alone,
}

use NodeLayout::*;

pub struct Lsegui {
    //The graph that will be displayed
    pub g: Graph<(), (), Directed, DefaultIx, NodeShape, EdgeShape>,
    //The user input string that will be used to create the graph
    input_string: String,
    //Boolean to display the graph once the user has entered a phrase
    graph_show: bool,
    //Circles to display the nodes in the graph
    circles: CircleLayout,
    //The processed phrase that the user entered
    phrase: Phrase,
}

impl Lsegui {
    fn new(cc: &CreationContext<'_>) -> Self {
        // Initialize the graph
        let g = StableGraph::new();
        //Apply the style from the theme module
        let style = theme::style();
        cc.egui_ctx.set_style(style);
        let circles = CircleLayout::new();
        let phrase = Phrase::new("Default Phrase");

        Self {
            //By default the graph is empty and not displayed
            g: Graph::from(&g),
            input_string: String::new(),
            graph_show: false,
            circles,
            phrase,
        }
    }
    //Reset the graph and its metadata to ensure that the graph is ready for the next input
    fn reset_graph(&mut self, ui: &mut egui::Ui) {
        let g = StableGraph::new();

        self.g = Graph::from(&g);

        GraphView::<(), (), Directed, DefaultIx>::reset_metadata(ui);
        self.circles = CircleLayout::new();
    }

    fn graph_creation(&mut self, phrase: &str) {
        self.phrase = Phrase::new(phrase);

        self.phrase.analyse_phrase();

        self.phrase.phrase_words.iter().for_each(|word| {
            let word_char_pairs = word.word.chars().zip(word.nodes.clone());

            word_char_pairs
                .clone()
                .for_each(|(current_char, current_node)| {
                    match current_char {
                        'A' | 'a' => (),
                        'B' | 'b' => {
                            let connections = "A";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'C' | 'c' => {
                            let connections = "AB";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'D' | 'd' => {
                            let connections = "ABC";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'E' | 'e' => {
                            let connections = "ACD";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'F' | 'f' => {
                            let connections = "ABDE";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'G' | 'g' => {
                            let connections = "AEF";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'H' | 'h' => {
                            let connections = "ABEFG";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'I' | 'i' => {
                            let connections = "ABCEGH";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'J' | 'j' => {
                            let connections = "ACDEFGHI";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'K' | 'k' => {
                            let connections = "ABCIJ";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'L' | 'l' => {
                            let connections = "ACDIJK";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'M' | 'm' => {
                            let connections = "ABCDEIKL";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'N' | 'n' => {
                            let connections = "ACDEFHJKM";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'O' | 'o' => {
                            let connections = "ABEFGILMN";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'P' | 'p' => {
                            let connections = "ACGHIKLNO";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'Q' | 'q' => {
                            let connections = "ABCDEHIJKLMP";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'R' | 'r' => {
                            let connections = "ABCDEGHIKLOPQ";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'S' | 's' => {
                            let connections = "ADEFGHILMO";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'T' | 't' => {
                            let connections = "ACDEFHIJLMNOQS";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'U' | 'u' => {
                            let connections = "ACDFGIJKMPQRST";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'V' | 'v' => {
                            let connections = "ABDEFHJKLNPQS";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'W' | 'w' => {
                            let connections = "AV";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'X' | 'x' => {
                            let connections = "AW";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'Y' | 'y' => {
                            let connections = "AX";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        'Z' | 'z' => {
                            let connections = "AY";
                            refactor_connections_check(
                                connections,
                                &word_char_pairs,
                                &mut self.phrase.graph,
                                &current_node,
                            );
                        }
                        _ => (),
                    };
                });

            word.nodes.iter().for_each(|current_node| {
                if self
                    .phrase
                    .graph
                    .neighbors_directed(*current_node, petgraph::Direction::Outgoing)
                    .count()
                    == 0
                    && self
                        .phrase
                        .graph
                        .neighbors_directed(*current_node, petgraph::Direction::Incoming)
                        .count()
                        == 0
                {
                    //Out of the nodes in the current word,
                    //connect the current node to the node
                    //representing the character closest to the current character on the alphabet
                    let mut closest_index = 0;
                    let mut closest_distance = 26;
                    let current_char = word_char_pairs
                        .clone()
                        .filter(|(_, n)| *n == *current_node)
                        .map(|(c, _)| c)
                        .next()
                        .unwrap();
                    word_char_pairs
                        .clone()
                        .filter(|(c, _)| *c != current_char)
                        .enumerate()
                        .for_each(|(i, (c, _))| {
                            let distance = (current_char as i32 - c as i32).abs();
                            if distance < closest_distance {
                                closest_distance = distance;
                                closest_index = i;
                            }
                        });
                    self.phrase
                        .graph
                        .add_edge(*current_node, word.nodes[closest_index], ());
                }
            });
        });

        self.g = Graph::from(&self.phrase.graph);

        self.phrase.phrase_words.iter().for_each(|word| {
            for (node, letter) in word.nodes.iter().zip(word.word.chars()) {
                self.g
                    .node_mut(*node)
                    .unwrap()
                    .set_label(letter.to_string());
            }
        });

        //self.node_circle_create();
        self.circles.layout(&self.phrase, &mut self.g);
    }

    /* fn node_circle_create(&mut self) {
        self.circles.circles.clear();
        self.phrase.phrase_words.iter().for_each(|word| {
            let circle = Circle::new(word.clone());
            self.circles.circles.push(circle);
        });
        self.circles.circle_layout(&mut self.g);
    } */
}

//Check if the char in the phrase is connected to any other char in the phrase and add an edge between them
//Some nodes may not get any connections if the current word does not have any letters for the current character to connect to
fn refactor_connections_check(
    connections: &str,
    word_char_pairs: &std::iter::Zip<std::str::Chars<'_>, std::vec::IntoIter<NodeIndex>>,
    g: &mut StableGraph<(), ()>,
    current_node: &NodeIndex,
) {
    connections.chars().for_each(|target_char| {
        word_char_pairs
            .clone()
            .filter(|(c_c, _)| *c_c == target_char)
            .for_each(|target_index| {
                g.add_edge(*current_node, target_index.1, ());
            });
    });
}

impl App for Lsegui {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Enter a Phrase:");
                //Take in user text input
                let re = ui.text_edit_singleline(&mut self.input_string);
                if re.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.graph_show = false;
                    println!("Input: {}", self.input_string);
                    //Clear the current graph just in case
                    self.reset_graph(ui);
                    //Convert the input to a string slice
                    let phrase = self.input_string.clone();
                    let phrase = phrase.as_str();
                    //Create the graph from the input
                    self.graph_creation(phrase);
                    //Display the graph
                    self.graph_show = true;

                    self.phrase.phrase_words.iter().for_each(|word| {
                        println!("Word: {}", word.word);
                        println!("Word layout_top{:?}", word.layout_top);
                        println!("Word layout_bottom{:?}", word.layout_bottom);
                    });
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.set_min_height(25.0);
                if ui.button("Reset").clicked() {
                    self.reset_graph(ui);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.graph_show {
                let graph = ui.add(
                    &mut GraphView::<_, _, _, _, NodeShape, EdgeShape>::new(&mut self.g)
                        .with_navigations(
                            &SettingsNavigation::default()
                                .with_fit_to_screen_enabled(false)
                                .with_zoom_and_pan_enabled(true),
                        )
                        .with_interactions(
                            &SettingsInteraction::default()
                                .with_dragging_enabled(true)
                                .with_node_selection_enabled(true)
                                .with_edge_selection_enabled(true),
                        ),
                );
                let clip_rect = graph.rect;
                ui.set_clip_rect(clip_rect);
                self.circles.draw_circles(ui);
            }
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    patreon_and_github(ui);
                });
            });
        });
    }
}

fn patreon_and_github(ui: &mut egui::Ui) {
    ui.label("Made by: Rin aka The Golden Atlas");
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Consider Donating through ");
        ui.hyperlink_to("Patreon", "https://patreon.com/Rinoxide");
        ui.label(".");
    });
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lost Skies Gall Language Art Generator",
        native_options,
        Box::new(|cc| Box::new(Lsegui::new(cc))),
    )
    .unwrap();
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(Lsegui::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
