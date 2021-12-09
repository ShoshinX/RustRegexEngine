fn main() {
    let string = "eeehhhhhh";
    let regex  = "e?eeh*";
    let post = infix_to_postfix(regex.chars().collect());
    println!("postfix: {:?}", post);
    let (start, mut states) = postfix_to_nfa(post);
    for state in states.iter(){
        print!(" {:?}", state);
    }
    println!();
    if match_expr(start, string.to_string(), &mut states){
        println!("{} matched with {}", regex, string);
    }else {
        println!("Regex failed with\n regex: {}\n string: {}", regex, string);
    }


}

// Operator array


type Character = usize;
type StateIndex = usize;
const SPLIT : usize= 256;
const MATCH : usize= 257;

#[derive(Hash,Eq,PartialEq,Debug)]
enum State {
    Single{ c: Character, out: StateIndex},
    Double{c: Character, out1:StateIndex, out:StateIndex},
}

#[derive(Debug)]
struct Frag {
    start: StateIndex,
    out: Vec<StateIndex>,
}

fn patch(frag_out: Vec<StateIndex>, state_index: StateIndex, states: &mut Vec<State>){
    for index in frag_out{
        match states[index] {
            State::Single{ref mut out, .. } => *out = state_index,
            State::Double{ref mut out, .. } => *out = state_index,
        }
    }
}

// TODO: Is there a way to debug an NFA construction because it's very hard to visualise what the
// NFA code is doing?

fn postfix_to_nfa(input: Vec<char>) -> (StateIndex, Vec<State>){

    let mut states : Vec<State> = vec![];
    let mut frags : Vec<Frag> = vec![];

    for c in input {
        match c {
            '.' => {
                let e2 = frags.pop().unwrap();
                let e1 = frags.pop().unwrap();
                patch(e1.out, e2.start, &mut states);
                frags.push(Frag {start: e1.start, out: e2.out});
            },
            '|' => {
                let e2 = frags.pop().unwrap();
                let mut e1 = frags.pop().unwrap();
                let s = State::Double{ c: SPLIT, out1: e1.start, out: e2.start };
                states.push(s);
                e1.out.extend(e2.out);
                frags.push(Frag {start: states.len() - 1, out:e1.out });
            },
            '?' => {
                let mut e = frags.pop().unwrap();
                let s = State::Double{c: SPLIT, out1: e.start, out: 0};
                states.push(s);
                let state_index = states.len() - 1;
                e.out.push(state_index);
                frags.push(Frag{start: state_index, out: e.out}); 
            },
            '*' => {
                let e = frags.pop().unwrap();
                let s = State::Double{c: SPLIT, out1 :e.start, out: 0};
                states.push(s);
                let state_index = states.len() - 1;
                patch(e.out, state_index, &mut states);
                frags.push(Frag {start: state_index, out: vec![state_index]});
            },
            '+' => {
                let e = frags.pop().unwrap();
                let s = State::Double{c: SPLIT, out1: e.start, out: 0};
                states.push(s);
                let state_index = states.len() - 1;
                patch(e.out, state_index, &mut states);
                frags.push(Frag {start: e.start, out: vec![state_index]});
            }
            _ => {
                let s = State::Single {c: c as usize, out: 0};
                states.push(s);
                // nothing got updated in out, if its passed to patch since out is an empty vector
                let state_index = states.len() - 1;
                frags.push(Frag {start: state_index as usize, out: vec![state_index]});
            }
        }
    }

    let e = frags.pop().unwrap();
    states.push(State::Single{ c: MATCH, out: states.len()});
    patch(e.out, states.len() - 1, &mut states);
    return (e.start, states);

}




fn match_expr (start: StateIndex, string: String, states: &mut Vec<State>) -> bool{
    // currentSet
    let mut cset:std::collections::HashSet<StateIndex>= std::collections::HashSet::new();
    // nextset
    let mut nset:std::collections::HashSet<StateIndex> = std::collections::HashSet::new();
    add_state(&mut cset, start, states);
    for c in string.chars() {
        step(c as usize, &mut cset,&mut nset, states);
        let temp = cset;
        cset = nset;
        nset = temp;
    }
    return is_match(cset, states);
}
fn is_match(set: std::collections::HashSet<StateIndex>, states: &mut Vec<State>) -> bool{
    for state_index in set.iter() {
       let state = states.get(*state_index).unwrap(); 
       match state {
           State::Single{c,..} => if *c == MATCH { return true; },
           State::Double{c,..} => if *c == MATCH { return true; },
       }
    }
    return false;
}

fn add_state(set: &mut std::collections::HashSet<StateIndex>, state: StateIndex, states: &mut Vec<State>){
    let s = states.get(state).unwrap();
    match *s {
        State::Single{..} => {
            set.insert(state);
            ()
        },
        State::Double{out, out1,c} => { 
            set.insert(state);
            if c == SPLIT { 
                add_state(set, out, states);
                add_state(set, out1, states);
            }
        },
    }
}

fn step(character: usize, cset: &mut std::collections::HashSet<StateIndex>, nset: &mut std::collections::HashSet<StateIndex>, states: &mut Vec<State>){
    nset.clear();
    for state in cset.iter(){
        match states.get(*state).unwrap() {
            State::Single{out,c,..} =>{
                if character == *c {
                    add_state(nset, *out, states);
                }
            },
            State::Double{out,c,..} =>{
                if character == *c {
                    add_state(nset, *out, states);
                } 
            }
        }
    }
}



struct AtomCounter {
    nalt : u32,
    natom: u32,
}
fn infix_to_postfix(input: Vec<char>) -> Vec<char>{
    //
    let mut postfix_expr :Vec<char> = vec![];
    let mut ncounter : Vec<AtomCounter> = vec![];
    let mut natom = 0;
    let mut nalt = 0;
    for c in input{
        match c {
            '(' => {
                if natom > 1 {
                    postfix_expr.push('.');
                    natom -=1;
                }
                ncounter.push(AtomCounter{ nalt, natom});
                natom = 0;
                nalt = 0;
            },
            ')' => {
                while natom > 1{
                    postfix_expr.push('.');
                    natom -= 1;
                }
                while nalt > 0 {
                    postfix_expr.push('|');
                    nalt -= 1;
                }
                let atom_counter = ncounter.pop().unwrap();
                nalt = atom_counter.nalt;
                natom = atom_counter.natom;
                natom += 1;
            },
            '|' => {
                //TODO: What the fuck? why must I do natom -= 1 first?
                natom -= 1;
                while natom > 0 {
                    postfix_expr.push('.'); 
                    natom -=1;
                }
                nalt += 1
            },
            '?' | '+' | '*' => {
                postfix_expr.push(c)
            },
            _ => { 
                if natom > 1 {
                    postfix_expr.push('.');
                    natom -= 1;
                }
                postfix_expr.push(c);
                natom += 1;
            }

        }
    }
    while natom > 1 {
        postfix_expr.push('.');
        natom -= 1;
    }
    while nalt > 1 {
        postfix_expr.push('|');
        nalt -= 1;
    }
    return postfix_expr;
}


fn test_cases() {
    let test1 = "a(bb)+a";
    let test2 = "a(bb|a|aa)+a*";
    println!("Infix   RE: {}", test1);
    println!("Postfix RE: {}", infix_to_postfix(test1.chars().collect()).into_iter().collect::<String>());
    println!("Infix   RE: {}", test2);
    println!("Postfix RE: {}", infix_to_postfix(test2.chars().collect()).into_iter().collect::<String>());
}

