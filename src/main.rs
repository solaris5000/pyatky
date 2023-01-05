use std::io;

use rand::Rng;



struct Matrix {
    fields : [u8; 16],
    freespace: u8,
    up : bool,
    down : bool,
    left : bool,
    right : bool,
}

impl Matrix {

    fn randomize_matrix(&mut self){
        self.fields = [0; 16];

        let mut rng = rand::thread_rng();
        self.fields[0] = rng.gen::<u8>()%16;
        let mut x = 1;

        loop 
            {
                let mut found : bool = false;
                let tmp = rng.gen::<u8>()%16;
                (0..x).for_each(|n:usize| 
                    {
                        if self.fields[n] == tmp { found = true; };
                    });
                
                    if found {found = false; continue;}
                    else {
                        self.fields[x] = tmp;
                        x+=1;
                    }
                if x > 15 {break;}
            }
        
    }

    fn check_aviables(&mut self){
        let mut fspc:u8 = 0;
        (0..=15).for_each(|x:usize| 
        {
            if self.fields[x] == 0 { self.freespace = fspc};
            fspc += 1;
        });

   
        if self.freespace.wrapping_sub(4) > 15 { self.down = false; self.up = true; }
        else
        if self.freespace + 4 >= 16 { self.down = true; self.up = false; }
        else { self.down = true; self.up = true; }

        if self.freespace % 4 == 0 { self.left = true; self.right = false; }
        else
        if self.freespace % 4 == 3 { self.left = false; self.right = true; }
        else { self.left = true; self.right = true;}
    }

    fn print_out(&self){
        for x in 0..16 {
            if self.fields[x] == 0 {print!("__ ");}
            else
            {print!("{: <2} ", self.fields[x]);}
            if (x+1)%4 == 0{
            println!(""); }
        }
        println!("\nFreespace: {}", self.freespace);
        println!("\nMoves aviable:");
        if self.up {println!("1. Up");}
        if self.down {println!("2. Down");}
        if self.left {println!("3. Left");}
        if self.right {println!("4. Right");}
    }

    fn get_move(&mut self)
    {
        let mut input = String::new();

        println!("Enter a direction: ");
        io::stdin().read_line(&mut input);

        let trimmed = input.trim();
        match trimmed.parse::<i8>() {
            Ok(i) => {
                if self.up && i == 1{self.fields[usize::from(self.freespace)] = self.fields[usize::from(self.freespace+4)]; self.fields[usize::from(self.freespace+4)] = 0} else
                if self.down && i == 2{self.fields[usize::from(self.freespace)] = self.fields[usize::from(self.freespace-4)]; self.fields[usize::from(self.freespace-4)] = 0} else
                if self.left && i == 3{self.fields[usize::from(self.freespace)] = self.fields[usize::from(self.freespace+1)]; self.fields[usize::from(self.freespace+1)] = 0} else
                if self.right && i == 4{self.fields[usize::from(self.freespace)] = self.fields[usize::from(self.freespace-1)]; self.fields[usize::from(self.freespace-1)] = 0} else {
                println!("this was not an valid move"); input.clear(); self.get_move();
                }
            },
            Err(..) => {println!("this was not an valid number: {}", trimmed); input.clear(); self.get_move();},
        };

        input.clear();
    }

    fn check_win(&self) -> bool
    {
        let mut itr:u8 = 0;
        if self.fields[0] == 0 { 
            loop { //winstate in case of 15-14, with starting empty space on 1st tile
                if itr == 15 {break;}
                if self.fields[usize::from(itr)] != itr{
                    return false;
                }
                itr +=1;
            }
        }
        else{ //winstate in case for normal 14-15, with starting "1" on 1st tile
        loop {
            if itr == 15 {break;}
            if self.fields[usize::from(itr)] != itr+1{
                return false;
            }
            itr +=1;
        }}
        println!("*********************");
        println!("{:^20}","CONGRATULATIONS");
        println!("*********************");
        return true;
    }

    fn game(&mut self)
    {
        self.randomize_matrix();
        loop {
            self.check_aviables();
            self.print_out();
            self.get_move();
            if self.check_win() {break;}
        }
    }

}

fn main() {
    let mut mtx : Matrix = Matrix { fields : [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15], freespace : 0, up : true, down : true, left : true,  right: true};

    mtx.game();
   /* mtx.randomize_matrix();
    mtx.check_aviables();
    mtx.print_out();

    mtx.get_move();

    mtx.print_out();*/

}
