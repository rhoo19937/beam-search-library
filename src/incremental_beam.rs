#[allow(non_camel_case_types)]
type uint=u16;


#[derive(Clone,Default)]
struct Node{
    track_id:uint,
    refs:u8,
}
impl Node{
    fn new_node(&self,cand:&Cand)->Node{
        todo!();
    }
    
    fn apply(&mut self,cand:&Cand){
        todo!();
    }
}


#[derive(Clone)]
struct Cand{
    op:u8,
    parent:uint,
    eval_score:i64,
    hash:u64,
}
impl Cand{
    fn raw_score(&self,input:&Input)->i64{
        todo!();
    }
}


const MAX_WIDTH:usize=1000;
const TURN:usize=100;
const MAX_NODES:usize=MAX_WIDTH*TURN;



struct BeamSearch{
    track:Vec<(uint,u8)>,
    nodes:Vec<Node>,
    free:Vec<usize>,
    at:usize,
    cands:Vec<Cand>,
}
impl BeamSearch{
    fn new(node:Node)->Self{
        assert!(MAX_NODES<uint::MAX as usize);
        let mut nodes=vec![Node::default();MAX_WIDTH*2];
        nodes[0]=node;
        
        BeamSearch{
            free:(0..nodes.len()).collect(),
            nodes,
            at:1,
            track:Vec::with_capacity(MAX_NODES),
            cands:Vec::with_capacity(MAX_WIDTH),
        }
    }

    fn reset(&mut self,node:Node){
        self.nodes[0]=node;
        self.at=1;
        self.free.clear();
        self.free.extend(0..self.nodes.len());
        self.track.clear();
        self.cands.clear();
    }
    
    fn enum_cands(&self,input:&Input,cands:&mut Vec<Cand>){
        for &i in &self.free[..self.at]{
            self.append_cands(input,i,cands);
        }
    }
    
    fn update(&mut self,cands:impl Iterator<Item=Cand>){
        self.cands.clear();
        for cand in cands{
            self.nodes[cand.parent as usize].refs+=1;
            self.cands.push(cand);
        }

        for i in (0..self.at).rev(){
            if self.nodes[self.free[i]].refs==0{
                self.at-=1;
                self.free.swap(i,self.at);
            }
        }

        for cand in &self.cands{
            let node=&mut self.nodes[cand.parent as usize];
            assert_ne!(node.refs,0);
            node.refs-=1;
            let prev=node.track_id;
            
            let new=if node.refs==0{
                node.apply(cand);
                node
            }
            else{
                let mut new=node.new_node(cand);
                new.refs=0;
                let idx=if let Some(&idx)=self.free.get(self.at){
                    self.nodes[idx]=new;
                    idx
                } else{
                    self.free.push(self.at);
                    self.nodes.push(new);
                    self.at
                };
                self.at+=1;
                &mut self.nodes[idx]
            };

            self.track.push((prev,cand.op));
            new.track_id=self.track.len() as uint-1;
        }
    }
    
    fn restore(&self,mut idx:uint)->Vec<u8>{
        idx=self.nodes[idx as usize].track_id;
        let mut ret=vec![];
        while idx!=!0{
            ret.push(self.track[idx as usize].1);
            idx=self.track[idx as usize].0;
        }
        ret.reverse();
        ret
    }

    fn append_cands(&self,input:&Input,idx:usize,cands:&mut Vec<Cand>){
        let node=&self.nodes[idx];
        todo!();
    }

    fn solve(&mut self,input:&Input)->Vec<u8>{
        use std::cmp::Reverse;
        let M=MAX_WIDTH;
        
        let mut cands=Vec::<Cand>::with_capacity(MAX_WIDTH);
        let mut set=rustc_hash::FxHashSet::default();
        for t in 0..TURN{
            if t!=0{
                let M0=(M as f64*2.).round() as usize;
                if cands.len()>M0{
                    cands.select_nth_unstable_by_key(M0,|a|Reverse(a.eval_score));
                    cands.truncate(M0);
                }
                cands.sort_unstable_by_key(|a|Reverse(a.eval_score));
                set.clear();
                self.update(cands.drain(..).filter(|cand|
                    set.insert(cand.hash)
                ).take(M));
            }
            cands.clear();
            self.enum_cands(input, &mut cands);
            assert!(!cands.is_empty());
        }

        let best=cands.iter().max_by_key(|a|a.raw_score(input)).unwrap();
        eprintln!("score = {}",best.raw_score(input));

        let mut ret=self.restore(best.parent);
        ret.push(best.op);

        ret
    }
}