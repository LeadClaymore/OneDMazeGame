use rand::Rng;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
//use raylib::consts::MouseCursor::*;

const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 1000.0;
const TARGET_FPS: u32 = 30;
const MAZESIZE_X: u32 = 100;
const MAZESIZE_Y: u32 = 100;

/// This represents each square of the maze.
/// Clockwise from the left edge
/// B = blocked,
/// C = cleared.
/// for example BCCC = Only Left Blocked
enum Configuration {
    ERROR,
    BCCC,
    CBCC,
    CCBC,
    CCCB,
    BBCC,
    CBBC,
    CCBB,
    BCCB,
    BCBC,
    CBCB,
    BBBC,
    CBBB,
    BCBB,
    BBCB,
}

/// This stores what the coords and the left (l), up (u), right (r), and down (d).
/// false is blocked, and true is clear
#[derive(Copy, Clone, Debug, PartialEq)]
struct MazePiece {
    /// x cord
    x: u32,
    /// y cord
    y: u32,
    /// if left is clear
    l: bool,
    /// if up is clear
    u: bool,
    /// if right is clear
    r: bool,
    /// if down is clear
    d: bool,
}

impl MazePiece {
    fn new(new_x: u32, new_y: u32, new_l: bool, new_u: bool, new_r: bool, new_d: bool) -> Self {
        MazePiece {
            x: new_x,
            y: new_y,
            l: new_l,
            u: new_u,
            r: new_r,
            d: new_d, 
        }
    }

    fn default() -> Self {
        MazePiece {
            x: 0,
            y: 0,
            l: false,
            u: false,
            r: false,
            d: false, 
        }
    }

    /// returns true if all sides are blocked (false)
    fn unexplored(&self) -> bool {
        !(self.l || self.u || self.r || self.d)
    }

    /// u8 is the direction, 0 for left, 1 for up, 2 for right, 3 for down
    fn set_opening(&mut self, dir: u8) {
        match dir {
            0 => self.l = true,
            1 => self.u = true,
            2 => self.r = true,
            3 => self.d = true,
            _ => panic!("Wrong dirrection sent"),
        }
    }

    /// u8 is the direction, 0 for left, 1 for up, 2 for right, 3 for down
    fn set_oposite_opening(&mut self, dir: u8) {
        match dir {
            0 => self.r = true,
            1 => self.d = true,
            2 => self.l = true,
            3 => self.u = true,
            _ => panic!("Wrong opposit dirrection sent"),
        }
    }

    /// returns what enum would match this piece
    fn get_configuration(&self) -> Configuration {
        match (self.l, self.u, self.r, self.d) {
            (false, true, true, true)  => Configuration::BCCC,
            (true, false, true, true) => Configuration::CBCC,
            (true, true, false, true) => Configuration::CCBC,
            (true, true, true, false) => Configuration::CCCB,
            (false, false, true, true) => Configuration::BBCC,
            (true, false, false, true) => Configuration::CBBC,
            (true, true, false, false) => Configuration::CCBB,
            (false, true, true, false) => Configuration::BCCB,
            (false, true, false, true) => Configuration::BCBC,
            (true, false, true, false) => Configuration::CBCB,
            (false, false, false, true) => Configuration::BBBC,
            (true, false, false, false) => Configuration::CBBB,
            (false, true, false, false) => Configuration::BCBB,
            (false, false, true, false) => Configuration::BBCB,
            _ => Configuration::ERROR,
        }
    }

