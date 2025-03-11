use super::ast::*;
use super::datatypes::Level;
use super::ptree::FilterLayer;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;

use hashlink::LinkedHashMap;
use petgraph::algo;
use petgraph::graph::NodeIndex;

use crate::filter::FilterError;
use crate::port::Port;

use anyhow::{bail, Result};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlatPattern {
    pub predicates: Vec<Predicate>,
}

impl FlatPattern {
    // Returns true if pattern is empty
    pub(super) fn is_empty(&self) -> bool {
        self.predicates.is_empty()
    }

    // Returns true if self is a fully qualified FlatPattern
    pub(super) fn is_fully_qualified(&self) -> bool {
        let (layers, labels) = (&*LAYERS, &*NODE_BIMAP);

        let mut ret = true;
        let mut prev_header = unwrap_or_ret_false!(labels.get_by_right(&protocol!("ethernet")));

        for predicate in self.predicates.iter() {
            match predicate {
                Predicate::Unary { protocol, .. } => {
                    let cur_header = unwrap_or_ret_false!(labels.get_by_right(protocol));
                    ret = ret && layers.contains_edge(*cur_header, *prev_header);
                    prev_header = cur_header;
                }
                Predicate::Binary { protocol, .. } => {
                    let cur_header = unwrap_or_ret_false!(labels.get_by_right(protocol));
                    ret = ret && (*cur_header == *prev_header)
                }
            }
        }
        ret
    }

    // Returns true if a pattern should be skipped at a given filter layer
    // Example: we don't need to check the pattern "ipv4 and tcp" at the session filter layer.
    pub(super) fn is_prev_layer(
        &self,
        filter_layer: FilterLayer,
        subscription_level: &Level,
    ) -> bool {
        self.predicates
            .iter()
            .all(|p| p.is_prev_layer(filter_layer, subscription_level))
    }

    // Returns a vector of fully qualified patterns from self
    pub(super) fn to_fully_qualified(&self) -> Result<Vec<LayeredPattern>> {
        if self.is_empty() {
            return Ok(Vec::new());
        }

        let (layers, labels, (dists_to_root, nodes_same_layer)) = (&*LAYERS, &*NODE_BIMAP, &*NODES_BY_LAYER);
        
        // println!("layers: {:#?}", layers);
        // println!("labels: {:#?}", labels);
        // println!("dists_to_root: {:#?}", dists_to_root);
        // println!("nodes_same_layer: {:#?}", nodes_same_layer);

        let mut node_paths: HashSet<Vec<NodeIndex>> = HashSet::new();
        let mut headers = HashSet::new();
        for pred in self.predicates.iter() {
            let protocol_name = pred.get_protocol();
            match labels.get_by_right(protocol_name) {
                Some(node) => {
                    let ethernet = labels
                        .get_by_right(&protocol!("ethernet"))
                        .expect("Ethernet not defined.");

                    // if not predicate
                    if pred.is_unary() && pred.is_not_predicate() {
                        // add all other nodes in the LAYERS graph that are at the same layer in the tree as this node to node_paths
                        if let Some(dist) = dists_to_root.get(node) {
                            if let Some(cousin_nodes) = nodes_same_layer.get(dist) {
                                // println!("cousin_nodes: {:#?}", cousin_nodes);
                                for cousin_node in cousin_nodes {
                                    if cousin_node.index() != node.index() {
                                        let cousin_protocol_name = labels.get_by_left(cousin_node).unwrap();
                                        // println!("cousin_protocol_name: {:#?}", cousin_protocol_name);
                                        headers.insert(cousin_protocol_name);

                                        let node_path: HashSet<Vec<NodeIndex>> =
                                            algo::all_simple_paths(&layers, *cousin_node, *ethernet, 0, None).collect();
                                        node_paths.extend(node_path.iter().map(|p| p.to_vec()));
                                    }
                                }
                            }
                        }
                    } else {
                        headers.insert(protocol_name);
                        let node_path: HashSet<Vec<NodeIndex>> =
                            algo::all_simple_paths(&layers, *node, *ethernet, 0, None).collect();
                        node_paths.extend(node_path.iter().map(|p| p.to_vec()));
                    }
                }
                None => panic!("Invalid protocol name: {}", protocol_name),
            } 
        }

        // println!("node_paths: {:#?}", node_paths);

        // let mut node_paths: HashSet<Vec<NodeIndex>> = HashSet::new();
        // let headers = self
           // .predicates
            //.iter()
            //.map(|c| c.get_protocol())
            //.collect::<HashSet<_>>();
        

        // println!("headers: {:#?}", headers);

        // for header in headers.iter() {
        //     println!("header: {:#?}", header);
        //     match labels.get_by_right(header) {
        //         Some(node) => {
        //             let ethernet = labels
        //                 .get_by_right(&protocol!("ethernet"))
        //                 .expect("Ethernet not defined.");
        //             let node_path: HashSet<Vec<NodeIndex>> =
        //                 algo::all_simple_paths(&layers, *node, *ethernet, 0, None).collect();
        //             node_paths.extend(node_path.iter().map(|p| p.to_vec()));
        //         }
        //         None => panic!("Predicate header invalid: {}", header),
        //     }
        // }

        // println!("node_paths: {:#?}", node_paths);

        // all possible fully qualified paths from predicated headers
        let mut fq_paths = HashSet::new();
        for node_path in node_paths {
            let mut fq_path = node_path
                .iter()
                .map(|n| labels.get_by_left(n).unwrap().to_owned())
                .collect::<Vec<_>>();
            fq_path.remove(fq_path.len() - 1); // remove ethernet
            fq_path.reverse();
            fq_paths.insert(fq_path);
        }

        //println!("fq_paths: {:#?}", fq_paths);
        //println!("headers: {:#?}", headers);
        
        // build fully qualified patterns (could have multiple per non-fully-qualified pattern)
        let mut fq_patterns = vec![];
        for fq_path in fq_paths {
            let fq_headers: HashSet<&ProtocolName> = fq_path.iter().clone().collect();
            //println!("fq_headers: {:#?}", fq_headers);
            if !headers.is_disjoint(&fq_headers) {
                let mut fq_pattern = LayeredPattern::new();
                for protocol in fq_path.iter() {
                    let proto_predicates = self
                        .predicates
                        .iter()
                        .filter(|c| c.get_protocol() == protocol && c.is_binary())
                        .map(|c| c.to_owned())
                        .collect::<HashSet<_>>();

                    let mut proto_predicates = proto_predicates.into_iter().collect::<Vec<_>>();
                    proto_predicates.sort();

                    assert!(fq_pattern.add_protocol(protocol.to_owned(), proto_predicates));
                }
                fq_patterns.push(fq_pattern);
                //println!("fq_patterns: {:#?}", fq_patterns);
            }
        }
        if fq_patterns.is_empty() {
            // This happens when the headers provided do not have a directed path to ethernet node
            bail!(FilterError::InvalidPatternLayers(self.to_owned()));
        }
        Ok(fq_patterns)
    }

