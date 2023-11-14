#[allow(non_camel_case_types)]
type uint=u16;


#[derive(Clone)]
struct Node{
    op:u8,
    parent:uint,
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
impl Cand{
    fn raw_score(&self,input:&Input)->i64{
        todo!();
    }
}


const MAX_WIDTH:usize=1000;
const TURN:usize=100;
const MAX_NODES:usize=MAX_WIDTH*TURN;


struct BeamSearch{
    latest:usize,
    nodes:Vec<Node>,
}
impl BeamSearch{
    fn new(node:Node)->BeamSearch{
        assert!(MAX_NODES<uint::MAX as usize,"MAX_NODEが足りないからuintのサイズを大きくしてね");
        let mut nodes=Vec::with_capacity(MAX_NODES);
        nodes.push(node);
        
        BeamSearch{
            latest:0,
            nodes,
        }
    }

    fn reset(&mut self,node:Node){
        self.nodes.clear();
        self.nodes.push(node);
        self.latest=0;
    }
    
    fn enum_cands(&self,input:&Input,turn:usize,cands:&mut Vec<Vec<Cand>>){
        for i in self.latest..self.nodes.len(){
            self.append_cands(input,turn,i,cands)
        }
    }
    
    fn update<I:Iterator<Item=Cand>>(&mut self,cands:I){
        self.latest=self.nodes.len();
        for cand in cands{
            let mut new=self.nodes[cand.parent as usize].new_node(&cand);
            new.op=cand.op;
            new.parent=cand.parent;
            self.nodes.push(new);
        }
    }
    
    fn restore(&self,mut idx:uint)->Vec<u8>{
        let mut ret=vec![];

        while idx!=!0{
            ret.push(self.nodes[idx as usize].op);
            idx=self.nodes[idx as usize].parent;
        }
        
        ret.reverse();
        ret
    }

    fn append_cands(&self,input:&Input,turn:usize,idx:usize,cands:&mut Vec<Vec<Cand>>){
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
                self.update(cands.drain(..).filter(|cand|
                    set.insert(cand.hash)
                ).take(M));
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