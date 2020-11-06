//https://kata-log.rocks/christmas-lights-kata

fn turn(mut lights: [[i32; M]; N], start_column: usize, start_row: usize,
        end_column: usize, end_row: usize, state: i32) -> ([[i32; M]; N]) {
    for i in start_column..=end_column {
        for j in start_row..=end_row {
            lights[i][j] += state;
        }
    }

    return lights;
}

fn turn_on(mut lights: [[i32; M]; N], start_column: usize, start_row: usize,
           end_column: usize, end_row: usize) -> ([[i32; M]; N]) {
    lights = turn(lights, start_column, start_row, end_column, end_row, 1);

    return lights;
}

fn turn_off(mut lights: [[i32; M]; N], start_column: usize, start_row: usize,
            end_column: usize, end_row: usize) -> ([[i32; M]; N]) {
    lights = turn(lights, start_column, start_row, end_column, end_row, -1);

    return lights;
}

fn toogle(mut lights: [[i32; M]; N], start_column: usize, start_row: usize,
          end_column: usize, end_row: usize) -> ([[i32; M]; N]) {
    lights = turn(lights, start_column, start_row, end_column, end_row, 2);

    return lights;
}

const M: usize = 3;
const N: usize = 3;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn count(lights: [[i32; M]; N]) -> i32 {
        let mut counter = 0i32;
    
        for column in lights.iter() {
            for row in column.iter() {
                counter += row;
            }
        }
    
        return counter;
    }

    #[test]
    fn test_turn() {
        let mut lights = [[0i32; M]; N];

        lights = turn(lights, 0, 0, 1, 1, 1);

        assert_eq!(count(lights), 4);
    }

    #[test]
    fn test_turn_on() {
        let mut lights = [[0i32; M]; N];

        lights = turn_on(lights, 0, 0, 2, 2);

        assert_eq!(count(lights), 9);
    }

    #[test]
    fn test_turn_off() {
        let mut lights = [[0i32; M]; N];

        lights = turn_off(lights, 0, 0, 2, 2);

        assert_eq!(count(lights), -9);
    }

    #[test]
    fn test_toogle() {
        let mut lights = [[1i32; M]; N];

        lights = toogle(lights, 0, 0, 2, 2);

        assert_eq!(count(lights), 27);
    }

    #[test]
    fn test_turn_on_one_light() {
        let mut lights = [[0i32; M]; N];

        lights = turn_on(lights, 0, 0, 0, 0);

        assert_eq!(count(lights), 1);
    }

    #[test]
    fn test_toogle_all_lights() {
        let mut lights = [[0i32; M]; N];

        lights = toogle(lights, 0, 0, M-1, N-1);

        assert_eq!(count(lights), 18);
    }
}