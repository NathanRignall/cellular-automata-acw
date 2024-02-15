fn main() {
    // open a csv file and write the header
    let mut wtr = csv::Writer::from_path("output.csv").unwrap();
    wtr.write_record(&["x", "y"]).unwrap();

    // create a grid of cells 100x100 which are bool and set to false
    let mut grid = vec![vec![false; 100]; 100];
    grid[50][50] = true;

    // loop for 100 iterations
    for _ in 0..10000 {

        // make a new grid of cells
        let mut new_grid = vec![vec![false; 100]; 100];

        let mut count = 0;

        // loop through each cell
        for x in 0..99 {
            for y in 0..99 {
                
                // if the cell is alive move the cell 
                if grid[x][y] {
                    count += 1;
                    
                    // get a random number between 0 and 1
                    let random_x = rand::random::<f32>();
                    let random_y = rand::random::<f32>();

                    // get movement direction
                    let x_dir: i32 = if random_x < 0.5 { -1 } else { 1 };
                    let y_dir: i32 = if random_y < 0.5 { -1 } else { 1 };

                    // get coordinates of the new cell
                    let mut new_x: usize = 0;
                    let mut new_y: usize = 0;

                    if x_dir == -1 && x > 0 {
                        new_x = x - 1;
                    } else {
                        new_x = x + 1;
                    }

                    if y_dir == -1 && y > 0 {
                        new_y = y - 1;
                    } else {
                        new_y = y + 1;
                    }

                    // limit new cell to the grid
                    if new_x > 98 {
                        new_x = 98;
                    }

                    if new_y > 98 {
                        new_y = 98;
                    }

                    // write the new cell to the new grid
                    new_grid[new_x][new_y] = true;

                    // print the new cell
                    println!("x: {}, y: {}", new_x, new_y);

                    // write the new cell to the csv file
                    wtr.write_record(&[new_x.to_string(), new_y.to_string()]).unwrap();
                        
                }
            }
        }

        // if 0 cells break the loop
        if count == 0 {
            println!("No cells");
            break;
        }

        // set the new grid to the old grid
        grid = new_grid;
    }
}
