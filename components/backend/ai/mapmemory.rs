use common::{Position,Cell};
use std::collections::HashMap;
use petgraph::algo::dijkstra;
use petgraph::visit::{Visitable,Graphlike,VisitMap};

struct MapGrid {
    data: HashMap<Position, Cell>,
}

impl MapGrid {
    pub fn new() -> Self {
        MapGrid {
            data: HashMap::new(),
        }
    }

    pub fn get_path(&mut self, origin: Position, target: Position) -> HashMap<Position, usize>{
        dijkstra(self, origin, Some(target), |graph, pos| {
            pos.neighbors().into_iter()
        })
    }
}

impl Visitable for MapGrid{
    type Map = MapVisitMap;
    fn visit_map(&self) -> MapVisitMap { MapVisitMap::new() }
}

impl Graphlike for MapGrid {
    type NodeId = Position;
}

struct MapVisitMap {
    data: HashMap<Position, ()>,
}

impl MapVisitMap {
    fn new() -> Self {
        MapVisitMap{
            data: HashMap::new(),
        }
    }
}

impl VisitMap<Position> for MapVisitMap {
    fn visit(&mut self, pos: Position) -> bool {
        !self.data.insert(pos, ()).is_some()
    }
    fn is_visited(&self, pos: &Position) -> bool {
        self.data.get(pos).is_some()
    }
}

pub struct MapMemory {
    grid: MapGrid, 
}

impl MapMemory {
    pub fn new() -> Self {
        MapMemory{
            grid: MapGrid::new(),
        }
    }
    pub fn get_path(&mut self, origin: Position, target: Position) -> RoutingInstructions {
        RoutingInstructions::new(self.grid.get_path(origin, target.clone()), target)
    }
}

#[derive(Debug,Clone)]
pub struct RoutingInstructions {
    pub instructions: HashMap<Position, usize>,
    pub target: Position,
}

impl RoutingInstructions {
    pub fn new(instructions: HashMap<Position, usize>, target: Position) -> Self {
        RoutingInstructions{
            instructions: instructions,
            target: target,
        }
    }

    // TODO: should this function have an internal cache?
    pub fn get_next(&self, cur: Position) -> Option<Position> {
        let mut path = vec![];
        let mut end = self.target.clone();
        loop {
            if end == cur {
                break;
            }
            //println!("PUSHING: {:?}", end);
            path.push(end.clone());
            let mut end_cost = self.instructions.get(&end).unwrap_or(&0).to_owned();
            for (t_end, _) in end.neighbors().into_iter() {
                self.instructions.get(&t_end).and_then(|&t_end_cost| {
                    if t_end_cost < end_cost {
                        end_cost = t_end_cost;
                        end = t_end;
                    }
                    Some(())
                });
            }
            
        }
        path.into_iter().rev().nth(0)
        /* 
        let cur_weight: usize = self.instructions.get(&cur).unwrap_or(&0).to_owned();
        let mut res = None;
        let mut res_weight = None;

        for (next, _) in cur.neighbors().into_iter() {
            let weight = self.instructions.get(&next).unwrap_or(&cur_weight).to_owned();
            println!("NEXT: {:?} -{:?} = {:?}", cur_weight, weight, next);
            if let None = res {
                res = Some(next);
                res_weight = Some(weight);
            } else if let Some(tres_weight) = res_weight {
                if weight as i32 - cur_weight as i32 > tres_weight as i32{
                    res = Some(next);
                    res_weight = Some(weight);
                }
            }
        }
        
        res
        */
    }
}
