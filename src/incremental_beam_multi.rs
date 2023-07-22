#![allow(non_snake_case)]


fn main(){}


#[allow(non_camel_case_types)]
type uint=u16;


#[derive(Clone,Default)]
struct Node{
    track_id:uint,
    refs:u8,
}
impl Node {
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
    free:Vec<usize>,
    at:usize,
    next:Vec<usize>,
    cands:Vec<Cand>,
}
impl BeamSearch{
    fn new(node:Node)->BeamSearch{
        const MAX_NODES:usize=MAX_WIDTH*TURN;
        assert!(MAX_NODES<uint::MAX as usize);
        let mut nodes=vec![Node::default();MAX_WIDTH*5];
        nodes[0]=node;
        let mut next=Vec::with_capacity(MAX_WIDTH);
        next.push(0);
        
        BeamSearch{
            free:(0..nodes.len()).collect(),
            nodes,
            at:1,
            next,
            track:Vec::with_capacity(MAX_NODES),
            cands:Vec::with_capacity(MAX_WIDTH),
        }
    }
    
    fn enum_cands(&mut self,input:&Input,turn:usize,cands:&mut Vec<Vec<Cand>>){
        for &i in &self.next{
            let cnt=self.append_cands(input,turn,i,cands);
            self.nodes[i].refs+=cnt;
        }
    }

    fn update<I:Iterator<Item=(Cand,bool)>>(&mut self,cands:I){
        self.cands.clear();
        for (cand,f) in cands{
            if f{
                self.cands.push(cand);
            }
            else{
                assert_ne!(self.nodes[cand.parent as usize].refs,0);
                self.nodes[cand.parent as usize].refs-=1;
            }
        }

        for i in (0..self.at).rev(){
            if self.nodes[self.free[i]].refs==0{
                self.at-=1;
                self.free.swap(i,self.at);
            }
        }

        self.next.clear();
        for cand in &self.cands{
            let node=&mut self.nodes[cand.parent as usize];
            assert_ne!(node.refs,0);
            node.refs-=1;
            let prev=node.track_id;

            let new=if node.refs==0{
                node.apply(cand);
                self.next.push(cand.parent as usize);
                node
            }
            else{
                let mut new=node.new_node(cand);
                new.refs=0;
                let idx=self.free[self.at];
                self.at+=1;
                self.next.push(idx);
                self.nodes[idx]=new;
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

    // 子供の個数を返す
    fn append_cands(&self,input:&Input,turn:usize,idx:usize,cands:&mut Vec<Vec<Cand>>)->u8{
        let node=&self.nodes[idx];
        todo!();
    }

    fn solve(&mut self,input:&Input)->Vec<u8>{
        use std::cmp::Reverse;
        let M=MAX_WIDTH;
        
        let mut cands=(0..=TURN).map(|_|Vec::<Cand>::with_capacity(MAX_WIDTH*4)).collect::<Vec<_>>();
        let mut set=rustc_hash::FxHashSet::default();
        for t in 0..TURN{
            if t!=0{
                let M0=(M as f64*2.).round() as usize;
                let cands=&mut cands[t];
                assert!(!cands.is_empty());
                if cands.len()>M0{
                    cands.select_nth_unstable_by_key(M0,|a|Reverse(a.eval_score));
                    cands.truncate(M0);
                }
                
                cands.sort_unstable_by_key(|a|Reverse(a.eval_score));
                set.clear();
                let mut total=0;
                self.update(cands.drain(..).map(|cand|{
                    let f=total<M && set.insert(cand.hash);
                    total+=f as usize;
                    (cand,f)
                }));
            }
            self.enum_cands(input,t,&mut cands);
        }

        let best=cands.last().unwrap().iter().max_by_key(|a|a.raw_score(input)).unwrap();
        eprintln!("score = {}",best.raw_score(input));

        let mut ret=self.restore(best.parent);
        ret.push(best.op);

        ret
    }
}


struct Input{}