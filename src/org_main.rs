use eframe::{App, CreationContext};
use egui::Context;
use egui_graphs::{DefaultEdgeShape, Graph, GraphView, SettingsInteraction, SettingsNavigation};

use eframe::egui;
use petgraph::{
    stable_graph::{DefaultIx, NodeIndex, StableGraph},
    visit::EdgeRef,
    Directed,
};

mod node;
mod theme;
use node::NodeShapeAnimated;
mod edge;

pub struct Payload {
    pub visable: bool,
}

impl Payload {
    pub fn new(tf: bool) -> Self {
        Self { visable: tf }
    }
}

pub struct Lsegui {
    //g: Graph<(), (), Directed, DefaultIx>,
    g: Graph<(), (), Directed, DefaultIx, NodeShapeAnimated>,
    input_string: String,
    graph_show: bool,
}

fn init_graph() -> StableGraph<(), ()> {
    let mut g = StableGraph::new();

    let mut node_indices: Vec<petgraph::prelude::NodeIndex> = Vec::new();
    node_maker(&mut g, &mut node_indices);

    g
}

fn node_maker(
    g: &mut StableGraph<(), ()>,
    node_indices: &mut Vec<petgraph::prelude::NodeIndex>,
) {
    for _i in 0..26 {
        node_indices.push(g.add_node(()));
   }
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

    fn set_visiablity(&mut self, id: &petgraph::prelude::NodeIndex, visable: bool) {
        let g_indices = self.g.g.node_indices().collect::<Vec<_>>();
        if (g_indices.contains(id) == true) && (visable == false) {
            self.remove_node(*id)
        }
    }

    fn remove_node(&mut self, idx: NodeIndex) {
        // before removing nodes we need to remove all edges connected to it
        let neighbors = self.g.g.neighbors_undirected(idx).collect::<Vec<_>>();
        for n in &neighbors {
            self.remove_edges(idx, *n);
            self.remove_edges(*n, idx);
        }

        self.g.g.remove_node(idx).unwrap();
    }

    fn remove_edge(&mut self, start: NodeIndex, end: NodeIndex) {
        let g_idx = self.g.g.find_edge(start, end);
        if g_idx.is_none() {
            return;
        }

        let order = self.g.g.edge_weight(g_idx.unwrap()).unwrap().order();

        self.g.g.remove_edge(g_idx.unwrap()).unwrap();

        // update order of the edges
        let left_siblings = self
            .g
            .g
            .edges_connecting(start, end)
            .map(|edge_ref| edge_ref.id())
            .collect::<Vec<_>>();

        for idx in &left_siblings {
            let sibling_order = self.g.g.edge_weight(*idx).unwrap().order();
            if sibling_order < order {
                return;
            }
            self.g
                .g
                .edge_weight_mut(*idx)
                .unwrap()
                .set_order(sibling_order - 1);
        }
    }

    fn remove_edges(&mut self, start: NodeIndex, end: NodeIndex) {
        let g_idxs = self
            .g
            .g
            .edges_connecting(start, end)
            .map(|e| e.id())
            .collect::<Vec<_>>();
        if g_idxs.is_empty() {
            return;
        }

        g_idxs.iter().for_each(|e| {
            self.g.g.remove_edge(*e).unwrap();
        });
    }

    
    fn reset_graph(&mut self, ui: &mut egui::Ui) {
        let g = init_graph();

        self.g = Graph::from(&g);

        GraphView::<(), (), Directed, DefaultIx>::reset_metadata(ui);
    }

    fn graph_filter(&mut self, phrase: Vec<usize>) {
        let graph_indices = self.g.g.node_indices().collect::<Vec<_>>();
            println!("{:?}", graph_indices);

        for i in &self.g.g.node_indices().collect::<Vec<_>>() {
            //let unused_node = graph_indices.iter().find(|&nodeind| nodeind.index() != *i);
            if phrase.contains(&i.index()) {
                self.set_visiablity(&i, true);
            } else {
                self.set_visiablity(&i, false);
            }
        }
    }

    fn real_graph(&mut self, ui: &mut egui::Ui,phrase: Vec<usize>) {
        GraphView::<(), (), Directed, DefaultIx>::reset_metadata(ui);
        let mut g = StableGraph::new();

        let mut node_indices: Vec<petgraph::prelude::NodeIndex> = Vec::new();

        node_maker(&mut g, &mut node_indices);

        let graph_indices = g.node_indices().collect::<Vec<_>>();
        println!("{:?}", graph_indices);

        //Add edges for each letter in phrase
        for i in g.node_indices().collect::<Vec<_>>() {
            let k = i.index();
            if phrase.contains(&i.index()) {
                match k {
                    0 => (),
                    1 => {
                        g.add_edge(node_indices[k], node_indices[0], ());
                    }
                    2 => {
                        let connections = [0, 1];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    3 => {
                        let connections = [0, 1, 2];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    4 => {
                        let connections = [0, 2, 3];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    5 => {
                        let connections = [0, 1, 3, 4];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    6 => {
                        let connections = [0, 4, 5];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    7 => {
                        let connections = [0, 1, 4, 5, 6];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    8 => {
                        let connections = [0, 1, 2, 4, 6, 7];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    9 => {
                        let connections = [0, 2, 3, 4, 5, 6, 7, 8];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    10 => {
                        let connections = [0, 1, 2, 8, 9];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    11 => {
                        let connections = [0, 2, 3, 8, 9, 10];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    12 => {
                        let connections = [0, 1, 2, 3, 4, 8, 10, 11];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    13 => {
                        let connections = [0, 2, 3, 4, 5, 7, 9, 10, 12];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    14 => {
                        let connections = [0, 1, 4, 5, 6, 8, 11, 12, 13];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    15 => {
                        let connections = [0, 2, 6, 7, 8, 10, 11, 13, 14];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    16 => {
                        let connections = [0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 12, 15];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    17 => {
                        let connections = [0, 1, 2, 3, 4, 6, 7, 8, 10, 11, 14, 15, 16];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    18 => {
                        let connections = [0, 3, 4, 5, 6, 7, 10, 11, 14, 17];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    19 => {
                        let connections = [0, 2, 3, 4, 5, 7, 8, 9, 11, 12, 13, 14, 16, 18];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    20 => {
                        let connections = [0, 2, 3, 5, 6, 8, 9, 10, 12, 15, 16, 17, 18, 19];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    21 => {
                        let connections = [0, 1, 3, 4, 5, 7, 9, 10, 11, 13, 15, 16, 18, 20];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    22 => {
                        let connections = [0, 21];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                            println!("{:?}",con)
                        }
                    }
                    23 => {
                        let connections = [0, 22];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    24 => {
                        let connections = [0, 23];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    25 => {
                        let connections = [0, 24];
                        for &con in connections.iter() {
                            g.add_edge(node_indices[k], node_indices[con], ());
                        }
                    }
                    _ => (),
                }
        
            }
        };

        self.g = Graph::from(&g);

        //Set Label for each node A-Z
        let labels = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"
        ];

        for (i, label) in g.node_indices().zip(labels.iter()) {
            self.g.node_mut(i).unwrap().set_label(label.to_string());
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
                    let phrase = parse_phrase(&self.input_string);

                    println!("{:?}", phrase);

                    self.real_graph(ui, phrase.clone());
                    self.graph_filter(phrase);
                    self.graph_show = true;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.graph_show == true {
                ui.add(&mut GraphView::<_, _, _, _, NodeShapeAnimated, DefaultEdgeShape>::new(&mut self.g)
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

fn convert_word_to_char_ints(word: &str) -> Vec<usize> {
    let mut char_ints = Vec::new(); // Vector to store char_ints for the current word
    for char in word.chars() {
        let char_int = match char {
            'A' | 'a' => 0,
            'B' | 'b' => 1,
            'C' | 'c' => 2,
            'D' | 'd' => 3,
            'E' | 'e' => 4,
            'F' | 'f' => 5,
            'G' | 'g' => 6,
            'H' | 'h' => 7,
            'I' | 'i' => 8,
            'J' | 'j' => 9,
            'K' | 'k' => 10,
            'L' | 'l' => 11,
            'M' | 'm' => 12,
            'N' | 'n' => 13,
            'O' | 'o' => 14,
            'P' | 'p' => 15,
            'Q' | 'q' => 16,
            'R' | 'r' => 17,
            'S' | 's' => 18,
            'T' | 't' => 19,
            'U' | 'u' => 20,
            'V' | 'v' => 21,
            'W' | 'w' => 22,
            'X' | 'x' => 23,
            'Y' | 'y' => 24,
            'Z' | 'z' => 25,
            ' ' => 404202,
            _ => {
                // Handle the case when char is not any of the above
                continue;
            }
        };
        char_ints.push(char_int);
    }
    char_ints
}

fn parse_phrase(phrase: &str) -> Vec<usize> {
    let char_ints = convert_word_to_char_ints(phrase);

    char_ints
}
