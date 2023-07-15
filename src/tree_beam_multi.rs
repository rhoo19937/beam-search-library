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
            refs:0,
            valid:0,
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
    refs:u8,
    valid:u16,
}


const MAX_WIDTH:usize=1000;
const TURN:usize=100;

struct BeamSearch{
    state:State,
    nodes:Vec<Node>,
    que:Vec<uint>,
    cur_node:usize,
    free:Vec<uint>,
    at:u16,
}
impl BeamSearch{
    fn new(state:State,node:Node)->BeamSearch{
        const MAX_NODES:usize=MAX_WIDTH*5;
        assert!(MAX_NODES<uint::MAX as usize,"uintのサイズが足りないよ");
        let mut nodes=vec![Node::default();MAX_NODES];
        nodes[0]=node;
        let free=(1..MAX_NODES as uint).rev().collect();

        BeamSearch{
            state,nodes,free,
            que:Vec::with_capacity(MAX_WIDTH),
            cur_node:0,
            at:0,
        }
    }
    
    fn add_node(&mut self,cand:Cand){
        let next=self.nodes[cand.parent as usize].child;
        let new=self.free.pop().expect("MAX_NODEが足りないよ") as uint;
        if next!=!0{
            self.nodes[next as usize].prev=new;
        }
        self.nodes[cand.parent as usize].child=new;
        
        self.nodes[new as usize]=Node{next,..cand.to_node()};
        self.retarget(new);
    }

    fn del_node(&mut self,mut idx:uint){
        assert_eq!(self.nodes[idx as usize].refs,0);
        loop{
            self.free.push(idx);
            let Node{prev,next,parent,..}=self.nodes[idx as usize];
            assert_ne!(parent,!0,"全てのノードを消そうとしています");
            if prev&next==!0 && self.nodes[parent as usize].refs==0{
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

    fn dfs(&mut self,input:&Input,cands:&mut Vec<Vec<Cand>>,single:bool){
        if self.nodes[self.cur_node].child==!0{
            let len=cands.len();
            self.append_cands(input,self.cur_node,cands);
            if len==cands.len(){
                self.que.push(self.cur_node as uint);
            }
            self.nodes[self.cur_node].refs+=(cands.len()-len) as u8;
            return;
        }

        let node=self.cur_node;
        let mut child=self.nodes[node].child;
        let next_single=single&(self.nodes[child as usize].next==!0);

        // let prev_state=self.state.clone();
        'a: loop{
            self.cur_node=child as usize;
            self.state.apply(&self.nodes[child as usize]);
            self.dfs(input,cands,next_single);

            if !next_single{
                self.state.revert(&self.nodes[child as usize]);
                // assert!(prev_state==self.state);
            }

            loop{
                child=self.nodes[child as usize].next;
                if child==!0{
                    break 'a;
                }
                else if self.nodes[child as usize].valid==self.at{
                    break;
                }
            }
        }
        
        if !next_single{
            self.cur_node=node;
        }
    }

    fn no_dfs(&mut self,input:&Input,cands:&mut Vec<Vec<Cand>>){
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
                let len=cands.len();
                self.append_cands(input,self.cur_node,cands);
                if len==cands.len(){
                    self.que.push(self.cur_node as uint);
                }
                self.nodes[self.cur_node].refs+=(cands.len()-len) as u8;
                loop{
                    if self.cur_node==root{
                        return;
                    }
                    let node=&self.nodes[self.cur_node];
                    self.state.revert(&node);

                    loop{
                        self.cur_node=self.nodes[self.cur_node].next as usize;
                        if self.cur_node==!0{
                            self.cur_node=node.parent as usize;
                            break;
                        }
                        else if self.nodes[self.cur_node].valid==self.at{
                            self.state.apply(&self.nodes[self.cur_node]);
                            break;
                        }
                    }
                }
            }
            else{
                self.cur_node=child as usize;
                self.state.apply(&self.nodes[self.cur_node]);
            }
        }
    }

    fn enum_cands(&mut self,input:&Input,cands:&mut Vec<Vec<Cand>>){
        assert_eq!(self.nodes[self.cur_node].valid,self.at);
        self.que.clear();
        self.dfs(input,cands,true);
        // self.no_dfs(input,cands);
    }

    fn retarget(&mut self,mut idx:uint){
        while idx as usize!=self.cur_node{
            self.nodes[idx as usize].valid=self.at;
            idx=self.nodes[idx as usize].parent;
        }
    }

    fn update<I:Iterator<Item=(Cand,bool)>>(&mut self,cands:I){
        self.at+=1;
        for i in 0..self.que.len(){
            self.del_node(self.que[i]);
        }
        
        for (cand,f) in cands{
            if f{
                self.add_node(cand);
            }
            else{
                self.nodes[cand.parent as usize].refs-=1;
                if self.nodes[cand.parent as usize].refs==0{
                    self.del_node(cand.parent);
                }
            }
        }
    }

    fn restore(&self,mut idx:usize)->Vec<u8>{
        let mut ret=vec![];
        loop{
            let Node{op,parent,..}=self.nodes[idx];
            if op==!0{
                break;
            }
            ret.push(op);
            idx=parent as usize;
        }
        
        ret.reverse();
        ret
    }

    fn append_cands(&self,input:&Input,idx:usize,cands:&mut Vec<Vec<Cand>>){
        let node=&self.nodes[idx];
        assert_eq!(node.child,!0);
        assert_eq!(node.valid,self.at);

        todo!();
    }

    fn solve(&mut self,input:&Input)->Vec<u8>{
        use std::cmp::Reverse;
        let M=MAX_WIDTH;
    
        let mut cands=(0..TURN).map(|_|Vec::<Cand>::with_capacity(MAX_WIDTH*4)).collect::<Vec<_>>();
        let mut set=rustc_hash::FxHashSet::default();
        for t in 0..TURN{
            if t!=0{
                let M0=(M as f64*2.).round() as usize;
                let cands=&mut cands[t];
                if cands.len()>M0{
                    cands.select_nth_unstable_by_key(M0,|a|Reverse(a.eval_score));
                    cands.truncate(M0);
                }
                
                cands.sort_unstable_by_key(|a|Reverse(a.eval_score));
                set.clear();
                let mut total=0;
                self.update(cands.iter().map(|cand|{
                    let f=total<M && set.insert(cand.hash);
                    total+=f as usize;
                    (cand.clone(),f)
                }));
            }
            
            cands.clear();
            self.enum_cands(input,&mut cands);
            assert!(!cands.is_empty());
        }
    
        let best=cands.last().unwrap().iter().max_by_key(|a|a.raw_score(input)).unwrap();
        eprintln!("score = {}",best.raw_score(input));
        let mut ret=self.restore(best.parent as usize);
        ret.push(best.op);
    
        ret
    }
}


struct Input{}