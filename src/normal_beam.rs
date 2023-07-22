#![allow(non_snake_case)]


fn main(){}


#[allow(non_camel_case_types)]
type uint=u16;


#[derive(Clone)]
struct Node{
    track_id:uint,
}
impl Node{
    fn new_node(&self,cand:&Cand)->Node{
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
impl Cand {
    fn raw_score(&self,input:&Input)->i64{
        todo!();
    }
}


const MAX_WIDTH:usize=1000;
const TURN:usize=100;

struct BeamSearch{
    track:Vec<(uint,u8)>,
    nodes:Vec<Node>,
    next_nodes:Vec<Node>,
}
impl BeamSearch{
    fn new(node:Node)->BeamSearch{
        const MAX_NODES:usize=MAX_WIDTH*TURN;
        assert!(MAX_NODES<uint::MAX as usize,"MAX_NODEが足りないからuintのサイズを大きくしてね");
        let mut nodes=Vec::with_capacity(MAX_WIDTH);
        nodes.push(node);
        
        BeamSearch{
            nodes,
            track:Vec::with_capacity(MAX_NODES),
            next_nodes:Vec::with_capacity(MAX_WIDTH),
        }
    }
    
    fn enum_cands(&self,input:&Input,cands:&mut Vec<Cand>){
        for i in 0..self.nodes.len(){
            self.append_cands(input,i,cands);
        }
    }
    
    fn update<I:Iterator<Item=Cand>>(&mut self,cands:I){
        self.next_nodes.clear();
        for cand in cands{
            let node=&self.nodes[cand.parent as usize];
            let mut new=node.new_node(&cand);
            self.track.push((node.track_id,cand.op));
            new.track_id=self.track.len() as uint-1;
            self.next_nodes.push(new);
        }
        
        std::mem::swap(&mut self.nodes,&mut self.next_nodes);
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
            assert!(!cands.is_empty(),"次の合法手が存在しないよ");
        }

        let best=cands.iter().max_by_key(|a|a.raw_score(input)).unwrap();
        eprintln!("score = {}",best.raw_score(input));

        let mut ret=self.restore(best.parent);
        ret.push(best.op);

        ret
    }
}


struct Input{}