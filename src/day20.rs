use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct Tile {
    // (0,0) is bottom left corner
    grid: HashMap<(usize, usize), bool>,
    id: usize,
    xmax: usize,
    ymax: usize,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            grid: HashMap::new(),
            id: 0,
            xmax: 0,
            ymax: 0,
        }
    }

    fn print(&self) {
        println!("Tile {}:", self.id);
        for yy in 0..=self.ymax {
            let y = self.ymax - yy;
            for x in 0..=self.xmax {
                match self.grid.get(&(x, y)) {
                    Some(true) => print!("#"),
                    _ => print!("."),
                }
            }
            println!("");
        }
    }

    fn from_str(s: &str) -> Tile {
        let tile_id_re = Regex::new(r"Tile (?<id>\d+)").unwrap();
        let mut tile = Tile::new();

        let mut line_iter = s.lines();
        match tile_id_re.captures(line_iter.next().unwrap()) {
            None => panic!("missing tile id"),
            Some(caps) => {
                tile.id = caps["id"].parse::<usize>().unwrap();
            }
        }

        let mut x = 0;
        let mut y = 10;
        loop {
            match line_iter.next() {
                None => break,
                Some(line) => {
                    x = 0;
                    y -= 1;
                    for c in line.chars() {
                        if x > tile.xmax {
                            tile.xmax = x;
                        }
                        if y > tile.ymax {
                            tile.ymax = y;
                        }
                        if c == '#' {
                            tile.grid.insert((x, y), true);
                        }
                        x += 1;
                    }
                }
            }
        }

        tile
    }

    fn rotate(&self) -> Tile {
        let mut tile = Tile::new();
        tile.id = self.id;
        tile.xmax = self.ymax;
        tile.ymax = self.xmax;

        for x in 0..=self.xmax {
            for y in 0..=self.ymax {
                let new_x = tile.xmax - y;
                let new_y = x;
                if self.grid.get(&(x, y)) != None {
                    tile.grid.insert((new_x, new_y), true);
                }
            }
        }

        tile
    }

    fn flip(&self) -> Tile {
        let mut tile = Tile::new();
        tile.id = self.id;
        tile.xmax = self.xmax;
        tile.ymax = self.ymax;

        for x in 0..=self.xmax {
            for y in 0..=self.ymax {
                let new_x = tile.xmax - x;
                let new_y = y;
                if self.grid.get(&(x, y)) != None {
                    tile.grid.insert((new_x, new_y), true);
                }
            }
        }

        tile
    }

    // only works correctly for equal tile widths
    fn vertical_match(top_tile: &Tile, bottom_tile: &Tile) -> bool {
        for x in 0..=top_tile.xmax {
            if top_tile.grid.get(&(x, 0)) != bottom_tile.grid.get(&(x, bottom_tile.ymax)) {
                return false;
            }
        }
        true
    }

    // only works correctly for equal tile heights
    fn horizontal_match(left_tile: &Tile, right_tile: &Tile) -> bool {
        for y in 0..=left_tile.ymax {
            if left_tile.grid.get(&(left_tile.xmax, y)) != right_tile.grid.get(&(0, y)) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct Space {
    tiles: HashMap<(i32, i32), Tile>,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

impl Space {
    fn new() -> Space {
        Space {
            tiles: HashMap::new(),
            xmin: 0,
            xmax: 0,
            ymin: 0,
            ymax: 0,
        }
    }

    // this returns false in the case that there are 0 vert/horiz adjacent tiles
    fn tile_fits(&self, tile: &Tile, x: i32, y: i32) -> bool {
        let mut none_adjacent = true;

        match self.tiles.get(&(x, y)) {
            Some(_) => {
                return false;
            }
            None => {}
        }

        match self.tiles.get(&(x - 1, y)) {
            Some(left_tile) => {
                none_adjacent = false;
                if !Tile::horizontal_match(left_tile, tile) {
                    return false;
                }
            }
            None => {}
        }

        match self.tiles.get(&(x + 1, y)) {
            Some(right_tile) => {
                none_adjacent = false;
                if !Tile::horizontal_match(tile, right_tile) {
                    return false;
                }
            }
            None => {}
        }

        match self.tiles.get(&(x, y + 1)) {
            Some(top_tile) => {
                none_adjacent = false;
                if !Tile::vertical_match(top_tile, tile) {
                    return false;
                }
            }
            None => {}
        }

        match self.tiles.get(&(x, y - 1)) {
            Some(bottom_tile) => {
                none_adjacent = false;
                if !Tile::vertical_match(tile, bottom_tile) {
                    return false;
                }
            }
            None => {}
        }

        none_adjacent == false
    }

    fn fit_tile(&self, tile: &Tile, x: i32, y: i32) -> Option<Tile> {
        let mut tile: Tile = tile.to_owned();

        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }

        tile = tile.rotate();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }
        tile = tile.rotate();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }
        tile = tile.rotate();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }

        tile = tile.flip();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }

        tile = tile.rotate();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }
        tile = tile.rotate();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }
        tile = tile.rotate();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }

        None
    }
}

