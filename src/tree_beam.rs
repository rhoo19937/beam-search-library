#![allow(non_snake_case)]


fn main(){}


#[allow(non_camel_case_types)]
type uint=u16;


#[derive(Clone,PartialEq)]
struct State{}
impl State{
    fn new(input:&Input)->State{
        todo!();
    }

    fn apply(&mut self,node:&Node){
        todo!();
    }

    fn revert(&mut self,node:&Node){
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
    
    fn to_node(&self)->Node{
        Node{
            child:!0,
            prev:!0,
            next:!0,
            op:self.op,
            parent:self.parent,
        }
    }
}


#[derive(Clone,Default)]
struct Node{
    op:u8,
    parent:uint,
    child:uint,
    prev:uint,
    next:uint,
}


const MAX_WIDTH:usize=1000;
const TURN:usize=100;

struct BeamSearch{
    state:State,
    leaf:Vec<uint>,
    next_leaf:Vec<uint>,
    nodes:Vec<Node>,
    cur_node:usize,
    free:Vec<uint>,
}
impl BeamSearch{
    fn new(state:State,node:Node)->BeamSearch{
        const MAX_NODES:usize=MAX_WIDTH*5;
        assert!(MAX_NODES<uint::MAX as usize,"uintのサイズが足りないよ");
        let mut nodes=vec![Node::default();MAX_NODES];
        nodes[0]=node;

        let mut leaf=Vec::with_capacity(MAX_WIDTH);
        leaf.push(0);
        let next_leaf=Vec::with_capacity(MAX_WIDTH);
        let free=(1..MAX_NODES as uint).rev().collect();

        BeamSearch{
            state,nodes,free,
            leaf,next_leaf,
            cur_node:0,
        }
    }
    
    fn add_node(&mut self,cand:Cand){
        let next=self.nodes[cand.parent as usize].child;
        let new=self.free.pop().expect("MAX_NODEが足りないよ") as uint;
        if next!=!0{
            self.nodes[next as usize].prev=new;
        }
        self.nodes[cand.parent as usize].child=new;
        
        self.next_leaf.push(new);
        self.nodes[new as usize]=Node{next,..cand.to_node()};
    }

    fn del_node(&mut self,mut idx:uint){
        loop{
            self.free.push(idx);
            let Node{prev,next,parent,..}=self.nodes[idx as usize];
            assert_ne!(parent,!0,"全てのノードを消そうとしています");
            if prev&next==!0{
                idx=parent;
                continue;
            }

            if prev!=!0{
                self.nodes[prev as usize].next=next;
            }
            else{
                self.nodes[parent as usize].child=next;
            }
            if next!=!0{
                self.nodes[next as usize].prev=prev;
            }
            
            break;
        }
    }

    fn dfs(&mut self,input:&Input,cands:&mut Vec<Cand>,single:bool){
        if self.nodes[self.cur_node].child==!0{
            self.append_cands(input,self.cur_node,cands);
            return;
        }

        let node=self.cur_node;
        let mut child=self.nodes[node].child;
        let next_single=single&(self.nodes[child as usize].next==!0);

        // let prev_state=self.state.clone();
        loop{
            self.cur_node=child as usize;
            self.state.apply(&self.nodes[child as usize]);
            self.dfs(input,cands,next_single);

            if !next_single{
                self.state.revert(&self.nodes[child as usize]);
                // assert!(prev_state==self.state);
            }
            child=self.nodes[child as usize].next;
            if child==!0{
                break;
            }
        }
        
        if !next_single{
            self.cur_node=node;
        }
    }

    fn no_dfs(&mut self,input:&Input,cands:&mut Vec<Cand>){
        loop{
            let Node{next,child,..}=self.nodes[self.cur_node];
            if next==!0 || child==!0{
                break;
            }
            self.cur_node=child as usize;
            self.state.apply(&self.nodes[self.cur_node]);
        }

        let root=self.cur_node;
        loop{
            let child=self.nodes[self.cur_node].child;
            if child==!0{
                self.append_cands(input,self.cur_node,cands);
                loop{
                    if self.cur_node==root{
                        return;
                    }
                    let node=&self.nodes[self.cur_node];
                    self.state.revert(&node);
                    if node.next!=!0{
                        self.cur_node=node.next as usize;
                        self.state.apply(&self.nodes[self.cur_node]);
                        break;
                    }
                    self.cur_node=node.parent as usize;
                }
            }
            else{
                self.cur_node=child as usize;
                self.state.apply(&self.nodes[self.cur_node]);
            }
        }
    }

    fn enum_cands(&mut self,input:&Input,cands:&mut Vec<Cand>){
        // self.dfs(input,cands,true);
        self.no_dfs(input,cands);
    }

    fn update<I:Iterator<Item=Cand>>(&mut self,cands:I){
        self.next_leaf.clear();
        for cand in cands{
            self.add_node(cand);
        }

        for i in 0..self.leaf.len(){
            let n=self.leaf[i];
            if self.nodes[n as usize].child==!0{
                self.del_node(n);
            }
        }

        std::mem::swap(&mut self.leaf,&mut self.next_leaf);
    }

    fn restore(&self,mut idx:uint)->Vec<u8>{
        let mut ret=vec![];
        loop{
            let Node{op,parent,..}=self.nodes[idx as usize];
            if parent==!0{
                break;
            }
            ret.push(op);
            idx=parent;
        }
        
        ret.reverse();
        ret
    }

    fn append_cands(&self,input:&Input,idx:usize,cands:&mut Vec<Cand>){
        let node=&self.nodes[idx];
        assert_eq!(node.child,!0);

        todo!();
    }

    fn solve(&mut self,input:&Input)->Vec<u8>{
        use std::cmp::Reverse;
        let M=MAX_WIDTH;
    
        let mut cands:Vec<Cand>=vec![];
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
                self.update(cands.iter().filter(|cand|
                    set.insert(cand.hash)
                ).take(M).cloned());
            }
            
            cands.clear();
            self.enum_cands(input,&mut cands);
            assert!(!cands.is_empty());
        }
    
        let best=cands.into_iter().max_by_key(|a|a.raw_score(input)).unwrap();
        eprintln!("score = {}",best.raw_score(input));
        let mut ret=self.restore(best.parent);
        ret.push(best.op);
    
        ret
    }
}


struct Input{}