    /// returns what color would match this piece
    fn get_color(&self) -> (Color, Color, Color, Color) {
        match (self.l, self.u, self.r, self.d) {
            (true, true, true, true)    => (Color::WHITE, Color::WHITE, Color::WHITE, Color::WHITE),
            (false, true, true, true)   => (Color::BLACK, Color::WHITE, Color::WHITE, Color::WHITE),
            (true, false, true, true)   => (Color::WHITE, Color::BLACK, Color::WHITE, Color::WHITE),
            (true, true, false, true)   => (Color::WHITE, Color::WHITE, Color::BLACK, Color::WHITE),
            (true, true, true, false)   => (Color::WHITE, Color::WHITE, Color::WHITE, Color::BLACK),
            (false, false, true, true)  => (Color::BLACK, Color::BLACK, Color::WHITE, Color::WHITE),
            (true, false, false, true)  => (Color::WHITE, Color::BLACK, Color::BLACK, Color::WHITE),
            (true, true, false, false)  => (Color::WHITE, Color::WHITE, Color::BLACK, Color::BLACK),
            (false, true, true, false)  => (Color::BLACK, Color::WHITE, Color::WHITE, Color::BLACK),
            (false, true, false, true)  => (Color::BLACK, Color::WHITE, Color::BLACK, Color::WHITE),
            (true, false, true, false)  => (Color::WHITE, Color::BLACK, Color::WHITE, Color::BLACK),
            (false, false, false, true) => (Color::BLACK, Color::BLACK, Color::BLACK, Color::WHITE),
            (true, false, false, false) => (Color::WHITE, Color::BLACK, Color::BLACK, Color::BLACK),
            (false, true, false, false) => (Color::BLACK, Color::WHITE, Color::BLACK, Color::BLACK),
            (false, false, true, false) => (Color::BLACK, Color::BLACK, Color::WHITE, Color::BLACK),
            _                           => (Color::BLACK, Color::BLACK, Color::BLACK, Color::BLACK),
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Also try Costco hotdog soda combo")
        .vsync()
        .build();

    rl.set_target_fps(TARGET_FPS);

    let mut maze: Vec<Vec<MazePiece>> = Vec::new();
    for ii in 0..MAZESIZE_X as usize {
        maze.push(Vec::new());
        for jj in 0..MAZESIZE_Y as usize {
            println!("[{ii}][{jj}]");
            maze[ii].push(MazePiece::new(ii as u32, jj as u32, false, false, false, false));
        }
    }

    //for the random path
    let mut rng = rand::thread_rng();
    //setting up the maze
    generate_maze(&mut maze, 0, 0, MAZESIZE_X - 1, MAZESIZE_Y - 1, &mut rng);
    //TODO this is where the program runs out of mem, make it more efficent

    // getting the size of each rectangle
    let (rec_x, rec_y, subrec_x, subrec_y) = (
        SCREEN_WIDTH / MAZESIZE_X as f32,
        SCREEN_HEIGHT / MAZESIZE_Y as f32,
        SCREEN_WIDTH / (MAZESIZE_X as f32 * 3.0),
        SCREEN_HEIGHT / (MAZESIZE_Y as f32 * 3.0),
    );

    //drawing the window
    while !rl.window_should_close() {
        //let dt = rl.get_frame_time();

        //Draw
        //start
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        //ongoing

        // this draws the rectangles
        // TODO fix the weird white lines
        // I think its caused by float to int conversion
        // Potential fix 1: round down not up or vice versa
        for ii in 0..MAZESIZE_X as usize {
            for jj in 0..MAZESIZE_Y as usize {
                // top left
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32, 
                    (rec_y * jj as f32) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    Color::BLACK);
                
                //top right
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 2.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 0.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    Color::BLACK);
                
                //bottom left
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 2.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 2.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    Color::BLACK);

                //bottom right
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 0.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 2.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    Color::BLACK);
                
                //gets the color needed
                let (color_left, color_up, color_right, color_down) = maze[ii][jj].get_color();
                
                // left
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 0.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 1.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    color_left);
                
                //top (up)
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 1.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 0.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    color_up);
                
                //right
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 2.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 1.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    color_right);

                //bottom (down)
                d.draw_rectangle(
                    (rec_x * ii as f32) as i32 + (subrec_x * 1.0) as i32, 
                    (rec_y * jj as f32) as i32 + (subrec_y * 2.0) as i32, 
                    subrec_x as i32, 
                    subrec_y as i32, 
                    color_down);

            }
        }
        d.draw_fps(10, 10);
    }
}

fn generate_maze(maze: &mut Vec<Vec<MazePiece>>, x: u32, y: u32, endx: u32, endy: u32, rng: &mut rand::prelude::ThreadRng) {
    // if bad cords are sent, then it just returns (might do an option later)
    if x > MAZESIZE_X || y > MAZESIZE_Y { return }; 

    // get current possible routes
    let mut possible_route: Vec<(u32, u32, u8)> = Vec::new();
    if x != 0 && maze[(x - 1) as usize][y as usize].unexplored() { possible_route.push((x - 1, y, 0)) };
    if y != 0 && maze[x as usize][(y - 1) as usize].unexplored() { possible_route.push((x, y - 1, 1)) };
    if x != (MAZESIZE_X - 1) && maze[(x + 1) as usize][y as usize].unexplored() { possible_route.push((x + 1, y, 2)) };
    if y != (MAZESIZE_Y - 1) && maze[x as usize][(y + 1) as usize].unexplored() { possible_route.push((x, y + 1, 3)) };

    // select route
    if possible_route.len() > 0 {
        let (x2, y2, dir) = possible_route[rng.gen_range(0..possible_route.len())];
        maze[x2 as usize][y2 as usize].set_oposite_opening(dir);
        maze[x as usize][y as usize].set_opening(dir);
        generate_maze(maze, x2, y2, endx, endy, rng);
        generate_maze(maze, x, y, endx, endy, rng);
        generate_maze(maze, x, y, endx, endy, rng);
    }
    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unexplored() {
        let MP = MazePiece::new(0, 0, false, false, false, false);
        assert_eq!(MP.unexplored(), true);
        let MP2 = MazePiece::new(0, 0, true, true, true, true);
        assert_eq!(MP2.unexplored(), false);
        let MP3 = MazePiece::new(0, 0, true, false, false, false);
        assert_eq!(MP3.unexplored(), false);
        // let MP4 = MazePiece::new(0, 0, false, true, false, false);
        // assert_eq!(MP4.unexplored(), false);
        // let MP5 = MazePiece::new(0, 0, false, false, true, false);
        // assert_eq!(MP5.unexplored(), false);
        // let MP6 = MazePiece::new(0, 0, false, false, false, true);
        // assert_eq!(MP6.unexplored(), false);
    }
}

/* 
    Works but need to change later:
    TODO remove un-needed code
    un-used enum
    TODO you lazy bastard you just called the funtion 2 more times rather then figuring out a good way to only do it again if you can
    PotFix 1: have it return a bool
    PotFix 2: split the determining routes and selecting to 2 diffrent functions
    TODO try to make the code run faster / not run out of mem, IDK how
*/