pub fn run(filename: &str) {
    let mut tiles: Vec<Tile> = read_to_string(filename)
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|s| Tile::from_str(s))
        .collect();

    let mut space = Space::new();
    space.tiles.insert((0, 0), tiles.pop().unwrap());

    while tiles.len() > 0 {
        tiles.rotate_left(1);
        let tile = tiles[0].clone();

        'xy: for x in (space.xmin - 1)..=(space.xmax + 1) {
            for y in (space.ymin - 1)..=(space.ymax + 1) {
                match space.fit_tile(&tile, x, y) {
                    None => {}
                    Some(t) => {
                        space.tiles.insert((x, y), t);

                        if x < space.xmin {
                            space.xmin = x;
                        }
                        if x > space.xmax {
                            space.xmax = x;
                        }
                        if y < space.ymin {
                            space.ymin = y;
                        }
                        if y > space.ymax {
                            space.ymax = y;
                        }

                        tiles.remove(0);
                        break 'xy;
                    }
                }
            }
        }
    }

    let t1 = space.tiles.get(&(space.xmax, space.ymax)).unwrap();
    let t2 = space.tiles.get(&(space.xmax, space.ymin)).unwrap();
    let t3 = space.tiles.get(&(space.xmin, space.ymax)).unwrap();
    let t4 = space.tiles.get(&(space.xmin, space.ymin)).unwrap();

    let product: u64 = t1.id as u64 * t2.id as u64 * t3.id as u64 * t4.id as u64;

    println!("{product}");

    // ok part 2 go
    let mut image: HashMap<(usize, usize), bool> = HashMap::new();
    let mut imagexmax: usize = 0;
    let mut imageymax: usize = 0;

    for spacex in space.xmin..=space.xmax {
        for spacey in space.ymin..=space.ymax {
            let tile = space.tiles.get(&(spacex, spacey)).unwrap();
            for x in 1..tile.xmax {
                for y in 1..tile.ymax {
                    let imagex = (tile.xmax - 1) * (spacex - space.xmin) as usize + x;
                    let imagey = (tile.ymax - 1) * (spacey - space.ymin) as usize + y;
                    if imagexmax < imagex {
                        imagexmax = imagex;
                    }
                    if imageymax < imagey {
                        imageymax = imagey;
                    }
                    match tile.grid.get(&(x, y)) {
                        Some(true) => {
                            image.insert((imagex, imagey), true);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // let's use a Tile for this because it has rotate and flip already
    let mut image_tile = Tile {
        id: 666,
        grid: image,
        xmax: imagexmax,
        ymax: imageymax,
    };

    let mut sea_monster_coords = find_sea_monster_coords(&image_tile);
    if sea_monster_coords.len() == 0 {
        image_tile = image_tile.rotate();
        sea_monster_coords = find_sea_monster_coords(&image_tile);
    }
    if sea_monster_coords.len() == 0 {
        image_tile = image_tile.rotate();
        sea_monster_coords = find_sea_monster_coords(&image_tile);
    }
    if sea_monster_coords.len() == 0 {
        image_tile = image_tile.rotate();
        sea_monster_coords = find_sea_monster_coords(&image_tile);
    }
    if sea_monster_coords.len() == 0 {
        image_tile = image_tile.flip();
        sea_monster_coords = find_sea_monster_coords(&image_tile);
    }
    if sea_monster_coords.len() == 0 {
        image_tile = image_tile.rotate();
        sea_monster_coords = find_sea_monster_coords(&image_tile);
    }
    if sea_monster_coords.len() == 0 {
        image_tile = image_tile.rotate();
        sea_monster_coords = find_sea_monster_coords(&image_tile);
    }
    if sea_monster_coords.len() == 0 {
        image_tile = image_tile.rotate();
        sea_monster_coords = find_sea_monster_coords(&image_tile);
    }

    let roughness = image_tile
        .grid
        .into_keys()
        .filter(|coord| !sea_monster_coords.contains(coord))
        .count();

    println!("{roughness}");
}

lazy_static::lazy_static! {
    static ref SEA_MONSTER_COORDS: Vec<(usize, usize)> = vec![
        (0, 1),
        (1, 0),
        (4, 0),
        (5, 1),
        (6, 1),
        (7, 0),
        (10, 0),
        (11, 1),
        (12, 1),
        (13, 0),
        (16, 0),
        (17, 1),
        (18, 1),
        (18, 2),
        (19, 1),
    ];
}

fn find_sea_monster_coords(tile: &Tile) -> HashSet<(usize, usize)> {
    let mut coords: HashSet<(usize, usize)> = HashSet::new();

    for x in 0..=tile.xmax {
        for y in 0..=tile.ymax {
            if sea_monster_at(&tile, x, y) {
                SEA_MONSTER_COORDS
                    .iter()
                    .map(|(xx, yy)| (xx + x, yy + y))
                    .for_each(|coord| {
                        coords.insert(coord);
                    });
            }
        }
    }

    coords
}

fn sea_monster_at(tile: &Tile, x: usize, y: usize) -> bool {
    let spaces_filled = SEA_MONSTER_COORDS
        .iter()
        .filter(|(xx, yy)| tile.grid.get(&(xx + x, yy + y)) != None)
        .count();
    spaces_filled == SEA_MONSTER_COORDS.len()
}
