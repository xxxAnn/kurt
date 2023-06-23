type Numbers = u64;

fn main() {
    println!("{:?}", parse("3, mul(4, sum(4, 5)), 6, 7"));
}


#[derive(Debug)]
struct Var {
    name: String, 
    id: usize
}

#[derive(Debug)]
enum Token {
    Constant(Numbers),
    Variable(Var),
    Function(Var),
    FunctionEnd
}

fn parse<T>(str_inp: T) -> Vec<Token>
where T: Into<String> {
    let mut tokens = vec![];

    let s: String = str_inp.into().replace(' ', "");
    
    let ss = sbgsc(s);

    if ss.len() != 1 {
        for indv in ss {
            tokens.append(&mut parse(indv));
        }

        tokens
    } else {
        
        let s = ss.first().unwrap();

        if s.contains('(') {
            let mut split = s.splitn(2, '(');
            let first = split.next().unwrap();
            let rest = split.next().unwrap();
            if rest.ends_with(')') {
                let fname = first.replace('(', "");
                let content = &rest[0..rest.len()-1];
                tokens.push(Token::Function((Var { name: fname, id: 0 }))); // get ids
                tokens.append(&mut parse(content));
                tokens.push(Token::FunctionEnd);
            }
        } else {
            if !s.chars().nth(s.len()-1).unwrap().is_numeric() {
                let prefix_const = &s[0..s.len()-1];
                let var = s.chars().nth(s.len()-1).unwrap();

                if prefix_const != "" {
                    tokens.push(Token::Constant(prefix_const.parse().unwrap()));
                }


                tokens.push(Token::Variable(Var { name: var.to_string(), id: 0 })); // get ids

            } else {

                let prefix_const = &s[0..s.len()];

                if prefix_const != "" {
                    tokens.push(Token::Constant(prefix_const.parse().unwrap()));
                }
            }
        }

        tokens
    }
}

// split by global scope commas
fn sbgsc<T>(str_inp: T) -> Vec<String> 
where T: Into<String> {
    let s: String = str_inp.into();
    let mut res = vec![];

    if !s.contains(',') {
        res.push(s);
    } else {
        let mut current = vec![];
        let mut scope_level = 0;
        for char in s.chars() {
            if char == ',' && scope_level == 0 {
                let temp_res: String = current.iter().collect();
                current = vec![];
                res.push(temp_res);
            } else {
                current.push(char);
            }
            if char == '(' {
                scope_level += 1
            } else if char == ')' {
                scope_level -= 1
            }
        }
        if current.len() != 0 {
            let temp_res: String = current.iter().collect();
            res.push(temp_res);
        }
    }

    res
}