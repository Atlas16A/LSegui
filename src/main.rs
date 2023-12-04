#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, App, CreationContext};
use egui::Context;
use egui_graphs::{DefaultEdgeShape, Graph, GraphView, SettingsInteraction, SettingsNavigation};
use petgraph::{
    stable_graph::{DefaultIx, NodeIndex, StableGraph},
    Directed,
};

//eframe and egui styling
mod theme;
//Edge Display code
mod edge;
//Node Display code
mod node;
use node::NodeShapeAnimated;

struct Circles {
    circles: Vec<Circle>,
}

struct Circle {
    center_x: f32,
    center_y: f32,
    radius: f32,
    word: Word,
}

impl Circle {
    fn new(
        word: Word,
        graph: &mut Graph<(), (), Directed, u32, NodeShapeAnimated, DefaultEdgeShape>,
    ) -> Self {
        let mut angle: f32 = -90.0;
        let angle_increment = 360.0 / word.word.len() as f32;
        let radius = 20.0 * word.word.len() as f32;
        let center_x = 0.0;
        let center_y = 0.0;

        word.nodes.iter().enumerate().for_each(|(i, node)| {
            if i < word.word.len() {
                let x = center_x + angle.to_radians().cos() * radius;
                let y = center_y + angle.to_radians().sin() * radius;
                graph
                    .node_mut(*node)
                    .expect("NodeIndex should be within node indices")
                    .set_location(egui::Pos2::new(x, y));

                angle += angle_increment;
            }
        });

        Self {
            center_x,
            center_y,
            radius,
            word,
        }
    }

    fn set_pos(
        &mut self,
        x: f32,
        y: f32,
        graph: &mut Graph<(), (), Directed, u32, NodeShapeAnimated, DefaultEdgeShape>,
    ) {
        self.center_x = x;
        self.center_y = y;
        //Adjust the position of the circle
        //And the position of the nodes in the circle
        let mut angle: f32 = -90.0;
        let angle_increment = 360.0 / self.word.word.len() as f32;
        let radius = 20.0 * self.word.word.len() as f32;

        self.word.nodes.iter().enumerate().for_each(|(i, node)| {
            if i < self.word.word.len() {
                let x = self.center_x + angle.to_radians().cos() * radius;
                let y = self.center_y + angle.to_radians().sin() * radius;
                graph
                    .node_mut(*node)
                    .expect("NodeIndex should be within node indices")
                    .set_location(egui::Pos2::new(x, y));

                angle += angle_increment;
            }
        });
    }
}
#[derive(Clone)]
struct Word {
    word: String,
    nodes: Vec<NodeIndex<u32>>,
}

impl Word {
    fn new(word: String, nodes: Vec<NodeIndex<u32>>) -> Self {
        Self { word, nodes }
    }
}

struct Phrase {
    phrase: Vec<String>,
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
            phrase,
            phrase_words,
            graph: g,
        }
    }
}

/* enum NodeLayout {
    RepelTop,
    RepelBottom,
    SameCharTop,
    SameCharBottom,
}

use NodeLayout::*; */

pub struct Lsegui {
    //The graph that will be displayed
    g: Graph<(), (), Directed, DefaultIx, NodeShapeAnimated>,
    //The user input string that will be used to create the graph
    input_string: String,
    //Boolean to display the graph once the user has entered a phrase
    graph_show: bool,
    //Circles to display the nodes in the graph
    circles: Circles,
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
        let circles = Circles { circles: vec![] };
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
    }

    fn graph_creation(&mut self, phrase: &str) {
        self.phrase = Phrase::new(phrase);

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

        self.node_circle_create();

        //testing ideas above
    }

    fn node_circle_create(&mut self) {
        //Position each node along a circle for each word in phrase
        //with radius based off the length of the word
        //with the first circle being at the center of the canvas
        //and the next circle being the radius of the first circle + the radius of the next circle

        let center_x = 0.0; // x-coordinate of the center of the canvas
        let mut center_y = 0.0; // y-coordinate of the center of the canvas
        let mut prev_radius = 0.0; // radius of the previous circle to calculate the center_y of the next circle

        self.phrase.phrase_words.iter().for_each(|word| {
            let radius = 20.0 * word.word.len() as f32;
            if prev_radius != 0.0 {
                center_y += prev_radius + radius;
            }
            let mut circle = Circle::new(word.clone(), &mut self.g);
            circle.set_pos(center_x, center_y, &mut self.g);

            self.circles.circles.push(circle);

            prev_radius = radius;
        });
    }

    /* fn analyse_phrase(&mut self, phrase: Vec<String>, node_indices: Vec<NodeIndex>) {
        //Analyse the phrase, given N words, where 1 is the first word and N is the last word
        //for each word in the phrase:
        //  find a letter shared between each subsequent word, so that word 1 shares a letter with word 2, word 2 shares a letter with word 3, etc.
        //  if there is no letter shared between the current word and the next word
        //      then shift the nodes of each of the two words away from the border between the two words
        //      so that the nodes of the current word are shifted away from the bottom most point of the circle of the current word
        //      and the nodes of the next word are shifted away from the top most point of the circle of the next word
        //      node shift should shift the nodes as little distance as possible
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

/* fn node_clean_up(
    g: &mut StableGraph<(), ()>,
    node_indices: Vec<NodeIndex>,
    k: usize,
    char_node_pairs: Vec<(char, NodeIndex)>,
    char: char,
) {
    //If the current node does not have any incoming or outgoing edges
    if g.neighbors_directed(node_indices[k], petgraph::Direction::Outgoing)
        .count()
        == 0
        && g.neighbors_directed(node_indices[k], petgraph::Direction::Incoming)
            .count()
            == 0
    {
        //Out of the nodes in the current word,
        //connect the current node to the node
        //representing the character closest to the current character on the alphabet
        let mut closest_index = 0;
        let mut closest_distance = 26;
        char_node_pairs
            .iter()
            .filter(|(c, _)| *c != char)
            .enumerate()
            .for_each(|(i, (c, _))| {
                let distance = (char as i32 - *c as i32).abs();
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_index = i;
                }
            });
        g.add_edge(node_indices[k], node_indices[closest_index], ());
    }
}
 */

impl App for Lsegui {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Enter a Phrase:");
                //Take in user text input
                let re = ui.text_edit_singleline(&mut self.input_string);
                if re.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
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
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.graph_show {
                ui.add(
                    &mut GraphView::<_, _, _, _, NodeShapeAnimated, DefaultEdgeShape>::new(
                        &mut self.g,
                    )
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
            }
        });
    }
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
