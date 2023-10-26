use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::path::Path;


const FIELD_COL:usize=25;
const FIELD_ROW:usize=80;


enum Direction {
    Up,
    Down,
    Left,
    Right
}



fn pop(stack:&mut Vec<u8>) -> u8 {
    let result = stack.pop();
    match result {
        Some(c) => c,
        None => 0
    }
}

struct Cursor {
    x: usize,
    y: usize,
    direction: Direction,
    string_mode: bool
}

impl Cursor {
    fn mv_cursor(&mut self){
        match self.direction {

            Direction::Right => {
                if self.x < 24 {
                    self.x += 1
                } else {
                    panic!("Attempt to leave playing field. Please check your script. Position {} {}", self.x, self.y);
                }
            },
            Direction::Left => {
                if self.x > 0 {
                    self.x -= 1
                } else {
                    panic!("Attempt to leave playing field. Please check your script. Position {} {}", self.x, self.y);
                }
            },

            Direction::Down=> {
                if self.y < 79 {
                    self.y += 1
                } else { 
                    panic!("Attempt to leave playing field. Please check your script. Position {} {}", self.x, self.y);
                }
            },
            Direction::Up=> {
                if self.y > 0{
                    self.y -= 1
                } else {
                    panic!("Attempt to leave playing field. Please check your script. Position {} {}", self.x, self.y);
                }
            }


        } 
    }

}

fn main() {
    let path = Path::new("hello.bf");
    let file = match File::open(&path){
        Ok(file) => file,
        Err(why) => {
            panic!("{}", why);
        }
    };
    let file = BufReader::new(file);

    let mut grid_raw = vec![' '; FIELD_COL*FIELD_ROW]; //80 rows and 25 columns for a b93 playing field
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(FIELD_COL).collect();
    let playing_field = grid_base.as_mut_slice();
    
    for (index, line) in file.lines().enumerate(){
        //gotta match cuz the line is a Result<Ok, Err>
        match line {
            Ok(line) => {
                let line: Vec<char> = line.chars().collect();
                for (char_index, c) in line.iter().enumerate(){
                    playing_field[index][char_index] = *c;
                }
            },
            Err(why) => {
                panic!("{}", why);
            }
        }

        
    }
    //println!("{:?}", playing_field);
    
    let mut pc = Cursor {
        x: 0,
        y: 0,
        direction: Direction::Right,
        string_mode: false
    };




    let mut stack: Vec<u8> = Vec::new();

    loop {
        let cur: char = playing_field[pc.y][pc.x];

        if cur == '@' {
            //println!("We have reached the end");
            break;
        }

        if (pc.string_mode) & (cur != '"') {
            stack.push(cur as u8);
            pc.mv_cursor(); //skip to the next iteration
            continue;
        }


        
        //if cur != ' '{
            //println!("{}", cur);
        //}

        //Directional stuff
        if cur == 'v'{
            pc.direction = Direction::Down;
        }
        else if cur == '>' {
            pc.direction = Direction::Right;
        } 
        else if cur == '<' {
            pc.direction = Direction::Left;
        }
        else if cur == '^' {
            pc.direction = Direction::Up;
        }
        else if cur == '_' {
            let result = pop(&mut stack); 
            if result == 0 {
               pc.direction = Direction::Right; 
            } else {
                pc.direction = Direction::Left;
            }
        }
        else if cur == '|'{

            let result = pop(&mut stack);
            
            if result == 0 {
                pc.direction = Direction::Down;
            } else {
                pc.direction = Direction::Up;
            }

        }
        
        if cur == '"' {
            pc.string_mode  = !pc.string_mode;
        }



        if cur.is_ascii_digit() {
            stack.push(cur as u8);



            //let digit = &cur.to_digit(10);
            //match digit {
            //    Some(int) => stack.push(*int),
            //    None => stack.push(0)
            //}
        }
        if cur == ',' {
            print!("{}",pop(&mut stack) as char);
        }
        if cur == ':' {
            stack.push(stack[stack.len() - 1])
        }
        if cur == '\\' {
            let len = stack.len();
            let temp:u8 = stack[len - 1];
            stack[len - 1] = stack[len - 2];
            stack[len - 2] = temp;
        }
        if cur == '$'{
            stack.pop();
        }




        if cur == '#' {
            pc.mv_cursor();
        }
        if cur == '+'{
            let a:u8 = pop(&mut stack);
            let b:u8 = pop(&mut stack);
            stack.push(a+b);
        }
        if cur == '-'{
            let a:u8 = pop(&mut stack);
            let b:u8 = pop(&mut stack);
            stack.push(b-a);
        }
        if cur == '*'{
            let a:u8 = pop(&mut stack);
            let b:u8 = pop(&mut stack);
            stack.push(a*b);
        }
        if cur == '/'{
            let a:u8 = pop(&mut stack);
            let b:u8 = pop(&mut stack);
            if a!=0{
                stack.push(b/a);
            } else {
                stack.push(0);
            }
        }
       

        
        //println!("{:?}",stack);
        pc.mv_cursor();
    }   

    

    

} 
