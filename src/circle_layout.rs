use eframe::egui;
use egui::{epaint::CubicBezierShape, Pos2, Stroke};

use egui_graphs::Metadata;
use petgraph::stable_graph::NodeIndex;

/* use eframe::{egui, App, CreationContext};
use egui::{epaint::CubicBezierShape, Context, Pos2, Stroke};

use egui_graphs::{Graph, GraphView, Metadata, SettingsInteraction, SettingsNavigation};
use petgraph::{
    graph,
    stable_graph::{DefaultIx, NodeIndex, StableGraph},
    Directed,
}; */

use crate::edge;
use crate::node;
use crate::Phrase;
use crate::Word;

#[derive(Clone, Debug)]
struct NodePos {
    pos: Pos2,
    node: NodeIndex<u32>,
}

struct NodePosList {
    node_pos_list: Vec<NodePos>,
}

struct Circle {
    center: Pos2,
    radius: f32,
    word: Word,
    node_pos_list: NodePosList,
    origin: Pos2,
}

pub struct CircleLayout {
    circle_list: Vec<Circle>,
}

impl Circle {
    pub fn new(word: Word) -> Self {
        let node_pos = word
            .nodes
            .iter()
            .map(|node| NodePos {
                pos: Pos2::new(0.0, 0.0),
                node: *node,
            })
            .collect::<Vec<NodePos>>();
        let node_pos_list = NodePosList {
            node_pos_list: node_pos,
        };
        let origin = Pos2::new(0.0, 0.0);
        let center = Pos2::new(0.0, 0.0);
        let radius = 20.0 * word.word.len() as f32;

        Circle {
            center,
            radius,
            word,
            node_pos_list,
            origin,
        }
    }
}

impl CircleLayout {
    pub fn new() -> Self {
        CircleLayout {
            circle_list: Vec::new(),
        }
    }

    pub fn layout(
        &mut self,
        phrase: &Phrase,
        graph: &mut egui_graphs::Graph<
            (),
            (),
            petgraph::Directed,
            u32,
            node::NodeShape,
            edge::EdgeShape,
        >,
    ) {
        phrase
            .phrase_words
            .iter()
            .enumerate()
            .for_each(|(i, word)| {
                let mut angle: f32 = -90.0;
                let angle_increment = 360.0 / word.word.len() as f32;

                let mut circle = Circle::new(phrase.phrase_words[i].clone());

                if i != 0 {
                    let current_word = word.clone();
                    let previous_word = phrase.phrase_words[i - 1].clone();
                    let mut previous_w_nodes =
                        previous_word.word.chars().zip(previous_word.nodes.iter());
                    println!("{} ", current_word.word.clone());
                    circle.center.y = circle.radius
                        + self.circle_list[i - 1].center.y
                        + self.circle_list[i - 1].radius;
                    circle.center.x = self.circle_list[i - 1].center.x;
                    angle += (360.0 - 20.0) / word.word.len() as f32;
                    current_word.word.chars().enumerate().for_each(|(ci, c)| {
                        //If the current word starts with a character in the previous word
                        if ci == 0 && previous_word.word.contains(c) {
                            let origin_node = previous_w_nodes.find(|(pc, _)| *pc == c).unwrap().1;

                            let origin_node_pos = self.circle_list[i - 1]
                                .node_pos_list
                                .node_pos_list
                                .iter()
                                .find(|node_pos| node_pos.node == *origin_node)
                                .unwrap()
                                .pos;
                            circle.origin = origin_node_pos;

                            //Get the angle between the previous circle's center and the origin node of the current circle
                            angle = (circle.origin.y - self.circle_list[i - 1].center.y)
                                .atan2(circle.origin.x - self.circle_list[i - 1].center.x)
                                .to_degrees();

                            let hyp = self.circle_list[i - 1].center.distance(circle.origin)
                                + circle.radius;

                            circle.center.x =
                                hyp * angle.to_radians().cos() + self.circle_list[i - 1].center.x;

                            circle.center.y =
                                hyp * angle.to_radians().sin() + self.circle_list[i - 1].center.y;

                            angle += 180.0;
                        }
                    });
                }

                circle.word.nodes.iter().enumerate().for_each(|(i, node)| {
                    if i < circle.word.word.len() {
                        let x = circle.center.x + angle.to_radians().cos() * circle.radius;
                        let y = circle.center.y + angle.to_radians().sin() * circle.radius;

                        circle.node_pos_list.node_pos_list[i].pos = Pos2::new(x, y);
                        println!("{:?}", circle.node_pos_list.node_pos_list[i].pos);

                        graph
                            .node_mut(*node)
                            .expect("NodeIndex should be within node indices")
                            .set_location(egui::Pos2::new(x, y));

                        angle += angle_increment;
                    }
                });

                self.circle_list.push(circle);
            });
    }
    pub fn draw_circles(&self, ui: &mut egui::Ui) {
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
        if self.circle_list.is_empty() {
            return;
        }
        self.circle_list.iter().for_each(|circle| {
            let circle_center = Metadata::get(ui).canvas_to_screen_pos(egui::Pos2 {
                x: (circle.center.x),
                y: (circle.center.y),
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
