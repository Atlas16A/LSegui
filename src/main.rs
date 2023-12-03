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

pub struct Lsegui {
    //The graph that will be displayed
    g: Graph<(), (), Directed, DefaultIx, NodeShapeAnimated>,
    //The user input string that will be used to create the graph
    input_string: String,
    //Boolean to display the graph once the user has entered a phrase
    graph_show: bool,
}

impl Lsegui {
    fn new(cc: &CreationContext<'_>) -> Self {
        // Initialize the graph
        let g = StableGraph::new();
        //Apply the style from the theme module
        let style = theme::style();
        cc.egui_ctx.set_style(style);

        Self {
            //By default the graph is empty and not displayed
            g: Graph::from(&g),
            input_string: String::new(),
            graph_show: false,
        }
    }
    //Reset the graph and its metadata to ensure that the graph is ready for the next input
    fn reset_graph(&mut self, ui: &mut egui::Ui) {
        let g = StableGraph::new();

        self.g = Graph::from(&g);

        GraphView::<(), (), Directed, DefaultIx>::reset_metadata(ui);
    }

    //Check if the char in the phrase is connected to any other char in the phrase and add an edge between them
    //Some nodes may not get any connections if the current word does not have any letters for the current character to connect to
    fn connections_check(
        &mut self,
        connections: &str,
        g: &mut StableGraph<(), ()>,
        node_indices: Vec<petgraph::prelude::NodeIndex>,
        char_node_pairs: &mut [(char, NodeIndex<u32>)],
        k: usize,
    ) {
        connections.chars().for_each(|target_char| {
            let target_indices: Vec<NodeIndex<u32>> = char_node_pairs
                .iter()
                .filter(|(c, _)| *c == target_char)
                .map(|(_, index)| *index)
                .collect();

            target_indices.into_iter().for_each(|target_index| {
                g.add_edge(node_indices[k], target_index, ());
            });
        });
    }

