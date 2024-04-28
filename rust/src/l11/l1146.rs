pub struct SnapshotArray {
    snap_shots: Vec<usize>,
    data: Vec<Node>,
}

impl SnapshotArray {
    pub fn new(_length: i32) -> Self {
        SnapshotArray {
            snap_shots: Vec::new(),
            data: Vec::new(),
        }
    }
    
    pub fn set(&mut self, index: i32, val: i32) {
        let curr_snap_id = self.snap_shots.len();
        let prev_snap = if curr_snap_id == 0 {
            0
        } else {
            self.snap_shots[curr_snap_id - 1]
        };
        for (i , value) in self.data.iter_mut().enumerate().rev() {
            if i < prev_snap {
                break;
            }
            if value.index == index {
                value.value = val;
                return;
            }
        }
        self.data.push(Node{index, value:val});
    }
    
    pub fn snap(&mut self) -> i32 {
        self.snap_shots.push(self.data.len());
        self.snap_shots.len() as i32 - 1
    }
    
    pub fn get(&self, index: i32, snap_id: i32) -> i32 {
        let snap_id = snap_id as usize;
        let snap = self.snap_shots[snap_id];
        for i in (0..snap).rev(){
            if self.data[i].index == index {
                return self.data[i].value;
            }
        }
        0
    }
}

struct Node {
    index: i32,
    value:i32,
}