use eframe::{App, CreationContext};
use egui::Context;
use egui_graphs::{DefaultEdgeShape, Graph, GraphView, SettingsInteraction, SettingsNavigation};

use eframe::egui;
use petgraph::{
    stable_graph::{DefaultIx, NodeIndex, StableGraph},
    Directed,
};

mod node;
mod theme;
use node::NodeShapeAnimated;
mod edge;

pub struct Lsegui {
    //Graph<(), (), Directed, DefaultIx, NodeShapeAnimated>
    g: Graph<(), (), Directed, DefaultIx, NodeShapeAnimated>,
    input_string: String,
    graph_show: bool,
}

impl Lsegui {
    fn new(cc: &CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let g = init_graph();
        let style = theme::style();
        cc.egui_ctx.set_style(style);
        Self {
            g: Graph::from(&g),
            input_string: String::new(),
            graph_show: false,
        }
    }

    fn reset_graph(&mut self, ui: &mut egui::Ui) {
        let g = init_graph();

        self.g = Graph::from(&g);

        GraphView::<(), (), Directed, DefaultIx>::reset_metadata(ui);
    }

    //if char_node_pairs contains a char value that is in connections create an edge between the nodeIndex of the current char and the nodeIndex of the char in connections
    fn connections_check(
        &mut self,
        connections: &str,
        phrase: &str,
        g: &mut StableGraph<(), ()>,
        node_indices: Vec<petgraph::prelude::NodeIndex>,
        char_node_pairs: &mut [(char, NodeIndex<u32>)],
        k: usize,
    ) {
        for target_char in connections.chars() {
            if phrase.contains(target_char) {
                let target_index = char_node_pairs
                    .iter()
                    .find(|(c, _)| *c == target_char)
                    .map(|(_, index)| *index);

                if let Some(target_index) = target_index {
                    g.add_edge(node_indices[k], target_index, ());
                }
            }
        }
    }

    fn node_creation(&mut self, phrase: &str) {
        let mut g = init_graph();

        let binding = phrase.to_uppercase();
        let binding = binding.split_whitespace();

        println!("Phrase: {}", &phrase);
        println!("Binding: {:?}", &binding.clone().collect::<Vec<_>>());

        binding.for_each(|word| {
            println!("Phrase: {}", &word);
            //Add Node for each letter in phrase
            let mut node_indices: Vec<NodeIndex<u32>> = vec![];
            for _char in word.chars() {
                node_indices.push(g.add_node(()));
            }

            //put the chars from phrase into tuple pairs with the node idex of the char
            let mut char_node_pairs: Vec<(char, NodeIndex<u32>)> = vec![];
            for (i, char) in word.char_indices() {
                char_node_pairs.push((char, node_indices[i]));
            }

            //Add edges for each letter in phrase
            for (k, char) in word.char_indices() {
                match char {
                    'A' | 'a' => (),
                    'B' | 'b' => self.connections_check(
                        "A",
                        word,
                        &mut g,
                        node_indices.clone(),
                        &mut char_node_pairs,
                        k,
                    ),
                    'C' | 'c' => {
                        let connections = "AB";
                        self.connections_check(
                            connections,
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
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
                            word,
                            &mut g,
                            node_indices.clone(),
                            &mut char_node_pairs,
                            k,
                        );
                    }
                    _ => {
                        // Handle the case when char is not any of the above
                        continue;
                    }
                };
            }
        });

        let node_indices = g.node_indices().collect::<Vec<_>>();
        self.g = Graph::from(&g);

        let phrase = phrase
            .split_whitespace()
            .collect::<Vec<_>>()
            .concat()
            .to_uppercase();

        //Set Label for each node, Label is the char in phrase at the same index as the node
        for (i, char) in phrase.char_indices() {
            self.g
                .node_mut(node_indices[i])
                .unwrap()
                .set_label(char.to_string());
        }
    }
}

impl App for Lsegui {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                //take sentence input, with font size 20
                ui.label("Enter a Phrase:");
                let re = ui.text_edit_singleline(&mut self.input_string);
                if re.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    println!("Input: {}", self.input_string);
                    self.reset_graph(ui);
                    let phrase = self.input_string.clone();
                    let phrase = phrase.as_str();

                    self.node_creation(phrase);
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

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lost Skies Gall Language Art Generator",
        native_options,
        Box::new(|cc| Box::new(Lsegui::new(cc))),
    )
    .unwrap();
}

fn init_graph() -> StableGraph<(), ()> {
    StableGraph::new()
}
