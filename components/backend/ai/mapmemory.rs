use common::{Position,Cell};
use backend_traits::entity_thread::EntityThreadArea;
use std::collections::HashMap;
use petgraph::algo::dijkstra;
use petgraph::visit::{Visitable,Graphlike,VisitMap};
use std::sync::mpsc::Sender;

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

    pub fn get_cell(&mut self, pos: &Position) -> Option<Cell> {
        self.data.get(pos).and_then(|cell| {
            Some(cell.to_owned())
        })
    }

    pub fn push(&mut self, data: Vec<(Position, Cell)>) {
        for (pos, cell) in data.into_iter() {
            self.data.insert(pos, cell);
        }
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
    latest_push: Vec<(Position, Cell)>,
}

impl MapMemory {
    pub fn new() -> Self {
        MapMemory{
            grid: MapGrid::new(),
            latest_push: vec![],
        }
    }
    pub fn push(&mut self, data: Vec<(Position, Cell)>) {
        self.latest_push = data.clone();
        self.grid.push(data);
        //println!("push data: {:?}", data);
    }
    pub fn get_path(&mut self, origin: Position, target: Position) -> RoutingInstructions {
        RoutingInstructions::new(self.grid.get_path(origin, target.clone()), target)
    }
    pub fn get_area(&mut self, sender: Sender<Vec<EntityThreadArea>>, pos_1: Position, pos_2: Position) {
        let size = ((pos_2.x - pos_1.x) * (pos_2.y - pos_1.y)) as usize;
        //println!("area size: {:?}", size);
        let mut ret = Vec::with_capacity(size);
        for t_x in pos_1.x..pos_2.x {
            for t_y in pos_1.y..pos_2.y {
                let tpos = Position{ x: t_x, y: t_y };
                if let Some(cell) = self.grid.get_cell(&tpos).clone() {
                    ret.push(EntityThreadArea{
                        pos: tpos.clone(),
                        cell: cell,
                        is_from_memory: self.latest_push.iter().find(|&&(ref pos, _)| pos == &tpos).is_none(),
                    });
                }
            }
        }
        sender.send(ret);
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