    // Returns FlatPattern of only predicates that can be filtered in hardware
    pub(super) fn retain_hardware_predicates(&self, port: &Port) -> FlatPattern {
        FlatPattern {
            predicates: self
                .predicates
                .clone()
                .into_iter()
                .filter(|p| p.is_hardware_filterable(port))
                .collect::<Vec<_>>(),
        }
    }
}

impl fmt::Display for FlatPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "[")?;
        let mut first = true;
        for predicate in self.predicates.iter() {
            if !first {
                write!(f, ", {}", predicate)?;
            } else {
                write!(f, "{}", predicate)?;
            }
            first = false;
        }
        write!(f, "]")?;
        Ok(())
    }
}

// Represents a fully qualified pattern, ordered by header layer
#[derive(Debug, Clone)]
pub struct LayeredPattern(LinkedHashMap<ProtocolName, Vec<Predicate>>);

impl LayeredPattern {
    pub(super) fn new() -> Self {
        LayeredPattern(LinkedHashMap::new())
    }

    pub(super) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // Adds predicates on protocol header. Returns true on success
    fn add_protocol(&mut self, proto_name: ProtocolName, field_predicates: Vec<Predicate>) -> bool {
        let (layers, labels) = (&*LAYERS, &*NODE_BIMAP);

        // check that there is an edge to previous protocol header
        // check that field_predicates are all binary
        // check that field_predicates are all predicates on protocol
        let mut ret = true;
        let node = unwrap_or_ret_false!(labels.get_by_right(&proto_name));
        if let Some((outer_proto, _)) = self.0.back() {
            let prev = unwrap_or_ret_false!(labels.get_by_right(outer_proto));
            ret = ret && layers.contains_edge(*node, *prev);
            for pred in field_predicates.iter() {
                ret = ret
                    && match pred {
                        Predicate::Unary { .. } => false,
                        Predicate::Binary { protocol, .. } => protocol == &proto_name,
                    }
            }
        } else {
            // IPv4 or IPv6
            let root = unwrap_or_ret_false!(labels.get_by_right(&protocol!("ethernet")));
            ret = ret && layers.contains_edge(*node, *root);
        }

        if ret {
            self.0.insert(proto_name, field_predicates);
            ret
        } else {
            false
        }
    }

    // flattens LayeredPattern to fully qualified FlatPattern
    pub(super) fn to_flat_pattern(&self) -> FlatPattern {
        let mut predicates = vec![];
        for (protocol, field_preds) in self.0.iter() {
            predicates.push(Predicate::Unary {
                protocol: protocol.to_owned(),
                not_op: false,
            });
            // println!("to_flat_pattern: protocol: {:?}", protocol);
            // println!("to_flat_pattern: field_preds: {:?}", field_preds);
            predicates.extend(field_preds.to_owned());
        }
        FlatPattern { predicates }
    }

    pub(super) fn get_header_predicates(&self) -> &LinkedHashMap<ProtocolName, Vec<Predicate>> {
        &self.0
    }
}

impl Default for LayeredPattern {
    fn default() -> Self {
        Self::new()
    }
}

impl Ord for LayeredPattern {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_flat_pattern().cmp(&other.to_flat_pattern())
    }
}

impl PartialOrd for LayeredPattern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LayeredPattern {
    fn eq(&self, other: &Self) -> bool {
        self.to_flat_pattern() == other.to_flat_pattern()
    }
}

impl Eq for LayeredPattern {}

impl fmt::Display for LayeredPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_flat_pattern())?;
        Ok(())
    }
}
