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




    let mut stack: Vec<char> = Vec::new();

    loop {
        let cur: char = playing_field[pc.y][pc.x];

        if cur == '@' {
            println!("We have reached the end");
            break;
        }
        
        if cur != ' '{
            print!("{}", cur);
        }

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




        if cur.is_ascii_digit() {
            stack.push(cur);



            //let digit = &cur.to_digit(10);
            //match digit {
            //    Some(int) => stack.push(*int),
            //    None => stack.push(0)
            //}
        }
       

        
        println!("{:?}",stack);
        pc.mv_cursor();
    }   

    

    

} 
