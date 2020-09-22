use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, channel, Receiver, TryRecvError};
use std::thread;

pub const INFINITY: i32 = -1;
pub const D4: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1),];
pub const D8: [(i32, i32); 8] = [(0, 1), (1, 0), (-1, 0), (0, -1), (1, 1), (1, -1), (-1, -1), (-1, 1)];

pub struct PathFinder {
    terminator: Sender<()>,
    data: Arc<Data>,
    costs: Vec<Vec<i32>>,
}

impl PathFinder {
    pub fn new(costs: Vec<Vec<i32>>) -> Self {
        let (terminator, t) = channel();
        PathFinder{
            terminator,
            data: Arc::new(Data::new(costs[0].len(), costs.len(), t)),
            costs,
        }
    }

    pub fn update(&mut self, frontier: &Vec<(usize, usize)>, changes: &Vec<(usize, usize, i32)>) {
        for change in changes.iter() {
            self.costs[change.1][change.0] = change.2;
        }

        let mut processor = match self.data.processor.try_lock(){
            Ok(guard) => guard,
            Err(_) => {
                self.terminator.send(());
                self.data.processor.lock().unwrap()
            }
        };
        processor.frontier.extend(frontier);
        drop(processor);

        let costs = self.costs.clone();
        let data = Arc::clone(&self.data);
        thread::spawn(move || data.processor.lock().unwrap().update(&costs, &data.map, data.size.0, data.size.1));
    }

    pub fn get_step(&self, current: (usize, usize)) -> (usize, usize) {
        let mut best = current;
        let mut lowest = i32::MAX;
        let mut map = self.data.map.lock().unwrap();
        for d in D8.iter() {
            let pos = (current.0 as i32 + d.0, current.1 as i32 + d.1);
            if pos.0 < 0 || pos.1 < 0 || pos.0 == self.data.size.0 || pos.1 == self.data.size.1 {
                continue
            }

            let pos = (pos.0 as usize, pos.1 as usize);

            let val = map[pos.1][pos.0];
            if val != INFINITY && (d.1 == 0 || d.0 == 0 || map[pos.1][current.0] != INFINITY ||
                map[current.1][pos.0] != INFINITY) && val < lowest {
                lowest = val;
                best = pos;
            }
        }
        best
    }
}



pub struct Data {
    costs: Vec<Vec<i32>>,
    pub map: Mutex<Vec<Vec<i32>>>,
    pub processor: Mutex<Processor>,
    size: (i32, i32),
}

impl Data {
    pub fn new(w: usize, h: usize, terminator: Receiver<()>) -> Self {
        let map = vec![vec![INFINITY; w]; h];
        Self {
            costs: map.clone(),
            map: Mutex::new(map.clone()),
            processor: Mutex::new(Processor {
                terminator,
                workspace: map,
                frontier: Vec::with_capacity(w + h),
                collector: Vec::with_capacity(w + h),

            }),
            size: (w as i32, h as i32),
        }
    }
}

pub struct Processor {
    terminator: Receiver<()>,
    workspace: Vec<Vec<i32>>,
    pub frontier: Vec<(usize, usize)>,
    collector: Vec<(usize, usize)>,
}

impl Processor {

    pub fn update(&mut self, costs: &Vec<Vec<i32>>, map: &Mutex<Vec<Vec<i32>>>, w: i32, h: i32) {

        for row in self.workspace.iter_mut() {
            for tile in row.iter_mut() {
                *tile = INFINITY;
            }
        }

        for pos in self.frontier.iter() {
            self.workspace[pos.1][pos.0] = 0;
        }

        let mut current_cost;
        let mut pos;
        let mut con;
        let mut value;
        let mut previous;
        let mut supposed;

        while self.frontier.len() != 0 {
            match self.terminator.try_recv() {
                Ok(_) => {
                    self.frontier.clear();
                    return
                },
                Err(err) => match err {
                    TryRecvError::Disconnected => {
                        self.frontier.clear();
                        return
                    },
                    _ => {}
                }
            }



            for current in self.frontier.drain(..) {
                current_cost = self.workspace[current.1][current.0];
                for d in D4.iter() {
                    pos = (current.0 as i32 + d.0, current.1 as i32 + d.1);

                    if  pos.0 < 0 || pos.1 < 0 || pos.0 == w || pos.1 == h {
                        continue
                    }

                    con = (pos.0 as usize, pos.1 as usize);

                    value = costs[con.1][con.0];
                    if value == -1 {
                        continue
                    }

                    previous = self.workspace[con.1][con.0];
                    supposed = current_cost + value;
                    if previous == -1 || previous > supposed {
                        self.workspace[con.1][con.0] = supposed;
                        self.collector.push(con);
                    }
                }
            }
            self.frontier.append(&mut self.collector);
        }

        map.lock().unwrap().clone_from(&self.workspace);
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::pathfinder::{Processor, Data};
    use std::sync::mpsc::channel;
    use std::sync::Mutex;

    #[test]
    fn update_test() {
        let costs = vec![vec![1; 10]; 10];
        let chan = channel();
        let pro = Data::new(10, 10, chan.1);
        {
            pro.processor.lock().unwrap().frontier.push((0, 0));
        }
        pro.processor.lock().unwrap().update(&costs, &pro.map, 10, 10);
        for i in pro.map.lock().unwrap().iter() {
            println!("{:?}", i);
        }
    }
}