    //Create the graph from the user input
    //Take in the phrase and convert it to uppercase
    //Split the phrase into words
    //For each word in the phrase:
    //  Add a node for each letter in the word
    //  Add edges between the nodes for each letter in the word
    //  Set the label for each node to the letter in the word at the same index as the node
    //  Position each node along a circle for each word in phrase
    //      with the first circle being at the center of the canvas and
    //      the next circle being to the bottom of the first circle
    fn node_creation(&mut self, phrase: &str) {
        let mut g = StableGraph::new();

        //Sanitize the input to only english letters
        let phrase = phrase
            .chars()
            .filter(|c| c.is_ascii_alphabetic() || c.is_whitespace())
            .collect::<String>();

        let binding = phrase.to_uppercase();
        let binding = binding
            .split_whitespace()
            .map(|word| {
                if word.chars().next() == word.chars().last() {
                    //remove the last letter from the word

                    let new_word = word.chars().take(word.len() - 1).collect::<String>();
                    new_word
                } else {
                    word.to_string()
                }
            })
            .collect::<Vec<_>>();

        println!("Phrase: {}", &phrase);
        println!("Binding: {:?}", &binding.clone().iter().collect::<Vec<_>>());

        binding.clone().iter().for_each(|word| {
            println!("Phrase: {}", &word);
            //Add Node for each letter in phrase
            let mut node_indices: Vec<NodeIndex<u32>> = vec![];
            word.chars().for_each(|_char| {
                node_indices.push(g.add_node(()));
            });

            //put the chars from phrase into tuple pairs with the node idex of the char
            let mut char_node_pairs: Vec<(char, NodeIndex<u32>)> = vec![];
            word.char_indices().for_each(|(i, char)| {
                char_node_pairs.push((char, node_indices[i]));
            });

            //Add edges for each letter in phrase
            word.char_indices().for_each(|(k, char)| {
                match char {
                    'A' | 'a' => (),
                    'B' | 'b' => self.connections_check(
                        "A",
                        &mut g,
                        node_indices.clone(),
                        &mut char_node_pairs,
                        k,
                    ),
                    'C' | 'c' => {
                        let connections = "AB";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'D' | 'd' => {
                        let connections = "ABC";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'E' | 'e' => {
                        let connections = "ACD";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'F' | 'f' => {
                        let connections = "ABDE";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'G' | 'g' => {
                        let connections = "AEF";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'H' | 'h' => {
                        let connections = "ABEFG";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'I' | 'i' => {
                        let connections = "ABCEGH";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'J' | 'j' => {
                        let connections = "ACDEFGHI";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'K' | 'k' => {
                        let connections = "ABCIJ";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'L' | 'l' => {
                        let connections = "ACDIJK";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'M' | 'm' => {
                        let connections = "ABCDEIKL";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'N' | 'n' => {
                        let connections = "ACDEFHJKM";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'O' | 'o' => {
                        let connections = "ABEFGILMN";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'P' | 'p' => {
                        let connections = "ACGHIKLNO";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'Q' | 'q' => {
                        let connections = "ABCDEHIJKLMP";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'R' | 'r' => {
                        let connections = "ABCDEGHIKLOPQ";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'S' | 's' => {
                        let connections = "ADEFGHILMO";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'T' | 't' => {
                        let connections = "ACDEFHIJLMNOQS";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'U' | 'u' => {
                        let connections = "ACDFGIJKMPQRST";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'V' | 'v' => {
                        let connections = "ABDEFHJKLNPQS";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'W' | 'w' => {
                        let connections = "AV";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'X' | 'x' => {
                        let connections = "AW";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'Y' | 'y' => {
                        let connections = "AX";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    'Z' | 'z' => {
                        let connections = "AY";
                        self.connections_check(
                            connections,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    _ => (),
                };
            });

            //Deal with nodes that do not have any connections
            //Had to seperate from the previous loop because need to have all the edges added before checking for nodes with no connections
            word.char_indices().for_each(|(k, char)| {
                node_clean_up(
                    &mut g,
                    node_indices.clone(),
                    k,
                    char_node_pairs.clone(),
                    char,
                );
            });
        });

        let node_indices = g.node_indices().collect::<Vec<_>>();
        self.g = Graph::from(&g);

        let phrase = binding.concat().to_uppercase();

        //Set Label for each node, Label is the char in phrase at the same index as the node
        phrase.char_indices().for_each(|(i, char)| {
            self.g
                .node_mut(node_indices[i])
                .unwrap()
                .set_label(char.to_string());
        });

        self.layout_nodes(binding, node_indices);
    }

    fn layout_nodes(&mut self, phrase: Vec<String>, node_indices: Vec<NodeIndex>) {
        //Position each node along a circle for each word in phrase
        //with radius based off the length of the word
        //with the first circle being at the center of the canvas
        //and the next circle being the radius of the first circle + the radius of the next circle

        let center_x = 0.0; // x-coordinate of the center of the canvas
        let mut center_y = 0.0; // y-coordinate of the center of the canvas
        let mut offset = 0; // offset to keep track of the current node index
        let mut prev_radius = 0.0; // radius of the previous circle to calculate the center_y of the next circle

        phrase.iter().for_each(|word| {
            let mut angle: f32 = -90.0;
            let angle_increment = 360.0 / word.len() as f32;
            let radius = 20.0 * word.len() as f32;

            if prev_radius != 0.0 {
                center_y += prev_radius + radius;
            }

            node_indices.iter().enumerate().for_each(|(i, _)| {
                if i < word.len() {
                    let x = center_x + angle.to_radians().cos() * radius;
                    let y = center_y + angle.to_radians().sin() * radius;
                    self.g
                        .node_mut(node_indices[i + offset])
                        .expect("NodeIndex should be within node indices")
                        .set_location(egui::Pos2::new(x, y));

                    angle += angle_increment;
                }
            });
            prev_radius = radius;
            offset += word.len();
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

fn node_clean_up(
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
                    self.node_creation(phrase);
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
